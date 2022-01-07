#[cfg(feature = "implement_exclusive")]
pub trait IAppBarButtonTemplateSettingsImpl: Sized {
    fn KeyboardAcceleratorTextMinWidth(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBarButtonTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IAppBarButtonTemplateSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBarButtonTemplateSettingsVtbl {
    pub const fn new<Impl: IAppBarButtonTemplateSettingsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppBarButtonTemplateSettingsVtbl {
        unsafe extern "system" fn KeyboardAcceleratorTextMinWidth<Impl: IAppBarButtonTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyboardAcceleratorTextMinWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppBarButtonTemplateSettings>, base.5, KeyboardAcceleratorTextMinWidth::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBarTemplateSettingsImpl: Sized {
    fn ClipRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect>;
    fn CompactVerticalDelta(&self) -> ::windows::core::Result<f64>;
    fn CompactRootMargin(&self) -> ::windows::core::Result<super::super::Thickness>;
    fn MinimalVerticalDelta(&self) -> ::windows::core::Result<f64>;
    fn MinimalRootMargin(&self) -> ::windows::core::Result<super::super::Thickness>;
    fn HiddenVerticalDelta(&self) -> ::windows::core::Result<f64>;
    fn HiddenRootMargin(&self) -> ::windows::core::Result<super::super::Thickness>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBarTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IAppBarTemplateSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBarTemplateSettingsVtbl {
    pub const fn new<Impl: IAppBarTemplateSettingsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppBarTemplateSettingsVtbl {
        unsafe extern "system" fn ClipRect<Impl: IAppBarTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ClipRect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompactVerticalDelta<Impl: IAppBarTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CompactVerticalDelta() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompactRootMargin<Impl: IAppBarTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CompactRootMargin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinimalVerticalDelta<Impl: IAppBarTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MinimalVerticalDelta() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinimalRootMargin<Impl: IAppBarTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MinimalRootMargin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HiddenVerticalDelta<Impl: IAppBarTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HiddenVerticalDelta() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HiddenRootMargin<Impl: IAppBarTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HiddenRootMargin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppBarTemplateSettings>, base.5, ClipRect::<Impl, OFFSET>, CompactVerticalDelta::<Impl, OFFSET>, CompactRootMargin::<Impl, OFFSET>, MinimalVerticalDelta::<Impl, OFFSET>, MinimalRootMargin::<Impl, OFFSET>, HiddenVerticalDelta::<Impl, OFFSET>, HiddenRootMargin::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBarTemplateSettings2Impl: Sized {
    fn NegativeCompactVerticalDelta(&self) -> ::windows::core::Result<f64>;
    fn NegativeMinimalVerticalDelta(&self) -> ::windows::core::Result<f64>;
    fn NegativeHiddenVerticalDelta(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBarTemplateSettings2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IAppBarTemplateSettings2";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBarTemplateSettings2Vtbl {
    pub const fn new<Impl: IAppBarTemplateSettings2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppBarTemplateSettings2Vtbl {
        unsafe extern "system" fn NegativeCompactVerticalDelta<Impl: IAppBarTemplateSettings2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NegativeCompactVerticalDelta() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NegativeMinimalVerticalDelta<Impl: IAppBarTemplateSettings2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NegativeMinimalVerticalDelta() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NegativeHiddenVerticalDelta<Impl: IAppBarTemplateSettings2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NegativeHiddenVerticalDelta() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppBarTemplateSettings2>, base.5, NegativeCompactVerticalDelta::<Impl, OFFSET>, NegativeMinimalVerticalDelta::<Impl, OFFSET>, NegativeHiddenVerticalDelta::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBarToggleButtonTemplateSettingsImpl: Sized {
    fn KeyboardAcceleratorTextMinWidth(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBarToggleButtonTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IAppBarToggleButtonTemplateSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBarToggleButtonTemplateSettingsVtbl {
    pub const fn new<Impl: IAppBarToggleButtonTemplateSettingsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppBarToggleButtonTemplateSettingsVtbl {
        unsafe extern "system" fn KeyboardAcceleratorTextMinWidth<Impl: IAppBarToggleButtonTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyboardAcceleratorTextMinWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppBarToggleButtonTemplateSettings>, base.5, KeyboardAcceleratorTextMinWidth::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IButtonBaseImpl: Sized {
    fn ClickMode(&self) -> ::windows::core::Result<super::ClickMode>;
    fn SetClickMode(&self, value: super::ClickMode) -> ::windows::core::Result<()>;
    fn IsPointerOver(&self) -> ::windows::core::Result<bool>;
    fn IsPressed(&self) -> ::windows::core::Result<bool>;
    fn Command(&self) -> ::windows::core::Result<super::super::Input::ICommand>;
    fn SetCommand(&self, value: &::core::option::Option<super::super::Input::ICommand>) -> ::windows::core::Result<()>;
    fn CommandParameter(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetCommandParameter(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn Click(&self, handler: &::core::option::Option<super::super::RoutedEventHandler>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveClick(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IButtonBase {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IButtonBase";
}
#[cfg(feature = "implement_exclusive")]
impl IButtonBaseVtbl {
    pub const fn new<Impl: IButtonBaseImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IButtonBaseVtbl {
        unsafe extern "system" fn ClickMode<Impl: IButtonBaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::ClickMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ClickMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClickMode<Impl: IButtonBaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::ClickMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetClickMode(value).into()
        }
        unsafe extern "system" fn IsPointerOver<Impl: IButtonBaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsPointerOver() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPressed<Impl: IButtonBaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsPressed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Command<Impl: IButtonBaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Command() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCommand<Impl: IButtonBaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCommand(&*(&value as *const <super::super::Input::ICommand as ::windows::core::Abi>::Abi as *const <super::super::Input::ICommand as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CommandParameter<Impl: IButtonBaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CommandParameter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCommandParameter<Impl: IButtonBaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCommandParameter(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Click<Impl: IButtonBaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Click(&*(&handler as *const <super::super::RoutedEventHandler as ::windows::core::Abi>::Abi as *const <super::super::RoutedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveClick<Impl: IButtonBaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveClick(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IButtonBase>, base.5, ClickMode::<Impl, OFFSET>, SetClickMode::<Impl, OFFSET>, IsPointerOver::<Impl, OFFSET>, IsPressed::<Impl, OFFSET>, Command::<Impl, OFFSET>, SetCommand::<Impl, OFFSET>, CommandParameter::<Impl, OFFSET>, SetCommandParameter::<Impl, OFFSET>, Click::<Impl, OFFSET>, RemoveClick::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IButtonBaseFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ButtonBase>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IButtonBaseFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IButtonBaseFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IButtonBaseFactoryVtbl {
    pub const fn new<Impl: IButtonBaseFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IButtonBaseFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IButtonBaseFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IButtonBaseFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IButtonBaseStaticsImpl: Sized {
    fn ClickModeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn IsPointerOverProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn IsPressedProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CommandProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CommandParameterProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IButtonBaseStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IButtonBaseStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IButtonBaseStaticsVtbl {
    pub const fn new<Impl: IButtonBaseStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IButtonBaseStaticsVtbl {
        unsafe extern "system" fn ClickModeProperty<Impl: IButtonBaseStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ClickModeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPointerOverProperty<Impl: IButtonBaseStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsPointerOverProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPressedProperty<Impl: IButtonBaseStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsPressedProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommandProperty<Impl: IButtonBaseStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CommandProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommandParameterProperty<Impl: IButtonBaseStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CommandParameterProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IButtonBaseStatics>, base.5, ClickModeProperty::<Impl, OFFSET>, IsPointerOverProperty::<Impl, OFFSET>, IsPressedProperty::<Impl, OFFSET>, CommandProperty::<Impl, OFFSET>, CommandParameterProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICalendarPanelImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICalendarPanel {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ICalendarPanel";
}
#[cfg(feature = "implement_exclusive")]
impl ICalendarPanelVtbl {
    pub const fn new<Impl: ICalendarPanelImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICalendarPanelVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICalendarPanel>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICalendarViewTemplateSettingsImpl: Sized {
    fn MinViewWidth(&self) -> ::windows::core::Result<f64>;
    fn HeaderText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WeekDay1(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WeekDay2(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WeekDay3(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WeekDay4(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WeekDay5(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WeekDay6(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WeekDay7(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HasMoreContentAfter(&self) -> ::windows::core::Result<bool>;
    fn HasMoreContentBefore(&self) -> ::windows::core::Result<bool>;
    fn HasMoreViews(&self) -> ::windows::core::Result<bool>;
    fn ClipRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect>;
    fn CenterX(&self) -> ::windows::core::Result<f64>;
    fn CenterY(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICalendarViewTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ICalendarViewTemplateSettings";
}
#[cfg(feature = "implement_exclusive")]
impl ICalendarViewTemplateSettingsVtbl {
    pub const fn new<Impl: ICalendarViewTemplateSettingsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICalendarViewTemplateSettingsVtbl {
        unsafe extern "system" fn MinViewWidth<Impl: ICalendarViewTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MinViewWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HeaderText<Impl: ICalendarViewTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HeaderText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WeekDay1<Impl: ICalendarViewTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WeekDay1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WeekDay2<Impl: ICalendarViewTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WeekDay2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WeekDay3<Impl: ICalendarViewTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WeekDay3() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WeekDay4<Impl: ICalendarViewTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WeekDay4() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WeekDay5<Impl: ICalendarViewTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WeekDay5() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WeekDay6<Impl: ICalendarViewTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WeekDay6() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WeekDay7<Impl: ICalendarViewTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WeekDay7() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasMoreContentAfter<Impl: ICalendarViewTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HasMoreContentAfter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasMoreContentBefore<Impl: ICalendarViewTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HasMoreContentBefore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasMoreViews<Impl: ICalendarViewTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HasMoreViews() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClipRect<Impl: ICalendarViewTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ClipRect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CenterX<Impl: ICalendarViewTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CenterX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CenterY<Impl: ICalendarViewTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CenterY() {
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
            ::windows::core::GetRuntimeClassName::<ICalendarViewTemplateSettings>,
            base.5,
            MinViewWidth::<Impl, OFFSET>,
            HeaderText::<Impl, OFFSET>,
            WeekDay1::<Impl, OFFSET>,
            WeekDay2::<Impl, OFFSET>,
            WeekDay3::<Impl, OFFSET>,
            WeekDay4::<Impl, OFFSET>,
            WeekDay5::<Impl, OFFSET>,
            WeekDay6::<Impl, OFFSET>,
            WeekDay7::<Impl, OFFSET>,
            HasMoreContentAfter::<Impl, OFFSET>,
            HasMoreContentBefore::<Impl, OFFSET>,
            HasMoreViews::<Impl, OFFSET>,
            ClipRect::<Impl, OFFSET>,
            CenterX::<Impl, OFFSET>,
            CenterY::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICarouselPanelImpl: Sized {
    fn CanVerticallyScroll(&self) -> ::windows::core::Result<bool>;
    fn SetCanVerticallyScroll(&self, value: bool) -> ::windows::core::Result<()>;
    fn CanHorizontallyScroll(&self) -> ::windows::core::Result<bool>;
    fn SetCanHorizontallyScroll(&self, value: bool) -> ::windows::core::Result<()>;
    fn ExtentWidth(&self) -> ::windows::core::Result<f64>;
    fn ExtentHeight(&self) -> ::windows::core::Result<f64>;
    fn ViewportWidth(&self) -> ::windows::core::Result<f64>;
    fn ViewportHeight(&self) -> ::windows::core::Result<f64>;
    fn HorizontalOffset(&self) -> ::windows::core::Result<f64>;
    fn VerticalOffset(&self) -> ::windows::core::Result<f64>;
    fn ScrollOwner(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetScrollOwner(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn LineUp(&self) -> ::windows::core::Result<()>;
    fn LineDown(&self) -> ::windows::core::Result<()>;
    fn LineLeft(&self) -> ::windows::core::Result<()>;
    fn LineRight(&self) -> ::windows::core::Result<()>;
    fn PageUp(&self) -> ::windows::core::Result<()>;
    fn PageDown(&self) -> ::windows::core::Result<()>;
    fn PageLeft(&self) -> ::windows::core::Result<()>;
    fn PageRight(&self) -> ::windows::core::Result<()>;
    fn MouseWheelUp(&self) -> ::windows::core::Result<()>;
    fn MouseWheelDown(&self) -> ::windows::core::Result<()>;
    fn MouseWheelLeft(&self) -> ::windows::core::Result<()>;
    fn MouseWheelRight(&self) -> ::windows::core::Result<()>;
    fn SetHorizontalOffset(&self, offset: f64) -> ::windows::core::Result<()>;
    fn SetVerticalOffset(&self, offset: f64) -> ::windows::core::Result<()>;
    fn MakeVisible(&self, visual: &::core::option::Option<super::super::UIElement>, rectangle: &super::super::super::super::Foundation::Rect) -> ::windows::core::Result<super::super::super::super::Foundation::Rect>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICarouselPanel {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ICarouselPanel";
}
#[cfg(feature = "implement_exclusive")]
impl ICarouselPanelVtbl {
    pub const fn new<Impl: ICarouselPanelImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICarouselPanelVtbl {
        unsafe extern "system" fn CanVerticallyScroll<Impl: ICarouselPanelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanVerticallyScroll() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCanVerticallyScroll<Impl: ICarouselPanelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCanVerticallyScroll(value).into()
        }
        unsafe extern "system" fn CanHorizontallyScroll<Impl: ICarouselPanelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanHorizontallyScroll() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCanHorizontallyScroll<Impl: ICarouselPanelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCanHorizontallyScroll(value).into()
        }
        unsafe extern "system" fn ExtentWidth<Impl: ICarouselPanelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExtentWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtentHeight<Impl: ICarouselPanelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExtentHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ViewportWidth<Impl: ICarouselPanelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ViewportWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ViewportHeight<Impl: ICarouselPanelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ViewportHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HorizontalOffset<Impl: ICarouselPanelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HorizontalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerticalOffset<Impl: ICarouselPanelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VerticalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScrollOwner<Impl: ICarouselPanelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScrollOwner() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScrollOwner<Impl: ICarouselPanelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetScrollOwner(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LineUp<Impl: ICarouselPanelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).LineUp().into()
        }
        unsafe extern "system" fn LineDown<Impl: ICarouselPanelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).LineDown().into()
        }
        unsafe extern "system" fn LineLeft<Impl: ICarouselPanelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).LineLeft().into()
        }
        unsafe extern "system" fn LineRight<Impl: ICarouselPanelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).LineRight().into()
        }
        unsafe extern "system" fn PageUp<Impl: ICarouselPanelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).PageUp().into()
        }
        unsafe extern "system" fn PageDown<Impl: ICarouselPanelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).PageDown().into()
        }
        unsafe extern "system" fn PageLeft<Impl: ICarouselPanelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).PageLeft().into()
        }
        unsafe extern "system" fn PageRight<Impl: ICarouselPanelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).PageRight().into()
        }
        unsafe extern "system" fn MouseWheelUp<Impl: ICarouselPanelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).MouseWheelUp().into()
        }
        unsafe extern "system" fn MouseWheelDown<Impl: ICarouselPanelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).MouseWheelDown().into()
        }
        unsafe extern "system" fn MouseWheelLeft<Impl: ICarouselPanelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).MouseWheelLeft().into()
        }
        unsafe extern "system" fn MouseWheelRight<Impl: ICarouselPanelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).MouseWheelRight().into()
        }
        unsafe extern "system" fn SetHorizontalOffset<Impl: ICarouselPanelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offset: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHorizontalOffset(offset).into()
        }
        unsafe extern "system" fn SetVerticalOffset<Impl: ICarouselPanelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offset: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetVerticalOffset(offset).into()
        }
        unsafe extern "system" fn MakeVisible<Impl: ICarouselPanelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, visual: ::windows::core::RawPtr, rectangle: super::super::super::super::Foundation::Rect, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MakeVisible(&*(&visual as *const <super::super::UIElement as ::windows::core::Abi>::Abi as *const <super::super::UIElement as ::windows::core::DefaultType>::DefaultType), &*(&rectangle as *const <super::super::super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<ICarouselPanel>,
            base.5,
            CanVerticallyScroll::<Impl, OFFSET>,
            SetCanVerticallyScroll::<Impl, OFFSET>,
            CanHorizontallyScroll::<Impl, OFFSET>,
            SetCanHorizontallyScroll::<Impl, OFFSET>,
            ExtentWidth::<Impl, OFFSET>,
            ExtentHeight::<Impl, OFFSET>,
            ViewportWidth::<Impl, OFFSET>,
            ViewportHeight::<Impl, OFFSET>,
            HorizontalOffset::<Impl, OFFSET>,
            VerticalOffset::<Impl, OFFSET>,
            ScrollOwner::<Impl, OFFSET>,
            SetScrollOwner::<Impl, OFFSET>,
            LineUp::<Impl, OFFSET>,
            LineDown::<Impl, OFFSET>,
            LineLeft::<Impl, OFFSET>,
            LineRight::<Impl, OFFSET>,
            PageUp::<Impl, OFFSET>,
            PageDown::<Impl, OFFSET>,
            PageLeft::<Impl, OFFSET>,
            PageRight::<Impl, OFFSET>,
            MouseWheelUp::<Impl, OFFSET>,
            MouseWheelDown::<Impl, OFFSET>,
            MouseWheelLeft::<Impl, OFFSET>,
            MouseWheelRight::<Impl, OFFSET>,
            SetHorizontalOffset::<Impl, OFFSET>,
            SetVerticalOffset::<Impl, OFFSET>,
            MakeVisible::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICarouselPanelFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<CarouselPanel>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICarouselPanelFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ICarouselPanelFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICarouselPanelFactoryVtbl {
    pub const fn new<Impl: ICarouselPanelFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICarouselPanelFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: ICarouselPanelFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICarouselPanelFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorPickerSliderImpl: Sized {
    fn ColorChannel(&self) -> ::windows::core::Result<super::ColorPickerHsvChannel>;
    fn SetColorChannel(&self, value: super::ColorPickerHsvChannel) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IColorPickerSlider {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IColorPickerSlider";
}
#[cfg(feature = "implement_exclusive")]
impl IColorPickerSliderVtbl {
    pub const fn new<Impl: IColorPickerSliderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IColorPickerSliderVtbl {
        unsafe extern "system" fn ColorChannel<Impl: IColorPickerSliderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::ColorPickerHsvChannel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ColorChannel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorChannel<Impl: IColorPickerSliderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::ColorPickerHsvChannel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetColorChannel(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IColorPickerSlider>, base.5, ColorChannel::<Impl, OFFSET>, SetColorChannel::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorPickerSliderFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ColorPickerSlider>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IColorPickerSliderFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IColorPickerSliderFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IColorPickerSliderFactoryVtbl {
    pub const fn new<Impl: IColorPickerSliderFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IColorPickerSliderFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IColorPickerSliderFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IColorPickerSliderFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorPickerSliderStaticsImpl: Sized {
    fn ColorChannelProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IColorPickerSliderStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IColorPickerSliderStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IColorPickerSliderStaticsVtbl {
    pub const fn new<Impl: IColorPickerSliderStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IColorPickerSliderStaticsVtbl {
        unsafe extern "system" fn ColorChannelProperty<Impl: IColorPickerSliderStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ColorChannelProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IColorPickerSliderStatics>, base.5, ColorChannelProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorSpectrumImpl: Sized {
    fn Color(&self) -> ::windows::core::Result<super::super::super::Color>;
    fn SetColor(&self, value: &super::super::super::Color) -> ::windows::core::Result<()>;
    fn HsvColor(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Numerics::Vector4>;
    fn SetHsvColor(&self, value: &super::super::super::super::Foundation::Numerics::Vector4) -> ::windows::core::Result<()>;
    fn MinHue(&self) -> ::windows::core::Result<i32>;
    fn SetMinHue(&self, value: i32) -> ::windows::core::Result<()>;
    fn MaxHue(&self) -> ::windows::core::Result<i32>;
    fn SetMaxHue(&self, value: i32) -> ::windows::core::Result<()>;
    fn MinSaturation(&self) -> ::windows::core::Result<i32>;
    fn SetMinSaturation(&self, value: i32) -> ::windows::core::Result<()>;
    fn MaxSaturation(&self) -> ::windows::core::Result<i32>;
    fn SetMaxSaturation(&self, value: i32) -> ::windows::core::Result<()>;
    fn MinValue(&self) -> ::windows::core::Result<i32>;
    fn SetMinValue(&self, value: i32) -> ::windows::core::Result<()>;
    fn MaxValue(&self) -> ::windows::core::Result<i32>;
    fn SetMaxValue(&self, value: i32) -> ::windows::core::Result<()>;
    fn Shape(&self) -> ::windows::core::Result<super::ColorSpectrumShape>;
    fn SetShape(&self, value: super::ColorSpectrumShape) -> ::windows::core::Result<()>;
    fn Components(&self) -> ::windows::core::Result<super::ColorSpectrumComponents>;
    fn SetComponents(&self, value: super::ColorSpectrumComponents) -> ::windows::core::Result<()>;
    fn ColorChanged(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<ColorSpectrum, super::ColorChangedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveColorChanged(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IColorSpectrum {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IColorSpectrum";
}
#[cfg(feature = "implement_exclusive")]
impl IColorSpectrumVtbl {
    pub const fn new<Impl: IColorSpectrumImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IColorSpectrumVtbl {
        unsafe extern "system" fn Color<Impl: IColorSpectrumImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Color() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColor<Impl: IColorSpectrumImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetColor(&*(&value as *const <super::super::super::Color as ::windows::core::Abi>::Abi as *const <super::super::super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn HsvColor<Impl: IColorSpectrumImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Numerics::Vector4) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HsvColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHsvColor<Impl: IColorSpectrumImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::super::super::Foundation::Numerics::Vector4) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHsvColor(&*(&value as *const <super::super::super::super::Foundation::Numerics::Vector4 as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Numerics::Vector4 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MinHue<Impl: IColorSpectrumImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MinHue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinHue<Impl: IColorSpectrumImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMinHue(value).into()
        }
        unsafe extern "system" fn MaxHue<Impl: IColorSpectrumImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxHue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxHue<Impl: IColorSpectrumImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMaxHue(value).into()
        }
        unsafe extern "system" fn MinSaturation<Impl: IColorSpectrumImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MinSaturation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinSaturation<Impl: IColorSpectrumImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMinSaturation(value).into()
        }
        unsafe extern "system" fn MaxSaturation<Impl: IColorSpectrumImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxSaturation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxSaturation<Impl: IColorSpectrumImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMaxSaturation(value).into()
        }
        unsafe extern "system" fn MinValue<Impl: IColorSpectrumImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MinValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinValue<Impl: IColorSpectrumImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMinValue(value).into()
        }
        unsafe extern "system" fn MaxValue<Impl: IColorSpectrumImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxValue<Impl: IColorSpectrumImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMaxValue(value).into()
        }
        unsafe extern "system" fn Shape<Impl: IColorSpectrumImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::ColorSpectrumShape) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Shape() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShape<Impl: IColorSpectrumImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::ColorSpectrumShape) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetShape(value).into()
        }
        unsafe extern "system" fn Components<Impl: IColorSpectrumImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::ColorSpectrumComponents) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Components() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetComponents<Impl: IColorSpectrumImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::ColorSpectrumComponents) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetComponents(value).into()
        }
        unsafe extern "system" fn ColorChanged<Impl: IColorSpectrumImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ColorChanged(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<ColorSpectrum, super::ColorChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<ColorSpectrum, super::ColorChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveColorChanged<Impl: IColorSpectrumImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveColorChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IColorSpectrum>,
            base.5,
            Color::<Impl, OFFSET>,
            SetColor::<Impl, OFFSET>,
            HsvColor::<Impl, OFFSET>,
            SetHsvColor::<Impl, OFFSET>,
            MinHue::<Impl, OFFSET>,
            SetMinHue::<Impl, OFFSET>,
            MaxHue::<Impl, OFFSET>,
            SetMaxHue::<Impl, OFFSET>,
            MinSaturation::<Impl, OFFSET>,
            SetMinSaturation::<Impl, OFFSET>,
            MaxSaturation::<Impl, OFFSET>,
            SetMaxSaturation::<Impl, OFFSET>,
            MinValue::<Impl, OFFSET>,
            SetMinValue::<Impl, OFFSET>,
            MaxValue::<Impl, OFFSET>,
            SetMaxValue::<Impl, OFFSET>,
            Shape::<Impl, OFFSET>,
            SetShape::<Impl, OFFSET>,
            Components::<Impl, OFFSET>,
            SetComponents::<Impl, OFFSET>,
            ColorChanged::<Impl, OFFSET>,
            RemoveColorChanged::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorSpectrumFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ColorSpectrum>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IColorSpectrumFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IColorSpectrumFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IColorSpectrumFactoryVtbl {
    pub const fn new<Impl: IColorSpectrumFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IColorSpectrumFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IColorSpectrumFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IColorSpectrumFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IColorSpectrumStaticsImpl: Sized {
    fn ColorProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn HsvColorProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn MinHueProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn MaxHueProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn MinSaturationProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn MaxSaturationProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn MinValueProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn MaxValueProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ShapeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ComponentsProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IColorSpectrumStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IColorSpectrumStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IColorSpectrumStaticsVtbl {
    pub const fn new<Impl: IColorSpectrumStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IColorSpectrumStaticsVtbl {
        unsafe extern "system" fn ColorProperty<Impl: IColorSpectrumStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ColorProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HsvColorProperty<Impl: IColorSpectrumStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HsvColorProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinHueProperty<Impl: IColorSpectrumStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MinHueProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxHueProperty<Impl: IColorSpectrumStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxHueProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinSaturationProperty<Impl: IColorSpectrumStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MinSaturationProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxSaturationProperty<Impl: IColorSpectrumStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxSaturationProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinValueProperty<Impl: IColorSpectrumStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MinValueProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxValueProperty<Impl: IColorSpectrumStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxValueProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShapeProperty<Impl: IColorSpectrumStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShapeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComponentsProperty<Impl: IColorSpectrumStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ComponentsProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IColorSpectrumStatics>, base.5, ColorProperty::<Impl, OFFSET>, HsvColorProperty::<Impl, OFFSET>, MinHueProperty::<Impl, OFFSET>, MaxHueProperty::<Impl, OFFSET>, MinSaturationProperty::<Impl, OFFSET>, MaxSaturationProperty::<Impl, OFFSET>, MinValueProperty::<Impl, OFFSET>, MaxValueProperty::<Impl, OFFSET>, ShapeProperty::<Impl, OFFSET>, ComponentsProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IComboBoxTemplateSettingsImpl: Sized {
    fn DropDownOpenedHeight(&self) -> ::windows::core::Result<f64>;
    fn DropDownClosedHeight(&self) -> ::windows::core::Result<f64>;
    fn DropDownOffset(&self) -> ::windows::core::Result<f64>;
    fn SelectedItemDirection(&self) -> ::windows::core::Result<AnimationDirection>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IComboBoxTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IComboBoxTemplateSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IComboBoxTemplateSettingsVtbl {
    pub const fn new<Impl: IComboBoxTemplateSettingsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IComboBoxTemplateSettingsVtbl {
        unsafe extern "system" fn DropDownOpenedHeight<Impl: IComboBoxTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DropDownOpenedHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DropDownClosedHeight<Impl: IComboBoxTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DropDownClosedHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DropDownOffset<Impl: IComboBoxTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DropDownOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedItemDirection<Impl: IComboBoxTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut AnimationDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectedItemDirection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IComboBoxTemplateSettings>, base.5, DropDownOpenedHeight::<Impl, OFFSET>, DropDownClosedHeight::<Impl, OFFSET>, DropDownOffset::<Impl, OFFSET>, SelectedItemDirection::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IComboBoxTemplateSettings2Impl: Sized {
    fn DropDownContentMinWidth(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IComboBoxTemplateSettings2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IComboBoxTemplateSettings2";
}
#[cfg(feature = "implement_exclusive")]
impl IComboBoxTemplateSettings2Vtbl {
    pub const fn new<Impl: IComboBoxTemplateSettings2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IComboBoxTemplateSettings2Vtbl {
        unsafe extern "system" fn DropDownContentMinWidth<Impl: IComboBoxTemplateSettings2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DropDownContentMinWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IComboBoxTemplateSettings2>, base.5, DropDownContentMinWidth::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICommandBarFlyoutCommandBarImpl: Sized {
    fn FlyoutTemplateSettings(&self) -> ::windows::core::Result<CommandBarFlyoutCommandBarTemplateSettings>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICommandBarFlyoutCommandBar {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ICommandBarFlyoutCommandBar";
}
#[cfg(feature = "implement_exclusive")]
impl ICommandBarFlyoutCommandBarVtbl {
    pub const fn new<Impl: ICommandBarFlyoutCommandBarImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICommandBarFlyoutCommandBarVtbl {
        unsafe extern "system" fn FlyoutTemplateSettings<Impl: ICommandBarFlyoutCommandBarImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FlyoutTemplateSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICommandBarFlyoutCommandBar>, base.5, FlyoutTemplateSettings::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICommandBarFlyoutCommandBarFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<CommandBarFlyoutCommandBar>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICommandBarFlyoutCommandBarFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ICommandBarFlyoutCommandBarFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICommandBarFlyoutCommandBarFactoryVtbl {
    pub const fn new<Impl: ICommandBarFlyoutCommandBarFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICommandBarFlyoutCommandBarFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: ICommandBarFlyoutCommandBarFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICommandBarFlyoutCommandBarFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICommandBarFlyoutCommandBarTemplateSettingsImpl: Sized {
    fn OpenAnimationStartPosition(&self) -> ::windows::core::Result<f64>;
    fn OpenAnimationEndPosition(&self) -> ::windows::core::Result<f64>;
    fn CloseAnimationEndPosition(&self) -> ::windows::core::Result<f64>;
    fn CurrentWidth(&self) -> ::windows::core::Result<f64>;
    fn ExpandedWidth(&self) -> ::windows::core::Result<f64>;
    fn WidthExpansionDelta(&self) -> ::windows::core::Result<f64>;
    fn WidthExpansionAnimationStartPosition(&self) -> ::windows::core::Result<f64>;
    fn WidthExpansionAnimationEndPosition(&self) -> ::windows::core::Result<f64>;
    fn WidthExpansionMoreButtonAnimationStartPosition(&self) -> ::windows::core::Result<f64>;
    fn WidthExpansionMoreButtonAnimationEndPosition(&self) -> ::windows::core::Result<f64>;
    fn ExpandUpOverflowVerticalPosition(&self) -> ::windows::core::Result<f64>;
    fn ExpandDownOverflowVerticalPosition(&self) -> ::windows::core::Result<f64>;
    fn ExpandUpAnimationStartPosition(&self) -> ::windows::core::Result<f64>;
    fn ExpandUpAnimationEndPosition(&self) -> ::windows::core::Result<f64>;
    fn ExpandUpAnimationHoldPosition(&self) -> ::windows::core::Result<f64>;
    fn ExpandDownAnimationStartPosition(&self) -> ::windows::core::Result<f64>;
    fn ExpandDownAnimationEndPosition(&self) -> ::windows::core::Result<f64>;
    fn ExpandDownAnimationHoldPosition(&self) -> ::windows::core::Result<f64>;
    fn ContentClipRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect>;
    fn OverflowContentClipRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICommandBarFlyoutCommandBarTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ICommandBarFlyoutCommandBarTemplateSettings";
}
#[cfg(feature = "implement_exclusive")]
impl ICommandBarFlyoutCommandBarTemplateSettingsVtbl {
    pub const fn new<Impl: ICommandBarFlyoutCommandBarTemplateSettingsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICommandBarFlyoutCommandBarTemplateSettingsVtbl {
        unsafe extern "system" fn OpenAnimationStartPosition<Impl: ICommandBarFlyoutCommandBarTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenAnimationStartPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenAnimationEndPosition<Impl: ICommandBarFlyoutCommandBarTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenAnimationEndPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloseAnimationEndPosition<Impl: ICommandBarFlyoutCommandBarTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CloseAnimationEndPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentWidth<Impl: ICommandBarFlyoutCommandBarTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CurrentWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpandedWidth<Impl: ICommandBarFlyoutCommandBarTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExpandedWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WidthExpansionDelta<Impl: ICommandBarFlyoutCommandBarTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WidthExpansionDelta() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WidthExpansionAnimationStartPosition<Impl: ICommandBarFlyoutCommandBarTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WidthExpansionAnimationStartPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WidthExpansionAnimationEndPosition<Impl: ICommandBarFlyoutCommandBarTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WidthExpansionAnimationEndPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WidthExpansionMoreButtonAnimationStartPosition<Impl: ICommandBarFlyoutCommandBarTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WidthExpansionMoreButtonAnimationStartPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WidthExpansionMoreButtonAnimationEndPosition<Impl: ICommandBarFlyoutCommandBarTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WidthExpansionMoreButtonAnimationEndPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpandUpOverflowVerticalPosition<Impl: ICommandBarFlyoutCommandBarTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExpandUpOverflowVerticalPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpandDownOverflowVerticalPosition<Impl: ICommandBarFlyoutCommandBarTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExpandDownOverflowVerticalPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpandUpAnimationStartPosition<Impl: ICommandBarFlyoutCommandBarTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExpandUpAnimationStartPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpandUpAnimationEndPosition<Impl: ICommandBarFlyoutCommandBarTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExpandUpAnimationEndPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpandUpAnimationHoldPosition<Impl: ICommandBarFlyoutCommandBarTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExpandUpAnimationHoldPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpandDownAnimationStartPosition<Impl: ICommandBarFlyoutCommandBarTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExpandDownAnimationStartPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpandDownAnimationEndPosition<Impl: ICommandBarFlyoutCommandBarTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExpandDownAnimationEndPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpandDownAnimationHoldPosition<Impl: ICommandBarFlyoutCommandBarTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExpandDownAnimationHoldPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentClipRect<Impl: ICommandBarFlyoutCommandBarTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContentClipRect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OverflowContentClipRect<Impl: ICommandBarFlyoutCommandBarTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OverflowContentClipRect() {
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
            ::windows::core::GetRuntimeClassName::<ICommandBarFlyoutCommandBarTemplateSettings>,
            base.5,
            OpenAnimationStartPosition::<Impl, OFFSET>,
            OpenAnimationEndPosition::<Impl, OFFSET>,
            CloseAnimationEndPosition::<Impl, OFFSET>,
            CurrentWidth::<Impl, OFFSET>,
            ExpandedWidth::<Impl, OFFSET>,
            WidthExpansionDelta::<Impl, OFFSET>,
            WidthExpansionAnimationStartPosition::<Impl, OFFSET>,
            WidthExpansionAnimationEndPosition::<Impl, OFFSET>,
            WidthExpansionMoreButtonAnimationStartPosition::<Impl, OFFSET>,
            WidthExpansionMoreButtonAnimationEndPosition::<Impl, OFFSET>,
            ExpandUpOverflowVerticalPosition::<Impl, OFFSET>,
            ExpandDownOverflowVerticalPosition::<Impl, OFFSET>,
            ExpandUpAnimationStartPosition::<Impl, OFFSET>,
            ExpandUpAnimationEndPosition::<Impl, OFFSET>,
            ExpandUpAnimationHoldPosition::<Impl, OFFSET>,
            ExpandDownAnimationStartPosition::<Impl, OFFSET>,
            ExpandDownAnimationEndPosition::<Impl, OFFSET>,
            ExpandDownAnimationHoldPosition::<Impl, OFFSET>,
            ContentClipRect::<Impl, OFFSET>,
            OverflowContentClipRect::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICommandBarTemplateSettingsImpl: Sized {
    fn ContentHeight(&self) -> ::windows::core::Result<f64>;
    fn OverflowContentClipRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect>;
    fn OverflowContentMinWidth(&self) -> ::windows::core::Result<f64>;
    fn OverflowContentMaxHeight(&self) -> ::windows::core::Result<f64>;
    fn OverflowContentHorizontalOffset(&self) -> ::windows::core::Result<f64>;
    fn OverflowContentHeight(&self) -> ::windows::core::Result<f64>;
    fn NegativeOverflowContentHeight(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICommandBarTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ICommandBarTemplateSettings";
}
#[cfg(feature = "implement_exclusive")]
impl ICommandBarTemplateSettingsVtbl {
    pub const fn new<Impl: ICommandBarTemplateSettingsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICommandBarTemplateSettingsVtbl {
        unsafe extern "system" fn ContentHeight<Impl: ICommandBarTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContentHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OverflowContentClipRect<Impl: ICommandBarTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OverflowContentClipRect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OverflowContentMinWidth<Impl: ICommandBarTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OverflowContentMinWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OverflowContentMaxHeight<Impl: ICommandBarTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OverflowContentMaxHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OverflowContentHorizontalOffset<Impl: ICommandBarTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OverflowContentHorizontalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OverflowContentHeight<Impl: ICommandBarTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OverflowContentHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NegativeOverflowContentHeight<Impl: ICommandBarTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NegativeOverflowContentHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICommandBarTemplateSettings>, base.5, ContentHeight::<Impl, OFFSET>, OverflowContentClipRect::<Impl, OFFSET>, OverflowContentMinWidth::<Impl, OFFSET>, OverflowContentMaxHeight::<Impl, OFFSET>, OverflowContentHorizontalOffset::<Impl, OFFSET>, OverflowContentHeight::<Impl, OFFSET>, NegativeOverflowContentHeight::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICommandBarTemplateSettings2Impl: Sized {
    fn OverflowContentMaxWidth(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICommandBarTemplateSettings2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ICommandBarTemplateSettings2";
}
#[cfg(feature = "implement_exclusive")]
impl ICommandBarTemplateSettings2Vtbl {
    pub const fn new<Impl: ICommandBarTemplateSettings2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICommandBarTemplateSettings2Vtbl {
        unsafe extern "system" fn OverflowContentMaxWidth<Impl: ICommandBarTemplateSettings2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OverflowContentMaxWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICommandBarTemplateSettings2>, base.5, OverflowContentMaxWidth::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICommandBarTemplateSettings3Impl: Sized {
    fn EffectiveOverflowButtonVisibility(&self) -> ::windows::core::Result<super::super::Visibility>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICommandBarTemplateSettings3 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ICommandBarTemplateSettings3";
}
#[cfg(feature = "implement_exclusive")]
impl ICommandBarTemplateSettings3Vtbl {
    pub const fn new<Impl: ICommandBarTemplateSettings3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICommandBarTemplateSettings3Vtbl {
        unsafe extern "system" fn EffectiveOverflowButtonVisibility<Impl: ICommandBarTemplateSettings3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Visibility) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EffectiveOverflowButtonVisibility() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICommandBarTemplateSettings3>, base.5, EffectiveOverflowButtonVisibility::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICommandBarTemplateSettings4Impl: Sized {
    fn OverflowContentCompactYTranslation(&self) -> ::windows::core::Result<f64>;
    fn OverflowContentMinimalYTranslation(&self) -> ::windows::core::Result<f64>;
    fn OverflowContentHiddenYTranslation(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICommandBarTemplateSettings4 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ICommandBarTemplateSettings4";
}
#[cfg(feature = "implement_exclusive")]
impl ICommandBarTemplateSettings4Vtbl {
    pub const fn new<Impl: ICommandBarTemplateSettings4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICommandBarTemplateSettings4Vtbl {
        unsafe extern "system" fn OverflowContentCompactYTranslation<Impl: ICommandBarTemplateSettings4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OverflowContentCompactYTranslation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OverflowContentMinimalYTranslation<Impl: ICommandBarTemplateSettings4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OverflowContentMinimalYTranslation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OverflowContentHiddenYTranslation<Impl: ICommandBarTemplateSettings4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OverflowContentHiddenYTranslation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICommandBarTemplateSettings4>, base.5, OverflowContentCompactYTranslation::<Impl, OFFSET>, OverflowContentMinimalYTranslation::<Impl, OFFSET>, OverflowContentHiddenYTranslation::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDragCompletedEventArgsImpl: Sized {
    fn HorizontalChange(&self) -> ::windows::core::Result<f64>;
    fn VerticalChange(&self) -> ::windows::core::Result<f64>;
    fn Canceled(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDragCompletedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IDragCompletedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IDragCompletedEventArgsVtbl {
    pub const fn new<Impl: IDragCompletedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDragCompletedEventArgsVtbl {
        unsafe extern "system" fn HorizontalChange<Impl: IDragCompletedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HorizontalChange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerticalChange<Impl: IDragCompletedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VerticalChange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Canceled<Impl: IDragCompletedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Canceled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDragCompletedEventArgs>, base.5, HorizontalChange::<Impl, OFFSET>, VerticalChange::<Impl, OFFSET>, Canceled::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDragCompletedEventArgsFactoryImpl: Sized {
    fn CreateInstanceWithHorizontalChangeVerticalChangeAndCanceled(&self, horizontalchange: f64, verticalchange: f64, canceled: bool, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<DragCompletedEventArgs>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDragCompletedEventArgsFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IDragCompletedEventArgsFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IDragCompletedEventArgsFactoryVtbl {
    pub const fn new<Impl: IDragCompletedEventArgsFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDragCompletedEventArgsFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithHorizontalChangeVerticalChangeAndCanceled<Impl: IDragCompletedEventArgsFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, horizontalchange: f64, verticalchange: f64, canceled: bool, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithHorizontalChangeVerticalChangeAndCanceled(horizontalchange, verticalchange, canceled, &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDragCompletedEventArgsFactory>, base.5, CreateInstanceWithHorizontalChangeVerticalChangeAndCanceled::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDragDeltaEventArgsImpl: Sized {
    fn HorizontalChange(&self) -> ::windows::core::Result<f64>;
    fn VerticalChange(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDragDeltaEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IDragDeltaEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IDragDeltaEventArgsVtbl {
    pub const fn new<Impl: IDragDeltaEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDragDeltaEventArgsVtbl {
        unsafe extern "system" fn HorizontalChange<Impl: IDragDeltaEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HorizontalChange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerticalChange<Impl: IDragDeltaEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VerticalChange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDragDeltaEventArgs>, base.5, HorizontalChange::<Impl, OFFSET>, VerticalChange::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDragDeltaEventArgsFactoryImpl: Sized {
    fn CreateInstanceWithHorizontalChangeAndVerticalChange(&self, horizontalchange: f64, verticalchange: f64, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<DragDeltaEventArgs>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDragDeltaEventArgsFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IDragDeltaEventArgsFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IDragDeltaEventArgsFactoryVtbl {
    pub const fn new<Impl: IDragDeltaEventArgsFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDragDeltaEventArgsFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithHorizontalChangeAndVerticalChange<Impl: IDragDeltaEventArgsFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, horizontalchange: f64, verticalchange: f64, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithHorizontalChangeAndVerticalChange(horizontalchange, verticalchange, &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDragDeltaEventArgsFactory>, base.5, CreateInstanceWithHorizontalChangeAndVerticalChange::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDragStartedEventArgsImpl: Sized {
    fn HorizontalOffset(&self) -> ::windows::core::Result<f64>;
    fn VerticalOffset(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDragStartedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IDragStartedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IDragStartedEventArgsVtbl {
    pub const fn new<Impl: IDragStartedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDragStartedEventArgsVtbl {
        unsafe extern "system" fn HorizontalOffset<Impl: IDragStartedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HorizontalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerticalOffset<Impl: IDragStartedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VerticalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDragStartedEventArgs>, base.5, HorizontalOffset::<Impl, OFFSET>, VerticalOffset::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDragStartedEventArgsFactoryImpl: Sized {
    fn CreateInstanceWithHorizontalOffsetAndVerticalOffset(&self, horizontaloffset: f64, verticaloffset: f64, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<DragStartedEventArgs>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDragStartedEventArgsFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IDragStartedEventArgsFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IDragStartedEventArgsFactoryVtbl {
    pub const fn new<Impl: IDragStartedEventArgsFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDragStartedEventArgsFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithHorizontalOffsetAndVerticalOffset<Impl: IDragStartedEventArgsFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, horizontaloffset: f64, verticaloffset: f64, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithHorizontalOffsetAndVerticalOffset(horizontaloffset, verticaloffset, &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDragStartedEventArgsFactory>, base.5, CreateInstanceWithHorizontalOffsetAndVerticalOffset::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlyoutBaseImpl: Sized {
    fn Placement(&self) -> ::windows::core::Result<FlyoutPlacementMode>;
    fn SetPlacement(&self, value: FlyoutPlacementMode) -> ::windows::core::Result<()>;
    fn Opened(&self, handler: &::core::option::Option<super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveOpened(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Closed(&self, handler: &::core::option::Option<super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Opening(&self, handler: &::core::option::Option<super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveOpening(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ShowAt(&self, placementtarget: &::core::option::Option<super::super::FrameworkElement>) -> ::windows::core::Result<()>;
    fn Hide(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFlyoutBase {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IFlyoutBase";
}
#[cfg(feature = "implement_exclusive")]
impl IFlyoutBaseVtbl {
    pub const fn new<Impl: IFlyoutBaseImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFlyoutBaseVtbl {
        unsafe extern "system" fn Placement<Impl: IFlyoutBaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut FlyoutPlacementMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Placement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlacement<Impl: IFlyoutBaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: FlyoutPlacementMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPlacement(value).into()
        }
        unsafe extern "system" fn Opened<Impl: IFlyoutBaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Opened(&*(&handler as *const <super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveOpened<Impl: IFlyoutBaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveOpened(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Closed<Impl: IFlyoutBaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Closed(&*(&handler as *const <super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveClosed<Impl: IFlyoutBaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveClosed(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Opening<Impl: IFlyoutBaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Opening(&*(&handler as *const <super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveOpening<Impl: IFlyoutBaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveOpening(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ShowAt<Impl: IFlyoutBaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, placementtarget: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ShowAt(&*(&placementtarget as *const <super::super::FrameworkElement as ::windows::core::Abi>::Abi as *const <super::super::FrameworkElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Hide<Impl: IFlyoutBaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Hide().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFlyoutBase>, base.5, Placement::<Impl, OFFSET>, SetPlacement::<Impl, OFFSET>, Opened::<Impl, OFFSET>, RemoveOpened::<Impl, OFFSET>, Closed::<Impl, OFFSET>, RemoveClosed::<Impl, OFFSET>, Opening::<Impl, OFFSET>, RemoveOpening::<Impl, OFFSET>, ShowAt::<Impl, OFFSET>, Hide::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlyoutBase2Impl: Sized {
    fn Target(&self) -> ::windows::core::Result<super::super::FrameworkElement>;
    fn AllowFocusOnInteraction(&self) -> ::windows::core::Result<bool>;
    fn SetAllowFocusOnInteraction(&self, value: bool) -> ::windows::core::Result<()>;
    fn LightDismissOverlayMode(&self) -> ::windows::core::Result<super::LightDismissOverlayMode>;
    fn SetLightDismissOverlayMode(&self, value: super::LightDismissOverlayMode) -> ::windows::core::Result<()>;
    fn AllowFocusWhenDisabled(&self) -> ::windows::core::Result<bool>;
    fn SetAllowFocusWhenDisabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn ElementSoundMode(&self) -> ::windows::core::Result<super::super::ElementSoundMode>;
    fn SetElementSoundMode(&self, value: super::super::ElementSoundMode) -> ::windows::core::Result<()>;
    fn Closing(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<FlyoutBase, FlyoutBaseClosingEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosing(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFlyoutBase2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IFlyoutBase2";
}
#[cfg(feature = "implement_exclusive")]
impl IFlyoutBase2Vtbl {
    pub const fn new<Impl: IFlyoutBase2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFlyoutBase2Vtbl {
        unsafe extern "system" fn Target<Impl: IFlyoutBase2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Target() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowFocusOnInteraction<Impl: IFlyoutBase2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllowFocusOnInteraction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowFocusOnInteraction<Impl: IFlyoutBase2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAllowFocusOnInteraction(value).into()
        }
        unsafe extern "system" fn LightDismissOverlayMode<Impl: IFlyoutBase2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::LightDismissOverlayMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LightDismissOverlayMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLightDismissOverlayMode<Impl: IFlyoutBase2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::LightDismissOverlayMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetLightDismissOverlayMode(value).into()
        }
        unsafe extern "system" fn AllowFocusWhenDisabled<Impl: IFlyoutBase2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllowFocusWhenDisabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowFocusWhenDisabled<Impl: IFlyoutBase2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAllowFocusWhenDisabled(value).into()
        }
        unsafe extern "system" fn ElementSoundMode<Impl: IFlyoutBase2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::ElementSoundMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ElementSoundMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetElementSoundMode<Impl: IFlyoutBase2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::ElementSoundMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetElementSoundMode(value).into()
        }
        unsafe extern "system" fn Closing<Impl: IFlyoutBase2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Closing(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<FlyoutBase, FlyoutBaseClosingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<FlyoutBase, FlyoutBaseClosingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveClosing<Impl: IFlyoutBase2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveClosing(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IFlyoutBase2>,
            base.5,
            Target::<Impl, OFFSET>,
            AllowFocusOnInteraction::<Impl, OFFSET>,
            SetAllowFocusOnInteraction::<Impl, OFFSET>,
            LightDismissOverlayMode::<Impl, OFFSET>,
            SetLightDismissOverlayMode::<Impl, OFFSET>,
            AllowFocusWhenDisabled::<Impl, OFFSET>,
            SetAllowFocusWhenDisabled::<Impl, OFFSET>,
            ElementSoundMode::<Impl, OFFSET>,
            SetElementSoundMode::<Impl, OFFSET>,
            Closing::<Impl, OFFSET>,
            RemoveClosing::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlyoutBase3Impl: Sized {
    fn OverlayInputPassThroughElement(&self) -> ::windows::core::Result<super::super::DependencyObject>;
    fn SetOverlayInputPassThroughElement(&self, value: &::core::option::Option<super::super::DependencyObject>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFlyoutBase3 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IFlyoutBase3";
}
#[cfg(feature = "implement_exclusive")]
impl IFlyoutBase3Vtbl {
    pub const fn new<Impl: IFlyoutBase3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFlyoutBase3Vtbl {
        unsafe extern "system" fn OverlayInputPassThroughElement<Impl: IFlyoutBase3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OverlayInputPassThroughElement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOverlayInputPassThroughElement<Impl: IFlyoutBase3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetOverlayInputPassThroughElement(&*(&value as *const <super::super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFlyoutBase3>, base.5, OverlayInputPassThroughElement::<Impl, OFFSET>, SetOverlayInputPassThroughElement::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlyoutBase4Impl: Sized {
    fn TryInvokeKeyboardAccelerator(&self, args: &::core::option::Option<super::super::Input::ProcessKeyboardAcceleratorEventArgs>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFlyoutBase4 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IFlyoutBase4";
}
#[cfg(feature = "implement_exclusive")]
impl IFlyoutBase4Vtbl {
    pub const fn new<Impl: IFlyoutBase4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFlyoutBase4Vtbl {
        unsafe extern "system" fn TryInvokeKeyboardAccelerator<Impl: IFlyoutBase4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).TryInvokeKeyboardAccelerator(&*(&args as *const <super::super::Input::ProcessKeyboardAcceleratorEventArgs as ::windows::core::Abi>::Abi as *const <super::super::Input::ProcessKeyboardAcceleratorEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFlyoutBase4>, base.5, TryInvokeKeyboardAccelerator::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlyoutBase5Impl: Sized {
    fn ShowMode(&self) -> ::windows::core::Result<FlyoutShowMode>;
    fn SetShowMode(&self, value: FlyoutShowMode) -> ::windows::core::Result<()>;
    fn InputDevicePrefersPrimaryCommands(&self) -> ::windows::core::Result<bool>;
    fn AreOpenCloseAnimationsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetAreOpenCloseAnimationsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsOpen(&self) -> ::windows::core::Result<bool>;
    fn ShowAt(&self, placementtarget: &::core::option::Option<super::super::DependencyObject>, showoptions: &::core::option::Option<FlyoutShowOptions>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFlyoutBase5 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IFlyoutBase5";
}
#[cfg(feature = "implement_exclusive")]
impl IFlyoutBase5Vtbl {
    pub const fn new<Impl: IFlyoutBase5Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFlyoutBase5Vtbl {
        unsafe extern "system" fn ShowMode<Impl: IFlyoutBase5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut FlyoutShowMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShowMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowMode<Impl: IFlyoutBase5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: FlyoutShowMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetShowMode(value).into()
        }
        unsafe extern "system" fn InputDevicePrefersPrimaryCommands<Impl: IFlyoutBase5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InputDevicePrefersPrimaryCommands() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AreOpenCloseAnimationsEnabled<Impl: IFlyoutBase5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AreOpenCloseAnimationsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAreOpenCloseAnimationsEnabled<Impl: IFlyoutBase5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAreOpenCloseAnimationsEnabled(value).into()
        }
        unsafe extern "system" fn IsOpen<Impl: IFlyoutBase5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsOpen() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowAt<Impl: IFlyoutBase5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, placementtarget: ::windows::core::RawPtr, showoptions: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ShowAt(&*(&placementtarget as *const <super::super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::super::DependencyObject as ::windows::core::DefaultType>::DefaultType), &*(&showoptions as *const <FlyoutShowOptions as ::windows::core::Abi>::Abi as *const <FlyoutShowOptions as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFlyoutBase5>, base.5, ShowMode::<Impl, OFFSET>, SetShowMode::<Impl, OFFSET>, InputDevicePrefersPrimaryCommands::<Impl, OFFSET>, AreOpenCloseAnimationsEnabled::<Impl, OFFSET>, SetAreOpenCloseAnimationsEnabled::<Impl, OFFSET>, IsOpen::<Impl, OFFSET>, ShowAt::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlyoutBase6Impl: Sized {
    fn ShouldConstrainToRootBounds(&self) -> ::windows::core::Result<bool>;
    fn SetShouldConstrainToRootBounds(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsConstrainedToRootBounds(&self) -> ::windows::core::Result<bool>;
    fn XamlRoot(&self) -> ::windows::core::Result<super::super::XamlRoot>;
    fn SetXamlRoot(&self, value: &::core::option::Option<super::super::XamlRoot>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFlyoutBase6 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IFlyoutBase6";
}
#[cfg(feature = "implement_exclusive")]
impl IFlyoutBase6Vtbl {
    pub const fn new<Impl: IFlyoutBase6Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFlyoutBase6Vtbl {
        unsafe extern "system" fn ShouldConstrainToRootBounds<Impl: IFlyoutBase6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShouldConstrainToRootBounds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShouldConstrainToRootBounds<Impl: IFlyoutBase6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetShouldConstrainToRootBounds(value).into()
        }
        unsafe extern "system" fn IsConstrainedToRootBounds<Impl: IFlyoutBase6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsConstrainedToRootBounds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn XamlRoot<Impl: IFlyoutBase6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).XamlRoot() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetXamlRoot<Impl: IFlyoutBase6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetXamlRoot(&*(&value as *const <super::super::XamlRoot as ::windows::core::Abi>::Abi as *const <super::super::XamlRoot as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFlyoutBase6>, base.5, ShouldConstrainToRootBounds::<Impl, OFFSET>, SetShouldConstrainToRootBounds::<Impl, OFFSET>, IsConstrainedToRootBounds::<Impl, OFFSET>, XamlRoot::<Impl, OFFSET>, SetXamlRoot::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlyoutBaseClosingEventArgsImpl: Sized {
    fn Cancel(&self) -> ::windows::core::Result<bool>;
    fn SetCancel(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFlyoutBaseClosingEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IFlyoutBaseClosingEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IFlyoutBaseClosingEventArgsVtbl {
    pub const fn new<Impl: IFlyoutBaseClosingEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFlyoutBaseClosingEventArgsVtbl {
        unsafe extern "system" fn Cancel<Impl: IFlyoutBaseClosingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCancel<Impl: IFlyoutBaseClosingEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCancel(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFlyoutBaseClosingEventArgs>, base.5, Cancel::<Impl, OFFSET>, SetCancel::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlyoutBaseFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<FlyoutBase>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFlyoutBaseFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IFlyoutBaseFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IFlyoutBaseFactoryVtbl {
    pub const fn new<Impl: IFlyoutBaseFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFlyoutBaseFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IFlyoutBaseFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFlyoutBaseFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlyoutBaseOverridesImpl: Sized {
    fn CreatePresenter(&self) -> ::windows::core::Result<super::Control>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFlyoutBaseOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IFlyoutBaseOverrides";
}
#[cfg(feature = "implement_exclusive")]
impl IFlyoutBaseOverridesVtbl {
    pub const fn new<Impl: IFlyoutBaseOverridesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFlyoutBaseOverridesVtbl {
        unsafe extern "system" fn CreatePresenter<Impl: IFlyoutBaseOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreatePresenter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFlyoutBaseOverrides>, base.5, CreatePresenter::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlyoutBaseOverrides4Impl: Sized {
    fn OnProcessKeyboardAccelerators(&self, args: &::core::option::Option<super::super::Input::ProcessKeyboardAcceleratorEventArgs>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFlyoutBaseOverrides4 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IFlyoutBaseOverrides4";
}
#[cfg(feature = "implement_exclusive")]
impl IFlyoutBaseOverrides4Vtbl {
    pub const fn new<Impl: IFlyoutBaseOverrides4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFlyoutBaseOverrides4Vtbl {
        unsafe extern "system" fn OnProcessKeyboardAccelerators<Impl: IFlyoutBaseOverrides4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).OnProcessKeyboardAccelerators(&*(&args as *const <super::super::Input::ProcessKeyboardAcceleratorEventArgs as ::windows::core::Abi>::Abi as *const <super::super::Input::ProcessKeyboardAcceleratorEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFlyoutBaseOverrides4>, base.5, OnProcessKeyboardAccelerators::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlyoutBaseStaticsImpl: Sized {
    fn PlacementProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn AttachedFlyoutProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn GetAttachedFlyout(&self, element: &::core::option::Option<super::super::FrameworkElement>) -> ::windows::core::Result<FlyoutBase>;
    fn SetAttachedFlyout(&self, element: &::core::option::Option<super::super::FrameworkElement>, value: &::core::option::Option<FlyoutBase>) -> ::windows::core::Result<()>;
    fn ShowAttachedFlyout(&self, flyoutowner: &::core::option::Option<super::super::FrameworkElement>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFlyoutBaseStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IFlyoutBaseStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IFlyoutBaseStaticsVtbl {
    pub const fn new<Impl: IFlyoutBaseStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFlyoutBaseStaticsVtbl {
        unsafe extern "system" fn PlacementProperty<Impl: IFlyoutBaseStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PlacementProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttachedFlyoutProperty<Impl: IFlyoutBaseStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AttachedFlyoutProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttachedFlyout<Impl: IFlyoutBaseStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAttachedFlyout(&*(&element as *const <super::super::FrameworkElement as ::windows::core::Abi>::Abi as *const <super::super::FrameworkElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttachedFlyout<Impl: IFlyoutBaseStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAttachedFlyout(&*(&element as *const <super::super::FrameworkElement as ::windows::core::Abi>::Abi as *const <super::super::FrameworkElement as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <FlyoutBase as ::windows::core::Abi>::Abi as *const <FlyoutBase as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ShowAttachedFlyout<Impl: IFlyoutBaseStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flyoutowner: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ShowAttachedFlyout(&*(&flyoutowner as *const <super::super::FrameworkElement as ::windows::core::Abi>::Abi as *const <super::super::FrameworkElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFlyoutBaseStatics>, base.5, PlacementProperty::<Impl, OFFSET>, AttachedFlyoutProperty::<Impl, OFFSET>, GetAttachedFlyout::<Impl, OFFSET>, SetAttachedFlyout::<Impl, OFFSET>, ShowAttachedFlyout::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlyoutBaseStatics2Impl: Sized {
    fn AllowFocusOnInteractionProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn LightDismissOverlayModeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn AllowFocusWhenDisabledProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ElementSoundModeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFlyoutBaseStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IFlyoutBaseStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IFlyoutBaseStatics2Vtbl {
    pub const fn new<Impl: IFlyoutBaseStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFlyoutBaseStatics2Vtbl {
        unsafe extern "system" fn AllowFocusOnInteractionProperty<Impl: IFlyoutBaseStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllowFocusOnInteractionProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LightDismissOverlayModeProperty<Impl: IFlyoutBaseStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LightDismissOverlayModeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowFocusWhenDisabledProperty<Impl: IFlyoutBaseStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllowFocusWhenDisabledProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ElementSoundModeProperty<Impl: IFlyoutBaseStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ElementSoundModeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFlyoutBaseStatics2>, base.5, AllowFocusOnInteractionProperty::<Impl, OFFSET>, LightDismissOverlayModeProperty::<Impl, OFFSET>, AllowFocusWhenDisabledProperty::<Impl, OFFSET>, ElementSoundModeProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlyoutBaseStatics3Impl: Sized {
    fn OverlayInputPassThroughElementProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFlyoutBaseStatics3 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IFlyoutBaseStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IFlyoutBaseStatics3Vtbl {
    pub const fn new<Impl: IFlyoutBaseStatics3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFlyoutBaseStatics3Vtbl {
        unsafe extern "system" fn OverlayInputPassThroughElementProperty<Impl: IFlyoutBaseStatics3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OverlayInputPassThroughElementProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFlyoutBaseStatics3>, base.5, OverlayInputPassThroughElementProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlyoutBaseStatics5Impl: Sized {
    fn TargetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ShowModeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn InputDevicePrefersPrimaryCommandsProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn AreOpenCloseAnimationsEnabledProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn IsOpenProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFlyoutBaseStatics5 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IFlyoutBaseStatics5";
}
#[cfg(feature = "implement_exclusive")]
impl IFlyoutBaseStatics5Vtbl {
    pub const fn new<Impl: IFlyoutBaseStatics5Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFlyoutBaseStatics5Vtbl {
        unsafe extern "system" fn TargetProperty<Impl: IFlyoutBaseStatics5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TargetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowModeProperty<Impl: IFlyoutBaseStatics5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShowModeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InputDevicePrefersPrimaryCommandsProperty<Impl: IFlyoutBaseStatics5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InputDevicePrefersPrimaryCommandsProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AreOpenCloseAnimationsEnabledProperty<Impl: IFlyoutBaseStatics5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AreOpenCloseAnimationsEnabledProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOpenProperty<Impl: IFlyoutBaseStatics5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsOpenProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFlyoutBaseStatics5>, base.5, TargetProperty::<Impl, OFFSET>, ShowModeProperty::<Impl, OFFSET>, InputDevicePrefersPrimaryCommandsProperty::<Impl, OFFSET>, AreOpenCloseAnimationsEnabledProperty::<Impl, OFFSET>, IsOpenProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlyoutBaseStatics6Impl: Sized {
    fn ShouldConstrainToRootBoundsProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFlyoutBaseStatics6 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IFlyoutBaseStatics6";
}
#[cfg(feature = "implement_exclusive")]
impl IFlyoutBaseStatics6Vtbl {
    pub const fn new<Impl: IFlyoutBaseStatics6Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFlyoutBaseStatics6Vtbl {
        unsafe extern "system" fn ShouldConstrainToRootBoundsProperty<Impl: IFlyoutBaseStatics6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShouldConstrainToRootBoundsProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFlyoutBaseStatics6>, base.5, ShouldConstrainToRootBoundsProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlyoutShowOptionsImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point>>;
    fn SetPosition(&self, value: &::core::option::Option<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point>>) -> ::windows::core::Result<()>;
    fn ExclusionRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Rect>>;
    fn SetExclusionRect(&self, value: &::core::option::Option<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Rect>>) -> ::windows::core::Result<()>;
    fn ShowMode(&self) -> ::windows::core::Result<FlyoutShowMode>;
    fn SetShowMode(&self, value: FlyoutShowMode) -> ::windows::core::Result<()>;
    fn Placement(&self) -> ::windows::core::Result<FlyoutPlacementMode>;
    fn SetPlacement(&self, value: FlyoutPlacementMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFlyoutShowOptions {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IFlyoutShowOptions";
}
#[cfg(feature = "implement_exclusive")]
impl IFlyoutShowOptionsVtbl {
    pub const fn new<Impl: IFlyoutShowOptionsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFlyoutShowOptionsVtbl {
        unsafe extern "system" fn Position<Impl: IFlyoutShowOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPosition<Impl: IFlyoutShowOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPosition(&*(&value as *const <super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ExclusionRect<Impl: IFlyoutShowOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExclusionRect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExclusionRect<Impl: IFlyoutShowOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetExclusionRect(&*(&value as *const <super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Rect> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Rect> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ShowMode<Impl: IFlyoutShowOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut FlyoutShowMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShowMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowMode<Impl: IFlyoutShowOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: FlyoutShowMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetShowMode(value).into()
        }
        unsafe extern "system" fn Placement<Impl: IFlyoutShowOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut FlyoutPlacementMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Placement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlacement<Impl: IFlyoutShowOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: FlyoutPlacementMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPlacement(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFlyoutShowOptions>, base.5, Position::<Impl, OFFSET>, SetPosition::<Impl, OFFSET>, ExclusionRect::<Impl, OFFSET>, SetExclusionRect::<Impl, OFFSET>, ShowMode::<Impl, OFFSET>, SetShowMode::<Impl, OFFSET>, Placement::<Impl, OFFSET>, SetPlacement::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFlyoutShowOptionsFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<FlyoutShowOptions>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFlyoutShowOptionsFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IFlyoutShowOptionsFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IFlyoutShowOptionsFactoryVtbl {
    pub const fn new<Impl: IFlyoutShowOptionsFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFlyoutShowOptionsFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IFlyoutShowOptionsFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFlyoutShowOptionsFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeneratorPositionHelperImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeneratorPositionHelper {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IGeneratorPositionHelper";
}
#[cfg(feature = "implement_exclusive")]
impl IGeneratorPositionHelperVtbl {
    pub const fn new<Impl: IGeneratorPositionHelperImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGeneratorPositionHelperVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGeneratorPositionHelper>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeneratorPositionHelperStaticsImpl: Sized {
    fn FromIndexAndOffset(&self, index: i32, offset: i32) -> ::windows::core::Result<GeneratorPosition>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeneratorPositionHelperStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IGeneratorPositionHelperStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGeneratorPositionHelperStaticsVtbl {
    pub const fn new<Impl: IGeneratorPositionHelperStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGeneratorPositionHelperStaticsVtbl {
        unsafe extern "system" fn FromIndexAndOffset<Impl: IGeneratorPositionHelperStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, offset: i32, result__: *mut GeneratorPosition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromIndexAndOffset(index, offset) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGeneratorPositionHelperStatics>, base.5, FromIndexAndOffset::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGridViewItemPresenterImpl: Sized {
    fn SelectionCheckMarkVisualEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetSelectionCheckMarkVisualEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn CheckHintBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckHintBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckSelectingBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckSelectingBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn DragBackground(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetDragBackground(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn DragForeground(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetDragForeground(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn FocusBorderBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetFocusBorderBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn PlaceholderBackground(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetPlaceholderBackground(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn PointerOverBackground(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetPointerOverBackground(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectedBackground(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetSelectedBackground(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectedForeground(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetSelectedForeground(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectedPointerOverBackground(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetSelectedPointerOverBackground(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectedPointerOverBorderBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetSelectedPointerOverBorderBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectedBorderThickness(&self) -> ::windows::core::Result<super::super::Thickness>;
    fn SetSelectedBorderThickness(&self, value: &super::super::Thickness) -> ::windows::core::Result<()>;
    fn DisabledOpacity(&self) -> ::windows::core::Result<f64>;
    fn SetDisabledOpacity(&self, value: f64) -> ::windows::core::Result<()>;
    fn DragOpacity(&self) -> ::windows::core::Result<f64>;
    fn SetDragOpacity(&self, value: f64) -> ::windows::core::Result<()>;
    fn ReorderHintOffset(&self) -> ::windows::core::Result<f64>;
    fn SetReorderHintOffset(&self, value: f64) -> ::windows::core::Result<()>;
    fn GridViewItemPresenterHorizontalContentAlignment(&self) -> ::windows::core::Result<super::super::HorizontalAlignment>;
    fn SetGridViewItemPresenterHorizontalContentAlignment(&self, value: super::super::HorizontalAlignment) -> ::windows::core::Result<()>;
    fn GridViewItemPresenterVerticalContentAlignment(&self) -> ::windows::core::Result<super::super::VerticalAlignment>;
    fn SetGridViewItemPresenterVerticalContentAlignment(&self, value: super::super::VerticalAlignment) -> ::windows::core::Result<()>;
    fn GridViewItemPresenterPadding(&self) -> ::windows::core::Result<super::super::Thickness>;
    fn SetGridViewItemPresenterPadding(&self, value: &super::super::Thickness) -> ::windows::core::Result<()>;
    fn PointerOverBackgroundMargin(&self) -> ::windows::core::Result<super::super::Thickness>;
    fn SetPointerOverBackgroundMargin(&self, value: &super::super::Thickness) -> ::windows::core::Result<()>;
    fn ContentMargin(&self) -> ::windows::core::Result<super::super::Thickness>;
    fn SetContentMargin(&self, value: &super::super::Thickness) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGridViewItemPresenter {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IGridViewItemPresenter";
}
#[cfg(feature = "implement_exclusive")]
impl IGridViewItemPresenterVtbl {
    pub const fn new<Impl: IGridViewItemPresenterImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGridViewItemPresenterVtbl {
        unsafe extern "system" fn SelectionCheckMarkVisualEnabled<Impl: IGridViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectionCheckMarkVisualEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectionCheckMarkVisualEnabled<Impl: IGridViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSelectionCheckMarkVisualEnabled(value).into()
        }
        unsafe extern "system" fn CheckHintBrush<Impl: IGridViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckHintBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCheckHintBrush<Impl: IGridViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCheckHintBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckSelectingBrush<Impl: IGridViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckSelectingBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCheckSelectingBrush<Impl: IGridViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCheckSelectingBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckBrush<Impl: IGridViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCheckBrush<Impl: IGridViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCheckBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DragBackground<Impl: IGridViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DragBackground() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDragBackground<Impl: IGridViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDragBackground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DragForeground<Impl: IGridViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DragForeground() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDragForeground<Impl: IGridViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDragForeground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FocusBorderBrush<Impl: IGridViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FocusBorderBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFocusBorderBrush<Impl: IGridViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFocusBorderBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PlaceholderBackground<Impl: IGridViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PlaceholderBackground() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlaceholderBackground<Impl: IGridViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPlaceholderBackground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerOverBackground<Impl: IGridViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointerOverBackground() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPointerOverBackground<Impl: IGridViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPointerOverBackground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedBackground<Impl: IGridViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectedBackground() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedBackground<Impl: IGridViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSelectedBackground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedForeground<Impl: IGridViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectedForeground() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedForeground<Impl: IGridViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSelectedForeground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedPointerOverBackground<Impl: IGridViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectedPointerOverBackground() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedPointerOverBackground<Impl: IGridViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSelectedPointerOverBackground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedPointerOverBorderBrush<Impl: IGridViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectedPointerOverBorderBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedPointerOverBorderBrush<Impl: IGridViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSelectedPointerOverBorderBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedBorderThickness<Impl: IGridViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectedBorderThickness() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedBorderThickness<Impl: IGridViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSelectedBorderThickness(&*(&value as *const <super::super::Thickness as ::windows::core::Abi>::Abi as *const <super::super::Thickness as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DisabledOpacity<Impl: IGridViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisabledOpacity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisabledOpacity<Impl: IGridViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDisabledOpacity(value).into()
        }
        unsafe extern "system" fn DragOpacity<Impl: IGridViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DragOpacity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDragOpacity<Impl: IGridViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDragOpacity(value).into()
        }
        unsafe extern "system" fn ReorderHintOffset<Impl: IGridViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReorderHintOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReorderHintOffset<Impl: IGridViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetReorderHintOffset(value).into()
        }
        unsafe extern "system" fn GridViewItemPresenterHorizontalContentAlignment<Impl: IGridViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::HorizontalAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GridViewItemPresenterHorizontalContentAlignment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGridViewItemPresenterHorizontalContentAlignment<Impl: IGridViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::HorizontalAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetGridViewItemPresenterHorizontalContentAlignment(value).into()
        }
        unsafe extern "system" fn GridViewItemPresenterVerticalContentAlignment<Impl: IGridViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::VerticalAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GridViewItemPresenterVerticalContentAlignment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGridViewItemPresenterVerticalContentAlignment<Impl: IGridViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::VerticalAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetGridViewItemPresenterVerticalContentAlignment(value).into()
        }
        unsafe extern "system" fn GridViewItemPresenterPadding<Impl: IGridViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GridViewItemPresenterPadding() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGridViewItemPresenterPadding<Impl: IGridViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetGridViewItemPresenterPadding(&*(&value as *const <super::super::Thickness as ::windows::core::Abi>::Abi as *const <super::super::Thickness as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerOverBackgroundMargin<Impl: IGridViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointerOverBackgroundMargin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPointerOverBackgroundMargin<Impl: IGridViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPointerOverBackgroundMargin(&*(&value as *const <super::super::Thickness as ::windows::core::Abi>::Abi as *const <super::super::Thickness as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContentMargin<Impl: IGridViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContentMargin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentMargin<Impl: IGridViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetContentMargin(&*(&value as *const <super::super::Thickness as ::windows::core::Abi>::Abi as *const <super::super::Thickness as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IGridViewItemPresenter>,
            base.5,
            SelectionCheckMarkVisualEnabled::<Impl, OFFSET>,
            SetSelectionCheckMarkVisualEnabled::<Impl, OFFSET>,
            CheckHintBrush::<Impl, OFFSET>,
            SetCheckHintBrush::<Impl, OFFSET>,
            CheckSelectingBrush::<Impl, OFFSET>,
            SetCheckSelectingBrush::<Impl, OFFSET>,
            CheckBrush::<Impl, OFFSET>,
            SetCheckBrush::<Impl, OFFSET>,
            DragBackground::<Impl, OFFSET>,
            SetDragBackground::<Impl, OFFSET>,
            DragForeground::<Impl, OFFSET>,
            SetDragForeground::<Impl, OFFSET>,
            FocusBorderBrush::<Impl, OFFSET>,
            SetFocusBorderBrush::<Impl, OFFSET>,
            PlaceholderBackground::<Impl, OFFSET>,
            SetPlaceholderBackground::<Impl, OFFSET>,
            PointerOverBackground::<Impl, OFFSET>,
            SetPointerOverBackground::<Impl, OFFSET>,
            SelectedBackground::<Impl, OFFSET>,
            SetSelectedBackground::<Impl, OFFSET>,
            SelectedForeground::<Impl, OFFSET>,
            SetSelectedForeground::<Impl, OFFSET>,
            SelectedPointerOverBackground::<Impl, OFFSET>,
            SetSelectedPointerOverBackground::<Impl, OFFSET>,
            SelectedPointerOverBorderBrush::<Impl, OFFSET>,
            SetSelectedPointerOverBorderBrush::<Impl, OFFSET>,
            SelectedBorderThickness::<Impl, OFFSET>,
            SetSelectedBorderThickness::<Impl, OFFSET>,
            DisabledOpacity::<Impl, OFFSET>,
            SetDisabledOpacity::<Impl, OFFSET>,
            DragOpacity::<Impl, OFFSET>,
            SetDragOpacity::<Impl, OFFSET>,
            ReorderHintOffset::<Impl, OFFSET>,
            SetReorderHintOffset::<Impl, OFFSET>,
            GridViewItemPresenterHorizontalContentAlignment::<Impl, OFFSET>,
            SetGridViewItemPresenterHorizontalContentAlignment::<Impl, OFFSET>,
            GridViewItemPresenterVerticalContentAlignment::<Impl, OFFSET>,
            SetGridViewItemPresenterVerticalContentAlignment::<Impl, OFFSET>,
            GridViewItemPresenterPadding::<Impl, OFFSET>,
            SetGridViewItemPresenterPadding::<Impl, OFFSET>,
            PointerOverBackgroundMargin::<Impl, OFFSET>,
            SetPointerOverBackgroundMargin::<Impl, OFFSET>,
            ContentMargin::<Impl, OFFSET>,
            SetContentMargin::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGridViewItemPresenterFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<GridViewItemPresenter>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGridViewItemPresenterFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IGridViewItemPresenterFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IGridViewItemPresenterFactoryVtbl {
    pub const fn new<Impl: IGridViewItemPresenterFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGridViewItemPresenterFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IGridViewItemPresenterFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGridViewItemPresenterFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGridViewItemPresenterStaticsImpl: Sized {
    fn SelectionCheckMarkVisualEnabledProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckHintBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckSelectingBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn DragBackgroundProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn DragForegroundProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn FocusBorderBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn PlaceholderBackgroundProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn PointerOverBackgroundProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedBackgroundProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedForegroundProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedPointerOverBackgroundProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedPointerOverBorderBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedBorderThicknessProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn DisabledOpacityProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn DragOpacityProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ReorderHintOffsetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn GridViewItemPresenterHorizontalContentAlignmentProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn GridViewItemPresenterVerticalContentAlignmentProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn GridViewItemPresenterPaddingProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn PointerOverBackgroundMarginProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ContentMarginProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGridViewItemPresenterStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IGridViewItemPresenterStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGridViewItemPresenterStaticsVtbl {
    pub const fn new<Impl: IGridViewItemPresenterStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGridViewItemPresenterStaticsVtbl {
        unsafe extern "system" fn SelectionCheckMarkVisualEnabledProperty<Impl: IGridViewItemPresenterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectionCheckMarkVisualEnabledProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckHintBrushProperty<Impl: IGridViewItemPresenterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckHintBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckSelectingBrushProperty<Impl: IGridViewItemPresenterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckSelectingBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckBrushProperty<Impl: IGridViewItemPresenterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DragBackgroundProperty<Impl: IGridViewItemPresenterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DragBackgroundProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DragForegroundProperty<Impl: IGridViewItemPresenterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DragForegroundProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FocusBorderBrushProperty<Impl: IGridViewItemPresenterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FocusBorderBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PlaceholderBackgroundProperty<Impl: IGridViewItemPresenterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PlaceholderBackgroundProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PointerOverBackgroundProperty<Impl: IGridViewItemPresenterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointerOverBackgroundProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedBackgroundProperty<Impl: IGridViewItemPresenterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectedBackgroundProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedForegroundProperty<Impl: IGridViewItemPresenterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectedForegroundProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedPointerOverBackgroundProperty<Impl: IGridViewItemPresenterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectedPointerOverBackgroundProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedPointerOverBorderBrushProperty<Impl: IGridViewItemPresenterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectedPointerOverBorderBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedBorderThicknessProperty<Impl: IGridViewItemPresenterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectedBorderThicknessProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisabledOpacityProperty<Impl: IGridViewItemPresenterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisabledOpacityProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DragOpacityProperty<Impl: IGridViewItemPresenterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DragOpacityProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReorderHintOffsetProperty<Impl: IGridViewItemPresenterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReorderHintOffsetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GridViewItemPresenterHorizontalContentAlignmentProperty<Impl: IGridViewItemPresenterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GridViewItemPresenterHorizontalContentAlignmentProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GridViewItemPresenterVerticalContentAlignmentProperty<Impl: IGridViewItemPresenterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GridViewItemPresenterVerticalContentAlignmentProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GridViewItemPresenterPaddingProperty<Impl: IGridViewItemPresenterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GridViewItemPresenterPaddingProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PointerOverBackgroundMarginProperty<Impl: IGridViewItemPresenterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointerOverBackgroundMarginProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentMarginProperty<Impl: IGridViewItemPresenterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContentMarginProperty() {
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
            ::windows::core::GetRuntimeClassName::<IGridViewItemPresenterStatics>,
            base.5,
            SelectionCheckMarkVisualEnabledProperty::<Impl, OFFSET>,
            CheckHintBrushProperty::<Impl, OFFSET>,
            CheckSelectingBrushProperty::<Impl, OFFSET>,
            CheckBrushProperty::<Impl, OFFSET>,
            DragBackgroundProperty::<Impl, OFFSET>,
            DragForegroundProperty::<Impl, OFFSET>,
            FocusBorderBrushProperty::<Impl, OFFSET>,
            PlaceholderBackgroundProperty::<Impl, OFFSET>,
            PointerOverBackgroundProperty::<Impl, OFFSET>,
            SelectedBackgroundProperty::<Impl, OFFSET>,
            SelectedForegroundProperty::<Impl, OFFSET>,
            SelectedPointerOverBackgroundProperty::<Impl, OFFSET>,
            SelectedPointerOverBorderBrushProperty::<Impl, OFFSET>,
            SelectedBorderThicknessProperty::<Impl, OFFSET>,
            DisabledOpacityProperty::<Impl, OFFSET>,
            DragOpacityProperty::<Impl, OFFSET>,
            ReorderHintOffsetProperty::<Impl, OFFSET>,
            GridViewItemPresenterHorizontalContentAlignmentProperty::<Impl, OFFSET>,
            GridViewItemPresenterVerticalContentAlignmentProperty::<Impl, OFFSET>,
            GridViewItemPresenterPaddingProperty::<Impl, OFFSET>,
            PointerOverBackgroundMarginProperty::<Impl, OFFSET>,
            ContentMarginProperty::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGridViewItemTemplateSettingsImpl: Sized {
    fn DragItemsCount(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGridViewItemTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IGridViewItemTemplateSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IGridViewItemTemplateSettingsVtbl {
    pub const fn new<Impl: IGridViewItemTemplateSettingsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGridViewItemTemplateSettingsVtbl {
        unsafe extern "system" fn DragItemsCount<Impl: IGridViewItemTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DragItemsCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGridViewItemTemplateSettings>, base.5, DragItemsCount::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IItemsChangedEventArgsImpl: Sized {
    fn Action(&self) -> ::windows::core::Result<i32>;
    fn Position(&self) -> ::windows::core::Result<GeneratorPosition>;
    fn OldPosition(&self) -> ::windows::core::Result<GeneratorPosition>;
    fn ItemCount(&self) -> ::windows::core::Result<i32>;
    fn ItemUICount(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IItemsChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IItemsChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IItemsChangedEventArgsVtbl {
    pub const fn new<Impl: IItemsChangedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IItemsChangedEventArgsVtbl {
        unsafe extern "system" fn Action<Impl: IItemsChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Action() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Impl: IItemsChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut GeneratorPosition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OldPosition<Impl: IItemsChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut GeneratorPosition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OldPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemCount<Impl: IItemsChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ItemCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemUICount<Impl: IItemsChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ItemUICount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IItemsChangedEventArgs>, base.5, Action::<Impl, OFFSET>, Position::<Impl, OFFSET>, OldPosition::<Impl, OFFSET>, ItemCount::<Impl, OFFSET>, ItemUICount::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IJumpListItemBackgroundConverterImpl: Sized {
    fn Enabled(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetEnabled(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn Disabled(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetDisabled(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IJumpListItemBackgroundConverter {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IJumpListItemBackgroundConverter";
}
#[cfg(feature = "implement_exclusive")]
impl IJumpListItemBackgroundConverterVtbl {
    pub const fn new<Impl: IJumpListItemBackgroundConverterImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IJumpListItemBackgroundConverterVtbl {
        unsafe extern "system" fn Enabled<Impl: IJumpListItemBackgroundConverterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Impl: IJumpListItemBackgroundConverterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetEnabled(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Disabled<Impl: IJumpListItemBackgroundConverterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Disabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisabled<Impl: IJumpListItemBackgroundConverterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDisabled(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IJumpListItemBackgroundConverter>, base.5, Enabled::<Impl, OFFSET>, SetEnabled::<Impl, OFFSET>, Disabled::<Impl, OFFSET>, SetDisabled::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IJumpListItemBackgroundConverterStaticsImpl: Sized {
    fn EnabledProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn DisabledProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IJumpListItemBackgroundConverterStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IJumpListItemBackgroundConverterStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IJumpListItemBackgroundConverterStaticsVtbl {
    pub const fn new<Impl: IJumpListItemBackgroundConverterStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IJumpListItemBackgroundConverterStaticsVtbl {
        unsafe extern "system" fn EnabledProperty<Impl: IJumpListItemBackgroundConverterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnabledProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisabledProperty<Impl: IJumpListItemBackgroundConverterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisabledProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IJumpListItemBackgroundConverterStatics>, base.5, EnabledProperty::<Impl, OFFSET>, DisabledProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IJumpListItemForegroundConverterImpl: Sized {
    fn Enabled(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetEnabled(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn Disabled(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetDisabled(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IJumpListItemForegroundConverter {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IJumpListItemForegroundConverter";
}
#[cfg(feature = "implement_exclusive")]
impl IJumpListItemForegroundConverterVtbl {
    pub const fn new<Impl: IJumpListItemForegroundConverterImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IJumpListItemForegroundConverterVtbl {
        unsafe extern "system" fn Enabled<Impl: IJumpListItemForegroundConverterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Impl: IJumpListItemForegroundConverterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetEnabled(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Disabled<Impl: IJumpListItemForegroundConverterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Disabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisabled<Impl: IJumpListItemForegroundConverterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDisabled(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IJumpListItemForegroundConverter>, base.5, Enabled::<Impl, OFFSET>, SetEnabled::<Impl, OFFSET>, Disabled::<Impl, OFFSET>, SetDisabled::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IJumpListItemForegroundConverterStaticsImpl: Sized {
    fn EnabledProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn DisabledProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IJumpListItemForegroundConverterStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IJumpListItemForegroundConverterStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IJumpListItemForegroundConverterStaticsVtbl {
    pub const fn new<Impl: IJumpListItemForegroundConverterStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IJumpListItemForegroundConverterStaticsVtbl {
        unsafe extern "system" fn EnabledProperty<Impl: IJumpListItemForegroundConverterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnabledProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisabledProperty<Impl: IJumpListItemForegroundConverterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisabledProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IJumpListItemForegroundConverterStatics>, base.5, EnabledProperty::<Impl, OFFSET>, DisabledProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILayoutInformationImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILayoutInformation {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ILayoutInformation";
}
#[cfg(feature = "implement_exclusive")]
impl ILayoutInformationVtbl {
    pub const fn new<Impl: ILayoutInformationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILayoutInformationVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILayoutInformation>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILayoutInformationStaticsImpl: Sized {
    fn GetLayoutExceptionElement(&self, dispatcher: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<super::super::UIElement>;
    fn GetLayoutSlot(&self, element: &::core::option::Option<super::super::FrameworkElement>) -> ::windows::core::Result<super::super::super::super::Foundation::Rect>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILayoutInformationStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ILayoutInformationStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ILayoutInformationStaticsVtbl {
    pub const fn new<Impl: ILayoutInformationStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILayoutInformationStaticsVtbl {
        unsafe extern "system" fn GetLayoutExceptionElement<Impl: ILayoutInformationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dispatcher: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLayoutExceptionElement(&*(&dispatcher as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLayoutSlot<Impl: ILayoutInformationStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLayoutSlot(&*(&element as *const <super::super::FrameworkElement as ::windows::core::Abi>::Abi as *const <super::super::FrameworkElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILayoutInformationStatics>, base.5, GetLayoutExceptionElement::<Impl, OFFSET>, GetLayoutSlot::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILayoutInformationStatics2Impl: Sized {
    fn GetAvailableSize(&self, element: &::core::option::Option<super::super::UIElement>) -> ::windows::core::Result<super::super::super::super::Foundation::Size>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILayoutInformationStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ILayoutInformationStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl ILayoutInformationStatics2Vtbl {
    pub const fn new<Impl: ILayoutInformationStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILayoutInformationStatics2Vtbl {
        unsafe extern "system" fn GetAvailableSize<Impl: ILayoutInformationStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAvailableSize(&*(&element as *const <super::super::UIElement as ::windows::core::Abi>::Abi as *const <super::super::UIElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILayoutInformationStatics2>, base.5, GetAvailableSize::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewItemPresenterImpl: Sized {
    fn SelectionCheckMarkVisualEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetSelectionCheckMarkVisualEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn CheckHintBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckHintBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckSelectingBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckSelectingBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn DragBackground(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetDragBackground(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn DragForeground(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetDragForeground(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn FocusBorderBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetFocusBorderBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn PlaceholderBackground(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetPlaceholderBackground(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn PointerOverBackground(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetPointerOverBackground(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectedBackground(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetSelectedBackground(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectedForeground(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetSelectedForeground(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectedPointerOverBackground(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetSelectedPointerOverBackground(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectedPointerOverBorderBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetSelectedPointerOverBorderBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectedBorderThickness(&self) -> ::windows::core::Result<super::super::Thickness>;
    fn SetSelectedBorderThickness(&self, value: &super::super::Thickness) -> ::windows::core::Result<()>;
    fn DisabledOpacity(&self) -> ::windows::core::Result<f64>;
    fn SetDisabledOpacity(&self, value: f64) -> ::windows::core::Result<()>;
    fn DragOpacity(&self) -> ::windows::core::Result<f64>;
    fn SetDragOpacity(&self, value: f64) -> ::windows::core::Result<()>;
    fn ReorderHintOffset(&self) -> ::windows::core::Result<f64>;
    fn SetReorderHintOffset(&self, value: f64) -> ::windows::core::Result<()>;
    fn ListViewItemPresenterHorizontalContentAlignment(&self) -> ::windows::core::Result<super::super::HorizontalAlignment>;
    fn SetListViewItemPresenterHorizontalContentAlignment(&self, value: super::super::HorizontalAlignment) -> ::windows::core::Result<()>;
    fn ListViewItemPresenterVerticalContentAlignment(&self) -> ::windows::core::Result<super::super::VerticalAlignment>;
    fn SetListViewItemPresenterVerticalContentAlignment(&self, value: super::super::VerticalAlignment) -> ::windows::core::Result<()>;
    fn ListViewItemPresenterPadding(&self) -> ::windows::core::Result<super::super::Thickness>;
    fn SetListViewItemPresenterPadding(&self, value: &super::super::Thickness) -> ::windows::core::Result<()>;
    fn PointerOverBackgroundMargin(&self) -> ::windows::core::Result<super::super::Thickness>;
    fn SetPointerOverBackgroundMargin(&self, value: &super::super::Thickness) -> ::windows::core::Result<()>;
    fn ContentMargin(&self) -> ::windows::core::Result<super::super::Thickness>;
    fn SetContentMargin(&self, value: &super::super::Thickness) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListViewItemPresenter {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IListViewItemPresenter";
}
#[cfg(feature = "implement_exclusive")]
impl IListViewItemPresenterVtbl {
    pub const fn new<Impl: IListViewItemPresenterImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IListViewItemPresenterVtbl {
        unsafe extern "system" fn SelectionCheckMarkVisualEnabled<Impl: IListViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectionCheckMarkVisualEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectionCheckMarkVisualEnabled<Impl: IListViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSelectionCheckMarkVisualEnabled(value).into()
        }
        unsafe extern "system" fn CheckHintBrush<Impl: IListViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckHintBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCheckHintBrush<Impl: IListViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCheckHintBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckSelectingBrush<Impl: IListViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckSelectingBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCheckSelectingBrush<Impl: IListViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCheckSelectingBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckBrush<Impl: IListViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCheckBrush<Impl: IListViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCheckBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DragBackground<Impl: IListViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DragBackground() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDragBackground<Impl: IListViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDragBackground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DragForeground<Impl: IListViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DragForeground() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDragForeground<Impl: IListViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDragForeground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FocusBorderBrush<Impl: IListViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FocusBorderBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFocusBorderBrush<Impl: IListViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFocusBorderBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PlaceholderBackground<Impl: IListViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PlaceholderBackground() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlaceholderBackground<Impl: IListViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPlaceholderBackground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerOverBackground<Impl: IListViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointerOverBackground() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPointerOverBackground<Impl: IListViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPointerOverBackground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedBackground<Impl: IListViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectedBackground() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedBackground<Impl: IListViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSelectedBackground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedForeground<Impl: IListViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectedForeground() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedForeground<Impl: IListViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSelectedForeground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedPointerOverBackground<Impl: IListViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectedPointerOverBackground() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedPointerOverBackground<Impl: IListViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSelectedPointerOverBackground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedPointerOverBorderBrush<Impl: IListViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectedPointerOverBorderBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedPointerOverBorderBrush<Impl: IListViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSelectedPointerOverBorderBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedBorderThickness<Impl: IListViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectedBorderThickness() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedBorderThickness<Impl: IListViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSelectedBorderThickness(&*(&value as *const <super::super::Thickness as ::windows::core::Abi>::Abi as *const <super::super::Thickness as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DisabledOpacity<Impl: IListViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisabledOpacity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisabledOpacity<Impl: IListViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDisabledOpacity(value).into()
        }
        unsafe extern "system" fn DragOpacity<Impl: IListViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DragOpacity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDragOpacity<Impl: IListViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDragOpacity(value).into()
        }
        unsafe extern "system" fn ReorderHintOffset<Impl: IListViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReorderHintOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReorderHintOffset<Impl: IListViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetReorderHintOffset(value).into()
        }
        unsafe extern "system" fn ListViewItemPresenterHorizontalContentAlignment<Impl: IListViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::HorizontalAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ListViewItemPresenterHorizontalContentAlignment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListViewItemPresenterHorizontalContentAlignment<Impl: IListViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::HorizontalAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetListViewItemPresenterHorizontalContentAlignment(value).into()
        }
        unsafe extern "system" fn ListViewItemPresenterVerticalContentAlignment<Impl: IListViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::VerticalAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ListViewItemPresenterVerticalContentAlignment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListViewItemPresenterVerticalContentAlignment<Impl: IListViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::VerticalAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetListViewItemPresenterVerticalContentAlignment(value).into()
        }
        unsafe extern "system" fn ListViewItemPresenterPadding<Impl: IListViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ListViewItemPresenterPadding() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListViewItemPresenterPadding<Impl: IListViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetListViewItemPresenterPadding(&*(&value as *const <super::super::Thickness as ::windows::core::Abi>::Abi as *const <super::super::Thickness as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerOverBackgroundMargin<Impl: IListViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointerOverBackgroundMargin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPointerOverBackgroundMargin<Impl: IListViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPointerOverBackgroundMargin(&*(&value as *const <super::super::Thickness as ::windows::core::Abi>::Abi as *const <super::super::Thickness as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContentMargin<Impl: IListViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContentMargin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentMargin<Impl: IListViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetContentMargin(&*(&value as *const <super::super::Thickness as ::windows::core::Abi>::Abi as *const <super::super::Thickness as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IListViewItemPresenter>,
            base.5,
            SelectionCheckMarkVisualEnabled::<Impl, OFFSET>,
            SetSelectionCheckMarkVisualEnabled::<Impl, OFFSET>,
            CheckHintBrush::<Impl, OFFSET>,
            SetCheckHintBrush::<Impl, OFFSET>,
            CheckSelectingBrush::<Impl, OFFSET>,
            SetCheckSelectingBrush::<Impl, OFFSET>,
            CheckBrush::<Impl, OFFSET>,
            SetCheckBrush::<Impl, OFFSET>,
            DragBackground::<Impl, OFFSET>,
            SetDragBackground::<Impl, OFFSET>,
            DragForeground::<Impl, OFFSET>,
            SetDragForeground::<Impl, OFFSET>,
            FocusBorderBrush::<Impl, OFFSET>,
            SetFocusBorderBrush::<Impl, OFFSET>,
            PlaceholderBackground::<Impl, OFFSET>,
            SetPlaceholderBackground::<Impl, OFFSET>,
            PointerOverBackground::<Impl, OFFSET>,
            SetPointerOverBackground::<Impl, OFFSET>,
            SelectedBackground::<Impl, OFFSET>,
            SetSelectedBackground::<Impl, OFFSET>,
            SelectedForeground::<Impl, OFFSET>,
            SetSelectedForeground::<Impl, OFFSET>,
            SelectedPointerOverBackground::<Impl, OFFSET>,
            SetSelectedPointerOverBackground::<Impl, OFFSET>,
            SelectedPointerOverBorderBrush::<Impl, OFFSET>,
            SetSelectedPointerOverBorderBrush::<Impl, OFFSET>,
            SelectedBorderThickness::<Impl, OFFSET>,
            SetSelectedBorderThickness::<Impl, OFFSET>,
            DisabledOpacity::<Impl, OFFSET>,
            SetDisabledOpacity::<Impl, OFFSET>,
            DragOpacity::<Impl, OFFSET>,
            SetDragOpacity::<Impl, OFFSET>,
            ReorderHintOffset::<Impl, OFFSET>,
            SetReorderHintOffset::<Impl, OFFSET>,
            ListViewItemPresenterHorizontalContentAlignment::<Impl, OFFSET>,
            SetListViewItemPresenterHorizontalContentAlignment::<Impl, OFFSET>,
            ListViewItemPresenterVerticalContentAlignment::<Impl, OFFSET>,
            SetListViewItemPresenterVerticalContentAlignment::<Impl, OFFSET>,
            ListViewItemPresenterPadding::<Impl, OFFSET>,
            SetListViewItemPresenterPadding::<Impl, OFFSET>,
            PointerOverBackgroundMargin::<Impl, OFFSET>,
            SetPointerOverBackgroundMargin::<Impl, OFFSET>,
            ContentMargin::<Impl, OFFSET>,
            SetContentMargin::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewItemPresenter2Impl: Sized {
    fn SelectedPressedBackground(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetSelectedPressedBackground(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn PressedBackground(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetPressedBackground(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckBoxBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckBoxBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn FocusSecondaryBorderBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetFocusSecondaryBorderBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckMode(&self) -> ::windows::core::Result<ListViewItemPresenterCheckMode>;
    fn SetCheckMode(&self, value: ListViewItemPresenterCheckMode) -> ::windows::core::Result<()>;
    fn PointerOverForeground(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetPointerOverForeground(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListViewItemPresenter2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IListViewItemPresenter2";
}
#[cfg(feature = "implement_exclusive")]
impl IListViewItemPresenter2Vtbl {
    pub const fn new<Impl: IListViewItemPresenter2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IListViewItemPresenter2Vtbl {
        unsafe extern "system" fn SelectedPressedBackground<Impl: IListViewItemPresenter2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectedPressedBackground() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedPressedBackground<Impl: IListViewItemPresenter2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSelectedPressedBackground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PressedBackground<Impl: IListViewItemPresenter2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PressedBackground() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPressedBackground<Impl: IListViewItemPresenter2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPressedBackground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckBoxBrush<Impl: IListViewItemPresenter2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckBoxBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCheckBoxBrush<Impl: IListViewItemPresenter2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCheckBoxBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FocusSecondaryBorderBrush<Impl: IListViewItemPresenter2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FocusSecondaryBorderBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFocusSecondaryBorderBrush<Impl: IListViewItemPresenter2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFocusSecondaryBorderBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckMode<Impl: IListViewItemPresenter2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ListViewItemPresenterCheckMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCheckMode<Impl: IListViewItemPresenter2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ListViewItemPresenterCheckMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCheckMode(value).into()
        }
        unsafe extern "system" fn PointerOverForeground<Impl: IListViewItemPresenter2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointerOverForeground() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPointerOverForeground<Impl: IListViewItemPresenter2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPointerOverForeground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IListViewItemPresenter2>,
            base.5,
            SelectedPressedBackground::<Impl, OFFSET>,
            SetSelectedPressedBackground::<Impl, OFFSET>,
            PressedBackground::<Impl, OFFSET>,
            SetPressedBackground::<Impl, OFFSET>,
            CheckBoxBrush::<Impl, OFFSET>,
            SetCheckBoxBrush::<Impl, OFFSET>,
            FocusSecondaryBorderBrush::<Impl, OFFSET>,
            SetFocusSecondaryBorderBrush::<Impl, OFFSET>,
            CheckMode::<Impl, OFFSET>,
            SetCheckMode::<Impl, OFFSET>,
            PointerOverForeground::<Impl, OFFSET>,
            SetPointerOverForeground::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewItemPresenter3Impl: Sized {
    fn RevealBackground(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetRevealBackground(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn RevealBorderBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetRevealBorderBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn RevealBorderThickness(&self) -> ::windows::core::Result<super::super::Thickness>;
    fn SetRevealBorderThickness(&self, value: &super::super::Thickness) -> ::windows::core::Result<()>;
    fn RevealBackgroundShowsAboveContent(&self) -> ::windows::core::Result<bool>;
    fn SetRevealBackgroundShowsAboveContent(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListViewItemPresenter3 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IListViewItemPresenter3";
}
#[cfg(feature = "implement_exclusive")]
impl IListViewItemPresenter3Vtbl {
    pub const fn new<Impl: IListViewItemPresenter3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IListViewItemPresenter3Vtbl {
        unsafe extern "system" fn RevealBackground<Impl: IListViewItemPresenter3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RevealBackground() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRevealBackground<Impl: IListViewItemPresenter3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRevealBackground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RevealBorderBrush<Impl: IListViewItemPresenter3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RevealBorderBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRevealBorderBrush<Impl: IListViewItemPresenter3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRevealBorderBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RevealBorderThickness<Impl: IListViewItemPresenter3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RevealBorderThickness() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRevealBorderThickness<Impl: IListViewItemPresenter3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRevealBorderThickness(&*(&value as *const <super::super::Thickness as ::windows::core::Abi>::Abi as *const <super::super::Thickness as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RevealBackgroundShowsAboveContent<Impl: IListViewItemPresenter3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RevealBackgroundShowsAboveContent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRevealBackgroundShowsAboveContent<Impl: IListViewItemPresenter3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRevealBackgroundShowsAboveContent(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IListViewItemPresenter3>, base.5, RevealBackground::<Impl, OFFSET>, SetRevealBackground::<Impl, OFFSET>, RevealBorderBrush::<Impl, OFFSET>, SetRevealBorderBrush::<Impl, OFFSET>, RevealBorderThickness::<Impl, OFFSET>, SetRevealBorderThickness::<Impl, OFFSET>, RevealBackgroundShowsAboveContent::<Impl, OFFSET>, SetRevealBackgroundShowsAboveContent::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewItemPresenter4Impl: Sized {
    fn SelectedDisabledBackground(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetSelectedDisabledBackground(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckPressedBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckPressedBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckDisabledBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckDisabledBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckBoxPointerOverBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckBoxPointerOverBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckBoxPressedBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckBoxPressedBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckBoxDisabledBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckBoxDisabledBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckBoxSelectedBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckBoxSelectedBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckBoxSelectedPointerOverBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckBoxSelectedPointerOverBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckBoxSelectedPressedBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckBoxSelectedPressedBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckBoxSelectedDisabledBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckBoxSelectedDisabledBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckBoxBorderBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckBoxBorderBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckBoxPointerOverBorderBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckBoxPointerOverBorderBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckBoxPressedBorderBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckBoxPressedBorderBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckBoxDisabledBorderBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckBoxDisabledBorderBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckBoxCornerRadius(&self) -> ::windows::core::Result<super::super::CornerRadius>;
    fn SetCheckBoxCornerRadius(&self, value: &super::super::CornerRadius) -> ::windows::core::Result<()>;
    fn SelectionIndicatorCornerRadius(&self) -> ::windows::core::Result<super::super::CornerRadius>;
    fn SetSelectionIndicatorCornerRadius(&self, value: &super::super::CornerRadius) -> ::windows::core::Result<()>;
    fn SelectionIndicatorVisualEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetSelectionIndicatorVisualEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn SelectionIndicatorMode(&self) -> ::windows::core::Result<ListViewItemPresenterSelectionIndicatorMode>;
    fn SetSelectionIndicatorMode(&self, value: ListViewItemPresenterSelectionIndicatorMode) -> ::windows::core::Result<()>;
    fn SelectionIndicatorBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetSelectionIndicatorBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectionIndicatorPointerOverBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetSelectionIndicatorPointerOverBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectionIndicatorPressedBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetSelectionIndicatorPressedBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectionIndicatorDisabledBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetSelectionIndicatorDisabledBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectedBorderBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetSelectedBorderBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectedPressedBorderBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetSelectedPressedBorderBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectedDisabledBorderBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetSelectedDisabledBorderBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectedInnerBorderBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetSelectedInnerBorderBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn PointerOverBorderBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetPointerOverBorderBrush(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListViewItemPresenter4 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IListViewItemPresenter4";
}
#[cfg(feature = "implement_exclusive")]
impl IListViewItemPresenter4Vtbl {
    pub const fn new<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IListViewItemPresenter4Vtbl {
        unsafe extern "system" fn SelectedDisabledBackground<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectedDisabledBackground() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedDisabledBackground<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSelectedDisabledBackground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckPressedBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckPressedBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCheckPressedBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCheckPressedBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckDisabledBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckDisabledBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCheckDisabledBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCheckDisabledBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckBoxPointerOverBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckBoxPointerOverBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCheckBoxPointerOverBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCheckBoxPointerOverBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckBoxPressedBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckBoxPressedBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCheckBoxPressedBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCheckBoxPressedBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckBoxDisabledBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckBoxDisabledBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCheckBoxDisabledBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCheckBoxDisabledBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckBoxSelectedBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckBoxSelectedBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCheckBoxSelectedBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCheckBoxSelectedBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckBoxSelectedPointerOverBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckBoxSelectedPointerOverBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCheckBoxSelectedPointerOverBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCheckBoxSelectedPointerOverBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckBoxSelectedPressedBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckBoxSelectedPressedBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCheckBoxSelectedPressedBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCheckBoxSelectedPressedBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckBoxSelectedDisabledBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckBoxSelectedDisabledBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCheckBoxSelectedDisabledBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCheckBoxSelectedDisabledBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckBoxBorderBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckBoxBorderBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCheckBoxBorderBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCheckBoxBorderBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckBoxPointerOverBorderBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckBoxPointerOverBorderBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCheckBoxPointerOverBorderBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCheckBoxPointerOverBorderBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckBoxPressedBorderBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckBoxPressedBorderBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCheckBoxPressedBorderBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCheckBoxPressedBorderBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckBoxDisabledBorderBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckBoxDisabledBorderBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCheckBoxDisabledBorderBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCheckBoxDisabledBorderBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckBoxCornerRadius<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::CornerRadius) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckBoxCornerRadius() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCheckBoxCornerRadius<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::CornerRadius) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCheckBoxCornerRadius(&*(&value as *const <super::super::CornerRadius as ::windows::core::Abi>::Abi as *const <super::super::CornerRadius as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectionIndicatorCornerRadius<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::CornerRadius) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectionIndicatorCornerRadius() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectionIndicatorCornerRadius<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::CornerRadius) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSelectionIndicatorCornerRadius(&*(&value as *const <super::super::CornerRadius as ::windows::core::Abi>::Abi as *const <super::super::CornerRadius as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectionIndicatorVisualEnabled<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectionIndicatorVisualEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectionIndicatorVisualEnabled<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSelectionIndicatorVisualEnabled(value).into()
        }
        unsafe extern "system" fn SelectionIndicatorMode<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ListViewItemPresenterSelectionIndicatorMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectionIndicatorMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectionIndicatorMode<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ListViewItemPresenterSelectionIndicatorMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSelectionIndicatorMode(value).into()
        }
        unsafe extern "system" fn SelectionIndicatorBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectionIndicatorBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectionIndicatorBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSelectionIndicatorBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectionIndicatorPointerOverBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectionIndicatorPointerOverBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectionIndicatorPointerOverBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSelectionIndicatorPointerOverBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectionIndicatorPressedBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectionIndicatorPressedBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectionIndicatorPressedBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSelectionIndicatorPressedBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectionIndicatorDisabledBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectionIndicatorDisabledBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectionIndicatorDisabledBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSelectionIndicatorDisabledBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedBorderBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectedBorderBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedBorderBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSelectedBorderBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedPressedBorderBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectedPressedBorderBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedPressedBorderBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSelectedPressedBorderBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedDisabledBorderBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectedDisabledBorderBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedDisabledBorderBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSelectedDisabledBorderBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedInnerBorderBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectedInnerBorderBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedInnerBorderBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSelectedInnerBorderBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerOverBorderBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointerOverBorderBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPointerOverBorderBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPointerOverBorderBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IListViewItemPresenter4>,
            base.5,
            SelectedDisabledBackground::<Impl, OFFSET>,
            SetSelectedDisabledBackground::<Impl, OFFSET>,
            CheckPressedBrush::<Impl, OFFSET>,
            SetCheckPressedBrush::<Impl, OFFSET>,
            CheckDisabledBrush::<Impl, OFFSET>,
            SetCheckDisabledBrush::<Impl, OFFSET>,
            CheckBoxPointerOverBrush::<Impl, OFFSET>,
            SetCheckBoxPointerOverBrush::<Impl, OFFSET>,
            CheckBoxPressedBrush::<Impl, OFFSET>,
            SetCheckBoxPressedBrush::<Impl, OFFSET>,
            CheckBoxDisabledBrush::<Impl, OFFSET>,
            SetCheckBoxDisabledBrush::<Impl, OFFSET>,
            CheckBoxSelectedBrush::<Impl, OFFSET>,
            SetCheckBoxSelectedBrush::<Impl, OFFSET>,
            CheckBoxSelectedPointerOverBrush::<Impl, OFFSET>,
            SetCheckBoxSelectedPointerOverBrush::<Impl, OFFSET>,
            CheckBoxSelectedPressedBrush::<Impl, OFFSET>,
            SetCheckBoxSelectedPressedBrush::<Impl, OFFSET>,
            CheckBoxSelectedDisabledBrush::<Impl, OFFSET>,
            SetCheckBoxSelectedDisabledBrush::<Impl, OFFSET>,
            CheckBoxBorderBrush::<Impl, OFFSET>,
            SetCheckBoxBorderBrush::<Impl, OFFSET>,
            CheckBoxPointerOverBorderBrush::<Impl, OFFSET>,
            SetCheckBoxPointerOverBorderBrush::<Impl, OFFSET>,
            CheckBoxPressedBorderBrush::<Impl, OFFSET>,
            SetCheckBoxPressedBorderBrush::<Impl, OFFSET>,
            CheckBoxDisabledBorderBrush::<Impl, OFFSET>,
            SetCheckBoxDisabledBorderBrush::<Impl, OFFSET>,
            CheckBoxCornerRadius::<Impl, OFFSET>,
            SetCheckBoxCornerRadius::<Impl, OFFSET>,
            SelectionIndicatorCornerRadius::<Impl, OFFSET>,
            SetSelectionIndicatorCornerRadius::<Impl, OFFSET>,
            SelectionIndicatorVisualEnabled::<Impl, OFFSET>,
            SetSelectionIndicatorVisualEnabled::<Impl, OFFSET>,
            SelectionIndicatorMode::<Impl, OFFSET>,
            SetSelectionIndicatorMode::<Impl, OFFSET>,
            SelectionIndicatorBrush::<Impl, OFFSET>,
            SetSelectionIndicatorBrush::<Impl, OFFSET>,
            SelectionIndicatorPointerOverBrush::<Impl, OFFSET>,
            SetSelectionIndicatorPointerOverBrush::<Impl, OFFSET>,
            SelectionIndicatorPressedBrush::<Impl, OFFSET>,
            SetSelectionIndicatorPressedBrush::<Impl, OFFSET>,
            SelectionIndicatorDisabledBrush::<Impl, OFFSET>,
            SetSelectionIndicatorDisabledBrush::<Impl, OFFSET>,
            SelectedBorderBrush::<Impl, OFFSET>,
            SetSelectedBorderBrush::<Impl, OFFSET>,
            SelectedPressedBorderBrush::<Impl, OFFSET>,
            SetSelectedPressedBorderBrush::<Impl, OFFSET>,
            SelectedDisabledBorderBrush::<Impl, OFFSET>,
            SetSelectedDisabledBorderBrush::<Impl, OFFSET>,
            SelectedInnerBorderBrush::<Impl, OFFSET>,
            SetSelectedInnerBorderBrush::<Impl, OFFSET>,
            PointerOverBorderBrush::<Impl, OFFSET>,
            SetPointerOverBorderBrush::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewItemPresenterFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ListViewItemPresenter>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListViewItemPresenterFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IListViewItemPresenterFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IListViewItemPresenterFactoryVtbl {
    pub const fn new<Impl: IListViewItemPresenterFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IListViewItemPresenterFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IListViewItemPresenterFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IListViewItemPresenterFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewItemPresenterStaticsImpl: Sized {
    fn SelectionCheckMarkVisualEnabledProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckHintBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckSelectingBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn DragBackgroundProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn DragForegroundProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn FocusBorderBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn PlaceholderBackgroundProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn PointerOverBackgroundProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedBackgroundProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedForegroundProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedPointerOverBackgroundProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedPointerOverBorderBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedBorderThicknessProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn DisabledOpacityProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn DragOpacityProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ReorderHintOffsetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ListViewItemPresenterHorizontalContentAlignmentProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ListViewItemPresenterVerticalContentAlignmentProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ListViewItemPresenterPaddingProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn PointerOverBackgroundMarginProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ContentMarginProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListViewItemPresenterStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IListViewItemPresenterStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IListViewItemPresenterStaticsVtbl {
    pub const fn new<Impl: IListViewItemPresenterStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IListViewItemPresenterStaticsVtbl {
        unsafe extern "system" fn SelectionCheckMarkVisualEnabledProperty<Impl: IListViewItemPresenterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectionCheckMarkVisualEnabledProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckHintBrushProperty<Impl: IListViewItemPresenterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckHintBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckSelectingBrushProperty<Impl: IListViewItemPresenterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckSelectingBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckBrushProperty<Impl: IListViewItemPresenterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DragBackgroundProperty<Impl: IListViewItemPresenterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DragBackgroundProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DragForegroundProperty<Impl: IListViewItemPresenterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DragForegroundProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FocusBorderBrushProperty<Impl: IListViewItemPresenterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FocusBorderBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PlaceholderBackgroundProperty<Impl: IListViewItemPresenterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PlaceholderBackgroundProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PointerOverBackgroundProperty<Impl: IListViewItemPresenterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointerOverBackgroundProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedBackgroundProperty<Impl: IListViewItemPresenterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectedBackgroundProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedForegroundProperty<Impl: IListViewItemPresenterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectedForegroundProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedPointerOverBackgroundProperty<Impl: IListViewItemPresenterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectedPointerOverBackgroundProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedPointerOverBorderBrushProperty<Impl: IListViewItemPresenterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectedPointerOverBorderBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedBorderThicknessProperty<Impl: IListViewItemPresenterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectedBorderThicknessProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisabledOpacityProperty<Impl: IListViewItemPresenterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisabledOpacityProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DragOpacityProperty<Impl: IListViewItemPresenterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DragOpacityProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReorderHintOffsetProperty<Impl: IListViewItemPresenterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReorderHintOffsetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ListViewItemPresenterHorizontalContentAlignmentProperty<Impl: IListViewItemPresenterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ListViewItemPresenterHorizontalContentAlignmentProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ListViewItemPresenterVerticalContentAlignmentProperty<Impl: IListViewItemPresenterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ListViewItemPresenterVerticalContentAlignmentProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ListViewItemPresenterPaddingProperty<Impl: IListViewItemPresenterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ListViewItemPresenterPaddingProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PointerOverBackgroundMarginProperty<Impl: IListViewItemPresenterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointerOverBackgroundMarginProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentMarginProperty<Impl: IListViewItemPresenterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContentMarginProperty() {
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
            ::windows::core::GetRuntimeClassName::<IListViewItemPresenterStatics>,
            base.5,
            SelectionCheckMarkVisualEnabledProperty::<Impl, OFFSET>,
            CheckHintBrushProperty::<Impl, OFFSET>,
            CheckSelectingBrushProperty::<Impl, OFFSET>,
            CheckBrushProperty::<Impl, OFFSET>,
            DragBackgroundProperty::<Impl, OFFSET>,
            DragForegroundProperty::<Impl, OFFSET>,
            FocusBorderBrushProperty::<Impl, OFFSET>,
            PlaceholderBackgroundProperty::<Impl, OFFSET>,
            PointerOverBackgroundProperty::<Impl, OFFSET>,
            SelectedBackgroundProperty::<Impl, OFFSET>,
            SelectedForegroundProperty::<Impl, OFFSET>,
            SelectedPointerOverBackgroundProperty::<Impl, OFFSET>,
            SelectedPointerOverBorderBrushProperty::<Impl, OFFSET>,
            SelectedBorderThicknessProperty::<Impl, OFFSET>,
            DisabledOpacityProperty::<Impl, OFFSET>,
            DragOpacityProperty::<Impl, OFFSET>,
            ReorderHintOffsetProperty::<Impl, OFFSET>,
            ListViewItemPresenterHorizontalContentAlignmentProperty::<Impl, OFFSET>,
            ListViewItemPresenterVerticalContentAlignmentProperty::<Impl, OFFSET>,
            ListViewItemPresenterPaddingProperty::<Impl, OFFSET>,
            PointerOverBackgroundMarginProperty::<Impl, OFFSET>,
            ContentMarginProperty::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewItemPresenterStatics2Impl: Sized {
    fn SelectedPressedBackgroundProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn PressedBackgroundProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckBoxBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn FocusSecondaryBorderBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckModeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn PointerOverForegroundProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListViewItemPresenterStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IListViewItemPresenterStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IListViewItemPresenterStatics2Vtbl {
    pub const fn new<Impl: IListViewItemPresenterStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IListViewItemPresenterStatics2Vtbl {
        unsafe extern "system" fn SelectedPressedBackgroundProperty<Impl: IListViewItemPresenterStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectedPressedBackgroundProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PressedBackgroundProperty<Impl: IListViewItemPresenterStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PressedBackgroundProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckBoxBrushProperty<Impl: IListViewItemPresenterStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckBoxBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FocusSecondaryBorderBrushProperty<Impl: IListViewItemPresenterStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FocusSecondaryBorderBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckModeProperty<Impl: IListViewItemPresenterStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckModeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PointerOverForegroundProperty<Impl: IListViewItemPresenterStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointerOverForegroundProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IListViewItemPresenterStatics2>, base.5, SelectedPressedBackgroundProperty::<Impl, OFFSET>, PressedBackgroundProperty::<Impl, OFFSET>, CheckBoxBrushProperty::<Impl, OFFSET>, FocusSecondaryBorderBrushProperty::<Impl, OFFSET>, CheckModeProperty::<Impl, OFFSET>, PointerOverForegroundProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewItemPresenterStatics3Impl: Sized {
    fn RevealBackgroundProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn RevealBorderBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn RevealBorderThicknessProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn RevealBackgroundShowsAboveContentProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListViewItemPresenterStatics3 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IListViewItemPresenterStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IListViewItemPresenterStatics3Vtbl {
    pub const fn new<Impl: IListViewItemPresenterStatics3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IListViewItemPresenterStatics3Vtbl {
        unsafe extern "system" fn RevealBackgroundProperty<Impl: IListViewItemPresenterStatics3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RevealBackgroundProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RevealBorderBrushProperty<Impl: IListViewItemPresenterStatics3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RevealBorderBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RevealBorderThicknessProperty<Impl: IListViewItemPresenterStatics3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RevealBorderThicknessProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RevealBackgroundShowsAboveContentProperty<Impl: IListViewItemPresenterStatics3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RevealBackgroundShowsAboveContentProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IListViewItemPresenterStatics3>, base.5, RevealBackgroundProperty::<Impl, OFFSET>, RevealBorderBrushProperty::<Impl, OFFSET>, RevealBorderThicknessProperty::<Impl, OFFSET>, RevealBackgroundShowsAboveContentProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewItemPresenterStatics4Impl: Sized {
    fn SelectedDisabledBackgroundProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckPressedBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckDisabledBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckBoxPointerOverBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckBoxPressedBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckBoxDisabledBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckBoxSelectedBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckBoxSelectedPointerOverBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckBoxSelectedPressedBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckBoxSelectedDisabledBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckBoxBorderBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckBoxPointerOverBorderBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckBoxPressedBorderBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckBoxDisabledBorderBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckBoxCornerRadiusProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectionIndicatorCornerRadiusProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectionIndicatorVisualEnabledProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectionIndicatorModeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectionIndicatorBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectionIndicatorPointerOverBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectionIndicatorPressedBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectionIndicatorDisabledBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedBorderBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedPressedBorderBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedDisabledBorderBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedInnerBorderBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn PointerOverBorderBrushProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListViewItemPresenterStatics4 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IListViewItemPresenterStatics4";
}
#[cfg(feature = "implement_exclusive")]
impl IListViewItemPresenterStatics4Vtbl {
    pub const fn new<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IListViewItemPresenterStatics4Vtbl {
        unsafe extern "system" fn SelectedDisabledBackgroundProperty<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectedDisabledBackgroundProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckPressedBrushProperty<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckPressedBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckDisabledBrushProperty<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckDisabledBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckBoxPointerOverBrushProperty<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckBoxPointerOverBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckBoxPressedBrushProperty<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckBoxPressedBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckBoxDisabledBrushProperty<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckBoxDisabledBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckBoxSelectedBrushProperty<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckBoxSelectedBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckBoxSelectedPointerOverBrushProperty<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckBoxSelectedPointerOverBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckBoxSelectedPressedBrushProperty<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckBoxSelectedPressedBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckBoxSelectedDisabledBrushProperty<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckBoxSelectedDisabledBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckBoxBorderBrushProperty<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckBoxBorderBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckBoxPointerOverBorderBrushProperty<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckBoxPointerOverBorderBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckBoxPressedBorderBrushProperty<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckBoxPressedBorderBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckBoxDisabledBorderBrushProperty<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckBoxDisabledBorderBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckBoxCornerRadiusProperty<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CheckBoxCornerRadiusProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectionIndicatorCornerRadiusProperty<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectionIndicatorCornerRadiusProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectionIndicatorVisualEnabledProperty<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectionIndicatorVisualEnabledProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectionIndicatorModeProperty<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectionIndicatorModeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectionIndicatorBrushProperty<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectionIndicatorBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectionIndicatorPointerOverBrushProperty<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectionIndicatorPointerOverBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectionIndicatorPressedBrushProperty<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectionIndicatorPressedBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectionIndicatorDisabledBrushProperty<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectionIndicatorDisabledBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedBorderBrushProperty<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectedBorderBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedPressedBorderBrushProperty<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectedPressedBorderBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedDisabledBorderBrushProperty<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectedDisabledBorderBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedInnerBorderBrushProperty<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectedInnerBorderBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PointerOverBorderBrushProperty<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointerOverBorderBrushProperty() {
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
            ::windows::core::GetRuntimeClassName::<IListViewItemPresenterStatics4>,
            base.5,
            SelectedDisabledBackgroundProperty::<Impl, OFFSET>,
            CheckPressedBrushProperty::<Impl, OFFSET>,
            CheckDisabledBrushProperty::<Impl, OFFSET>,
            CheckBoxPointerOverBrushProperty::<Impl, OFFSET>,
            CheckBoxPressedBrushProperty::<Impl, OFFSET>,
            CheckBoxDisabledBrushProperty::<Impl, OFFSET>,
            CheckBoxSelectedBrushProperty::<Impl, OFFSET>,
            CheckBoxSelectedPointerOverBrushProperty::<Impl, OFFSET>,
            CheckBoxSelectedPressedBrushProperty::<Impl, OFFSET>,
            CheckBoxSelectedDisabledBrushProperty::<Impl, OFFSET>,
            CheckBoxBorderBrushProperty::<Impl, OFFSET>,
            CheckBoxPointerOverBorderBrushProperty::<Impl, OFFSET>,
            CheckBoxPressedBorderBrushProperty::<Impl, OFFSET>,
            CheckBoxDisabledBorderBrushProperty::<Impl, OFFSET>,
            CheckBoxCornerRadiusProperty::<Impl, OFFSET>,
            SelectionIndicatorCornerRadiusProperty::<Impl, OFFSET>,
            SelectionIndicatorVisualEnabledProperty::<Impl, OFFSET>,
            SelectionIndicatorModeProperty::<Impl, OFFSET>,
            SelectionIndicatorBrushProperty::<Impl, OFFSET>,
            SelectionIndicatorPointerOverBrushProperty::<Impl, OFFSET>,
            SelectionIndicatorPressedBrushProperty::<Impl, OFFSET>,
            SelectionIndicatorDisabledBrushProperty::<Impl, OFFSET>,
            SelectedBorderBrushProperty::<Impl, OFFSET>,
            SelectedPressedBorderBrushProperty::<Impl, OFFSET>,
            SelectedDisabledBorderBrushProperty::<Impl, OFFSET>,
            SelectedInnerBorderBrushProperty::<Impl, OFFSET>,
            PointerOverBorderBrushProperty::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IListViewItemTemplateSettingsImpl: Sized {
    fn DragItemsCount(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListViewItemTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IListViewItemTemplateSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IListViewItemTemplateSettingsVtbl {
    pub const fn new<Impl: IListViewItemTemplateSettingsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IListViewItemTemplateSettingsVtbl {
        unsafe extern "system" fn DragItemsCount<Impl: IListViewItemTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DragItemsCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IListViewItemTemplateSettings>, base.5, DragItemsCount::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoopingSelectorImpl: Sized {
    fn ShouldLoop(&self) -> ::windows::core::Result<bool>;
    fn SetShouldLoop(&self, value: bool) -> ::windows::core::Result<()>;
    fn Items(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<::windows::core::IInspectable>>;
    fn SetItems(&self, value: &::core::option::Option<super::super::super::super::Foundation::Collections::IVector<::windows::core::IInspectable>>) -> ::windows::core::Result<()>;
    fn SelectedIndex(&self) -> ::windows::core::Result<i32>;
    fn SetSelectedIndex(&self, value: i32) -> ::windows::core::Result<()>;
    fn SelectedItem(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetSelectedItem(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn ItemWidth(&self) -> ::windows::core::Result<i32>;
    fn SetItemWidth(&self, value: i32) -> ::windows::core::Result<()>;
    fn ItemHeight(&self) -> ::windows::core::Result<i32>;
    fn SetItemHeight(&self, value: i32) -> ::windows::core::Result<()>;
    fn ItemTemplate(&self) -> ::windows::core::Result<super::super::DataTemplate>;
    fn SetItemTemplate(&self, value: &::core::option::Option<super::super::DataTemplate>) -> ::windows::core::Result<()>;
    fn SelectionChanged(&self, handler: &::core::option::Option<super::SelectionChangedEventHandler>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSelectionChanged(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILoopingSelector {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ILoopingSelector";
}
#[cfg(feature = "implement_exclusive")]
impl ILoopingSelectorVtbl {
    pub const fn new<Impl: ILoopingSelectorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILoopingSelectorVtbl {
        unsafe extern "system" fn ShouldLoop<Impl: ILoopingSelectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShouldLoop() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShouldLoop<Impl: ILoopingSelectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetShouldLoop(value).into()
        }
        unsafe extern "system" fn Items<Impl: ILoopingSelectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Items() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetItems<Impl: ILoopingSelectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetItems(&*(&value as *const <super::super::super::super::Foundation::Collections::IVector<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Collections::IVector<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedIndex<Impl: ILoopingSelectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectedIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedIndex<Impl: ILoopingSelectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSelectedIndex(value).into()
        }
        unsafe extern "system" fn SelectedItem<Impl: ILoopingSelectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectedItem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedItem<Impl: ILoopingSelectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSelectedItem(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ItemWidth<Impl: ILoopingSelectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ItemWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetItemWidth<Impl: ILoopingSelectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetItemWidth(value).into()
        }
        unsafe extern "system" fn ItemHeight<Impl: ILoopingSelectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ItemHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetItemHeight<Impl: ILoopingSelectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetItemHeight(value).into()
        }
        unsafe extern "system" fn ItemTemplate<Impl: ILoopingSelectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ItemTemplate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetItemTemplate<Impl: ILoopingSelectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetItemTemplate(&*(&value as *const <super::super::DataTemplate as ::windows::core::Abi>::Abi as *const <super::super::DataTemplate as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectionChanged<Impl: ILoopingSelectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectionChanged(&*(&handler as *const <super::SelectionChangedEventHandler as ::windows::core::Abi>::Abi as *const <super::SelectionChangedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSelectionChanged<Impl: ILoopingSelectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveSelectionChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ILoopingSelector>,
            base.5,
            ShouldLoop::<Impl, OFFSET>,
            SetShouldLoop::<Impl, OFFSET>,
            Items::<Impl, OFFSET>,
            SetItems::<Impl, OFFSET>,
            SelectedIndex::<Impl, OFFSET>,
            SetSelectedIndex::<Impl, OFFSET>,
            SelectedItem::<Impl, OFFSET>,
            SetSelectedItem::<Impl, OFFSET>,
            ItemWidth::<Impl, OFFSET>,
            SetItemWidth::<Impl, OFFSET>,
            ItemHeight::<Impl, OFFSET>,
            SetItemHeight::<Impl, OFFSET>,
            ItemTemplate::<Impl, OFFSET>,
            SetItemTemplate::<Impl, OFFSET>,
            SelectionChanged::<Impl, OFFSET>,
            RemoveSelectionChanged::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoopingSelectorItemImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILoopingSelectorItem {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ILoopingSelectorItem";
}
#[cfg(feature = "implement_exclusive")]
impl ILoopingSelectorItemVtbl {
    pub const fn new<Impl: ILoopingSelectorItemImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILoopingSelectorItemVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILoopingSelectorItem>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoopingSelectorPanelImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILoopingSelectorPanel {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ILoopingSelectorPanel";
}
#[cfg(feature = "implement_exclusive")]
impl ILoopingSelectorPanelVtbl {
    pub const fn new<Impl: ILoopingSelectorPanelImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILoopingSelectorPanelVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILoopingSelectorPanel>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoopingSelectorStaticsImpl: Sized {
    fn ShouldLoopProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ItemsProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedIndexProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedItemProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ItemWidthProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ItemHeightProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ItemTemplateProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILoopingSelectorStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ILoopingSelectorStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ILoopingSelectorStaticsVtbl {
    pub const fn new<Impl: ILoopingSelectorStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILoopingSelectorStaticsVtbl {
        unsafe extern "system" fn ShouldLoopProperty<Impl: ILoopingSelectorStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShouldLoopProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemsProperty<Impl: ILoopingSelectorStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ItemsProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedIndexProperty<Impl: ILoopingSelectorStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectedIndexProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedItemProperty<Impl: ILoopingSelectorStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectedItemProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemWidthProperty<Impl: ILoopingSelectorStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ItemWidthProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemHeightProperty<Impl: ILoopingSelectorStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ItemHeightProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemTemplateProperty<Impl: ILoopingSelectorStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ItemTemplateProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILoopingSelectorStatics>, base.5, ShouldLoopProperty::<Impl, OFFSET>, ItemsProperty::<Impl, OFFSET>, SelectedIndexProperty::<Impl, OFFSET>, SelectedItemProperty::<Impl, OFFSET>, ItemWidthProperty::<Impl, OFFSET>, ItemHeightProperty::<Impl, OFFSET>, ItemTemplateProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuFlyoutItemTemplateSettingsImpl: Sized {
    fn KeyboardAcceleratorTextMinWidth(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMenuFlyoutItemTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IMenuFlyoutItemTemplateSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IMenuFlyoutItemTemplateSettingsVtbl {
    pub const fn new<Impl: IMenuFlyoutItemTemplateSettingsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMenuFlyoutItemTemplateSettingsVtbl {
        unsafe extern "system" fn KeyboardAcceleratorTextMinWidth<Impl: IMenuFlyoutItemTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyboardAcceleratorTextMinWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMenuFlyoutItemTemplateSettings>, base.5, KeyboardAcceleratorTextMinWidth::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMenuFlyoutPresenterTemplateSettingsImpl: Sized {
    fn FlyoutContentMinWidth(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMenuFlyoutPresenterTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IMenuFlyoutPresenterTemplateSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IMenuFlyoutPresenterTemplateSettingsVtbl {
    pub const fn new<Impl: IMenuFlyoutPresenterTemplateSettingsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMenuFlyoutPresenterTemplateSettingsVtbl {
        unsafe extern "system" fn FlyoutContentMinWidth<Impl: IMenuFlyoutPresenterTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FlyoutContentMinWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMenuFlyoutPresenterTemplateSettings>, base.5, FlyoutContentMinWidth::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationViewItemPresenterImpl: Sized {
    fn Icon(&self) -> ::windows::core::Result<super::IconElement>;
    fn SetIcon(&self, value: &::core::option::Option<super::IconElement>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INavigationViewItemPresenter {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.INavigationViewItemPresenter";
}
#[cfg(feature = "implement_exclusive")]
impl INavigationViewItemPresenterVtbl {
    pub const fn new<Impl: INavigationViewItemPresenterImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INavigationViewItemPresenterVtbl {
        unsafe extern "system" fn Icon<Impl: INavigationViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Icon() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIcon<Impl: INavigationViewItemPresenterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIcon(&*(&value as *const <super::IconElement as ::windows::core::Abi>::Abi as *const <super::IconElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INavigationViewItemPresenter>, base.5, Icon::<Impl, OFFSET>, SetIcon::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationViewItemPresenterFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<NavigationViewItemPresenter>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INavigationViewItemPresenterFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.INavigationViewItemPresenterFactory";
}
#[cfg(feature = "implement_exclusive")]
impl INavigationViewItemPresenterFactoryVtbl {
    pub const fn new<Impl: INavigationViewItemPresenterFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INavigationViewItemPresenterFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: INavigationViewItemPresenterFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INavigationViewItemPresenterFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INavigationViewItemPresenterStaticsImpl: Sized {
    fn IconProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INavigationViewItemPresenterStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.INavigationViewItemPresenterStatics";
}
#[cfg(feature = "implement_exclusive")]
impl INavigationViewItemPresenterStaticsVtbl {
    pub const fn new<Impl: INavigationViewItemPresenterStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INavigationViewItemPresenterStaticsVtbl {
        unsafe extern "system" fn IconProperty<Impl: INavigationViewItemPresenterStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IconProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INavigationViewItemPresenterStatics>, base.5, IconProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IOrientedVirtualizingPanelImpl: Sized {
    fn CanVerticallyScroll(&self) -> ::windows::core::Result<bool>;
    fn SetCanVerticallyScroll(&self, value: bool) -> ::windows::core::Result<()>;
    fn CanHorizontallyScroll(&self) -> ::windows::core::Result<bool>;
    fn SetCanHorizontallyScroll(&self, value: bool) -> ::windows::core::Result<()>;
    fn ExtentWidth(&self) -> ::windows::core::Result<f64>;
    fn ExtentHeight(&self) -> ::windows::core::Result<f64>;
    fn ViewportWidth(&self) -> ::windows::core::Result<f64>;
    fn ViewportHeight(&self) -> ::windows::core::Result<f64>;
    fn HorizontalOffset(&self) -> ::windows::core::Result<f64>;
    fn VerticalOffset(&self) -> ::windows::core::Result<f64>;
    fn ScrollOwner(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetScrollOwner(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn LineUp(&self) -> ::windows::core::Result<()>;
    fn LineDown(&self) -> ::windows::core::Result<()>;
    fn LineLeft(&self) -> ::windows::core::Result<()>;
    fn LineRight(&self) -> ::windows::core::Result<()>;
    fn PageUp(&self) -> ::windows::core::Result<()>;
    fn PageDown(&self) -> ::windows::core::Result<()>;
    fn PageLeft(&self) -> ::windows::core::Result<()>;
    fn PageRight(&self) -> ::windows::core::Result<()>;
    fn MouseWheelUp(&self) -> ::windows::core::Result<()>;
    fn MouseWheelDown(&self) -> ::windows::core::Result<()>;
    fn MouseWheelLeft(&self) -> ::windows::core::Result<()>;
    fn MouseWheelRight(&self) -> ::windows::core::Result<()>;
    fn SetHorizontalOffset(&self, offset: f64) -> ::windows::core::Result<()>;
    fn SetVerticalOffset(&self, offset: f64) -> ::windows::core::Result<()>;
    fn MakeVisible(&self, visual: &::core::option::Option<super::super::UIElement>, rectangle: &super::super::super::super::Foundation::Rect) -> ::windows::core::Result<super::super::super::super::Foundation::Rect>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IOrientedVirtualizingPanel {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IOrientedVirtualizingPanel";
}
#[cfg(feature = "implement_exclusive")]
impl IOrientedVirtualizingPanelVtbl {
    pub const fn new<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IOrientedVirtualizingPanelVtbl {
        unsafe extern "system" fn CanVerticallyScroll<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanVerticallyScroll() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCanVerticallyScroll<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCanVerticallyScroll(value).into()
        }
        unsafe extern "system" fn CanHorizontallyScroll<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanHorizontallyScroll() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCanHorizontallyScroll<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCanHorizontallyScroll(value).into()
        }
        unsafe extern "system" fn ExtentWidth<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExtentWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtentHeight<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExtentHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ViewportWidth<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ViewportWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ViewportHeight<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ViewportHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HorizontalOffset<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HorizontalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerticalOffset<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VerticalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScrollOwner<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScrollOwner() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScrollOwner<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetScrollOwner(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LineUp<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).LineUp().into()
        }
        unsafe extern "system" fn LineDown<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).LineDown().into()
        }
        unsafe extern "system" fn LineLeft<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).LineLeft().into()
        }
        unsafe extern "system" fn LineRight<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).LineRight().into()
        }
        unsafe extern "system" fn PageUp<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).PageUp().into()
        }
        unsafe extern "system" fn PageDown<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).PageDown().into()
        }
        unsafe extern "system" fn PageLeft<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).PageLeft().into()
        }
        unsafe extern "system" fn PageRight<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).PageRight().into()
        }
        unsafe extern "system" fn MouseWheelUp<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).MouseWheelUp().into()
        }
        unsafe extern "system" fn MouseWheelDown<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).MouseWheelDown().into()
        }
        unsafe extern "system" fn MouseWheelLeft<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).MouseWheelLeft().into()
        }
        unsafe extern "system" fn MouseWheelRight<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).MouseWheelRight().into()
        }
        unsafe extern "system" fn SetHorizontalOffset<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offset: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHorizontalOffset(offset).into()
        }
        unsafe extern "system" fn SetVerticalOffset<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offset: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetVerticalOffset(offset).into()
        }
        unsafe extern "system" fn MakeVisible<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, visual: ::windows::core::RawPtr, rectangle: super::super::super::super::Foundation::Rect, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MakeVisible(&*(&visual as *const <super::super::UIElement as ::windows::core::Abi>::Abi as *const <super::super::UIElement as ::windows::core::DefaultType>::DefaultType), &*(&rectangle as *const <super::super::super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<IOrientedVirtualizingPanel>,
            base.5,
            CanVerticallyScroll::<Impl, OFFSET>,
            SetCanVerticallyScroll::<Impl, OFFSET>,
            CanHorizontallyScroll::<Impl, OFFSET>,
            SetCanHorizontallyScroll::<Impl, OFFSET>,
            ExtentWidth::<Impl, OFFSET>,
            ExtentHeight::<Impl, OFFSET>,
            ViewportWidth::<Impl, OFFSET>,
            ViewportHeight::<Impl, OFFSET>,
            HorizontalOffset::<Impl, OFFSET>,
            VerticalOffset::<Impl, OFFSET>,
            ScrollOwner::<Impl, OFFSET>,
            SetScrollOwner::<Impl, OFFSET>,
            LineUp::<Impl, OFFSET>,
            LineDown::<Impl, OFFSET>,
            LineLeft::<Impl, OFFSET>,
            LineRight::<Impl, OFFSET>,
            PageUp::<Impl, OFFSET>,
            PageDown::<Impl, OFFSET>,
            PageLeft::<Impl, OFFSET>,
            PageRight::<Impl, OFFSET>,
            MouseWheelUp::<Impl, OFFSET>,
            MouseWheelDown::<Impl, OFFSET>,
            MouseWheelLeft::<Impl, OFFSET>,
            MouseWheelRight::<Impl, OFFSET>,
            SetHorizontalOffset::<Impl, OFFSET>,
            SetVerticalOffset::<Impl, OFFSET>,
            MakeVisible::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IOrientedVirtualizingPanelFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IOrientedVirtualizingPanelFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IOrientedVirtualizingPanelFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IOrientedVirtualizingPanelFactoryVtbl {
    pub const fn new<Impl: IOrientedVirtualizingPanelFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IOrientedVirtualizingPanelFactoryVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IOrientedVirtualizingPanelFactory>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPickerFlyoutBaseImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPickerFlyoutBase {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IPickerFlyoutBase";
}
#[cfg(feature = "implement_exclusive")]
impl IPickerFlyoutBaseVtbl {
    pub const fn new<Impl: IPickerFlyoutBaseImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPickerFlyoutBaseVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPickerFlyoutBase>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPickerFlyoutBaseFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<PickerFlyoutBase>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPickerFlyoutBaseFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IPickerFlyoutBaseFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPickerFlyoutBaseFactoryVtbl {
    pub const fn new<Impl: IPickerFlyoutBaseFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPickerFlyoutBaseFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IPickerFlyoutBaseFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPickerFlyoutBaseFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPickerFlyoutBaseOverridesImpl: Sized {
    fn OnConfirmed(&self) -> ::windows::core::Result<()>;
    fn ShouldShowConfirmationButtons(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPickerFlyoutBaseOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IPickerFlyoutBaseOverrides";
}
#[cfg(feature = "implement_exclusive")]
impl IPickerFlyoutBaseOverridesVtbl {
    pub const fn new<Impl: IPickerFlyoutBaseOverridesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPickerFlyoutBaseOverridesVtbl {
        unsafe extern "system" fn OnConfirmed<Impl: IPickerFlyoutBaseOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).OnConfirmed().into()
        }
        unsafe extern "system" fn ShouldShowConfirmationButtons<Impl: IPickerFlyoutBaseOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShouldShowConfirmationButtons() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPickerFlyoutBaseOverrides>, base.5, OnConfirmed::<Impl, OFFSET>, ShouldShowConfirmationButtons::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPickerFlyoutBaseStaticsImpl: Sized {
    fn TitleProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn GetTitle(&self, element: &::core::option::Option<super::super::DependencyObject>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitle(&self, element: &::core::option::Option<super::super::DependencyObject>, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPickerFlyoutBaseStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IPickerFlyoutBaseStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPickerFlyoutBaseStaticsVtbl {
    pub const fn new<Impl: IPickerFlyoutBaseStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPickerFlyoutBaseStaticsVtbl {
        unsafe extern "system" fn TitleProperty<Impl: IPickerFlyoutBaseStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TitleProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTitle<Impl: IPickerFlyoutBaseStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTitle(&*(&element as *const <super::super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTitle<Impl: IPickerFlyoutBaseStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTitle(&*(&element as *const <super::super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::super::DependencyObject as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPickerFlyoutBaseStatics>, base.5, TitleProperty::<Impl, OFFSET>, GetTitle::<Impl, OFFSET>, SetTitle::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPivotHeaderItemImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPivotHeaderItem {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IPivotHeaderItem";
}
#[cfg(feature = "implement_exclusive")]
impl IPivotHeaderItemVtbl {
    pub const fn new<Impl: IPivotHeaderItemImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPivotHeaderItemVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPivotHeaderItem>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPivotHeaderItemFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<PivotHeaderItem>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPivotHeaderItemFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IPivotHeaderItemFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPivotHeaderItemFactoryVtbl {
    pub const fn new<Impl: IPivotHeaderItemFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPivotHeaderItemFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IPivotHeaderItemFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPivotHeaderItemFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPivotHeaderPanelImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPivotHeaderPanel {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IPivotHeaderPanel";
}
#[cfg(feature = "implement_exclusive")]
impl IPivotHeaderPanelVtbl {
    pub const fn new<Impl: IPivotHeaderPanelImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPivotHeaderPanelVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPivotHeaderPanel>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPivotPanelImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPivotPanel {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IPivotPanel";
}
#[cfg(feature = "implement_exclusive")]
impl IPivotPanelVtbl {
    pub const fn new<Impl: IPivotPanelImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPivotPanelVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPivotPanel>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPopupImpl: Sized {
    fn Child(&self) -> ::windows::core::Result<super::super::UIElement>;
    fn SetChild(&self, value: &::core::option::Option<super::super::UIElement>) -> ::windows::core::Result<()>;
    fn IsOpen(&self) -> ::windows::core::Result<bool>;
    fn SetIsOpen(&self, value: bool) -> ::windows::core::Result<()>;
    fn HorizontalOffset(&self) -> ::windows::core::Result<f64>;
    fn SetHorizontalOffset(&self, value: f64) -> ::windows::core::Result<()>;
    fn VerticalOffset(&self) -> ::windows::core::Result<f64>;
    fn SetVerticalOffset(&self, value: f64) -> ::windows::core::Result<()>;
    fn ChildTransitions(&self) -> ::windows::core::Result<super::super::Media::Animation::TransitionCollection>;
    fn SetChildTransitions(&self, value: &::core::option::Option<super::super::Media::Animation::TransitionCollection>) -> ::windows::core::Result<()>;
    fn IsLightDismissEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsLightDismissEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn Opened(&self, handler: &::core::option::Option<super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveOpened(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Closed(&self, handler: &::core::option::Option<super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPopup {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IPopup";
}
#[cfg(feature = "implement_exclusive")]
impl IPopupVtbl {
    pub const fn new<Impl: IPopupImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPopupVtbl {
        unsafe extern "system" fn Child<Impl: IPopupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Child() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChild<Impl: IPopupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetChild(&*(&value as *const <super::super::UIElement as ::windows::core::Abi>::Abi as *const <super::super::UIElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsOpen<Impl: IPopupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsOpen() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsOpen<Impl: IPopupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsOpen(value).into()
        }
        unsafe extern "system" fn HorizontalOffset<Impl: IPopupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HorizontalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHorizontalOffset<Impl: IPopupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHorizontalOffset(value).into()
        }
        unsafe extern "system" fn VerticalOffset<Impl: IPopupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VerticalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVerticalOffset<Impl: IPopupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetVerticalOffset(value).into()
        }
        unsafe extern "system" fn ChildTransitions<Impl: IPopupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ChildTransitions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChildTransitions<Impl: IPopupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetChildTransitions(&*(&value as *const <super::super::Media::Animation::TransitionCollection as ::windows::core::Abi>::Abi as *const <super::super::Media::Animation::TransitionCollection as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsLightDismissEnabled<Impl: IPopupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsLightDismissEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsLightDismissEnabled<Impl: IPopupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsLightDismissEnabled(value).into()
        }
        unsafe extern "system" fn Opened<Impl: IPopupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Opened(&*(&handler as *const <super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveOpened<Impl: IPopupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveOpened(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Closed<Impl: IPopupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Closed(&*(&handler as *const <super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveClosed<Impl: IPopupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveClosed(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IPopup>,
            base.5,
            Child::<Impl, OFFSET>,
            SetChild::<Impl, OFFSET>,
            IsOpen::<Impl, OFFSET>,
            SetIsOpen::<Impl, OFFSET>,
            HorizontalOffset::<Impl, OFFSET>,
            SetHorizontalOffset::<Impl, OFFSET>,
            VerticalOffset::<Impl, OFFSET>,
            SetVerticalOffset::<Impl, OFFSET>,
            ChildTransitions::<Impl, OFFSET>,
            SetChildTransitions::<Impl, OFFSET>,
            IsLightDismissEnabled::<Impl, OFFSET>,
            SetIsLightDismissEnabled::<Impl, OFFSET>,
            Opened::<Impl, OFFSET>,
            RemoveOpened::<Impl, OFFSET>,
            Closed::<Impl, OFFSET>,
            RemoveClosed::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPopup2Impl: Sized {
    fn LightDismissOverlayMode(&self) -> ::windows::core::Result<super::LightDismissOverlayMode>;
    fn SetLightDismissOverlayMode(&self, value: super::LightDismissOverlayMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPopup2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IPopup2";
}
#[cfg(feature = "implement_exclusive")]
impl IPopup2Vtbl {
    pub const fn new<Impl: IPopup2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPopup2Vtbl {
        unsafe extern "system" fn LightDismissOverlayMode<Impl: IPopup2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::LightDismissOverlayMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LightDismissOverlayMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLightDismissOverlayMode<Impl: IPopup2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::LightDismissOverlayMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetLightDismissOverlayMode(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPopup2>, base.5, LightDismissOverlayMode::<Impl, OFFSET>, SetLightDismissOverlayMode::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPopup3Impl: Sized {
    fn ShouldConstrainToRootBounds(&self) -> ::windows::core::Result<bool>;
    fn SetShouldConstrainToRootBounds(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsConstrainedToRootBounds(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPopup3 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IPopup3";
}
#[cfg(feature = "implement_exclusive")]
impl IPopup3Vtbl {
    pub const fn new<Impl: IPopup3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPopup3Vtbl {
        unsafe extern "system" fn ShouldConstrainToRootBounds<Impl: IPopup3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShouldConstrainToRootBounds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShouldConstrainToRootBounds<Impl: IPopup3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetShouldConstrainToRootBounds(value).into()
        }
        unsafe extern "system" fn IsConstrainedToRootBounds<Impl: IPopup3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsConstrainedToRootBounds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPopup3>, base.5, ShouldConstrainToRootBounds::<Impl, OFFSET>, SetShouldConstrainToRootBounds::<Impl, OFFSET>, IsConstrainedToRootBounds::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPopup4Impl: Sized {
    fn PlacementTarget(&self) -> ::windows::core::Result<super::super::FrameworkElement>;
    fn SetPlacementTarget(&self, value: &::core::option::Option<super::super::FrameworkElement>) -> ::windows::core::Result<()>;
    fn DesiredPlacement(&self) -> ::windows::core::Result<PopupPlacementMode>;
    fn SetDesiredPlacement(&self, value: PopupPlacementMode) -> ::windows::core::Result<()>;
    fn ActualPlacement(&self) -> ::windows::core::Result<PopupPlacementMode>;
    fn ActualPlacementChanged(&self, handler: &::core::option::Option<super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveActualPlacementChanged(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPopup4 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IPopup4";
}
#[cfg(feature = "implement_exclusive")]
impl IPopup4Vtbl {
    pub const fn new<Impl: IPopup4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPopup4Vtbl {
        unsafe extern "system" fn PlacementTarget<Impl: IPopup4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PlacementTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlacementTarget<Impl: IPopup4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPlacementTarget(&*(&value as *const <super::super::FrameworkElement as ::windows::core::Abi>::Abi as *const <super::super::FrameworkElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DesiredPlacement<Impl: IPopup4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut PopupPlacementMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DesiredPlacement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredPlacement<Impl: IPopup4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: PopupPlacementMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDesiredPlacement(value).into()
        }
        unsafe extern "system" fn ActualPlacement<Impl: IPopup4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut PopupPlacementMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ActualPlacement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActualPlacementChanged<Impl: IPopup4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ActualPlacementChanged(&*(&handler as *const <super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveActualPlacementChanged<Impl: IPopup4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveActualPlacementChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPopup4>, base.5, PlacementTarget::<Impl, OFFSET>, SetPlacementTarget::<Impl, OFFSET>, DesiredPlacement::<Impl, OFFSET>, SetDesiredPlacement::<Impl, OFFSET>, ActualPlacement::<Impl, OFFSET>, ActualPlacementChanged::<Impl, OFFSET>, RemoveActualPlacementChanged::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPopupStaticsImpl: Sized {
    fn ChildProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn IsOpenProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn HorizontalOffsetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn VerticalOffsetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ChildTransitionsProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn IsLightDismissEnabledProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPopupStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IPopupStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPopupStaticsVtbl {
    pub const fn new<Impl: IPopupStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPopupStaticsVtbl {
        unsafe extern "system" fn ChildProperty<Impl: IPopupStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ChildProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOpenProperty<Impl: IPopupStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsOpenProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HorizontalOffsetProperty<Impl: IPopupStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HorizontalOffsetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerticalOffsetProperty<Impl: IPopupStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VerticalOffsetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChildTransitionsProperty<Impl: IPopupStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ChildTransitionsProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLightDismissEnabledProperty<Impl: IPopupStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsLightDismissEnabledProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPopupStatics>, base.5, ChildProperty::<Impl, OFFSET>, IsOpenProperty::<Impl, OFFSET>, HorizontalOffsetProperty::<Impl, OFFSET>, VerticalOffsetProperty::<Impl, OFFSET>, ChildTransitionsProperty::<Impl, OFFSET>, IsLightDismissEnabledProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPopupStatics2Impl: Sized {
    fn LightDismissOverlayModeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPopupStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IPopupStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IPopupStatics2Vtbl {
    pub const fn new<Impl: IPopupStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPopupStatics2Vtbl {
        unsafe extern "system" fn LightDismissOverlayModeProperty<Impl: IPopupStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LightDismissOverlayModeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPopupStatics2>, base.5, LightDismissOverlayModeProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPopupStatics3Impl: Sized {
    fn ShouldConstrainToRootBoundsProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPopupStatics3 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IPopupStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IPopupStatics3Vtbl {
    pub const fn new<Impl: IPopupStatics3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPopupStatics3Vtbl {
        unsafe extern "system" fn ShouldConstrainToRootBoundsProperty<Impl: IPopupStatics3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShouldConstrainToRootBoundsProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPopupStatics3>, base.5, ShouldConstrainToRootBoundsProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPopupStatics4Impl: Sized {
    fn PlacementTargetProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn DesiredPlacementProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPopupStatics4 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IPopupStatics4";
}
#[cfg(feature = "implement_exclusive")]
impl IPopupStatics4Vtbl {
    pub const fn new<Impl: IPopupStatics4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPopupStatics4Vtbl {
        unsafe extern "system" fn PlacementTargetProperty<Impl: IPopupStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PlacementTargetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DesiredPlacementProperty<Impl: IPopupStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DesiredPlacementProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPopupStatics4>, base.5, PlacementTargetProperty::<Impl, OFFSET>, DesiredPlacementProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProgressBarTemplateSettingsImpl: Sized {
    fn EllipseDiameter(&self) -> ::windows::core::Result<f64>;
    fn EllipseOffset(&self) -> ::windows::core::Result<f64>;
    fn EllipseAnimationWellPosition(&self) -> ::windows::core::Result<f64>;
    fn EllipseAnimationEndPosition(&self) -> ::windows::core::Result<f64>;
    fn ContainerAnimationStartPosition(&self) -> ::windows::core::Result<f64>;
    fn ContainerAnimationEndPosition(&self) -> ::windows::core::Result<f64>;
    fn IndicatorLengthDelta(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProgressBarTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IProgressBarTemplateSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IProgressBarTemplateSettingsVtbl {
    pub const fn new<Impl: IProgressBarTemplateSettingsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IProgressBarTemplateSettingsVtbl {
        unsafe extern "system" fn EllipseDiameter<Impl: IProgressBarTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EllipseDiameter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EllipseOffset<Impl: IProgressBarTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EllipseOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EllipseAnimationWellPosition<Impl: IProgressBarTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EllipseAnimationWellPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EllipseAnimationEndPosition<Impl: IProgressBarTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EllipseAnimationEndPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContainerAnimationStartPosition<Impl: IProgressBarTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContainerAnimationStartPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContainerAnimationEndPosition<Impl: IProgressBarTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContainerAnimationEndPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IndicatorLengthDelta<Impl: IProgressBarTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IndicatorLengthDelta() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IProgressBarTemplateSettings>, base.5, EllipseDiameter::<Impl, OFFSET>, EllipseOffset::<Impl, OFFSET>, EllipseAnimationWellPosition::<Impl, OFFSET>, EllipseAnimationEndPosition::<Impl, OFFSET>, ContainerAnimationStartPosition::<Impl, OFFSET>, ContainerAnimationEndPosition::<Impl, OFFSET>, IndicatorLengthDelta::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProgressRingTemplateSettingsImpl: Sized {
    fn EllipseDiameter(&self) -> ::windows::core::Result<f64>;
    fn EllipseOffset(&self) -> ::windows::core::Result<super::super::Thickness>;
    fn MaxSideLength(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProgressRingTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IProgressRingTemplateSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IProgressRingTemplateSettingsVtbl {
    pub const fn new<Impl: IProgressRingTemplateSettingsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IProgressRingTemplateSettingsVtbl {
        unsafe extern "system" fn EllipseDiameter<Impl: IProgressRingTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EllipseDiameter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EllipseOffset<Impl: IProgressRingTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EllipseOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxSideLength<Impl: IProgressRingTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaxSideLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IProgressRingTemplateSettings>, base.5, EllipseDiameter::<Impl, OFFSET>, EllipseOffset::<Impl, OFFSET>, MaxSideLength::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRangeBaseImpl: Sized {
    fn Minimum(&self) -> ::windows::core::Result<f64>;
    fn SetMinimum(&self, value: f64) -> ::windows::core::Result<()>;
    fn Maximum(&self) -> ::windows::core::Result<f64>;
    fn SetMaximum(&self, value: f64) -> ::windows::core::Result<()>;
    fn SmallChange(&self) -> ::windows::core::Result<f64>;
    fn SetSmallChange(&self, value: f64) -> ::windows::core::Result<()>;
    fn LargeChange(&self) -> ::windows::core::Result<f64>;
    fn SetLargeChange(&self, value: f64) -> ::windows::core::Result<()>;
    fn Value(&self) -> ::windows::core::Result<f64>;
    fn SetValue(&self, value: f64) -> ::windows::core::Result<()>;
    fn ValueChanged(&self, handler: &::core::option::Option<RangeBaseValueChangedEventHandler>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveValueChanged(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRangeBase {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IRangeBase";
}
#[cfg(feature = "implement_exclusive")]
impl IRangeBaseVtbl {
    pub const fn new<Impl: IRangeBaseImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRangeBaseVtbl {
        unsafe extern "system" fn Minimum<Impl: IRangeBaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Minimum() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinimum<Impl: IRangeBaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMinimum(value).into()
        }
        unsafe extern "system" fn Maximum<Impl: IRangeBaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Maximum() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaximum<Impl: IRangeBaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMaximum(value).into()
        }
        unsafe extern "system" fn SmallChange<Impl: IRangeBaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SmallChange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSmallChange<Impl: IRangeBaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSmallChange(value).into()
        }
        unsafe extern "system" fn LargeChange<Impl: IRangeBaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LargeChange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLargeChange<Impl: IRangeBaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetLargeChange(value).into()
        }
        unsafe extern "system" fn Value<Impl: IRangeBaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: IRangeBaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetValue(value).into()
        }
        unsafe extern "system" fn ValueChanged<Impl: IRangeBaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ValueChanged(&*(&handler as *const <RangeBaseValueChangedEventHandler as ::windows::core::Abi>::Abi as *const <RangeBaseValueChangedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveValueChanged<Impl: IRangeBaseImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveValueChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRangeBase>, base.5, Minimum::<Impl, OFFSET>, SetMinimum::<Impl, OFFSET>, Maximum::<Impl, OFFSET>, SetMaximum::<Impl, OFFSET>, SmallChange::<Impl, OFFSET>, SetSmallChange::<Impl, OFFSET>, LargeChange::<Impl, OFFSET>, SetLargeChange::<Impl, OFFSET>, Value::<Impl, OFFSET>, SetValue::<Impl, OFFSET>, ValueChanged::<Impl, OFFSET>, RemoveValueChanged::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRangeBaseFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<RangeBase>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRangeBaseFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IRangeBaseFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IRangeBaseFactoryVtbl {
    pub const fn new<Impl: IRangeBaseFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRangeBaseFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IRangeBaseFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRangeBaseFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRangeBaseOverridesImpl: Sized {
    fn OnMinimumChanged(&self, oldminimum: f64, newminimum: f64) -> ::windows::core::Result<()>;
    fn OnMaximumChanged(&self, oldmaximum: f64, newmaximum: f64) -> ::windows::core::Result<()>;
    fn OnValueChanged(&self, oldvalue: f64, newvalue: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRangeBaseOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IRangeBaseOverrides";
}
#[cfg(feature = "implement_exclusive")]
impl IRangeBaseOverridesVtbl {
    pub const fn new<Impl: IRangeBaseOverridesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRangeBaseOverridesVtbl {
        unsafe extern "system" fn OnMinimumChanged<Impl: IRangeBaseOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, oldminimum: f64, newminimum: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).OnMinimumChanged(oldminimum, newminimum).into()
        }
        unsafe extern "system" fn OnMaximumChanged<Impl: IRangeBaseOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, oldmaximum: f64, newmaximum: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).OnMaximumChanged(oldmaximum, newmaximum).into()
        }
        unsafe extern "system" fn OnValueChanged<Impl: IRangeBaseOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, oldvalue: f64, newvalue: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).OnValueChanged(oldvalue, newvalue).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRangeBaseOverrides>, base.5, OnMinimumChanged::<Impl, OFFSET>, OnMaximumChanged::<Impl, OFFSET>, OnValueChanged::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRangeBaseStaticsImpl: Sized {
    fn MinimumProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn MaximumProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SmallChangeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn LargeChangeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ValueProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRangeBaseStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IRangeBaseStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IRangeBaseStaticsVtbl {
    pub const fn new<Impl: IRangeBaseStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRangeBaseStaticsVtbl {
        unsafe extern "system" fn MinimumProperty<Impl: IRangeBaseStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MinimumProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaximumProperty<Impl: IRangeBaseStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MaximumProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SmallChangeProperty<Impl: IRangeBaseStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SmallChangeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LargeChangeProperty<Impl: IRangeBaseStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LargeChangeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValueProperty<Impl: IRangeBaseStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ValueProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRangeBaseStatics>, base.5, MinimumProperty::<Impl, OFFSET>, MaximumProperty::<Impl, OFFSET>, SmallChangeProperty::<Impl, OFFSET>, LargeChangeProperty::<Impl, OFFSET>, ValueProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRangeBaseValueChangedEventArgsImpl: Sized {
    fn OldValue(&self) -> ::windows::core::Result<f64>;
    fn NewValue(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRangeBaseValueChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IRangeBaseValueChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRangeBaseValueChangedEventArgsVtbl {
    pub const fn new<Impl: IRangeBaseValueChangedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRangeBaseValueChangedEventArgsVtbl {
        unsafe extern "system" fn OldValue<Impl: IRangeBaseValueChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OldValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NewValue<Impl: IRangeBaseValueChangedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NewValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRangeBaseValueChangedEventArgs>, base.5, OldValue::<Impl, OFFSET>, NewValue::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRepeatButtonImpl: Sized {
    fn Delay(&self) -> ::windows::core::Result<i32>;
    fn SetDelay(&self, value: i32) -> ::windows::core::Result<()>;
    fn Interval(&self) -> ::windows::core::Result<i32>;
    fn SetInterval(&self, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRepeatButton {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IRepeatButton";
}
#[cfg(feature = "implement_exclusive")]
impl IRepeatButtonVtbl {
    pub const fn new<Impl: IRepeatButtonImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRepeatButtonVtbl {
        unsafe extern "system" fn Delay<Impl: IRepeatButtonImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Delay() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDelay<Impl: IRepeatButtonImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDelay(value).into()
        }
        unsafe extern "system" fn Interval<Impl: IRepeatButtonImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Interval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInterval<Impl: IRepeatButtonImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetInterval(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRepeatButton>, base.5, Delay::<Impl, OFFSET>, SetDelay::<Impl, OFFSET>, Interval::<Impl, OFFSET>, SetInterval::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRepeatButtonStaticsImpl: Sized {
    fn DelayProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn IntervalProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRepeatButtonStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IRepeatButtonStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IRepeatButtonStaticsVtbl {
    pub const fn new<Impl: IRepeatButtonStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRepeatButtonStaticsVtbl {
        unsafe extern "system" fn DelayProperty<Impl: IRepeatButtonStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DelayProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IntervalProperty<Impl: IRepeatButtonStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IntervalProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRepeatButtonStatics>, base.5, DelayProperty::<Impl, OFFSET>, IntervalProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IScrollBarImpl: Sized {
    fn Orientation(&self) -> ::windows::core::Result<super::Orientation>;
    fn SetOrientation(&self, value: super::Orientation) -> ::windows::core::Result<()>;
    fn ViewportSize(&self) -> ::windows::core::Result<f64>;
    fn SetViewportSize(&self, value: f64) -> ::windows::core::Result<()>;
    fn IndicatorMode(&self) -> ::windows::core::Result<ScrollingIndicatorMode>;
    fn SetIndicatorMode(&self, value: ScrollingIndicatorMode) -> ::windows::core::Result<()>;
    fn Scroll(&self, handler: &::core::option::Option<ScrollEventHandler>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveScroll(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IScrollBar {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IScrollBar";
}
#[cfg(feature = "implement_exclusive")]
impl IScrollBarVtbl {
    pub const fn new<Impl: IScrollBarImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IScrollBarVtbl {
        unsafe extern "system" fn Orientation<Impl: IScrollBarImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::Orientation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Orientation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOrientation<Impl: IScrollBarImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::Orientation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetOrientation(value).into()
        }
        unsafe extern "system" fn ViewportSize<Impl: IScrollBarImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ViewportSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetViewportSize<Impl: IScrollBarImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetViewportSize(value).into()
        }
        unsafe extern "system" fn IndicatorMode<Impl: IScrollBarImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ScrollingIndicatorMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IndicatorMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIndicatorMode<Impl: IScrollBarImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ScrollingIndicatorMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIndicatorMode(value).into()
        }
        unsafe extern "system" fn Scroll<Impl: IScrollBarImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Scroll(&*(&handler as *const <ScrollEventHandler as ::windows::core::Abi>::Abi as *const <ScrollEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveScroll<Impl: IScrollBarImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveScroll(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IScrollBar>, base.5, Orientation::<Impl, OFFSET>, SetOrientation::<Impl, OFFSET>, ViewportSize::<Impl, OFFSET>, SetViewportSize::<Impl, OFFSET>, IndicatorMode::<Impl, OFFSET>, SetIndicatorMode::<Impl, OFFSET>, Scroll::<Impl, OFFSET>, RemoveScroll::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IScrollBarStaticsImpl: Sized {
    fn OrientationProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ViewportSizeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn IndicatorModeProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IScrollBarStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IScrollBarStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IScrollBarStaticsVtbl {
    pub const fn new<Impl: IScrollBarStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IScrollBarStaticsVtbl {
        unsafe extern "system" fn OrientationProperty<Impl: IScrollBarStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OrientationProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ViewportSizeProperty<Impl: IScrollBarStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ViewportSizeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IndicatorModeProperty<Impl: IScrollBarStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IndicatorModeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IScrollBarStatics>, base.5, OrientationProperty::<Impl, OFFSET>, ViewportSizeProperty::<Impl, OFFSET>, IndicatorModeProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IScrollEventArgsImpl: Sized {
    fn NewValue(&self) -> ::windows::core::Result<f64>;
    fn ScrollEventType(&self) -> ::windows::core::Result<ScrollEventType>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IScrollEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IScrollEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IScrollEventArgsVtbl {
    pub const fn new<Impl: IScrollEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IScrollEventArgsVtbl {
        unsafe extern "system" fn NewValue<Impl: IScrollEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NewValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScrollEventType<Impl: IScrollEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ScrollEventType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScrollEventType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IScrollEventArgs>, base.5, NewValue::<Impl, OFFSET>, ScrollEventType::<Impl, OFFSET>)
    }
}
pub trait IScrollSnapPointsInfoImpl: Sized {
    fn AreHorizontalSnapPointsRegular(&self) -> ::windows::core::Result<bool>;
    fn AreVerticalSnapPointsRegular(&self) -> ::windows::core::Result<bool>;
    fn HorizontalSnapPointsChanged(&self, handler: &::core::option::Option<super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveHorizontalSnapPointsChanged(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn VerticalSnapPointsChanged(&self, handler: &::core::option::Option<super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveVerticalSnapPointsChanged(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetIrregularSnapPoints(&self, orientation: super::Orientation, alignment: SnapPointsAlignment) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<f32>>;
    fn GetRegularSnapPoints(&self, orientation: super::Orientation, alignment: SnapPointsAlignment, offset: &mut f32) -> ::windows::core::Result<f32>;
}
impl ::windows::core::RuntimeName for IScrollSnapPointsInfo {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IScrollSnapPointsInfo";
}
impl IScrollSnapPointsInfoVtbl {
    pub const fn new<Impl: IScrollSnapPointsInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IScrollSnapPointsInfoVtbl {
        unsafe extern "system" fn AreHorizontalSnapPointsRegular<Impl: IScrollSnapPointsInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AreHorizontalSnapPointsRegular() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AreVerticalSnapPointsRegular<Impl: IScrollSnapPointsInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AreVerticalSnapPointsRegular() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HorizontalSnapPointsChanged<Impl: IScrollSnapPointsInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HorizontalSnapPointsChanged(&*(&handler as *const <super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveHorizontalSnapPointsChanged<Impl: IScrollSnapPointsInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveHorizontalSnapPointsChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn VerticalSnapPointsChanged<Impl: IScrollSnapPointsInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VerticalSnapPointsChanged(&*(&handler as *const <super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveVerticalSnapPointsChanged<Impl: IScrollSnapPointsInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveVerticalSnapPointsChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetIrregularSnapPoints<Impl: IScrollSnapPointsInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, orientation: super::Orientation, alignment: SnapPointsAlignment, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetIrregularSnapPoints(orientation, alignment) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRegularSnapPoints<Impl: IScrollSnapPointsInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, orientation: super::Orientation, alignment: SnapPointsAlignment, offset: *mut f32, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRegularSnapPoints(orientation, alignment, ::core::mem::transmute_copy(&offset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IScrollSnapPointsInfo>, base.5, AreHorizontalSnapPointsRegular::<Impl, OFFSET>, AreVerticalSnapPointsRegular::<Impl, OFFSET>, HorizontalSnapPointsChanged::<Impl, OFFSET>, RemoveHorizontalSnapPointsChanged::<Impl, OFFSET>, VerticalSnapPointsChanged::<Impl, OFFSET>, RemoveVerticalSnapPointsChanged::<Impl, OFFSET>, GetIrregularSnapPoints::<Impl, OFFSET>, GetRegularSnapPoints::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISelectorImpl: Sized {
    fn SelectedIndex(&self) -> ::windows::core::Result<i32>;
    fn SetSelectedIndex(&self, value: i32) -> ::windows::core::Result<()>;
    fn SelectedItem(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetSelectedItem(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn SelectedValue(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetSelectedValue(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn SelectedValuePath(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSelectedValuePath(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsSynchronizedWithCurrentItem(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<bool>>;
    fn SetIsSynchronizedWithCurrentItem(&self, value: &::core::option::Option<super::super::super::super::Foundation::IReference<bool>>) -> ::windows::core::Result<()>;
    fn SelectionChanged(&self, handler: &::core::option::Option<super::SelectionChangedEventHandler>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSelectionChanged(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISelector {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ISelector";
}
#[cfg(feature = "implement_exclusive")]
impl ISelectorVtbl {
    pub const fn new<Impl: ISelectorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISelectorVtbl {
        unsafe extern "system" fn SelectedIndex<Impl: ISelectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectedIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedIndex<Impl: ISelectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSelectedIndex(value).into()
        }
        unsafe extern "system" fn SelectedItem<Impl: ISelectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectedItem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedItem<Impl: ISelectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSelectedItem(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedValue<Impl: ISelectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectedValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedValue<Impl: ISelectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSelectedValue(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedValuePath<Impl: ISelectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectedValuePath() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedValuePath<Impl: ISelectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSelectedValuePath(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsSynchronizedWithCurrentItem<Impl: ISelectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSynchronizedWithCurrentItem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsSynchronizedWithCurrentItem<Impl: ISelectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsSynchronizedWithCurrentItem(&*(&value as *const <super::super::super::super::Foundation::IReference<bool> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::IReference<bool> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectionChanged<Impl: ISelectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectionChanged(&*(&handler as *const <super::SelectionChangedEventHandler as ::windows::core::Abi>::Abi as *const <super::SelectionChangedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSelectionChanged<Impl: ISelectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveSelectionChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ISelector>,
            base.5,
            SelectedIndex::<Impl, OFFSET>,
            SetSelectedIndex::<Impl, OFFSET>,
            SelectedItem::<Impl, OFFSET>,
            SetSelectedItem::<Impl, OFFSET>,
            SelectedValue::<Impl, OFFSET>,
            SetSelectedValue::<Impl, OFFSET>,
            SelectedValuePath::<Impl, OFFSET>,
            SetSelectedValuePath::<Impl, OFFSET>,
            IsSynchronizedWithCurrentItem::<Impl, OFFSET>,
            SetIsSynchronizedWithCurrentItem::<Impl, OFFSET>,
            SelectionChanged::<Impl, OFFSET>,
            RemoveSelectionChanged::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISelectorFactoryImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISelectorFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ISelectorFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ISelectorFactoryVtbl {
    pub const fn new<Impl: ISelectorFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISelectorFactoryVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISelectorFactory>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISelectorItemImpl: Sized {
    fn IsSelected(&self) -> ::windows::core::Result<bool>;
    fn SetIsSelected(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISelectorItem {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ISelectorItem";
}
#[cfg(feature = "implement_exclusive")]
impl ISelectorItemVtbl {
    pub const fn new<Impl: ISelectorItemImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISelectorItemVtbl {
        unsafe extern "system" fn IsSelected<Impl: ISelectorItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSelected() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsSelected<Impl: ISelectorItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsSelected(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISelectorItem>, base.5, IsSelected::<Impl, OFFSET>, SetIsSelected::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISelectorItemFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SelectorItem>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISelectorItemFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ISelectorItemFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ISelectorItemFactoryVtbl {
    pub const fn new<Impl: ISelectorItemFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISelectorItemFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: ISelectorItemFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISelectorItemFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISelectorItemStaticsImpl: Sized {
    fn IsSelectedProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISelectorItemStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ISelectorItemStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISelectorItemStaticsVtbl {
    pub const fn new<Impl: ISelectorItemStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISelectorItemStaticsVtbl {
        unsafe extern "system" fn IsSelectedProperty<Impl: ISelectorItemStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSelectedProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISelectorItemStatics>, base.5, IsSelectedProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISelectorStaticsImpl: Sized {
    fn SelectedIndexProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedItemProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedValueProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedValuePathProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn IsSynchronizedWithCurrentItemProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn GetIsSelectionActive(&self, element: &::core::option::Option<super::super::DependencyObject>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISelectorStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ISelectorStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISelectorStaticsVtbl {
    pub const fn new<Impl: ISelectorStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISelectorStaticsVtbl {
        unsafe extern "system" fn SelectedIndexProperty<Impl: ISelectorStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectedIndexProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedItemProperty<Impl: ISelectorStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectedItemProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedValueProperty<Impl: ISelectorStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectedValueProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedValuePathProperty<Impl: ISelectorStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectedValuePathProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSynchronizedWithCurrentItemProperty<Impl: ISelectorStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsSynchronizedWithCurrentItemProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsSelectionActive<Impl: ISelectorStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetIsSelectionActive(&*(&element as *const <super::super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISelectorStatics>, base.5, SelectedIndexProperty::<Impl, OFFSET>, SelectedItemProperty::<Impl, OFFSET>, SelectedValueProperty::<Impl, OFFSET>, SelectedValuePathProperty::<Impl, OFFSET>, IsSynchronizedWithCurrentItemProperty::<Impl, OFFSET>, GetIsSelectionActive::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISettingsFlyoutTemplateSettingsImpl: Sized {
    fn HeaderBackground(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn HeaderForeground(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn BorderBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn BorderThickness(&self) -> ::windows::core::Result<super::super::Thickness>;
    fn IconSource(&self) -> ::windows::core::Result<super::super::Media::ImageSource>;
    fn ContentTransitions(&self) -> ::windows::core::Result<super::super::Media::Animation::TransitionCollection>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISettingsFlyoutTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ISettingsFlyoutTemplateSettings";
}
#[cfg(feature = "implement_exclusive")]
impl ISettingsFlyoutTemplateSettingsVtbl {
    pub const fn new<Impl: ISettingsFlyoutTemplateSettingsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISettingsFlyoutTemplateSettingsVtbl {
        unsafe extern "system" fn HeaderBackground<Impl: ISettingsFlyoutTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HeaderBackground() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HeaderForeground<Impl: ISettingsFlyoutTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HeaderForeground() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BorderBrush<Impl: ISettingsFlyoutTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BorderBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BorderThickness<Impl: ISettingsFlyoutTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BorderThickness() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IconSource<Impl: ISettingsFlyoutTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IconSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentTransitions<Impl: ISettingsFlyoutTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ContentTransitions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISettingsFlyoutTemplateSettings>, base.5, HeaderBackground::<Impl, OFFSET>, HeaderForeground::<Impl, OFFSET>, BorderBrush::<Impl, OFFSET>, BorderThickness::<Impl, OFFSET>, IconSource::<Impl, OFFSET>, ContentTransitions::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISplitViewTemplateSettingsImpl: Sized {
    fn OpenPaneLength(&self) -> ::windows::core::Result<f64>;
    fn NegativeOpenPaneLength(&self) -> ::windows::core::Result<f64>;
    fn OpenPaneLengthMinusCompactLength(&self) -> ::windows::core::Result<f64>;
    fn NegativeOpenPaneLengthMinusCompactLength(&self) -> ::windows::core::Result<f64>;
    fn OpenPaneGridLength(&self) -> ::windows::core::Result<super::super::GridLength>;
    fn CompactPaneGridLength(&self) -> ::windows::core::Result<super::super::GridLength>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISplitViewTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ISplitViewTemplateSettings";
}
#[cfg(feature = "implement_exclusive")]
impl ISplitViewTemplateSettingsVtbl {
    pub const fn new<Impl: ISplitViewTemplateSettingsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISplitViewTemplateSettingsVtbl {
        unsafe extern "system" fn OpenPaneLength<Impl: ISplitViewTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenPaneLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NegativeOpenPaneLength<Impl: ISplitViewTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NegativeOpenPaneLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenPaneLengthMinusCompactLength<Impl: ISplitViewTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenPaneLengthMinusCompactLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NegativeOpenPaneLengthMinusCompactLength<Impl: ISplitViewTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NegativeOpenPaneLengthMinusCompactLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenPaneGridLength<Impl: ISplitViewTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::GridLength) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenPaneGridLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompactPaneGridLength<Impl: ISplitViewTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::GridLength) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CompactPaneGridLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISplitViewTemplateSettings>, base.5, OpenPaneLength::<Impl, OFFSET>, NegativeOpenPaneLength::<Impl, OFFSET>, OpenPaneLengthMinusCompactLength::<Impl, OFFSET>, NegativeOpenPaneLengthMinusCompactLength::<Impl, OFFSET>, OpenPaneGridLength::<Impl, OFFSET>, CompactPaneGridLength::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IThumbImpl: Sized {
    fn IsDragging(&self) -> ::windows::core::Result<bool>;
    fn DragStarted(&self, handler: &::core::option::Option<DragStartedEventHandler>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDragStarted(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DragDelta(&self, handler: &::core::option::Option<DragDeltaEventHandler>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDragDelta(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DragCompleted(&self, handler: &::core::option::Option<DragCompletedEventHandler>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDragCompleted(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CancelDrag(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IThumb {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IThumb";
}
#[cfg(feature = "implement_exclusive")]
impl IThumbVtbl {
    pub const fn new<Impl: IThumbImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IThumbVtbl {
        unsafe extern "system" fn IsDragging<Impl: IThumbImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsDragging() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DragStarted<Impl: IThumbImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DragStarted(&*(&handler as *const <DragStartedEventHandler as ::windows::core::Abi>::Abi as *const <DragStartedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDragStarted<Impl: IThumbImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveDragStarted(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DragDelta<Impl: IThumbImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DragDelta(&*(&handler as *const <DragDeltaEventHandler as ::windows::core::Abi>::Abi as *const <DragDeltaEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDragDelta<Impl: IThumbImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveDragDelta(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DragCompleted<Impl: IThumbImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DragCompleted(&*(&handler as *const <DragCompletedEventHandler as ::windows::core::Abi>::Abi as *const <DragCompletedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDragCompleted<Impl: IThumbImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveDragCompleted(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CancelDrag<Impl: IThumbImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).CancelDrag().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IThumb>, base.5, IsDragging::<Impl, OFFSET>, DragStarted::<Impl, OFFSET>, RemoveDragStarted::<Impl, OFFSET>, DragDelta::<Impl, OFFSET>, RemoveDragDelta::<Impl, OFFSET>, DragCompleted::<Impl, OFFSET>, RemoveDragCompleted::<Impl, OFFSET>, CancelDrag::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IThumbStaticsImpl: Sized {
    fn IsDraggingProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IThumbStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IThumbStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IThumbStaticsVtbl {
    pub const fn new<Impl: IThumbStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IThumbStaticsVtbl {
        unsafe extern "system" fn IsDraggingProperty<Impl: IThumbStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsDraggingProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IThumbStatics>, base.5, IsDraggingProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITickBarImpl: Sized {
    fn Fill(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetFill(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITickBar {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ITickBar";
}
#[cfg(feature = "implement_exclusive")]
impl ITickBarVtbl {
    pub const fn new<Impl: ITickBarImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITickBarVtbl {
        unsafe extern "system" fn Fill<Impl: ITickBarImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Fill() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFill<Impl: ITickBarImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetFill(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITickBar>, base.5, Fill::<Impl, OFFSET>, SetFill::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITickBarStaticsImpl: Sized {
    fn FillProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITickBarStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ITickBarStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ITickBarStaticsVtbl {
    pub const fn new<Impl: ITickBarStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITickBarStaticsVtbl {
        unsafe extern "system" fn FillProperty<Impl: ITickBarStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FillProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITickBarStatics>, base.5, FillProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IToggleButtonImpl: Sized {
    fn IsChecked(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<bool>>;
    fn SetIsChecked(&self, value: &::core::option::Option<super::super::super::super::Foundation::IReference<bool>>) -> ::windows::core::Result<()>;
    fn IsThreeState(&self) -> ::windows::core::Result<bool>;
    fn SetIsThreeState(&self, value: bool) -> ::windows::core::Result<()>;
    fn Checked(&self, handler: &::core::option::Option<super::super::RoutedEventHandler>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveChecked(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Unchecked(&self, handler: &::core::option::Option<super::super::RoutedEventHandler>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveUnchecked(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Indeterminate(&self, handler: &::core::option::Option<super::super::RoutedEventHandler>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveIndeterminate(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToggleButton {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IToggleButton";
}
#[cfg(feature = "implement_exclusive")]
impl IToggleButtonVtbl {
    pub const fn new<Impl: IToggleButtonImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IToggleButtonVtbl {
        unsafe extern "system" fn IsChecked<Impl: IToggleButtonImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsChecked() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsChecked<Impl: IToggleButtonImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsChecked(&*(&value as *const <super::super::super::super::Foundation::IReference<bool> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::IReference<bool> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsThreeState<Impl: IToggleButtonImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsThreeState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsThreeState<Impl: IToggleButtonImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsThreeState(value).into()
        }
        unsafe extern "system" fn Checked<Impl: IToggleButtonImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Checked(&*(&handler as *const <super::super::RoutedEventHandler as ::windows::core::Abi>::Abi as *const <super::super::RoutedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveChecked<Impl: IToggleButtonImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveChecked(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Unchecked<Impl: IToggleButtonImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Unchecked(&*(&handler as *const <super::super::RoutedEventHandler as ::windows::core::Abi>::Abi as *const <super::super::RoutedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUnchecked<Impl: IToggleButtonImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveUnchecked(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Indeterminate<Impl: IToggleButtonImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Indeterminate(&*(&handler as *const <super::super::RoutedEventHandler as ::windows::core::Abi>::Abi as *const <super::super::RoutedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveIndeterminate<Impl: IToggleButtonImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveIndeterminate(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IToggleButton>, base.5, IsChecked::<Impl, OFFSET>, SetIsChecked::<Impl, OFFSET>, IsThreeState::<Impl, OFFSET>, SetIsThreeState::<Impl, OFFSET>, Checked::<Impl, OFFSET>, RemoveChecked::<Impl, OFFSET>, Unchecked::<Impl, OFFSET>, RemoveUnchecked::<Impl, OFFSET>, Indeterminate::<Impl, OFFSET>, RemoveIndeterminate::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IToggleButtonFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ToggleButton>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToggleButtonFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IToggleButtonFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IToggleButtonFactoryVtbl {
    pub const fn new<Impl: IToggleButtonFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IToggleButtonFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IToggleButtonFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IToggleButtonFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IToggleButtonOverridesImpl: Sized {
    fn OnToggle(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToggleButtonOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IToggleButtonOverrides";
}
#[cfg(feature = "implement_exclusive")]
impl IToggleButtonOverridesVtbl {
    pub const fn new<Impl: IToggleButtonOverridesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IToggleButtonOverridesVtbl {
        unsafe extern "system" fn OnToggle<Impl: IToggleButtonOverridesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).OnToggle().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IToggleButtonOverrides>, base.5, OnToggle::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IToggleButtonStaticsImpl: Sized {
    fn IsCheckedProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn IsThreeStateProperty(&self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToggleButtonStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IToggleButtonStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IToggleButtonStaticsVtbl {
    pub const fn new<Impl: IToggleButtonStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IToggleButtonStaticsVtbl {
        unsafe extern "system" fn IsCheckedProperty<Impl: IToggleButtonStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsCheckedProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsThreeStateProperty<Impl: IToggleButtonStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsThreeStateProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IToggleButtonStatics>, base.5, IsCheckedProperty::<Impl, OFFSET>, IsThreeStateProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IToggleSwitchTemplateSettingsImpl: Sized {
    fn KnobCurrentToOnOffset(&self) -> ::windows::core::Result<f64>;
    fn KnobCurrentToOffOffset(&self) -> ::windows::core::Result<f64>;
    fn KnobOnToOffOffset(&self) -> ::windows::core::Result<f64>;
    fn KnobOffToOnOffset(&self) -> ::windows::core::Result<f64>;
    fn CurtainCurrentToOnOffset(&self) -> ::windows::core::Result<f64>;
    fn CurtainCurrentToOffOffset(&self) -> ::windows::core::Result<f64>;
    fn CurtainOnToOffOffset(&self) -> ::windows::core::Result<f64>;
    fn CurtainOffToOnOffset(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToggleSwitchTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IToggleSwitchTemplateSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IToggleSwitchTemplateSettingsVtbl {
    pub const fn new<Impl: IToggleSwitchTemplateSettingsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IToggleSwitchTemplateSettingsVtbl {
        unsafe extern "system" fn KnobCurrentToOnOffset<Impl: IToggleSwitchTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KnobCurrentToOnOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KnobCurrentToOffOffset<Impl: IToggleSwitchTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KnobCurrentToOffOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KnobOnToOffOffset<Impl: IToggleSwitchTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KnobOnToOffOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KnobOffToOnOffset<Impl: IToggleSwitchTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KnobOffToOnOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurtainCurrentToOnOffset<Impl: IToggleSwitchTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CurtainCurrentToOnOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurtainCurrentToOffOffset<Impl: IToggleSwitchTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CurtainCurrentToOffOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurtainOnToOffOffset<Impl: IToggleSwitchTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CurtainOnToOffOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurtainOffToOnOffset<Impl: IToggleSwitchTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CurtainOffToOnOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IToggleSwitchTemplateSettings>, base.5, KnobCurrentToOnOffset::<Impl, OFFSET>, KnobCurrentToOffOffset::<Impl, OFFSET>, KnobOnToOffOffset::<Impl, OFFSET>, KnobOffToOnOffset::<Impl, OFFSET>, CurtainCurrentToOnOffset::<Impl, OFFSET>, CurtainCurrentToOffOffset::<Impl, OFFSET>, CurtainOnToOffOffset::<Impl, OFFSET>, CurtainOffToOnOffset::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IToolTipTemplateSettingsImpl: Sized {
    fn FromHorizontalOffset(&self) -> ::windows::core::Result<f64>;
    fn FromVerticalOffset(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToolTipTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IToolTipTemplateSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IToolTipTemplateSettingsVtbl {
    pub const fn new<Impl: IToolTipTemplateSettingsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IToolTipTemplateSettingsVtbl {
        unsafe extern "system" fn FromHorizontalOffset<Impl: IToolTipTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromHorizontalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromVerticalOffset<Impl: IToolTipTemplateSettingsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FromVerticalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IToolTipTemplateSettings>, base.5, FromHorizontalOffset::<Impl, OFFSET>, FromVerticalOffset::<Impl, OFFSET>)
    }
}

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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBarButtonTemplateSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBarButtonTemplateSettingsVtbl {
        unsafe extern "system" fn KeyboardAcceleratorTextMinWidth<Impl: IAppBarButtonTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyboardAcceleratorTextMinWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBarButtonTemplateSettings, BASE_OFFSET>(),
            KeyboardAcceleratorTextMinWidth: KeyboardAcceleratorTextMinWidth::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBarButtonTemplateSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppBarTemplateSettingsImpl: Sized {
    fn ClipRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect>;
    fn CompactVerticalDelta(&self) -> ::windows::core::Result<f64>;
    fn CompactRootMargin(&self) -> ::windows::core::Result<super::super::Thickness>;
    fn MinimalVerticalDelta(&self) -> ::windows::core::Result<f64>;
    fn MinimalRootMargin(&self) -> ::windows::core::Result<super::super::Thickness>;
    fn HiddenVerticalDelta(&self) -> ::windows::core::Result<f64>;
    fn HiddenRootMargin(&self) -> ::windows::core::Result<super::super::Thickness>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppBarTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IAppBarTemplateSettings";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppBarTemplateSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBarTemplateSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBarTemplateSettingsVtbl {
        unsafe extern "system" fn ClipRect<Impl: IAppBarTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClipRect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompactVerticalDelta<Impl: IAppBarTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompactVerticalDelta() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompactRootMargin<Impl: IAppBarTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompactRootMargin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinimalVerticalDelta<Impl: IAppBarTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinimalVerticalDelta() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinimalRootMargin<Impl: IAppBarTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinimalRootMargin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HiddenVerticalDelta<Impl: IAppBarTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HiddenVerticalDelta() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HiddenRootMargin<Impl: IAppBarTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HiddenRootMargin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBarTemplateSettings, BASE_OFFSET>(),
            ClipRect: ClipRect::<Impl, IMPL_OFFSET>,
            CompactVerticalDelta: CompactVerticalDelta::<Impl, IMPL_OFFSET>,
            CompactRootMargin: CompactRootMargin::<Impl, IMPL_OFFSET>,
            MinimalVerticalDelta: MinimalVerticalDelta::<Impl, IMPL_OFFSET>,
            MinimalRootMargin: MinimalRootMargin::<Impl, IMPL_OFFSET>,
            HiddenVerticalDelta: HiddenVerticalDelta::<Impl, IMPL_OFFSET>,
            HiddenRootMargin: HiddenRootMargin::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBarTemplateSettings as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBarTemplateSettings2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBarTemplateSettings2Vtbl {
        unsafe extern "system" fn NegativeCompactVerticalDelta<Impl: IAppBarTemplateSettings2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NegativeCompactVerticalDelta() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NegativeMinimalVerticalDelta<Impl: IAppBarTemplateSettings2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NegativeMinimalVerticalDelta() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NegativeHiddenVerticalDelta<Impl: IAppBarTemplateSettings2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NegativeHiddenVerticalDelta() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBarTemplateSettings2, BASE_OFFSET>(),
            NegativeCompactVerticalDelta: NegativeCompactVerticalDelta::<Impl, IMPL_OFFSET>,
            NegativeMinimalVerticalDelta: NegativeMinimalVerticalDelta::<Impl, IMPL_OFFSET>,
            NegativeHiddenVerticalDelta: NegativeHiddenVerticalDelta::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBarTemplateSettings2 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBarToggleButtonTemplateSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBarToggleButtonTemplateSettingsVtbl {
        unsafe extern "system" fn KeyboardAcceleratorTextMinWidth<Impl: IAppBarToggleButtonTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyboardAcceleratorTextMinWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppBarToggleButtonTemplateSettings, BASE_OFFSET>(),
            KeyboardAcceleratorTextMinWidth: KeyboardAcceleratorTextMinWidth::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppBarToggleButtonTemplateSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "UI_Xaml_Input", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "Foundation", feature = "UI_Xaml_Input", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IButtonBase {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IButtonBase";
}
#[cfg(all(feature = "Foundation", feature = "UI_Xaml_Input", feature = "implement_exclusive"))]
impl IButtonBaseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IButtonBaseImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IButtonBaseVtbl {
        unsafe extern "system" fn ClickMode<Impl: IButtonBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::ClickMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClickMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClickMode<Impl: IButtonBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::ClickMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClickMode(value).into()
        }
        unsafe extern "system" fn IsPointerOver<Impl: IButtonBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPointerOver() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPressed<Impl: IButtonBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPressed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Command<Impl: IButtonBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Command() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCommand<Impl: IButtonBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCommand(&*(&value as *const <super::super::Input::ICommand as ::windows::core::Abi>::Abi as *const <super::super::Input::ICommand as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CommandParameter<Impl: IButtonBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommandParameter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCommandParameter<Impl: IButtonBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCommandParameter(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Click<Impl: IButtonBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Click(&*(&handler as *const <super::super::RoutedEventHandler as ::windows::core::Abi>::Abi as *const <super::super::RoutedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveClick<Impl: IButtonBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveClick(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IButtonBase, BASE_OFFSET>(),
            ClickMode: ClickMode::<Impl, IMPL_OFFSET>,
            SetClickMode: SetClickMode::<Impl, IMPL_OFFSET>,
            IsPointerOver: IsPointerOver::<Impl, IMPL_OFFSET>,
            IsPressed: IsPressed::<Impl, IMPL_OFFSET>,
            Command: Command::<Impl, IMPL_OFFSET>,
            SetCommand: SetCommand::<Impl, IMPL_OFFSET>,
            CommandParameter: CommandParameter::<Impl, IMPL_OFFSET>,
            SetCommandParameter: SetCommandParameter::<Impl, IMPL_OFFSET>,
            Click: Click::<Impl, IMPL_OFFSET>,
            RemoveClick: RemoveClick::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IButtonBase as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IButtonBaseFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IButtonBaseFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IButtonBaseFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IButtonBaseFactory, BASE_OFFSET>(), CreateInstance: CreateInstance::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IButtonBaseFactory as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IButtonBaseStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IButtonBaseStaticsVtbl {
        unsafe extern "system" fn ClickModeProperty<Impl: IButtonBaseStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClickModeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPointerOverProperty<Impl: IButtonBaseStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPointerOverProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPressedProperty<Impl: IButtonBaseStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPressedProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommandProperty<Impl: IButtonBaseStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommandProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommandParameterProperty<Impl: IButtonBaseStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommandParameterProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IButtonBaseStatics, BASE_OFFSET>(),
            ClickModeProperty: ClickModeProperty::<Impl, IMPL_OFFSET>,
            IsPointerOverProperty: IsPointerOverProperty::<Impl, IMPL_OFFSET>,
            IsPressedProperty: IsPressedProperty::<Impl, IMPL_OFFSET>,
            CommandProperty: CommandProperty::<Impl, IMPL_OFFSET>,
            CommandParameterProperty: CommandParameterProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IButtonBaseStatics as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICalendarPanelImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICalendarPanelVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICalendarPanel, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICalendarPanel as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICalendarViewTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ICalendarViewTemplateSettings";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICalendarViewTemplateSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICalendarViewTemplateSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICalendarViewTemplateSettingsVtbl {
        unsafe extern "system" fn MinViewWidth<Impl: ICalendarViewTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinViewWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HeaderText<Impl: ICalendarViewTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HeaderText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WeekDay1<Impl: ICalendarViewTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WeekDay1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WeekDay2<Impl: ICalendarViewTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WeekDay2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WeekDay3<Impl: ICalendarViewTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WeekDay3() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WeekDay4<Impl: ICalendarViewTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WeekDay4() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WeekDay5<Impl: ICalendarViewTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WeekDay5() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WeekDay6<Impl: ICalendarViewTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WeekDay6() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WeekDay7<Impl: ICalendarViewTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WeekDay7() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasMoreContentAfter<Impl: ICalendarViewTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasMoreContentAfter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasMoreContentBefore<Impl: ICalendarViewTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasMoreContentBefore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasMoreViews<Impl: ICalendarViewTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasMoreViews() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClipRect<Impl: ICalendarViewTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClipRect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CenterX<Impl: ICalendarViewTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CenterX() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CenterY<Impl: ICalendarViewTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CenterY() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICalendarViewTemplateSettings, BASE_OFFSET>(),
            MinViewWidth: MinViewWidth::<Impl, IMPL_OFFSET>,
            HeaderText: HeaderText::<Impl, IMPL_OFFSET>,
            WeekDay1: WeekDay1::<Impl, IMPL_OFFSET>,
            WeekDay2: WeekDay2::<Impl, IMPL_OFFSET>,
            WeekDay3: WeekDay3::<Impl, IMPL_OFFSET>,
            WeekDay4: WeekDay4::<Impl, IMPL_OFFSET>,
            WeekDay5: WeekDay5::<Impl, IMPL_OFFSET>,
            WeekDay6: WeekDay6::<Impl, IMPL_OFFSET>,
            WeekDay7: WeekDay7::<Impl, IMPL_OFFSET>,
            HasMoreContentAfter: HasMoreContentAfter::<Impl, IMPL_OFFSET>,
            HasMoreContentBefore: HasMoreContentBefore::<Impl, IMPL_OFFSET>,
            HasMoreViews: HasMoreViews::<Impl, IMPL_OFFSET>,
            ClipRect: ClipRect::<Impl, IMPL_OFFSET>,
            CenterX: CenterX::<Impl, IMPL_OFFSET>,
            CenterY: CenterY::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICalendarViewTemplateSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICarouselPanel {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ICarouselPanel";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICarouselPanelVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICarouselPanelImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICarouselPanelVtbl {
        unsafe extern "system" fn CanVerticallyScroll<Impl: ICarouselPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanVerticallyScroll() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCanVerticallyScroll<Impl: ICarouselPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCanVerticallyScroll(value).into()
        }
        unsafe extern "system" fn CanHorizontallyScroll<Impl: ICarouselPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanHorizontallyScroll() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCanHorizontallyScroll<Impl: ICarouselPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCanHorizontallyScroll(value).into()
        }
        unsafe extern "system" fn ExtentWidth<Impl: ICarouselPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtentWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtentHeight<Impl: ICarouselPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtentHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ViewportWidth<Impl: ICarouselPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ViewportWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ViewportHeight<Impl: ICarouselPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ViewportHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HorizontalOffset<Impl: ICarouselPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HorizontalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerticalOffset<Impl: ICarouselPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VerticalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScrollOwner<Impl: ICarouselPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScrollOwner() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScrollOwner<Impl: ICarouselPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScrollOwner(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LineUp<Impl: ICarouselPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LineUp().into()
        }
        unsafe extern "system" fn LineDown<Impl: ICarouselPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LineDown().into()
        }
        unsafe extern "system" fn LineLeft<Impl: ICarouselPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LineLeft().into()
        }
        unsafe extern "system" fn LineRight<Impl: ICarouselPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LineRight().into()
        }
        unsafe extern "system" fn PageUp<Impl: ICarouselPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PageUp().into()
        }
        unsafe extern "system" fn PageDown<Impl: ICarouselPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PageDown().into()
        }
        unsafe extern "system" fn PageLeft<Impl: ICarouselPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PageLeft().into()
        }
        unsafe extern "system" fn PageRight<Impl: ICarouselPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PageRight().into()
        }
        unsafe extern "system" fn MouseWheelUp<Impl: ICarouselPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MouseWheelUp().into()
        }
        unsafe extern "system" fn MouseWheelDown<Impl: ICarouselPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MouseWheelDown().into()
        }
        unsafe extern "system" fn MouseWheelLeft<Impl: ICarouselPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MouseWheelLeft().into()
        }
        unsafe extern "system" fn MouseWheelRight<Impl: ICarouselPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MouseWheelRight().into()
        }
        unsafe extern "system" fn SetHorizontalOffset<Impl: ICarouselPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHorizontalOffset(offset).into()
        }
        unsafe extern "system" fn SetVerticalOffset<Impl: ICarouselPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVerticalOffset(offset).into()
        }
        unsafe extern "system" fn MakeVisible<Impl: ICarouselPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visual: ::windows::core::RawPtr, rectangle: super::super::super::super::Foundation::Rect, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MakeVisible(&*(&visual as *const <super::super::UIElement as ::windows::core::Abi>::Abi as *const <super::super::UIElement as ::windows::core::DefaultType>::DefaultType), &*(&rectangle as *const <super::super::super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICarouselPanel, BASE_OFFSET>(),
            CanVerticallyScroll: CanVerticallyScroll::<Impl, IMPL_OFFSET>,
            SetCanVerticallyScroll: SetCanVerticallyScroll::<Impl, IMPL_OFFSET>,
            CanHorizontallyScroll: CanHorizontallyScroll::<Impl, IMPL_OFFSET>,
            SetCanHorizontallyScroll: SetCanHorizontallyScroll::<Impl, IMPL_OFFSET>,
            ExtentWidth: ExtentWidth::<Impl, IMPL_OFFSET>,
            ExtentHeight: ExtentHeight::<Impl, IMPL_OFFSET>,
            ViewportWidth: ViewportWidth::<Impl, IMPL_OFFSET>,
            ViewportHeight: ViewportHeight::<Impl, IMPL_OFFSET>,
            HorizontalOffset: HorizontalOffset::<Impl, IMPL_OFFSET>,
            VerticalOffset: VerticalOffset::<Impl, IMPL_OFFSET>,
            ScrollOwner: ScrollOwner::<Impl, IMPL_OFFSET>,
            SetScrollOwner: SetScrollOwner::<Impl, IMPL_OFFSET>,
            LineUp: LineUp::<Impl, IMPL_OFFSET>,
            LineDown: LineDown::<Impl, IMPL_OFFSET>,
            LineLeft: LineLeft::<Impl, IMPL_OFFSET>,
            LineRight: LineRight::<Impl, IMPL_OFFSET>,
            PageUp: PageUp::<Impl, IMPL_OFFSET>,
            PageDown: PageDown::<Impl, IMPL_OFFSET>,
            PageLeft: PageLeft::<Impl, IMPL_OFFSET>,
            PageRight: PageRight::<Impl, IMPL_OFFSET>,
            MouseWheelUp: MouseWheelUp::<Impl, IMPL_OFFSET>,
            MouseWheelDown: MouseWheelDown::<Impl, IMPL_OFFSET>,
            MouseWheelLeft: MouseWheelLeft::<Impl, IMPL_OFFSET>,
            MouseWheelRight: MouseWheelRight::<Impl, IMPL_OFFSET>,
            SetHorizontalOffset: SetHorizontalOffset::<Impl, IMPL_OFFSET>,
            SetVerticalOffset: SetVerticalOffset::<Impl, IMPL_OFFSET>,
            MakeVisible: MakeVisible::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICarouselPanel as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICarouselPanelFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICarouselPanelFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: ICarouselPanelFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICarouselPanelFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICarouselPanelFactory as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColorPickerSliderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IColorPickerSliderVtbl {
        unsafe extern "system" fn ColorChannel<Impl: IColorPickerSliderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::ColorPickerHsvChannel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ColorChannel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorChannel<Impl: IColorPickerSliderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::ColorPickerHsvChannel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColorChannel(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IColorPickerSlider, BASE_OFFSET>(),
            ColorChannel: ColorChannel::<Impl, IMPL_OFFSET>,
            SetColorChannel: SetColorChannel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IColorPickerSlider as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColorPickerSliderFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IColorPickerSliderFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IColorPickerSliderFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IColorPickerSliderFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IColorPickerSliderFactory as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColorPickerSliderStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IColorPickerSliderStaticsVtbl {
        unsafe extern "system" fn ColorChannelProperty<Impl: IColorPickerSliderStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ColorChannelProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IColorPickerSliderStatics, BASE_OFFSET>(),
            ColorChannelProperty: ColorChannelProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IColorPickerSliderStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IColorSpectrum {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IColorSpectrum";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IColorSpectrumVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColorSpectrumImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IColorSpectrumVtbl {
        unsafe extern "system" fn Color<Impl: IColorSpectrumImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Color() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColor<Impl: IColorSpectrumImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColor(&*(&value as *const <super::super::super::Color as ::windows::core::Abi>::Abi as *const <super::super::super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn HsvColor<Impl: IColorSpectrumImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Numerics::Vector4) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HsvColor() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHsvColor<Impl: IColorSpectrumImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::super::Foundation::Numerics::Vector4) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHsvColor(&*(&value as *const <super::super::super::super::Foundation::Numerics::Vector4 as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Numerics::Vector4 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MinHue<Impl: IColorSpectrumImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinHue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinHue<Impl: IColorSpectrumImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMinHue(value).into()
        }
        unsafe extern "system" fn MaxHue<Impl: IColorSpectrumImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxHue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxHue<Impl: IColorSpectrumImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxHue(value).into()
        }
        unsafe extern "system" fn MinSaturation<Impl: IColorSpectrumImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinSaturation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinSaturation<Impl: IColorSpectrumImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMinSaturation(value).into()
        }
        unsafe extern "system" fn MaxSaturation<Impl: IColorSpectrumImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxSaturation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxSaturation<Impl: IColorSpectrumImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxSaturation(value).into()
        }
        unsafe extern "system" fn MinValue<Impl: IColorSpectrumImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinValue<Impl: IColorSpectrumImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMinValue(value).into()
        }
        unsafe extern "system" fn MaxValue<Impl: IColorSpectrumImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxValue<Impl: IColorSpectrumImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxValue(value).into()
        }
        unsafe extern "system" fn Shape<Impl: IColorSpectrumImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::ColorSpectrumShape) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Shape() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShape<Impl: IColorSpectrumImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::ColorSpectrumShape) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShape(value).into()
        }
        unsafe extern "system" fn Components<Impl: IColorSpectrumImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::ColorSpectrumComponents) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Components() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetComponents<Impl: IColorSpectrumImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::ColorSpectrumComponents) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetComponents(value).into()
        }
        unsafe extern "system" fn ColorChanged<Impl: IColorSpectrumImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ColorChanged(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<ColorSpectrum, super::ColorChangedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<ColorSpectrum, super::ColorChangedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveColorChanged<Impl: IColorSpectrumImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveColorChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IColorSpectrum, BASE_OFFSET>(),
            Color: Color::<Impl, IMPL_OFFSET>,
            SetColor: SetColor::<Impl, IMPL_OFFSET>,
            HsvColor: HsvColor::<Impl, IMPL_OFFSET>,
            SetHsvColor: SetHsvColor::<Impl, IMPL_OFFSET>,
            MinHue: MinHue::<Impl, IMPL_OFFSET>,
            SetMinHue: SetMinHue::<Impl, IMPL_OFFSET>,
            MaxHue: MaxHue::<Impl, IMPL_OFFSET>,
            SetMaxHue: SetMaxHue::<Impl, IMPL_OFFSET>,
            MinSaturation: MinSaturation::<Impl, IMPL_OFFSET>,
            SetMinSaturation: SetMinSaturation::<Impl, IMPL_OFFSET>,
            MaxSaturation: MaxSaturation::<Impl, IMPL_OFFSET>,
            SetMaxSaturation: SetMaxSaturation::<Impl, IMPL_OFFSET>,
            MinValue: MinValue::<Impl, IMPL_OFFSET>,
            SetMinValue: SetMinValue::<Impl, IMPL_OFFSET>,
            MaxValue: MaxValue::<Impl, IMPL_OFFSET>,
            SetMaxValue: SetMaxValue::<Impl, IMPL_OFFSET>,
            Shape: Shape::<Impl, IMPL_OFFSET>,
            SetShape: SetShape::<Impl, IMPL_OFFSET>,
            Components: Components::<Impl, IMPL_OFFSET>,
            SetComponents: SetComponents::<Impl, IMPL_OFFSET>,
            ColorChanged: ColorChanged::<Impl, IMPL_OFFSET>,
            RemoveColorChanged: RemoveColorChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IColorSpectrum as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColorSpectrumFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IColorSpectrumFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IColorSpectrumFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IColorSpectrumFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IColorSpectrumFactory as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColorSpectrumStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IColorSpectrumStaticsVtbl {
        unsafe extern "system" fn ColorProperty<Impl: IColorSpectrumStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ColorProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HsvColorProperty<Impl: IColorSpectrumStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HsvColorProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinHueProperty<Impl: IColorSpectrumStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinHueProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxHueProperty<Impl: IColorSpectrumStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxHueProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinSaturationProperty<Impl: IColorSpectrumStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinSaturationProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxSaturationProperty<Impl: IColorSpectrumStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxSaturationProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinValueProperty<Impl: IColorSpectrumStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinValueProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxValueProperty<Impl: IColorSpectrumStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxValueProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShapeProperty<Impl: IColorSpectrumStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ComponentsProperty<Impl: IColorSpectrumStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ComponentsProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IColorSpectrumStatics, BASE_OFFSET>(),
            ColorProperty: ColorProperty::<Impl, IMPL_OFFSET>,
            HsvColorProperty: HsvColorProperty::<Impl, IMPL_OFFSET>,
            MinHueProperty: MinHueProperty::<Impl, IMPL_OFFSET>,
            MaxHueProperty: MaxHueProperty::<Impl, IMPL_OFFSET>,
            MinSaturationProperty: MinSaturationProperty::<Impl, IMPL_OFFSET>,
            MaxSaturationProperty: MaxSaturationProperty::<Impl, IMPL_OFFSET>,
            MinValueProperty: MinValueProperty::<Impl, IMPL_OFFSET>,
            MaxValueProperty: MaxValueProperty::<Impl, IMPL_OFFSET>,
            ShapeProperty: ShapeProperty::<Impl, IMPL_OFFSET>,
            ComponentsProperty: ComponentsProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IColorSpectrumStatics as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComboBoxTemplateSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComboBoxTemplateSettingsVtbl {
        unsafe extern "system" fn DropDownOpenedHeight<Impl: IComboBoxTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DropDownOpenedHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DropDownClosedHeight<Impl: IComboBoxTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DropDownClosedHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DropDownOffset<Impl: IComboBoxTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DropDownOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedItemDirection<Impl: IComboBoxTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AnimationDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedItemDirection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IComboBoxTemplateSettings, BASE_OFFSET>(),
            DropDownOpenedHeight: DropDownOpenedHeight::<Impl, IMPL_OFFSET>,
            DropDownClosedHeight: DropDownClosedHeight::<Impl, IMPL_OFFSET>,
            DropDownOffset: DropDownOffset::<Impl, IMPL_OFFSET>,
            SelectedItemDirection: SelectedItemDirection::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComboBoxTemplateSettings as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComboBoxTemplateSettings2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComboBoxTemplateSettings2Vtbl {
        unsafe extern "system" fn DropDownContentMinWidth<Impl: IComboBoxTemplateSettings2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DropDownContentMinWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IComboBoxTemplateSettings2, BASE_OFFSET>(),
            DropDownContentMinWidth: DropDownContentMinWidth::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComboBoxTemplateSettings2 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommandBarFlyoutCommandBarImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICommandBarFlyoutCommandBarVtbl {
        unsafe extern "system" fn FlyoutTemplateSettings<Impl: ICommandBarFlyoutCommandBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FlyoutTemplateSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICommandBarFlyoutCommandBar, BASE_OFFSET>(),
            FlyoutTemplateSettings: FlyoutTemplateSettings::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommandBarFlyoutCommandBar as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommandBarFlyoutCommandBarFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICommandBarFlyoutCommandBarFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: ICommandBarFlyoutCommandBarFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICommandBarFlyoutCommandBarFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommandBarFlyoutCommandBarFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICommandBarFlyoutCommandBarTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ICommandBarFlyoutCommandBarTemplateSettings";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICommandBarFlyoutCommandBarTemplateSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommandBarFlyoutCommandBarTemplateSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICommandBarFlyoutCommandBarTemplateSettingsVtbl {
        unsafe extern "system" fn OpenAnimationStartPosition<Impl: ICommandBarFlyoutCommandBarTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenAnimationStartPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenAnimationEndPosition<Impl: ICommandBarFlyoutCommandBarTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenAnimationEndPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloseAnimationEndPosition<Impl: ICommandBarFlyoutCommandBarTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CloseAnimationEndPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentWidth<Impl: ICommandBarFlyoutCommandBarTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpandedWidth<Impl: ICommandBarFlyoutCommandBarTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExpandedWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WidthExpansionDelta<Impl: ICommandBarFlyoutCommandBarTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WidthExpansionDelta() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WidthExpansionAnimationStartPosition<Impl: ICommandBarFlyoutCommandBarTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WidthExpansionAnimationStartPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WidthExpansionAnimationEndPosition<Impl: ICommandBarFlyoutCommandBarTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WidthExpansionAnimationEndPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WidthExpansionMoreButtonAnimationStartPosition<Impl: ICommandBarFlyoutCommandBarTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WidthExpansionMoreButtonAnimationStartPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WidthExpansionMoreButtonAnimationEndPosition<Impl: ICommandBarFlyoutCommandBarTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WidthExpansionMoreButtonAnimationEndPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpandUpOverflowVerticalPosition<Impl: ICommandBarFlyoutCommandBarTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExpandUpOverflowVerticalPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpandDownOverflowVerticalPosition<Impl: ICommandBarFlyoutCommandBarTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExpandDownOverflowVerticalPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpandUpAnimationStartPosition<Impl: ICommandBarFlyoutCommandBarTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExpandUpAnimationStartPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpandUpAnimationEndPosition<Impl: ICommandBarFlyoutCommandBarTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExpandUpAnimationEndPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpandUpAnimationHoldPosition<Impl: ICommandBarFlyoutCommandBarTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExpandUpAnimationHoldPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpandDownAnimationStartPosition<Impl: ICommandBarFlyoutCommandBarTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExpandDownAnimationStartPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpandDownAnimationEndPosition<Impl: ICommandBarFlyoutCommandBarTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExpandDownAnimationEndPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpandDownAnimationHoldPosition<Impl: ICommandBarFlyoutCommandBarTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExpandDownAnimationHoldPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentClipRect<Impl: ICommandBarFlyoutCommandBarTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentClipRect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OverflowContentClipRect<Impl: ICommandBarFlyoutCommandBarTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OverflowContentClipRect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICommandBarFlyoutCommandBarTemplateSettings, BASE_OFFSET>(),
            OpenAnimationStartPosition: OpenAnimationStartPosition::<Impl, IMPL_OFFSET>,
            OpenAnimationEndPosition: OpenAnimationEndPosition::<Impl, IMPL_OFFSET>,
            CloseAnimationEndPosition: CloseAnimationEndPosition::<Impl, IMPL_OFFSET>,
            CurrentWidth: CurrentWidth::<Impl, IMPL_OFFSET>,
            ExpandedWidth: ExpandedWidth::<Impl, IMPL_OFFSET>,
            WidthExpansionDelta: WidthExpansionDelta::<Impl, IMPL_OFFSET>,
            WidthExpansionAnimationStartPosition: WidthExpansionAnimationStartPosition::<Impl, IMPL_OFFSET>,
            WidthExpansionAnimationEndPosition: WidthExpansionAnimationEndPosition::<Impl, IMPL_OFFSET>,
            WidthExpansionMoreButtonAnimationStartPosition: WidthExpansionMoreButtonAnimationStartPosition::<Impl, IMPL_OFFSET>,
            WidthExpansionMoreButtonAnimationEndPosition: WidthExpansionMoreButtonAnimationEndPosition::<Impl, IMPL_OFFSET>,
            ExpandUpOverflowVerticalPosition: ExpandUpOverflowVerticalPosition::<Impl, IMPL_OFFSET>,
            ExpandDownOverflowVerticalPosition: ExpandDownOverflowVerticalPosition::<Impl, IMPL_OFFSET>,
            ExpandUpAnimationStartPosition: ExpandUpAnimationStartPosition::<Impl, IMPL_OFFSET>,
            ExpandUpAnimationEndPosition: ExpandUpAnimationEndPosition::<Impl, IMPL_OFFSET>,
            ExpandUpAnimationHoldPosition: ExpandUpAnimationHoldPosition::<Impl, IMPL_OFFSET>,
            ExpandDownAnimationStartPosition: ExpandDownAnimationStartPosition::<Impl, IMPL_OFFSET>,
            ExpandDownAnimationEndPosition: ExpandDownAnimationEndPosition::<Impl, IMPL_OFFSET>,
            ExpandDownAnimationHoldPosition: ExpandDownAnimationHoldPosition::<Impl, IMPL_OFFSET>,
            ContentClipRect: ContentClipRect::<Impl, IMPL_OFFSET>,
            OverflowContentClipRect: OverflowContentClipRect::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommandBarFlyoutCommandBarTemplateSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICommandBarTemplateSettingsImpl: Sized {
    fn ContentHeight(&self) -> ::windows::core::Result<f64>;
    fn OverflowContentClipRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect>;
    fn OverflowContentMinWidth(&self) -> ::windows::core::Result<f64>;
    fn OverflowContentMaxHeight(&self) -> ::windows::core::Result<f64>;
    fn OverflowContentHorizontalOffset(&self) -> ::windows::core::Result<f64>;
    fn OverflowContentHeight(&self) -> ::windows::core::Result<f64>;
    fn NegativeOverflowContentHeight(&self) -> ::windows::core::Result<f64>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICommandBarTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ICommandBarTemplateSettings";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICommandBarTemplateSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommandBarTemplateSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICommandBarTemplateSettingsVtbl {
        unsafe extern "system" fn ContentHeight<Impl: ICommandBarTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OverflowContentClipRect<Impl: ICommandBarTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OverflowContentClipRect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OverflowContentMinWidth<Impl: ICommandBarTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OverflowContentMinWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OverflowContentMaxHeight<Impl: ICommandBarTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OverflowContentMaxHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OverflowContentHorizontalOffset<Impl: ICommandBarTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OverflowContentHorizontalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OverflowContentHeight<Impl: ICommandBarTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OverflowContentHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NegativeOverflowContentHeight<Impl: ICommandBarTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NegativeOverflowContentHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICommandBarTemplateSettings, BASE_OFFSET>(),
            ContentHeight: ContentHeight::<Impl, IMPL_OFFSET>,
            OverflowContentClipRect: OverflowContentClipRect::<Impl, IMPL_OFFSET>,
            OverflowContentMinWidth: OverflowContentMinWidth::<Impl, IMPL_OFFSET>,
            OverflowContentMaxHeight: OverflowContentMaxHeight::<Impl, IMPL_OFFSET>,
            OverflowContentHorizontalOffset: OverflowContentHorizontalOffset::<Impl, IMPL_OFFSET>,
            OverflowContentHeight: OverflowContentHeight::<Impl, IMPL_OFFSET>,
            NegativeOverflowContentHeight: NegativeOverflowContentHeight::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommandBarTemplateSettings as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommandBarTemplateSettings2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICommandBarTemplateSettings2Vtbl {
        unsafe extern "system" fn OverflowContentMaxWidth<Impl: ICommandBarTemplateSettings2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OverflowContentMaxWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICommandBarTemplateSettings2, BASE_OFFSET>(),
            OverflowContentMaxWidth: OverflowContentMaxWidth::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommandBarTemplateSettings2 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommandBarTemplateSettings3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICommandBarTemplateSettings3Vtbl {
        unsafe extern "system" fn EffectiveOverflowButtonVisibility<Impl: ICommandBarTemplateSettings3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Visibility) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EffectiveOverflowButtonVisibility() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICommandBarTemplateSettings3, BASE_OFFSET>(),
            EffectiveOverflowButtonVisibility: EffectiveOverflowButtonVisibility::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommandBarTemplateSettings3 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommandBarTemplateSettings4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICommandBarTemplateSettings4Vtbl {
        unsafe extern "system" fn OverflowContentCompactYTranslation<Impl: ICommandBarTemplateSettings4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OverflowContentCompactYTranslation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OverflowContentMinimalYTranslation<Impl: ICommandBarTemplateSettings4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OverflowContentMinimalYTranslation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OverflowContentHiddenYTranslation<Impl: ICommandBarTemplateSettings4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OverflowContentHiddenYTranslation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICommandBarTemplateSettings4, BASE_OFFSET>(),
            OverflowContentCompactYTranslation: OverflowContentCompactYTranslation::<Impl, IMPL_OFFSET>,
            OverflowContentMinimalYTranslation: OverflowContentMinimalYTranslation::<Impl, IMPL_OFFSET>,
            OverflowContentHiddenYTranslation: OverflowContentHiddenYTranslation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommandBarTemplateSettings4 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDragCompletedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDragCompletedEventArgsVtbl {
        unsafe extern "system" fn HorizontalChange<Impl: IDragCompletedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HorizontalChange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerticalChange<Impl: IDragCompletedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VerticalChange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Canceled<Impl: IDragCompletedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Canceled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDragCompletedEventArgs, BASE_OFFSET>(),
            HorizontalChange: HorizontalChange::<Impl, IMPL_OFFSET>,
            VerticalChange: VerticalChange::<Impl, IMPL_OFFSET>,
            Canceled: Canceled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDragCompletedEventArgs as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDragCompletedEventArgsFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDragCompletedEventArgsFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithHorizontalChangeVerticalChangeAndCanceled<Impl: IDragCompletedEventArgsFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalchange: f64, verticalchange: f64, canceled: bool, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithHorizontalChangeVerticalChangeAndCanceled(horizontalchange, verticalchange, canceled, &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDragCompletedEventArgsFactory, BASE_OFFSET>(),
            CreateInstanceWithHorizontalChangeVerticalChangeAndCanceled: CreateInstanceWithHorizontalChangeVerticalChangeAndCanceled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDragCompletedEventArgsFactory as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDragDeltaEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDragDeltaEventArgsVtbl {
        unsafe extern "system" fn HorizontalChange<Impl: IDragDeltaEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HorizontalChange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerticalChange<Impl: IDragDeltaEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VerticalChange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDragDeltaEventArgs, BASE_OFFSET>(),
            HorizontalChange: HorizontalChange::<Impl, IMPL_OFFSET>,
            VerticalChange: VerticalChange::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDragDeltaEventArgs as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDragDeltaEventArgsFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDragDeltaEventArgsFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithHorizontalChangeAndVerticalChange<Impl: IDragDeltaEventArgsFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalchange: f64, verticalchange: f64, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithHorizontalChangeAndVerticalChange(horizontalchange, verticalchange, &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDragDeltaEventArgsFactory, BASE_OFFSET>(),
            CreateInstanceWithHorizontalChangeAndVerticalChange: CreateInstanceWithHorizontalChangeAndVerticalChange::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDragDeltaEventArgsFactory as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDragStartedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDragStartedEventArgsVtbl {
        unsafe extern "system" fn HorizontalOffset<Impl: IDragStartedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HorizontalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerticalOffset<Impl: IDragStartedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VerticalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDragStartedEventArgs, BASE_OFFSET>(),
            HorizontalOffset: HorizontalOffset::<Impl, IMPL_OFFSET>,
            VerticalOffset: VerticalOffset::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDragStartedEventArgs as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDragStartedEventArgsFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDragStartedEventArgsFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithHorizontalOffsetAndVerticalOffset<Impl: IDragStartedEventArgsFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontaloffset: f64, verticaloffset: f64, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithHorizontalOffsetAndVerticalOffset(horizontaloffset, verticaloffset, &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDragStartedEventArgsFactory, BASE_OFFSET>(),
            CreateInstanceWithHorizontalOffsetAndVerticalOffset: CreateInstanceWithHorizontalOffsetAndVerticalOffset::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDragStartedEventArgsFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFlyoutBase {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IFlyoutBase";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IFlyoutBaseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlyoutBaseImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlyoutBaseVtbl {
        unsafe extern "system" fn Placement<Impl: IFlyoutBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FlyoutPlacementMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Placement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlacement<Impl: IFlyoutBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FlyoutPlacementMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPlacement(value).into()
        }
        unsafe extern "system" fn Opened<Impl: IFlyoutBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Opened(&*(&handler as *const <super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveOpened<Impl: IFlyoutBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveOpened(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Closed<Impl: IFlyoutBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Closed(&*(&handler as *const <super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveClosed<Impl: IFlyoutBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveClosed(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Opening<Impl: IFlyoutBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Opening(&*(&handler as *const <super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveOpening<Impl: IFlyoutBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveOpening(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ShowAt<Impl: IFlyoutBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, placementtarget: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowAt(&*(&placementtarget as *const <super::super::FrameworkElement as ::windows::core::Abi>::Abi as *const <super::super::FrameworkElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Hide<Impl: IFlyoutBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Hide().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFlyoutBase, BASE_OFFSET>(),
            Placement: Placement::<Impl, IMPL_OFFSET>,
            SetPlacement: SetPlacement::<Impl, IMPL_OFFSET>,
            Opened: Opened::<Impl, IMPL_OFFSET>,
            RemoveOpened: RemoveOpened::<Impl, IMPL_OFFSET>,
            Closed: Closed::<Impl, IMPL_OFFSET>,
            RemoveClosed: RemoveClosed::<Impl, IMPL_OFFSET>,
            Opening: Opening::<Impl, IMPL_OFFSET>,
            RemoveOpening: RemoveOpening::<Impl, IMPL_OFFSET>,
            ShowAt: ShowAt::<Impl, IMPL_OFFSET>,
            Hide: Hide::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFlyoutBase as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFlyoutBase2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IFlyoutBase2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IFlyoutBase2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlyoutBase2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlyoutBase2Vtbl {
        unsafe extern "system" fn Target<Impl: IFlyoutBase2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Target() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowFocusOnInteraction<Impl: IFlyoutBase2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowFocusOnInteraction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowFocusOnInteraction<Impl: IFlyoutBase2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowFocusOnInteraction(value).into()
        }
        unsafe extern "system" fn LightDismissOverlayMode<Impl: IFlyoutBase2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::LightDismissOverlayMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LightDismissOverlayMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLightDismissOverlayMode<Impl: IFlyoutBase2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::LightDismissOverlayMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLightDismissOverlayMode(value).into()
        }
        unsafe extern "system" fn AllowFocusWhenDisabled<Impl: IFlyoutBase2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowFocusWhenDisabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowFocusWhenDisabled<Impl: IFlyoutBase2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowFocusWhenDisabled(value).into()
        }
        unsafe extern "system" fn ElementSoundMode<Impl: IFlyoutBase2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::ElementSoundMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ElementSoundMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetElementSoundMode<Impl: IFlyoutBase2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::ElementSoundMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetElementSoundMode(value).into()
        }
        unsafe extern "system" fn Closing<Impl: IFlyoutBase2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Closing(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<FlyoutBase, FlyoutBaseClosingEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<FlyoutBase, FlyoutBaseClosingEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveClosing<Impl: IFlyoutBase2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveClosing(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFlyoutBase2, BASE_OFFSET>(),
            Target: Target::<Impl, IMPL_OFFSET>,
            AllowFocusOnInteraction: AllowFocusOnInteraction::<Impl, IMPL_OFFSET>,
            SetAllowFocusOnInteraction: SetAllowFocusOnInteraction::<Impl, IMPL_OFFSET>,
            LightDismissOverlayMode: LightDismissOverlayMode::<Impl, IMPL_OFFSET>,
            SetLightDismissOverlayMode: SetLightDismissOverlayMode::<Impl, IMPL_OFFSET>,
            AllowFocusWhenDisabled: AllowFocusWhenDisabled::<Impl, IMPL_OFFSET>,
            SetAllowFocusWhenDisabled: SetAllowFocusWhenDisabled::<Impl, IMPL_OFFSET>,
            ElementSoundMode: ElementSoundMode::<Impl, IMPL_OFFSET>,
            SetElementSoundMode: SetElementSoundMode::<Impl, IMPL_OFFSET>,
            Closing: Closing::<Impl, IMPL_OFFSET>,
            RemoveClosing: RemoveClosing::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFlyoutBase2 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlyoutBase3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlyoutBase3Vtbl {
        unsafe extern "system" fn OverlayInputPassThroughElement<Impl: IFlyoutBase3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OverlayInputPassThroughElement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOverlayInputPassThroughElement<Impl: IFlyoutBase3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOverlayInputPassThroughElement(&*(&value as *const <super::super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFlyoutBase3, BASE_OFFSET>(),
            OverlayInputPassThroughElement: OverlayInputPassThroughElement::<Impl, IMPL_OFFSET>,
            SetOverlayInputPassThroughElement: SetOverlayInputPassThroughElement::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFlyoutBase3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Input", feature = "implement_exclusive"))]
pub trait IFlyoutBase4Impl: Sized {
    fn TryInvokeKeyboardAccelerator(&self, args: &::core::option::Option<super::super::Input::ProcessKeyboardAcceleratorEventArgs>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Xaml_Input", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFlyoutBase4 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IFlyoutBase4";
}
#[cfg(all(feature = "UI_Xaml_Input", feature = "implement_exclusive"))]
impl IFlyoutBase4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlyoutBase4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlyoutBase4Vtbl {
        unsafe extern "system" fn TryInvokeKeyboardAccelerator<Impl: IFlyoutBase4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TryInvokeKeyboardAccelerator(&*(&args as *const <super::super::Input::ProcessKeyboardAcceleratorEventArgs as ::windows::core::Abi>::Abi as *const <super::super::Input::ProcessKeyboardAcceleratorEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFlyoutBase4, BASE_OFFSET>(),
            TryInvokeKeyboardAccelerator: TryInvokeKeyboardAccelerator::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFlyoutBase4 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlyoutBase5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlyoutBase5Vtbl {
        unsafe extern "system" fn ShowMode<Impl: IFlyoutBase5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FlyoutShowMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowMode<Impl: IFlyoutBase5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FlyoutShowMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShowMode(value).into()
        }
        unsafe extern "system" fn InputDevicePrefersPrimaryCommands<Impl: IFlyoutBase5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InputDevicePrefersPrimaryCommands() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AreOpenCloseAnimationsEnabled<Impl: IFlyoutBase5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AreOpenCloseAnimationsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAreOpenCloseAnimationsEnabled<Impl: IFlyoutBase5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAreOpenCloseAnimationsEnabled(value).into()
        }
        unsafe extern "system" fn IsOpen<Impl: IFlyoutBase5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOpen() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowAt<Impl: IFlyoutBase5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, placementtarget: ::windows::core::RawPtr, showoptions: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowAt(&*(&placementtarget as *const <super::super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::super::DependencyObject as ::windows::core::DefaultType>::DefaultType), &*(&showoptions as *const <FlyoutShowOptions as ::windows::core::Abi>::Abi as *const <FlyoutShowOptions as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFlyoutBase5, BASE_OFFSET>(),
            ShowMode: ShowMode::<Impl, IMPL_OFFSET>,
            SetShowMode: SetShowMode::<Impl, IMPL_OFFSET>,
            InputDevicePrefersPrimaryCommands: InputDevicePrefersPrimaryCommands::<Impl, IMPL_OFFSET>,
            AreOpenCloseAnimationsEnabled: AreOpenCloseAnimationsEnabled::<Impl, IMPL_OFFSET>,
            SetAreOpenCloseAnimationsEnabled: SetAreOpenCloseAnimationsEnabled::<Impl, IMPL_OFFSET>,
            IsOpen: IsOpen::<Impl, IMPL_OFFSET>,
            ShowAt: ShowAt::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFlyoutBase5 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlyoutBase6Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlyoutBase6Vtbl {
        unsafe extern "system" fn ShouldConstrainToRootBounds<Impl: IFlyoutBase6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShouldConstrainToRootBounds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShouldConstrainToRootBounds<Impl: IFlyoutBase6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShouldConstrainToRootBounds(value).into()
        }
        unsafe extern "system" fn IsConstrainedToRootBounds<Impl: IFlyoutBase6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsConstrainedToRootBounds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn XamlRoot<Impl: IFlyoutBase6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XamlRoot() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetXamlRoot<Impl: IFlyoutBase6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetXamlRoot(&*(&value as *const <super::super::XamlRoot as ::windows::core::Abi>::Abi as *const <super::super::XamlRoot as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFlyoutBase6, BASE_OFFSET>(),
            ShouldConstrainToRootBounds: ShouldConstrainToRootBounds::<Impl, IMPL_OFFSET>,
            SetShouldConstrainToRootBounds: SetShouldConstrainToRootBounds::<Impl, IMPL_OFFSET>,
            IsConstrainedToRootBounds: IsConstrainedToRootBounds::<Impl, IMPL_OFFSET>,
            XamlRoot: XamlRoot::<Impl, IMPL_OFFSET>,
            SetXamlRoot: SetXamlRoot::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFlyoutBase6 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlyoutBaseClosingEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlyoutBaseClosingEventArgsVtbl {
        unsafe extern "system" fn Cancel<Impl: IFlyoutBaseClosingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cancel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCancel<Impl: IFlyoutBaseClosingEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCancel(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFlyoutBaseClosingEventArgs, BASE_OFFSET>(),
            Cancel: Cancel::<Impl, IMPL_OFFSET>,
            SetCancel: SetCancel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFlyoutBaseClosingEventArgs as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlyoutBaseFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlyoutBaseFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IFlyoutBaseFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IFlyoutBaseFactory, BASE_OFFSET>(), CreateInstance: CreateInstance::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFlyoutBaseFactory as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlyoutBaseOverridesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlyoutBaseOverridesVtbl {
        unsafe extern "system" fn CreatePresenter<Impl: IFlyoutBaseOverridesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePresenter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFlyoutBaseOverrides, BASE_OFFSET>(),
            CreatePresenter: CreatePresenter::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFlyoutBaseOverrides as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Input", feature = "implement_exclusive"))]
pub trait IFlyoutBaseOverrides4Impl: Sized {
    fn OnProcessKeyboardAccelerators(&self, args: &::core::option::Option<super::super::Input::ProcessKeyboardAcceleratorEventArgs>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Xaml_Input", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFlyoutBaseOverrides4 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IFlyoutBaseOverrides4";
}
#[cfg(all(feature = "UI_Xaml_Input", feature = "implement_exclusive"))]
impl IFlyoutBaseOverrides4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlyoutBaseOverrides4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlyoutBaseOverrides4Vtbl {
        unsafe extern "system" fn OnProcessKeyboardAccelerators<Impl: IFlyoutBaseOverrides4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnProcessKeyboardAccelerators(&*(&args as *const <super::super::Input::ProcessKeyboardAcceleratorEventArgs as ::windows::core::Abi>::Abi as *const <super::super::Input::ProcessKeyboardAcceleratorEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFlyoutBaseOverrides4, BASE_OFFSET>(),
            OnProcessKeyboardAccelerators: OnProcessKeyboardAccelerators::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFlyoutBaseOverrides4 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlyoutBaseStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlyoutBaseStaticsVtbl {
        unsafe extern "system" fn PlacementProperty<Impl: IFlyoutBaseStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlacementProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttachedFlyoutProperty<Impl: IFlyoutBaseStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AttachedFlyoutProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttachedFlyout<Impl: IFlyoutBaseStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAttachedFlyout(&*(&element as *const <super::super::FrameworkElement as ::windows::core::Abi>::Abi as *const <super::super::FrameworkElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttachedFlyout<Impl: IFlyoutBaseStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAttachedFlyout(&*(&element as *const <super::super::FrameworkElement as ::windows::core::Abi>::Abi as *const <super::super::FrameworkElement as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <FlyoutBase as ::windows::core::Abi>::Abi as *const <FlyoutBase as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ShowAttachedFlyout<Impl: IFlyoutBaseStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flyoutowner: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowAttachedFlyout(&*(&flyoutowner as *const <super::super::FrameworkElement as ::windows::core::Abi>::Abi as *const <super::super::FrameworkElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFlyoutBaseStatics, BASE_OFFSET>(),
            PlacementProperty: PlacementProperty::<Impl, IMPL_OFFSET>,
            AttachedFlyoutProperty: AttachedFlyoutProperty::<Impl, IMPL_OFFSET>,
            GetAttachedFlyout: GetAttachedFlyout::<Impl, IMPL_OFFSET>,
            SetAttachedFlyout: SetAttachedFlyout::<Impl, IMPL_OFFSET>,
            ShowAttachedFlyout: ShowAttachedFlyout::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFlyoutBaseStatics as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlyoutBaseStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlyoutBaseStatics2Vtbl {
        unsafe extern "system" fn AllowFocusOnInteractionProperty<Impl: IFlyoutBaseStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowFocusOnInteractionProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LightDismissOverlayModeProperty<Impl: IFlyoutBaseStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LightDismissOverlayModeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowFocusWhenDisabledProperty<Impl: IFlyoutBaseStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowFocusWhenDisabledProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ElementSoundModeProperty<Impl: IFlyoutBaseStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ElementSoundModeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFlyoutBaseStatics2, BASE_OFFSET>(),
            AllowFocusOnInteractionProperty: AllowFocusOnInteractionProperty::<Impl, IMPL_OFFSET>,
            LightDismissOverlayModeProperty: LightDismissOverlayModeProperty::<Impl, IMPL_OFFSET>,
            AllowFocusWhenDisabledProperty: AllowFocusWhenDisabledProperty::<Impl, IMPL_OFFSET>,
            ElementSoundModeProperty: ElementSoundModeProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFlyoutBaseStatics2 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlyoutBaseStatics3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlyoutBaseStatics3Vtbl {
        unsafe extern "system" fn OverlayInputPassThroughElementProperty<Impl: IFlyoutBaseStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OverlayInputPassThroughElementProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFlyoutBaseStatics3, BASE_OFFSET>(),
            OverlayInputPassThroughElementProperty: OverlayInputPassThroughElementProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFlyoutBaseStatics3 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlyoutBaseStatics5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlyoutBaseStatics5Vtbl {
        unsafe extern "system" fn TargetProperty<Impl: IFlyoutBaseStatics5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ShowModeProperty<Impl: IFlyoutBaseStatics5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowModeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InputDevicePrefersPrimaryCommandsProperty<Impl: IFlyoutBaseStatics5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InputDevicePrefersPrimaryCommandsProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AreOpenCloseAnimationsEnabledProperty<Impl: IFlyoutBaseStatics5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AreOpenCloseAnimationsEnabledProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOpenProperty<Impl: IFlyoutBaseStatics5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOpenProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFlyoutBaseStatics5, BASE_OFFSET>(),
            TargetProperty: TargetProperty::<Impl, IMPL_OFFSET>,
            ShowModeProperty: ShowModeProperty::<Impl, IMPL_OFFSET>,
            InputDevicePrefersPrimaryCommandsProperty: InputDevicePrefersPrimaryCommandsProperty::<Impl, IMPL_OFFSET>,
            AreOpenCloseAnimationsEnabledProperty: AreOpenCloseAnimationsEnabledProperty::<Impl, IMPL_OFFSET>,
            IsOpenProperty: IsOpenProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFlyoutBaseStatics5 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlyoutBaseStatics6Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlyoutBaseStatics6Vtbl {
        unsafe extern "system" fn ShouldConstrainToRootBoundsProperty<Impl: IFlyoutBaseStatics6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShouldConstrainToRootBoundsProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFlyoutBaseStatics6, BASE_OFFSET>(),
            ShouldConstrainToRootBoundsProperty: ShouldConstrainToRootBoundsProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFlyoutBaseStatics6 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFlyoutShowOptions {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IFlyoutShowOptions";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IFlyoutShowOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlyoutShowOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlyoutShowOptionsVtbl {
        unsafe extern "system" fn Position<Impl: IFlyoutShowOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPosition<Impl: IFlyoutShowOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPosition(&*(&value as *const <super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ExclusionRect<Impl: IFlyoutShowOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExclusionRect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExclusionRect<Impl: IFlyoutShowOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExclusionRect(&*(&value as *const <super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Rect> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Rect> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ShowMode<Impl: IFlyoutShowOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FlyoutShowMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShowMode<Impl: IFlyoutShowOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FlyoutShowMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShowMode(value).into()
        }
        unsafe extern "system" fn Placement<Impl: IFlyoutShowOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FlyoutPlacementMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Placement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlacement<Impl: IFlyoutShowOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FlyoutPlacementMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPlacement(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFlyoutShowOptions, BASE_OFFSET>(),
            Position: Position::<Impl, IMPL_OFFSET>,
            SetPosition: SetPosition::<Impl, IMPL_OFFSET>,
            ExclusionRect: ExclusionRect::<Impl, IMPL_OFFSET>,
            SetExclusionRect: SetExclusionRect::<Impl, IMPL_OFFSET>,
            ShowMode: ShowMode::<Impl, IMPL_OFFSET>,
            SetShowMode: SetShowMode::<Impl, IMPL_OFFSET>,
            Placement: Placement::<Impl, IMPL_OFFSET>,
            SetPlacement: SetPlacement::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFlyoutShowOptions as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlyoutShowOptionsFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlyoutShowOptionsFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IFlyoutShowOptionsFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFlyoutShowOptionsFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFlyoutShowOptionsFactory as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeneratorPositionHelperImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGeneratorPositionHelperVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IGeneratorPositionHelper, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeneratorPositionHelper as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeneratorPositionHelperStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGeneratorPositionHelperStaticsVtbl {
        unsafe extern "system" fn FromIndexAndOffset<Impl: IGeneratorPositionHelperStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, offset: i32, result__: *mut GeneratorPosition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromIndexAndOffset(index, offset) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGeneratorPositionHelperStatics, BASE_OFFSET>(),
            FromIndexAndOffset: FromIndexAndOffset::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeneratorPositionHelperStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGridViewItemPresenter {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IGridViewItemPresenter";
}
#[cfg(all(feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
impl IGridViewItemPresenterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridViewItemPresenterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGridViewItemPresenterVtbl {
        unsafe extern "system" fn SelectionCheckMarkVisualEnabled<Impl: IGridViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectionCheckMarkVisualEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectionCheckMarkVisualEnabled<Impl: IGridViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectionCheckMarkVisualEnabled(value).into()
        }
        unsafe extern "system" fn CheckHintBrush<Impl: IGridViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckHintBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCheckHintBrush<Impl: IGridViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCheckHintBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckSelectingBrush<Impl: IGridViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckSelectingBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCheckSelectingBrush<Impl: IGridViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCheckSelectingBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckBrush<Impl: IGridViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCheckBrush<Impl: IGridViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCheckBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DragBackground<Impl: IGridViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DragBackground() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDragBackground<Impl: IGridViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDragBackground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DragForeground<Impl: IGridViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DragForeground() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDragForeground<Impl: IGridViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDragForeground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FocusBorderBrush<Impl: IGridViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FocusBorderBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFocusBorderBrush<Impl: IGridViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFocusBorderBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PlaceholderBackground<Impl: IGridViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlaceholderBackground() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlaceholderBackground<Impl: IGridViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPlaceholderBackground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerOverBackground<Impl: IGridViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerOverBackground() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPointerOverBackground<Impl: IGridViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPointerOverBackground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedBackground<Impl: IGridViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedBackground() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedBackground<Impl: IGridViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectedBackground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedForeground<Impl: IGridViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedForeground() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedForeground<Impl: IGridViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectedForeground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedPointerOverBackground<Impl: IGridViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedPointerOverBackground() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedPointerOverBackground<Impl: IGridViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectedPointerOverBackground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedPointerOverBorderBrush<Impl: IGridViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedPointerOverBorderBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedPointerOverBorderBrush<Impl: IGridViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectedPointerOverBorderBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedBorderThickness<Impl: IGridViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedBorderThickness() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedBorderThickness<Impl: IGridViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectedBorderThickness(&*(&value as *const <super::super::Thickness as ::windows::core::Abi>::Abi as *const <super::super::Thickness as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DisabledOpacity<Impl: IGridViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisabledOpacity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisabledOpacity<Impl: IGridViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisabledOpacity(value).into()
        }
        unsafe extern "system" fn DragOpacity<Impl: IGridViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DragOpacity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDragOpacity<Impl: IGridViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDragOpacity(value).into()
        }
        unsafe extern "system" fn ReorderHintOffset<Impl: IGridViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReorderHintOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReorderHintOffset<Impl: IGridViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReorderHintOffset(value).into()
        }
        unsafe extern "system" fn GridViewItemPresenterHorizontalContentAlignment<Impl: IGridViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::HorizontalAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GridViewItemPresenterHorizontalContentAlignment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGridViewItemPresenterHorizontalContentAlignment<Impl: IGridViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::HorizontalAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGridViewItemPresenterHorizontalContentAlignment(value).into()
        }
        unsafe extern "system" fn GridViewItemPresenterVerticalContentAlignment<Impl: IGridViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::VerticalAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GridViewItemPresenterVerticalContentAlignment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGridViewItemPresenterVerticalContentAlignment<Impl: IGridViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::VerticalAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGridViewItemPresenterVerticalContentAlignment(value).into()
        }
        unsafe extern "system" fn GridViewItemPresenterPadding<Impl: IGridViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GridViewItemPresenterPadding() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGridViewItemPresenterPadding<Impl: IGridViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGridViewItemPresenterPadding(&*(&value as *const <super::super::Thickness as ::windows::core::Abi>::Abi as *const <super::super::Thickness as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerOverBackgroundMargin<Impl: IGridViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerOverBackgroundMargin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPointerOverBackgroundMargin<Impl: IGridViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPointerOverBackgroundMargin(&*(&value as *const <super::super::Thickness as ::windows::core::Abi>::Abi as *const <super::super::Thickness as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContentMargin<Impl: IGridViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentMargin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentMargin<Impl: IGridViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentMargin(&*(&value as *const <super::super::Thickness as ::windows::core::Abi>::Abi as *const <super::super::Thickness as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGridViewItemPresenter, BASE_OFFSET>(),
            SelectionCheckMarkVisualEnabled: SelectionCheckMarkVisualEnabled::<Impl, IMPL_OFFSET>,
            SetSelectionCheckMarkVisualEnabled: SetSelectionCheckMarkVisualEnabled::<Impl, IMPL_OFFSET>,
            CheckHintBrush: CheckHintBrush::<Impl, IMPL_OFFSET>,
            SetCheckHintBrush: SetCheckHintBrush::<Impl, IMPL_OFFSET>,
            CheckSelectingBrush: CheckSelectingBrush::<Impl, IMPL_OFFSET>,
            SetCheckSelectingBrush: SetCheckSelectingBrush::<Impl, IMPL_OFFSET>,
            CheckBrush: CheckBrush::<Impl, IMPL_OFFSET>,
            SetCheckBrush: SetCheckBrush::<Impl, IMPL_OFFSET>,
            DragBackground: DragBackground::<Impl, IMPL_OFFSET>,
            SetDragBackground: SetDragBackground::<Impl, IMPL_OFFSET>,
            DragForeground: DragForeground::<Impl, IMPL_OFFSET>,
            SetDragForeground: SetDragForeground::<Impl, IMPL_OFFSET>,
            FocusBorderBrush: FocusBorderBrush::<Impl, IMPL_OFFSET>,
            SetFocusBorderBrush: SetFocusBorderBrush::<Impl, IMPL_OFFSET>,
            PlaceholderBackground: PlaceholderBackground::<Impl, IMPL_OFFSET>,
            SetPlaceholderBackground: SetPlaceholderBackground::<Impl, IMPL_OFFSET>,
            PointerOverBackground: PointerOverBackground::<Impl, IMPL_OFFSET>,
            SetPointerOverBackground: SetPointerOverBackground::<Impl, IMPL_OFFSET>,
            SelectedBackground: SelectedBackground::<Impl, IMPL_OFFSET>,
            SetSelectedBackground: SetSelectedBackground::<Impl, IMPL_OFFSET>,
            SelectedForeground: SelectedForeground::<Impl, IMPL_OFFSET>,
            SetSelectedForeground: SetSelectedForeground::<Impl, IMPL_OFFSET>,
            SelectedPointerOverBackground: SelectedPointerOverBackground::<Impl, IMPL_OFFSET>,
            SetSelectedPointerOverBackground: SetSelectedPointerOverBackground::<Impl, IMPL_OFFSET>,
            SelectedPointerOverBorderBrush: SelectedPointerOverBorderBrush::<Impl, IMPL_OFFSET>,
            SetSelectedPointerOverBorderBrush: SetSelectedPointerOverBorderBrush::<Impl, IMPL_OFFSET>,
            SelectedBorderThickness: SelectedBorderThickness::<Impl, IMPL_OFFSET>,
            SetSelectedBorderThickness: SetSelectedBorderThickness::<Impl, IMPL_OFFSET>,
            DisabledOpacity: DisabledOpacity::<Impl, IMPL_OFFSET>,
            SetDisabledOpacity: SetDisabledOpacity::<Impl, IMPL_OFFSET>,
            DragOpacity: DragOpacity::<Impl, IMPL_OFFSET>,
            SetDragOpacity: SetDragOpacity::<Impl, IMPL_OFFSET>,
            ReorderHintOffset: ReorderHintOffset::<Impl, IMPL_OFFSET>,
            SetReorderHintOffset: SetReorderHintOffset::<Impl, IMPL_OFFSET>,
            GridViewItemPresenterHorizontalContentAlignment: GridViewItemPresenterHorizontalContentAlignment::<Impl, IMPL_OFFSET>,
            SetGridViewItemPresenterHorizontalContentAlignment: SetGridViewItemPresenterHorizontalContentAlignment::<Impl, IMPL_OFFSET>,
            GridViewItemPresenterVerticalContentAlignment: GridViewItemPresenterVerticalContentAlignment::<Impl, IMPL_OFFSET>,
            SetGridViewItemPresenterVerticalContentAlignment: SetGridViewItemPresenterVerticalContentAlignment::<Impl, IMPL_OFFSET>,
            GridViewItemPresenterPadding: GridViewItemPresenterPadding::<Impl, IMPL_OFFSET>,
            SetGridViewItemPresenterPadding: SetGridViewItemPresenterPadding::<Impl, IMPL_OFFSET>,
            PointerOverBackgroundMargin: PointerOverBackgroundMargin::<Impl, IMPL_OFFSET>,
            SetPointerOverBackgroundMargin: SetPointerOverBackgroundMargin::<Impl, IMPL_OFFSET>,
            ContentMargin: ContentMargin::<Impl, IMPL_OFFSET>,
            SetContentMargin: SetContentMargin::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGridViewItemPresenter as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridViewItemPresenterFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGridViewItemPresenterFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IGridViewItemPresenterFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGridViewItemPresenterFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGridViewItemPresenterFactory as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridViewItemPresenterStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGridViewItemPresenterStaticsVtbl {
        unsafe extern "system" fn SelectionCheckMarkVisualEnabledProperty<Impl: IGridViewItemPresenterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectionCheckMarkVisualEnabledProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckHintBrushProperty<Impl: IGridViewItemPresenterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckHintBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckSelectingBrushProperty<Impl: IGridViewItemPresenterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckSelectingBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckBrushProperty<Impl: IGridViewItemPresenterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DragBackgroundProperty<Impl: IGridViewItemPresenterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DragBackgroundProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DragForegroundProperty<Impl: IGridViewItemPresenterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DragForegroundProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FocusBorderBrushProperty<Impl: IGridViewItemPresenterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FocusBorderBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PlaceholderBackgroundProperty<Impl: IGridViewItemPresenterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlaceholderBackgroundProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PointerOverBackgroundProperty<Impl: IGridViewItemPresenterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerOverBackgroundProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedBackgroundProperty<Impl: IGridViewItemPresenterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedBackgroundProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedForegroundProperty<Impl: IGridViewItemPresenterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedForegroundProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedPointerOverBackgroundProperty<Impl: IGridViewItemPresenterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedPointerOverBackgroundProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedPointerOverBorderBrushProperty<Impl: IGridViewItemPresenterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedPointerOverBorderBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedBorderThicknessProperty<Impl: IGridViewItemPresenterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedBorderThicknessProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisabledOpacityProperty<Impl: IGridViewItemPresenterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisabledOpacityProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DragOpacityProperty<Impl: IGridViewItemPresenterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DragOpacityProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReorderHintOffsetProperty<Impl: IGridViewItemPresenterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReorderHintOffsetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GridViewItemPresenterHorizontalContentAlignmentProperty<Impl: IGridViewItemPresenterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GridViewItemPresenterHorizontalContentAlignmentProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GridViewItemPresenterVerticalContentAlignmentProperty<Impl: IGridViewItemPresenterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GridViewItemPresenterVerticalContentAlignmentProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GridViewItemPresenterPaddingProperty<Impl: IGridViewItemPresenterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GridViewItemPresenterPaddingProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PointerOverBackgroundMarginProperty<Impl: IGridViewItemPresenterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerOverBackgroundMarginProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentMarginProperty<Impl: IGridViewItemPresenterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentMarginProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGridViewItemPresenterStatics, BASE_OFFSET>(),
            SelectionCheckMarkVisualEnabledProperty: SelectionCheckMarkVisualEnabledProperty::<Impl, IMPL_OFFSET>,
            CheckHintBrushProperty: CheckHintBrushProperty::<Impl, IMPL_OFFSET>,
            CheckSelectingBrushProperty: CheckSelectingBrushProperty::<Impl, IMPL_OFFSET>,
            CheckBrushProperty: CheckBrushProperty::<Impl, IMPL_OFFSET>,
            DragBackgroundProperty: DragBackgroundProperty::<Impl, IMPL_OFFSET>,
            DragForegroundProperty: DragForegroundProperty::<Impl, IMPL_OFFSET>,
            FocusBorderBrushProperty: FocusBorderBrushProperty::<Impl, IMPL_OFFSET>,
            PlaceholderBackgroundProperty: PlaceholderBackgroundProperty::<Impl, IMPL_OFFSET>,
            PointerOverBackgroundProperty: PointerOverBackgroundProperty::<Impl, IMPL_OFFSET>,
            SelectedBackgroundProperty: SelectedBackgroundProperty::<Impl, IMPL_OFFSET>,
            SelectedForegroundProperty: SelectedForegroundProperty::<Impl, IMPL_OFFSET>,
            SelectedPointerOverBackgroundProperty: SelectedPointerOverBackgroundProperty::<Impl, IMPL_OFFSET>,
            SelectedPointerOverBorderBrushProperty: SelectedPointerOverBorderBrushProperty::<Impl, IMPL_OFFSET>,
            SelectedBorderThicknessProperty: SelectedBorderThicknessProperty::<Impl, IMPL_OFFSET>,
            DisabledOpacityProperty: DisabledOpacityProperty::<Impl, IMPL_OFFSET>,
            DragOpacityProperty: DragOpacityProperty::<Impl, IMPL_OFFSET>,
            ReorderHintOffsetProperty: ReorderHintOffsetProperty::<Impl, IMPL_OFFSET>,
            GridViewItemPresenterHorizontalContentAlignmentProperty: GridViewItemPresenterHorizontalContentAlignmentProperty::<Impl, IMPL_OFFSET>,
            GridViewItemPresenterVerticalContentAlignmentProperty: GridViewItemPresenterVerticalContentAlignmentProperty::<Impl, IMPL_OFFSET>,
            GridViewItemPresenterPaddingProperty: GridViewItemPresenterPaddingProperty::<Impl, IMPL_OFFSET>,
            PointerOverBackgroundMarginProperty: PointerOverBackgroundMarginProperty::<Impl, IMPL_OFFSET>,
            ContentMarginProperty: ContentMarginProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGridViewItemPresenterStatics as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridViewItemTemplateSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGridViewItemTemplateSettingsVtbl {
        unsafe extern "system" fn DragItemsCount<Impl: IGridViewItemTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DragItemsCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGridViewItemTemplateSettings, BASE_OFFSET>(),
            DragItemsCount: DragItemsCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGridViewItemTemplateSettings as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IItemsChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IItemsChangedEventArgsVtbl {
        unsafe extern "system" fn Action<Impl: IItemsChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Action() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Impl: IItemsChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GeneratorPosition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Position() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OldPosition<Impl: IItemsChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GeneratorPosition) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OldPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemCount<Impl: IItemsChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemUICount<Impl: IItemsChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemUICount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IItemsChangedEventArgs, BASE_OFFSET>(),
            Action: Action::<Impl, IMPL_OFFSET>,
            Position: Position::<Impl, IMPL_OFFSET>,
            OldPosition: OldPosition::<Impl, IMPL_OFFSET>,
            ItemCount: ItemCount::<Impl, IMPL_OFFSET>,
            ItemUICount: ItemUICount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IItemsChangedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
pub trait IJumpListItemBackgroundConverterImpl: Sized {
    fn Enabled(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetEnabled(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn Disabled(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetDisabled(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IJumpListItemBackgroundConverter {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IJumpListItemBackgroundConverter";
}
#[cfg(all(feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
impl IJumpListItemBackgroundConverterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IJumpListItemBackgroundConverterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IJumpListItemBackgroundConverterVtbl {
        unsafe extern "system" fn Enabled<Impl: IJumpListItemBackgroundConverterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Impl: IJumpListItemBackgroundConverterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnabled(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Disabled<Impl: IJumpListItemBackgroundConverterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Disabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisabled<Impl: IJumpListItemBackgroundConverterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisabled(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IJumpListItemBackgroundConverter, BASE_OFFSET>(),
            Enabled: Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled: SetEnabled::<Impl, IMPL_OFFSET>,
            Disabled: Disabled::<Impl, IMPL_OFFSET>,
            SetDisabled: SetDisabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IJumpListItemBackgroundConverter as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IJumpListItemBackgroundConverterStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IJumpListItemBackgroundConverterStaticsVtbl {
        unsafe extern "system" fn EnabledProperty<Impl: IJumpListItemBackgroundConverterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnabledProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisabledProperty<Impl: IJumpListItemBackgroundConverterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisabledProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IJumpListItemBackgroundConverterStatics, BASE_OFFSET>(),
            EnabledProperty: EnabledProperty::<Impl, IMPL_OFFSET>,
            DisabledProperty: DisabledProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IJumpListItemBackgroundConverterStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
pub trait IJumpListItemForegroundConverterImpl: Sized {
    fn Enabled(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetEnabled(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn Disabled(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetDisabled(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IJumpListItemForegroundConverter {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IJumpListItemForegroundConverter";
}
#[cfg(all(feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
impl IJumpListItemForegroundConverterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IJumpListItemForegroundConverterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IJumpListItemForegroundConverterVtbl {
        unsafe extern "system" fn Enabled<Impl: IJumpListItemForegroundConverterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Impl: IJumpListItemForegroundConverterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnabled(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Disabled<Impl: IJumpListItemForegroundConverterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Disabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisabled<Impl: IJumpListItemForegroundConverterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisabled(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IJumpListItemForegroundConverter, BASE_OFFSET>(),
            Enabled: Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled: SetEnabled::<Impl, IMPL_OFFSET>,
            Disabled: Disabled::<Impl, IMPL_OFFSET>,
            SetDisabled: SetDisabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IJumpListItemForegroundConverter as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IJumpListItemForegroundConverterStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IJumpListItemForegroundConverterStaticsVtbl {
        unsafe extern "system" fn EnabledProperty<Impl: IJumpListItemForegroundConverterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnabledProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisabledProperty<Impl: IJumpListItemForegroundConverterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisabledProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IJumpListItemForegroundConverterStatics, BASE_OFFSET>(),
            EnabledProperty: EnabledProperty::<Impl, IMPL_OFFSET>,
            DisabledProperty: DisabledProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IJumpListItemForegroundConverterStatics as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILayoutInformationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILayoutInformationVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ILayoutInformation, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILayoutInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ILayoutInformationStaticsImpl: Sized {
    fn GetLayoutExceptionElement(&self, dispatcher: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<super::super::UIElement>;
    fn GetLayoutSlot(&self, element: &::core::option::Option<super::super::FrameworkElement>) -> ::windows::core::Result<super::super::super::super::Foundation::Rect>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILayoutInformationStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ILayoutInformationStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ILayoutInformationStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILayoutInformationStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILayoutInformationStaticsVtbl {
        unsafe extern "system" fn GetLayoutExceptionElement<Impl: ILayoutInformationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispatcher: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLayoutExceptionElement(&*(&dispatcher as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLayoutSlot<Impl: ILayoutInformationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLayoutSlot(&*(&element as *const <super::super::FrameworkElement as ::windows::core::Abi>::Abi as *const <super::super::FrameworkElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILayoutInformationStatics, BASE_OFFSET>(),
            GetLayoutExceptionElement: GetLayoutExceptionElement::<Impl, IMPL_OFFSET>,
            GetLayoutSlot: GetLayoutSlot::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILayoutInformationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ILayoutInformationStatics2Impl: Sized {
    fn GetAvailableSize(&self, element: &::core::option::Option<super::super::UIElement>) -> ::windows::core::Result<super::super::super::super::Foundation::Size>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILayoutInformationStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ILayoutInformationStatics2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ILayoutInformationStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILayoutInformationStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILayoutInformationStatics2Vtbl {
        unsafe extern "system" fn GetAvailableSize<Impl: ILayoutInformationStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAvailableSize(&*(&element as *const <super::super::UIElement as ::windows::core::Abi>::Abi as *const <super::super::UIElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILayoutInformationStatics2, BASE_OFFSET>(),
            GetAvailableSize: GetAvailableSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILayoutInformationStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IListViewItemPresenter {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IListViewItemPresenter";
}
#[cfg(all(feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
impl IListViewItemPresenterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewItemPresenterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListViewItemPresenterVtbl {
        unsafe extern "system" fn SelectionCheckMarkVisualEnabled<Impl: IListViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectionCheckMarkVisualEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectionCheckMarkVisualEnabled<Impl: IListViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectionCheckMarkVisualEnabled(value).into()
        }
        unsafe extern "system" fn CheckHintBrush<Impl: IListViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckHintBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCheckHintBrush<Impl: IListViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCheckHintBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckSelectingBrush<Impl: IListViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckSelectingBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCheckSelectingBrush<Impl: IListViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCheckSelectingBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckBrush<Impl: IListViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCheckBrush<Impl: IListViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCheckBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DragBackground<Impl: IListViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DragBackground() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDragBackground<Impl: IListViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDragBackground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DragForeground<Impl: IListViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DragForeground() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDragForeground<Impl: IListViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDragForeground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FocusBorderBrush<Impl: IListViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FocusBorderBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFocusBorderBrush<Impl: IListViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFocusBorderBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PlaceholderBackground<Impl: IListViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlaceholderBackground() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlaceholderBackground<Impl: IListViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPlaceholderBackground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerOverBackground<Impl: IListViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerOverBackground() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPointerOverBackground<Impl: IListViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPointerOverBackground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedBackground<Impl: IListViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedBackground() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedBackground<Impl: IListViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectedBackground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedForeground<Impl: IListViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedForeground() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedForeground<Impl: IListViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectedForeground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedPointerOverBackground<Impl: IListViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedPointerOverBackground() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedPointerOverBackground<Impl: IListViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectedPointerOverBackground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedPointerOverBorderBrush<Impl: IListViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedPointerOverBorderBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedPointerOverBorderBrush<Impl: IListViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectedPointerOverBorderBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedBorderThickness<Impl: IListViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedBorderThickness() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedBorderThickness<Impl: IListViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectedBorderThickness(&*(&value as *const <super::super::Thickness as ::windows::core::Abi>::Abi as *const <super::super::Thickness as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DisabledOpacity<Impl: IListViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisabledOpacity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisabledOpacity<Impl: IListViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisabledOpacity(value).into()
        }
        unsafe extern "system" fn DragOpacity<Impl: IListViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DragOpacity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDragOpacity<Impl: IListViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDragOpacity(value).into()
        }
        unsafe extern "system" fn ReorderHintOffset<Impl: IListViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReorderHintOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReorderHintOffset<Impl: IListViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReorderHintOffset(value).into()
        }
        unsafe extern "system" fn ListViewItemPresenterHorizontalContentAlignment<Impl: IListViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::HorizontalAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ListViewItemPresenterHorizontalContentAlignment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListViewItemPresenterHorizontalContentAlignment<Impl: IListViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::HorizontalAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetListViewItemPresenterHorizontalContentAlignment(value).into()
        }
        unsafe extern "system" fn ListViewItemPresenterVerticalContentAlignment<Impl: IListViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::VerticalAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ListViewItemPresenterVerticalContentAlignment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListViewItemPresenterVerticalContentAlignment<Impl: IListViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::VerticalAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetListViewItemPresenterVerticalContentAlignment(value).into()
        }
        unsafe extern "system" fn ListViewItemPresenterPadding<Impl: IListViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ListViewItemPresenterPadding() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetListViewItemPresenterPadding<Impl: IListViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetListViewItemPresenterPadding(&*(&value as *const <super::super::Thickness as ::windows::core::Abi>::Abi as *const <super::super::Thickness as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerOverBackgroundMargin<Impl: IListViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerOverBackgroundMargin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPointerOverBackgroundMargin<Impl: IListViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPointerOverBackgroundMargin(&*(&value as *const <super::super::Thickness as ::windows::core::Abi>::Abi as *const <super::super::Thickness as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContentMargin<Impl: IListViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentMargin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentMargin<Impl: IListViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentMargin(&*(&value as *const <super::super::Thickness as ::windows::core::Abi>::Abi as *const <super::super::Thickness as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IListViewItemPresenter, BASE_OFFSET>(),
            SelectionCheckMarkVisualEnabled: SelectionCheckMarkVisualEnabled::<Impl, IMPL_OFFSET>,
            SetSelectionCheckMarkVisualEnabled: SetSelectionCheckMarkVisualEnabled::<Impl, IMPL_OFFSET>,
            CheckHintBrush: CheckHintBrush::<Impl, IMPL_OFFSET>,
            SetCheckHintBrush: SetCheckHintBrush::<Impl, IMPL_OFFSET>,
            CheckSelectingBrush: CheckSelectingBrush::<Impl, IMPL_OFFSET>,
            SetCheckSelectingBrush: SetCheckSelectingBrush::<Impl, IMPL_OFFSET>,
            CheckBrush: CheckBrush::<Impl, IMPL_OFFSET>,
            SetCheckBrush: SetCheckBrush::<Impl, IMPL_OFFSET>,
            DragBackground: DragBackground::<Impl, IMPL_OFFSET>,
            SetDragBackground: SetDragBackground::<Impl, IMPL_OFFSET>,
            DragForeground: DragForeground::<Impl, IMPL_OFFSET>,
            SetDragForeground: SetDragForeground::<Impl, IMPL_OFFSET>,
            FocusBorderBrush: FocusBorderBrush::<Impl, IMPL_OFFSET>,
            SetFocusBorderBrush: SetFocusBorderBrush::<Impl, IMPL_OFFSET>,
            PlaceholderBackground: PlaceholderBackground::<Impl, IMPL_OFFSET>,
            SetPlaceholderBackground: SetPlaceholderBackground::<Impl, IMPL_OFFSET>,
            PointerOverBackground: PointerOverBackground::<Impl, IMPL_OFFSET>,
            SetPointerOverBackground: SetPointerOverBackground::<Impl, IMPL_OFFSET>,
            SelectedBackground: SelectedBackground::<Impl, IMPL_OFFSET>,
            SetSelectedBackground: SetSelectedBackground::<Impl, IMPL_OFFSET>,
            SelectedForeground: SelectedForeground::<Impl, IMPL_OFFSET>,
            SetSelectedForeground: SetSelectedForeground::<Impl, IMPL_OFFSET>,
            SelectedPointerOverBackground: SelectedPointerOverBackground::<Impl, IMPL_OFFSET>,
            SetSelectedPointerOverBackground: SetSelectedPointerOverBackground::<Impl, IMPL_OFFSET>,
            SelectedPointerOverBorderBrush: SelectedPointerOverBorderBrush::<Impl, IMPL_OFFSET>,
            SetSelectedPointerOverBorderBrush: SetSelectedPointerOverBorderBrush::<Impl, IMPL_OFFSET>,
            SelectedBorderThickness: SelectedBorderThickness::<Impl, IMPL_OFFSET>,
            SetSelectedBorderThickness: SetSelectedBorderThickness::<Impl, IMPL_OFFSET>,
            DisabledOpacity: DisabledOpacity::<Impl, IMPL_OFFSET>,
            SetDisabledOpacity: SetDisabledOpacity::<Impl, IMPL_OFFSET>,
            DragOpacity: DragOpacity::<Impl, IMPL_OFFSET>,
            SetDragOpacity: SetDragOpacity::<Impl, IMPL_OFFSET>,
            ReorderHintOffset: ReorderHintOffset::<Impl, IMPL_OFFSET>,
            SetReorderHintOffset: SetReorderHintOffset::<Impl, IMPL_OFFSET>,
            ListViewItemPresenterHorizontalContentAlignment: ListViewItemPresenterHorizontalContentAlignment::<Impl, IMPL_OFFSET>,
            SetListViewItemPresenterHorizontalContentAlignment: SetListViewItemPresenterHorizontalContentAlignment::<Impl, IMPL_OFFSET>,
            ListViewItemPresenterVerticalContentAlignment: ListViewItemPresenterVerticalContentAlignment::<Impl, IMPL_OFFSET>,
            SetListViewItemPresenterVerticalContentAlignment: SetListViewItemPresenterVerticalContentAlignment::<Impl, IMPL_OFFSET>,
            ListViewItemPresenterPadding: ListViewItemPresenterPadding::<Impl, IMPL_OFFSET>,
            SetListViewItemPresenterPadding: SetListViewItemPresenterPadding::<Impl, IMPL_OFFSET>,
            PointerOverBackgroundMargin: PointerOverBackgroundMargin::<Impl, IMPL_OFFSET>,
            SetPointerOverBackgroundMargin: SetPointerOverBackgroundMargin::<Impl, IMPL_OFFSET>,
            ContentMargin: ContentMargin::<Impl, IMPL_OFFSET>,
            SetContentMargin: SetContentMargin::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IListViewItemPresenter as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IListViewItemPresenter2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IListViewItemPresenter2";
}
#[cfg(all(feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
impl IListViewItemPresenter2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewItemPresenter2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListViewItemPresenter2Vtbl {
        unsafe extern "system" fn SelectedPressedBackground<Impl: IListViewItemPresenter2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedPressedBackground() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedPressedBackground<Impl: IListViewItemPresenter2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectedPressedBackground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PressedBackground<Impl: IListViewItemPresenter2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PressedBackground() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPressedBackground<Impl: IListViewItemPresenter2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPressedBackground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckBoxBrush<Impl: IListViewItemPresenter2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckBoxBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCheckBoxBrush<Impl: IListViewItemPresenter2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCheckBoxBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FocusSecondaryBorderBrush<Impl: IListViewItemPresenter2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FocusSecondaryBorderBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFocusSecondaryBorderBrush<Impl: IListViewItemPresenter2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFocusSecondaryBorderBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckMode<Impl: IListViewItemPresenter2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ListViewItemPresenterCheckMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCheckMode<Impl: IListViewItemPresenter2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ListViewItemPresenterCheckMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCheckMode(value).into()
        }
        unsafe extern "system" fn PointerOverForeground<Impl: IListViewItemPresenter2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerOverForeground() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPointerOverForeground<Impl: IListViewItemPresenter2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPointerOverForeground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IListViewItemPresenter2, BASE_OFFSET>(),
            SelectedPressedBackground: SelectedPressedBackground::<Impl, IMPL_OFFSET>,
            SetSelectedPressedBackground: SetSelectedPressedBackground::<Impl, IMPL_OFFSET>,
            PressedBackground: PressedBackground::<Impl, IMPL_OFFSET>,
            SetPressedBackground: SetPressedBackground::<Impl, IMPL_OFFSET>,
            CheckBoxBrush: CheckBoxBrush::<Impl, IMPL_OFFSET>,
            SetCheckBoxBrush: SetCheckBoxBrush::<Impl, IMPL_OFFSET>,
            FocusSecondaryBorderBrush: FocusSecondaryBorderBrush::<Impl, IMPL_OFFSET>,
            SetFocusSecondaryBorderBrush: SetFocusSecondaryBorderBrush::<Impl, IMPL_OFFSET>,
            CheckMode: CheckMode::<Impl, IMPL_OFFSET>,
            SetCheckMode: SetCheckMode::<Impl, IMPL_OFFSET>,
            PointerOverForeground: PointerOverForeground::<Impl, IMPL_OFFSET>,
            SetPointerOverForeground: SetPointerOverForeground::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IListViewItemPresenter2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IListViewItemPresenter3 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IListViewItemPresenter3";
}
#[cfg(all(feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
impl IListViewItemPresenter3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewItemPresenter3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListViewItemPresenter3Vtbl {
        unsafe extern "system" fn RevealBackground<Impl: IListViewItemPresenter3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RevealBackground() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRevealBackground<Impl: IListViewItemPresenter3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRevealBackground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RevealBorderBrush<Impl: IListViewItemPresenter3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RevealBorderBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRevealBorderBrush<Impl: IListViewItemPresenter3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRevealBorderBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RevealBorderThickness<Impl: IListViewItemPresenter3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RevealBorderThickness() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRevealBorderThickness<Impl: IListViewItemPresenter3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRevealBorderThickness(&*(&value as *const <super::super::Thickness as ::windows::core::Abi>::Abi as *const <super::super::Thickness as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RevealBackgroundShowsAboveContent<Impl: IListViewItemPresenter3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RevealBackgroundShowsAboveContent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRevealBackgroundShowsAboveContent<Impl: IListViewItemPresenter3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRevealBackgroundShowsAboveContent(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IListViewItemPresenter3, BASE_OFFSET>(),
            RevealBackground: RevealBackground::<Impl, IMPL_OFFSET>,
            SetRevealBackground: SetRevealBackground::<Impl, IMPL_OFFSET>,
            RevealBorderBrush: RevealBorderBrush::<Impl, IMPL_OFFSET>,
            SetRevealBorderBrush: SetRevealBorderBrush::<Impl, IMPL_OFFSET>,
            RevealBorderThickness: RevealBorderThickness::<Impl, IMPL_OFFSET>,
            SetRevealBorderThickness: SetRevealBorderThickness::<Impl, IMPL_OFFSET>,
            RevealBackgroundShowsAboveContent: RevealBackgroundShowsAboveContent::<Impl, IMPL_OFFSET>,
            SetRevealBackgroundShowsAboveContent: SetRevealBackgroundShowsAboveContent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IListViewItemPresenter3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IListViewItemPresenter4 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IListViewItemPresenter4";
}
#[cfg(all(feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
impl IListViewItemPresenter4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewItemPresenter4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListViewItemPresenter4Vtbl {
        unsafe extern "system" fn SelectedDisabledBackground<Impl: IListViewItemPresenter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedDisabledBackground() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedDisabledBackground<Impl: IListViewItemPresenter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectedDisabledBackground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckPressedBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckPressedBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCheckPressedBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCheckPressedBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckDisabledBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckDisabledBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCheckDisabledBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCheckDisabledBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckBoxPointerOverBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckBoxPointerOverBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCheckBoxPointerOverBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCheckBoxPointerOverBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckBoxPressedBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckBoxPressedBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCheckBoxPressedBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCheckBoxPressedBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckBoxDisabledBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckBoxDisabledBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCheckBoxDisabledBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCheckBoxDisabledBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckBoxSelectedBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckBoxSelectedBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCheckBoxSelectedBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCheckBoxSelectedBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckBoxSelectedPointerOverBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckBoxSelectedPointerOverBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCheckBoxSelectedPointerOverBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCheckBoxSelectedPointerOverBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckBoxSelectedPressedBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckBoxSelectedPressedBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCheckBoxSelectedPressedBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCheckBoxSelectedPressedBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckBoxSelectedDisabledBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckBoxSelectedDisabledBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCheckBoxSelectedDisabledBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCheckBoxSelectedDisabledBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckBoxBorderBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckBoxBorderBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCheckBoxBorderBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCheckBoxBorderBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckBoxPointerOverBorderBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckBoxPointerOverBorderBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCheckBoxPointerOverBorderBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCheckBoxPointerOverBorderBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckBoxPressedBorderBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckBoxPressedBorderBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCheckBoxPressedBorderBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCheckBoxPressedBorderBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckBoxDisabledBorderBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckBoxDisabledBorderBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCheckBoxDisabledBorderBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCheckBoxDisabledBorderBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckBoxCornerRadius<Impl: IListViewItemPresenter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::CornerRadius) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckBoxCornerRadius() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCheckBoxCornerRadius<Impl: IListViewItemPresenter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::CornerRadius) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCheckBoxCornerRadius(&*(&value as *const <super::super::CornerRadius as ::windows::core::Abi>::Abi as *const <super::super::CornerRadius as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectionIndicatorCornerRadius<Impl: IListViewItemPresenter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::CornerRadius) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectionIndicatorCornerRadius() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectionIndicatorCornerRadius<Impl: IListViewItemPresenter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::CornerRadius) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectionIndicatorCornerRadius(&*(&value as *const <super::super::CornerRadius as ::windows::core::Abi>::Abi as *const <super::super::CornerRadius as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectionIndicatorVisualEnabled<Impl: IListViewItemPresenter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectionIndicatorVisualEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectionIndicatorVisualEnabled<Impl: IListViewItemPresenter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectionIndicatorVisualEnabled(value).into()
        }
        unsafe extern "system" fn SelectionIndicatorMode<Impl: IListViewItemPresenter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ListViewItemPresenterSelectionIndicatorMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectionIndicatorMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectionIndicatorMode<Impl: IListViewItemPresenter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ListViewItemPresenterSelectionIndicatorMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectionIndicatorMode(value).into()
        }
        unsafe extern "system" fn SelectionIndicatorBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectionIndicatorBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectionIndicatorBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectionIndicatorBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectionIndicatorPointerOverBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectionIndicatorPointerOverBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectionIndicatorPointerOverBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectionIndicatorPointerOverBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectionIndicatorPressedBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectionIndicatorPressedBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectionIndicatorPressedBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectionIndicatorPressedBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectionIndicatorDisabledBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectionIndicatorDisabledBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectionIndicatorDisabledBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectionIndicatorDisabledBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedBorderBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedBorderBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedBorderBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectedBorderBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedPressedBorderBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedPressedBorderBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedPressedBorderBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectedPressedBorderBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedDisabledBorderBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedDisabledBorderBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedDisabledBorderBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectedDisabledBorderBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedInnerBorderBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedInnerBorderBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedInnerBorderBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectedInnerBorderBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerOverBorderBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerOverBorderBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPointerOverBorderBrush<Impl: IListViewItemPresenter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPointerOverBorderBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IListViewItemPresenter4, BASE_OFFSET>(),
            SelectedDisabledBackground: SelectedDisabledBackground::<Impl, IMPL_OFFSET>,
            SetSelectedDisabledBackground: SetSelectedDisabledBackground::<Impl, IMPL_OFFSET>,
            CheckPressedBrush: CheckPressedBrush::<Impl, IMPL_OFFSET>,
            SetCheckPressedBrush: SetCheckPressedBrush::<Impl, IMPL_OFFSET>,
            CheckDisabledBrush: CheckDisabledBrush::<Impl, IMPL_OFFSET>,
            SetCheckDisabledBrush: SetCheckDisabledBrush::<Impl, IMPL_OFFSET>,
            CheckBoxPointerOverBrush: CheckBoxPointerOverBrush::<Impl, IMPL_OFFSET>,
            SetCheckBoxPointerOverBrush: SetCheckBoxPointerOverBrush::<Impl, IMPL_OFFSET>,
            CheckBoxPressedBrush: CheckBoxPressedBrush::<Impl, IMPL_OFFSET>,
            SetCheckBoxPressedBrush: SetCheckBoxPressedBrush::<Impl, IMPL_OFFSET>,
            CheckBoxDisabledBrush: CheckBoxDisabledBrush::<Impl, IMPL_OFFSET>,
            SetCheckBoxDisabledBrush: SetCheckBoxDisabledBrush::<Impl, IMPL_OFFSET>,
            CheckBoxSelectedBrush: CheckBoxSelectedBrush::<Impl, IMPL_OFFSET>,
            SetCheckBoxSelectedBrush: SetCheckBoxSelectedBrush::<Impl, IMPL_OFFSET>,
            CheckBoxSelectedPointerOverBrush: CheckBoxSelectedPointerOverBrush::<Impl, IMPL_OFFSET>,
            SetCheckBoxSelectedPointerOverBrush: SetCheckBoxSelectedPointerOverBrush::<Impl, IMPL_OFFSET>,
            CheckBoxSelectedPressedBrush: CheckBoxSelectedPressedBrush::<Impl, IMPL_OFFSET>,
            SetCheckBoxSelectedPressedBrush: SetCheckBoxSelectedPressedBrush::<Impl, IMPL_OFFSET>,
            CheckBoxSelectedDisabledBrush: CheckBoxSelectedDisabledBrush::<Impl, IMPL_OFFSET>,
            SetCheckBoxSelectedDisabledBrush: SetCheckBoxSelectedDisabledBrush::<Impl, IMPL_OFFSET>,
            CheckBoxBorderBrush: CheckBoxBorderBrush::<Impl, IMPL_OFFSET>,
            SetCheckBoxBorderBrush: SetCheckBoxBorderBrush::<Impl, IMPL_OFFSET>,
            CheckBoxPointerOverBorderBrush: CheckBoxPointerOverBorderBrush::<Impl, IMPL_OFFSET>,
            SetCheckBoxPointerOverBorderBrush: SetCheckBoxPointerOverBorderBrush::<Impl, IMPL_OFFSET>,
            CheckBoxPressedBorderBrush: CheckBoxPressedBorderBrush::<Impl, IMPL_OFFSET>,
            SetCheckBoxPressedBorderBrush: SetCheckBoxPressedBorderBrush::<Impl, IMPL_OFFSET>,
            CheckBoxDisabledBorderBrush: CheckBoxDisabledBorderBrush::<Impl, IMPL_OFFSET>,
            SetCheckBoxDisabledBorderBrush: SetCheckBoxDisabledBorderBrush::<Impl, IMPL_OFFSET>,
            CheckBoxCornerRadius: CheckBoxCornerRadius::<Impl, IMPL_OFFSET>,
            SetCheckBoxCornerRadius: SetCheckBoxCornerRadius::<Impl, IMPL_OFFSET>,
            SelectionIndicatorCornerRadius: SelectionIndicatorCornerRadius::<Impl, IMPL_OFFSET>,
            SetSelectionIndicatorCornerRadius: SetSelectionIndicatorCornerRadius::<Impl, IMPL_OFFSET>,
            SelectionIndicatorVisualEnabled: SelectionIndicatorVisualEnabled::<Impl, IMPL_OFFSET>,
            SetSelectionIndicatorVisualEnabled: SetSelectionIndicatorVisualEnabled::<Impl, IMPL_OFFSET>,
            SelectionIndicatorMode: SelectionIndicatorMode::<Impl, IMPL_OFFSET>,
            SetSelectionIndicatorMode: SetSelectionIndicatorMode::<Impl, IMPL_OFFSET>,
            SelectionIndicatorBrush: SelectionIndicatorBrush::<Impl, IMPL_OFFSET>,
            SetSelectionIndicatorBrush: SetSelectionIndicatorBrush::<Impl, IMPL_OFFSET>,
            SelectionIndicatorPointerOverBrush: SelectionIndicatorPointerOverBrush::<Impl, IMPL_OFFSET>,
            SetSelectionIndicatorPointerOverBrush: SetSelectionIndicatorPointerOverBrush::<Impl, IMPL_OFFSET>,
            SelectionIndicatorPressedBrush: SelectionIndicatorPressedBrush::<Impl, IMPL_OFFSET>,
            SetSelectionIndicatorPressedBrush: SetSelectionIndicatorPressedBrush::<Impl, IMPL_OFFSET>,
            SelectionIndicatorDisabledBrush: SelectionIndicatorDisabledBrush::<Impl, IMPL_OFFSET>,
            SetSelectionIndicatorDisabledBrush: SetSelectionIndicatorDisabledBrush::<Impl, IMPL_OFFSET>,
            SelectedBorderBrush: SelectedBorderBrush::<Impl, IMPL_OFFSET>,
            SetSelectedBorderBrush: SetSelectedBorderBrush::<Impl, IMPL_OFFSET>,
            SelectedPressedBorderBrush: SelectedPressedBorderBrush::<Impl, IMPL_OFFSET>,
            SetSelectedPressedBorderBrush: SetSelectedPressedBorderBrush::<Impl, IMPL_OFFSET>,
            SelectedDisabledBorderBrush: SelectedDisabledBorderBrush::<Impl, IMPL_OFFSET>,
            SetSelectedDisabledBorderBrush: SetSelectedDisabledBorderBrush::<Impl, IMPL_OFFSET>,
            SelectedInnerBorderBrush: SelectedInnerBorderBrush::<Impl, IMPL_OFFSET>,
            SetSelectedInnerBorderBrush: SetSelectedInnerBorderBrush::<Impl, IMPL_OFFSET>,
            PointerOverBorderBrush: PointerOverBorderBrush::<Impl, IMPL_OFFSET>,
            SetPointerOverBorderBrush: SetPointerOverBorderBrush::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IListViewItemPresenter4 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewItemPresenterFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListViewItemPresenterFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IListViewItemPresenterFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IListViewItemPresenterFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IListViewItemPresenterFactory as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewItemPresenterStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListViewItemPresenterStaticsVtbl {
        unsafe extern "system" fn SelectionCheckMarkVisualEnabledProperty<Impl: IListViewItemPresenterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectionCheckMarkVisualEnabledProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckHintBrushProperty<Impl: IListViewItemPresenterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckHintBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckSelectingBrushProperty<Impl: IListViewItemPresenterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckSelectingBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckBrushProperty<Impl: IListViewItemPresenterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DragBackgroundProperty<Impl: IListViewItemPresenterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DragBackgroundProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DragForegroundProperty<Impl: IListViewItemPresenterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DragForegroundProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FocusBorderBrushProperty<Impl: IListViewItemPresenterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FocusBorderBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PlaceholderBackgroundProperty<Impl: IListViewItemPresenterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlaceholderBackgroundProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PointerOverBackgroundProperty<Impl: IListViewItemPresenterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerOverBackgroundProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedBackgroundProperty<Impl: IListViewItemPresenterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedBackgroundProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedForegroundProperty<Impl: IListViewItemPresenterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedForegroundProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedPointerOverBackgroundProperty<Impl: IListViewItemPresenterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedPointerOverBackgroundProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedPointerOverBorderBrushProperty<Impl: IListViewItemPresenterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedPointerOverBorderBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedBorderThicknessProperty<Impl: IListViewItemPresenterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedBorderThicknessProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisabledOpacityProperty<Impl: IListViewItemPresenterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisabledOpacityProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DragOpacityProperty<Impl: IListViewItemPresenterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DragOpacityProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReorderHintOffsetProperty<Impl: IListViewItemPresenterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReorderHintOffsetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ListViewItemPresenterHorizontalContentAlignmentProperty<Impl: IListViewItemPresenterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ListViewItemPresenterHorizontalContentAlignmentProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ListViewItemPresenterVerticalContentAlignmentProperty<Impl: IListViewItemPresenterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ListViewItemPresenterVerticalContentAlignmentProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ListViewItemPresenterPaddingProperty<Impl: IListViewItemPresenterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ListViewItemPresenterPaddingProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PointerOverBackgroundMarginProperty<Impl: IListViewItemPresenterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerOverBackgroundMarginProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentMarginProperty<Impl: IListViewItemPresenterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentMarginProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IListViewItemPresenterStatics, BASE_OFFSET>(),
            SelectionCheckMarkVisualEnabledProperty: SelectionCheckMarkVisualEnabledProperty::<Impl, IMPL_OFFSET>,
            CheckHintBrushProperty: CheckHintBrushProperty::<Impl, IMPL_OFFSET>,
            CheckSelectingBrushProperty: CheckSelectingBrushProperty::<Impl, IMPL_OFFSET>,
            CheckBrushProperty: CheckBrushProperty::<Impl, IMPL_OFFSET>,
            DragBackgroundProperty: DragBackgroundProperty::<Impl, IMPL_OFFSET>,
            DragForegroundProperty: DragForegroundProperty::<Impl, IMPL_OFFSET>,
            FocusBorderBrushProperty: FocusBorderBrushProperty::<Impl, IMPL_OFFSET>,
            PlaceholderBackgroundProperty: PlaceholderBackgroundProperty::<Impl, IMPL_OFFSET>,
            PointerOverBackgroundProperty: PointerOverBackgroundProperty::<Impl, IMPL_OFFSET>,
            SelectedBackgroundProperty: SelectedBackgroundProperty::<Impl, IMPL_OFFSET>,
            SelectedForegroundProperty: SelectedForegroundProperty::<Impl, IMPL_OFFSET>,
            SelectedPointerOverBackgroundProperty: SelectedPointerOverBackgroundProperty::<Impl, IMPL_OFFSET>,
            SelectedPointerOverBorderBrushProperty: SelectedPointerOverBorderBrushProperty::<Impl, IMPL_OFFSET>,
            SelectedBorderThicknessProperty: SelectedBorderThicknessProperty::<Impl, IMPL_OFFSET>,
            DisabledOpacityProperty: DisabledOpacityProperty::<Impl, IMPL_OFFSET>,
            DragOpacityProperty: DragOpacityProperty::<Impl, IMPL_OFFSET>,
            ReorderHintOffsetProperty: ReorderHintOffsetProperty::<Impl, IMPL_OFFSET>,
            ListViewItemPresenterHorizontalContentAlignmentProperty: ListViewItemPresenterHorizontalContentAlignmentProperty::<Impl, IMPL_OFFSET>,
            ListViewItemPresenterVerticalContentAlignmentProperty: ListViewItemPresenterVerticalContentAlignmentProperty::<Impl, IMPL_OFFSET>,
            ListViewItemPresenterPaddingProperty: ListViewItemPresenterPaddingProperty::<Impl, IMPL_OFFSET>,
            PointerOverBackgroundMarginProperty: PointerOverBackgroundMarginProperty::<Impl, IMPL_OFFSET>,
            ContentMarginProperty: ContentMarginProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IListViewItemPresenterStatics as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewItemPresenterStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListViewItemPresenterStatics2Vtbl {
        unsafe extern "system" fn SelectedPressedBackgroundProperty<Impl: IListViewItemPresenterStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedPressedBackgroundProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PressedBackgroundProperty<Impl: IListViewItemPresenterStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PressedBackgroundProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckBoxBrushProperty<Impl: IListViewItemPresenterStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckBoxBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FocusSecondaryBorderBrushProperty<Impl: IListViewItemPresenterStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FocusSecondaryBorderBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckModeProperty<Impl: IListViewItemPresenterStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckModeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PointerOverForegroundProperty<Impl: IListViewItemPresenterStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerOverForegroundProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IListViewItemPresenterStatics2, BASE_OFFSET>(),
            SelectedPressedBackgroundProperty: SelectedPressedBackgroundProperty::<Impl, IMPL_OFFSET>,
            PressedBackgroundProperty: PressedBackgroundProperty::<Impl, IMPL_OFFSET>,
            CheckBoxBrushProperty: CheckBoxBrushProperty::<Impl, IMPL_OFFSET>,
            FocusSecondaryBorderBrushProperty: FocusSecondaryBorderBrushProperty::<Impl, IMPL_OFFSET>,
            CheckModeProperty: CheckModeProperty::<Impl, IMPL_OFFSET>,
            PointerOverForegroundProperty: PointerOverForegroundProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IListViewItemPresenterStatics2 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewItemPresenterStatics3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListViewItemPresenterStatics3Vtbl {
        unsafe extern "system" fn RevealBackgroundProperty<Impl: IListViewItemPresenterStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RevealBackgroundProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RevealBorderBrushProperty<Impl: IListViewItemPresenterStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RevealBorderBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RevealBorderThicknessProperty<Impl: IListViewItemPresenterStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RevealBorderThicknessProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RevealBackgroundShowsAboveContentProperty<Impl: IListViewItemPresenterStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RevealBackgroundShowsAboveContentProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IListViewItemPresenterStatics3, BASE_OFFSET>(),
            RevealBackgroundProperty: RevealBackgroundProperty::<Impl, IMPL_OFFSET>,
            RevealBorderBrushProperty: RevealBorderBrushProperty::<Impl, IMPL_OFFSET>,
            RevealBorderThicknessProperty: RevealBorderThicknessProperty::<Impl, IMPL_OFFSET>,
            RevealBackgroundShowsAboveContentProperty: RevealBackgroundShowsAboveContentProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IListViewItemPresenterStatics3 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewItemPresenterStatics4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListViewItemPresenterStatics4Vtbl {
        unsafe extern "system" fn SelectedDisabledBackgroundProperty<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedDisabledBackgroundProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckPressedBrushProperty<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckPressedBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckDisabledBrushProperty<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckDisabledBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckBoxPointerOverBrushProperty<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckBoxPointerOverBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckBoxPressedBrushProperty<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckBoxPressedBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckBoxDisabledBrushProperty<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckBoxDisabledBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckBoxSelectedBrushProperty<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckBoxSelectedBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckBoxSelectedPointerOverBrushProperty<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckBoxSelectedPointerOverBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckBoxSelectedPressedBrushProperty<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckBoxSelectedPressedBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckBoxSelectedDisabledBrushProperty<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckBoxSelectedDisabledBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckBoxBorderBrushProperty<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckBoxBorderBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckBoxPointerOverBorderBrushProperty<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckBoxPointerOverBorderBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckBoxPressedBorderBrushProperty<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckBoxPressedBorderBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckBoxDisabledBorderBrushProperty<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckBoxDisabledBorderBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckBoxCornerRadiusProperty<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckBoxCornerRadiusProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectionIndicatorCornerRadiusProperty<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectionIndicatorCornerRadiusProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectionIndicatorVisualEnabledProperty<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectionIndicatorVisualEnabledProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectionIndicatorModeProperty<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectionIndicatorModeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectionIndicatorBrushProperty<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectionIndicatorBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectionIndicatorPointerOverBrushProperty<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectionIndicatorPointerOverBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectionIndicatorPressedBrushProperty<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectionIndicatorPressedBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectionIndicatorDisabledBrushProperty<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectionIndicatorDisabledBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedBorderBrushProperty<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedBorderBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedPressedBorderBrushProperty<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedPressedBorderBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedDisabledBorderBrushProperty<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedDisabledBorderBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedInnerBorderBrushProperty<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedInnerBorderBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PointerOverBorderBrushProperty<Impl: IListViewItemPresenterStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerOverBorderBrushProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IListViewItemPresenterStatics4, BASE_OFFSET>(),
            SelectedDisabledBackgroundProperty: SelectedDisabledBackgroundProperty::<Impl, IMPL_OFFSET>,
            CheckPressedBrushProperty: CheckPressedBrushProperty::<Impl, IMPL_OFFSET>,
            CheckDisabledBrushProperty: CheckDisabledBrushProperty::<Impl, IMPL_OFFSET>,
            CheckBoxPointerOverBrushProperty: CheckBoxPointerOverBrushProperty::<Impl, IMPL_OFFSET>,
            CheckBoxPressedBrushProperty: CheckBoxPressedBrushProperty::<Impl, IMPL_OFFSET>,
            CheckBoxDisabledBrushProperty: CheckBoxDisabledBrushProperty::<Impl, IMPL_OFFSET>,
            CheckBoxSelectedBrushProperty: CheckBoxSelectedBrushProperty::<Impl, IMPL_OFFSET>,
            CheckBoxSelectedPointerOverBrushProperty: CheckBoxSelectedPointerOverBrushProperty::<Impl, IMPL_OFFSET>,
            CheckBoxSelectedPressedBrushProperty: CheckBoxSelectedPressedBrushProperty::<Impl, IMPL_OFFSET>,
            CheckBoxSelectedDisabledBrushProperty: CheckBoxSelectedDisabledBrushProperty::<Impl, IMPL_OFFSET>,
            CheckBoxBorderBrushProperty: CheckBoxBorderBrushProperty::<Impl, IMPL_OFFSET>,
            CheckBoxPointerOverBorderBrushProperty: CheckBoxPointerOverBorderBrushProperty::<Impl, IMPL_OFFSET>,
            CheckBoxPressedBorderBrushProperty: CheckBoxPressedBorderBrushProperty::<Impl, IMPL_OFFSET>,
            CheckBoxDisabledBorderBrushProperty: CheckBoxDisabledBorderBrushProperty::<Impl, IMPL_OFFSET>,
            CheckBoxCornerRadiusProperty: CheckBoxCornerRadiusProperty::<Impl, IMPL_OFFSET>,
            SelectionIndicatorCornerRadiusProperty: SelectionIndicatorCornerRadiusProperty::<Impl, IMPL_OFFSET>,
            SelectionIndicatorVisualEnabledProperty: SelectionIndicatorVisualEnabledProperty::<Impl, IMPL_OFFSET>,
            SelectionIndicatorModeProperty: SelectionIndicatorModeProperty::<Impl, IMPL_OFFSET>,
            SelectionIndicatorBrushProperty: SelectionIndicatorBrushProperty::<Impl, IMPL_OFFSET>,
            SelectionIndicatorPointerOverBrushProperty: SelectionIndicatorPointerOverBrushProperty::<Impl, IMPL_OFFSET>,
            SelectionIndicatorPressedBrushProperty: SelectionIndicatorPressedBrushProperty::<Impl, IMPL_OFFSET>,
            SelectionIndicatorDisabledBrushProperty: SelectionIndicatorDisabledBrushProperty::<Impl, IMPL_OFFSET>,
            SelectedBorderBrushProperty: SelectedBorderBrushProperty::<Impl, IMPL_OFFSET>,
            SelectedPressedBorderBrushProperty: SelectedPressedBorderBrushProperty::<Impl, IMPL_OFFSET>,
            SelectedDisabledBorderBrushProperty: SelectedDisabledBorderBrushProperty::<Impl, IMPL_OFFSET>,
            SelectedInnerBorderBrushProperty: SelectedInnerBorderBrushProperty::<Impl, IMPL_OFFSET>,
            PointerOverBorderBrushProperty: PointerOverBorderBrushProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IListViewItemPresenterStatics4 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewItemTemplateSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListViewItemTemplateSettingsVtbl {
        unsafe extern "system" fn DragItemsCount<Impl: IListViewItemTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DragItemsCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IListViewItemTemplateSettings, BASE_OFFSET>(),
            DragItemsCount: DragItemsCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IListViewItemTemplateSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILoopingSelector {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ILoopingSelector";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ILoopingSelectorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoopingSelectorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILoopingSelectorVtbl {
        unsafe extern "system" fn ShouldLoop<Impl: ILoopingSelectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShouldLoop() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShouldLoop<Impl: ILoopingSelectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShouldLoop(value).into()
        }
        unsafe extern "system" fn Items<Impl: ILoopingSelectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Items() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetItems<Impl: ILoopingSelectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetItems(&*(&value as *const <super::super::super::super::Foundation::Collections::IVector<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Collections::IVector<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedIndex<Impl: ILoopingSelectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedIndex<Impl: ILoopingSelectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectedIndex(value).into()
        }
        unsafe extern "system" fn SelectedItem<Impl: ILoopingSelectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedItem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedItem<Impl: ILoopingSelectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectedItem(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ItemWidth<Impl: ILoopingSelectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetItemWidth<Impl: ILoopingSelectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetItemWidth(value).into()
        }
        unsafe extern "system" fn ItemHeight<Impl: ILoopingSelectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetItemHeight<Impl: ILoopingSelectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetItemHeight(value).into()
        }
        unsafe extern "system" fn ItemTemplate<Impl: ILoopingSelectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemTemplate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetItemTemplate<Impl: ILoopingSelectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetItemTemplate(&*(&value as *const <super::super::DataTemplate as ::windows::core::Abi>::Abi as *const <super::super::DataTemplate as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectionChanged<Impl: ILoopingSelectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectionChanged(&*(&handler as *const <super::SelectionChangedEventHandler as ::windows::core::Abi>::Abi as *const <super::SelectionChangedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSelectionChanged<Impl: ILoopingSelectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSelectionChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILoopingSelector, BASE_OFFSET>(),
            ShouldLoop: ShouldLoop::<Impl, IMPL_OFFSET>,
            SetShouldLoop: SetShouldLoop::<Impl, IMPL_OFFSET>,
            Items: Items::<Impl, IMPL_OFFSET>,
            SetItems: SetItems::<Impl, IMPL_OFFSET>,
            SelectedIndex: SelectedIndex::<Impl, IMPL_OFFSET>,
            SetSelectedIndex: SetSelectedIndex::<Impl, IMPL_OFFSET>,
            SelectedItem: SelectedItem::<Impl, IMPL_OFFSET>,
            SetSelectedItem: SetSelectedItem::<Impl, IMPL_OFFSET>,
            ItemWidth: ItemWidth::<Impl, IMPL_OFFSET>,
            SetItemWidth: SetItemWidth::<Impl, IMPL_OFFSET>,
            ItemHeight: ItemHeight::<Impl, IMPL_OFFSET>,
            SetItemHeight: SetItemHeight::<Impl, IMPL_OFFSET>,
            ItemTemplate: ItemTemplate::<Impl, IMPL_OFFSET>,
            SetItemTemplate: SetItemTemplate::<Impl, IMPL_OFFSET>,
            SelectionChanged: SelectionChanged::<Impl, IMPL_OFFSET>,
            RemoveSelectionChanged: RemoveSelectionChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoopingSelector as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoopingSelectorItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILoopingSelectorItemVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ILoopingSelectorItem, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoopingSelectorItem as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoopingSelectorPanelImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILoopingSelectorPanelVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ILoopingSelectorPanel, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoopingSelectorPanel as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoopingSelectorStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILoopingSelectorStaticsVtbl {
        unsafe extern "system" fn ShouldLoopProperty<Impl: ILoopingSelectorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShouldLoopProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemsProperty<Impl: ILoopingSelectorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemsProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedIndexProperty<Impl: ILoopingSelectorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedIndexProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedItemProperty<Impl: ILoopingSelectorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedItemProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemWidthProperty<Impl: ILoopingSelectorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemWidthProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemHeightProperty<Impl: ILoopingSelectorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemHeightProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemTemplateProperty<Impl: ILoopingSelectorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemTemplateProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILoopingSelectorStatics, BASE_OFFSET>(),
            ShouldLoopProperty: ShouldLoopProperty::<Impl, IMPL_OFFSET>,
            ItemsProperty: ItemsProperty::<Impl, IMPL_OFFSET>,
            SelectedIndexProperty: SelectedIndexProperty::<Impl, IMPL_OFFSET>,
            SelectedItemProperty: SelectedItemProperty::<Impl, IMPL_OFFSET>,
            ItemWidthProperty: ItemWidthProperty::<Impl, IMPL_OFFSET>,
            ItemHeightProperty: ItemHeightProperty::<Impl, IMPL_OFFSET>,
            ItemTemplateProperty: ItemTemplateProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoopingSelectorStatics as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMenuFlyoutItemTemplateSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMenuFlyoutItemTemplateSettingsVtbl {
        unsafe extern "system" fn KeyboardAcceleratorTextMinWidth<Impl: IMenuFlyoutItemTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyboardAcceleratorTextMinWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMenuFlyoutItemTemplateSettings, BASE_OFFSET>(),
            KeyboardAcceleratorTextMinWidth: KeyboardAcceleratorTextMinWidth::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMenuFlyoutItemTemplateSettings as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMenuFlyoutPresenterTemplateSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMenuFlyoutPresenterTemplateSettingsVtbl {
        unsafe extern "system" fn FlyoutContentMinWidth<Impl: IMenuFlyoutPresenterTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FlyoutContentMinWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMenuFlyoutPresenterTemplateSettings, BASE_OFFSET>(),
            FlyoutContentMinWidth: FlyoutContentMinWidth::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMenuFlyoutPresenterTemplateSettings as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INavigationViewItemPresenterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INavigationViewItemPresenterVtbl {
        unsafe extern "system" fn Icon<Impl: INavigationViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Icon() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIcon<Impl: INavigationViewItemPresenterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIcon(&*(&value as *const <super::IconElement as ::windows::core::Abi>::Abi as *const <super::IconElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INavigationViewItemPresenter, BASE_OFFSET>(),
            Icon: Icon::<Impl, IMPL_OFFSET>,
            SetIcon: SetIcon::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INavigationViewItemPresenter as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INavigationViewItemPresenterFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INavigationViewItemPresenterFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: INavigationViewItemPresenterFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, INavigationViewItemPresenterFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INavigationViewItemPresenterFactory as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INavigationViewItemPresenterStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INavigationViewItemPresenterStaticsVtbl {
        unsafe extern "system" fn IconProperty<Impl: INavigationViewItemPresenterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IconProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INavigationViewItemPresenterStatics, BASE_OFFSET>(),
            IconProperty: IconProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INavigationViewItemPresenterStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IOrientedVirtualizingPanel {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IOrientedVirtualizingPanel";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IOrientedVirtualizingPanelVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOrientedVirtualizingPanelImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOrientedVirtualizingPanelVtbl {
        unsafe extern "system" fn CanVerticallyScroll<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanVerticallyScroll() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCanVerticallyScroll<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCanVerticallyScroll(value).into()
        }
        unsafe extern "system" fn CanHorizontallyScroll<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanHorizontallyScroll() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCanHorizontallyScroll<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCanHorizontallyScroll(value).into()
        }
        unsafe extern "system" fn ExtentWidth<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtentWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtentHeight<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtentHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ViewportWidth<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ViewportWidth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ViewportHeight<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ViewportHeight() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HorizontalOffset<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HorizontalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerticalOffset<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VerticalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScrollOwner<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScrollOwner() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScrollOwner<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScrollOwner(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LineUp<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LineUp().into()
        }
        unsafe extern "system" fn LineDown<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LineDown().into()
        }
        unsafe extern "system" fn LineLeft<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LineLeft().into()
        }
        unsafe extern "system" fn LineRight<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LineRight().into()
        }
        unsafe extern "system" fn PageUp<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PageUp().into()
        }
        unsafe extern "system" fn PageDown<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PageDown().into()
        }
        unsafe extern "system" fn PageLeft<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PageLeft().into()
        }
        unsafe extern "system" fn PageRight<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PageRight().into()
        }
        unsafe extern "system" fn MouseWheelUp<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MouseWheelUp().into()
        }
        unsafe extern "system" fn MouseWheelDown<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MouseWheelDown().into()
        }
        unsafe extern "system" fn MouseWheelLeft<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MouseWheelLeft().into()
        }
        unsafe extern "system" fn MouseWheelRight<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MouseWheelRight().into()
        }
        unsafe extern "system" fn SetHorizontalOffset<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHorizontalOffset(offset).into()
        }
        unsafe extern "system" fn SetVerticalOffset<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVerticalOffset(offset).into()
        }
        unsafe extern "system" fn MakeVisible<Impl: IOrientedVirtualizingPanelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visual: ::windows::core::RawPtr, rectangle: super::super::super::super::Foundation::Rect, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MakeVisible(&*(&visual as *const <super::super::UIElement as ::windows::core::Abi>::Abi as *const <super::super::UIElement as ::windows::core::DefaultType>::DefaultType), &*(&rectangle as *const <super::super::super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IOrientedVirtualizingPanel, BASE_OFFSET>(),
            CanVerticallyScroll: CanVerticallyScroll::<Impl, IMPL_OFFSET>,
            SetCanVerticallyScroll: SetCanVerticallyScroll::<Impl, IMPL_OFFSET>,
            CanHorizontallyScroll: CanHorizontallyScroll::<Impl, IMPL_OFFSET>,
            SetCanHorizontallyScroll: SetCanHorizontallyScroll::<Impl, IMPL_OFFSET>,
            ExtentWidth: ExtentWidth::<Impl, IMPL_OFFSET>,
            ExtentHeight: ExtentHeight::<Impl, IMPL_OFFSET>,
            ViewportWidth: ViewportWidth::<Impl, IMPL_OFFSET>,
            ViewportHeight: ViewportHeight::<Impl, IMPL_OFFSET>,
            HorizontalOffset: HorizontalOffset::<Impl, IMPL_OFFSET>,
            VerticalOffset: VerticalOffset::<Impl, IMPL_OFFSET>,
            ScrollOwner: ScrollOwner::<Impl, IMPL_OFFSET>,
            SetScrollOwner: SetScrollOwner::<Impl, IMPL_OFFSET>,
            LineUp: LineUp::<Impl, IMPL_OFFSET>,
            LineDown: LineDown::<Impl, IMPL_OFFSET>,
            LineLeft: LineLeft::<Impl, IMPL_OFFSET>,
            LineRight: LineRight::<Impl, IMPL_OFFSET>,
            PageUp: PageUp::<Impl, IMPL_OFFSET>,
            PageDown: PageDown::<Impl, IMPL_OFFSET>,
            PageLeft: PageLeft::<Impl, IMPL_OFFSET>,
            PageRight: PageRight::<Impl, IMPL_OFFSET>,
            MouseWheelUp: MouseWheelUp::<Impl, IMPL_OFFSET>,
            MouseWheelDown: MouseWheelDown::<Impl, IMPL_OFFSET>,
            MouseWheelLeft: MouseWheelLeft::<Impl, IMPL_OFFSET>,
            MouseWheelRight: MouseWheelRight::<Impl, IMPL_OFFSET>,
            SetHorizontalOffset: SetHorizontalOffset::<Impl, IMPL_OFFSET>,
            SetVerticalOffset: SetVerticalOffset::<Impl, IMPL_OFFSET>,
            MakeVisible: MakeVisible::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOrientedVirtualizingPanel as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOrientedVirtualizingPanelFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOrientedVirtualizingPanelFactoryVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IOrientedVirtualizingPanelFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOrientedVirtualizingPanelFactory as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPickerFlyoutBaseImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPickerFlyoutBaseVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPickerFlyoutBase, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPickerFlyoutBase as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPickerFlyoutBaseFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPickerFlyoutBaseFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IPickerFlyoutBaseFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPickerFlyoutBaseFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPickerFlyoutBaseFactory as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPickerFlyoutBaseOverridesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPickerFlyoutBaseOverridesVtbl {
        unsafe extern "system" fn OnConfirmed<Impl: IPickerFlyoutBaseOverridesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnConfirmed().into()
        }
        unsafe extern "system" fn ShouldShowConfirmationButtons<Impl: IPickerFlyoutBaseOverridesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShouldShowConfirmationButtons() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPickerFlyoutBaseOverrides, BASE_OFFSET>(),
            OnConfirmed: OnConfirmed::<Impl, IMPL_OFFSET>,
            ShouldShowConfirmationButtons: ShouldShowConfirmationButtons::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPickerFlyoutBaseOverrides as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPickerFlyoutBaseStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPickerFlyoutBaseStaticsVtbl {
        unsafe extern "system" fn TitleProperty<Impl: IPickerFlyoutBaseStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TitleProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTitle<Impl: IPickerFlyoutBaseStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTitle(&*(&element as *const <super::super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTitle<Impl: IPickerFlyoutBaseStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTitle(&*(&element as *const <super::super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::super::DependencyObject as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPickerFlyoutBaseStatics, BASE_OFFSET>(),
            TitleProperty: TitleProperty::<Impl, IMPL_OFFSET>,
            GetTitle: GetTitle::<Impl, IMPL_OFFSET>,
            SetTitle: SetTitle::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPickerFlyoutBaseStatics as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPivotHeaderItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPivotHeaderItemVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPivotHeaderItem, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPivotHeaderItem as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPivotHeaderItemFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPivotHeaderItemFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IPivotHeaderItemFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPivotHeaderItemFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPivotHeaderItemFactory as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPivotHeaderPanelImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPivotHeaderPanelVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPivotHeaderPanel, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPivotHeaderPanel as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPivotPanelImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPivotPanelVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPivotPanel, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPivotPanel as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPopup {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IPopup";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation", feature = "implement_exclusive"))]
impl IPopupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPopupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPopupVtbl {
        unsafe extern "system" fn Child<Impl: IPopupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Child() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChild<Impl: IPopupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetChild(&*(&value as *const <super::super::UIElement as ::windows::core::Abi>::Abi as *const <super::super::UIElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsOpen<Impl: IPopupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOpen() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsOpen<Impl: IPopupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsOpen(value).into()
        }
        unsafe extern "system" fn HorizontalOffset<Impl: IPopupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HorizontalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHorizontalOffset<Impl: IPopupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHorizontalOffset(value).into()
        }
        unsafe extern "system" fn VerticalOffset<Impl: IPopupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VerticalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVerticalOffset<Impl: IPopupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVerticalOffset(value).into()
        }
        unsafe extern "system" fn ChildTransitions<Impl: IPopupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChildTransitions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChildTransitions<Impl: IPopupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetChildTransitions(&*(&value as *const <super::super::Media::Animation::TransitionCollection as ::windows::core::Abi>::Abi as *const <super::super::Media::Animation::TransitionCollection as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsLightDismissEnabled<Impl: IPopupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLightDismissEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsLightDismissEnabled<Impl: IPopupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsLightDismissEnabled(value).into()
        }
        unsafe extern "system" fn Opened<Impl: IPopupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Opened(&*(&handler as *const <super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveOpened<Impl: IPopupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveOpened(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Closed<Impl: IPopupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Closed(&*(&handler as *const <super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveClosed<Impl: IPopupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveClosed(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPopup, BASE_OFFSET>(),
            Child: Child::<Impl, IMPL_OFFSET>,
            SetChild: SetChild::<Impl, IMPL_OFFSET>,
            IsOpen: IsOpen::<Impl, IMPL_OFFSET>,
            SetIsOpen: SetIsOpen::<Impl, IMPL_OFFSET>,
            HorizontalOffset: HorizontalOffset::<Impl, IMPL_OFFSET>,
            SetHorizontalOffset: SetHorizontalOffset::<Impl, IMPL_OFFSET>,
            VerticalOffset: VerticalOffset::<Impl, IMPL_OFFSET>,
            SetVerticalOffset: SetVerticalOffset::<Impl, IMPL_OFFSET>,
            ChildTransitions: ChildTransitions::<Impl, IMPL_OFFSET>,
            SetChildTransitions: SetChildTransitions::<Impl, IMPL_OFFSET>,
            IsLightDismissEnabled: IsLightDismissEnabled::<Impl, IMPL_OFFSET>,
            SetIsLightDismissEnabled: SetIsLightDismissEnabled::<Impl, IMPL_OFFSET>,
            Opened: Opened::<Impl, IMPL_OFFSET>,
            RemoveOpened: RemoveOpened::<Impl, IMPL_OFFSET>,
            Closed: Closed::<Impl, IMPL_OFFSET>,
            RemoveClosed: RemoveClosed::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPopup as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPopup2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPopup2Vtbl {
        unsafe extern "system" fn LightDismissOverlayMode<Impl: IPopup2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::LightDismissOverlayMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LightDismissOverlayMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLightDismissOverlayMode<Impl: IPopup2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::LightDismissOverlayMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLightDismissOverlayMode(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPopup2, BASE_OFFSET>(),
            LightDismissOverlayMode: LightDismissOverlayMode::<Impl, IMPL_OFFSET>,
            SetLightDismissOverlayMode: SetLightDismissOverlayMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPopup2 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPopup3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPopup3Vtbl {
        unsafe extern "system" fn ShouldConstrainToRootBounds<Impl: IPopup3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShouldConstrainToRootBounds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetShouldConstrainToRootBounds<Impl: IPopup3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShouldConstrainToRootBounds(value).into()
        }
        unsafe extern "system" fn IsConstrainedToRootBounds<Impl: IPopup3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsConstrainedToRootBounds() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPopup3, BASE_OFFSET>(),
            ShouldConstrainToRootBounds: ShouldConstrainToRootBounds::<Impl, IMPL_OFFSET>,
            SetShouldConstrainToRootBounds: SetShouldConstrainToRootBounds::<Impl, IMPL_OFFSET>,
            IsConstrainedToRootBounds: IsConstrainedToRootBounds::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPopup3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IPopup4Impl: Sized {
    fn PlacementTarget(&self) -> ::windows::core::Result<super::super::FrameworkElement>;
    fn SetPlacementTarget(&self, value: &::core::option::Option<super::super::FrameworkElement>) -> ::windows::core::Result<()>;
    fn DesiredPlacement(&self) -> ::windows::core::Result<PopupPlacementMode>;
    fn SetDesiredPlacement(&self, value: PopupPlacementMode) -> ::windows::core::Result<()>;
    fn ActualPlacement(&self) -> ::windows::core::Result<PopupPlacementMode>;
    fn ActualPlacementChanged(&self, handler: &::core::option::Option<super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveActualPlacementChanged(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPopup4 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IPopup4";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPopup4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPopup4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPopup4Vtbl {
        unsafe extern "system" fn PlacementTarget<Impl: IPopup4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlacementTarget() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlacementTarget<Impl: IPopup4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPlacementTarget(&*(&value as *const <super::super::FrameworkElement as ::windows::core::Abi>::Abi as *const <super::super::FrameworkElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DesiredPlacement<Impl: IPopup4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PopupPlacementMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredPlacement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredPlacement<Impl: IPopup4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PopupPlacementMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDesiredPlacement(value).into()
        }
        unsafe extern "system" fn ActualPlacement<Impl: IPopup4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PopupPlacementMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActualPlacement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActualPlacementChanged<Impl: IPopup4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActualPlacementChanged(&*(&handler as *const <super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveActualPlacementChanged<Impl: IPopup4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveActualPlacementChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPopup4, BASE_OFFSET>(),
            PlacementTarget: PlacementTarget::<Impl, IMPL_OFFSET>,
            SetPlacementTarget: SetPlacementTarget::<Impl, IMPL_OFFSET>,
            DesiredPlacement: DesiredPlacement::<Impl, IMPL_OFFSET>,
            SetDesiredPlacement: SetDesiredPlacement::<Impl, IMPL_OFFSET>,
            ActualPlacement: ActualPlacement::<Impl, IMPL_OFFSET>,
            ActualPlacementChanged: ActualPlacementChanged::<Impl, IMPL_OFFSET>,
            RemoveActualPlacementChanged: RemoveActualPlacementChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPopup4 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPopupStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPopupStaticsVtbl {
        unsafe extern "system" fn ChildProperty<Impl: IPopupStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChildProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOpenProperty<Impl: IPopupStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOpenProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HorizontalOffsetProperty<Impl: IPopupStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HorizontalOffsetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerticalOffsetProperty<Impl: IPopupStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VerticalOffsetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChildTransitionsProperty<Impl: IPopupStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChildTransitionsProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsLightDismissEnabledProperty<Impl: IPopupStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLightDismissEnabledProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPopupStatics, BASE_OFFSET>(),
            ChildProperty: ChildProperty::<Impl, IMPL_OFFSET>,
            IsOpenProperty: IsOpenProperty::<Impl, IMPL_OFFSET>,
            HorizontalOffsetProperty: HorizontalOffsetProperty::<Impl, IMPL_OFFSET>,
            VerticalOffsetProperty: VerticalOffsetProperty::<Impl, IMPL_OFFSET>,
            ChildTransitionsProperty: ChildTransitionsProperty::<Impl, IMPL_OFFSET>,
            IsLightDismissEnabledProperty: IsLightDismissEnabledProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPopupStatics as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPopupStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPopupStatics2Vtbl {
        unsafe extern "system" fn LightDismissOverlayModeProperty<Impl: IPopupStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LightDismissOverlayModeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPopupStatics2, BASE_OFFSET>(),
            LightDismissOverlayModeProperty: LightDismissOverlayModeProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPopupStatics2 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPopupStatics3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPopupStatics3Vtbl {
        unsafe extern "system" fn ShouldConstrainToRootBoundsProperty<Impl: IPopupStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShouldConstrainToRootBoundsProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPopupStatics3, BASE_OFFSET>(),
            ShouldConstrainToRootBoundsProperty: ShouldConstrainToRootBoundsProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPopupStatics3 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPopupStatics4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPopupStatics4Vtbl {
        unsafe extern "system" fn PlacementTargetProperty<Impl: IPopupStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlacementTargetProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DesiredPlacementProperty<Impl: IPopupStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredPlacementProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPopupStatics4, BASE_OFFSET>(),
            PlacementTargetProperty: PlacementTargetProperty::<Impl, IMPL_OFFSET>,
            DesiredPlacementProperty: DesiredPlacementProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPopupStatics4 as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProgressBarTemplateSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProgressBarTemplateSettingsVtbl {
        unsafe extern "system" fn EllipseDiameter<Impl: IProgressBarTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EllipseDiameter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EllipseOffset<Impl: IProgressBarTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EllipseOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EllipseAnimationWellPosition<Impl: IProgressBarTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EllipseAnimationWellPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EllipseAnimationEndPosition<Impl: IProgressBarTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EllipseAnimationEndPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContainerAnimationStartPosition<Impl: IProgressBarTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContainerAnimationStartPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContainerAnimationEndPosition<Impl: IProgressBarTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContainerAnimationEndPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IndicatorLengthDelta<Impl: IProgressBarTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IndicatorLengthDelta() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IProgressBarTemplateSettings, BASE_OFFSET>(),
            EllipseDiameter: EllipseDiameter::<Impl, IMPL_OFFSET>,
            EllipseOffset: EllipseOffset::<Impl, IMPL_OFFSET>,
            EllipseAnimationWellPosition: EllipseAnimationWellPosition::<Impl, IMPL_OFFSET>,
            EllipseAnimationEndPosition: EllipseAnimationEndPosition::<Impl, IMPL_OFFSET>,
            ContainerAnimationStartPosition: ContainerAnimationStartPosition::<Impl, IMPL_OFFSET>,
            ContainerAnimationEndPosition: ContainerAnimationEndPosition::<Impl, IMPL_OFFSET>,
            IndicatorLengthDelta: IndicatorLengthDelta::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProgressBarTemplateSettings as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProgressRingTemplateSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProgressRingTemplateSettingsVtbl {
        unsafe extern "system" fn EllipseDiameter<Impl: IProgressRingTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EllipseDiameter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EllipseOffset<Impl: IProgressRingTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EllipseOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxSideLength<Impl: IProgressRingTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxSideLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IProgressRingTemplateSettings, BASE_OFFSET>(),
            EllipseDiameter: EllipseDiameter::<Impl, IMPL_OFFSET>,
            EllipseOffset: EllipseOffset::<Impl, IMPL_OFFSET>,
            MaxSideLength: MaxSideLength::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProgressRingTemplateSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRangeBase {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IRangeBase";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IRangeBaseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRangeBaseImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRangeBaseVtbl {
        unsafe extern "system" fn Minimum<Impl: IRangeBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Minimum() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinimum<Impl: IRangeBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMinimum(value).into()
        }
        unsafe extern "system" fn Maximum<Impl: IRangeBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Maximum() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaximum<Impl: IRangeBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaximum(value).into()
        }
        unsafe extern "system" fn SmallChange<Impl: IRangeBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SmallChange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSmallChange<Impl: IRangeBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSmallChange(value).into()
        }
        unsafe extern "system" fn LargeChange<Impl: IRangeBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LargeChange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLargeChange<Impl: IRangeBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLargeChange(value).into()
        }
        unsafe extern "system" fn Value<Impl: IRangeBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: IRangeBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(value).into()
        }
        unsafe extern "system" fn ValueChanged<Impl: IRangeBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ValueChanged(&*(&handler as *const <RangeBaseValueChangedEventHandler as ::windows::core::Abi>::Abi as *const <RangeBaseValueChangedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveValueChanged<Impl: IRangeBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveValueChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRangeBase, BASE_OFFSET>(),
            Minimum: Minimum::<Impl, IMPL_OFFSET>,
            SetMinimum: SetMinimum::<Impl, IMPL_OFFSET>,
            Maximum: Maximum::<Impl, IMPL_OFFSET>,
            SetMaximum: SetMaximum::<Impl, IMPL_OFFSET>,
            SmallChange: SmallChange::<Impl, IMPL_OFFSET>,
            SetSmallChange: SetSmallChange::<Impl, IMPL_OFFSET>,
            LargeChange: LargeChange::<Impl, IMPL_OFFSET>,
            SetLargeChange: SetLargeChange::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
            ValueChanged: ValueChanged::<Impl, IMPL_OFFSET>,
            RemoveValueChanged: RemoveValueChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRangeBase as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRangeBaseFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRangeBaseFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IRangeBaseFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IRangeBaseFactory, BASE_OFFSET>(), CreateInstance: CreateInstance::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRangeBaseFactory as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRangeBaseOverridesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRangeBaseOverridesVtbl {
        unsafe extern "system" fn OnMinimumChanged<Impl: IRangeBaseOverridesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldminimum: f64, newminimum: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnMinimumChanged(oldminimum, newminimum).into()
        }
        unsafe extern "system" fn OnMaximumChanged<Impl: IRangeBaseOverridesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldmaximum: f64, newmaximum: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnMaximumChanged(oldmaximum, newmaximum).into()
        }
        unsafe extern "system" fn OnValueChanged<Impl: IRangeBaseOverridesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldvalue: f64, newvalue: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnValueChanged(oldvalue, newvalue).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRangeBaseOverrides, BASE_OFFSET>(),
            OnMinimumChanged: OnMinimumChanged::<Impl, IMPL_OFFSET>,
            OnMaximumChanged: OnMaximumChanged::<Impl, IMPL_OFFSET>,
            OnValueChanged: OnValueChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRangeBaseOverrides as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRangeBaseStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRangeBaseStaticsVtbl {
        unsafe extern "system" fn MinimumProperty<Impl: IRangeBaseStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MaximumProperty<Impl: IRangeBaseStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SmallChangeProperty<Impl: IRangeBaseStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LargeChangeProperty<Impl: IRangeBaseStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ValueProperty<Impl: IRangeBaseStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRangeBaseStatics, BASE_OFFSET>(),
            MinimumProperty: MinimumProperty::<Impl, IMPL_OFFSET>,
            MaximumProperty: MaximumProperty::<Impl, IMPL_OFFSET>,
            SmallChangeProperty: SmallChangeProperty::<Impl, IMPL_OFFSET>,
            LargeChangeProperty: LargeChangeProperty::<Impl, IMPL_OFFSET>,
            ValueProperty: ValueProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRangeBaseStatics as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRangeBaseValueChangedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRangeBaseValueChangedEventArgsVtbl {
        unsafe extern "system" fn OldValue<Impl: IRangeBaseValueChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OldValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NewValue<Impl: IRangeBaseValueChangedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NewValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRangeBaseValueChangedEventArgs, BASE_OFFSET>(),
            OldValue: OldValue::<Impl, IMPL_OFFSET>,
            NewValue: NewValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRangeBaseValueChangedEventArgs as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRepeatButtonImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRepeatButtonVtbl {
        unsafe extern "system" fn Delay<Impl: IRepeatButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Delay() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDelay<Impl: IRepeatButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDelay(value).into()
        }
        unsafe extern "system" fn Interval<Impl: IRepeatButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Interval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInterval<Impl: IRepeatButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInterval(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRepeatButton, BASE_OFFSET>(),
            Delay: Delay::<Impl, IMPL_OFFSET>,
            SetDelay: SetDelay::<Impl, IMPL_OFFSET>,
            Interval: Interval::<Impl, IMPL_OFFSET>,
            SetInterval: SetInterval::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRepeatButton as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRepeatButtonStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRepeatButtonStaticsVtbl {
        unsafe extern "system" fn DelayProperty<Impl: IRepeatButtonStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DelayProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IntervalProperty<Impl: IRepeatButtonStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IntervalProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRepeatButtonStatics, BASE_OFFSET>(),
            DelayProperty: DelayProperty::<Impl, IMPL_OFFSET>,
            IntervalProperty: IntervalProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRepeatButtonStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IScrollBar {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IScrollBar";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IScrollBarVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScrollBarImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScrollBarVtbl {
        unsafe extern "system" fn Orientation<Impl: IScrollBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Orientation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Orientation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOrientation<Impl: IScrollBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Orientation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOrientation(value).into()
        }
        unsafe extern "system" fn ViewportSize<Impl: IScrollBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ViewportSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetViewportSize<Impl: IScrollBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetViewportSize(value).into()
        }
        unsafe extern "system" fn IndicatorMode<Impl: IScrollBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ScrollingIndicatorMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IndicatorMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIndicatorMode<Impl: IScrollBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ScrollingIndicatorMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIndicatorMode(value).into()
        }
        unsafe extern "system" fn Scroll<Impl: IScrollBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Scroll(&*(&handler as *const <ScrollEventHandler as ::windows::core::Abi>::Abi as *const <ScrollEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveScroll<Impl: IScrollBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveScroll(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IScrollBar, BASE_OFFSET>(),
            Orientation: Orientation::<Impl, IMPL_OFFSET>,
            SetOrientation: SetOrientation::<Impl, IMPL_OFFSET>,
            ViewportSize: ViewportSize::<Impl, IMPL_OFFSET>,
            SetViewportSize: SetViewportSize::<Impl, IMPL_OFFSET>,
            IndicatorMode: IndicatorMode::<Impl, IMPL_OFFSET>,
            SetIndicatorMode: SetIndicatorMode::<Impl, IMPL_OFFSET>,
            Scroll: Scroll::<Impl, IMPL_OFFSET>,
            RemoveScroll: RemoveScroll::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IScrollBar as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScrollBarStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScrollBarStaticsVtbl {
        unsafe extern "system" fn OrientationProperty<Impl: IScrollBarStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ViewportSizeProperty<Impl: IScrollBarStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ViewportSizeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IndicatorModeProperty<Impl: IScrollBarStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IndicatorModeProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IScrollBarStatics, BASE_OFFSET>(),
            OrientationProperty: OrientationProperty::<Impl, IMPL_OFFSET>,
            ViewportSizeProperty: ViewportSizeProperty::<Impl, IMPL_OFFSET>,
            IndicatorModeProperty: IndicatorModeProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IScrollBarStatics as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScrollEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScrollEventArgsVtbl {
        unsafe extern "system" fn NewValue<Impl: IScrollEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NewValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScrollEventType<Impl: IScrollEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ScrollEventType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScrollEventType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IScrollEventArgs, BASE_OFFSET>(),
            NewValue: NewValue::<Impl, IMPL_OFFSET>,
            ScrollEventType: ScrollEventType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IScrollEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
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
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
impl ::windows::core::RuntimeName for IScrollSnapPointsInfo {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IScrollSnapPointsInfo";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
impl IScrollSnapPointsInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScrollSnapPointsInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScrollSnapPointsInfoVtbl {
        unsafe extern "system" fn AreHorizontalSnapPointsRegular<Impl: IScrollSnapPointsInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AreHorizontalSnapPointsRegular() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AreVerticalSnapPointsRegular<Impl: IScrollSnapPointsInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AreVerticalSnapPointsRegular() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HorizontalSnapPointsChanged<Impl: IScrollSnapPointsInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HorizontalSnapPointsChanged(&*(&handler as *const <super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveHorizontalSnapPointsChanged<Impl: IScrollSnapPointsInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveHorizontalSnapPointsChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn VerticalSnapPointsChanged<Impl: IScrollSnapPointsInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VerticalSnapPointsChanged(&*(&handler as *const <super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveVerticalSnapPointsChanged<Impl: IScrollSnapPointsInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveVerticalSnapPointsChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetIrregularSnapPoints<Impl: IScrollSnapPointsInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, orientation: super::Orientation, alignment: SnapPointsAlignment, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIrregularSnapPoints(orientation, alignment) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRegularSnapPoints<Impl: IScrollSnapPointsInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, orientation: super::Orientation, alignment: SnapPointsAlignment, offset: *mut f32, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRegularSnapPoints(orientation, alignment, ::core::mem::transmute_copy(&offset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IScrollSnapPointsInfo, BASE_OFFSET>(),
            AreHorizontalSnapPointsRegular: AreHorizontalSnapPointsRegular::<Impl, IMPL_OFFSET>,
            AreVerticalSnapPointsRegular: AreVerticalSnapPointsRegular::<Impl, IMPL_OFFSET>,
            HorizontalSnapPointsChanged: HorizontalSnapPointsChanged::<Impl, IMPL_OFFSET>,
            RemoveHorizontalSnapPointsChanged: RemoveHorizontalSnapPointsChanged::<Impl, IMPL_OFFSET>,
            VerticalSnapPointsChanged: VerticalSnapPointsChanged::<Impl, IMPL_OFFSET>,
            RemoveVerticalSnapPointsChanged: RemoveVerticalSnapPointsChanged::<Impl, IMPL_OFFSET>,
            GetIrregularSnapPoints: GetIrregularSnapPoints::<Impl, IMPL_OFFSET>,
            GetRegularSnapPoints: GetRegularSnapPoints::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IScrollSnapPointsInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISelector {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ISelector";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISelectorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISelectorVtbl {
        unsafe extern "system" fn SelectedIndex<Impl: ISelectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedIndex<Impl: ISelectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectedIndex(value).into()
        }
        unsafe extern "system" fn SelectedItem<Impl: ISelectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedItem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedItem<Impl: ISelectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectedItem(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedValue<Impl: ISelectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedValue<Impl: ISelectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectedValue(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedValuePath<Impl: ISelectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedValuePath() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelectedValuePath<Impl: ISelectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectedValuePath(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsSynchronizedWithCurrentItem<Impl: ISelectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSynchronizedWithCurrentItem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsSynchronizedWithCurrentItem<Impl: ISelectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsSynchronizedWithCurrentItem(&*(&value as *const <super::super::super::super::Foundation::IReference<bool> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::IReference<bool> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectionChanged<Impl: ISelectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectionChanged(&*(&handler as *const <super::SelectionChangedEventHandler as ::windows::core::Abi>::Abi as *const <super::SelectionChangedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSelectionChanged<Impl: ISelectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSelectionChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISelector, BASE_OFFSET>(),
            SelectedIndex: SelectedIndex::<Impl, IMPL_OFFSET>,
            SetSelectedIndex: SetSelectedIndex::<Impl, IMPL_OFFSET>,
            SelectedItem: SelectedItem::<Impl, IMPL_OFFSET>,
            SetSelectedItem: SetSelectedItem::<Impl, IMPL_OFFSET>,
            SelectedValue: SelectedValue::<Impl, IMPL_OFFSET>,
            SetSelectedValue: SetSelectedValue::<Impl, IMPL_OFFSET>,
            SelectedValuePath: SelectedValuePath::<Impl, IMPL_OFFSET>,
            SetSelectedValuePath: SetSelectedValuePath::<Impl, IMPL_OFFSET>,
            IsSynchronizedWithCurrentItem: IsSynchronizedWithCurrentItem::<Impl, IMPL_OFFSET>,
            SetIsSynchronizedWithCurrentItem: SetIsSynchronizedWithCurrentItem::<Impl, IMPL_OFFSET>,
            SelectionChanged: SelectionChanged::<Impl, IMPL_OFFSET>,
            RemoveSelectionChanged: RemoveSelectionChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISelector as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectorFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISelectorFactoryVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISelectorFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISelectorFactory as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectorItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISelectorItemVtbl {
        unsafe extern "system" fn IsSelected<Impl: ISelectorItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSelected() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsSelected<Impl: ISelectorItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsSelected(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISelectorItem, BASE_OFFSET>(),
            IsSelected: IsSelected::<Impl, IMPL_OFFSET>,
            SetIsSelected: SetIsSelected::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISelectorItem as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectorItemFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISelectorItemFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: ISelectorItemFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISelectorItemFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISelectorItemFactory as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectorItemStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISelectorItemStaticsVtbl {
        unsafe extern "system" fn IsSelectedProperty<Impl: ISelectorItemStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISelectorItemStatics, BASE_OFFSET>(),
            IsSelectedProperty: IsSelectedProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISelectorItemStatics as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectorStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISelectorStaticsVtbl {
        unsafe extern "system" fn SelectedIndexProperty<Impl: ISelectorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedIndexProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedItemProperty<Impl: ISelectorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedItemProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedValueProperty<Impl: ISelectorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedValueProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedValuePathProperty<Impl: ISelectorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedValuePathProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSynchronizedWithCurrentItemProperty<Impl: ISelectorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSynchronizedWithCurrentItemProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsSelectionActive<Impl: ISelectorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsSelectionActive(&*(&element as *const <super::super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISelectorStatics, BASE_OFFSET>(),
            SelectedIndexProperty: SelectedIndexProperty::<Impl, IMPL_OFFSET>,
            SelectedItemProperty: SelectedItemProperty::<Impl, IMPL_OFFSET>,
            SelectedValueProperty: SelectedValueProperty::<Impl, IMPL_OFFSET>,
            SelectedValuePathProperty: SelectedValuePathProperty::<Impl, IMPL_OFFSET>,
            IsSynchronizedWithCurrentItemProperty: IsSynchronizedWithCurrentItemProperty::<Impl, IMPL_OFFSET>,
            GetIsSelectionActive: GetIsSelectionActive::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISelectorStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media", feature = "UI_Xaml_Media_Animation", feature = "implement_exclusive"))]
pub trait ISettingsFlyoutTemplateSettingsImpl: Sized {
    fn HeaderBackground(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn HeaderForeground(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn BorderBrush(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn BorderThickness(&self) -> ::windows::core::Result<super::super::Thickness>;
    fn IconSource(&self) -> ::windows::core::Result<super::super::Media::ImageSource>;
    fn ContentTransitions(&self) -> ::windows::core::Result<super::super::Media::Animation::TransitionCollection>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media", feature = "UI_Xaml_Media_Animation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISettingsFlyoutTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ISettingsFlyoutTemplateSettings";
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media", feature = "UI_Xaml_Media_Animation", feature = "implement_exclusive"))]
impl ISettingsFlyoutTemplateSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISettingsFlyoutTemplateSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISettingsFlyoutTemplateSettingsVtbl {
        unsafe extern "system" fn HeaderBackground<Impl: ISettingsFlyoutTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HeaderBackground() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HeaderForeground<Impl: ISettingsFlyoutTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HeaderForeground() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BorderBrush<Impl: ISettingsFlyoutTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BorderBrush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BorderThickness<Impl: ISettingsFlyoutTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BorderThickness() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IconSource<Impl: ISettingsFlyoutTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IconSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentTransitions<Impl: ISettingsFlyoutTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentTransitions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISettingsFlyoutTemplateSettings, BASE_OFFSET>(),
            HeaderBackground: HeaderBackground::<Impl, IMPL_OFFSET>,
            HeaderForeground: HeaderForeground::<Impl, IMPL_OFFSET>,
            BorderBrush: BorderBrush::<Impl, IMPL_OFFSET>,
            BorderThickness: BorderThickness::<Impl, IMPL_OFFSET>,
            IconSource: IconSource::<Impl, IMPL_OFFSET>,
            ContentTransitions: ContentTransitions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISettingsFlyoutTemplateSettings as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISplitViewTemplateSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISplitViewTemplateSettingsVtbl {
        unsafe extern "system" fn OpenPaneLength<Impl: ISplitViewTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenPaneLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NegativeOpenPaneLength<Impl: ISplitViewTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NegativeOpenPaneLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenPaneLengthMinusCompactLength<Impl: ISplitViewTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenPaneLengthMinusCompactLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NegativeOpenPaneLengthMinusCompactLength<Impl: ISplitViewTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NegativeOpenPaneLengthMinusCompactLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenPaneGridLength<Impl: ISplitViewTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::GridLength) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenPaneGridLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompactPaneGridLength<Impl: ISplitViewTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::GridLength) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompactPaneGridLength() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISplitViewTemplateSettings, BASE_OFFSET>(),
            OpenPaneLength: OpenPaneLength::<Impl, IMPL_OFFSET>,
            NegativeOpenPaneLength: NegativeOpenPaneLength::<Impl, IMPL_OFFSET>,
            OpenPaneLengthMinusCompactLength: OpenPaneLengthMinusCompactLength::<Impl, IMPL_OFFSET>,
            NegativeOpenPaneLengthMinusCompactLength: NegativeOpenPaneLengthMinusCompactLength::<Impl, IMPL_OFFSET>,
            OpenPaneGridLength: OpenPaneGridLength::<Impl, IMPL_OFFSET>,
            CompactPaneGridLength: CompactPaneGridLength::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISplitViewTemplateSettings as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IThumb {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IThumb";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IThumbVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IThumbImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IThumbVtbl {
        unsafe extern "system" fn IsDragging<Impl: IThumbImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDragging() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DragStarted<Impl: IThumbImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DragStarted(&*(&handler as *const <DragStartedEventHandler as ::windows::core::Abi>::Abi as *const <DragStartedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDragStarted<Impl: IThumbImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDragStarted(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DragDelta<Impl: IThumbImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DragDelta(&*(&handler as *const <DragDeltaEventHandler as ::windows::core::Abi>::Abi as *const <DragDeltaEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDragDelta<Impl: IThumbImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDragDelta(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DragCompleted<Impl: IThumbImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DragCompleted(&*(&handler as *const <DragCompletedEventHandler as ::windows::core::Abi>::Abi as *const <DragCompletedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDragCompleted<Impl: IThumbImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDragCompleted(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CancelDrag<Impl: IThumbImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CancelDrag().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IThumb, BASE_OFFSET>(),
            IsDragging: IsDragging::<Impl, IMPL_OFFSET>,
            DragStarted: DragStarted::<Impl, IMPL_OFFSET>,
            RemoveDragStarted: RemoveDragStarted::<Impl, IMPL_OFFSET>,
            DragDelta: DragDelta::<Impl, IMPL_OFFSET>,
            RemoveDragDelta: RemoveDragDelta::<Impl, IMPL_OFFSET>,
            DragCompleted: DragCompleted::<Impl, IMPL_OFFSET>,
            RemoveDragCompleted: RemoveDragCompleted::<Impl, IMPL_OFFSET>,
            CancelDrag: CancelDrag::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IThumb as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IThumbStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IThumbStaticsVtbl {
        unsafe extern "system" fn IsDraggingProperty<Impl: IThumbStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDraggingProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IThumbStatics, BASE_OFFSET>(),
            IsDraggingProperty: IsDraggingProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IThumbStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
pub trait ITickBarImpl: Sized {
    fn Fill(&self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetFill(&self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITickBar {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ITickBar";
}
#[cfg(all(feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
impl ITickBarVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITickBarImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITickBarVtbl {
        unsafe extern "system" fn Fill<Impl: ITickBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Fill() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFill<Impl: ITickBarImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFill(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITickBar, BASE_OFFSET>(),
            Fill: Fill::<Impl, IMPL_OFFSET>,
            SetFill: SetFill::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITickBar as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITickBarStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITickBarStaticsVtbl {
        unsafe extern "system" fn FillProperty<Impl: ITickBarStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FillProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ITickBarStatics, BASE_OFFSET>(), FillProperty: FillProperty::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITickBarStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
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
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IToggleButton {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IToggleButton";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IToggleButtonVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToggleButtonImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToggleButtonVtbl {
        unsafe extern "system" fn IsChecked<Impl: IToggleButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsChecked() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsChecked<Impl: IToggleButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsChecked(&*(&value as *const <super::super::super::super::Foundation::IReference<bool> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::IReference<bool> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsThreeState<Impl: IToggleButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsThreeState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsThreeState<Impl: IToggleButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsThreeState(value).into()
        }
        unsafe extern "system" fn Checked<Impl: IToggleButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Checked(&*(&handler as *const <super::super::RoutedEventHandler as ::windows::core::Abi>::Abi as *const <super::super::RoutedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveChecked<Impl: IToggleButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveChecked(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Unchecked<Impl: IToggleButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unchecked(&*(&handler as *const <super::super::RoutedEventHandler as ::windows::core::Abi>::Abi as *const <super::super::RoutedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUnchecked<Impl: IToggleButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveUnchecked(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Indeterminate<Impl: IToggleButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Indeterminate(&*(&handler as *const <super::super::RoutedEventHandler as ::windows::core::Abi>::Abi as *const <super::super::RoutedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveIndeterminate<Impl: IToggleButtonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveIndeterminate(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IToggleButton, BASE_OFFSET>(),
            IsChecked: IsChecked::<Impl, IMPL_OFFSET>,
            SetIsChecked: SetIsChecked::<Impl, IMPL_OFFSET>,
            IsThreeState: IsThreeState::<Impl, IMPL_OFFSET>,
            SetIsThreeState: SetIsThreeState::<Impl, IMPL_OFFSET>,
            Checked: Checked::<Impl, IMPL_OFFSET>,
            RemoveChecked: RemoveChecked::<Impl, IMPL_OFFSET>,
            Unchecked: Unchecked::<Impl, IMPL_OFFSET>,
            RemoveUnchecked: RemoveUnchecked::<Impl, IMPL_OFFSET>,
            Indeterminate: Indeterminate::<Impl, IMPL_OFFSET>,
            RemoveIndeterminate: RemoveIndeterminate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToggleButton as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToggleButtonFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToggleButtonFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IToggleButtonFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IToggleButtonFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToggleButtonFactory as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToggleButtonOverridesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToggleButtonOverridesVtbl {
        unsafe extern "system" fn OnToggle<Impl: IToggleButtonOverridesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnToggle().into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IToggleButtonOverrides, BASE_OFFSET>(), OnToggle: OnToggle::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToggleButtonOverrides as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToggleButtonStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToggleButtonStaticsVtbl {
        unsafe extern "system" fn IsCheckedProperty<Impl: IToggleButtonStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCheckedProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsThreeStateProperty<Impl: IToggleButtonStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsThreeStateProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IToggleButtonStatics, BASE_OFFSET>(),
            IsCheckedProperty: IsCheckedProperty::<Impl, IMPL_OFFSET>,
            IsThreeStateProperty: IsThreeStateProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToggleButtonStatics as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToggleSwitchTemplateSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToggleSwitchTemplateSettingsVtbl {
        unsafe extern "system" fn KnobCurrentToOnOffset<Impl: IToggleSwitchTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KnobCurrentToOnOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KnobCurrentToOffOffset<Impl: IToggleSwitchTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KnobCurrentToOffOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KnobOnToOffOffset<Impl: IToggleSwitchTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KnobOnToOffOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KnobOffToOnOffset<Impl: IToggleSwitchTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KnobOffToOnOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurtainCurrentToOnOffset<Impl: IToggleSwitchTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurtainCurrentToOnOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurtainCurrentToOffOffset<Impl: IToggleSwitchTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurtainCurrentToOffOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurtainOnToOffOffset<Impl: IToggleSwitchTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurtainOnToOffOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurtainOffToOnOffset<Impl: IToggleSwitchTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurtainOffToOnOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IToggleSwitchTemplateSettings, BASE_OFFSET>(),
            KnobCurrentToOnOffset: KnobCurrentToOnOffset::<Impl, IMPL_OFFSET>,
            KnobCurrentToOffOffset: KnobCurrentToOffOffset::<Impl, IMPL_OFFSET>,
            KnobOnToOffOffset: KnobOnToOffOffset::<Impl, IMPL_OFFSET>,
            KnobOffToOnOffset: KnobOffToOnOffset::<Impl, IMPL_OFFSET>,
            CurtainCurrentToOnOffset: CurtainCurrentToOnOffset::<Impl, IMPL_OFFSET>,
            CurtainCurrentToOffOffset: CurtainCurrentToOffOffset::<Impl, IMPL_OFFSET>,
            CurtainOnToOffOffset: CurtainOnToOffOffset::<Impl, IMPL_OFFSET>,
            CurtainOffToOnOffset: CurtainOffToOnOffset::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToggleSwitchTemplateSettings as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToolTipTemplateSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToolTipTemplateSettingsVtbl {
        unsafe extern "system" fn FromHorizontalOffset<Impl: IToolTipTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromHorizontalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FromVerticalOffset<Impl: IToolTipTemplateSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FromVerticalOffset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IToolTipTemplateSettings, BASE_OFFSET>(),
            FromHorizontalOffset: FromHorizontalOffset::<Impl, IMPL_OFFSET>,
            FromVerticalOffset: FromVerticalOffset::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IToolTipTemplateSettings as ::windows::core::Interface>::IID
    }
}

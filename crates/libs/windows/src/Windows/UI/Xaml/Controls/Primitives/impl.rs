#[cfg(feature = "implement_exclusive")]
pub trait IAppBarButtonTemplateSettings_Impl: Sized {
    fn KeyboardAcceleratorTextMinWidth(&mut self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBarButtonTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IAppBarButtonTemplateSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBarButtonTemplateSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBarButtonTemplateSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBarButtonTemplateSettings_Vtbl {
        unsafe extern "system" fn KeyboardAcceleratorTextMinWidth<Impl: IAppBarButtonTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
pub trait IAppBarTemplateSettings_Impl: Sized {
    fn ClipRect(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect>;
    fn CompactVerticalDelta(&mut self) -> ::windows::core::Result<f64>;
    fn CompactRootMargin(&mut self) -> ::windows::core::Result<super::super::Thickness>;
    fn MinimalVerticalDelta(&mut self) -> ::windows::core::Result<f64>;
    fn MinimalRootMargin(&mut self) -> ::windows::core::Result<super::super::Thickness>;
    fn HiddenVerticalDelta(&mut self) -> ::windows::core::Result<f64>;
    fn HiddenRootMargin(&mut self) -> ::windows::core::Result<super::super::Thickness>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppBarTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IAppBarTemplateSettings";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppBarTemplateSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBarTemplateSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBarTemplateSettings_Vtbl {
        unsafe extern "system" fn ClipRect<Impl: IAppBarTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CompactVerticalDelta<Impl: IAppBarTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CompactRootMargin<Impl: IAppBarTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MinimalVerticalDelta<Impl: IAppBarTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MinimalRootMargin<Impl: IAppBarTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HiddenVerticalDelta<Impl: IAppBarTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HiddenRootMargin<Impl: IAppBarTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT {
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
pub trait IAppBarTemplateSettings2_Impl: Sized {
    fn NegativeCompactVerticalDelta(&mut self) -> ::windows::core::Result<f64>;
    fn NegativeMinimalVerticalDelta(&mut self) -> ::windows::core::Result<f64>;
    fn NegativeHiddenVerticalDelta(&mut self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBarTemplateSettings2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IAppBarTemplateSettings2";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBarTemplateSettings2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBarTemplateSettings2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBarTemplateSettings2_Vtbl {
        unsafe extern "system" fn NegativeCompactVerticalDelta<Impl: IAppBarTemplateSettings2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NegativeMinimalVerticalDelta<Impl: IAppBarTemplateSettings2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NegativeHiddenVerticalDelta<Impl: IAppBarTemplateSettings2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
pub trait IAppBarToggleButtonTemplateSettings_Impl: Sized {
    fn KeyboardAcceleratorTextMinWidth(&mut self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppBarToggleButtonTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IAppBarToggleButtonTemplateSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IAppBarToggleButtonTemplateSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppBarToggleButtonTemplateSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppBarToggleButtonTemplateSettings_Vtbl {
        unsafe extern "system" fn KeyboardAcceleratorTextMinWidth<Impl: IAppBarToggleButtonTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
pub trait IButtonBase_Impl: Sized {
    fn ClickMode(&mut self) -> ::windows::core::Result<super::ClickMode>;
    fn SetClickMode(&mut self, value: super::ClickMode) -> ::windows::core::Result<()>;
    fn IsPointerOver(&mut self) -> ::windows::core::Result<bool>;
    fn IsPressed(&mut self) -> ::windows::core::Result<bool>;
    fn Command(&mut self) -> ::windows::core::Result<super::super::Input::ICommand>;
    fn SetCommand(&mut self, value: &::core::option::Option<super::super::Input::ICommand>) -> ::windows::core::Result<()>;
    fn CommandParameter(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetCommandParameter(&mut self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn Click(&mut self, handler: &::core::option::Option<super::super::RoutedEventHandler>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveClick(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "UI_Xaml_Input", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IButtonBase {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IButtonBase";
}
#[cfg(all(feature = "Foundation", feature = "UI_Xaml_Input", feature = "implement_exclusive"))]
impl IButtonBase_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IButtonBase_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IButtonBase_Vtbl {
        unsafe extern "system" fn ClickMode<Impl: IButtonBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::ClickMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetClickMode<Impl: IButtonBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::ClickMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClickMode(value).into()
        }
        unsafe extern "system" fn IsPointerOver<Impl: IButtonBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsPressed<Impl: IButtonBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Command<Impl: IButtonBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCommand<Impl: IButtonBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCommand(&*(&value as *const <super::super::Input::ICommand as ::windows::core::Abi>::Abi as *const <super::super::Input::ICommand as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CommandParameter<Impl: IButtonBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCommandParameter<Impl: IButtonBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCommandParameter(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Click<Impl: IButtonBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveClick<Impl: IButtonBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
pub trait IButtonBaseFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ButtonBase>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IButtonBaseFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IButtonBaseFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IButtonBaseFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IButtonBaseFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IButtonBaseFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IButtonBaseFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IButtonBaseStatics_Impl: Sized {
    fn ClickModeProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn IsPointerOverProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn IsPressedProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CommandProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CommandParameterProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IButtonBaseStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IButtonBaseStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IButtonBaseStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IButtonBaseStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IButtonBaseStatics_Vtbl {
        unsafe extern "system" fn ClickModeProperty<Impl: IButtonBaseStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsPointerOverProperty<Impl: IButtonBaseStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsPressedProperty<Impl: IButtonBaseStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CommandProperty<Impl: IButtonBaseStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CommandParameterProperty<Impl: IButtonBaseStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ICalendarPanel_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICalendarPanel {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ICalendarPanel";
}
#[cfg(feature = "implement_exclusive")]
impl ICalendarPanel_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICalendarPanel_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICalendarPanel_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ICalendarPanel, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICalendarPanel as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ICalendarViewTemplateSettings_Impl: Sized {
    fn MinViewWidth(&mut self) -> ::windows::core::Result<f64>;
    fn HeaderText(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WeekDay1(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WeekDay2(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WeekDay3(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WeekDay4(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WeekDay5(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WeekDay6(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WeekDay7(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HasMoreContentAfter(&mut self) -> ::windows::core::Result<bool>;
    fn HasMoreContentBefore(&mut self) -> ::windows::core::Result<bool>;
    fn HasMoreViews(&mut self) -> ::windows::core::Result<bool>;
    fn ClipRect(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect>;
    fn CenterX(&mut self) -> ::windows::core::Result<f64>;
    fn CenterY(&mut self) -> ::windows::core::Result<f64>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICalendarViewTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ICalendarViewTemplateSettings";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICalendarViewTemplateSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICalendarViewTemplateSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICalendarViewTemplateSettings_Vtbl {
        unsafe extern "system" fn MinViewWidth<Impl: ICalendarViewTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HeaderText<Impl: ICalendarViewTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn WeekDay1<Impl: ICalendarViewTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn WeekDay2<Impl: ICalendarViewTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn WeekDay3<Impl: ICalendarViewTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn WeekDay4<Impl: ICalendarViewTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn WeekDay5<Impl: ICalendarViewTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn WeekDay6<Impl: ICalendarViewTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn WeekDay7<Impl: ICalendarViewTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HasMoreContentAfter<Impl: ICalendarViewTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HasMoreContentBefore<Impl: ICalendarViewTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HasMoreViews<Impl: ICalendarViewTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ClipRect<Impl: ICalendarViewTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CenterX<Impl: ICalendarViewTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CenterY<Impl: ICalendarViewTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
pub trait ICarouselPanel_Impl: Sized {
    fn CanVerticallyScroll(&mut self) -> ::windows::core::Result<bool>;
    fn SetCanVerticallyScroll(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn CanHorizontallyScroll(&mut self) -> ::windows::core::Result<bool>;
    fn SetCanHorizontallyScroll(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn ExtentWidth(&mut self) -> ::windows::core::Result<f64>;
    fn ExtentHeight(&mut self) -> ::windows::core::Result<f64>;
    fn ViewportWidth(&mut self) -> ::windows::core::Result<f64>;
    fn ViewportHeight(&mut self) -> ::windows::core::Result<f64>;
    fn HorizontalOffset(&mut self) -> ::windows::core::Result<f64>;
    fn VerticalOffset(&mut self) -> ::windows::core::Result<f64>;
    fn ScrollOwner(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetScrollOwner(&mut self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn LineUp(&mut self) -> ::windows::core::Result<()>;
    fn LineDown(&mut self) -> ::windows::core::Result<()>;
    fn LineLeft(&mut self) -> ::windows::core::Result<()>;
    fn LineRight(&mut self) -> ::windows::core::Result<()>;
    fn PageUp(&mut self) -> ::windows::core::Result<()>;
    fn PageDown(&mut self) -> ::windows::core::Result<()>;
    fn PageLeft(&mut self) -> ::windows::core::Result<()>;
    fn PageRight(&mut self) -> ::windows::core::Result<()>;
    fn MouseWheelUp(&mut self) -> ::windows::core::Result<()>;
    fn MouseWheelDown(&mut self) -> ::windows::core::Result<()>;
    fn MouseWheelLeft(&mut self) -> ::windows::core::Result<()>;
    fn MouseWheelRight(&mut self) -> ::windows::core::Result<()>;
    fn SetHorizontalOffset(&mut self, offset: f64) -> ::windows::core::Result<()>;
    fn SetVerticalOffset(&mut self, offset: f64) -> ::windows::core::Result<()>;
    fn MakeVisible(&mut self, visual: &::core::option::Option<super::super::UIElement>, rectangle: &super::super::super::super::Foundation::Rect) -> ::windows::core::Result<super::super::super::super::Foundation::Rect>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICarouselPanel {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ICarouselPanel";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICarouselPanel_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICarouselPanel_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICarouselPanel_Vtbl {
        unsafe extern "system" fn CanVerticallyScroll<Impl: ICarouselPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCanVerticallyScroll<Impl: ICarouselPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCanVerticallyScroll(value).into()
        }
        unsafe extern "system" fn CanHorizontallyScroll<Impl: ICarouselPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCanHorizontallyScroll<Impl: ICarouselPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCanHorizontallyScroll(value).into()
        }
        unsafe extern "system" fn ExtentWidth<Impl: ICarouselPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExtentHeight<Impl: ICarouselPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ViewportWidth<Impl: ICarouselPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ViewportHeight<Impl: ICarouselPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HorizontalOffset<Impl: ICarouselPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn VerticalOffset<Impl: ICarouselPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ScrollOwner<Impl: ICarouselPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetScrollOwner<Impl: ICarouselPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScrollOwner(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LineUp<Impl: ICarouselPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LineUp().into()
        }
        unsafe extern "system" fn LineDown<Impl: ICarouselPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LineDown().into()
        }
        unsafe extern "system" fn LineLeft<Impl: ICarouselPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LineLeft().into()
        }
        unsafe extern "system" fn LineRight<Impl: ICarouselPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LineRight().into()
        }
        unsafe extern "system" fn PageUp<Impl: ICarouselPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PageUp().into()
        }
        unsafe extern "system" fn PageDown<Impl: ICarouselPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PageDown().into()
        }
        unsafe extern "system" fn PageLeft<Impl: ICarouselPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PageLeft().into()
        }
        unsafe extern "system" fn PageRight<Impl: ICarouselPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PageRight().into()
        }
        unsafe extern "system" fn MouseWheelUp<Impl: ICarouselPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MouseWheelUp().into()
        }
        unsafe extern "system" fn MouseWheelDown<Impl: ICarouselPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MouseWheelDown().into()
        }
        unsafe extern "system" fn MouseWheelLeft<Impl: ICarouselPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MouseWheelLeft().into()
        }
        unsafe extern "system" fn MouseWheelRight<Impl: ICarouselPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MouseWheelRight().into()
        }
        unsafe extern "system" fn SetHorizontalOffset<Impl: ICarouselPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHorizontalOffset(offset).into()
        }
        unsafe extern "system" fn SetVerticalOffset<Impl: ICarouselPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVerticalOffset(offset).into()
        }
        unsafe extern "system" fn MakeVisible<Impl: ICarouselPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visual: ::windows::core::RawPtr, rectangle: super::super::super::super::Foundation::Rect, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
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
pub trait ICarouselPanelFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<CarouselPanel>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICarouselPanelFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ICarouselPanelFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICarouselPanelFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICarouselPanelFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICarouselPanelFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: ICarouselPanelFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IColorPickerSlider_Impl: Sized {
    fn ColorChannel(&mut self) -> ::windows::core::Result<super::ColorPickerHsvChannel>;
    fn SetColorChannel(&mut self, value: super::ColorPickerHsvChannel) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IColorPickerSlider {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IColorPickerSlider";
}
#[cfg(feature = "implement_exclusive")]
impl IColorPickerSlider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColorPickerSlider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IColorPickerSlider_Vtbl {
        unsafe extern "system" fn ColorChannel<Impl: IColorPickerSlider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::ColorPickerHsvChannel) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetColorChannel<Impl: IColorPickerSlider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::ColorPickerHsvChannel) -> ::windows::core::HRESULT {
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
pub trait IColorPickerSliderFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ColorPickerSlider>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IColorPickerSliderFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IColorPickerSliderFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IColorPickerSliderFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColorPickerSliderFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IColorPickerSliderFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IColorPickerSliderFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IColorPickerSliderStatics_Impl: Sized {
    fn ColorChannelProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IColorPickerSliderStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IColorPickerSliderStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IColorPickerSliderStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColorPickerSliderStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IColorPickerSliderStatics_Vtbl {
        unsafe extern "system" fn ColorChannelProperty<Impl: IColorPickerSliderStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IColorSpectrum_Impl: Sized {
    fn Color(&mut self) -> ::windows::core::Result<super::super::super::Color>;
    fn SetColor(&mut self, value: &super::super::super::Color) -> ::windows::core::Result<()>;
    fn HsvColor(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Numerics::Vector4>;
    fn SetHsvColor(&mut self, value: &super::super::super::super::Foundation::Numerics::Vector4) -> ::windows::core::Result<()>;
    fn MinHue(&mut self) -> ::windows::core::Result<i32>;
    fn SetMinHue(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn MaxHue(&mut self) -> ::windows::core::Result<i32>;
    fn SetMaxHue(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn MinSaturation(&mut self) -> ::windows::core::Result<i32>;
    fn SetMinSaturation(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn MaxSaturation(&mut self) -> ::windows::core::Result<i32>;
    fn SetMaxSaturation(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn MinValue(&mut self) -> ::windows::core::Result<i32>;
    fn SetMinValue(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn MaxValue(&mut self) -> ::windows::core::Result<i32>;
    fn SetMaxValue(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn Shape(&mut self) -> ::windows::core::Result<super::ColorSpectrumShape>;
    fn SetShape(&mut self, value: super::ColorSpectrumShape) -> ::windows::core::Result<()>;
    fn Components(&mut self) -> ::windows::core::Result<super::ColorSpectrumComponents>;
    fn SetComponents(&mut self, value: super::ColorSpectrumComponents) -> ::windows::core::Result<()>;
    fn ColorChanged(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<ColorSpectrum, super::ColorChangedEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveColorChanged(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IColorSpectrum {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IColorSpectrum";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Numerics", feature = "implement_exclusive"))]
impl IColorSpectrum_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColorSpectrum_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IColorSpectrum_Vtbl {
        unsafe extern "system" fn Color<Impl: IColorSpectrum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Color) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetColor<Impl: IColorSpectrum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Color) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColor(&*(&value as *const <super::super::super::Color as ::windows::core::Abi>::Abi as *const <super::super::super::Color as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn HsvColor<Impl: IColorSpectrum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Numerics::Vector4) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHsvColor<Impl: IColorSpectrum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::super::Foundation::Numerics::Vector4) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHsvColor(&*(&value as *const <super::super::super::super::Foundation::Numerics::Vector4 as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Numerics::Vector4 as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MinHue<Impl: IColorSpectrum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMinHue<Impl: IColorSpectrum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMinHue(value).into()
        }
        unsafe extern "system" fn MaxHue<Impl: IColorSpectrum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMaxHue<Impl: IColorSpectrum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxHue(value).into()
        }
        unsafe extern "system" fn MinSaturation<Impl: IColorSpectrum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMinSaturation<Impl: IColorSpectrum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMinSaturation(value).into()
        }
        unsafe extern "system" fn MaxSaturation<Impl: IColorSpectrum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMaxSaturation<Impl: IColorSpectrum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxSaturation(value).into()
        }
        unsafe extern "system" fn MinValue<Impl: IColorSpectrum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMinValue<Impl: IColorSpectrum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMinValue(value).into()
        }
        unsafe extern "system" fn MaxValue<Impl: IColorSpectrum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMaxValue<Impl: IColorSpectrum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxValue(value).into()
        }
        unsafe extern "system" fn Shape<Impl: IColorSpectrum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::ColorSpectrumShape) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetShape<Impl: IColorSpectrum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::ColorSpectrumShape) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShape(value).into()
        }
        unsafe extern "system" fn Components<Impl: IColorSpectrum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::ColorSpectrumComponents) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetComponents<Impl: IColorSpectrum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::ColorSpectrumComponents) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetComponents(value).into()
        }
        unsafe extern "system" fn ColorChanged<Impl: IColorSpectrum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveColorChanged<Impl: IColorSpectrum_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
pub trait IColorSpectrumFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ColorSpectrum>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IColorSpectrumFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IColorSpectrumFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IColorSpectrumFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColorSpectrumFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IColorSpectrumFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IColorSpectrumFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IColorSpectrumStatics_Impl: Sized {
    fn ColorProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn HsvColorProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn MinHueProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn MaxHueProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn MinSaturationProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn MaxSaturationProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn MinValueProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn MaxValueProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ShapeProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ComponentsProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IColorSpectrumStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IColorSpectrumStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IColorSpectrumStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IColorSpectrumStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IColorSpectrumStatics_Vtbl {
        unsafe extern "system" fn ColorProperty<Impl: IColorSpectrumStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HsvColorProperty<Impl: IColorSpectrumStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MinHueProperty<Impl: IColorSpectrumStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MaxHueProperty<Impl: IColorSpectrumStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MinSaturationProperty<Impl: IColorSpectrumStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MaxSaturationProperty<Impl: IColorSpectrumStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MinValueProperty<Impl: IColorSpectrumStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MaxValueProperty<Impl: IColorSpectrumStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ShapeProperty<Impl: IColorSpectrumStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ComponentsProperty<Impl: IColorSpectrumStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IComboBoxTemplateSettings_Impl: Sized {
    fn DropDownOpenedHeight(&mut self) -> ::windows::core::Result<f64>;
    fn DropDownClosedHeight(&mut self) -> ::windows::core::Result<f64>;
    fn DropDownOffset(&mut self) -> ::windows::core::Result<f64>;
    fn SelectedItemDirection(&mut self) -> ::windows::core::Result<AnimationDirection>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IComboBoxTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IComboBoxTemplateSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IComboBoxTemplateSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComboBoxTemplateSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComboBoxTemplateSettings_Vtbl {
        unsafe extern "system" fn DropDownOpenedHeight<Impl: IComboBoxTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DropDownClosedHeight<Impl: IComboBoxTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DropDownOffset<Impl: IComboBoxTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SelectedItemDirection<Impl: IComboBoxTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AnimationDirection) -> ::windows::core::HRESULT {
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
pub trait IComboBoxTemplateSettings2_Impl: Sized {
    fn DropDownContentMinWidth(&mut self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IComboBoxTemplateSettings2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IComboBoxTemplateSettings2";
}
#[cfg(feature = "implement_exclusive")]
impl IComboBoxTemplateSettings2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComboBoxTemplateSettings2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComboBoxTemplateSettings2_Vtbl {
        unsafe extern "system" fn DropDownContentMinWidth<Impl: IComboBoxTemplateSettings2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
pub trait ICommandBarFlyoutCommandBar_Impl: Sized {
    fn FlyoutTemplateSettings(&mut self) -> ::windows::core::Result<CommandBarFlyoutCommandBarTemplateSettings>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICommandBarFlyoutCommandBar {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ICommandBarFlyoutCommandBar";
}
#[cfg(feature = "implement_exclusive")]
impl ICommandBarFlyoutCommandBar_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommandBarFlyoutCommandBar_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICommandBarFlyoutCommandBar_Vtbl {
        unsafe extern "system" fn FlyoutTemplateSettings<Impl: ICommandBarFlyoutCommandBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ICommandBarFlyoutCommandBarFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<CommandBarFlyoutCommandBar>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICommandBarFlyoutCommandBarFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ICommandBarFlyoutCommandBarFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICommandBarFlyoutCommandBarFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommandBarFlyoutCommandBarFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICommandBarFlyoutCommandBarFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: ICommandBarFlyoutCommandBarFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ICommandBarFlyoutCommandBarTemplateSettings_Impl: Sized {
    fn OpenAnimationStartPosition(&mut self) -> ::windows::core::Result<f64>;
    fn OpenAnimationEndPosition(&mut self) -> ::windows::core::Result<f64>;
    fn CloseAnimationEndPosition(&mut self) -> ::windows::core::Result<f64>;
    fn CurrentWidth(&mut self) -> ::windows::core::Result<f64>;
    fn ExpandedWidth(&mut self) -> ::windows::core::Result<f64>;
    fn WidthExpansionDelta(&mut self) -> ::windows::core::Result<f64>;
    fn WidthExpansionAnimationStartPosition(&mut self) -> ::windows::core::Result<f64>;
    fn WidthExpansionAnimationEndPosition(&mut self) -> ::windows::core::Result<f64>;
    fn WidthExpansionMoreButtonAnimationStartPosition(&mut self) -> ::windows::core::Result<f64>;
    fn WidthExpansionMoreButtonAnimationEndPosition(&mut self) -> ::windows::core::Result<f64>;
    fn ExpandUpOverflowVerticalPosition(&mut self) -> ::windows::core::Result<f64>;
    fn ExpandDownOverflowVerticalPosition(&mut self) -> ::windows::core::Result<f64>;
    fn ExpandUpAnimationStartPosition(&mut self) -> ::windows::core::Result<f64>;
    fn ExpandUpAnimationEndPosition(&mut self) -> ::windows::core::Result<f64>;
    fn ExpandUpAnimationHoldPosition(&mut self) -> ::windows::core::Result<f64>;
    fn ExpandDownAnimationStartPosition(&mut self) -> ::windows::core::Result<f64>;
    fn ExpandDownAnimationEndPosition(&mut self) -> ::windows::core::Result<f64>;
    fn ExpandDownAnimationHoldPosition(&mut self) -> ::windows::core::Result<f64>;
    fn ContentClipRect(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect>;
    fn OverflowContentClipRect(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICommandBarFlyoutCommandBarTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ICommandBarFlyoutCommandBarTemplateSettings";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICommandBarFlyoutCommandBarTemplateSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommandBarFlyoutCommandBarTemplateSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICommandBarFlyoutCommandBarTemplateSettings_Vtbl {
        unsafe extern "system" fn OpenAnimationStartPosition<Impl: ICommandBarFlyoutCommandBarTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OpenAnimationEndPosition<Impl: ICommandBarFlyoutCommandBarTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CloseAnimationEndPosition<Impl: ICommandBarFlyoutCommandBarTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CurrentWidth<Impl: ICommandBarFlyoutCommandBarTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExpandedWidth<Impl: ICommandBarFlyoutCommandBarTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn WidthExpansionDelta<Impl: ICommandBarFlyoutCommandBarTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn WidthExpansionAnimationStartPosition<Impl: ICommandBarFlyoutCommandBarTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn WidthExpansionAnimationEndPosition<Impl: ICommandBarFlyoutCommandBarTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn WidthExpansionMoreButtonAnimationStartPosition<Impl: ICommandBarFlyoutCommandBarTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn WidthExpansionMoreButtonAnimationEndPosition<Impl: ICommandBarFlyoutCommandBarTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExpandUpOverflowVerticalPosition<Impl: ICommandBarFlyoutCommandBarTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExpandDownOverflowVerticalPosition<Impl: ICommandBarFlyoutCommandBarTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExpandUpAnimationStartPosition<Impl: ICommandBarFlyoutCommandBarTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExpandUpAnimationEndPosition<Impl: ICommandBarFlyoutCommandBarTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExpandUpAnimationHoldPosition<Impl: ICommandBarFlyoutCommandBarTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExpandDownAnimationStartPosition<Impl: ICommandBarFlyoutCommandBarTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExpandDownAnimationEndPosition<Impl: ICommandBarFlyoutCommandBarTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExpandDownAnimationHoldPosition<Impl: ICommandBarFlyoutCommandBarTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ContentClipRect<Impl: ICommandBarFlyoutCommandBarTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OverflowContentClipRect<Impl: ICommandBarFlyoutCommandBarTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
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
pub trait ICommandBarTemplateSettings_Impl: Sized {
    fn ContentHeight(&mut self) -> ::windows::core::Result<f64>;
    fn OverflowContentClipRect(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect>;
    fn OverflowContentMinWidth(&mut self) -> ::windows::core::Result<f64>;
    fn OverflowContentMaxHeight(&mut self) -> ::windows::core::Result<f64>;
    fn OverflowContentHorizontalOffset(&mut self) -> ::windows::core::Result<f64>;
    fn OverflowContentHeight(&mut self) -> ::windows::core::Result<f64>;
    fn NegativeOverflowContentHeight(&mut self) -> ::windows::core::Result<f64>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICommandBarTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ICommandBarTemplateSettings";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ICommandBarTemplateSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommandBarTemplateSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICommandBarTemplateSettings_Vtbl {
        unsafe extern "system" fn ContentHeight<Impl: ICommandBarTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OverflowContentClipRect<Impl: ICommandBarTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OverflowContentMinWidth<Impl: ICommandBarTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OverflowContentMaxHeight<Impl: ICommandBarTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OverflowContentHorizontalOffset<Impl: ICommandBarTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OverflowContentHeight<Impl: ICommandBarTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NegativeOverflowContentHeight<Impl: ICommandBarTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
pub trait ICommandBarTemplateSettings2_Impl: Sized {
    fn OverflowContentMaxWidth(&mut self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICommandBarTemplateSettings2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ICommandBarTemplateSettings2";
}
#[cfg(feature = "implement_exclusive")]
impl ICommandBarTemplateSettings2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommandBarTemplateSettings2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICommandBarTemplateSettings2_Vtbl {
        unsafe extern "system" fn OverflowContentMaxWidth<Impl: ICommandBarTemplateSettings2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
pub trait ICommandBarTemplateSettings3_Impl: Sized {
    fn EffectiveOverflowButtonVisibility(&mut self) -> ::windows::core::Result<super::super::Visibility>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICommandBarTemplateSettings3 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ICommandBarTemplateSettings3";
}
#[cfg(feature = "implement_exclusive")]
impl ICommandBarTemplateSettings3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommandBarTemplateSettings3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICommandBarTemplateSettings3_Vtbl {
        unsafe extern "system" fn EffectiveOverflowButtonVisibility<Impl: ICommandBarTemplateSettings3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Visibility) -> ::windows::core::HRESULT {
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
pub trait ICommandBarTemplateSettings4_Impl: Sized {
    fn OverflowContentCompactYTranslation(&mut self) -> ::windows::core::Result<f64>;
    fn OverflowContentMinimalYTranslation(&mut self) -> ::windows::core::Result<f64>;
    fn OverflowContentHiddenYTranslation(&mut self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICommandBarTemplateSettings4 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ICommandBarTemplateSettings4";
}
#[cfg(feature = "implement_exclusive")]
impl ICommandBarTemplateSettings4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommandBarTemplateSettings4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICommandBarTemplateSettings4_Vtbl {
        unsafe extern "system" fn OverflowContentCompactYTranslation<Impl: ICommandBarTemplateSettings4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OverflowContentMinimalYTranslation<Impl: ICommandBarTemplateSettings4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OverflowContentHiddenYTranslation<Impl: ICommandBarTemplateSettings4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
pub trait IDragCompletedEventArgs_Impl: Sized {
    fn HorizontalChange(&mut self) -> ::windows::core::Result<f64>;
    fn VerticalChange(&mut self) -> ::windows::core::Result<f64>;
    fn Canceled(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDragCompletedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IDragCompletedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IDragCompletedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDragCompletedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDragCompletedEventArgs_Vtbl {
        unsafe extern "system" fn HorizontalChange<Impl: IDragCompletedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn VerticalChange<Impl: IDragCompletedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Canceled<Impl: IDragCompletedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
pub trait IDragCompletedEventArgsFactory_Impl: Sized {
    fn CreateInstanceWithHorizontalChangeVerticalChangeAndCanceled(&mut self, horizontalchange: f64, verticalchange: f64, canceled: bool, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<DragCompletedEventArgs>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDragCompletedEventArgsFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IDragCompletedEventArgsFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IDragCompletedEventArgsFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDragCompletedEventArgsFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDragCompletedEventArgsFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithHorizontalChangeVerticalChangeAndCanceled<Impl: IDragCompletedEventArgsFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalchange: f64, verticalchange: f64, canceled: bool, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IDragDeltaEventArgs_Impl: Sized {
    fn HorizontalChange(&mut self) -> ::windows::core::Result<f64>;
    fn VerticalChange(&mut self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDragDeltaEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IDragDeltaEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IDragDeltaEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDragDeltaEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDragDeltaEventArgs_Vtbl {
        unsafe extern "system" fn HorizontalChange<Impl: IDragDeltaEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn VerticalChange<Impl: IDragDeltaEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
pub trait IDragDeltaEventArgsFactory_Impl: Sized {
    fn CreateInstanceWithHorizontalChangeAndVerticalChange(&mut self, horizontalchange: f64, verticalchange: f64, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<DragDeltaEventArgs>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDragDeltaEventArgsFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IDragDeltaEventArgsFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IDragDeltaEventArgsFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDragDeltaEventArgsFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDragDeltaEventArgsFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithHorizontalChangeAndVerticalChange<Impl: IDragDeltaEventArgsFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontalchange: f64, verticalchange: f64, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IDragStartedEventArgs_Impl: Sized {
    fn HorizontalOffset(&mut self) -> ::windows::core::Result<f64>;
    fn VerticalOffset(&mut self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDragStartedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IDragStartedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IDragStartedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDragStartedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDragStartedEventArgs_Vtbl {
        unsafe extern "system" fn HorizontalOffset<Impl: IDragStartedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn VerticalOffset<Impl: IDragStartedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
pub trait IDragStartedEventArgsFactory_Impl: Sized {
    fn CreateInstanceWithHorizontalOffsetAndVerticalOffset(&mut self, horizontaloffset: f64, verticaloffset: f64, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<DragStartedEventArgs>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDragStartedEventArgsFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IDragStartedEventArgsFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IDragStartedEventArgsFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDragStartedEventArgsFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDragStartedEventArgsFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithHorizontalOffsetAndVerticalOffset<Impl: IDragStartedEventArgsFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, horizontaloffset: f64, verticaloffset: f64, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IFlyoutBase_Impl: Sized {
    fn Placement(&mut self) -> ::windows::core::Result<FlyoutPlacementMode>;
    fn SetPlacement(&mut self, value: FlyoutPlacementMode) -> ::windows::core::Result<()>;
    fn Opened(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveOpened(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Closed(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Opening(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveOpening(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ShowAt(&mut self, placementtarget: &::core::option::Option<super::super::FrameworkElement>) -> ::windows::core::Result<()>;
    fn Hide(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFlyoutBase {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IFlyoutBase";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IFlyoutBase_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlyoutBase_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlyoutBase_Vtbl {
        unsafe extern "system" fn Placement<Impl: IFlyoutBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FlyoutPlacementMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPlacement<Impl: IFlyoutBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FlyoutPlacementMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPlacement(value).into()
        }
        unsafe extern "system" fn Opened<Impl: IFlyoutBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveOpened<Impl: IFlyoutBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveOpened(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Closed<Impl: IFlyoutBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveClosed<Impl: IFlyoutBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveClosed(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Opening<Impl: IFlyoutBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveOpening<Impl: IFlyoutBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveOpening(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ShowAt<Impl: IFlyoutBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, placementtarget: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowAt(&*(&placementtarget as *const <super::super::FrameworkElement as ::windows::core::Abi>::Abi as *const <super::super::FrameworkElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Hide<Impl: IFlyoutBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IFlyoutBase2_Impl: Sized {
    fn Target(&mut self) -> ::windows::core::Result<super::super::FrameworkElement>;
    fn AllowFocusOnInteraction(&mut self) -> ::windows::core::Result<bool>;
    fn SetAllowFocusOnInteraction(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn LightDismissOverlayMode(&mut self) -> ::windows::core::Result<super::LightDismissOverlayMode>;
    fn SetLightDismissOverlayMode(&mut self, value: super::LightDismissOverlayMode) -> ::windows::core::Result<()>;
    fn AllowFocusWhenDisabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetAllowFocusWhenDisabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn ElementSoundMode(&mut self) -> ::windows::core::Result<super::super::ElementSoundMode>;
    fn SetElementSoundMode(&mut self, value: super::super::ElementSoundMode) -> ::windows::core::Result<()>;
    fn Closing(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<FlyoutBase, FlyoutBaseClosingEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosing(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFlyoutBase2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IFlyoutBase2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IFlyoutBase2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlyoutBase2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlyoutBase2_Vtbl {
        unsafe extern "system" fn Target<Impl: IFlyoutBase2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AllowFocusOnInteraction<Impl: IFlyoutBase2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAllowFocusOnInteraction<Impl: IFlyoutBase2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowFocusOnInteraction(value).into()
        }
        unsafe extern "system" fn LightDismissOverlayMode<Impl: IFlyoutBase2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::LightDismissOverlayMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetLightDismissOverlayMode<Impl: IFlyoutBase2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::LightDismissOverlayMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLightDismissOverlayMode(value).into()
        }
        unsafe extern "system" fn AllowFocusWhenDisabled<Impl: IFlyoutBase2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAllowFocusWhenDisabled<Impl: IFlyoutBase2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowFocusWhenDisabled(value).into()
        }
        unsafe extern "system" fn ElementSoundMode<Impl: IFlyoutBase2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::ElementSoundMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetElementSoundMode<Impl: IFlyoutBase2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::ElementSoundMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetElementSoundMode(value).into()
        }
        unsafe extern "system" fn Closing<Impl: IFlyoutBase2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveClosing<Impl: IFlyoutBase2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
pub trait IFlyoutBase3_Impl: Sized {
    fn OverlayInputPassThroughElement(&mut self) -> ::windows::core::Result<super::super::DependencyObject>;
    fn SetOverlayInputPassThroughElement(&mut self, value: &::core::option::Option<super::super::DependencyObject>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFlyoutBase3 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IFlyoutBase3";
}
#[cfg(feature = "implement_exclusive")]
impl IFlyoutBase3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlyoutBase3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlyoutBase3_Vtbl {
        unsafe extern "system" fn OverlayInputPassThroughElement<Impl: IFlyoutBase3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOverlayInputPassThroughElement<Impl: IFlyoutBase3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IFlyoutBase4_Impl: Sized {
    fn TryInvokeKeyboardAccelerator(&mut self, args: &::core::option::Option<super::super::Input::ProcessKeyboardAcceleratorEventArgs>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Xaml_Input", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFlyoutBase4 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IFlyoutBase4";
}
#[cfg(all(feature = "UI_Xaml_Input", feature = "implement_exclusive"))]
impl IFlyoutBase4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlyoutBase4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlyoutBase4_Vtbl {
        unsafe extern "system" fn TryInvokeKeyboardAccelerator<Impl: IFlyoutBase4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IFlyoutBase5_Impl: Sized {
    fn ShowMode(&mut self) -> ::windows::core::Result<FlyoutShowMode>;
    fn SetShowMode(&mut self, value: FlyoutShowMode) -> ::windows::core::Result<()>;
    fn InputDevicePrefersPrimaryCommands(&mut self) -> ::windows::core::Result<bool>;
    fn AreOpenCloseAnimationsEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetAreOpenCloseAnimationsEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsOpen(&mut self) -> ::windows::core::Result<bool>;
    fn ShowAt(&mut self, placementtarget: &::core::option::Option<super::super::DependencyObject>, showoptions: &::core::option::Option<FlyoutShowOptions>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFlyoutBase5 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IFlyoutBase5";
}
#[cfg(feature = "implement_exclusive")]
impl IFlyoutBase5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlyoutBase5_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlyoutBase5_Vtbl {
        unsafe extern "system" fn ShowMode<Impl: IFlyoutBase5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FlyoutShowMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetShowMode<Impl: IFlyoutBase5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FlyoutShowMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShowMode(value).into()
        }
        unsafe extern "system" fn InputDevicePrefersPrimaryCommands<Impl: IFlyoutBase5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AreOpenCloseAnimationsEnabled<Impl: IFlyoutBase5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAreOpenCloseAnimationsEnabled<Impl: IFlyoutBase5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAreOpenCloseAnimationsEnabled(value).into()
        }
        unsafe extern "system" fn IsOpen<Impl: IFlyoutBase5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ShowAt<Impl: IFlyoutBase5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, placementtarget: ::windows::core::RawPtr, showoptions: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IFlyoutBase6_Impl: Sized {
    fn ShouldConstrainToRootBounds(&mut self) -> ::windows::core::Result<bool>;
    fn SetShouldConstrainToRootBounds(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsConstrainedToRootBounds(&mut self) -> ::windows::core::Result<bool>;
    fn XamlRoot(&mut self) -> ::windows::core::Result<super::super::XamlRoot>;
    fn SetXamlRoot(&mut self, value: &::core::option::Option<super::super::XamlRoot>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFlyoutBase6 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IFlyoutBase6";
}
#[cfg(feature = "implement_exclusive")]
impl IFlyoutBase6_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlyoutBase6_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlyoutBase6_Vtbl {
        unsafe extern "system" fn ShouldConstrainToRootBounds<Impl: IFlyoutBase6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetShouldConstrainToRootBounds<Impl: IFlyoutBase6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShouldConstrainToRootBounds(value).into()
        }
        unsafe extern "system" fn IsConstrainedToRootBounds<Impl: IFlyoutBase6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn XamlRoot<Impl: IFlyoutBase6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetXamlRoot<Impl: IFlyoutBase6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IFlyoutBaseClosingEventArgs_Impl: Sized {
    fn Cancel(&mut self) -> ::windows::core::Result<bool>;
    fn SetCancel(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFlyoutBaseClosingEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IFlyoutBaseClosingEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IFlyoutBaseClosingEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlyoutBaseClosingEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlyoutBaseClosingEventArgs_Vtbl {
        unsafe extern "system" fn Cancel<Impl: IFlyoutBaseClosingEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCancel<Impl: IFlyoutBaseClosingEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
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
pub trait IFlyoutBaseFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<FlyoutBase>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFlyoutBaseFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IFlyoutBaseFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IFlyoutBaseFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlyoutBaseFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlyoutBaseFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IFlyoutBaseFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IFlyoutBaseOverrides_Impl: Sized {
    fn CreatePresenter(&mut self) -> ::windows::core::Result<super::Control>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFlyoutBaseOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IFlyoutBaseOverrides";
}
#[cfg(feature = "implement_exclusive")]
impl IFlyoutBaseOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlyoutBaseOverrides_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlyoutBaseOverrides_Vtbl {
        unsafe extern "system" fn CreatePresenter<Impl: IFlyoutBaseOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IFlyoutBaseOverrides4_Impl: Sized {
    fn OnProcessKeyboardAccelerators(&mut self, args: &::core::option::Option<super::super::Input::ProcessKeyboardAcceleratorEventArgs>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Xaml_Input", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFlyoutBaseOverrides4 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IFlyoutBaseOverrides4";
}
#[cfg(all(feature = "UI_Xaml_Input", feature = "implement_exclusive"))]
impl IFlyoutBaseOverrides4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlyoutBaseOverrides4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlyoutBaseOverrides4_Vtbl {
        unsafe extern "system" fn OnProcessKeyboardAccelerators<Impl: IFlyoutBaseOverrides4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IFlyoutBaseStatics_Impl: Sized {
    fn PlacementProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn AttachedFlyoutProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn GetAttachedFlyout(&mut self, element: &::core::option::Option<super::super::FrameworkElement>) -> ::windows::core::Result<FlyoutBase>;
    fn SetAttachedFlyout(&mut self, element: &::core::option::Option<super::super::FrameworkElement>, value: &::core::option::Option<FlyoutBase>) -> ::windows::core::Result<()>;
    fn ShowAttachedFlyout(&mut self, flyoutowner: &::core::option::Option<super::super::FrameworkElement>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFlyoutBaseStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IFlyoutBaseStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IFlyoutBaseStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlyoutBaseStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlyoutBaseStatics_Vtbl {
        unsafe extern "system" fn PlacementProperty<Impl: IFlyoutBaseStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AttachedFlyoutProperty<Impl: IFlyoutBaseStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetAttachedFlyout<Impl: IFlyoutBaseStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAttachedFlyout<Impl: IFlyoutBaseStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAttachedFlyout(&*(&element as *const <super::super::FrameworkElement as ::windows::core::Abi>::Abi as *const <super::super::FrameworkElement as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <FlyoutBase as ::windows::core::Abi>::Abi as *const <FlyoutBase as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ShowAttachedFlyout<Impl: IFlyoutBaseStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flyoutowner: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IFlyoutBaseStatics2_Impl: Sized {
    fn AllowFocusOnInteractionProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn LightDismissOverlayModeProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn AllowFocusWhenDisabledProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ElementSoundModeProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFlyoutBaseStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IFlyoutBaseStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IFlyoutBaseStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlyoutBaseStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlyoutBaseStatics2_Vtbl {
        unsafe extern "system" fn AllowFocusOnInteractionProperty<Impl: IFlyoutBaseStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LightDismissOverlayModeProperty<Impl: IFlyoutBaseStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AllowFocusWhenDisabledProperty<Impl: IFlyoutBaseStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ElementSoundModeProperty<Impl: IFlyoutBaseStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IFlyoutBaseStatics3_Impl: Sized {
    fn OverlayInputPassThroughElementProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFlyoutBaseStatics3 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IFlyoutBaseStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IFlyoutBaseStatics3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlyoutBaseStatics3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlyoutBaseStatics3_Vtbl {
        unsafe extern "system" fn OverlayInputPassThroughElementProperty<Impl: IFlyoutBaseStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IFlyoutBaseStatics5_Impl: Sized {
    fn TargetProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ShowModeProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn InputDevicePrefersPrimaryCommandsProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn AreOpenCloseAnimationsEnabledProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn IsOpenProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFlyoutBaseStatics5 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IFlyoutBaseStatics5";
}
#[cfg(feature = "implement_exclusive")]
impl IFlyoutBaseStatics5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlyoutBaseStatics5_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlyoutBaseStatics5_Vtbl {
        unsafe extern "system" fn TargetProperty<Impl: IFlyoutBaseStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ShowModeProperty<Impl: IFlyoutBaseStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn InputDevicePrefersPrimaryCommandsProperty<Impl: IFlyoutBaseStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AreOpenCloseAnimationsEnabledProperty<Impl: IFlyoutBaseStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsOpenProperty<Impl: IFlyoutBaseStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IFlyoutBaseStatics6_Impl: Sized {
    fn ShouldConstrainToRootBoundsProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFlyoutBaseStatics6 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IFlyoutBaseStatics6";
}
#[cfg(feature = "implement_exclusive")]
impl IFlyoutBaseStatics6_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlyoutBaseStatics6_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlyoutBaseStatics6_Vtbl {
        unsafe extern "system" fn ShouldConstrainToRootBoundsProperty<Impl: IFlyoutBaseStatics6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IFlyoutShowOptions_Impl: Sized {
    fn Position(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point>>;
    fn SetPosition(&mut self, value: &::core::option::Option<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point>>) -> ::windows::core::Result<()>;
    fn ExclusionRect(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Rect>>;
    fn SetExclusionRect(&mut self, value: &::core::option::Option<super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Rect>>) -> ::windows::core::Result<()>;
    fn ShowMode(&mut self) -> ::windows::core::Result<FlyoutShowMode>;
    fn SetShowMode(&mut self, value: FlyoutShowMode) -> ::windows::core::Result<()>;
    fn Placement(&mut self) -> ::windows::core::Result<FlyoutPlacementMode>;
    fn SetPlacement(&mut self, value: FlyoutPlacementMode) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFlyoutShowOptions {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IFlyoutShowOptions";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IFlyoutShowOptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlyoutShowOptions_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlyoutShowOptions_Vtbl {
        unsafe extern "system" fn Position<Impl: IFlyoutShowOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPosition<Impl: IFlyoutShowOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPosition(&*(&value as *const <super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Point> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ExclusionRect<Impl: IFlyoutShowOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetExclusionRect<Impl: IFlyoutShowOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExclusionRect(&*(&value as *const <super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Rect> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::IReference<super::super::super::super::Foundation::Rect> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ShowMode<Impl: IFlyoutShowOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FlyoutShowMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetShowMode<Impl: IFlyoutShowOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FlyoutShowMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShowMode(value).into()
        }
        unsafe extern "system" fn Placement<Impl: IFlyoutShowOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FlyoutPlacementMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPlacement<Impl: IFlyoutShowOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: FlyoutPlacementMode) -> ::windows::core::HRESULT {
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
pub trait IFlyoutShowOptionsFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<FlyoutShowOptions>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFlyoutShowOptionsFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IFlyoutShowOptionsFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IFlyoutShowOptionsFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFlyoutShowOptionsFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFlyoutShowOptionsFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IFlyoutShowOptionsFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IGeneratorPositionHelper_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeneratorPositionHelper {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IGeneratorPositionHelper";
}
#[cfg(feature = "implement_exclusive")]
impl IGeneratorPositionHelper_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeneratorPositionHelper_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGeneratorPositionHelper_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IGeneratorPositionHelper, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGeneratorPositionHelper as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeneratorPositionHelperStatics_Impl: Sized {
    fn FromIndexAndOffset(&mut self, index: i32, offset: i32) -> ::windows::core::Result<GeneratorPosition>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGeneratorPositionHelperStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IGeneratorPositionHelperStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGeneratorPositionHelperStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGeneratorPositionHelperStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGeneratorPositionHelperStatics_Vtbl {
        unsafe extern "system" fn FromIndexAndOffset<Impl: IGeneratorPositionHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, offset: i32, result__: *mut GeneratorPosition) -> ::windows::core::HRESULT {
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
pub trait IGridViewItemPresenter_Impl: Sized {
    fn SelectionCheckMarkVisualEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetSelectionCheckMarkVisualEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn CheckHintBrush(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckHintBrush(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckSelectingBrush(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckSelectingBrush(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckBrush(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckBrush(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn DragBackground(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetDragBackground(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn DragForeground(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetDragForeground(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn FocusBorderBrush(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetFocusBorderBrush(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn PlaceholderBackground(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetPlaceholderBackground(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn PointerOverBackground(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetPointerOverBackground(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectedBackground(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetSelectedBackground(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectedForeground(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetSelectedForeground(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectedPointerOverBackground(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetSelectedPointerOverBackground(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectedPointerOverBorderBrush(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetSelectedPointerOverBorderBrush(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectedBorderThickness(&mut self) -> ::windows::core::Result<super::super::Thickness>;
    fn SetSelectedBorderThickness(&mut self, value: &super::super::Thickness) -> ::windows::core::Result<()>;
    fn DisabledOpacity(&mut self) -> ::windows::core::Result<f64>;
    fn SetDisabledOpacity(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn DragOpacity(&mut self) -> ::windows::core::Result<f64>;
    fn SetDragOpacity(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn ReorderHintOffset(&mut self) -> ::windows::core::Result<f64>;
    fn SetReorderHintOffset(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn GridViewItemPresenterHorizontalContentAlignment(&mut self) -> ::windows::core::Result<super::super::HorizontalAlignment>;
    fn SetGridViewItemPresenterHorizontalContentAlignment(&mut self, value: super::super::HorizontalAlignment) -> ::windows::core::Result<()>;
    fn GridViewItemPresenterVerticalContentAlignment(&mut self) -> ::windows::core::Result<super::super::VerticalAlignment>;
    fn SetGridViewItemPresenterVerticalContentAlignment(&mut self, value: super::super::VerticalAlignment) -> ::windows::core::Result<()>;
    fn GridViewItemPresenterPadding(&mut self) -> ::windows::core::Result<super::super::Thickness>;
    fn SetGridViewItemPresenterPadding(&mut self, value: &super::super::Thickness) -> ::windows::core::Result<()>;
    fn PointerOverBackgroundMargin(&mut self) -> ::windows::core::Result<super::super::Thickness>;
    fn SetPointerOverBackgroundMargin(&mut self, value: &super::super::Thickness) -> ::windows::core::Result<()>;
    fn ContentMargin(&mut self) -> ::windows::core::Result<super::super::Thickness>;
    fn SetContentMargin(&mut self, value: &super::super::Thickness) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IGridViewItemPresenter {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IGridViewItemPresenter";
}
#[cfg(all(feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
impl IGridViewItemPresenter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridViewItemPresenter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGridViewItemPresenter_Vtbl {
        unsafe extern "system" fn SelectionCheckMarkVisualEnabled<Impl: IGridViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSelectionCheckMarkVisualEnabled<Impl: IGridViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectionCheckMarkVisualEnabled(value).into()
        }
        unsafe extern "system" fn CheckHintBrush<Impl: IGridViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCheckHintBrush<Impl: IGridViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCheckHintBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckSelectingBrush<Impl: IGridViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCheckSelectingBrush<Impl: IGridViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCheckSelectingBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckBrush<Impl: IGridViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCheckBrush<Impl: IGridViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCheckBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DragBackground<Impl: IGridViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDragBackground<Impl: IGridViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDragBackground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DragForeground<Impl: IGridViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDragForeground<Impl: IGridViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDragForeground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FocusBorderBrush<Impl: IGridViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetFocusBorderBrush<Impl: IGridViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFocusBorderBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PlaceholderBackground<Impl: IGridViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPlaceholderBackground<Impl: IGridViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPlaceholderBackground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerOverBackground<Impl: IGridViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPointerOverBackground<Impl: IGridViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPointerOverBackground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedBackground<Impl: IGridViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSelectedBackground<Impl: IGridViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectedBackground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedForeground<Impl: IGridViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSelectedForeground<Impl: IGridViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectedForeground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedPointerOverBackground<Impl: IGridViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSelectedPointerOverBackground<Impl: IGridViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectedPointerOverBackground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedPointerOverBorderBrush<Impl: IGridViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSelectedPointerOverBorderBrush<Impl: IGridViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectedPointerOverBorderBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedBorderThickness<Impl: IGridViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSelectedBorderThickness<Impl: IGridViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectedBorderThickness(&*(&value as *const <super::super::Thickness as ::windows::core::Abi>::Abi as *const <super::super::Thickness as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DisabledOpacity<Impl: IGridViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDisabledOpacity<Impl: IGridViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisabledOpacity(value).into()
        }
        unsafe extern "system" fn DragOpacity<Impl: IGridViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDragOpacity<Impl: IGridViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDragOpacity(value).into()
        }
        unsafe extern "system" fn ReorderHintOffset<Impl: IGridViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetReorderHintOffset<Impl: IGridViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReorderHintOffset(value).into()
        }
        unsafe extern "system" fn GridViewItemPresenterHorizontalContentAlignment<Impl: IGridViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::HorizontalAlignment) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetGridViewItemPresenterHorizontalContentAlignment<Impl: IGridViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::HorizontalAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGridViewItemPresenterHorizontalContentAlignment(value).into()
        }
        unsafe extern "system" fn GridViewItemPresenterVerticalContentAlignment<Impl: IGridViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::VerticalAlignment) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetGridViewItemPresenterVerticalContentAlignment<Impl: IGridViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::VerticalAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGridViewItemPresenterVerticalContentAlignment(value).into()
        }
        unsafe extern "system" fn GridViewItemPresenterPadding<Impl: IGridViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetGridViewItemPresenterPadding<Impl: IGridViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGridViewItemPresenterPadding(&*(&value as *const <super::super::Thickness as ::windows::core::Abi>::Abi as *const <super::super::Thickness as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerOverBackgroundMargin<Impl: IGridViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPointerOverBackgroundMargin<Impl: IGridViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPointerOverBackgroundMargin(&*(&value as *const <super::super::Thickness as ::windows::core::Abi>::Abi as *const <super::super::Thickness as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContentMargin<Impl: IGridViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetContentMargin<Impl: IGridViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Thickness) -> ::windows::core::HRESULT {
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
pub trait IGridViewItemPresenterFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<GridViewItemPresenter>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGridViewItemPresenterFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IGridViewItemPresenterFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IGridViewItemPresenterFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridViewItemPresenterFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGridViewItemPresenterFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IGridViewItemPresenterFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IGridViewItemPresenterStatics_Impl: Sized {
    fn SelectionCheckMarkVisualEnabledProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckHintBrushProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckSelectingBrushProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckBrushProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn DragBackgroundProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn DragForegroundProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn FocusBorderBrushProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn PlaceholderBackgroundProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn PointerOverBackgroundProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedBackgroundProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedForegroundProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedPointerOverBackgroundProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedPointerOverBorderBrushProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedBorderThicknessProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn DisabledOpacityProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn DragOpacityProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ReorderHintOffsetProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn GridViewItemPresenterHorizontalContentAlignmentProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn GridViewItemPresenterVerticalContentAlignmentProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn GridViewItemPresenterPaddingProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn PointerOverBackgroundMarginProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ContentMarginProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGridViewItemPresenterStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IGridViewItemPresenterStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGridViewItemPresenterStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridViewItemPresenterStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGridViewItemPresenterStatics_Vtbl {
        unsafe extern "system" fn SelectionCheckMarkVisualEnabledProperty<Impl: IGridViewItemPresenterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CheckHintBrushProperty<Impl: IGridViewItemPresenterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CheckSelectingBrushProperty<Impl: IGridViewItemPresenterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CheckBrushProperty<Impl: IGridViewItemPresenterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DragBackgroundProperty<Impl: IGridViewItemPresenterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DragForegroundProperty<Impl: IGridViewItemPresenterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FocusBorderBrushProperty<Impl: IGridViewItemPresenterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PlaceholderBackgroundProperty<Impl: IGridViewItemPresenterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PointerOverBackgroundProperty<Impl: IGridViewItemPresenterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SelectedBackgroundProperty<Impl: IGridViewItemPresenterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SelectedForegroundProperty<Impl: IGridViewItemPresenterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SelectedPointerOverBackgroundProperty<Impl: IGridViewItemPresenterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SelectedPointerOverBorderBrushProperty<Impl: IGridViewItemPresenterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SelectedBorderThicknessProperty<Impl: IGridViewItemPresenterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DisabledOpacityProperty<Impl: IGridViewItemPresenterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DragOpacityProperty<Impl: IGridViewItemPresenterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReorderHintOffsetProperty<Impl: IGridViewItemPresenterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GridViewItemPresenterHorizontalContentAlignmentProperty<Impl: IGridViewItemPresenterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GridViewItemPresenterVerticalContentAlignmentProperty<Impl: IGridViewItemPresenterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GridViewItemPresenterPaddingProperty<Impl: IGridViewItemPresenterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PointerOverBackgroundMarginProperty<Impl: IGridViewItemPresenterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ContentMarginProperty<Impl: IGridViewItemPresenterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IGridViewItemTemplateSettings_Impl: Sized {
    fn DragItemsCount(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGridViewItemTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IGridViewItemTemplateSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IGridViewItemTemplateSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGridViewItemTemplateSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGridViewItemTemplateSettings_Vtbl {
        unsafe extern "system" fn DragItemsCount<Impl: IGridViewItemTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
pub trait IItemsChangedEventArgs_Impl: Sized {
    fn Action(&mut self) -> ::windows::core::Result<i32>;
    fn Position(&mut self) -> ::windows::core::Result<GeneratorPosition>;
    fn OldPosition(&mut self) -> ::windows::core::Result<GeneratorPosition>;
    fn ItemCount(&mut self) -> ::windows::core::Result<i32>;
    fn ItemUICount(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IItemsChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IItemsChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IItemsChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IItemsChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IItemsChangedEventArgs_Vtbl {
        unsafe extern "system" fn Action<Impl: IItemsChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Position<Impl: IItemsChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GeneratorPosition) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OldPosition<Impl: IItemsChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GeneratorPosition) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ItemCount<Impl: IItemsChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ItemUICount<Impl: IItemsChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
pub trait IJumpListItemBackgroundConverter_Impl: Sized {
    fn Enabled(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetEnabled(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn Disabled(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetDisabled(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IJumpListItemBackgroundConverter {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IJumpListItemBackgroundConverter";
}
#[cfg(all(feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
impl IJumpListItemBackgroundConverter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IJumpListItemBackgroundConverter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IJumpListItemBackgroundConverter_Vtbl {
        unsafe extern "system" fn Enabled<Impl: IJumpListItemBackgroundConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetEnabled<Impl: IJumpListItemBackgroundConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnabled(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Disabled<Impl: IJumpListItemBackgroundConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDisabled<Impl: IJumpListItemBackgroundConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IJumpListItemBackgroundConverterStatics_Impl: Sized {
    fn EnabledProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn DisabledProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IJumpListItemBackgroundConverterStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IJumpListItemBackgroundConverterStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IJumpListItemBackgroundConverterStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IJumpListItemBackgroundConverterStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IJumpListItemBackgroundConverterStatics_Vtbl {
        unsafe extern "system" fn EnabledProperty<Impl: IJumpListItemBackgroundConverterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DisabledProperty<Impl: IJumpListItemBackgroundConverterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IJumpListItemForegroundConverter_Impl: Sized {
    fn Enabled(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetEnabled(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn Disabled(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetDisabled(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IJumpListItemForegroundConverter {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IJumpListItemForegroundConverter";
}
#[cfg(all(feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
impl IJumpListItemForegroundConverter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IJumpListItemForegroundConverter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IJumpListItemForegroundConverter_Vtbl {
        unsafe extern "system" fn Enabled<Impl: IJumpListItemForegroundConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetEnabled<Impl: IJumpListItemForegroundConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnabled(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Disabled<Impl: IJumpListItemForegroundConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDisabled<Impl: IJumpListItemForegroundConverter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IJumpListItemForegroundConverterStatics_Impl: Sized {
    fn EnabledProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn DisabledProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IJumpListItemForegroundConverterStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IJumpListItemForegroundConverterStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IJumpListItemForegroundConverterStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IJumpListItemForegroundConverterStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IJumpListItemForegroundConverterStatics_Vtbl {
        unsafe extern "system" fn EnabledProperty<Impl: IJumpListItemForegroundConverterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DisabledProperty<Impl: IJumpListItemForegroundConverterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ILayoutInformation_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILayoutInformation {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ILayoutInformation";
}
#[cfg(feature = "implement_exclusive")]
impl ILayoutInformation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILayoutInformation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILayoutInformation_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ILayoutInformation, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILayoutInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait ILayoutInformationStatics_Impl: Sized {
    fn GetLayoutExceptionElement(&mut self, dispatcher: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<super::super::UIElement>;
    fn GetLayoutSlot(&mut self, element: &::core::option::Option<super::super::FrameworkElement>) -> ::windows::core::Result<super::super::super::super::Foundation::Rect>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILayoutInformationStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ILayoutInformationStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ILayoutInformationStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILayoutInformationStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILayoutInformationStatics_Vtbl {
        unsafe extern "system" fn GetLayoutExceptionElement<Impl: ILayoutInformationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispatcher: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetLayoutSlot<Impl: ILayoutInformationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
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
pub trait ILayoutInformationStatics2_Impl: Sized {
    fn GetAvailableSize(&mut self, element: &::core::option::Option<super::super::UIElement>) -> ::windows::core::Result<super::super::super::super::Foundation::Size>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILayoutInformationStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ILayoutInformationStatics2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ILayoutInformationStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILayoutInformationStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILayoutInformationStatics2_Vtbl {
        unsafe extern "system" fn GetAvailableSize<Impl: ILayoutInformationStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::Size) -> ::windows::core::HRESULT {
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
pub trait IListViewItemPresenter_Impl: Sized {
    fn SelectionCheckMarkVisualEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetSelectionCheckMarkVisualEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn CheckHintBrush(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckHintBrush(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckSelectingBrush(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckSelectingBrush(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckBrush(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckBrush(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn DragBackground(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetDragBackground(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn DragForeground(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetDragForeground(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn FocusBorderBrush(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetFocusBorderBrush(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn PlaceholderBackground(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetPlaceholderBackground(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn PointerOverBackground(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetPointerOverBackground(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectedBackground(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetSelectedBackground(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectedForeground(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetSelectedForeground(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectedPointerOverBackground(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetSelectedPointerOverBackground(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectedPointerOverBorderBrush(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetSelectedPointerOverBorderBrush(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectedBorderThickness(&mut self) -> ::windows::core::Result<super::super::Thickness>;
    fn SetSelectedBorderThickness(&mut self, value: &super::super::Thickness) -> ::windows::core::Result<()>;
    fn DisabledOpacity(&mut self) -> ::windows::core::Result<f64>;
    fn SetDisabledOpacity(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn DragOpacity(&mut self) -> ::windows::core::Result<f64>;
    fn SetDragOpacity(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn ReorderHintOffset(&mut self) -> ::windows::core::Result<f64>;
    fn SetReorderHintOffset(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn ListViewItemPresenterHorizontalContentAlignment(&mut self) -> ::windows::core::Result<super::super::HorizontalAlignment>;
    fn SetListViewItemPresenterHorizontalContentAlignment(&mut self, value: super::super::HorizontalAlignment) -> ::windows::core::Result<()>;
    fn ListViewItemPresenterVerticalContentAlignment(&mut self) -> ::windows::core::Result<super::super::VerticalAlignment>;
    fn SetListViewItemPresenterVerticalContentAlignment(&mut self, value: super::super::VerticalAlignment) -> ::windows::core::Result<()>;
    fn ListViewItemPresenterPadding(&mut self) -> ::windows::core::Result<super::super::Thickness>;
    fn SetListViewItemPresenterPadding(&mut self, value: &super::super::Thickness) -> ::windows::core::Result<()>;
    fn PointerOverBackgroundMargin(&mut self) -> ::windows::core::Result<super::super::Thickness>;
    fn SetPointerOverBackgroundMargin(&mut self, value: &super::super::Thickness) -> ::windows::core::Result<()>;
    fn ContentMargin(&mut self) -> ::windows::core::Result<super::super::Thickness>;
    fn SetContentMargin(&mut self, value: &super::super::Thickness) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IListViewItemPresenter {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IListViewItemPresenter";
}
#[cfg(all(feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
impl IListViewItemPresenter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewItemPresenter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListViewItemPresenter_Vtbl {
        unsafe extern "system" fn SelectionCheckMarkVisualEnabled<Impl: IListViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSelectionCheckMarkVisualEnabled<Impl: IListViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectionCheckMarkVisualEnabled(value).into()
        }
        unsafe extern "system" fn CheckHintBrush<Impl: IListViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCheckHintBrush<Impl: IListViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCheckHintBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckSelectingBrush<Impl: IListViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCheckSelectingBrush<Impl: IListViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCheckSelectingBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckBrush<Impl: IListViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCheckBrush<Impl: IListViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCheckBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DragBackground<Impl: IListViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDragBackground<Impl: IListViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDragBackground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DragForeground<Impl: IListViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDragForeground<Impl: IListViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDragForeground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FocusBorderBrush<Impl: IListViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetFocusBorderBrush<Impl: IListViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFocusBorderBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PlaceholderBackground<Impl: IListViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPlaceholderBackground<Impl: IListViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPlaceholderBackground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerOverBackground<Impl: IListViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPointerOverBackground<Impl: IListViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPointerOverBackground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedBackground<Impl: IListViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSelectedBackground<Impl: IListViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectedBackground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedForeground<Impl: IListViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSelectedForeground<Impl: IListViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectedForeground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedPointerOverBackground<Impl: IListViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSelectedPointerOverBackground<Impl: IListViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectedPointerOverBackground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedPointerOverBorderBrush<Impl: IListViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSelectedPointerOverBorderBrush<Impl: IListViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectedPointerOverBorderBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedBorderThickness<Impl: IListViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSelectedBorderThickness<Impl: IListViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectedBorderThickness(&*(&value as *const <super::super::Thickness as ::windows::core::Abi>::Abi as *const <super::super::Thickness as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DisabledOpacity<Impl: IListViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDisabledOpacity<Impl: IListViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisabledOpacity(value).into()
        }
        unsafe extern "system" fn DragOpacity<Impl: IListViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDragOpacity<Impl: IListViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDragOpacity(value).into()
        }
        unsafe extern "system" fn ReorderHintOffset<Impl: IListViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetReorderHintOffset<Impl: IListViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReorderHintOffset(value).into()
        }
        unsafe extern "system" fn ListViewItemPresenterHorizontalContentAlignment<Impl: IListViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::HorizontalAlignment) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetListViewItemPresenterHorizontalContentAlignment<Impl: IListViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::HorizontalAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetListViewItemPresenterHorizontalContentAlignment(value).into()
        }
        unsafe extern "system" fn ListViewItemPresenterVerticalContentAlignment<Impl: IListViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::VerticalAlignment) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetListViewItemPresenterVerticalContentAlignment<Impl: IListViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::VerticalAlignment) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetListViewItemPresenterVerticalContentAlignment(value).into()
        }
        unsafe extern "system" fn ListViewItemPresenterPadding<Impl: IListViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetListViewItemPresenterPadding<Impl: IListViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetListViewItemPresenterPadding(&*(&value as *const <super::super::Thickness as ::windows::core::Abi>::Abi as *const <super::super::Thickness as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerOverBackgroundMargin<Impl: IListViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPointerOverBackgroundMargin<Impl: IListViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPointerOverBackgroundMargin(&*(&value as *const <super::super::Thickness as ::windows::core::Abi>::Abi as *const <super::super::Thickness as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ContentMargin<Impl: IListViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetContentMargin<Impl: IListViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Thickness) -> ::windows::core::HRESULT {
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
pub trait IListViewItemPresenter2_Impl: Sized {
    fn SelectedPressedBackground(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetSelectedPressedBackground(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn PressedBackground(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetPressedBackground(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckBoxBrush(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckBoxBrush(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn FocusSecondaryBorderBrush(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetFocusSecondaryBorderBrush(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckMode(&mut self) -> ::windows::core::Result<ListViewItemPresenterCheckMode>;
    fn SetCheckMode(&mut self, value: ListViewItemPresenterCheckMode) -> ::windows::core::Result<()>;
    fn PointerOverForeground(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetPointerOverForeground(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IListViewItemPresenter2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IListViewItemPresenter2";
}
#[cfg(all(feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
impl IListViewItemPresenter2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewItemPresenter2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListViewItemPresenter2_Vtbl {
        unsafe extern "system" fn SelectedPressedBackground<Impl: IListViewItemPresenter2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSelectedPressedBackground<Impl: IListViewItemPresenter2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectedPressedBackground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PressedBackground<Impl: IListViewItemPresenter2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPressedBackground<Impl: IListViewItemPresenter2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPressedBackground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckBoxBrush<Impl: IListViewItemPresenter2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCheckBoxBrush<Impl: IListViewItemPresenter2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCheckBoxBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FocusSecondaryBorderBrush<Impl: IListViewItemPresenter2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetFocusSecondaryBorderBrush<Impl: IListViewItemPresenter2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFocusSecondaryBorderBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckMode<Impl: IListViewItemPresenter2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ListViewItemPresenterCheckMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCheckMode<Impl: IListViewItemPresenter2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ListViewItemPresenterCheckMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCheckMode(value).into()
        }
        unsafe extern "system" fn PointerOverForeground<Impl: IListViewItemPresenter2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPointerOverForeground<Impl: IListViewItemPresenter2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IListViewItemPresenter3_Impl: Sized {
    fn RevealBackground(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetRevealBackground(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn RevealBorderBrush(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetRevealBorderBrush(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn RevealBorderThickness(&mut self) -> ::windows::core::Result<super::super::Thickness>;
    fn SetRevealBorderThickness(&mut self, value: &super::super::Thickness) -> ::windows::core::Result<()>;
    fn RevealBackgroundShowsAboveContent(&mut self) -> ::windows::core::Result<bool>;
    fn SetRevealBackgroundShowsAboveContent(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IListViewItemPresenter3 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IListViewItemPresenter3";
}
#[cfg(all(feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
impl IListViewItemPresenter3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewItemPresenter3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListViewItemPresenter3_Vtbl {
        unsafe extern "system" fn RevealBackground<Impl: IListViewItemPresenter3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRevealBackground<Impl: IListViewItemPresenter3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRevealBackground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RevealBorderBrush<Impl: IListViewItemPresenter3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRevealBorderBrush<Impl: IListViewItemPresenter3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRevealBorderBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RevealBorderThickness<Impl: IListViewItemPresenter3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRevealBorderThickness<Impl: IListViewItemPresenter3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Thickness) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRevealBorderThickness(&*(&value as *const <super::super::Thickness as ::windows::core::Abi>::Abi as *const <super::super::Thickness as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RevealBackgroundShowsAboveContent<Impl: IListViewItemPresenter3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRevealBackgroundShowsAboveContent<Impl: IListViewItemPresenter3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
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
pub trait IListViewItemPresenter4_Impl: Sized {
    fn SelectedDisabledBackground(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetSelectedDisabledBackground(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckPressedBrush(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckPressedBrush(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckDisabledBrush(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckDisabledBrush(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckBoxPointerOverBrush(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckBoxPointerOverBrush(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckBoxPressedBrush(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckBoxPressedBrush(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckBoxDisabledBrush(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckBoxDisabledBrush(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckBoxSelectedBrush(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckBoxSelectedBrush(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckBoxSelectedPointerOverBrush(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckBoxSelectedPointerOverBrush(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckBoxSelectedPressedBrush(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckBoxSelectedPressedBrush(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckBoxSelectedDisabledBrush(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckBoxSelectedDisabledBrush(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckBoxBorderBrush(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckBoxBorderBrush(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckBoxPointerOverBorderBrush(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckBoxPointerOverBorderBrush(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckBoxPressedBorderBrush(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckBoxPressedBorderBrush(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckBoxDisabledBorderBrush(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetCheckBoxDisabledBorderBrush(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn CheckBoxCornerRadius(&mut self) -> ::windows::core::Result<super::super::CornerRadius>;
    fn SetCheckBoxCornerRadius(&mut self, value: &super::super::CornerRadius) -> ::windows::core::Result<()>;
    fn SelectionIndicatorCornerRadius(&mut self) -> ::windows::core::Result<super::super::CornerRadius>;
    fn SetSelectionIndicatorCornerRadius(&mut self, value: &super::super::CornerRadius) -> ::windows::core::Result<()>;
    fn SelectionIndicatorVisualEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetSelectionIndicatorVisualEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn SelectionIndicatorMode(&mut self) -> ::windows::core::Result<ListViewItemPresenterSelectionIndicatorMode>;
    fn SetSelectionIndicatorMode(&mut self, value: ListViewItemPresenterSelectionIndicatorMode) -> ::windows::core::Result<()>;
    fn SelectionIndicatorBrush(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetSelectionIndicatorBrush(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectionIndicatorPointerOverBrush(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetSelectionIndicatorPointerOverBrush(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectionIndicatorPressedBrush(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetSelectionIndicatorPressedBrush(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectionIndicatorDisabledBrush(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetSelectionIndicatorDisabledBrush(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectedBorderBrush(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetSelectedBorderBrush(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectedPressedBorderBrush(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetSelectedPressedBorderBrush(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectedDisabledBorderBrush(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetSelectedDisabledBorderBrush(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn SelectedInnerBorderBrush(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetSelectedInnerBorderBrush(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
    fn PointerOverBorderBrush(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetPointerOverBorderBrush(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IListViewItemPresenter4 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IListViewItemPresenter4";
}
#[cfg(all(feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
impl IListViewItemPresenter4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewItemPresenter4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListViewItemPresenter4_Vtbl {
        unsafe extern "system" fn SelectedDisabledBackground<Impl: IListViewItemPresenter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSelectedDisabledBackground<Impl: IListViewItemPresenter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectedDisabledBackground(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckPressedBrush<Impl: IListViewItemPresenter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCheckPressedBrush<Impl: IListViewItemPresenter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCheckPressedBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckDisabledBrush<Impl: IListViewItemPresenter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCheckDisabledBrush<Impl: IListViewItemPresenter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCheckDisabledBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckBoxPointerOverBrush<Impl: IListViewItemPresenter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCheckBoxPointerOverBrush<Impl: IListViewItemPresenter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCheckBoxPointerOverBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckBoxPressedBrush<Impl: IListViewItemPresenter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCheckBoxPressedBrush<Impl: IListViewItemPresenter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCheckBoxPressedBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckBoxDisabledBrush<Impl: IListViewItemPresenter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCheckBoxDisabledBrush<Impl: IListViewItemPresenter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCheckBoxDisabledBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckBoxSelectedBrush<Impl: IListViewItemPresenter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCheckBoxSelectedBrush<Impl: IListViewItemPresenter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCheckBoxSelectedBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckBoxSelectedPointerOverBrush<Impl: IListViewItemPresenter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCheckBoxSelectedPointerOverBrush<Impl: IListViewItemPresenter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCheckBoxSelectedPointerOverBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckBoxSelectedPressedBrush<Impl: IListViewItemPresenter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCheckBoxSelectedPressedBrush<Impl: IListViewItemPresenter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCheckBoxSelectedPressedBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckBoxSelectedDisabledBrush<Impl: IListViewItemPresenter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCheckBoxSelectedDisabledBrush<Impl: IListViewItemPresenter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCheckBoxSelectedDisabledBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckBoxBorderBrush<Impl: IListViewItemPresenter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCheckBoxBorderBrush<Impl: IListViewItemPresenter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCheckBoxBorderBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckBoxPointerOverBorderBrush<Impl: IListViewItemPresenter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCheckBoxPointerOverBorderBrush<Impl: IListViewItemPresenter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCheckBoxPointerOverBorderBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckBoxPressedBorderBrush<Impl: IListViewItemPresenter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCheckBoxPressedBorderBrush<Impl: IListViewItemPresenter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCheckBoxPressedBorderBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckBoxDisabledBorderBrush<Impl: IListViewItemPresenter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCheckBoxDisabledBorderBrush<Impl: IListViewItemPresenter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCheckBoxDisabledBorderBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CheckBoxCornerRadius<Impl: IListViewItemPresenter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::CornerRadius) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCheckBoxCornerRadius<Impl: IListViewItemPresenter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::CornerRadius) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCheckBoxCornerRadius(&*(&value as *const <super::super::CornerRadius as ::windows::core::Abi>::Abi as *const <super::super::CornerRadius as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectionIndicatorCornerRadius<Impl: IListViewItemPresenter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::CornerRadius) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSelectionIndicatorCornerRadius<Impl: IListViewItemPresenter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::CornerRadius) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectionIndicatorCornerRadius(&*(&value as *const <super::super::CornerRadius as ::windows::core::Abi>::Abi as *const <super::super::CornerRadius as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectionIndicatorVisualEnabled<Impl: IListViewItemPresenter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSelectionIndicatorVisualEnabled<Impl: IListViewItemPresenter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectionIndicatorVisualEnabled(value).into()
        }
        unsafe extern "system" fn SelectionIndicatorMode<Impl: IListViewItemPresenter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ListViewItemPresenterSelectionIndicatorMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSelectionIndicatorMode<Impl: IListViewItemPresenter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ListViewItemPresenterSelectionIndicatorMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectionIndicatorMode(value).into()
        }
        unsafe extern "system" fn SelectionIndicatorBrush<Impl: IListViewItemPresenter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSelectionIndicatorBrush<Impl: IListViewItemPresenter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectionIndicatorBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectionIndicatorPointerOverBrush<Impl: IListViewItemPresenter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSelectionIndicatorPointerOverBrush<Impl: IListViewItemPresenter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectionIndicatorPointerOverBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectionIndicatorPressedBrush<Impl: IListViewItemPresenter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSelectionIndicatorPressedBrush<Impl: IListViewItemPresenter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectionIndicatorPressedBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectionIndicatorDisabledBrush<Impl: IListViewItemPresenter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSelectionIndicatorDisabledBrush<Impl: IListViewItemPresenter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectionIndicatorDisabledBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedBorderBrush<Impl: IListViewItemPresenter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSelectedBorderBrush<Impl: IListViewItemPresenter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectedBorderBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedPressedBorderBrush<Impl: IListViewItemPresenter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSelectedPressedBorderBrush<Impl: IListViewItemPresenter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectedPressedBorderBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedDisabledBorderBrush<Impl: IListViewItemPresenter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSelectedDisabledBorderBrush<Impl: IListViewItemPresenter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectedDisabledBorderBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedInnerBorderBrush<Impl: IListViewItemPresenter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSelectedInnerBorderBrush<Impl: IListViewItemPresenter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectedInnerBorderBrush(&*(&value as *const <super::super::Media::Brush as ::windows::core::Abi>::Abi as *const <super::super::Media::Brush as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PointerOverBorderBrush<Impl: IListViewItemPresenter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPointerOverBorderBrush<Impl: IListViewItemPresenter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IListViewItemPresenterFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ListViewItemPresenter>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListViewItemPresenterFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IListViewItemPresenterFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IListViewItemPresenterFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewItemPresenterFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListViewItemPresenterFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IListViewItemPresenterFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IListViewItemPresenterStatics_Impl: Sized {
    fn SelectionCheckMarkVisualEnabledProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckHintBrushProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckSelectingBrushProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckBrushProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn DragBackgroundProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn DragForegroundProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn FocusBorderBrushProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn PlaceholderBackgroundProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn PointerOverBackgroundProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedBackgroundProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedForegroundProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedPointerOverBackgroundProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedPointerOverBorderBrushProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedBorderThicknessProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn DisabledOpacityProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn DragOpacityProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ReorderHintOffsetProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ListViewItemPresenterHorizontalContentAlignmentProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ListViewItemPresenterVerticalContentAlignmentProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ListViewItemPresenterPaddingProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn PointerOverBackgroundMarginProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ContentMarginProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListViewItemPresenterStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IListViewItemPresenterStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IListViewItemPresenterStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewItemPresenterStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListViewItemPresenterStatics_Vtbl {
        unsafe extern "system" fn SelectionCheckMarkVisualEnabledProperty<Impl: IListViewItemPresenterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CheckHintBrushProperty<Impl: IListViewItemPresenterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CheckSelectingBrushProperty<Impl: IListViewItemPresenterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CheckBrushProperty<Impl: IListViewItemPresenterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DragBackgroundProperty<Impl: IListViewItemPresenterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DragForegroundProperty<Impl: IListViewItemPresenterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FocusBorderBrushProperty<Impl: IListViewItemPresenterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PlaceholderBackgroundProperty<Impl: IListViewItemPresenterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PointerOverBackgroundProperty<Impl: IListViewItemPresenterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SelectedBackgroundProperty<Impl: IListViewItemPresenterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SelectedForegroundProperty<Impl: IListViewItemPresenterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SelectedPointerOverBackgroundProperty<Impl: IListViewItemPresenterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SelectedPointerOverBorderBrushProperty<Impl: IListViewItemPresenterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SelectedBorderThicknessProperty<Impl: IListViewItemPresenterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DisabledOpacityProperty<Impl: IListViewItemPresenterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DragOpacityProperty<Impl: IListViewItemPresenterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReorderHintOffsetProperty<Impl: IListViewItemPresenterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ListViewItemPresenterHorizontalContentAlignmentProperty<Impl: IListViewItemPresenterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ListViewItemPresenterVerticalContentAlignmentProperty<Impl: IListViewItemPresenterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ListViewItemPresenterPaddingProperty<Impl: IListViewItemPresenterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PointerOverBackgroundMarginProperty<Impl: IListViewItemPresenterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ContentMarginProperty<Impl: IListViewItemPresenterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IListViewItemPresenterStatics2_Impl: Sized {
    fn SelectedPressedBackgroundProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn PressedBackgroundProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckBoxBrushProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn FocusSecondaryBorderBrushProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckModeProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn PointerOverForegroundProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListViewItemPresenterStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IListViewItemPresenterStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IListViewItemPresenterStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewItemPresenterStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListViewItemPresenterStatics2_Vtbl {
        unsafe extern "system" fn SelectedPressedBackgroundProperty<Impl: IListViewItemPresenterStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PressedBackgroundProperty<Impl: IListViewItemPresenterStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CheckBoxBrushProperty<Impl: IListViewItemPresenterStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FocusSecondaryBorderBrushProperty<Impl: IListViewItemPresenterStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CheckModeProperty<Impl: IListViewItemPresenterStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PointerOverForegroundProperty<Impl: IListViewItemPresenterStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IListViewItemPresenterStatics3_Impl: Sized {
    fn RevealBackgroundProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn RevealBorderBrushProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn RevealBorderThicknessProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn RevealBackgroundShowsAboveContentProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListViewItemPresenterStatics3 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IListViewItemPresenterStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IListViewItemPresenterStatics3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewItemPresenterStatics3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListViewItemPresenterStatics3_Vtbl {
        unsafe extern "system" fn RevealBackgroundProperty<Impl: IListViewItemPresenterStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RevealBorderBrushProperty<Impl: IListViewItemPresenterStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RevealBorderThicknessProperty<Impl: IListViewItemPresenterStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RevealBackgroundShowsAboveContentProperty<Impl: IListViewItemPresenterStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IListViewItemPresenterStatics4_Impl: Sized {
    fn SelectedDisabledBackgroundProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckPressedBrushProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckDisabledBrushProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckBoxPointerOverBrushProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckBoxPressedBrushProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckBoxDisabledBrushProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckBoxSelectedBrushProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckBoxSelectedPointerOverBrushProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckBoxSelectedPressedBrushProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckBoxSelectedDisabledBrushProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckBoxBorderBrushProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckBoxPointerOverBorderBrushProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckBoxPressedBorderBrushProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckBoxDisabledBorderBrushProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn CheckBoxCornerRadiusProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectionIndicatorCornerRadiusProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectionIndicatorVisualEnabledProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectionIndicatorModeProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectionIndicatorBrushProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectionIndicatorPointerOverBrushProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectionIndicatorPressedBrushProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectionIndicatorDisabledBrushProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedBorderBrushProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedPressedBorderBrushProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedDisabledBorderBrushProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedInnerBorderBrushProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn PointerOverBorderBrushProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListViewItemPresenterStatics4 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IListViewItemPresenterStatics4";
}
#[cfg(feature = "implement_exclusive")]
impl IListViewItemPresenterStatics4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewItemPresenterStatics4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListViewItemPresenterStatics4_Vtbl {
        unsafe extern "system" fn SelectedDisabledBackgroundProperty<Impl: IListViewItemPresenterStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CheckPressedBrushProperty<Impl: IListViewItemPresenterStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CheckDisabledBrushProperty<Impl: IListViewItemPresenterStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CheckBoxPointerOverBrushProperty<Impl: IListViewItemPresenterStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CheckBoxPressedBrushProperty<Impl: IListViewItemPresenterStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CheckBoxDisabledBrushProperty<Impl: IListViewItemPresenterStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CheckBoxSelectedBrushProperty<Impl: IListViewItemPresenterStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CheckBoxSelectedPointerOverBrushProperty<Impl: IListViewItemPresenterStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CheckBoxSelectedPressedBrushProperty<Impl: IListViewItemPresenterStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CheckBoxSelectedDisabledBrushProperty<Impl: IListViewItemPresenterStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CheckBoxBorderBrushProperty<Impl: IListViewItemPresenterStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CheckBoxPointerOverBorderBrushProperty<Impl: IListViewItemPresenterStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CheckBoxPressedBorderBrushProperty<Impl: IListViewItemPresenterStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CheckBoxDisabledBorderBrushProperty<Impl: IListViewItemPresenterStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CheckBoxCornerRadiusProperty<Impl: IListViewItemPresenterStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SelectionIndicatorCornerRadiusProperty<Impl: IListViewItemPresenterStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SelectionIndicatorVisualEnabledProperty<Impl: IListViewItemPresenterStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SelectionIndicatorModeProperty<Impl: IListViewItemPresenterStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SelectionIndicatorBrushProperty<Impl: IListViewItemPresenterStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SelectionIndicatorPointerOverBrushProperty<Impl: IListViewItemPresenterStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SelectionIndicatorPressedBrushProperty<Impl: IListViewItemPresenterStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SelectionIndicatorDisabledBrushProperty<Impl: IListViewItemPresenterStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SelectedBorderBrushProperty<Impl: IListViewItemPresenterStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SelectedPressedBorderBrushProperty<Impl: IListViewItemPresenterStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SelectedDisabledBorderBrushProperty<Impl: IListViewItemPresenterStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SelectedInnerBorderBrushProperty<Impl: IListViewItemPresenterStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PointerOverBorderBrushProperty<Impl: IListViewItemPresenterStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IListViewItemTemplateSettings_Impl: Sized {
    fn DragItemsCount(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IListViewItemTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IListViewItemTemplateSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IListViewItemTemplateSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IListViewItemTemplateSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IListViewItemTemplateSettings_Vtbl {
        unsafe extern "system" fn DragItemsCount<Impl: IListViewItemTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
pub trait ILoopingSelector_Impl: Sized {
    fn ShouldLoop(&mut self) -> ::windows::core::Result<bool>;
    fn SetShouldLoop(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Items(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVector<::windows::core::IInspectable>>;
    fn SetItems(&mut self, value: &::core::option::Option<super::super::super::super::Foundation::Collections::IVector<::windows::core::IInspectable>>) -> ::windows::core::Result<()>;
    fn SelectedIndex(&mut self) -> ::windows::core::Result<i32>;
    fn SetSelectedIndex(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn SelectedItem(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetSelectedItem(&mut self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn ItemWidth(&mut self) -> ::windows::core::Result<i32>;
    fn SetItemWidth(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn ItemHeight(&mut self) -> ::windows::core::Result<i32>;
    fn SetItemHeight(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn ItemTemplate(&mut self) -> ::windows::core::Result<super::super::DataTemplate>;
    fn SetItemTemplate(&mut self, value: &::core::option::Option<super::super::DataTemplate>) -> ::windows::core::Result<()>;
    fn SelectionChanged(&mut self, handler: &::core::option::Option<super::SelectionChangedEventHandler>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSelectionChanged(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ILoopingSelector {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ILoopingSelector";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ILoopingSelector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoopingSelector_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILoopingSelector_Vtbl {
        unsafe extern "system" fn ShouldLoop<Impl: ILoopingSelector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetShouldLoop<Impl: ILoopingSelector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShouldLoop(value).into()
        }
        unsafe extern "system" fn Items<Impl: ILoopingSelector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetItems<Impl: ILoopingSelector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetItems(&*(&value as *const <super::super::super::super::Foundation::Collections::IVector<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::Collections::IVector<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedIndex<Impl: ILoopingSelector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSelectedIndex<Impl: ILoopingSelector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectedIndex(value).into()
        }
        unsafe extern "system" fn SelectedItem<Impl: ILoopingSelector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSelectedItem<Impl: ILoopingSelector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectedItem(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ItemWidth<Impl: ILoopingSelector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetItemWidth<Impl: ILoopingSelector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetItemWidth(value).into()
        }
        unsafe extern "system" fn ItemHeight<Impl: ILoopingSelector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetItemHeight<Impl: ILoopingSelector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetItemHeight(value).into()
        }
        unsafe extern "system" fn ItemTemplate<Impl: ILoopingSelector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetItemTemplate<Impl: ILoopingSelector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetItemTemplate(&*(&value as *const <super::super::DataTemplate as ::windows::core::Abi>::Abi as *const <super::super::DataTemplate as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectionChanged<Impl: ILoopingSelector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveSelectionChanged<Impl: ILoopingSelector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
pub trait ILoopingSelectorItem_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILoopingSelectorItem {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ILoopingSelectorItem";
}
#[cfg(feature = "implement_exclusive")]
impl ILoopingSelectorItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoopingSelectorItem_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILoopingSelectorItem_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ILoopingSelectorItem, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoopingSelectorItem as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoopingSelectorPanel_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILoopingSelectorPanel {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ILoopingSelectorPanel";
}
#[cfg(feature = "implement_exclusive")]
impl ILoopingSelectorPanel_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoopingSelectorPanel_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILoopingSelectorPanel_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ILoopingSelectorPanel, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILoopingSelectorPanel as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoopingSelectorStatics_Impl: Sized {
    fn ShouldLoopProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ItemsProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedIndexProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedItemProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ItemWidthProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ItemHeightProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ItemTemplateProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILoopingSelectorStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ILoopingSelectorStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ILoopingSelectorStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILoopingSelectorStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILoopingSelectorStatics_Vtbl {
        unsafe extern "system" fn ShouldLoopProperty<Impl: ILoopingSelectorStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ItemsProperty<Impl: ILoopingSelectorStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SelectedIndexProperty<Impl: ILoopingSelectorStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SelectedItemProperty<Impl: ILoopingSelectorStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ItemWidthProperty<Impl: ILoopingSelectorStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ItemHeightProperty<Impl: ILoopingSelectorStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ItemTemplateProperty<Impl: ILoopingSelectorStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IMenuFlyoutItemTemplateSettings_Impl: Sized {
    fn KeyboardAcceleratorTextMinWidth(&mut self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMenuFlyoutItemTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IMenuFlyoutItemTemplateSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IMenuFlyoutItemTemplateSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMenuFlyoutItemTemplateSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMenuFlyoutItemTemplateSettings_Vtbl {
        unsafe extern "system" fn KeyboardAcceleratorTextMinWidth<Impl: IMenuFlyoutItemTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
pub trait IMenuFlyoutPresenterTemplateSettings_Impl: Sized {
    fn FlyoutContentMinWidth(&mut self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMenuFlyoutPresenterTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IMenuFlyoutPresenterTemplateSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IMenuFlyoutPresenterTemplateSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMenuFlyoutPresenterTemplateSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMenuFlyoutPresenterTemplateSettings_Vtbl {
        unsafe extern "system" fn FlyoutContentMinWidth<Impl: IMenuFlyoutPresenterTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
pub trait INavigationViewItemPresenter_Impl: Sized {
    fn Icon(&mut self) -> ::windows::core::Result<super::IconElement>;
    fn SetIcon(&mut self, value: &::core::option::Option<super::IconElement>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INavigationViewItemPresenter {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.INavigationViewItemPresenter";
}
#[cfg(feature = "implement_exclusive")]
impl INavigationViewItemPresenter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INavigationViewItemPresenter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INavigationViewItemPresenter_Vtbl {
        unsafe extern "system" fn Icon<Impl: INavigationViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIcon<Impl: INavigationViewItemPresenter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait INavigationViewItemPresenterFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<NavigationViewItemPresenter>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INavigationViewItemPresenterFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.INavigationViewItemPresenterFactory";
}
#[cfg(feature = "implement_exclusive")]
impl INavigationViewItemPresenterFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INavigationViewItemPresenterFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INavigationViewItemPresenterFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: INavigationViewItemPresenterFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait INavigationViewItemPresenterStatics_Impl: Sized {
    fn IconProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INavigationViewItemPresenterStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.INavigationViewItemPresenterStatics";
}
#[cfg(feature = "implement_exclusive")]
impl INavigationViewItemPresenterStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INavigationViewItemPresenterStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INavigationViewItemPresenterStatics_Vtbl {
        unsafe extern "system" fn IconProperty<Impl: INavigationViewItemPresenterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IOrientedVirtualizingPanel_Impl: Sized {
    fn CanVerticallyScroll(&mut self) -> ::windows::core::Result<bool>;
    fn SetCanVerticallyScroll(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn CanHorizontallyScroll(&mut self) -> ::windows::core::Result<bool>;
    fn SetCanHorizontallyScroll(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn ExtentWidth(&mut self) -> ::windows::core::Result<f64>;
    fn ExtentHeight(&mut self) -> ::windows::core::Result<f64>;
    fn ViewportWidth(&mut self) -> ::windows::core::Result<f64>;
    fn ViewportHeight(&mut self) -> ::windows::core::Result<f64>;
    fn HorizontalOffset(&mut self) -> ::windows::core::Result<f64>;
    fn VerticalOffset(&mut self) -> ::windows::core::Result<f64>;
    fn ScrollOwner(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetScrollOwner(&mut self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn LineUp(&mut self) -> ::windows::core::Result<()>;
    fn LineDown(&mut self) -> ::windows::core::Result<()>;
    fn LineLeft(&mut self) -> ::windows::core::Result<()>;
    fn LineRight(&mut self) -> ::windows::core::Result<()>;
    fn PageUp(&mut self) -> ::windows::core::Result<()>;
    fn PageDown(&mut self) -> ::windows::core::Result<()>;
    fn PageLeft(&mut self) -> ::windows::core::Result<()>;
    fn PageRight(&mut self) -> ::windows::core::Result<()>;
    fn MouseWheelUp(&mut self) -> ::windows::core::Result<()>;
    fn MouseWheelDown(&mut self) -> ::windows::core::Result<()>;
    fn MouseWheelLeft(&mut self) -> ::windows::core::Result<()>;
    fn MouseWheelRight(&mut self) -> ::windows::core::Result<()>;
    fn SetHorizontalOffset(&mut self, offset: f64) -> ::windows::core::Result<()>;
    fn SetVerticalOffset(&mut self, offset: f64) -> ::windows::core::Result<()>;
    fn MakeVisible(&mut self, visual: &::core::option::Option<super::super::UIElement>, rectangle: &super::super::super::super::Foundation::Rect) -> ::windows::core::Result<super::super::super::super::Foundation::Rect>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IOrientedVirtualizingPanel {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IOrientedVirtualizingPanel";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IOrientedVirtualizingPanel_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOrientedVirtualizingPanel_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOrientedVirtualizingPanel_Vtbl {
        unsafe extern "system" fn CanVerticallyScroll<Impl: IOrientedVirtualizingPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCanVerticallyScroll<Impl: IOrientedVirtualizingPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCanVerticallyScroll(value).into()
        }
        unsafe extern "system" fn CanHorizontallyScroll<Impl: IOrientedVirtualizingPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCanHorizontallyScroll<Impl: IOrientedVirtualizingPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCanHorizontallyScroll(value).into()
        }
        unsafe extern "system" fn ExtentWidth<Impl: IOrientedVirtualizingPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExtentHeight<Impl: IOrientedVirtualizingPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ViewportWidth<Impl: IOrientedVirtualizingPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ViewportHeight<Impl: IOrientedVirtualizingPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HorizontalOffset<Impl: IOrientedVirtualizingPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn VerticalOffset<Impl: IOrientedVirtualizingPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ScrollOwner<Impl: IOrientedVirtualizingPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetScrollOwner<Impl: IOrientedVirtualizingPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScrollOwner(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LineUp<Impl: IOrientedVirtualizingPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LineUp().into()
        }
        unsafe extern "system" fn LineDown<Impl: IOrientedVirtualizingPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LineDown().into()
        }
        unsafe extern "system" fn LineLeft<Impl: IOrientedVirtualizingPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LineLeft().into()
        }
        unsafe extern "system" fn LineRight<Impl: IOrientedVirtualizingPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LineRight().into()
        }
        unsafe extern "system" fn PageUp<Impl: IOrientedVirtualizingPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PageUp().into()
        }
        unsafe extern "system" fn PageDown<Impl: IOrientedVirtualizingPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PageDown().into()
        }
        unsafe extern "system" fn PageLeft<Impl: IOrientedVirtualizingPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PageLeft().into()
        }
        unsafe extern "system" fn PageRight<Impl: IOrientedVirtualizingPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PageRight().into()
        }
        unsafe extern "system" fn MouseWheelUp<Impl: IOrientedVirtualizingPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MouseWheelUp().into()
        }
        unsafe extern "system" fn MouseWheelDown<Impl: IOrientedVirtualizingPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MouseWheelDown().into()
        }
        unsafe extern "system" fn MouseWheelLeft<Impl: IOrientedVirtualizingPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MouseWheelLeft().into()
        }
        unsafe extern "system" fn MouseWheelRight<Impl: IOrientedVirtualizingPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MouseWheelRight().into()
        }
        unsafe extern "system" fn SetHorizontalOffset<Impl: IOrientedVirtualizingPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHorizontalOffset(offset).into()
        }
        unsafe extern "system" fn SetVerticalOffset<Impl: IOrientedVirtualizingPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, offset: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVerticalOffset(offset).into()
        }
        unsafe extern "system" fn MakeVisible<Impl: IOrientedVirtualizingPanel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, visual: ::windows::core::RawPtr, rectangle: super::super::super::super::Foundation::Rect, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
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
pub trait IOrientedVirtualizingPanelFactory_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IOrientedVirtualizingPanelFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IOrientedVirtualizingPanelFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IOrientedVirtualizingPanelFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOrientedVirtualizingPanelFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOrientedVirtualizingPanelFactory_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IOrientedVirtualizingPanelFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOrientedVirtualizingPanelFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPickerFlyoutBase_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPickerFlyoutBase {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IPickerFlyoutBase";
}
#[cfg(feature = "implement_exclusive")]
impl IPickerFlyoutBase_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPickerFlyoutBase_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPickerFlyoutBase_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPickerFlyoutBase, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPickerFlyoutBase as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPickerFlyoutBaseFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<PickerFlyoutBase>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPickerFlyoutBaseFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IPickerFlyoutBaseFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPickerFlyoutBaseFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPickerFlyoutBaseFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPickerFlyoutBaseFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IPickerFlyoutBaseFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IPickerFlyoutBaseOverrides_Impl: Sized {
    fn OnConfirmed(&mut self) -> ::windows::core::Result<()>;
    fn ShouldShowConfirmationButtons(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPickerFlyoutBaseOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IPickerFlyoutBaseOverrides";
}
#[cfg(feature = "implement_exclusive")]
impl IPickerFlyoutBaseOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPickerFlyoutBaseOverrides_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPickerFlyoutBaseOverrides_Vtbl {
        unsafe extern "system" fn OnConfirmed<Impl: IPickerFlyoutBaseOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnConfirmed().into()
        }
        unsafe extern "system" fn ShouldShowConfirmationButtons<Impl: IPickerFlyoutBaseOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
pub trait IPickerFlyoutBaseStatics_Impl: Sized {
    fn TitleProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn GetTitle(&mut self, element: &::core::option::Option<super::super::DependencyObject>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitle(&mut self, element: &::core::option::Option<super::super::DependencyObject>, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPickerFlyoutBaseStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IPickerFlyoutBaseStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPickerFlyoutBaseStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPickerFlyoutBaseStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPickerFlyoutBaseStatics_Vtbl {
        unsafe extern "system" fn TitleProperty<Impl: IPickerFlyoutBaseStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetTitle<Impl: IPickerFlyoutBaseStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTitle<Impl: IPickerFlyoutBaseStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
pub trait IPivotHeaderItem_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPivotHeaderItem {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IPivotHeaderItem";
}
#[cfg(feature = "implement_exclusive")]
impl IPivotHeaderItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPivotHeaderItem_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPivotHeaderItem_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPivotHeaderItem, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPivotHeaderItem as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPivotHeaderItemFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<PivotHeaderItem>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPivotHeaderItemFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IPivotHeaderItemFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPivotHeaderItemFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPivotHeaderItemFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPivotHeaderItemFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IPivotHeaderItemFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IPivotHeaderPanel_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPivotHeaderPanel {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IPivotHeaderPanel";
}
#[cfg(feature = "implement_exclusive")]
impl IPivotHeaderPanel_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPivotHeaderPanel_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPivotHeaderPanel_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPivotHeaderPanel, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPivotHeaderPanel as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPivotPanel_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPivotPanel {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IPivotPanel";
}
#[cfg(feature = "implement_exclusive")]
impl IPivotPanel_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPivotPanel_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPivotPanel_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPivotPanel, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPivotPanel as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation", feature = "implement_exclusive"))]
pub trait IPopup_Impl: Sized {
    fn Child(&mut self) -> ::windows::core::Result<super::super::UIElement>;
    fn SetChild(&mut self, value: &::core::option::Option<super::super::UIElement>) -> ::windows::core::Result<()>;
    fn IsOpen(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsOpen(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn HorizontalOffset(&mut self) -> ::windows::core::Result<f64>;
    fn SetHorizontalOffset(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn VerticalOffset(&mut self) -> ::windows::core::Result<f64>;
    fn SetVerticalOffset(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn ChildTransitions(&mut self) -> ::windows::core::Result<super::super::Media::Animation::TransitionCollection>;
    fn SetChildTransitions(&mut self, value: &::core::option::Option<super::super::Media::Animation::TransitionCollection>) -> ::windows::core::Result<()>;
    fn IsLightDismissEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsLightDismissEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Opened(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveOpened(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Closed(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPopup {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IPopup";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "UI_Xaml_Media_Animation", feature = "implement_exclusive"))]
impl IPopup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPopup_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPopup_Vtbl {
        unsafe extern "system" fn Child<Impl: IPopup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetChild<Impl: IPopup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetChild(&*(&value as *const <super::super::UIElement as ::windows::core::Abi>::Abi as *const <super::super::UIElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsOpen<Impl: IPopup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsOpen<Impl: IPopup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsOpen(value).into()
        }
        unsafe extern "system" fn HorizontalOffset<Impl: IPopup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHorizontalOffset<Impl: IPopup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHorizontalOffset(value).into()
        }
        unsafe extern "system" fn VerticalOffset<Impl: IPopup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetVerticalOffset<Impl: IPopup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVerticalOffset(value).into()
        }
        unsafe extern "system" fn ChildTransitions<Impl: IPopup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetChildTransitions<Impl: IPopup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetChildTransitions(&*(&value as *const <super::super::Media::Animation::TransitionCollection as ::windows::core::Abi>::Abi as *const <super::super::Media::Animation::TransitionCollection as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsLightDismissEnabled<Impl: IPopup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsLightDismissEnabled<Impl: IPopup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsLightDismissEnabled(value).into()
        }
        unsafe extern "system" fn Opened<Impl: IPopup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveOpened<Impl: IPopup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveOpened(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Closed<Impl: IPopup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveClosed<Impl: IPopup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
pub trait IPopup2_Impl: Sized {
    fn LightDismissOverlayMode(&mut self) -> ::windows::core::Result<super::LightDismissOverlayMode>;
    fn SetLightDismissOverlayMode(&mut self, value: super::LightDismissOverlayMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPopup2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IPopup2";
}
#[cfg(feature = "implement_exclusive")]
impl IPopup2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPopup2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPopup2_Vtbl {
        unsafe extern "system" fn LightDismissOverlayMode<Impl: IPopup2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::LightDismissOverlayMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetLightDismissOverlayMode<Impl: IPopup2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::LightDismissOverlayMode) -> ::windows::core::HRESULT {
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
pub trait IPopup3_Impl: Sized {
    fn ShouldConstrainToRootBounds(&mut self) -> ::windows::core::Result<bool>;
    fn SetShouldConstrainToRootBounds(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsConstrainedToRootBounds(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPopup3 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IPopup3";
}
#[cfg(feature = "implement_exclusive")]
impl IPopup3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPopup3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPopup3_Vtbl {
        unsafe extern "system" fn ShouldConstrainToRootBounds<Impl: IPopup3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetShouldConstrainToRootBounds<Impl: IPopup3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetShouldConstrainToRootBounds(value).into()
        }
        unsafe extern "system" fn IsConstrainedToRootBounds<Impl: IPopup3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
pub trait IPopup4_Impl: Sized {
    fn PlacementTarget(&mut self) -> ::windows::core::Result<super::super::FrameworkElement>;
    fn SetPlacementTarget(&mut self, value: &::core::option::Option<super::super::FrameworkElement>) -> ::windows::core::Result<()>;
    fn DesiredPlacement(&mut self) -> ::windows::core::Result<PopupPlacementMode>;
    fn SetDesiredPlacement(&mut self, value: PopupPlacementMode) -> ::windows::core::Result<()>;
    fn ActualPlacement(&mut self) -> ::windows::core::Result<PopupPlacementMode>;
    fn ActualPlacementChanged(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveActualPlacementChanged(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPopup4 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IPopup4";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IPopup4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPopup4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPopup4_Vtbl {
        unsafe extern "system" fn PlacementTarget<Impl: IPopup4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPlacementTarget<Impl: IPopup4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPlacementTarget(&*(&value as *const <super::super::FrameworkElement as ::windows::core::Abi>::Abi as *const <super::super::FrameworkElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DesiredPlacement<Impl: IPopup4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PopupPlacementMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDesiredPlacement<Impl: IPopup4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: PopupPlacementMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDesiredPlacement(value).into()
        }
        unsafe extern "system" fn ActualPlacement<Impl: IPopup4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PopupPlacementMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ActualPlacementChanged<Impl: IPopup4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveActualPlacementChanged<Impl: IPopup4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
pub trait IPopupStatics_Impl: Sized {
    fn ChildProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn IsOpenProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn HorizontalOffsetProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn VerticalOffsetProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ChildTransitionsProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn IsLightDismissEnabledProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPopupStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IPopupStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPopupStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPopupStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPopupStatics_Vtbl {
        unsafe extern "system" fn ChildProperty<Impl: IPopupStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsOpenProperty<Impl: IPopupStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HorizontalOffsetProperty<Impl: IPopupStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn VerticalOffsetProperty<Impl: IPopupStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ChildTransitionsProperty<Impl: IPopupStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsLightDismissEnabledProperty<Impl: IPopupStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IPopupStatics2_Impl: Sized {
    fn LightDismissOverlayModeProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPopupStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IPopupStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IPopupStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPopupStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPopupStatics2_Vtbl {
        unsafe extern "system" fn LightDismissOverlayModeProperty<Impl: IPopupStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IPopupStatics3_Impl: Sized {
    fn ShouldConstrainToRootBoundsProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPopupStatics3 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IPopupStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IPopupStatics3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPopupStatics3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPopupStatics3_Vtbl {
        unsafe extern "system" fn ShouldConstrainToRootBoundsProperty<Impl: IPopupStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IPopupStatics4_Impl: Sized {
    fn PlacementTargetProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn DesiredPlacementProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPopupStatics4 {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IPopupStatics4";
}
#[cfg(feature = "implement_exclusive")]
impl IPopupStatics4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPopupStatics4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPopupStatics4_Vtbl {
        unsafe extern "system" fn PlacementTargetProperty<Impl: IPopupStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DesiredPlacementProperty<Impl: IPopupStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IProgressBarTemplateSettings_Impl: Sized {
    fn EllipseDiameter(&mut self) -> ::windows::core::Result<f64>;
    fn EllipseOffset(&mut self) -> ::windows::core::Result<f64>;
    fn EllipseAnimationWellPosition(&mut self) -> ::windows::core::Result<f64>;
    fn EllipseAnimationEndPosition(&mut self) -> ::windows::core::Result<f64>;
    fn ContainerAnimationStartPosition(&mut self) -> ::windows::core::Result<f64>;
    fn ContainerAnimationEndPosition(&mut self) -> ::windows::core::Result<f64>;
    fn IndicatorLengthDelta(&mut self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProgressBarTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IProgressBarTemplateSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IProgressBarTemplateSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProgressBarTemplateSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProgressBarTemplateSettings_Vtbl {
        unsafe extern "system" fn EllipseDiameter<Impl: IProgressBarTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn EllipseOffset<Impl: IProgressBarTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn EllipseAnimationWellPosition<Impl: IProgressBarTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn EllipseAnimationEndPosition<Impl: IProgressBarTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ContainerAnimationStartPosition<Impl: IProgressBarTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ContainerAnimationEndPosition<Impl: IProgressBarTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IndicatorLengthDelta<Impl: IProgressBarTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
pub trait IProgressRingTemplateSettings_Impl: Sized {
    fn EllipseDiameter(&mut self) -> ::windows::core::Result<f64>;
    fn EllipseOffset(&mut self) -> ::windows::core::Result<super::super::Thickness>;
    fn MaxSideLength(&mut self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProgressRingTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IProgressRingTemplateSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IProgressRingTemplateSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProgressRingTemplateSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProgressRingTemplateSettings_Vtbl {
        unsafe extern "system" fn EllipseDiameter<Impl: IProgressRingTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn EllipseOffset<Impl: IProgressRingTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MaxSideLength<Impl: IProgressRingTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
pub trait IRangeBase_Impl: Sized {
    fn Minimum(&mut self) -> ::windows::core::Result<f64>;
    fn SetMinimum(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn Maximum(&mut self) -> ::windows::core::Result<f64>;
    fn SetMaximum(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn SmallChange(&mut self) -> ::windows::core::Result<f64>;
    fn SetSmallChange(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn LargeChange(&mut self) -> ::windows::core::Result<f64>;
    fn SetLargeChange(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn Value(&mut self) -> ::windows::core::Result<f64>;
    fn SetValue(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn ValueChanged(&mut self, handler: &::core::option::Option<RangeBaseValueChangedEventHandler>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveValueChanged(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRangeBase {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IRangeBase";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IRangeBase_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRangeBase_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRangeBase_Vtbl {
        unsafe extern "system" fn Minimum<Impl: IRangeBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMinimum<Impl: IRangeBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMinimum(value).into()
        }
        unsafe extern "system" fn Maximum<Impl: IRangeBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMaximum<Impl: IRangeBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaximum(value).into()
        }
        unsafe extern "system" fn SmallChange<Impl: IRangeBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSmallChange<Impl: IRangeBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSmallChange(value).into()
        }
        unsafe extern "system" fn LargeChange<Impl: IRangeBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetLargeChange<Impl: IRangeBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLargeChange(value).into()
        }
        unsafe extern "system" fn Value<Impl: IRangeBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetValue<Impl: IRangeBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(value).into()
        }
        unsafe extern "system" fn ValueChanged<Impl: IRangeBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveValueChanged<Impl: IRangeBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
pub trait IRangeBaseFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<RangeBase>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRangeBaseFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IRangeBaseFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IRangeBaseFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRangeBaseFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRangeBaseFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IRangeBaseFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IRangeBaseOverrides_Impl: Sized {
    fn OnMinimumChanged(&mut self, oldminimum: f64, newminimum: f64) -> ::windows::core::Result<()>;
    fn OnMaximumChanged(&mut self, oldmaximum: f64, newmaximum: f64) -> ::windows::core::Result<()>;
    fn OnValueChanged(&mut self, oldvalue: f64, newvalue: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRangeBaseOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IRangeBaseOverrides";
}
#[cfg(feature = "implement_exclusive")]
impl IRangeBaseOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRangeBaseOverrides_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRangeBaseOverrides_Vtbl {
        unsafe extern "system" fn OnMinimumChanged<Impl: IRangeBaseOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldminimum: f64, newminimum: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnMinimumChanged(oldminimum, newminimum).into()
        }
        unsafe extern "system" fn OnMaximumChanged<Impl: IRangeBaseOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldmaximum: f64, newmaximum: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnMaximumChanged(oldmaximum, newmaximum).into()
        }
        unsafe extern "system" fn OnValueChanged<Impl: IRangeBaseOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, oldvalue: f64, newvalue: f64) -> ::windows::core::HRESULT {
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
pub trait IRangeBaseStatics_Impl: Sized {
    fn MinimumProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn MaximumProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SmallChangeProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn LargeChangeProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ValueProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRangeBaseStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IRangeBaseStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IRangeBaseStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRangeBaseStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRangeBaseStatics_Vtbl {
        unsafe extern "system" fn MinimumProperty<Impl: IRangeBaseStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MaximumProperty<Impl: IRangeBaseStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SmallChangeProperty<Impl: IRangeBaseStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LargeChangeProperty<Impl: IRangeBaseStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ValueProperty<Impl: IRangeBaseStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IRangeBaseValueChangedEventArgs_Impl: Sized {
    fn OldValue(&mut self) -> ::windows::core::Result<f64>;
    fn NewValue(&mut self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRangeBaseValueChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IRangeBaseValueChangedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRangeBaseValueChangedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRangeBaseValueChangedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRangeBaseValueChangedEventArgs_Vtbl {
        unsafe extern "system" fn OldValue<Impl: IRangeBaseValueChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NewValue<Impl: IRangeBaseValueChangedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
pub trait IRepeatButton_Impl: Sized {
    fn Delay(&mut self) -> ::windows::core::Result<i32>;
    fn SetDelay(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn Interval(&mut self) -> ::windows::core::Result<i32>;
    fn SetInterval(&mut self, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRepeatButton {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IRepeatButton";
}
#[cfg(feature = "implement_exclusive")]
impl IRepeatButton_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRepeatButton_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRepeatButton_Vtbl {
        unsafe extern "system" fn Delay<Impl: IRepeatButton_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDelay<Impl: IRepeatButton_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDelay(value).into()
        }
        unsafe extern "system" fn Interval<Impl: IRepeatButton_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetInterval<Impl: IRepeatButton_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
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
pub trait IRepeatButtonStatics_Impl: Sized {
    fn DelayProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn IntervalProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRepeatButtonStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IRepeatButtonStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IRepeatButtonStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRepeatButtonStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRepeatButtonStatics_Vtbl {
        unsafe extern "system" fn DelayProperty<Impl: IRepeatButtonStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IntervalProperty<Impl: IRepeatButtonStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IScrollBar_Impl: Sized {
    fn Orientation(&mut self) -> ::windows::core::Result<super::Orientation>;
    fn SetOrientation(&mut self, value: super::Orientation) -> ::windows::core::Result<()>;
    fn ViewportSize(&mut self) -> ::windows::core::Result<f64>;
    fn SetViewportSize(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn IndicatorMode(&mut self) -> ::windows::core::Result<ScrollingIndicatorMode>;
    fn SetIndicatorMode(&mut self, value: ScrollingIndicatorMode) -> ::windows::core::Result<()>;
    fn Scroll(&mut self, handler: &::core::option::Option<ScrollEventHandler>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveScroll(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IScrollBar {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IScrollBar";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IScrollBar_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScrollBar_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScrollBar_Vtbl {
        unsafe extern "system" fn Orientation<Impl: IScrollBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::Orientation) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOrientation<Impl: IScrollBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::Orientation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOrientation(value).into()
        }
        unsafe extern "system" fn ViewportSize<Impl: IScrollBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetViewportSize<Impl: IScrollBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetViewportSize(value).into()
        }
        unsafe extern "system" fn IndicatorMode<Impl: IScrollBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ScrollingIndicatorMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIndicatorMode<Impl: IScrollBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ScrollingIndicatorMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIndicatorMode(value).into()
        }
        unsafe extern "system" fn Scroll<Impl: IScrollBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveScroll<Impl: IScrollBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
pub trait IScrollBarStatics_Impl: Sized {
    fn OrientationProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn ViewportSizeProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn IndicatorModeProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IScrollBarStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IScrollBarStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IScrollBarStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScrollBarStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScrollBarStatics_Vtbl {
        unsafe extern "system" fn OrientationProperty<Impl: IScrollBarStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ViewportSizeProperty<Impl: IScrollBarStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IndicatorModeProperty<Impl: IScrollBarStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IScrollEventArgs_Impl: Sized {
    fn NewValue(&mut self) -> ::windows::core::Result<f64>;
    fn ScrollEventType(&mut self) -> ::windows::core::Result<ScrollEventType>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IScrollEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IScrollEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IScrollEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScrollEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScrollEventArgs_Vtbl {
        unsafe extern "system" fn NewValue<Impl: IScrollEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ScrollEventType<Impl: IScrollEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ScrollEventType) -> ::windows::core::HRESULT {
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
pub trait IScrollSnapPointsInfo_Impl: Sized {
    fn AreHorizontalSnapPointsRegular(&mut self) -> ::windows::core::Result<bool>;
    fn AreVerticalSnapPointsRegular(&mut self) -> ::windows::core::Result<bool>;
    fn HorizontalSnapPointsChanged(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveHorizontalSnapPointsChanged(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn VerticalSnapPointsChanged(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveVerticalSnapPointsChanged(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetIrregularSnapPoints(&mut self, orientation: super::Orientation, alignment: SnapPointsAlignment) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<f32>>;
    fn GetRegularSnapPoints(&mut self, orientation: super::Orientation, alignment: SnapPointsAlignment, offset: &mut f32) -> ::windows::core::Result<f32>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
impl ::windows::core::RuntimeName for IScrollSnapPointsInfo {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IScrollSnapPointsInfo";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
impl IScrollSnapPointsInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IScrollSnapPointsInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IScrollSnapPointsInfo_Vtbl {
        unsafe extern "system" fn AreHorizontalSnapPointsRegular<Impl: IScrollSnapPointsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AreVerticalSnapPointsRegular<Impl: IScrollSnapPointsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HorizontalSnapPointsChanged<Impl: IScrollSnapPointsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveHorizontalSnapPointsChanged<Impl: IScrollSnapPointsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveHorizontalSnapPointsChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn VerticalSnapPointsChanged<Impl: IScrollSnapPointsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveVerticalSnapPointsChanged<Impl: IScrollSnapPointsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveVerticalSnapPointsChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetIrregularSnapPoints<Impl: IScrollSnapPointsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, orientation: super::Orientation, alignment: SnapPointsAlignment, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetRegularSnapPoints<Impl: IScrollSnapPointsInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, orientation: super::Orientation, alignment: SnapPointsAlignment, offset: *mut f32, result__: *mut f32) -> ::windows::core::HRESULT {
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
pub trait ISelector_Impl: Sized {
    fn SelectedIndex(&mut self) -> ::windows::core::Result<i32>;
    fn SetSelectedIndex(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn SelectedItem(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetSelectedItem(&mut self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn SelectedValue(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetSelectedValue(&mut self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn SelectedValuePath(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSelectedValuePath(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsSynchronizedWithCurrentItem(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<bool>>;
    fn SetIsSynchronizedWithCurrentItem(&mut self, value: &::core::option::Option<super::super::super::super::Foundation::IReference<bool>>) -> ::windows::core::Result<()>;
    fn SelectionChanged(&mut self, handler: &::core::option::Option<super::SelectionChangedEventHandler>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSelectionChanged(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISelector {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ISelector";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ISelector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelector_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISelector_Vtbl {
        unsafe extern "system" fn SelectedIndex<Impl: ISelector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSelectedIndex<Impl: ISelector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectedIndex(value).into()
        }
        unsafe extern "system" fn SelectedItem<Impl: ISelector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSelectedItem<Impl: ISelector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectedItem(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedValue<Impl: ISelector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSelectedValue<Impl: ISelector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectedValue(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectedValuePath<Impl: ISelector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSelectedValuePath<Impl: ISelector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelectedValuePath(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsSynchronizedWithCurrentItem<Impl: ISelector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsSynchronizedWithCurrentItem<Impl: ISelector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsSynchronizedWithCurrentItem(&*(&value as *const <super::super::super::super::Foundation::IReference<bool> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::IReference<bool> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SelectionChanged<Impl: ISelector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveSelectionChanged<Impl: ISelector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
pub trait ISelectorFactory_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISelectorFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ISelectorFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ISelectorFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectorFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISelectorFactory_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISelectorFactory, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISelectorFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISelectorItem_Impl: Sized {
    fn IsSelected(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsSelected(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISelectorItem {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ISelectorItem";
}
#[cfg(feature = "implement_exclusive")]
impl ISelectorItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectorItem_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISelectorItem_Vtbl {
        unsafe extern "system" fn IsSelected<Impl: ISelectorItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsSelected<Impl: ISelectorItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
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
pub trait ISelectorItemFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<SelectorItem>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISelectorItemFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ISelectorItemFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ISelectorItemFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectorItemFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISelectorItemFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: ISelectorItemFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ISelectorItemStatics_Impl: Sized {
    fn IsSelectedProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISelectorItemStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ISelectorItemStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISelectorItemStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectorItemStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISelectorItemStatics_Vtbl {
        unsafe extern "system" fn IsSelectedProperty<Impl: ISelectorItemStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ISelectorStatics_Impl: Sized {
    fn SelectedIndexProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedItemProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedValueProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn SelectedValuePathProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn IsSynchronizedWithCurrentItemProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn GetIsSelectionActive(&mut self, element: &::core::option::Option<super::super::DependencyObject>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISelectorStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ISelectorStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISelectorStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectorStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISelectorStatics_Vtbl {
        unsafe extern "system" fn SelectedIndexProperty<Impl: ISelectorStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SelectedItemProperty<Impl: ISelectorStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SelectedValueProperty<Impl: ISelectorStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SelectedValuePathProperty<Impl: ISelectorStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsSynchronizedWithCurrentItemProperty<Impl: ISelectorStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetIsSelectionActive<Impl: ISelectorStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
pub trait ISettingsFlyoutTemplateSettings_Impl: Sized {
    fn HeaderBackground(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn HeaderForeground(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn BorderBrush(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn BorderThickness(&mut self) -> ::windows::core::Result<super::super::Thickness>;
    fn IconSource(&mut self) -> ::windows::core::Result<super::super::Media::ImageSource>;
    fn ContentTransitions(&mut self) -> ::windows::core::Result<super::super::Media::Animation::TransitionCollection>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media", feature = "UI_Xaml_Media_Animation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISettingsFlyoutTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ISettingsFlyoutTemplateSettings";
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Media", feature = "UI_Xaml_Media_Animation", feature = "implement_exclusive"))]
impl ISettingsFlyoutTemplateSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISettingsFlyoutTemplateSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISettingsFlyoutTemplateSettings_Vtbl {
        unsafe extern "system" fn HeaderBackground<Impl: ISettingsFlyoutTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HeaderForeground<Impl: ISettingsFlyoutTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn BorderBrush<Impl: ISettingsFlyoutTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn BorderThickness<Impl: ISettingsFlyoutTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Thickness) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IconSource<Impl: ISettingsFlyoutTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ContentTransitions<Impl: ISettingsFlyoutTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ISplitViewTemplateSettings_Impl: Sized {
    fn OpenPaneLength(&mut self) -> ::windows::core::Result<f64>;
    fn NegativeOpenPaneLength(&mut self) -> ::windows::core::Result<f64>;
    fn OpenPaneLengthMinusCompactLength(&mut self) -> ::windows::core::Result<f64>;
    fn NegativeOpenPaneLengthMinusCompactLength(&mut self) -> ::windows::core::Result<f64>;
    fn OpenPaneGridLength(&mut self) -> ::windows::core::Result<super::super::GridLength>;
    fn CompactPaneGridLength(&mut self) -> ::windows::core::Result<super::super::GridLength>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISplitViewTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ISplitViewTemplateSettings";
}
#[cfg(feature = "implement_exclusive")]
impl ISplitViewTemplateSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISplitViewTemplateSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISplitViewTemplateSettings_Vtbl {
        unsafe extern "system" fn OpenPaneLength<Impl: ISplitViewTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NegativeOpenPaneLength<Impl: ISplitViewTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OpenPaneLengthMinusCompactLength<Impl: ISplitViewTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NegativeOpenPaneLengthMinusCompactLength<Impl: ISplitViewTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OpenPaneGridLength<Impl: ISplitViewTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::GridLength) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CompactPaneGridLength<Impl: ISplitViewTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::GridLength) -> ::windows::core::HRESULT {
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
pub trait IThumb_Impl: Sized {
    fn IsDragging(&mut self) -> ::windows::core::Result<bool>;
    fn DragStarted(&mut self, handler: &::core::option::Option<DragStartedEventHandler>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDragStarted(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DragDelta(&mut self, handler: &::core::option::Option<DragDeltaEventHandler>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDragDelta(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DragCompleted(&mut self, handler: &::core::option::Option<DragCompletedEventHandler>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDragCompleted(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CancelDrag(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IThumb {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IThumb";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IThumb_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IThumb_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IThumb_Vtbl {
        unsafe extern "system" fn IsDragging<Impl: IThumb_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DragStarted<Impl: IThumb_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveDragStarted<Impl: IThumb_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDragStarted(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DragDelta<Impl: IThumb_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveDragDelta<Impl: IThumb_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDragDelta(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DragCompleted<Impl: IThumb_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveDragCompleted<Impl: IThumb_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDragCompleted(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CancelDrag<Impl: IThumb_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IThumbStatics_Impl: Sized {
    fn IsDraggingProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IThumbStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IThumbStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IThumbStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IThumbStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IThumbStatics_Vtbl {
        unsafe extern "system" fn IsDraggingProperty<Impl: IThumbStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ITickBar_Impl: Sized {
    fn Fill(&mut self) -> ::windows::core::Result<super::super::Media::Brush>;
    fn SetFill(&mut self, value: &::core::option::Option<super::super::Media::Brush>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITickBar {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ITickBar";
}
#[cfg(all(feature = "UI_Xaml_Media", feature = "implement_exclusive"))]
impl ITickBar_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITickBar_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITickBar_Vtbl {
        unsafe extern "system" fn Fill<Impl: ITickBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetFill<Impl: ITickBar_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ITickBarStatics_Impl: Sized {
    fn FillProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITickBarStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.ITickBarStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ITickBarStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITickBarStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITickBarStatics_Vtbl {
        unsafe extern "system" fn FillProperty<Impl: ITickBarStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IToggleButton_Impl: Sized {
    fn IsChecked(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::IReference<bool>>;
    fn SetIsChecked(&mut self, value: &::core::option::Option<super::super::super::super::Foundation::IReference<bool>>) -> ::windows::core::Result<()>;
    fn IsThreeState(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsThreeState(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Checked(&mut self, handler: &::core::option::Option<super::super::RoutedEventHandler>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveChecked(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Unchecked(&mut self, handler: &::core::option::Option<super::super::RoutedEventHandler>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveUnchecked(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Indeterminate(&mut self, handler: &::core::option::Option<super::super::RoutedEventHandler>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveIndeterminate(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IToggleButton {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IToggleButton";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IToggleButton_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToggleButton_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToggleButton_Vtbl {
        unsafe extern "system" fn IsChecked<Impl: IToggleButton_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsChecked<Impl: IToggleButton_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsChecked(&*(&value as *const <super::super::super::super::Foundation::IReference<bool> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::IReference<bool> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsThreeState<Impl: IToggleButton_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsThreeState<Impl: IToggleButton_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsThreeState(value).into()
        }
        unsafe extern "system" fn Checked<Impl: IToggleButton_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveChecked<Impl: IToggleButton_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveChecked(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Unchecked<Impl: IToggleButton_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveUnchecked<Impl: IToggleButton_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveUnchecked(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Indeterminate<Impl: IToggleButton_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveIndeterminate<Impl: IToggleButton_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
pub trait IToggleButtonFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ToggleButton>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToggleButtonFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IToggleButtonFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IToggleButtonFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToggleButtonFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToggleButtonFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IToggleButtonFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IToggleButtonOverrides_Impl: Sized {
    fn OnToggle(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToggleButtonOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IToggleButtonOverrides";
}
#[cfg(feature = "implement_exclusive")]
impl IToggleButtonOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToggleButtonOverrides_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToggleButtonOverrides_Vtbl {
        unsafe extern "system" fn OnToggle<Impl: IToggleButtonOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IToggleButtonStatics_Impl: Sized {
    fn IsCheckedProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
    fn IsThreeStateProperty(&mut self) -> ::windows::core::Result<super::super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToggleButtonStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IToggleButtonStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IToggleButtonStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToggleButtonStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToggleButtonStatics_Vtbl {
        unsafe extern "system" fn IsCheckedProperty<Impl: IToggleButtonStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsThreeStateProperty<Impl: IToggleButtonStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IToggleSwitchTemplateSettings_Impl: Sized {
    fn KnobCurrentToOnOffset(&mut self) -> ::windows::core::Result<f64>;
    fn KnobCurrentToOffOffset(&mut self) -> ::windows::core::Result<f64>;
    fn KnobOnToOffOffset(&mut self) -> ::windows::core::Result<f64>;
    fn KnobOffToOnOffset(&mut self) -> ::windows::core::Result<f64>;
    fn CurtainCurrentToOnOffset(&mut self) -> ::windows::core::Result<f64>;
    fn CurtainCurrentToOffOffset(&mut self) -> ::windows::core::Result<f64>;
    fn CurtainOnToOffOffset(&mut self) -> ::windows::core::Result<f64>;
    fn CurtainOffToOnOffset(&mut self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToggleSwitchTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IToggleSwitchTemplateSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IToggleSwitchTemplateSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToggleSwitchTemplateSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToggleSwitchTemplateSettings_Vtbl {
        unsafe extern "system" fn KnobCurrentToOnOffset<Impl: IToggleSwitchTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn KnobCurrentToOffOffset<Impl: IToggleSwitchTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn KnobOnToOffOffset<Impl: IToggleSwitchTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn KnobOffToOnOffset<Impl: IToggleSwitchTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CurtainCurrentToOnOffset<Impl: IToggleSwitchTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CurtainCurrentToOffOffset<Impl: IToggleSwitchTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CurtainOnToOffOffset<Impl: IToggleSwitchTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CurtainOffToOnOffset<Impl: IToggleSwitchTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
pub trait IToolTipTemplateSettings_Impl: Sized {
    fn FromHorizontalOffset(&mut self) -> ::windows::core::Result<f64>;
    fn FromVerticalOffset(&mut self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IToolTipTemplateSettings {
    const NAME: &'static str = "Windows.UI.Xaml.Controls.Primitives.IToolTipTemplateSettings";
}
#[cfg(feature = "implement_exclusive")]
impl IToolTipTemplateSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IToolTipTemplateSettings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IToolTipTemplateSettings_Vtbl {
        unsafe extern "system" fn FromHorizontalOffset<Impl: IToolTipTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FromVerticalOffset<Impl: IToolTipTemplateSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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

#[cfg(feature = "implement_exclusive")]
pub trait IAccessKeyDisplayDismissedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAccessKeyDisplayDismissedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IAccessKeyDisplayDismissedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAccessKeyDisplayDismissedEventArgsVtbl {
    pub const fn new<Impl: IAccessKeyDisplayDismissedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAccessKeyDisplayDismissedEventArgsVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAccessKeyDisplayDismissedEventArgs>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccessKeyDisplayRequestedEventArgsImpl: Sized {
    fn PressedKeys(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAccessKeyDisplayRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IAccessKeyDisplayRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAccessKeyDisplayRequestedEventArgsVtbl {
    pub const fn new<Impl: IAccessKeyDisplayRequestedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAccessKeyDisplayRequestedEventArgsVtbl {
        unsafe extern "system" fn PressedKeys<Impl: IAccessKeyDisplayRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PressedKeys() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAccessKeyDisplayRequestedEventArgs>, base.5, PressedKeys::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccessKeyInvokedEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAccessKeyInvokedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IAccessKeyInvokedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAccessKeyInvokedEventArgsVtbl {
    pub const fn new<Impl: IAccessKeyInvokedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAccessKeyInvokedEventArgsVtbl {
        unsafe extern "system" fn Handled<Impl: IAccessKeyInvokedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IAccessKeyInvokedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAccessKeyInvokedEventArgs>, base.5, Handled::<Impl, OFFSET>, SetHandled::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccessKeyManagerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAccessKeyManager {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IAccessKeyManager";
}
#[cfg(feature = "implement_exclusive")]
impl IAccessKeyManagerVtbl {
    pub const fn new<Impl: IAccessKeyManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAccessKeyManagerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAccessKeyManager>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccessKeyManagerStaticsImpl: Sized {
    fn IsDisplayModeEnabled(&self) -> ::windows::core::Result<bool>;
    fn IsDisplayModeEnabledChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveIsDisplayModeEnabledChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ExitDisplayMode(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAccessKeyManagerStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IAccessKeyManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAccessKeyManagerStaticsVtbl {
    pub const fn new<Impl: IAccessKeyManagerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAccessKeyManagerStaticsVtbl {
        unsafe extern "system" fn IsDisplayModeEnabled<Impl: IAccessKeyManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsDisplayModeEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDisplayModeEnabledChanged<Impl: IAccessKeyManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsDisplayModeEnabledChanged(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveIsDisplayModeEnabledChanged<Impl: IAccessKeyManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveIsDisplayModeEnabledChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ExitDisplayMode<Impl: IAccessKeyManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ExitDisplayMode().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAccessKeyManagerStatics>, base.5, IsDisplayModeEnabled::<Impl, OFFSET>, IsDisplayModeEnabledChanged::<Impl, OFFSET>, RemoveIsDisplayModeEnabledChanged::<Impl, OFFSET>, ExitDisplayMode::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccessKeyManagerStatics2Impl: Sized {
    fn AreKeyTipsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetAreKeyTipsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAccessKeyManagerStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IAccessKeyManagerStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IAccessKeyManagerStatics2Vtbl {
    pub const fn new<Impl: IAccessKeyManagerStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAccessKeyManagerStatics2Vtbl {
        unsafe extern "system" fn AreKeyTipsEnabled<Impl: IAccessKeyManagerStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AreKeyTipsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAreKeyTipsEnabled<Impl: IAccessKeyManagerStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAreKeyTipsEnabled(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAccessKeyManagerStatics2>, base.5, AreKeyTipsEnabled::<Impl, OFFSET>, SetAreKeyTipsEnabled::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICanExecuteRequestedEventArgsImpl: Sized {
    fn Parameter(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CanExecute(&self) -> ::windows::core::Result<bool>;
    fn SetCanExecute(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICanExecuteRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.ICanExecuteRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ICanExecuteRequestedEventArgsVtbl {
    pub const fn new<Impl: ICanExecuteRequestedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICanExecuteRequestedEventArgsVtbl {
        unsafe extern "system" fn Parameter<Impl: ICanExecuteRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Parameter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanExecute<Impl: ICanExecuteRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanExecute() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCanExecute<Impl: ICanExecuteRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCanExecute(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICanExecuteRequestedEventArgs>, base.5, Parameter::<Impl, OFFSET>, CanExecute::<Impl, OFFSET>, SetCanExecute::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICharacterReceivedRoutedEventArgsImpl: Sized {
    fn Character(&self) -> ::windows::core::Result<u16>;
    fn KeyStatus(&self) -> ::windows::core::Result<super::super::Core::CorePhysicalKeyStatus>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICharacterReceivedRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.ICharacterReceivedRoutedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ICharacterReceivedRoutedEventArgsVtbl {
    pub const fn new<Impl: ICharacterReceivedRoutedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICharacterReceivedRoutedEventArgsVtbl {
        unsafe extern "system" fn Character<Impl: ICharacterReceivedRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Character() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyStatus<Impl: ICharacterReceivedRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Core::CorePhysicalKeyStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handled<Impl: ICharacterReceivedRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: ICharacterReceivedRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICharacterReceivedRoutedEventArgs>, base.5, Character::<Impl, OFFSET>, KeyStatus::<Impl, OFFSET>, Handled::<Impl, OFFSET>, SetHandled::<Impl, OFFSET>)
    }
}
pub trait ICommandImpl: Sized {
    fn CanExecuteChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCanExecuteChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CanExecute(&self, parameter: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<bool>;
    fn Execute(&self, parameter: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICommand {
    const NAME: &'static str = "Windows.UI.Xaml.Input.ICommand";
}
impl ICommandVtbl {
    pub const fn new<Impl: ICommandImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICommandVtbl {
        unsafe extern "system" fn CanExecuteChanged<Impl: ICommandImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanExecuteChanged(&*(&handler as *const <super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCanExecuteChanged<Impl: ICommandImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveCanExecuteChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CanExecute<Impl: ICommandImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parameter: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanExecute(&*(&parameter as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Execute<Impl: ICommandImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parameter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Execute(&*(&parameter as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICommand>, base.5, CanExecuteChanged::<Impl, OFFSET>, RemoveCanExecuteChanged::<Impl, OFFSET>, CanExecute::<Impl, OFFSET>, Execute::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IContextRequestedEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn TryGetPosition(&self, relativeto: &::core::option::Option<super::UIElement>, point: &mut super::super::super::Foundation::Point) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IContextRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IContextRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IContextRequestedEventArgsVtbl {
    pub const fn new<Impl: IContextRequestedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IContextRequestedEventArgsVtbl {
        unsafe extern "system" fn Handled<Impl: IContextRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IContextRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn TryGetPosition<Impl: IContextRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relativeto: ::windows::core::RawPtr, point: *mut super::super::super::Foundation::Point, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryGetPosition(&*(&relativeto as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&point)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IContextRequestedEventArgs>, base.5, Handled::<Impl, OFFSET>, SetHandled::<Impl, OFFSET>, TryGetPosition::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDoubleTappedRoutedEventArgsImpl: Sized {
    fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::super::Devices::Input::PointerDeviceType>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn GetPosition(&self, relativeto: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<super::super::super::Foundation::Point>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDoubleTappedRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IDoubleTappedRoutedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IDoubleTappedRoutedEventArgsVtbl {
    pub const fn new<Impl: IDoubleTappedRoutedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDoubleTappedRoutedEventArgsVtbl {
        unsafe extern "system" fn PointerDeviceType<Impl: IDoubleTappedRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointerDeviceType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handled<Impl: IDoubleTappedRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IDoubleTappedRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn GetPosition<Impl: IDoubleTappedRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relativeto: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPosition(&*(&relativeto as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDoubleTappedRoutedEventArgs>, base.5, PointerDeviceType::<Impl, OFFSET>, Handled::<Impl, OFFSET>, SetHandled::<Impl, OFFSET>, GetPosition::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IExecuteRequestedEventArgsImpl: Sized {
    fn Parameter(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IExecuteRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IExecuteRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IExecuteRequestedEventArgsVtbl {
    pub const fn new<Impl: IExecuteRequestedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IExecuteRequestedEventArgsVtbl {
        unsafe extern "system" fn Parameter<Impl: IExecuteRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Parameter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IExecuteRequestedEventArgs>, base.5, Parameter::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFindNextElementOptionsImpl: Sized {
    fn SearchRoot(&self) -> ::windows::core::Result<super::DependencyObject>;
    fn SetSearchRoot(&self, value: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<()>;
    fn ExclusionRect(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn SetExclusionRect(&self, value: &super::super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn HintRect(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn SetHintRect(&self, value: &super::super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn XYFocusNavigationStrategyOverride(&self) -> ::windows::core::Result<XYFocusNavigationStrategyOverride>;
    fn SetXYFocusNavigationStrategyOverride(&self, value: XYFocusNavigationStrategyOverride) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFindNextElementOptions {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IFindNextElementOptions";
}
#[cfg(feature = "implement_exclusive")]
impl IFindNextElementOptionsVtbl {
    pub const fn new<Impl: IFindNextElementOptionsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFindNextElementOptionsVtbl {
        unsafe extern "system" fn SearchRoot<Impl: IFindNextElementOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SearchRoot() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSearchRoot<Impl: IFindNextElementOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetSearchRoot(&*(&value as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ExclusionRect<Impl: IFindNextElementOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetExclusionRect<Impl: IFindNextElementOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetExclusionRect(&*(&value as *const <super::super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn HintRect<Impl: IFindNextElementOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HintRect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHintRect<Impl: IFindNextElementOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHintRect(&*(&value as *const <super::super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn XYFocusNavigationStrategyOverride<Impl: IFindNextElementOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut XYFocusNavigationStrategyOverride) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).XYFocusNavigationStrategyOverride() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetXYFocusNavigationStrategyOverride<Impl: IFindNextElementOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: XYFocusNavigationStrategyOverride) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetXYFocusNavigationStrategyOverride(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFindNextElementOptions>, base.5, SearchRoot::<Impl, OFFSET>, SetSearchRoot::<Impl, OFFSET>, ExclusionRect::<Impl, OFFSET>, SetExclusionRect::<Impl, OFFSET>, HintRect::<Impl, OFFSET>, SetHintRect::<Impl, OFFSET>, XYFocusNavigationStrategyOverride::<Impl, OFFSET>, SetXYFocusNavigationStrategyOverride::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFocusManagerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFocusManager {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IFocusManager";
}
#[cfg(feature = "implement_exclusive")]
impl IFocusManagerVtbl {
    pub const fn new<Impl: IFocusManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFocusManagerVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFocusManager>, base.5)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFocusManagerGotFocusEventArgsImpl: Sized {
    fn NewFocusedElement(&self) -> ::windows::core::Result<super::DependencyObject>;
    fn CorrelationId(&self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFocusManagerGotFocusEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IFocusManagerGotFocusEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IFocusManagerGotFocusEventArgsVtbl {
    pub const fn new<Impl: IFocusManagerGotFocusEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFocusManagerGotFocusEventArgsVtbl {
        unsafe extern "system" fn NewFocusedElement<Impl: IFocusManagerGotFocusEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NewFocusedElement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CorrelationId<Impl: IFocusManagerGotFocusEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CorrelationId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFocusManagerGotFocusEventArgs>, base.5, NewFocusedElement::<Impl, OFFSET>, CorrelationId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFocusManagerLostFocusEventArgsImpl: Sized {
    fn OldFocusedElement(&self) -> ::windows::core::Result<super::DependencyObject>;
    fn CorrelationId(&self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFocusManagerLostFocusEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IFocusManagerLostFocusEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IFocusManagerLostFocusEventArgsVtbl {
    pub const fn new<Impl: IFocusManagerLostFocusEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFocusManagerLostFocusEventArgsVtbl {
        unsafe extern "system" fn OldFocusedElement<Impl: IFocusManagerLostFocusEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OldFocusedElement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CorrelationId<Impl: IFocusManagerLostFocusEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CorrelationId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFocusManagerLostFocusEventArgs>, base.5, OldFocusedElement::<Impl, OFFSET>, CorrelationId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFocusManagerStaticsImpl: Sized {
    fn GetFocusedElement(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFocusManagerStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IFocusManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IFocusManagerStaticsVtbl {
    pub const fn new<Impl: IFocusManagerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFocusManagerStaticsVtbl {
        unsafe extern "system" fn GetFocusedElement<Impl: IFocusManagerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFocusedElement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFocusManagerStatics>, base.5, GetFocusedElement::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFocusManagerStatics2Impl: Sized {
    fn TryMoveFocus(&self, focusnavigationdirection: FocusNavigationDirection) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFocusManagerStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IFocusManagerStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IFocusManagerStatics2Vtbl {
    pub const fn new<Impl: IFocusManagerStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFocusManagerStatics2Vtbl {
        unsafe extern "system" fn TryMoveFocus<Impl: IFocusManagerStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, focusnavigationdirection: FocusNavigationDirection, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryMoveFocus(focusnavigationdirection) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFocusManagerStatics2>, base.5, TryMoveFocus::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFocusManagerStatics3Impl: Sized {
    fn FindNextFocusableElement(&self, focusnavigationdirection: FocusNavigationDirection) -> ::windows::core::Result<super::UIElement>;
    fn FindNextFocusableElementWithHint(&self, focusnavigationdirection: FocusNavigationDirection, hintrect: &super::super::super::Foundation::Rect) -> ::windows::core::Result<super::UIElement>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFocusManagerStatics3 {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IFocusManagerStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IFocusManagerStatics3Vtbl {
    pub const fn new<Impl: IFocusManagerStatics3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFocusManagerStatics3Vtbl {
        unsafe extern "system" fn FindNextFocusableElement<Impl: IFocusManagerStatics3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, focusnavigationdirection: FocusNavigationDirection, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindNextFocusableElement(focusnavigationdirection) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindNextFocusableElementWithHint<Impl: IFocusManagerStatics3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, focusnavigationdirection: FocusNavigationDirection, hintrect: super::super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindNextFocusableElementWithHint(focusnavigationdirection, &*(&hintrect as *const <super::super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFocusManagerStatics3>, base.5, FindNextFocusableElement::<Impl, OFFSET>, FindNextFocusableElementWithHint::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFocusManagerStatics4Impl: Sized {
    fn TryMoveFocusWithOptions(&self, focusnavigationdirection: FocusNavigationDirection, focusnavigationoptions: &::core::option::Option<FindNextElementOptions>) -> ::windows::core::Result<bool>;
    fn FindNextElement(&self, focusnavigationdirection: FocusNavigationDirection) -> ::windows::core::Result<super::DependencyObject>;
    fn FindFirstFocusableElement(&self, searchscope: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<super::DependencyObject>;
    fn FindLastFocusableElement(&self, searchscope: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<super::DependencyObject>;
    fn FindNextElementWithOptions(&self, focusnavigationdirection: FocusNavigationDirection, focusnavigationoptions: &::core::option::Option<FindNextElementOptions>) -> ::windows::core::Result<super::DependencyObject>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFocusManagerStatics4 {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IFocusManagerStatics4";
}
#[cfg(feature = "implement_exclusive")]
impl IFocusManagerStatics4Vtbl {
    pub const fn new<Impl: IFocusManagerStatics4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFocusManagerStatics4Vtbl {
        unsafe extern "system" fn TryMoveFocusWithOptions<Impl: IFocusManagerStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, focusnavigationdirection: FocusNavigationDirection, focusnavigationoptions: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryMoveFocusWithOptions(focusnavigationdirection, &*(&focusnavigationoptions as *const <FindNextElementOptions as ::windows::core::Abi>::Abi as *const <FindNextElementOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindNextElement<Impl: IFocusManagerStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, focusnavigationdirection: FocusNavigationDirection, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindNextElement(focusnavigationdirection) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFirstFocusableElement<Impl: IFocusManagerStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, searchscope: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindFirstFocusableElement(&*(&searchscope as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindLastFocusableElement<Impl: IFocusManagerStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, searchscope: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindLastFocusableElement(&*(&searchscope as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindNextElementWithOptions<Impl: IFocusManagerStatics4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, focusnavigationdirection: FocusNavigationDirection, focusnavigationoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindNextElementWithOptions(focusnavigationdirection, &*(&focusnavigationoptions as *const <FindNextElementOptions as ::windows::core::Abi>::Abi as *const <FindNextElementOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFocusManagerStatics4>, base.5, TryMoveFocusWithOptions::<Impl, OFFSET>, FindNextElement::<Impl, OFFSET>, FindFirstFocusableElement::<Impl, OFFSET>, FindLastFocusableElement::<Impl, OFFSET>, FindNextElementWithOptions::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFocusManagerStatics5Impl: Sized {
    fn TryFocusAsync(&self, element: &::core::option::Option<super::DependencyObject>, value: super::FocusState) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<FocusMovementResult>>;
    fn TryMoveFocusAsync(&self, focusnavigationdirection: FocusNavigationDirection) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<FocusMovementResult>>;
    fn TryMoveFocusWithOptionsAsync(&self, focusnavigationdirection: FocusNavigationDirection, focusnavigationoptions: &::core::option::Option<FindNextElementOptions>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<FocusMovementResult>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFocusManagerStatics5 {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IFocusManagerStatics5";
}
#[cfg(feature = "implement_exclusive")]
impl IFocusManagerStatics5Vtbl {
    pub const fn new<Impl: IFocusManagerStatics5Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFocusManagerStatics5Vtbl {
        unsafe extern "system" fn TryFocusAsync<Impl: IFocusManagerStatics5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: super::FocusState, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryFocusAsync(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryMoveFocusAsync<Impl: IFocusManagerStatics5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, focusnavigationdirection: FocusNavigationDirection, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryMoveFocusAsync(focusnavigationdirection) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryMoveFocusWithOptionsAsync<Impl: IFocusManagerStatics5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, focusnavigationdirection: FocusNavigationDirection, focusnavigationoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryMoveFocusWithOptionsAsync(focusnavigationdirection, &*(&focusnavigationoptions as *const <FindNextElementOptions as ::windows::core::Abi>::Abi as *const <FindNextElementOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFocusManagerStatics5>, base.5, TryFocusAsync::<Impl, OFFSET>, TryMoveFocusAsync::<Impl, OFFSET>, TryMoveFocusWithOptionsAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFocusManagerStatics6Impl: Sized {
    fn GotFocus(&self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<FocusManagerGotFocusEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveGotFocus(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn LostFocus(&self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<FocusManagerLostFocusEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveLostFocus(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GettingFocus(&self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<GettingFocusEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveGettingFocus(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn LosingFocus(&self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<LosingFocusEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveLosingFocus(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFocusManagerStatics6 {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IFocusManagerStatics6";
}
#[cfg(feature = "implement_exclusive")]
impl IFocusManagerStatics6Vtbl {
    pub const fn new<Impl: IFocusManagerStatics6Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFocusManagerStatics6Vtbl {
        unsafe extern "system" fn GotFocus<Impl: IFocusManagerStatics6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GotFocus(&*(&handler as *const <super::super::super::Foundation::EventHandler<FocusManagerGotFocusEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventHandler<FocusManagerGotFocusEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveGotFocus<Impl: IFocusManagerStatics6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveGotFocus(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LostFocus<Impl: IFocusManagerStatics6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LostFocus(&*(&handler as *const <super::super::super::Foundation::EventHandler<FocusManagerLostFocusEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventHandler<FocusManagerLostFocusEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLostFocus<Impl: IFocusManagerStatics6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveLostFocus(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GettingFocus<Impl: IFocusManagerStatics6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GettingFocus(&*(&handler as *const <super::super::super::Foundation::EventHandler<GettingFocusEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventHandler<GettingFocusEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveGettingFocus<Impl: IFocusManagerStatics6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveGettingFocus(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LosingFocus<Impl: IFocusManagerStatics6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LosingFocus(&*(&handler as *const <super::super::super::Foundation::EventHandler<LosingFocusEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventHandler<LosingFocusEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLosingFocus<Impl: IFocusManagerStatics6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveLosingFocus(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFocusManagerStatics6>, base.5, GotFocus::<Impl, OFFSET>, RemoveGotFocus::<Impl, OFFSET>, LostFocus::<Impl, OFFSET>, RemoveLostFocus::<Impl, OFFSET>, GettingFocus::<Impl, OFFSET>, RemoveGettingFocus::<Impl, OFFSET>, LosingFocus::<Impl, OFFSET>, RemoveLosingFocus::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFocusManagerStatics7Impl: Sized {
    fn GetFocusedElement(&self, xamlroot: &::core::option::Option<super::XamlRoot>) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFocusManagerStatics7 {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IFocusManagerStatics7";
}
#[cfg(feature = "implement_exclusive")]
impl IFocusManagerStatics7Vtbl {
    pub const fn new<Impl: IFocusManagerStatics7Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFocusManagerStatics7Vtbl {
        unsafe extern "system" fn GetFocusedElement<Impl: IFocusManagerStatics7Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xamlroot: ::windows::core::RawPtr, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFocusedElement(&*(&xamlroot as *const <super::XamlRoot as ::windows::core::Abi>::Abi as *const <super::XamlRoot as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFocusManagerStatics7>, base.5, GetFocusedElement::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFocusMovementResultImpl: Sized {
    fn Succeeded(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFocusMovementResult {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IFocusMovementResult";
}
#[cfg(feature = "implement_exclusive")]
impl IFocusMovementResultVtbl {
    pub const fn new<Impl: IFocusMovementResultImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFocusMovementResultVtbl {
        unsafe extern "system" fn Succeeded<Impl: IFocusMovementResultImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Succeeded() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFocusMovementResult>, base.5, Succeeded::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGettingFocusEventArgsImpl: Sized {
    fn OldFocusedElement(&self) -> ::windows::core::Result<super::DependencyObject>;
    fn NewFocusedElement(&self) -> ::windows::core::Result<super::DependencyObject>;
    fn SetNewFocusedElement(&self, value: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<()>;
    fn FocusState(&self) -> ::windows::core::Result<super::FocusState>;
    fn Direction(&self) -> ::windows::core::Result<FocusNavigationDirection>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn InputDevice(&self) -> ::windows::core::Result<FocusInputDeviceKind>;
    fn Cancel(&self) -> ::windows::core::Result<bool>;
    fn SetCancel(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGettingFocusEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IGettingFocusEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IGettingFocusEventArgsVtbl {
    pub const fn new<Impl: IGettingFocusEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGettingFocusEventArgsVtbl {
        unsafe extern "system" fn OldFocusedElement<Impl: IGettingFocusEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OldFocusedElement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NewFocusedElement<Impl: IGettingFocusEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NewFocusedElement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNewFocusedElement<Impl: IGettingFocusEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetNewFocusedElement(&*(&value as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FocusState<Impl: IGettingFocusEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::FocusState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FocusState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Direction<Impl: IGettingFocusEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut FocusNavigationDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Direction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handled<Impl: IGettingFocusEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IGettingFocusEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn InputDevice<Impl: IGettingFocusEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut FocusInputDeviceKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InputDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Impl: IGettingFocusEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCancel<Impl: IGettingFocusEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCancel(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGettingFocusEventArgs>, base.5, OldFocusedElement::<Impl, OFFSET>, NewFocusedElement::<Impl, OFFSET>, SetNewFocusedElement::<Impl, OFFSET>, FocusState::<Impl, OFFSET>, Direction::<Impl, OFFSET>, Handled::<Impl, OFFSET>, SetHandled::<Impl, OFFSET>, InputDevice::<Impl, OFFSET>, Cancel::<Impl, OFFSET>, SetCancel::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGettingFocusEventArgs2Impl: Sized {
    fn TryCancel(&self) -> ::windows::core::Result<bool>;
    fn TrySetNewFocusedElement(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGettingFocusEventArgs2 {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IGettingFocusEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IGettingFocusEventArgs2Vtbl {
    pub const fn new<Impl: IGettingFocusEventArgs2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGettingFocusEventArgs2Vtbl {
        unsafe extern "system" fn TryCancel<Impl: IGettingFocusEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryCancel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrySetNewFocusedElement<Impl: IGettingFocusEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TrySetNewFocusedElement(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGettingFocusEventArgs2>, base.5, TryCancel::<Impl, OFFSET>, TrySetNewFocusedElement::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGettingFocusEventArgs3Impl: Sized {
    fn CorrelationId(&self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGettingFocusEventArgs3 {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IGettingFocusEventArgs3";
}
#[cfg(feature = "implement_exclusive")]
impl IGettingFocusEventArgs3Vtbl {
    pub const fn new<Impl: IGettingFocusEventArgs3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IGettingFocusEventArgs3Vtbl {
        unsafe extern "system" fn CorrelationId<Impl: IGettingFocusEventArgs3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CorrelationId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IGettingFocusEventArgs3>, base.5, CorrelationId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IHoldingRoutedEventArgsImpl: Sized {
    fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::super::Devices::Input::PointerDeviceType>;
    fn HoldingState(&self) -> ::windows::core::Result<super::super::Input::HoldingState>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn GetPosition(&self, relativeto: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<super::super::super::Foundation::Point>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IHoldingRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IHoldingRoutedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IHoldingRoutedEventArgsVtbl {
    pub const fn new<Impl: IHoldingRoutedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHoldingRoutedEventArgsVtbl {
        unsafe extern "system" fn PointerDeviceType<Impl: IHoldingRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointerDeviceType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HoldingState<Impl: IHoldingRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Input::HoldingState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).HoldingState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handled<Impl: IHoldingRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IHoldingRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn GetPosition<Impl: IHoldingRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relativeto: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPosition(&*(&relativeto as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHoldingRoutedEventArgs>, base.5, PointerDeviceType::<Impl, OFFSET>, HoldingState::<Impl, OFFSET>, Handled::<Impl, OFFSET>, SetHandled::<Impl, OFFSET>, GetPosition::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInertiaExpansionBehaviorImpl: Sized {
    fn DesiredDeceleration(&self) -> ::windows::core::Result<f64>;
    fn SetDesiredDeceleration(&self, value: f64) -> ::windows::core::Result<()>;
    fn DesiredExpansion(&self) -> ::windows::core::Result<f64>;
    fn SetDesiredExpansion(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInertiaExpansionBehavior {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IInertiaExpansionBehavior";
}
#[cfg(feature = "implement_exclusive")]
impl IInertiaExpansionBehaviorVtbl {
    pub const fn new<Impl: IInertiaExpansionBehaviorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInertiaExpansionBehaviorVtbl {
        unsafe extern "system" fn DesiredDeceleration<Impl: IInertiaExpansionBehaviorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DesiredDeceleration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredDeceleration<Impl: IInertiaExpansionBehaviorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDesiredDeceleration(value).into()
        }
        unsafe extern "system" fn DesiredExpansion<Impl: IInertiaExpansionBehaviorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DesiredExpansion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredExpansion<Impl: IInertiaExpansionBehaviorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDesiredExpansion(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInertiaExpansionBehavior>, base.5, DesiredDeceleration::<Impl, OFFSET>, SetDesiredDeceleration::<Impl, OFFSET>, DesiredExpansion::<Impl, OFFSET>, SetDesiredExpansion::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInertiaRotationBehaviorImpl: Sized {
    fn DesiredDeceleration(&self) -> ::windows::core::Result<f64>;
    fn SetDesiredDeceleration(&self, value: f64) -> ::windows::core::Result<()>;
    fn DesiredRotation(&self) -> ::windows::core::Result<f64>;
    fn SetDesiredRotation(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInertiaRotationBehavior {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IInertiaRotationBehavior";
}
#[cfg(feature = "implement_exclusive")]
impl IInertiaRotationBehaviorVtbl {
    pub const fn new<Impl: IInertiaRotationBehaviorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInertiaRotationBehaviorVtbl {
        unsafe extern "system" fn DesiredDeceleration<Impl: IInertiaRotationBehaviorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DesiredDeceleration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredDeceleration<Impl: IInertiaRotationBehaviorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDesiredDeceleration(value).into()
        }
        unsafe extern "system" fn DesiredRotation<Impl: IInertiaRotationBehaviorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DesiredRotation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredRotation<Impl: IInertiaRotationBehaviorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDesiredRotation(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInertiaRotationBehavior>, base.5, DesiredDeceleration::<Impl, OFFSET>, SetDesiredDeceleration::<Impl, OFFSET>, DesiredRotation::<Impl, OFFSET>, SetDesiredRotation::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInertiaTranslationBehaviorImpl: Sized {
    fn DesiredDeceleration(&self) -> ::windows::core::Result<f64>;
    fn SetDesiredDeceleration(&self, value: f64) -> ::windows::core::Result<()>;
    fn DesiredDisplacement(&self) -> ::windows::core::Result<f64>;
    fn SetDesiredDisplacement(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInertiaTranslationBehavior {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IInertiaTranslationBehavior";
}
#[cfg(feature = "implement_exclusive")]
impl IInertiaTranslationBehaviorVtbl {
    pub const fn new<Impl: IInertiaTranslationBehaviorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInertiaTranslationBehaviorVtbl {
        unsafe extern "system" fn DesiredDeceleration<Impl: IInertiaTranslationBehaviorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DesiredDeceleration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredDeceleration<Impl: IInertiaTranslationBehaviorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDesiredDeceleration(value).into()
        }
        unsafe extern "system" fn DesiredDisplacement<Impl: IInertiaTranslationBehaviorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DesiredDisplacement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredDisplacement<Impl: IInertiaTranslationBehaviorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDesiredDisplacement(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInertiaTranslationBehavior>, base.5, DesiredDeceleration::<Impl, OFFSET>, SetDesiredDeceleration::<Impl, OFFSET>, DesiredDisplacement::<Impl, OFFSET>, SetDesiredDisplacement::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInputScopeImpl: Sized {
    fn Names(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<InputScopeName>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInputScope {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IInputScope";
}
#[cfg(feature = "implement_exclusive")]
impl IInputScopeVtbl {
    pub const fn new<Impl: IInputScopeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInputScopeVtbl {
        unsafe extern "system" fn Names<Impl: IInputScopeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Names() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInputScope>, base.5, Names::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInputScopeNameImpl: Sized {
    fn NameValue(&self) -> ::windows::core::Result<InputScopeNameValue>;
    fn SetNameValue(&self, value: InputScopeNameValue) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInputScopeName {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IInputScopeName";
}
#[cfg(feature = "implement_exclusive")]
impl IInputScopeNameVtbl {
    pub const fn new<Impl: IInputScopeNameImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInputScopeNameVtbl {
        unsafe extern "system" fn NameValue<Impl: IInputScopeNameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut InputScopeNameValue) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NameValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNameValue<Impl: IInputScopeNameImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: InputScopeNameValue) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetNameValue(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInputScopeName>, base.5, NameValue::<Impl, OFFSET>, SetNameValue::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInputScopeNameFactoryImpl: Sized {
    fn CreateInstance(&self, namevalue: InputScopeNameValue) -> ::windows::core::Result<InputScopeName>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInputScopeNameFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IInputScopeNameFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IInputScopeNameFactoryVtbl {
    pub const fn new<Impl: IInputScopeNameFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInputScopeNameFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IInputScopeNameFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, namevalue: InputScopeNameValue, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstance(namevalue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInputScopeNameFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyRoutedEventArgsImpl: Sized {
    fn Key(&self) -> ::windows::core::Result<super::super::super::System::VirtualKey>;
    fn KeyStatus(&self) -> ::windows::core::Result<super::super::Core::CorePhysicalKeyStatus>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeyRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IKeyRoutedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IKeyRoutedEventArgsVtbl {
    pub const fn new<Impl: IKeyRoutedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IKeyRoutedEventArgsVtbl {
        unsafe extern "system" fn Key<Impl: IKeyRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::System::VirtualKey) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Key() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyStatus<Impl: IKeyRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Core::CorePhysicalKeyStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handled<Impl: IKeyRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IKeyRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IKeyRoutedEventArgs>, base.5, Key::<Impl, OFFSET>, KeyStatus::<Impl, OFFSET>, Handled::<Impl, OFFSET>, SetHandled::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyRoutedEventArgs2Impl: Sized {
    fn OriginalKey(&self) -> ::windows::core::Result<super::super::super::System::VirtualKey>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeyRoutedEventArgs2 {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IKeyRoutedEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IKeyRoutedEventArgs2Vtbl {
    pub const fn new<Impl: IKeyRoutedEventArgs2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IKeyRoutedEventArgs2Vtbl {
        unsafe extern "system" fn OriginalKey<Impl: IKeyRoutedEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::System::VirtualKey) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OriginalKey() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IKeyRoutedEventArgs2>, base.5, OriginalKey::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyRoutedEventArgs3Impl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeyRoutedEventArgs3 {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IKeyRoutedEventArgs3";
}
#[cfg(feature = "implement_exclusive")]
impl IKeyRoutedEventArgs3Vtbl {
    pub const fn new<Impl: IKeyRoutedEventArgs3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IKeyRoutedEventArgs3Vtbl {
        unsafe extern "system" fn DeviceId<Impl: IKeyRoutedEventArgs3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IKeyRoutedEventArgs3>, base.5, DeviceId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyboardAcceleratorImpl: Sized {
    fn Key(&self) -> ::windows::core::Result<super::super::super::System::VirtualKey>;
    fn SetKey(&self, value: super::super::super::System::VirtualKey) -> ::windows::core::Result<()>;
    fn Modifiers(&self) -> ::windows::core::Result<super::super::super::System::VirtualKeyModifiers>;
    fn SetModifiers(&self, value: super::super::super::System::VirtualKeyModifiers) -> ::windows::core::Result<()>;
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn ScopeOwner(&self) -> ::windows::core::Result<super::DependencyObject>;
    fn SetScopeOwner(&self, value: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<()>;
    fn Invoked(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<KeyboardAccelerator, KeyboardAcceleratorInvokedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveInvoked(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeyboardAccelerator {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IKeyboardAccelerator";
}
#[cfg(feature = "implement_exclusive")]
impl IKeyboardAcceleratorVtbl {
    pub const fn new<Impl: IKeyboardAcceleratorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IKeyboardAcceleratorVtbl {
        unsafe extern "system" fn Key<Impl: IKeyboardAcceleratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::System::VirtualKey) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Key() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKey<Impl: IKeyboardAcceleratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::super::System::VirtualKey) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetKey(value).into()
        }
        unsafe extern "system" fn Modifiers<Impl: IKeyboardAcceleratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Modifiers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetModifiers<Impl: IKeyboardAcceleratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetModifiers(value).into()
        }
        unsafe extern "system" fn IsEnabled<Impl: IKeyboardAcceleratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsEnabled<Impl: IKeyboardAcceleratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsEnabled(value).into()
        }
        unsafe extern "system" fn ScopeOwner<Impl: IKeyboardAcceleratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScopeOwner() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScopeOwner<Impl: IKeyboardAcceleratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetScopeOwner(&*(&value as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Invoked<Impl: IKeyboardAcceleratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Invoked(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<KeyboardAccelerator, KeyboardAcceleratorInvokedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<KeyboardAccelerator, KeyboardAcceleratorInvokedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveInvoked<Impl: IKeyboardAcceleratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveInvoked(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IKeyboardAccelerator>, base.5, Key::<Impl, OFFSET>, SetKey::<Impl, OFFSET>, Modifiers::<Impl, OFFSET>, SetModifiers::<Impl, OFFSET>, IsEnabled::<Impl, OFFSET>, SetIsEnabled::<Impl, OFFSET>, ScopeOwner::<Impl, OFFSET>, SetScopeOwner::<Impl, OFFSET>, Invoked::<Impl, OFFSET>, RemoveInvoked::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyboardAcceleratorFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<KeyboardAccelerator>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeyboardAcceleratorFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IKeyboardAcceleratorFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IKeyboardAcceleratorFactoryVtbl {
    pub const fn new<Impl: IKeyboardAcceleratorFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IKeyboardAcceleratorFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IKeyboardAcceleratorFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IKeyboardAcceleratorFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyboardAcceleratorInvokedEventArgsImpl: Sized {
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn Element(&self) -> ::windows::core::Result<super::DependencyObject>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeyboardAcceleratorInvokedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IKeyboardAcceleratorInvokedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IKeyboardAcceleratorInvokedEventArgsVtbl {
    pub const fn new<Impl: IKeyboardAcceleratorInvokedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IKeyboardAcceleratorInvokedEventArgsVtbl {
        unsafe extern "system" fn Handled<Impl: IKeyboardAcceleratorInvokedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IKeyboardAcceleratorInvokedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn Element<Impl: IKeyboardAcceleratorInvokedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Element() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IKeyboardAcceleratorInvokedEventArgs>, base.5, Handled::<Impl, OFFSET>, SetHandled::<Impl, OFFSET>, Element::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyboardAcceleratorInvokedEventArgs2Impl: Sized {
    fn KeyboardAccelerator(&self) -> ::windows::core::Result<KeyboardAccelerator>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeyboardAcceleratorInvokedEventArgs2 {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IKeyboardAcceleratorInvokedEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IKeyboardAcceleratorInvokedEventArgs2Vtbl {
    pub const fn new<Impl: IKeyboardAcceleratorInvokedEventArgs2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IKeyboardAcceleratorInvokedEventArgs2Vtbl {
        unsafe extern "system" fn KeyboardAccelerator<Impl: IKeyboardAcceleratorInvokedEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyboardAccelerator() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IKeyboardAcceleratorInvokedEventArgs2>, base.5, KeyboardAccelerator::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyboardAcceleratorStaticsImpl: Sized {
    fn KeyProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ModifiersProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsEnabledProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ScopeOwnerProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeyboardAcceleratorStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IKeyboardAcceleratorStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IKeyboardAcceleratorStaticsVtbl {
    pub const fn new<Impl: IKeyboardAcceleratorStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IKeyboardAcceleratorStaticsVtbl {
        unsafe extern "system" fn KeyProperty<Impl: IKeyboardAcceleratorStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifiersProperty<Impl: IKeyboardAcceleratorStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ModifiersProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEnabledProperty<Impl: IKeyboardAcceleratorStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsEnabledProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScopeOwnerProperty<Impl: IKeyboardAcceleratorStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScopeOwnerProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IKeyboardAcceleratorStatics>, base.5, KeyProperty::<Impl, OFFSET>, ModifiersProperty::<Impl, OFFSET>, IsEnabledProperty::<Impl, OFFSET>, ScopeOwnerProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILosingFocusEventArgsImpl: Sized {
    fn OldFocusedElement(&self) -> ::windows::core::Result<super::DependencyObject>;
    fn NewFocusedElement(&self) -> ::windows::core::Result<super::DependencyObject>;
    fn SetNewFocusedElement(&self, value: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<()>;
    fn FocusState(&self) -> ::windows::core::Result<super::FocusState>;
    fn Direction(&self) -> ::windows::core::Result<FocusNavigationDirection>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn InputDevice(&self) -> ::windows::core::Result<FocusInputDeviceKind>;
    fn Cancel(&self) -> ::windows::core::Result<bool>;
    fn SetCancel(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILosingFocusEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.ILosingFocusEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ILosingFocusEventArgsVtbl {
    pub const fn new<Impl: ILosingFocusEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILosingFocusEventArgsVtbl {
        unsafe extern "system" fn OldFocusedElement<Impl: ILosingFocusEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OldFocusedElement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NewFocusedElement<Impl: ILosingFocusEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NewFocusedElement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNewFocusedElement<Impl: ILosingFocusEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetNewFocusedElement(&*(&value as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FocusState<Impl: ILosingFocusEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::FocusState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FocusState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Direction<Impl: ILosingFocusEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut FocusNavigationDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Direction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handled<Impl: ILosingFocusEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: ILosingFocusEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn InputDevice<Impl: ILosingFocusEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut FocusInputDeviceKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InputDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Impl: ILosingFocusEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCancel<Impl: ILosingFocusEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCancel(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILosingFocusEventArgs>, base.5, OldFocusedElement::<Impl, OFFSET>, NewFocusedElement::<Impl, OFFSET>, SetNewFocusedElement::<Impl, OFFSET>, FocusState::<Impl, OFFSET>, Direction::<Impl, OFFSET>, Handled::<Impl, OFFSET>, SetHandled::<Impl, OFFSET>, InputDevice::<Impl, OFFSET>, Cancel::<Impl, OFFSET>, SetCancel::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILosingFocusEventArgs2Impl: Sized {
    fn TryCancel(&self) -> ::windows::core::Result<bool>;
    fn TrySetNewFocusedElement(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILosingFocusEventArgs2 {
    const NAME: &'static str = "Windows.UI.Xaml.Input.ILosingFocusEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl ILosingFocusEventArgs2Vtbl {
    pub const fn new<Impl: ILosingFocusEventArgs2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILosingFocusEventArgs2Vtbl {
        unsafe extern "system" fn TryCancel<Impl: ILosingFocusEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryCancel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrySetNewFocusedElement<Impl: ILosingFocusEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TrySetNewFocusedElement(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILosingFocusEventArgs2>, base.5, TryCancel::<Impl, OFFSET>, TrySetNewFocusedElement::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILosingFocusEventArgs3Impl: Sized {
    fn CorrelationId(&self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILosingFocusEventArgs3 {
    const NAME: &'static str = "Windows.UI.Xaml.Input.ILosingFocusEventArgs3";
}
#[cfg(feature = "implement_exclusive")]
impl ILosingFocusEventArgs3Vtbl {
    pub const fn new<Impl: ILosingFocusEventArgs3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILosingFocusEventArgs3Vtbl {
        unsafe extern "system" fn CorrelationId<Impl: ILosingFocusEventArgs3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CorrelationId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILosingFocusEventArgs3>, base.5, CorrelationId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IManipulationCompletedRoutedEventArgsImpl: Sized {
    fn Container(&self) -> ::windows::core::Result<super::UIElement>;
    fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn IsInertial(&self) -> ::windows::core::Result<bool>;
    fn Cumulative(&self) -> ::windows::core::Result<super::super::Input::ManipulationDelta>;
    fn Velocities(&self) -> ::windows::core::Result<super::super::Input::ManipulationVelocities>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::super::Devices::Input::PointerDeviceType>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IManipulationCompletedRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IManipulationCompletedRoutedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IManipulationCompletedRoutedEventArgsVtbl {
    pub const fn new<Impl: IManipulationCompletedRoutedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IManipulationCompletedRoutedEventArgsVtbl {
        unsafe extern "system" fn Container<Impl: IManipulationCompletedRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Container() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Impl: IManipulationCompletedRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsInertial<Impl: IManipulationCompletedRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsInertial() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cumulative<Impl: IManipulationCompletedRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Input::ManipulationDelta) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Cumulative() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Velocities<Impl: IManipulationCompletedRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Input::ManipulationVelocities) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Velocities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handled<Impl: IManipulationCompletedRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IManipulationCompletedRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn PointerDeviceType<Impl: IManipulationCompletedRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointerDeviceType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IManipulationCompletedRoutedEventArgs>, base.5, Container::<Impl, OFFSET>, Position::<Impl, OFFSET>, IsInertial::<Impl, OFFSET>, Cumulative::<Impl, OFFSET>, Velocities::<Impl, OFFSET>, Handled::<Impl, OFFSET>, SetHandled::<Impl, OFFSET>, PointerDeviceType::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IManipulationDeltaRoutedEventArgsImpl: Sized {
    fn Container(&self) -> ::windows::core::Result<super::UIElement>;
    fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn IsInertial(&self) -> ::windows::core::Result<bool>;
    fn Delta(&self) -> ::windows::core::Result<super::super::Input::ManipulationDelta>;
    fn Cumulative(&self) -> ::windows::core::Result<super::super::Input::ManipulationDelta>;
    fn Velocities(&self) -> ::windows::core::Result<super::super::Input::ManipulationVelocities>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::super::Devices::Input::PointerDeviceType>;
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IManipulationDeltaRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IManipulationDeltaRoutedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IManipulationDeltaRoutedEventArgsVtbl {
    pub const fn new<Impl: IManipulationDeltaRoutedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IManipulationDeltaRoutedEventArgsVtbl {
        unsafe extern "system" fn Container<Impl: IManipulationDeltaRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Container() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Impl: IManipulationDeltaRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsInertial<Impl: IManipulationDeltaRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsInertial() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delta<Impl: IManipulationDeltaRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Input::ManipulationDelta) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Delta() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cumulative<Impl: IManipulationDeltaRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Input::ManipulationDelta) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Cumulative() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Velocities<Impl: IManipulationDeltaRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Input::ManipulationVelocities) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Velocities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handled<Impl: IManipulationDeltaRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IManipulationDeltaRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn PointerDeviceType<Impl: IManipulationDeltaRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointerDeviceType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Complete<Impl: IManipulationDeltaRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IManipulationDeltaRoutedEventArgs>, base.5, Container::<Impl, OFFSET>, Position::<Impl, OFFSET>, IsInertial::<Impl, OFFSET>, Delta::<Impl, OFFSET>, Cumulative::<Impl, OFFSET>, Velocities::<Impl, OFFSET>, Handled::<Impl, OFFSET>, SetHandled::<Impl, OFFSET>, PointerDeviceType::<Impl, OFFSET>, Complete::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IManipulationInertiaStartingRoutedEventArgsImpl: Sized {
    fn Container(&self) -> ::windows::core::Result<super::UIElement>;
    fn ExpansionBehavior(&self) -> ::windows::core::Result<InertiaExpansionBehavior>;
    fn SetExpansionBehavior(&self, value: &::core::option::Option<InertiaExpansionBehavior>) -> ::windows::core::Result<()>;
    fn RotationBehavior(&self) -> ::windows::core::Result<InertiaRotationBehavior>;
    fn SetRotationBehavior(&self, value: &::core::option::Option<InertiaRotationBehavior>) -> ::windows::core::Result<()>;
    fn TranslationBehavior(&self) -> ::windows::core::Result<InertiaTranslationBehavior>;
    fn SetTranslationBehavior(&self, value: &::core::option::Option<InertiaTranslationBehavior>) -> ::windows::core::Result<()>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::super::Devices::Input::PointerDeviceType>;
    fn Delta(&self) -> ::windows::core::Result<super::super::Input::ManipulationDelta>;
    fn Cumulative(&self) -> ::windows::core::Result<super::super::Input::ManipulationDelta>;
    fn Velocities(&self) -> ::windows::core::Result<super::super::Input::ManipulationVelocities>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IManipulationInertiaStartingRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IManipulationInertiaStartingRoutedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IManipulationInertiaStartingRoutedEventArgsVtbl {
    pub const fn new<Impl: IManipulationInertiaStartingRoutedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IManipulationInertiaStartingRoutedEventArgsVtbl {
        unsafe extern "system" fn Container<Impl: IManipulationInertiaStartingRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Container() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpansionBehavior<Impl: IManipulationInertiaStartingRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExpansionBehavior() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExpansionBehavior<Impl: IManipulationInertiaStartingRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetExpansionBehavior(&*(&value as *const <InertiaExpansionBehavior as ::windows::core::Abi>::Abi as *const <InertiaExpansionBehavior as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RotationBehavior<Impl: IManipulationInertiaStartingRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RotationBehavior() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotationBehavior<Impl: IManipulationInertiaStartingRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRotationBehavior(&*(&value as *const <InertiaRotationBehavior as ::windows::core::Abi>::Abi as *const <InertiaRotationBehavior as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TranslationBehavior<Impl: IManipulationInertiaStartingRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TranslationBehavior() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTranslationBehavior<Impl: IManipulationInertiaStartingRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTranslationBehavior(&*(&value as *const <InertiaTranslationBehavior as ::windows::core::Abi>::Abi as *const <InertiaTranslationBehavior as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Handled<Impl: IManipulationInertiaStartingRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IManipulationInertiaStartingRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn PointerDeviceType<Impl: IManipulationInertiaStartingRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointerDeviceType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delta<Impl: IManipulationInertiaStartingRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Input::ManipulationDelta) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Delta() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cumulative<Impl: IManipulationInertiaStartingRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Input::ManipulationDelta) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Cumulative() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Velocities<Impl: IManipulationInertiaStartingRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Input::ManipulationVelocities) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Velocities() {
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
            ::windows::core::GetRuntimeClassName::<IManipulationInertiaStartingRoutedEventArgs>,
            base.5,
            Container::<Impl, OFFSET>,
            ExpansionBehavior::<Impl, OFFSET>,
            SetExpansionBehavior::<Impl, OFFSET>,
            RotationBehavior::<Impl, OFFSET>,
            SetRotationBehavior::<Impl, OFFSET>,
            TranslationBehavior::<Impl, OFFSET>,
            SetTranslationBehavior::<Impl, OFFSET>,
            Handled::<Impl, OFFSET>,
            SetHandled::<Impl, OFFSET>,
            PointerDeviceType::<Impl, OFFSET>,
            Delta::<Impl, OFFSET>,
            Cumulative::<Impl, OFFSET>,
            Velocities::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IManipulationPivotImpl: Sized {
    fn Center(&self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn SetCenter(&self, value: &super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn Radius(&self) -> ::windows::core::Result<f64>;
    fn SetRadius(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IManipulationPivot {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IManipulationPivot";
}
#[cfg(feature = "implement_exclusive")]
impl IManipulationPivotVtbl {
    pub const fn new<Impl: IManipulationPivotImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IManipulationPivotVtbl {
        unsafe extern "system" fn Center<Impl: IManipulationPivotImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Center() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenter<Impl: IManipulationPivotImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCenter(&*(&value as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Radius<Impl: IManipulationPivotImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Radius() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRadius<Impl: IManipulationPivotImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRadius(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IManipulationPivot>, base.5, Center::<Impl, OFFSET>, SetCenter::<Impl, OFFSET>, Radius::<Impl, OFFSET>, SetRadius::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IManipulationPivotFactoryImpl: Sized {
    fn CreateInstanceWithCenterAndRadius(&self, center: &super::super::super::Foundation::Point, radius: f64) -> ::windows::core::Result<ManipulationPivot>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IManipulationPivotFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IManipulationPivotFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IManipulationPivotFactoryVtbl {
    pub const fn new<Impl: IManipulationPivotFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IManipulationPivotFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithCenterAndRadius<Impl: IManipulationPivotFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, center: super::super::super::Foundation::Point, radius: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithCenterAndRadius(&*(&center as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType), radius) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IManipulationPivotFactory>, base.5, CreateInstanceWithCenterAndRadius::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IManipulationStartedRoutedEventArgsImpl: Sized {
    fn Container(&self) -> ::windows::core::Result<super::UIElement>;
    fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::super::Devices::Input::PointerDeviceType>;
    fn Cumulative(&self) -> ::windows::core::Result<super::super::Input::ManipulationDelta>;
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IManipulationStartedRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IManipulationStartedRoutedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IManipulationStartedRoutedEventArgsVtbl {
    pub const fn new<Impl: IManipulationStartedRoutedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IManipulationStartedRoutedEventArgsVtbl {
        unsafe extern "system" fn Container<Impl: IManipulationStartedRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Container() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Impl: IManipulationStartedRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Handled<Impl: IManipulationStartedRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IManipulationStartedRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn PointerDeviceType<Impl: IManipulationStartedRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointerDeviceType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cumulative<Impl: IManipulationStartedRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Input::ManipulationDelta) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Cumulative() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Complete<Impl: IManipulationStartedRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IManipulationStartedRoutedEventArgs>, base.5, Container::<Impl, OFFSET>, Position::<Impl, OFFSET>, Handled::<Impl, OFFSET>, SetHandled::<Impl, OFFSET>, PointerDeviceType::<Impl, OFFSET>, Cumulative::<Impl, OFFSET>, Complete::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IManipulationStartedRoutedEventArgsFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ManipulationStartedRoutedEventArgs>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IManipulationStartedRoutedEventArgsFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IManipulationStartedRoutedEventArgsFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IManipulationStartedRoutedEventArgsFactoryVtbl {
    pub const fn new<Impl: IManipulationStartedRoutedEventArgsFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IManipulationStartedRoutedEventArgsFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IManipulationStartedRoutedEventArgsFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IManipulationStartedRoutedEventArgsFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IManipulationStartingRoutedEventArgsImpl: Sized {
    fn Mode(&self) -> ::windows::core::Result<ManipulationModes>;
    fn SetMode(&self, value: ManipulationModes) -> ::windows::core::Result<()>;
    fn Container(&self) -> ::windows::core::Result<super::UIElement>;
    fn SetContainer(&self, value: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn Pivot(&self) -> ::windows::core::Result<ManipulationPivot>;
    fn SetPivot(&self, value: &::core::option::Option<ManipulationPivot>) -> ::windows::core::Result<()>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IManipulationStartingRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IManipulationStartingRoutedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IManipulationStartingRoutedEventArgsVtbl {
    pub const fn new<Impl: IManipulationStartingRoutedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IManipulationStartingRoutedEventArgsVtbl {
        unsafe extern "system" fn Mode<Impl: IManipulationStartingRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ManipulationModes) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMode<Impl: IManipulationStartingRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ManipulationModes) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetMode(value).into()
        }
        unsafe extern "system" fn Container<Impl: IManipulationStartingRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Container() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContainer<Impl: IManipulationStartingRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetContainer(&*(&value as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Pivot<Impl: IManipulationStartingRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Pivot() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPivot<Impl: IManipulationStartingRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPivot(&*(&value as *const <ManipulationPivot as ::windows::core::Abi>::Abi as *const <ManipulationPivot as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Handled<Impl: IManipulationStartingRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IManipulationStartingRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IManipulationStartingRoutedEventArgs>, base.5, Mode::<Impl, OFFSET>, SetMode::<Impl, OFFSET>, Container::<Impl, OFFSET>, SetContainer::<Impl, OFFSET>, Pivot::<Impl, OFFSET>, SetPivot::<Impl, OFFSET>, Handled::<Impl, OFFSET>, SetHandled::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INoFocusCandidateFoundEventArgsImpl: Sized {
    fn Direction(&self) -> ::windows::core::Result<FocusNavigationDirection>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn InputDevice(&self) -> ::windows::core::Result<FocusInputDeviceKind>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INoFocusCandidateFoundEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.INoFocusCandidateFoundEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl INoFocusCandidateFoundEventArgsVtbl {
    pub const fn new<Impl: INoFocusCandidateFoundEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> INoFocusCandidateFoundEventArgsVtbl {
        unsafe extern "system" fn Direction<Impl: INoFocusCandidateFoundEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut FocusNavigationDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Direction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handled<Impl: INoFocusCandidateFoundEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: INoFocusCandidateFoundEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn InputDevice<Impl: INoFocusCandidateFoundEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut FocusInputDeviceKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InputDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<INoFocusCandidateFoundEventArgs>, base.5, Direction::<Impl, OFFSET>, Handled::<Impl, OFFSET>, SetHandled::<Impl, OFFSET>, InputDevice::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointerImpl: Sized {
    fn PointerId(&self) -> ::windows::core::Result<u32>;
    fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::super::Devices::Input::PointerDeviceType>;
    fn IsInContact(&self) -> ::windows::core::Result<bool>;
    fn IsInRange(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPointer {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IPointer";
}
#[cfg(feature = "implement_exclusive")]
impl IPointerVtbl {
    pub const fn new<Impl: IPointerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPointerVtbl {
        unsafe extern "system" fn PointerId<Impl: IPointerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointerId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PointerDeviceType<Impl: IPointerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointerDeviceType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInContact<Impl: IPointerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsInContact() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInRange<Impl: IPointerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsInRange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPointer>, base.5, PointerId::<Impl, OFFSET>, PointerDeviceType::<Impl, OFFSET>, IsInContact::<Impl, OFFSET>, IsInRange::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointerRoutedEventArgsImpl: Sized {
    fn Pointer(&self) -> ::windows::core::Result<Pointer>;
    fn KeyModifiers(&self) -> ::windows::core::Result<super::super::super::System::VirtualKeyModifiers>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn GetCurrentPoint(&self, relativeto: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<super::super::Input::PointerPoint>;
    fn GetIntermediatePoints(&self, relativeto: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<super::super::Input::PointerPoint>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPointerRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IPointerRoutedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IPointerRoutedEventArgsVtbl {
    pub const fn new<Impl: IPointerRoutedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPointerRoutedEventArgsVtbl {
        unsafe extern "system" fn Pointer<Impl: IPointerRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Pointer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyModifiers<Impl: IPointerRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyModifiers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handled<Impl: IPointerRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IPointerRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn GetCurrentPoint<Impl: IPointerRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relativeto: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCurrentPoint(&*(&relativeto as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIntermediatePoints<Impl: IPointerRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relativeto: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetIntermediatePoints(&*(&relativeto as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPointerRoutedEventArgs>, base.5, Pointer::<Impl, OFFSET>, KeyModifiers::<Impl, OFFSET>, Handled::<Impl, OFFSET>, SetHandled::<Impl, OFFSET>, GetCurrentPoint::<Impl, OFFSET>, GetIntermediatePoints::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointerRoutedEventArgs2Impl: Sized {
    fn IsGenerated(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPointerRoutedEventArgs2 {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IPointerRoutedEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IPointerRoutedEventArgs2Vtbl {
    pub const fn new<Impl: IPointerRoutedEventArgs2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPointerRoutedEventArgs2Vtbl {
        unsafe extern "system" fn IsGenerated<Impl: IPointerRoutedEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsGenerated() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPointerRoutedEventArgs2>, base.5, IsGenerated::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProcessKeyboardAcceleratorEventArgsImpl: Sized {
    fn Key(&self) -> ::windows::core::Result<super::super::super::System::VirtualKey>;
    fn Modifiers(&self) -> ::windows::core::Result<super::super::super::System::VirtualKeyModifiers>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProcessKeyboardAcceleratorEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IProcessKeyboardAcceleratorEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IProcessKeyboardAcceleratorEventArgsVtbl {
    pub const fn new<Impl: IProcessKeyboardAcceleratorEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IProcessKeyboardAcceleratorEventArgsVtbl {
        unsafe extern "system" fn Key<Impl: IProcessKeyboardAcceleratorEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::System::VirtualKey) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Key() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Modifiers<Impl: IProcessKeyboardAcceleratorEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Modifiers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handled<Impl: IProcessKeyboardAcceleratorEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IProcessKeyboardAcceleratorEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IProcessKeyboardAcceleratorEventArgs>, base.5, Key::<Impl, OFFSET>, Modifiers::<Impl, OFFSET>, Handled::<Impl, OFFSET>, SetHandled::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRightTappedRoutedEventArgsImpl: Sized {
    fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::super::Devices::Input::PointerDeviceType>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn GetPosition(&self, relativeto: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<super::super::super::Foundation::Point>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRightTappedRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IRightTappedRoutedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRightTappedRoutedEventArgsVtbl {
    pub const fn new<Impl: IRightTappedRoutedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRightTappedRoutedEventArgsVtbl {
        unsafe extern "system" fn PointerDeviceType<Impl: IRightTappedRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointerDeviceType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handled<Impl: IRightTappedRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IRightTappedRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn GetPosition<Impl: IRightTappedRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relativeto: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPosition(&*(&relativeto as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRightTappedRoutedEventArgs>, base.5, PointerDeviceType::<Impl, OFFSET>, Handled::<Impl, OFFSET>, SetHandled::<Impl, OFFSET>, GetPosition::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStandardUICommandImpl: Sized {
    fn Kind(&self) -> ::windows::core::Result<StandardUICommandKind>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStandardUICommand {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IStandardUICommand";
}
#[cfg(feature = "implement_exclusive")]
impl IStandardUICommandVtbl {
    pub const fn new<Impl: IStandardUICommandImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IStandardUICommandVtbl {
        unsafe extern "system" fn Kind<Impl: IStandardUICommandImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut StandardUICommandKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Kind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IStandardUICommand>, base.5, Kind::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStandardUICommand2Impl: Sized {
    fn SetKind(&self, value: StandardUICommandKind) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStandardUICommand2 {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IStandardUICommand2";
}
#[cfg(feature = "implement_exclusive")]
impl IStandardUICommand2Vtbl {
    pub const fn new<Impl: IStandardUICommand2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IStandardUICommand2Vtbl {
        unsafe extern "system" fn SetKind<Impl: IStandardUICommand2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: StandardUICommandKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetKind(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IStandardUICommand2>, base.5, SetKind::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStandardUICommandFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<StandardUICommand>;
    fn CreateInstanceWithKind(&self, kind: StandardUICommandKind, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<StandardUICommand>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStandardUICommandFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IStandardUICommandFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IStandardUICommandFactoryVtbl {
    pub const fn new<Impl: IStandardUICommandFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IStandardUICommandFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IStandardUICommandFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateInstanceWithKind<Impl: IStandardUICommandFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, kind: StandardUICommandKind, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithKind(kind, &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IStandardUICommandFactory>, base.5, CreateInstance::<Impl, OFFSET>, CreateInstanceWithKind::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStandardUICommandStaticsImpl: Sized {
    fn KindProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStandardUICommandStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IStandardUICommandStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IStandardUICommandStaticsVtbl {
    pub const fn new<Impl: IStandardUICommandStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IStandardUICommandStaticsVtbl {
        unsafe extern "system" fn KindProperty<Impl: IStandardUICommandStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KindProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IStandardUICommandStatics>, base.5, KindProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ITappedRoutedEventArgsImpl: Sized {
    fn PointerDeviceType(&self) -> ::windows::core::Result<super::super::super::Devices::Input::PointerDeviceType>;
    fn Handled(&self) -> ::windows::core::Result<bool>;
    fn SetHandled(&self, value: bool) -> ::windows::core::Result<()>;
    fn GetPosition(&self, relativeto: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<super::super::super::Foundation::Point>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ITappedRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.ITappedRoutedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ITappedRoutedEventArgsVtbl {
    pub const fn new<Impl: ITappedRoutedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITappedRoutedEventArgsVtbl {
        unsafe extern "system" fn PointerDeviceType<Impl: ITappedRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PointerDeviceType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handled<Impl: ITappedRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: ITappedRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn GetPosition<Impl: ITappedRoutedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, relativeto: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPosition(&*(&relativeto as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITappedRoutedEventArgs>, base.5, PointerDeviceType::<Impl, OFFSET>, Handled::<Impl, OFFSET>, SetHandled::<Impl, OFFSET>, GetPosition::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlUICommandImpl: Sized {
    fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLabel(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IconSource(&self) -> ::windows::core::Result<super::Controls::IconSource>;
    fn SetIconSource(&self, value: &::core::option::Option<super::Controls::IconSource>) -> ::windows::core::Result<()>;
    fn KeyboardAccelerators(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<KeyboardAccelerator>>;
    fn AccessKey(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAccessKey(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Command(&self) -> ::windows::core::Result<ICommand>;
    fn SetCommand(&self, value: &::core::option::Option<ICommand>) -> ::windows::core::Result<()>;
    fn ExecuteRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<XamlUICommand, ExecuteRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveExecuteRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CanExecuteRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<XamlUICommand, CanExecuteRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCanExecuteRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NotifyCanExecuteChanged(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXamlUICommand {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IXamlUICommand";
}
#[cfg(feature = "implement_exclusive")]
impl IXamlUICommandVtbl {
    pub const fn new<Impl: IXamlUICommandImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXamlUICommandVtbl {
        unsafe extern "system" fn Label<Impl: IXamlUICommandImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Label() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Impl: IXamlUICommandImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetLabel(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IconSource<Impl: IXamlUICommandImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIconSource<Impl: IXamlUICommandImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIconSource(&*(&value as *const <super::Controls::IconSource as ::windows::core::Abi>::Abi as *const <super::Controls::IconSource as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn KeyboardAccelerators<Impl: IXamlUICommandImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyboardAccelerators() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AccessKey<Impl: IXamlUICommandImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AccessKey() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccessKey<Impl: IXamlUICommandImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAccessKey(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Description<Impl: IXamlUICommandImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IXamlUICommandImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDescription(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Command<Impl: IXamlUICommandImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCommand<Impl: IXamlUICommandImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCommand(&*(&value as *const <ICommand as ::windows::core::Abi>::Abi as *const <ICommand as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ExecuteRequested<Impl: IXamlUICommandImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExecuteRequested(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<XamlUICommand, ExecuteRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<XamlUICommand, ExecuteRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveExecuteRequested<Impl: IXamlUICommandImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveExecuteRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CanExecuteRequested<Impl: IXamlUICommandImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CanExecuteRequested(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<XamlUICommand, CanExecuteRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<XamlUICommand, CanExecuteRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCanExecuteRequested<Impl: IXamlUICommandImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveCanExecuteRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NotifyCanExecuteChanged<Impl: IXamlUICommandImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).NotifyCanExecuteChanged().into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IXamlUICommand>,
            base.5,
            Label::<Impl, OFFSET>,
            SetLabel::<Impl, OFFSET>,
            IconSource::<Impl, OFFSET>,
            SetIconSource::<Impl, OFFSET>,
            KeyboardAccelerators::<Impl, OFFSET>,
            AccessKey::<Impl, OFFSET>,
            SetAccessKey::<Impl, OFFSET>,
            Description::<Impl, OFFSET>,
            SetDescription::<Impl, OFFSET>,
            Command::<Impl, OFFSET>,
            SetCommand::<Impl, OFFSET>,
            ExecuteRequested::<Impl, OFFSET>,
            RemoveExecuteRequested::<Impl, OFFSET>,
            CanExecuteRequested::<Impl, OFFSET>,
            RemoveCanExecuteRequested::<Impl, OFFSET>,
            NotifyCanExecuteChanged::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlUICommandFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<XamlUICommand>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXamlUICommandFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IXamlUICommandFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IXamlUICommandFactoryVtbl {
    pub const fn new<Impl: IXamlUICommandFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXamlUICommandFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IXamlUICommandFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXamlUICommandFactory>, base.5, CreateInstance::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlUICommandStaticsImpl: Sized {
    fn LabelProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IconSourceProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn KeyboardAcceleratorsProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn AccessKeyProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn DescriptionProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CommandProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXamlUICommandStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IXamlUICommandStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IXamlUICommandStaticsVtbl {
    pub const fn new<Impl: IXamlUICommandStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXamlUICommandStaticsVtbl {
        unsafe extern "system" fn LabelProperty<Impl: IXamlUICommandStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LabelProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IconSourceProperty<Impl: IXamlUICommandStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IconSourceProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyboardAcceleratorsProperty<Impl: IXamlUICommandStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).KeyboardAcceleratorsProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AccessKeyProperty<Impl: IXamlUICommandStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AccessKeyProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DescriptionProperty<Impl: IXamlUICommandStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DescriptionProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommandProperty<Impl: IXamlUICommandStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXamlUICommandStatics>, base.5, LabelProperty::<Impl, OFFSET>, IconSourceProperty::<Impl, OFFSET>, KeyboardAcceleratorsProperty::<Impl, OFFSET>, AccessKeyProperty::<Impl, OFFSET>, DescriptionProperty::<Impl, OFFSET>, CommandProperty::<Impl, OFFSET>)
    }
}

#[cfg(feature = "implement_exclusive")]
pub trait IAccessKeyDisplayDismissedEventArgs_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAccessKeyDisplayDismissedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IAccessKeyDisplayDismissedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAccessKeyDisplayDismissedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccessKeyDisplayDismissedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccessKeyDisplayDismissedEventArgs_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAccessKeyDisplayDismissedEventArgs, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccessKeyDisplayDismissedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccessKeyDisplayRequestedEventArgs_Impl: Sized {
    fn PressedKeys(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAccessKeyDisplayRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IAccessKeyDisplayRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAccessKeyDisplayRequestedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccessKeyDisplayRequestedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccessKeyDisplayRequestedEventArgs_Vtbl {
        unsafe extern "system" fn PressedKeys<Impl: IAccessKeyDisplayRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PressedKeys() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAccessKeyDisplayRequestedEventArgs, BASE_OFFSET>(),
            PressedKeys: PressedKeys::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccessKeyDisplayRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccessKeyInvokedEventArgs_Impl: Sized {
    fn Handled(&mut self) -> ::windows::core::Result<bool>;
    fn SetHandled(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAccessKeyInvokedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IAccessKeyInvokedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAccessKeyInvokedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccessKeyInvokedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccessKeyInvokedEventArgs_Vtbl {
        unsafe extern "system" fn Handled<Impl: IAccessKeyInvokedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IAccessKeyInvokedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAccessKeyInvokedEventArgs, BASE_OFFSET>(),
            Handled: Handled::<Impl, IMPL_OFFSET>,
            SetHandled: SetHandled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccessKeyInvokedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccessKeyManager_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAccessKeyManager {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IAccessKeyManager";
}
#[cfg(feature = "implement_exclusive")]
impl IAccessKeyManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccessKeyManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccessKeyManager_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAccessKeyManager, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccessKeyManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAccessKeyManagerStatics_Impl: Sized {
    fn IsDisplayModeEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn IsDisplayModeEnabledChanged(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveIsDisplayModeEnabledChanged(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ExitDisplayMode(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAccessKeyManagerStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IAccessKeyManagerStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAccessKeyManagerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccessKeyManagerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccessKeyManagerStatics_Vtbl {
        unsafe extern "system" fn IsDisplayModeEnabled<Impl: IAccessKeyManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDisplayModeEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDisplayModeEnabledChanged<Impl: IAccessKeyManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDisplayModeEnabledChanged(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveIsDisplayModeEnabledChanged<Impl: IAccessKeyManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveIsDisplayModeEnabledChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ExitDisplayMode<Impl: IAccessKeyManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ExitDisplayMode().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAccessKeyManagerStatics, BASE_OFFSET>(),
            IsDisplayModeEnabled: IsDisplayModeEnabled::<Impl, IMPL_OFFSET>,
            IsDisplayModeEnabledChanged: IsDisplayModeEnabledChanged::<Impl, IMPL_OFFSET>,
            RemoveIsDisplayModeEnabledChanged: RemoveIsDisplayModeEnabledChanged::<Impl, IMPL_OFFSET>,
            ExitDisplayMode: ExitDisplayMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccessKeyManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccessKeyManagerStatics2_Impl: Sized {
    fn AreKeyTipsEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetAreKeyTipsEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAccessKeyManagerStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IAccessKeyManagerStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IAccessKeyManagerStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccessKeyManagerStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccessKeyManagerStatics2_Vtbl {
        unsafe extern "system" fn AreKeyTipsEnabled<Impl: IAccessKeyManagerStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AreKeyTipsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAreKeyTipsEnabled<Impl: IAccessKeyManagerStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAreKeyTipsEnabled(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAccessKeyManagerStatics2, BASE_OFFSET>(),
            AreKeyTipsEnabled: AreKeyTipsEnabled::<Impl, IMPL_OFFSET>,
            SetAreKeyTipsEnabled: SetAreKeyTipsEnabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccessKeyManagerStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICanExecuteRequestedEventArgs_Impl: Sized {
    fn Parameter(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CanExecute(&mut self) -> ::windows::core::Result<bool>;
    fn SetCanExecute(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICanExecuteRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.ICanExecuteRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ICanExecuteRequestedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICanExecuteRequestedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICanExecuteRequestedEventArgs_Vtbl {
        unsafe extern "system" fn Parameter<Impl: ICanExecuteRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parameter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanExecute<Impl: ICanExecuteRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanExecute() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCanExecute<Impl: ICanExecuteRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCanExecute(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICanExecuteRequestedEventArgs, BASE_OFFSET>(),
            Parameter: Parameter::<Impl, IMPL_OFFSET>,
            CanExecute: CanExecute::<Impl, IMPL_OFFSET>,
            SetCanExecute: SetCanExecute::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICanExecuteRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Core", feature = "implement_exclusive"))]
pub trait ICharacterReceivedRoutedEventArgs_Impl: Sized {
    fn Character(&mut self) -> ::windows::core::Result<u16>;
    fn KeyStatus(&mut self) -> ::windows::core::Result<super::super::Core::CorePhysicalKeyStatus>;
    fn Handled(&mut self) -> ::windows::core::Result<bool>;
    fn SetHandled(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "UI_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICharacterReceivedRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.ICharacterReceivedRoutedEventArgs";
}
#[cfg(all(feature = "UI_Core", feature = "implement_exclusive"))]
impl ICharacterReceivedRoutedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICharacterReceivedRoutedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICharacterReceivedRoutedEventArgs_Vtbl {
        unsafe extern "system" fn Character<Impl: ICharacterReceivedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Character() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyStatus<Impl: ICharacterReceivedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Core::CorePhysicalKeyStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handled<Impl: ICharacterReceivedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: ICharacterReceivedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICharacterReceivedRoutedEventArgs, BASE_OFFSET>(),
            Character: Character::<Impl, IMPL_OFFSET>,
            KeyStatus: KeyStatus::<Impl, IMPL_OFFSET>,
            Handled: Handled::<Impl, IMPL_OFFSET>,
            SetHandled: SetHandled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICharacterReceivedRoutedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait ICommand_Impl: Sized {
    fn CanExecuteChanged(&mut self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCanExecuteChanged(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CanExecute(&mut self, parameter: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<bool>;
    fn Execute(&mut self, parameter: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for ICommand {
    const NAME: &'static str = "Windows.UI.Xaml.Input.ICommand";
}
#[cfg(feature = "Foundation")]
impl ICommand_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommand_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICommand_Vtbl {
        unsafe extern "system" fn CanExecuteChanged<Impl: ICommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanExecuteChanged(&*(&handler as *const <super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventHandler<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCanExecuteChanged<Impl: ICommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCanExecuteChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CanExecute<Impl: ICommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameter: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanExecute(&*(&parameter as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Execute<Impl: ICommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Execute(&*(&parameter as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICommand, BASE_OFFSET>(),
            CanExecuteChanged: CanExecuteChanged::<Impl, IMPL_OFFSET>,
            RemoveCanExecuteChanged: RemoveCanExecuteChanged::<Impl, IMPL_OFFSET>,
            CanExecute: CanExecute::<Impl, IMPL_OFFSET>,
            Execute: Execute::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICommand as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IContextRequestedEventArgs_Impl: Sized {
    fn Handled(&mut self) -> ::windows::core::Result<bool>;
    fn SetHandled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn TryGetPosition(&mut self, relativeto: &::core::option::Option<super::UIElement>, point: &mut super::super::super::Foundation::Point) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IContextRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IContextRequestedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IContextRequestedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContextRequestedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContextRequestedEventArgs_Vtbl {
        unsafe extern "system" fn Handled<Impl: IContextRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IContextRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn TryGetPosition<Impl: IContextRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relativeto: ::windows::core::RawPtr, point: *mut super::super::super::Foundation::Point, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryGetPosition(&*(&relativeto as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&point)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IContextRequestedEventArgs, BASE_OFFSET>(),
            Handled: Handled::<Impl, IMPL_OFFSET>,
            SetHandled: SetHandled::<Impl, IMPL_OFFSET>,
            TryGetPosition: TryGetPosition::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContextRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Input", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDoubleTappedRoutedEventArgs_Impl: Sized {
    fn PointerDeviceType(&mut self) -> ::windows::core::Result<super::super::super::Devices::Input::PointerDeviceType>;
    fn Handled(&mut self) -> ::windows::core::Result<bool>;
    fn SetHandled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn GetPosition(&mut self, relativeto: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<super::super::super::Foundation::Point>;
}
#[cfg(all(feature = "Devices_Input", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDoubleTappedRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IDoubleTappedRoutedEventArgs";
}
#[cfg(all(feature = "Devices_Input", feature = "Foundation", feature = "implement_exclusive"))]
impl IDoubleTappedRoutedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDoubleTappedRoutedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDoubleTappedRoutedEventArgs_Vtbl {
        unsafe extern "system" fn PointerDeviceType<Impl: IDoubleTappedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerDeviceType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handled<Impl: IDoubleTappedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IDoubleTappedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn GetPosition<Impl: IDoubleTappedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relativeto: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPosition(&*(&relativeto as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDoubleTappedRoutedEventArgs, BASE_OFFSET>(),
            PointerDeviceType: PointerDeviceType::<Impl, IMPL_OFFSET>,
            Handled: Handled::<Impl, IMPL_OFFSET>,
            SetHandled: SetHandled::<Impl, IMPL_OFFSET>,
            GetPosition: GetPosition::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDoubleTappedRoutedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IExecuteRequestedEventArgs_Impl: Sized {
    fn Parameter(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IExecuteRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IExecuteRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IExecuteRequestedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExecuteRequestedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IExecuteRequestedEventArgs_Vtbl {
        unsafe extern "system" fn Parameter<Impl: IExecuteRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Parameter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IExecuteRequestedEventArgs, BASE_OFFSET>(), Parameter: Parameter::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IExecuteRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IFindNextElementOptions_Impl: Sized {
    fn SearchRoot(&mut self) -> ::windows::core::Result<super::DependencyObject>;
    fn SetSearchRoot(&mut self, value: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<()>;
    fn ExclusionRect(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn SetExclusionRect(&mut self, value: &super::super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn HintRect(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Rect>;
    fn SetHintRect(&mut self, value: &super::super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn XYFocusNavigationStrategyOverride(&mut self) -> ::windows::core::Result<XYFocusNavigationStrategyOverride>;
    fn SetXYFocusNavigationStrategyOverride(&mut self, value: XYFocusNavigationStrategyOverride) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFindNextElementOptions {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IFindNextElementOptions";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IFindNextElementOptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFindNextElementOptions_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFindNextElementOptions_Vtbl {
        unsafe extern "system" fn SearchRoot<Impl: IFindNextElementOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchRoot() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSearchRoot<Impl: IFindNextElementOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSearchRoot(&*(&value as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ExclusionRect<Impl: IFindNextElementOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetExclusionRect<Impl: IFindNextElementOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExclusionRect(&*(&value as *const <super::super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn HintRect<Impl: IFindNextElementOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HintRect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHintRect<Impl: IFindNextElementOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHintRect(&*(&value as *const <super::super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn XYFocusNavigationStrategyOverride<Impl: IFindNextElementOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut XYFocusNavigationStrategyOverride) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).XYFocusNavigationStrategyOverride() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetXYFocusNavigationStrategyOverride<Impl: IFindNextElementOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: XYFocusNavigationStrategyOverride) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetXYFocusNavigationStrategyOverride(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFindNextElementOptions, BASE_OFFSET>(),
            SearchRoot: SearchRoot::<Impl, IMPL_OFFSET>,
            SetSearchRoot: SetSearchRoot::<Impl, IMPL_OFFSET>,
            ExclusionRect: ExclusionRect::<Impl, IMPL_OFFSET>,
            SetExclusionRect: SetExclusionRect::<Impl, IMPL_OFFSET>,
            HintRect: HintRect::<Impl, IMPL_OFFSET>,
            SetHintRect: SetHintRect::<Impl, IMPL_OFFSET>,
            XYFocusNavigationStrategyOverride: XYFocusNavigationStrategyOverride::<Impl, IMPL_OFFSET>,
            SetXYFocusNavigationStrategyOverride: SetXYFocusNavigationStrategyOverride::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFindNextElementOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFocusManager_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFocusManager {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IFocusManager";
}
#[cfg(feature = "implement_exclusive")]
impl IFocusManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFocusManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFocusManager_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IFocusManager, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFocusManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFocusManagerGotFocusEventArgs_Impl: Sized {
    fn NewFocusedElement(&mut self) -> ::windows::core::Result<super::DependencyObject>;
    fn CorrelationId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFocusManagerGotFocusEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IFocusManagerGotFocusEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IFocusManagerGotFocusEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFocusManagerGotFocusEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFocusManagerGotFocusEventArgs_Vtbl {
        unsafe extern "system" fn NewFocusedElement<Impl: IFocusManagerGotFocusEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NewFocusedElement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CorrelationId<Impl: IFocusManagerGotFocusEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CorrelationId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFocusManagerGotFocusEventArgs, BASE_OFFSET>(),
            NewFocusedElement: NewFocusedElement::<Impl, IMPL_OFFSET>,
            CorrelationId: CorrelationId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFocusManagerGotFocusEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFocusManagerLostFocusEventArgs_Impl: Sized {
    fn OldFocusedElement(&mut self) -> ::windows::core::Result<super::DependencyObject>;
    fn CorrelationId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFocusManagerLostFocusEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IFocusManagerLostFocusEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IFocusManagerLostFocusEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFocusManagerLostFocusEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFocusManagerLostFocusEventArgs_Vtbl {
        unsafe extern "system" fn OldFocusedElement<Impl: IFocusManagerLostFocusEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OldFocusedElement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CorrelationId<Impl: IFocusManagerLostFocusEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CorrelationId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFocusManagerLostFocusEventArgs, BASE_OFFSET>(),
            OldFocusedElement: OldFocusedElement::<Impl, IMPL_OFFSET>,
            CorrelationId: CorrelationId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFocusManagerLostFocusEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFocusManagerStatics_Impl: Sized {
    fn GetFocusedElement(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFocusManagerStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IFocusManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IFocusManagerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFocusManagerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFocusManagerStatics_Vtbl {
        unsafe extern "system" fn GetFocusedElement<Impl: IFocusManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFocusedElement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFocusManagerStatics, BASE_OFFSET>(),
            GetFocusedElement: GetFocusedElement::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFocusManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFocusManagerStatics2_Impl: Sized {
    fn TryMoveFocus(&mut self, focusnavigationdirection: FocusNavigationDirection) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFocusManagerStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IFocusManagerStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IFocusManagerStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFocusManagerStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFocusManagerStatics2_Vtbl {
        unsafe extern "system" fn TryMoveFocus<Impl: IFocusManagerStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, focusnavigationdirection: FocusNavigationDirection, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryMoveFocus(focusnavigationdirection) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IFocusManagerStatics2, BASE_OFFSET>(), TryMoveFocus: TryMoveFocus::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFocusManagerStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IFocusManagerStatics3_Impl: Sized {
    fn FindNextFocusableElement(&mut self, focusnavigationdirection: FocusNavigationDirection) -> ::windows::core::Result<super::UIElement>;
    fn FindNextFocusableElementWithHint(&mut self, focusnavigationdirection: FocusNavigationDirection, hintrect: &super::super::super::Foundation::Rect) -> ::windows::core::Result<super::UIElement>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFocusManagerStatics3 {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IFocusManagerStatics3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IFocusManagerStatics3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFocusManagerStatics3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFocusManagerStatics3_Vtbl {
        unsafe extern "system" fn FindNextFocusableElement<Impl: IFocusManagerStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, focusnavigationdirection: FocusNavigationDirection, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindNextFocusableElement(focusnavigationdirection) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindNextFocusableElementWithHint<Impl: IFocusManagerStatics3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, focusnavigationdirection: FocusNavigationDirection, hintrect: super::super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindNextFocusableElementWithHint(focusnavigationdirection, &*(&hintrect as *const <super::super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFocusManagerStatics3, BASE_OFFSET>(),
            FindNextFocusableElement: FindNextFocusableElement::<Impl, IMPL_OFFSET>,
            FindNextFocusableElementWithHint: FindNextFocusableElementWithHint::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFocusManagerStatics3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFocusManagerStatics4_Impl: Sized {
    fn TryMoveFocusWithOptions(&mut self, focusnavigationdirection: FocusNavigationDirection, focusnavigationoptions: &::core::option::Option<FindNextElementOptions>) -> ::windows::core::Result<bool>;
    fn FindNextElement(&mut self, focusnavigationdirection: FocusNavigationDirection) -> ::windows::core::Result<super::DependencyObject>;
    fn FindFirstFocusableElement(&mut self, searchscope: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<super::DependencyObject>;
    fn FindLastFocusableElement(&mut self, searchscope: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<super::DependencyObject>;
    fn FindNextElementWithOptions(&mut self, focusnavigationdirection: FocusNavigationDirection, focusnavigationoptions: &::core::option::Option<FindNextElementOptions>) -> ::windows::core::Result<super::DependencyObject>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFocusManagerStatics4 {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IFocusManagerStatics4";
}
#[cfg(feature = "implement_exclusive")]
impl IFocusManagerStatics4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFocusManagerStatics4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFocusManagerStatics4_Vtbl {
        unsafe extern "system" fn TryMoveFocusWithOptions<Impl: IFocusManagerStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, focusnavigationdirection: FocusNavigationDirection, focusnavigationoptions: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryMoveFocusWithOptions(focusnavigationdirection, &*(&focusnavigationoptions as *const <FindNextElementOptions as ::windows::core::Abi>::Abi as *const <FindNextElementOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindNextElement<Impl: IFocusManagerStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, focusnavigationdirection: FocusNavigationDirection, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindNextElement(focusnavigationdirection) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindFirstFocusableElement<Impl: IFocusManagerStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, searchscope: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindFirstFocusableElement(&*(&searchscope as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindLastFocusableElement<Impl: IFocusManagerStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, searchscope: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindLastFocusableElement(&*(&searchscope as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindNextElementWithOptions<Impl: IFocusManagerStatics4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, focusnavigationdirection: FocusNavigationDirection, focusnavigationoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindNextElementWithOptions(focusnavigationdirection, &*(&focusnavigationoptions as *const <FindNextElementOptions as ::windows::core::Abi>::Abi as *const <FindNextElementOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFocusManagerStatics4, BASE_OFFSET>(),
            TryMoveFocusWithOptions: TryMoveFocusWithOptions::<Impl, IMPL_OFFSET>,
            FindNextElement: FindNextElement::<Impl, IMPL_OFFSET>,
            FindFirstFocusableElement: FindFirstFocusableElement::<Impl, IMPL_OFFSET>,
            FindLastFocusableElement: FindLastFocusableElement::<Impl, IMPL_OFFSET>,
            FindNextElementWithOptions: FindNextElementWithOptions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFocusManagerStatics4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IFocusManagerStatics5_Impl: Sized {
    fn TryFocusAsync(&mut self, element: &::core::option::Option<super::DependencyObject>, value: super::FocusState) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<FocusMovementResult>>;
    fn TryMoveFocusAsync(&mut self, focusnavigationdirection: FocusNavigationDirection) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<FocusMovementResult>>;
    fn TryMoveFocusWithOptionsAsync(&mut self, focusnavigationdirection: FocusNavigationDirection, focusnavigationoptions: &::core::option::Option<FindNextElementOptions>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<FocusMovementResult>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFocusManagerStatics5 {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IFocusManagerStatics5";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IFocusManagerStatics5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFocusManagerStatics5_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFocusManagerStatics5_Vtbl {
        unsafe extern "system" fn TryFocusAsync<Impl: IFocusManagerStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: super::FocusState, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryFocusAsync(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryMoveFocusAsync<Impl: IFocusManagerStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, focusnavigationdirection: FocusNavigationDirection, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryMoveFocusAsync(focusnavigationdirection) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryMoveFocusWithOptionsAsync<Impl: IFocusManagerStatics5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, focusnavigationdirection: FocusNavigationDirection, focusnavigationoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryMoveFocusWithOptionsAsync(focusnavigationdirection, &*(&focusnavigationoptions as *const <FindNextElementOptions as ::windows::core::Abi>::Abi as *const <FindNextElementOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFocusManagerStatics5, BASE_OFFSET>(),
            TryFocusAsync: TryFocusAsync::<Impl, IMPL_OFFSET>,
            TryMoveFocusAsync: TryMoveFocusAsync::<Impl, IMPL_OFFSET>,
            TryMoveFocusWithOptionsAsync: TryMoveFocusWithOptionsAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFocusManagerStatics5 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IFocusManagerStatics6_Impl: Sized {
    fn GotFocus(&mut self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<FocusManagerGotFocusEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveGotFocus(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn LostFocus(&mut self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<FocusManagerLostFocusEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveLostFocus(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GettingFocus(&mut self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<GettingFocusEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveGettingFocus(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn LosingFocus(&mut self, handler: &::core::option::Option<super::super::super::Foundation::EventHandler<LosingFocusEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveLosingFocus(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFocusManagerStatics6 {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IFocusManagerStatics6";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IFocusManagerStatics6_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFocusManagerStatics6_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFocusManagerStatics6_Vtbl {
        unsafe extern "system" fn GotFocus<Impl: IFocusManagerStatics6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GotFocus(&*(&handler as *const <super::super::super::Foundation::EventHandler<FocusManagerGotFocusEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventHandler<FocusManagerGotFocusEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveGotFocus<Impl: IFocusManagerStatics6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveGotFocus(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LostFocus<Impl: IFocusManagerStatics6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LostFocus(&*(&handler as *const <super::super::super::Foundation::EventHandler<FocusManagerLostFocusEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventHandler<FocusManagerLostFocusEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLostFocus<Impl: IFocusManagerStatics6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveLostFocus(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GettingFocus<Impl: IFocusManagerStatics6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GettingFocus(&*(&handler as *const <super::super::super::Foundation::EventHandler<GettingFocusEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventHandler<GettingFocusEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveGettingFocus<Impl: IFocusManagerStatics6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveGettingFocus(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LosingFocus<Impl: IFocusManagerStatics6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LosingFocus(&*(&handler as *const <super::super::super::Foundation::EventHandler<LosingFocusEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventHandler<LosingFocusEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveLosingFocus<Impl: IFocusManagerStatics6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveLosingFocus(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFocusManagerStatics6, BASE_OFFSET>(),
            GotFocus: GotFocus::<Impl, IMPL_OFFSET>,
            RemoveGotFocus: RemoveGotFocus::<Impl, IMPL_OFFSET>,
            LostFocus: LostFocus::<Impl, IMPL_OFFSET>,
            RemoveLostFocus: RemoveLostFocus::<Impl, IMPL_OFFSET>,
            GettingFocus: GettingFocus::<Impl, IMPL_OFFSET>,
            RemoveGettingFocus: RemoveGettingFocus::<Impl, IMPL_OFFSET>,
            LosingFocus: LosingFocus::<Impl, IMPL_OFFSET>,
            RemoveLosingFocus: RemoveLosingFocus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFocusManagerStatics6 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFocusManagerStatics7_Impl: Sized {
    fn GetFocusedElement(&mut self, xamlroot: &::core::option::Option<super::XamlRoot>) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFocusManagerStatics7 {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IFocusManagerStatics7";
}
#[cfg(feature = "implement_exclusive")]
impl IFocusManagerStatics7_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFocusManagerStatics7_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFocusManagerStatics7_Vtbl {
        unsafe extern "system" fn GetFocusedElement<Impl: IFocusManagerStatics7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xamlroot: ::windows::core::RawPtr, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFocusedElement(&*(&xamlroot as *const <super::XamlRoot as ::windows::core::Abi>::Abi as *const <super::XamlRoot as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFocusManagerStatics7, BASE_OFFSET>(),
            GetFocusedElement: GetFocusedElement::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFocusManagerStatics7 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFocusMovementResult_Impl: Sized {
    fn Succeeded(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFocusMovementResult {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IFocusMovementResult";
}
#[cfg(feature = "implement_exclusive")]
impl IFocusMovementResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFocusMovementResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFocusMovementResult_Vtbl {
        unsafe extern "system" fn Succeeded<Impl: IFocusMovementResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Succeeded() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IFocusMovementResult, BASE_OFFSET>(), Succeeded: Succeeded::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFocusMovementResult as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGettingFocusEventArgs_Impl: Sized {
    fn OldFocusedElement(&mut self) -> ::windows::core::Result<super::DependencyObject>;
    fn NewFocusedElement(&mut self) -> ::windows::core::Result<super::DependencyObject>;
    fn SetNewFocusedElement(&mut self, value: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<()>;
    fn FocusState(&mut self) -> ::windows::core::Result<super::FocusState>;
    fn Direction(&mut self) -> ::windows::core::Result<FocusNavigationDirection>;
    fn Handled(&mut self) -> ::windows::core::Result<bool>;
    fn SetHandled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn InputDevice(&mut self) -> ::windows::core::Result<FocusInputDeviceKind>;
    fn Cancel(&mut self) -> ::windows::core::Result<bool>;
    fn SetCancel(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGettingFocusEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IGettingFocusEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IGettingFocusEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGettingFocusEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGettingFocusEventArgs_Vtbl {
        unsafe extern "system" fn OldFocusedElement<Impl: IGettingFocusEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OldFocusedElement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NewFocusedElement<Impl: IGettingFocusEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NewFocusedElement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNewFocusedElement<Impl: IGettingFocusEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNewFocusedElement(&*(&value as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FocusState<Impl: IGettingFocusEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::FocusState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FocusState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Direction<Impl: IGettingFocusEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FocusNavigationDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Direction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handled<Impl: IGettingFocusEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IGettingFocusEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn InputDevice<Impl: IGettingFocusEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FocusInputDeviceKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InputDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Impl: IGettingFocusEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCancel<Impl: IGettingFocusEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCancel(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGettingFocusEventArgs, BASE_OFFSET>(),
            OldFocusedElement: OldFocusedElement::<Impl, IMPL_OFFSET>,
            NewFocusedElement: NewFocusedElement::<Impl, IMPL_OFFSET>,
            SetNewFocusedElement: SetNewFocusedElement::<Impl, IMPL_OFFSET>,
            FocusState: FocusState::<Impl, IMPL_OFFSET>,
            Direction: Direction::<Impl, IMPL_OFFSET>,
            Handled: Handled::<Impl, IMPL_OFFSET>,
            SetHandled: SetHandled::<Impl, IMPL_OFFSET>,
            InputDevice: InputDevice::<Impl, IMPL_OFFSET>,
            Cancel: Cancel::<Impl, IMPL_OFFSET>,
            SetCancel: SetCancel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGettingFocusEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGettingFocusEventArgs2_Impl: Sized {
    fn TryCancel(&mut self) -> ::windows::core::Result<bool>;
    fn TrySetNewFocusedElement(&mut self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGettingFocusEventArgs2 {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IGettingFocusEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IGettingFocusEventArgs2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGettingFocusEventArgs2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGettingFocusEventArgs2_Vtbl {
        unsafe extern "system" fn TryCancel<Impl: IGettingFocusEventArgs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryCancel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrySetNewFocusedElement<Impl: IGettingFocusEventArgs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrySetNewFocusedElement(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGettingFocusEventArgs2, BASE_OFFSET>(),
            TryCancel: TryCancel::<Impl, IMPL_OFFSET>,
            TrySetNewFocusedElement: TrySetNewFocusedElement::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGettingFocusEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGettingFocusEventArgs3_Impl: Sized {
    fn CorrelationId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGettingFocusEventArgs3 {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IGettingFocusEventArgs3";
}
#[cfg(feature = "implement_exclusive")]
impl IGettingFocusEventArgs3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGettingFocusEventArgs3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGettingFocusEventArgs3_Vtbl {
        unsafe extern "system" fn CorrelationId<Impl: IGettingFocusEventArgs3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CorrelationId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGettingFocusEventArgs3, BASE_OFFSET>(),
            CorrelationId: CorrelationId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGettingFocusEventArgs3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Input", feature = "Foundation", feature = "UI_Input", feature = "implement_exclusive"))]
pub trait IHoldingRoutedEventArgs_Impl: Sized {
    fn PointerDeviceType(&mut self) -> ::windows::core::Result<super::super::super::Devices::Input::PointerDeviceType>;
    fn HoldingState(&mut self) -> ::windows::core::Result<super::super::Input::HoldingState>;
    fn Handled(&mut self) -> ::windows::core::Result<bool>;
    fn SetHandled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn GetPosition(&mut self, relativeto: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<super::super::super::Foundation::Point>;
}
#[cfg(all(feature = "Devices_Input", feature = "Foundation", feature = "UI_Input", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHoldingRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IHoldingRoutedEventArgs";
}
#[cfg(all(feature = "Devices_Input", feature = "Foundation", feature = "UI_Input", feature = "implement_exclusive"))]
impl IHoldingRoutedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHoldingRoutedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHoldingRoutedEventArgs_Vtbl {
        unsafe extern "system" fn PointerDeviceType<Impl: IHoldingRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerDeviceType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HoldingState<Impl: IHoldingRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Input::HoldingState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HoldingState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handled<Impl: IHoldingRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IHoldingRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn GetPosition<Impl: IHoldingRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relativeto: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPosition(&*(&relativeto as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHoldingRoutedEventArgs, BASE_OFFSET>(),
            PointerDeviceType: PointerDeviceType::<Impl, IMPL_OFFSET>,
            HoldingState: HoldingState::<Impl, IMPL_OFFSET>,
            Handled: Handled::<Impl, IMPL_OFFSET>,
            SetHandled: SetHandled::<Impl, IMPL_OFFSET>,
            GetPosition: GetPosition::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHoldingRoutedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInertiaExpansionBehavior_Impl: Sized {
    fn DesiredDeceleration(&mut self) -> ::windows::core::Result<f64>;
    fn SetDesiredDeceleration(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn DesiredExpansion(&mut self) -> ::windows::core::Result<f64>;
    fn SetDesiredExpansion(&mut self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInertiaExpansionBehavior {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IInertiaExpansionBehavior";
}
#[cfg(feature = "implement_exclusive")]
impl IInertiaExpansionBehavior_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaExpansionBehavior_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInertiaExpansionBehavior_Vtbl {
        unsafe extern "system" fn DesiredDeceleration<Impl: IInertiaExpansionBehavior_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredDeceleration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredDeceleration<Impl: IInertiaExpansionBehavior_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDesiredDeceleration(value).into()
        }
        unsafe extern "system" fn DesiredExpansion<Impl: IInertiaExpansionBehavior_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredExpansion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredExpansion<Impl: IInertiaExpansionBehavior_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDesiredExpansion(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInertiaExpansionBehavior, BASE_OFFSET>(),
            DesiredDeceleration: DesiredDeceleration::<Impl, IMPL_OFFSET>,
            SetDesiredDeceleration: SetDesiredDeceleration::<Impl, IMPL_OFFSET>,
            DesiredExpansion: DesiredExpansion::<Impl, IMPL_OFFSET>,
            SetDesiredExpansion: SetDesiredExpansion::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInertiaExpansionBehavior as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInertiaRotationBehavior_Impl: Sized {
    fn DesiredDeceleration(&mut self) -> ::windows::core::Result<f64>;
    fn SetDesiredDeceleration(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn DesiredRotation(&mut self) -> ::windows::core::Result<f64>;
    fn SetDesiredRotation(&mut self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInertiaRotationBehavior {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IInertiaRotationBehavior";
}
#[cfg(feature = "implement_exclusive")]
impl IInertiaRotationBehavior_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaRotationBehavior_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInertiaRotationBehavior_Vtbl {
        unsafe extern "system" fn DesiredDeceleration<Impl: IInertiaRotationBehavior_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredDeceleration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredDeceleration<Impl: IInertiaRotationBehavior_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDesiredDeceleration(value).into()
        }
        unsafe extern "system" fn DesiredRotation<Impl: IInertiaRotationBehavior_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredRotation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredRotation<Impl: IInertiaRotationBehavior_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDesiredRotation(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInertiaRotationBehavior, BASE_OFFSET>(),
            DesiredDeceleration: DesiredDeceleration::<Impl, IMPL_OFFSET>,
            SetDesiredDeceleration: SetDesiredDeceleration::<Impl, IMPL_OFFSET>,
            DesiredRotation: DesiredRotation::<Impl, IMPL_OFFSET>,
            SetDesiredRotation: SetDesiredRotation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInertiaRotationBehavior as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInertiaTranslationBehavior_Impl: Sized {
    fn DesiredDeceleration(&mut self) -> ::windows::core::Result<f64>;
    fn SetDesiredDeceleration(&mut self, value: f64) -> ::windows::core::Result<()>;
    fn DesiredDisplacement(&mut self) -> ::windows::core::Result<f64>;
    fn SetDesiredDisplacement(&mut self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInertiaTranslationBehavior {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IInertiaTranslationBehavior";
}
#[cfg(feature = "implement_exclusive")]
impl IInertiaTranslationBehavior_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaTranslationBehavior_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInertiaTranslationBehavior_Vtbl {
        unsafe extern "system" fn DesiredDeceleration<Impl: IInertiaTranslationBehavior_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredDeceleration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredDeceleration<Impl: IInertiaTranslationBehavior_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDesiredDeceleration(value).into()
        }
        unsafe extern "system" fn DesiredDisplacement<Impl: IInertiaTranslationBehavior_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredDisplacement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredDisplacement<Impl: IInertiaTranslationBehavior_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDesiredDisplacement(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInertiaTranslationBehavior, BASE_OFFSET>(),
            DesiredDeceleration: DesiredDeceleration::<Impl, IMPL_OFFSET>,
            SetDesiredDeceleration: SetDesiredDeceleration::<Impl, IMPL_OFFSET>,
            DesiredDisplacement: DesiredDisplacement::<Impl, IMPL_OFFSET>,
            SetDesiredDisplacement: SetDesiredDisplacement::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInertiaTranslationBehavior as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IInputScope_Impl: Sized {
    fn Names(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<InputScopeName>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInputScope {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IInputScope";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IInputScope_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInputScope_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInputScope_Vtbl {
        unsafe extern "system" fn Names<Impl: IInputScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Names() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IInputScope, BASE_OFFSET>(), Names: Names::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInputScope as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInputScopeName_Impl: Sized {
    fn NameValue(&mut self) -> ::windows::core::Result<InputScopeNameValue>;
    fn SetNameValue(&mut self, value: InputScopeNameValue) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInputScopeName {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IInputScopeName";
}
#[cfg(feature = "implement_exclusive")]
impl IInputScopeName_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInputScopeName_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInputScopeName_Vtbl {
        unsafe extern "system" fn NameValue<Impl: IInputScopeName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InputScopeNameValue) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NameValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNameValue<Impl: IInputScopeName_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: InputScopeNameValue) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNameValue(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInputScopeName, BASE_OFFSET>(),
            NameValue: NameValue::<Impl, IMPL_OFFSET>,
            SetNameValue: SetNameValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInputScopeName as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IInputScopeNameFactory_Impl: Sized {
    fn CreateInstance(&mut self, namevalue: InputScopeNameValue) -> ::windows::core::Result<InputScopeName>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInputScopeNameFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IInputScopeNameFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IInputScopeNameFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInputScopeNameFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInputScopeNameFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IInputScopeNameFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namevalue: InputScopeNameValue, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(namevalue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IInputScopeNameFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInputScopeNameFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "UI_Core", feature = "implement_exclusive"))]
pub trait IKeyRoutedEventArgs_Impl: Sized {
    fn Key(&mut self) -> ::windows::core::Result<super::super::super::System::VirtualKey>;
    fn KeyStatus(&mut self) -> ::windows::core::Result<super::super::Core::CorePhysicalKeyStatus>;
    fn Handled(&mut self) -> ::windows::core::Result<bool>;
    fn SetHandled(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "System", feature = "UI_Core", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IKeyRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IKeyRoutedEventArgs";
}
#[cfg(all(feature = "System", feature = "UI_Core", feature = "implement_exclusive"))]
impl IKeyRoutedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeyRoutedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeyRoutedEventArgs_Vtbl {
        unsafe extern "system" fn Key<Impl: IKeyRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::System::VirtualKey) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Key() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyStatus<Impl: IKeyRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Core::CorePhysicalKeyStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handled<Impl: IKeyRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IKeyRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKeyRoutedEventArgs, BASE_OFFSET>(),
            Key: Key::<Impl, IMPL_OFFSET>,
            KeyStatus: KeyStatus::<Impl, IMPL_OFFSET>,
            Handled: Handled::<Impl, IMPL_OFFSET>,
            SetHandled: SetHandled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKeyRoutedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait IKeyRoutedEventArgs2_Impl: Sized {
    fn OriginalKey(&mut self) -> ::windows::core::Result<super::super::super::System::VirtualKey>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IKeyRoutedEventArgs2 {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IKeyRoutedEventArgs2";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl IKeyRoutedEventArgs2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeyRoutedEventArgs2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeyRoutedEventArgs2_Vtbl {
        unsafe extern "system" fn OriginalKey<Impl: IKeyRoutedEventArgs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::System::VirtualKey) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OriginalKey() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IKeyRoutedEventArgs2, BASE_OFFSET>(), OriginalKey: OriginalKey::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKeyRoutedEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyRoutedEventArgs3_Impl: Sized {
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeyRoutedEventArgs3 {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IKeyRoutedEventArgs3";
}
#[cfg(feature = "implement_exclusive")]
impl IKeyRoutedEventArgs3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeyRoutedEventArgs3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeyRoutedEventArgs3_Vtbl {
        unsafe extern "system" fn DeviceId<Impl: IKeyRoutedEventArgs3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IKeyRoutedEventArgs3, BASE_OFFSET>(), DeviceId: DeviceId::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKeyRoutedEventArgs3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
pub trait IKeyboardAccelerator_Impl: Sized {
    fn Key(&mut self) -> ::windows::core::Result<super::super::super::System::VirtualKey>;
    fn SetKey(&mut self, value: super::super::super::System::VirtualKey) -> ::windows::core::Result<()>;
    fn Modifiers(&mut self) -> ::windows::core::Result<super::super::super::System::VirtualKeyModifiers>;
    fn SetModifiers(&mut self, value: super::super::super::System::VirtualKeyModifiers) -> ::windows::core::Result<()>;
    fn IsEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn ScopeOwner(&mut self) -> ::windows::core::Result<super::DependencyObject>;
    fn SetScopeOwner(&mut self, value: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<()>;
    fn Invoked(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<KeyboardAccelerator, KeyboardAcceleratorInvokedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveInvoked(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IKeyboardAccelerator {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IKeyboardAccelerator";
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl IKeyboardAccelerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeyboardAccelerator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeyboardAccelerator_Vtbl {
        unsafe extern "system" fn Key<Impl: IKeyboardAccelerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::System::VirtualKey) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Key() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKey<Impl: IKeyboardAccelerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::System::VirtualKey) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKey(value).into()
        }
        unsafe extern "system" fn Modifiers<Impl: IKeyboardAccelerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Modifiers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetModifiers<Impl: IKeyboardAccelerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetModifiers(value).into()
        }
        unsafe extern "system" fn IsEnabled<Impl: IKeyboardAccelerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsEnabled<Impl: IKeyboardAccelerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsEnabled(value).into()
        }
        unsafe extern "system" fn ScopeOwner<Impl: IKeyboardAccelerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScopeOwner() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScopeOwner<Impl: IKeyboardAccelerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScopeOwner(&*(&value as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Invoked<Impl: IKeyboardAccelerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Invoked(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<KeyboardAccelerator, KeyboardAcceleratorInvokedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<KeyboardAccelerator, KeyboardAcceleratorInvokedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveInvoked<Impl: IKeyboardAccelerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveInvoked(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKeyboardAccelerator, BASE_OFFSET>(),
            Key: Key::<Impl, IMPL_OFFSET>,
            SetKey: SetKey::<Impl, IMPL_OFFSET>,
            Modifiers: Modifiers::<Impl, IMPL_OFFSET>,
            SetModifiers: SetModifiers::<Impl, IMPL_OFFSET>,
            IsEnabled: IsEnabled::<Impl, IMPL_OFFSET>,
            SetIsEnabled: SetIsEnabled::<Impl, IMPL_OFFSET>,
            ScopeOwner: ScopeOwner::<Impl, IMPL_OFFSET>,
            SetScopeOwner: SetScopeOwner::<Impl, IMPL_OFFSET>,
            Invoked: Invoked::<Impl, IMPL_OFFSET>,
            RemoveInvoked: RemoveInvoked::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKeyboardAccelerator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyboardAcceleratorFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<KeyboardAccelerator>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeyboardAcceleratorFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IKeyboardAcceleratorFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IKeyboardAcceleratorFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeyboardAcceleratorFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeyboardAcceleratorFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IKeyboardAcceleratorFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKeyboardAcceleratorFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKeyboardAcceleratorFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyboardAcceleratorInvokedEventArgs_Impl: Sized {
    fn Handled(&mut self) -> ::windows::core::Result<bool>;
    fn SetHandled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Element(&mut self) -> ::windows::core::Result<super::DependencyObject>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeyboardAcceleratorInvokedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IKeyboardAcceleratorInvokedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IKeyboardAcceleratorInvokedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeyboardAcceleratorInvokedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeyboardAcceleratorInvokedEventArgs_Vtbl {
        unsafe extern "system" fn Handled<Impl: IKeyboardAcceleratorInvokedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IKeyboardAcceleratorInvokedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn Element<Impl: IKeyboardAcceleratorInvokedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKeyboardAcceleratorInvokedEventArgs, BASE_OFFSET>(),
            Handled: Handled::<Impl, IMPL_OFFSET>,
            SetHandled: SetHandled::<Impl, IMPL_OFFSET>,
            Element: Element::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKeyboardAcceleratorInvokedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyboardAcceleratorInvokedEventArgs2_Impl: Sized {
    fn KeyboardAccelerator(&mut self) -> ::windows::core::Result<KeyboardAccelerator>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeyboardAcceleratorInvokedEventArgs2 {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IKeyboardAcceleratorInvokedEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IKeyboardAcceleratorInvokedEventArgs2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeyboardAcceleratorInvokedEventArgs2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeyboardAcceleratorInvokedEventArgs2_Vtbl {
        unsafe extern "system" fn KeyboardAccelerator<Impl: IKeyboardAcceleratorInvokedEventArgs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyboardAccelerator() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKeyboardAcceleratorInvokedEventArgs2, BASE_OFFSET>(),
            KeyboardAccelerator: KeyboardAccelerator::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKeyboardAcceleratorInvokedEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyboardAcceleratorStatics_Impl: Sized {
    fn KeyProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ModifiersProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IsEnabledProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn ScopeOwnerProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeyboardAcceleratorStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IKeyboardAcceleratorStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IKeyboardAcceleratorStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeyboardAcceleratorStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeyboardAcceleratorStatics_Vtbl {
        unsafe extern "system" fn KeyProperty<Impl: IKeyboardAcceleratorStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ModifiersProperty<Impl: IKeyboardAcceleratorStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModifiersProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEnabledProperty<Impl: IKeyboardAcceleratorStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ScopeOwnerProperty<Impl: IKeyboardAcceleratorStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScopeOwnerProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IKeyboardAcceleratorStatics, BASE_OFFSET>(),
            KeyProperty: KeyProperty::<Impl, IMPL_OFFSET>,
            ModifiersProperty: ModifiersProperty::<Impl, IMPL_OFFSET>,
            IsEnabledProperty: IsEnabledProperty::<Impl, IMPL_OFFSET>,
            ScopeOwnerProperty: ScopeOwnerProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKeyboardAcceleratorStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILosingFocusEventArgs_Impl: Sized {
    fn OldFocusedElement(&mut self) -> ::windows::core::Result<super::DependencyObject>;
    fn NewFocusedElement(&mut self) -> ::windows::core::Result<super::DependencyObject>;
    fn SetNewFocusedElement(&mut self, value: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<()>;
    fn FocusState(&mut self) -> ::windows::core::Result<super::FocusState>;
    fn Direction(&mut self) -> ::windows::core::Result<FocusNavigationDirection>;
    fn Handled(&mut self) -> ::windows::core::Result<bool>;
    fn SetHandled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn InputDevice(&mut self) -> ::windows::core::Result<FocusInputDeviceKind>;
    fn Cancel(&mut self) -> ::windows::core::Result<bool>;
    fn SetCancel(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILosingFocusEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.ILosingFocusEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ILosingFocusEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILosingFocusEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILosingFocusEventArgs_Vtbl {
        unsafe extern "system" fn OldFocusedElement<Impl: ILosingFocusEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OldFocusedElement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NewFocusedElement<Impl: ILosingFocusEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NewFocusedElement() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNewFocusedElement<Impl: ILosingFocusEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNewFocusedElement(&*(&value as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FocusState<Impl: ILosingFocusEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::FocusState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FocusState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Direction<Impl: ILosingFocusEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FocusNavigationDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Direction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handled<Impl: ILosingFocusEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: ILosingFocusEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn InputDevice<Impl: ILosingFocusEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FocusInputDeviceKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InputDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Impl: ILosingFocusEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCancel<Impl: ILosingFocusEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCancel(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILosingFocusEventArgs, BASE_OFFSET>(),
            OldFocusedElement: OldFocusedElement::<Impl, IMPL_OFFSET>,
            NewFocusedElement: NewFocusedElement::<Impl, IMPL_OFFSET>,
            SetNewFocusedElement: SetNewFocusedElement::<Impl, IMPL_OFFSET>,
            FocusState: FocusState::<Impl, IMPL_OFFSET>,
            Direction: Direction::<Impl, IMPL_OFFSET>,
            Handled: Handled::<Impl, IMPL_OFFSET>,
            SetHandled: SetHandled::<Impl, IMPL_OFFSET>,
            InputDevice: InputDevice::<Impl, IMPL_OFFSET>,
            Cancel: Cancel::<Impl, IMPL_OFFSET>,
            SetCancel: SetCancel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILosingFocusEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILosingFocusEventArgs2_Impl: Sized {
    fn TryCancel(&mut self) -> ::windows::core::Result<bool>;
    fn TrySetNewFocusedElement(&mut self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILosingFocusEventArgs2 {
    const NAME: &'static str = "Windows.UI.Xaml.Input.ILosingFocusEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl ILosingFocusEventArgs2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILosingFocusEventArgs2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILosingFocusEventArgs2_Vtbl {
        unsafe extern "system" fn TryCancel<Impl: ILosingFocusEventArgs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryCancel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrySetNewFocusedElement<Impl: ILosingFocusEventArgs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrySetNewFocusedElement(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILosingFocusEventArgs2, BASE_OFFSET>(),
            TryCancel: TryCancel::<Impl, IMPL_OFFSET>,
            TrySetNewFocusedElement: TrySetNewFocusedElement::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILosingFocusEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ILosingFocusEventArgs3_Impl: Sized {
    fn CorrelationId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILosingFocusEventArgs3 {
    const NAME: &'static str = "Windows.UI.Xaml.Input.ILosingFocusEventArgs3";
}
#[cfg(feature = "implement_exclusive")]
impl ILosingFocusEventArgs3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILosingFocusEventArgs3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILosingFocusEventArgs3_Vtbl {
        unsafe extern "system" fn CorrelationId<Impl: ILosingFocusEventArgs3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CorrelationId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ILosingFocusEventArgs3, BASE_OFFSET>(),
            CorrelationId: CorrelationId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILosingFocusEventArgs3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Input", feature = "Foundation", feature = "UI_Input", feature = "implement_exclusive"))]
pub trait IManipulationCompletedRoutedEventArgs_Impl: Sized {
    fn Container(&mut self) -> ::windows::core::Result<super::UIElement>;
    fn Position(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn IsInertial(&mut self) -> ::windows::core::Result<bool>;
    fn Cumulative(&mut self) -> ::windows::core::Result<super::super::Input::ManipulationDelta>;
    fn Velocities(&mut self) -> ::windows::core::Result<super::super::Input::ManipulationVelocities>;
    fn Handled(&mut self) -> ::windows::core::Result<bool>;
    fn SetHandled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn PointerDeviceType(&mut self) -> ::windows::core::Result<super::super::super::Devices::Input::PointerDeviceType>;
}
#[cfg(all(feature = "Devices_Input", feature = "Foundation", feature = "UI_Input", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IManipulationCompletedRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IManipulationCompletedRoutedEventArgs";
}
#[cfg(all(feature = "Devices_Input", feature = "Foundation", feature = "UI_Input", feature = "implement_exclusive"))]
impl IManipulationCompletedRoutedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IManipulationCompletedRoutedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IManipulationCompletedRoutedEventArgs_Vtbl {
        unsafe extern "system" fn Container<Impl: IManipulationCompletedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Container() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Impl: IManipulationCompletedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsInertial<Impl: IManipulationCompletedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsInertial() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cumulative<Impl: IManipulationCompletedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Input::ManipulationDelta) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cumulative() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Velocities<Impl: IManipulationCompletedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Input::ManipulationVelocities) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Velocities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handled<Impl: IManipulationCompletedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IManipulationCompletedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn PointerDeviceType<Impl: IManipulationCompletedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerDeviceType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IManipulationCompletedRoutedEventArgs, BASE_OFFSET>(),
            Container: Container::<Impl, IMPL_OFFSET>,
            Position: Position::<Impl, IMPL_OFFSET>,
            IsInertial: IsInertial::<Impl, IMPL_OFFSET>,
            Cumulative: Cumulative::<Impl, IMPL_OFFSET>,
            Velocities: Velocities::<Impl, IMPL_OFFSET>,
            Handled: Handled::<Impl, IMPL_OFFSET>,
            SetHandled: SetHandled::<Impl, IMPL_OFFSET>,
            PointerDeviceType: PointerDeviceType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IManipulationCompletedRoutedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Input", feature = "Foundation", feature = "UI_Input", feature = "implement_exclusive"))]
pub trait IManipulationDeltaRoutedEventArgs_Impl: Sized {
    fn Container(&mut self) -> ::windows::core::Result<super::UIElement>;
    fn Position(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn IsInertial(&mut self) -> ::windows::core::Result<bool>;
    fn Delta(&mut self) -> ::windows::core::Result<super::super::Input::ManipulationDelta>;
    fn Cumulative(&mut self) -> ::windows::core::Result<super::super::Input::ManipulationDelta>;
    fn Velocities(&mut self) -> ::windows::core::Result<super::super::Input::ManipulationVelocities>;
    fn Handled(&mut self) -> ::windows::core::Result<bool>;
    fn SetHandled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn PointerDeviceType(&mut self) -> ::windows::core::Result<super::super::super::Devices::Input::PointerDeviceType>;
    fn Complete(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Devices_Input", feature = "Foundation", feature = "UI_Input", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IManipulationDeltaRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IManipulationDeltaRoutedEventArgs";
}
#[cfg(all(feature = "Devices_Input", feature = "Foundation", feature = "UI_Input", feature = "implement_exclusive"))]
impl IManipulationDeltaRoutedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IManipulationDeltaRoutedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IManipulationDeltaRoutedEventArgs_Vtbl {
        unsafe extern "system" fn Container<Impl: IManipulationDeltaRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Container() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Impl: IManipulationDeltaRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsInertial<Impl: IManipulationDeltaRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsInertial() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delta<Impl: IManipulationDeltaRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Input::ManipulationDelta) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Delta() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cumulative<Impl: IManipulationDeltaRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Input::ManipulationDelta) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cumulative() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Velocities<Impl: IManipulationDeltaRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Input::ManipulationVelocities) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Velocities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handled<Impl: IManipulationDeltaRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IManipulationDeltaRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn PointerDeviceType<Impl: IManipulationDeltaRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerDeviceType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Complete<Impl: IManipulationDeltaRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IManipulationDeltaRoutedEventArgs, BASE_OFFSET>(),
            Container: Container::<Impl, IMPL_OFFSET>,
            Position: Position::<Impl, IMPL_OFFSET>,
            IsInertial: IsInertial::<Impl, IMPL_OFFSET>,
            Delta: Delta::<Impl, IMPL_OFFSET>,
            Cumulative: Cumulative::<Impl, IMPL_OFFSET>,
            Velocities: Velocities::<Impl, IMPL_OFFSET>,
            Handled: Handled::<Impl, IMPL_OFFSET>,
            SetHandled: SetHandled::<Impl, IMPL_OFFSET>,
            PointerDeviceType: PointerDeviceType::<Impl, IMPL_OFFSET>,
            Complete: Complete::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IManipulationDeltaRoutedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Input", feature = "Foundation", feature = "UI_Input", feature = "implement_exclusive"))]
pub trait IManipulationInertiaStartingRoutedEventArgs_Impl: Sized {
    fn Container(&mut self) -> ::windows::core::Result<super::UIElement>;
    fn ExpansionBehavior(&mut self) -> ::windows::core::Result<InertiaExpansionBehavior>;
    fn SetExpansionBehavior(&mut self, value: &::core::option::Option<InertiaExpansionBehavior>) -> ::windows::core::Result<()>;
    fn RotationBehavior(&mut self) -> ::windows::core::Result<InertiaRotationBehavior>;
    fn SetRotationBehavior(&mut self, value: &::core::option::Option<InertiaRotationBehavior>) -> ::windows::core::Result<()>;
    fn TranslationBehavior(&mut self) -> ::windows::core::Result<InertiaTranslationBehavior>;
    fn SetTranslationBehavior(&mut self, value: &::core::option::Option<InertiaTranslationBehavior>) -> ::windows::core::Result<()>;
    fn Handled(&mut self) -> ::windows::core::Result<bool>;
    fn SetHandled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn PointerDeviceType(&mut self) -> ::windows::core::Result<super::super::super::Devices::Input::PointerDeviceType>;
    fn Delta(&mut self) -> ::windows::core::Result<super::super::Input::ManipulationDelta>;
    fn Cumulative(&mut self) -> ::windows::core::Result<super::super::Input::ManipulationDelta>;
    fn Velocities(&mut self) -> ::windows::core::Result<super::super::Input::ManipulationVelocities>;
}
#[cfg(all(feature = "Devices_Input", feature = "Foundation", feature = "UI_Input", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IManipulationInertiaStartingRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IManipulationInertiaStartingRoutedEventArgs";
}
#[cfg(all(feature = "Devices_Input", feature = "Foundation", feature = "UI_Input", feature = "implement_exclusive"))]
impl IManipulationInertiaStartingRoutedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IManipulationInertiaStartingRoutedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IManipulationInertiaStartingRoutedEventArgs_Vtbl {
        unsafe extern "system" fn Container<Impl: IManipulationInertiaStartingRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Container() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpansionBehavior<Impl: IManipulationInertiaStartingRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExpansionBehavior() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExpansionBehavior<Impl: IManipulationInertiaStartingRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExpansionBehavior(&*(&value as *const <InertiaExpansionBehavior as ::windows::core::Abi>::Abi as *const <InertiaExpansionBehavior as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RotationBehavior<Impl: IManipulationInertiaStartingRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RotationBehavior() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotationBehavior<Impl: IManipulationInertiaStartingRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRotationBehavior(&*(&value as *const <InertiaRotationBehavior as ::windows::core::Abi>::Abi as *const <InertiaRotationBehavior as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TranslationBehavior<Impl: IManipulationInertiaStartingRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TranslationBehavior() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTranslationBehavior<Impl: IManipulationInertiaStartingRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTranslationBehavior(&*(&value as *const <InertiaTranslationBehavior as ::windows::core::Abi>::Abi as *const <InertiaTranslationBehavior as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Handled<Impl: IManipulationInertiaStartingRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IManipulationInertiaStartingRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn PointerDeviceType<Impl: IManipulationInertiaStartingRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerDeviceType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delta<Impl: IManipulationInertiaStartingRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Input::ManipulationDelta) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Delta() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cumulative<Impl: IManipulationInertiaStartingRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Input::ManipulationDelta) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cumulative() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Velocities<Impl: IManipulationInertiaStartingRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Input::ManipulationVelocities) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Velocities() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IManipulationInertiaStartingRoutedEventArgs, BASE_OFFSET>(),
            Container: Container::<Impl, IMPL_OFFSET>,
            ExpansionBehavior: ExpansionBehavior::<Impl, IMPL_OFFSET>,
            SetExpansionBehavior: SetExpansionBehavior::<Impl, IMPL_OFFSET>,
            RotationBehavior: RotationBehavior::<Impl, IMPL_OFFSET>,
            SetRotationBehavior: SetRotationBehavior::<Impl, IMPL_OFFSET>,
            TranslationBehavior: TranslationBehavior::<Impl, IMPL_OFFSET>,
            SetTranslationBehavior: SetTranslationBehavior::<Impl, IMPL_OFFSET>,
            Handled: Handled::<Impl, IMPL_OFFSET>,
            SetHandled: SetHandled::<Impl, IMPL_OFFSET>,
            PointerDeviceType: PointerDeviceType::<Impl, IMPL_OFFSET>,
            Delta: Delta::<Impl, IMPL_OFFSET>,
            Cumulative: Cumulative::<Impl, IMPL_OFFSET>,
            Velocities: Velocities::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IManipulationInertiaStartingRoutedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IManipulationPivot_Impl: Sized {
    fn Center(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn SetCenter(&mut self, value: &super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn Radius(&mut self) -> ::windows::core::Result<f64>;
    fn SetRadius(&mut self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IManipulationPivot {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IManipulationPivot";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IManipulationPivot_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IManipulationPivot_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IManipulationPivot_Vtbl {
        unsafe extern "system" fn Center<Impl: IManipulationPivot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Center() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCenter<Impl: IManipulationPivot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenter(&*(&value as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Radius<Impl: IManipulationPivot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Radius() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRadius<Impl: IManipulationPivot_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRadius(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IManipulationPivot, BASE_OFFSET>(),
            Center: Center::<Impl, IMPL_OFFSET>,
            SetCenter: SetCenter::<Impl, IMPL_OFFSET>,
            Radius: Radius::<Impl, IMPL_OFFSET>,
            SetRadius: SetRadius::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IManipulationPivot as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IManipulationPivotFactory_Impl: Sized {
    fn CreateInstanceWithCenterAndRadius(&mut self, center: &super::super::super::Foundation::Point, radius: f64) -> ::windows::core::Result<ManipulationPivot>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IManipulationPivotFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IManipulationPivotFactory";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IManipulationPivotFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IManipulationPivotFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IManipulationPivotFactory_Vtbl {
        unsafe extern "system" fn CreateInstanceWithCenterAndRadius<Impl: IManipulationPivotFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, center: super::super::super::Foundation::Point, radius: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithCenterAndRadius(&*(&center as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType), radius) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IManipulationPivotFactory, BASE_OFFSET>(),
            CreateInstanceWithCenterAndRadius: CreateInstanceWithCenterAndRadius::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IManipulationPivotFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Input", feature = "Foundation", feature = "UI_Input", feature = "implement_exclusive"))]
pub trait IManipulationStartedRoutedEventArgs_Impl: Sized {
    fn Container(&mut self) -> ::windows::core::Result<super::UIElement>;
    fn Position(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn Handled(&mut self) -> ::windows::core::Result<bool>;
    fn SetHandled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn PointerDeviceType(&mut self) -> ::windows::core::Result<super::super::super::Devices::Input::PointerDeviceType>;
    fn Cumulative(&mut self) -> ::windows::core::Result<super::super::Input::ManipulationDelta>;
    fn Complete(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Devices_Input", feature = "Foundation", feature = "UI_Input", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IManipulationStartedRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IManipulationStartedRoutedEventArgs";
}
#[cfg(all(feature = "Devices_Input", feature = "Foundation", feature = "UI_Input", feature = "implement_exclusive"))]
impl IManipulationStartedRoutedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IManipulationStartedRoutedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IManipulationStartedRoutedEventArgs_Vtbl {
        unsafe extern "system" fn Container<Impl: IManipulationStartedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Container() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Impl: IManipulationStartedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Handled<Impl: IManipulationStartedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IManipulationStartedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn PointerDeviceType<Impl: IManipulationStartedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerDeviceType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cumulative<Impl: IManipulationStartedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Input::ManipulationDelta) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cumulative() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Complete<Impl: IManipulationStartedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IManipulationStartedRoutedEventArgs, BASE_OFFSET>(),
            Container: Container::<Impl, IMPL_OFFSET>,
            Position: Position::<Impl, IMPL_OFFSET>,
            Handled: Handled::<Impl, IMPL_OFFSET>,
            SetHandled: SetHandled::<Impl, IMPL_OFFSET>,
            PointerDeviceType: PointerDeviceType::<Impl, IMPL_OFFSET>,
            Cumulative: Cumulative::<Impl, IMPL_OFFSET>,
            Complete: Complete::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IManipulationStartedRoutedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IManipulationStartedRoutedEventArgsFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ManipulationStartedRoutedEventArgs>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IManipulationStartedRoutedEventArgsFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IManipulationStartedRoutedEventArgsFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IManipulationStartedRoutedEventArgsFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IManipulationStartedRoutedEventArgsFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IManipulationStartedRoutedEventArgsFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IManipulationStartedRoutedEventArgsFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IManipulationStartedRoutedEventArgsFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IManipulationStartedRoutedEventArgsFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IManipulationStartingRoutedEventArgs_Impl: Sized {
    fn Mode(&mut self) -> ::windows::core::Result<ManipulationModes>;
    fn SetMode(&mut self, value: ManipulationModes) -> ::windows::core::Result<()>;
    fn Container(&mut self) -> ::windows::core::Result<super::UIElement>;
    fn SetContainer(&mut self, value: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn Pivot(&mut self) -> ::windows::core::Result<ManipulationPivot>;
    fn SetPivot(&mut self, value: &::core::option::Option<ManipulationPivot>) -> ::windows::core::Result<()>;
    fn Handled(&mut self) -> ::windows::core::Result<bool>;
    fn SetHandled(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IManipulationStartingRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IManipulationStartingRoutedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IManipulationStartingRoutedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IManipulationStartingRoutedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IManipulationStartingRoutedEventArgs_Vtbl {
        unsafe extern "system" fn Mode<Impl: IManipulationStartingRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ManipulationModes) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Mode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMode<Impl: IManipulationStartingRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ManipulationModes) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMode(value).into()
        }
        unsafe extern "system" fn Container<Impl: IManipulationStartingRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Container() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContainer<Impl: IManipulationStartingRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContainer(&*(&value as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Pivot<Impl: IManipulationStartingRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pivot() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPivot<Impl: IManipulationStartingRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPivot(&*(&value as *const <ManipulationPivot as ::windows::core::Abi>::Abi as *const <ManipulationPivot as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Handled<Impl: IManipulationStartingRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IManipulationStartingRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IManipulationStartingRoutedEventArgs, BASE_OFFSET>(),
            Mode: Mode::<Impl, IMPL_OFFSET>,
            SetMode: SetMode::<Impl, IMPL_OFFSET>,
            Container: Container::<Impl, IMPL_OFFSET>,
            SetContainer: SetContainer::<Impl, IMPL_OFFSET>,
            Pivot: Pivot::<Impl, IMPL_OFFSET>,
            SetPivot: SetPivot::<Impl, IMPL_OFFSET>,
            Handled: Handled::<Impl, IMPL_OFFSET>,
            SetHandled: SetHandled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IManipulationStartingRoutedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait INoFocusCandidateFoundEventArgs_Impl: Sized {
    fn Direction(&mut self) -> ::windows::core::Result<FocusNavigationDirection>;
    fn Handled(&mut self) -> ::windows::core::Result<bool>;
    fn SetHandled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn InputDevice(&mut self) -> ::windows::core::Result<FocusInputDeviceKind>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for INoFocusCandidateFoundEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.INoFocusCandidateFoundEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl INoFocusCandidateFoundEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INoFocusCandidateFoundEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INoFocusCandidateFoundEventArgs_Vtbl {
        unsafe extern "system" fn Direction<Impl: INoFocusCandidateFoundEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FocusNavigationDirection) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Direction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handled<Impl: INoFocusCandidateFoundEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: INoFocusCandidateFoundEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn InputDevice<Impl: INoFocusCandidateFoundEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FocusInputDeviceKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InputDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, INoFocusCandidateFoundEventArgs, BASE_OFFSET>(),
            Direction: Direction::<Impl, IMPL_OFFSET>,
            Handled: Handled::<Impl, IMPL_OFFSET>,
            SetHandled: SetHandled::<Impl, IMPL_OFFSET>,
            InputDevice: InputDevice::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INoFocusCandidateFoundEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Input", feature = "implement_exclusive"))]
pub trait IPointer_Impl: Sized {
    fn PointerId(&mut self) -> ::windows::core::Result<u32>;
    fn PointerDeviceType(&mut self) -> ::windows::core::Result<super::super::super::Devices::Input::PointerDeviceType>;
    fn IsInContact(&mut self) -> ::windows::core::Result<bool>;
    fn IsInRange(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Devices_Input", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPointer {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IPointer";
}
#[cfg(all(feature = "Devices_Input", feature = "implement_exclusive"))]
impl IPointer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPointer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPointer_Vtbl {
        unsafe extern "system" fn PointerId<Impl: IPointer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PointerDeviceType<Impl: IPointer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerDeviceType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInContact<Impl: IPointer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsInContact() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInRange<Impl: IPointer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsInRange() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPointer, BASE_OFFSET>(),
            PointerId: PointerId::<Impl, IMPL_OFFSET>,
            PointerDeviceType: PointerDeviceType::<Impl, IMPL_OFFSET>,
            IsInContact: IsInContact::<Impl, IMPL_OFFSET>,
            IsInRange: IsInRange::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPointer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "System", feature = "UI_Input", feature = "implement_exclusive"))]
pub trait IPointerRoutedEventArgs_Impl: Sized {
    fn Pointer(&mut self) -> ::windows::core::Result<Pointer>;
    fn KeyModifiers(&mut self) -> ::windows::core::Result<super::super::super::System::VirtualKeyModifiers>;
    fn Handled(&mut self) -> ::windows::core::Result<bool>;
    fn SetHandled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn GetCurrentPoint(&mut self, relativeto: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<super::super::Input::PointerPoint>;
    fn GetIntermediatePoints(&mut self, relativeto: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<super::super::Input::PointerPoint>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "System", feature = "UI_Input", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IPointerRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IPointerRoutedEventArgs";
}
#[cfg(all(feature = "Foundation_Collections", feature = "System", feature = "UI_Input", feature = "implement_exclusive"))]
impl IPointerRoutedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPointerRoutedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPointerRoutedEventArgs_Vtbl {
        unsafe extern "system" fn Pointer<Impl: IPointerRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Pointer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyModifiers<Impl: IPointerRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyModifiers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handled<Impl: IPointerRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IPointerRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn GetCurrentPoint<Impl: IPointerRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relativeto: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentPoint(&*(&relativeto as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIntermediatePoints<Impl: IPointerRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relativeto: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIntermediatePoints(&*(&relativeto as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPointerRoutedEventArgs, BASE_OFFSET>(),
            Pointer: Pointer::<Impl, IMPL_OFFSET>,
            KeyModifiers: KeyModifiers::<Impl, IMPL_OFFSET>,
            Handled: Handled::<Impl, IMPL_OFFSET>,
            SetHandled: SetHandled::<Impl, IMPL_OFFSET>,
            GetCurrentPoint: GetCurrentPoint::<Impl, IMPL_OFFSET>,
            GetIntermediatePoints: GetIntermediatePoints::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPointerRoutedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPointerRoutedEventArgs2_Impl: Sized {
    fn IsGenerated(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPointerRoutedEventArgs2 {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IPointerRoutedEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IPointerRoutedEventArgs2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPointerRoutedEventArgs2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPointerRoutedEventArgs2_Vtbl {
        unsafe extern "system" fn IsGenerated<Impl: IPointerRoutedEventArgs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsGenerated() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPointerRoutedEventArgs2, BASE_OFFSET>(), IsGenerated: IsGenerated::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPointerRoutedEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait IProcessKeyboardAcceleratorEventArgs_Impl: Sized {
    fn Key(&mut self) -> ::windows::core::Result<super::super::super::System::VirtualKey>;
    fn Modifiers(&mut self) -> ::windows::core::Result<super::super::super::System::VirtualKeyModifiers>;
    fn Handled(&mut self) -> ::windows::core::Result<bool>;
    fn SetHandled(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IProcessKeyboardAcceleratorEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IProcessKeyboardAcceleratorEventArgs";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl IProcessKeyboardAcceleratorEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProcessKeyboardAcceleratorEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProcessKeyboardAcceleratorEventArgs_Vtbl {
        unsafe extern "system" fn Key<Impl: IProcessKeyboardAcceleratorEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::System::VirtualKey) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Key() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Modifiers<Impl: IProcessKeyboardAcceleratorEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Modifiers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handled<Impl: IProcessKeyboardAcceleratorEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IProcessKeyboardAcceleratorEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IProcessKeyboardAcceleratorEventArgs, BASE_OFFSET>(),
            Key: Key::<Impl, IMPL_OFFSET>,
            Modifiers: Modifiers::<Impl, IMPL_OFFSET>,
            Handled: Handled::<Impl, IMPL_OFFSET>,
            SetHandled: SetHandled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProcessKeyboardAcceleratorEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Input", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IRightTappedRoutedEventArgs_Impl: Sized {
    fn PointerDeviceType(&mut self) -> ::windows::core::Result<super::super::super::Devices::Input::PointerDeviceType>;
    fn Handled(&mut self) -> ::windows::core::Result<bool>;
    fn SetHandled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn GetPosition(&mut self, relativeto: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<super::super::super::Foundation::Point>;
}
#[cfg(all(feature = "Devices_Input", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRightTappedRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IRightTappedRoutedEventArgs";
}
#[cfg(all(feature = "Devices_Input", feature = "Foundation", feature = "implement_exclusive"))]
impl IRightTappedRoutedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRightTappedRoutedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRightTappedRoutedEventArgs_Vtbl {
        unsafe extern "system" fn PointerDeviceType<Impl: IRightTappedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerDeviceType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handled<Impl: IRightTappedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: IRightTappedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn GetPosition<Impl: IRightTappedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relativeto: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPosition(&*(&relativeto as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRightTappedRoutedEventArgs, BASE_OFFSET>(),
            PointerDeviceType: PointerDeviceType::<Impl, IMPL_OFFSET>,
            Handled: Handled::<Impl, IMPL_OFFSET>,
            SetHandled: SetHandled::<Impl, IMPL_OFFSET>,
            GetPosition: GetPosition::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRightTappedRoutedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStandardUICommand_Impl: Sized {
    fn Kind(&mut self) -> ::windows::core::Result<StandardUICommandKind>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStandardUICommand {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IStandardUICommand";
}
#[cfg(feature = "implement_exclusive")]
impl IStandardUICommand_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStandardUICommand_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStandardUICommand_Vtbl {
        unsafe extern "system" fn Kind<Impl: IStandardUICommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut StandardUICommandKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Kind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IStandardUICommand, BASE_OFFSET>(), Kind: Kind::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStandardUICommand as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStandardUICommand2_Impl: Sized {
    fn SetKind(&mut self, value: StandardUICommandKind) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStandardUICommand2 {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IStandardUICommand2";
}
#[cfg(feature = "implement_exclusive")]
impl IStandardUICommand2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStandardUICommand2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStandardUICommand2_Vtbl {
        unsafe extern "system" fn SetKind<Impl: IStandardUICommand2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: StandardUICommandKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKind(value).into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IStandardUICommand2, BASE_OFFSET>(), SetKind: SetKind::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStandardUICommand2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStandardUICommandFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<StandardUICommand>;
    fn CreateInstanceWithKind(&mut self, kind: StandardUICommandKind, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<StandardUICommand>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStandardUICommandFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IStandardUICommandFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IStandardUICommandFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStandardUICommandFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStandardUICommandFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IStandardUICommandFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateInstanceWithKind<Impl: IStandardUICommandFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, kind: StandardUICommandKind, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithKind(kind, &*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStandardUICommandFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
            CreateInstanceWithKind: CreateInstanceWithKind::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStandardUICommandFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStandardUICommandStatics_Impl: Sized {
    fn KindProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStandardUICommandStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IStandardUICommandStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IStandardUICommandStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStandardUICommandStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStandardUICommandStatics_Vtbl {
        unsafe extern "system" fn KindProperty<Impl: IStandardUICommandStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KindProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStandardUICommandStatics, BASE_OFFSET>(),
            KindProperty: KindProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStandardUICommandStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Devices_Input", feature = "Foundation", feature = "implement_exclusive"))]
pub trait ITappedRoutedEventArgs_Impl: Sized {
    fn PointerDeviceType(&mut self) -> ::windows::core::Result<super::super::super::Devices::Input::PointerDeviceType>;
    fn Handled(&mut self) -> ::windows::core::Result<bool>;
    fn SetHandled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn GetPosition(&mut self, relativeto: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<super::super::super::Foundation::Point>;
}
#[cfg(all(feature = "Devices_Input", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ITappedRoutedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.ITappedRoutedEventArgs";
}
#[cfg(all(feature = "Devices_Input", feature = "Foundation", feature = "implement_exclusive"))]
impl ITappedRoutedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITappedRoutedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITappedRoutedEventArgs_Vtbl {
        unsafe extern "system" fn PointerDeviceType<Impl: ITappedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PointerDeviceType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Handled<Impl: ITappedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Handled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHandled<Impl: ITappedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn GetPosition<Impl: ITappedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relativeto: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPosition(&*(&relativeto as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ITappedRoutedEventArgs, BASE_OFFSET>(),
            PointerDeviceType: PointerDeviceType::<Impl, IMPL_OFFSET>,
            Handled: Handled::<Impl, IMPL_OFFSET>,
            SetHandled: SetHandled::<Impl, IMPL_OFFSET>,
            GetPosition: GetPosition::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITappedRoutedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
pub trait IXamlUICommand_Impl: Sized {
    fn Label(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLabel(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IconSource(&mut self) -> ::windows::core::Result<super::Controls::IconSource>;
    fn SetIconSource(&mut self, value: &::core::option::Option<super::Controls::IconSource>) -> ::windows::core::Result<()>;
    fn KeyboardAccelerators(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<KeyboardAccelerator>>;
    fn AccessKey(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAccessKey(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Description(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Command(&mut self) -> ::windows::core::Result<ICommand>;
    fn SetCommand(&mut self, value: &::core::option::Option<ICommand>) -> ::windows::core::Result<()>;
    fn ExecuteRequested(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<XamlUICommand, ExecuteRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveExecuteRequested(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CanExecuteRequested(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<XamlUICommand, CanExecuteRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCanExecuteRequested(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn NotifyCanExecuteChanged(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IXamlUICommand {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IXamlUICommand";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "UI_Xaml_Controls", feature = "implement_exclusive"))]
impl IXamlUICommand_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlUICommand_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlUICommand_Vtbl {
        unsafe extern "system" fn Label<Impl: IXamlUICommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Label() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLabel<Impl: IXamlUICommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLabel(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IconSource<Impl: IXamlUICommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIconSource<Impl: IXamlUICommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIconSource(&*(&value as *const <super::Controls::IconSource as ::windows::core::Abi>::Abi as *const <super::Controls::IconSource as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn KeyboardAccelerators<Impl: IXamlUICommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyboardAccelerators() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AccessKey<Impl: IXamlUICommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccessKey() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccessKey<Impl: IXamlUICommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAccessKey(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Description<Impl: IXamlUICommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IXamlUICommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Command<Impl: IXamlUICommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCommand<Impl: IXamlUICommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCommand(&*(&value as *const <ICommand as ::windows::core::Abi>::Abi as *const <ICommand as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ExecuteRequested<Impl: IXamlUICommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExecuteRequested(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<XamlUICommand, ExecuteRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<XamlUICommand, ExecuteRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveExecuteRequested<Impl: IXamlUICommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveExecuteRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CanExecuteRequested<Impl: IXamlUICommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanExecuteRequested(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<XamlUICommand, CanExecuteRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<XamlUICommand, CanExecuteRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCanExecuteRequested<Impl: IXamlUICommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCanExecuteRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NotifyCanExecuteChanged<Impl: IXamlUICommand_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyCanExecuteChanged().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXamlUICommand, BASE_OFFSET>(),
            Label: Label::<Impl, IMPL_OFFSET>,
            SetLabel: SetLabel::<Impl, IMPL_OFFSET>,
            IconSource: IconSource::<Impl, IMPL_OFFSET>,
            SetIconSource: SetIconSource::<Impl, IMPL_OFFSET>,
            KeyboardAccelerators: KeyboardAccelerators::<Impl, IMPL_OFFSET>,
            AccessKey: AccessKey::<Impl, IMPL_OFFSET>,
            SetAccessKey: SetAccessKey::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            SetDescription: SetDescription::<Impl, IMPL_OFFSET>,
            Command: Command::<Impl, IMPL_OFFSET>,
            SetCommand: SetCommand::<Impl, IMPL_OFFSET>,
            ExecuteRequested: ExecuteRequested::<Impl, IMPL_OFFSET>,
            RemoveExecuteRequested: RemoveExecuteRequested::<Impl, IMPL_OFFSET>,
            CanExecuteRequested: CanExecuteRequested::<Impl, IMPL_OFFSET>,
            RemoveCanExecuteRequested: RemoveCanExecuteRequested::<Impl, IMPL_OFFSET>,
            NotifyCanExecuteChanged: NotifyCanExecuteChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlUICommand as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlUICommandFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<XamlUICommand>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXamlUICommandFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IXamlUICommandFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IXamlUICommandFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlUICommandFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlUICommandFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IXamlUICommandFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXamlUICommandFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlUICommandFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlUICommandStatics_Impl: Sized {
    fn LabelProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn IconSourceProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn KeyboardAcceleratorsProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn AccessKeyProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn DescriptionProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn CommandProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXamlUICommandStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IXamlUICommandStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IXamlUICommandStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlUICommandStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlUICommandStatics_Vtbl {
        unsafe extern "system" fn LabelProperty<Impl: IXamlUICommandStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LabelProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IconSourceProperty<Impl: IXamlUICommandStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IconSourceProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyboardAcceleratorsProperty<Impl: IXamlUICommandStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyboardAcceleratorsProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AccessKeyProperty<Impl: IXamlUICommandStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DescriptionProperty<Impl: IXamlUICommandStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DescriptionProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommandProperty<Impl: IXamlUICommandStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXamlUICommandStatics, BASE_OFFSET>(),
            LabelProperty: LabelProperty::<Impl, IMPL_OFFSET>,
            IconSourceProperty: IconSourceProperty::<Impl, IMPL_OFFSET>,
            KeyboardAcceleratorsProperty: KeyboardAcceleratorsProperty::<Impl, IMPL_OFFSET>,
            AccessKeyProperty: AccessKeyProperty::<Impl, IMPL_OFFSET>,
            DescriptionProperty: DescriptionProperty::<Impl, IMPL_OFFSET>,
            CommandProperty: CommandProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlUICommandStatics as ::windows::core::Interface>::IID
    }
}

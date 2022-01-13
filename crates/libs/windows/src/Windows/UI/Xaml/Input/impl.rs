#[cfg(feature = "implement_exclusive")]
pub trait IAccessKeyDisplayDismissedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAccessKeyDisplayDismissedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IAccessKeyDisplayDismissedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAccessKeyDisplayDismissedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccessKeyDisplayDismissedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccessKeyDisplayDismissedEventArgsVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAccessKeyDisplayDismissedEventArgs, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccessKeyDisplayDismissedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccessKeyDisplayRequestedEventArgsImpl: Sized {
    fn PressedKeys(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAccessKeyDisplayRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IAccessKeyDisplayRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAccessKeyDisplayRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccessKeyDisplayRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccessKeyDisplayRequestedEventArgsVtbl {
        unsafe extern "system" fn PressedKeys<Impl: IAccessKeyDisplayRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
pub trait IAccessKeyInvokedEventArgsImpl: Sized {
    fn Handled(&mut self) -> ::windows::core::Result<bool>;
    fn SetHandled(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAccessKeyInvokedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IAccessKeyInvokedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAccessKeyInvokedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccessKeyInvokedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccessKeyInvokedEventArgsVtbl {
        unsafe extern "system" fn Handled<Impl: IAccessKeyInvokedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHandled<Impl: IAccessKeyInvokedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
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
pub trait IAccessKeyManagerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAccessKeyManager {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IAccessKeyManager";
}
#[cfg(feature = "implement_exclusive")]
impl IAccessKeyManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccessKeyManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccessKeyManagerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAccessKeyManager, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccessKeyManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAccessKeyManagerStaticsImpl: Sized {
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
impl IAccessKeyManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccessKeyManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccessKeyManagerStaticsVtbl {
        unsafe extern "system" fn IsDisplayModeEnabled<Impl: IAccessKeyManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsDisplayModeEnabledChanged<Impl: IAccessKeyManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveIsDisplayModeEnabledChanged<Impl: IAccessKeyManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveIsDisplayModeEnabledChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ExitDisplayMode<Impl: IAccessKeyManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IAccessKeyManagerStatics2Impl: Sized {
    fn AreKeyTipsEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetAreKeyTipsEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAccessKeyManagerStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IAccessKeyManagerStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IAccessKeyManagerStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccessKeyManagerStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccessKeyManagerStatics2Vtbl {
        unsafe extern "system" fn AreKeyTipsEnabled<Impl: IAccessKeyManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAreKeyTipsEnabled<Impl: IAccessKeyManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
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
pub trait ICanExecuteRequestedEventArgsImpl: Sized {
    fn Parameter(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CanExecute(&mut self) -> ::windows::core::Result<bool>;
    fn SetCanExecute(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICanExecuteRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.ICanExecuteRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl ICanExecuteRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICanExecuteRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICanExecuteRequestedEventArgsVtbl {
        unsafe extern "system" fn Parameter<Impl: ICanExecuteRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CanExecute<Impl: ICanExecuteRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCanExecute<Impl: ICanExecuteRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
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
pub trait ICharacterReceivedRoutedEventArgsImpl: Sized {
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
impl ICharacterReceivedRoutedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICharacterReceivedRoutedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICharacterReceivedRoutedEventArgsVtbl {
        unsafe extern "system" fn Character<Impl: ICharacterReceivedRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn KeyStatus<Impl: ICharacterReceivedRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Core::CorePhysicalKeyStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Handled<Impl: ICharacterReceivedRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHandled<Impl: ICharacterReceivedRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
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
pub trait ICommandImpl: Sized {
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
impl ICommandVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICommandImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICommandVtbl {
        unsafe extern "system" fn CanExecuteChanged<Impl: ICommandImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveCanExecuteChanged<Impl: ICommandImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCanExecuteChanged(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CanExecute<Impl: ICommandImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameter: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Execute<Impl: ICommandImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IContextRequestedEventArgsImpl: Sized {
    fn Handled(&mut self) -> ::windows::core::Result<bool>;
    fn SetHandled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn TryGetPosition(&mut self, relativeto: &::core::option::Option<super::UIElement>, point: &mut super::super::super::Foundation::Point) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IContextRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IContextRequestedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IContextRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContextRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContextRequestedEventArgsVtbl {
        unsafe extern "system" fn Handled<Impl: IContextRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHandled<Impl: IContextRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn TryGetPosition<Impl: IContextRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relativeto: ::windows::core::RawPtr, point: *mut super::super::super::Foundation::Point, result__: *mut bool) -> ::windows::core::HRESULT {
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
pub trait IDoubleTappedRoutedEventArgsImpl: Sized {
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
impl IDoubleTappedRoutedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDoubleTappedRoutedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDoubleTappedRoutedEventArgsVtbl {
        unsafe extern "system" fn PointerDeviceType<Impl: IDoubleTappedRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Handled<Impl: IDoubleTappedRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHandled<Impl: IDoubleTappedRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn GetPosition<Impl: IDoubleTappedRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relativeto: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
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
pub trait IExecuteRequestedEventArgsImpl: Sized {
    fn Parameter(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IExecuteRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IExecuteRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IExecuteRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExecuteRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IExecuteRequestedEventArgsVtbl {
        unsafe extern "system" fn Parameter<Impl: IExecuteRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IFindNextElementOptionsImpl: Sized {
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
impl IFindNextElementOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFindNextElementOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFindNextElementOptionsVtbl {
        unsafe extern "system" fn SearchRoot<Impl: IFindNextElementOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSearchRoot<Impl: IFindNextElementOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSearchRoot(&*(&value as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ExclusionRect<Impl: IFindNextElementOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetExclusionRect<Impl: IFindNextElementOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExclusionRect(&*(&value as *const <super::super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn HintRect<Impl: IFindNextElementOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHintRect<Impl: IFindNextElementOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHintRect(&*(&value as *const <super::super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn XYFocusNavigationStrategyOverride<Impl: IFindNextElementOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut XYFocusNavigationStrategyOverride) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetXYFocusNavigationStrategyOverride<Impl: IFindNextElementOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: XYFocusNavigationStrategyOverride) -> ::windows::core::HRESULT {
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
pub trait IFocusManagerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFocusManager {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IFocusManager";
}
#[cfg(feature = "implement_exclusive")]
impl IFocusManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFocusManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFocusManagerVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IFocusManager, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFocusManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IFocusManagerGotFocusEventArgsImpl: Sized {
    fn NewFocusedElement(&mut self) -> ::windows::core::Result<super::DependencyObject>;
    fn CorrelationId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFocusManagerGotFocusEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IFocusManagerGotFocusEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IFocusManagerGotFocusEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFocusManagerGotFocusEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFocusManagerGotFocusEventArgsVtbl {
        unsafe extern "system" fn NewFocusedElement<Impl: IFocusManagerGotFocusEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CorrelationId<Impl: IFocusManagerGotFocusEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
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
pub trait IFocusManagerLostFocusEventArgsImpl: Sized {
    fn OldFocusedElement(&mut self) -> ::windows::core::Result<super::DependencyObject>;
    fn CorrelationId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFocusManagerLostFocusEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IFocusManagerLostFocusEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IFocusManagerLostFocusEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFocusManagerLostFocusEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFocusManagerLostFocusEventArgsVtbl {
        unsafe extern "system" fn OldFocusedElement<Impl: IFocusManagerLostFocusEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CorrelationId<Impl: IFocusManagerLostFocusEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
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
pub trait IFocusManagerStaticsImpl: Sized {
    fn GetFocusedElement(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFocusManagerStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IFocusManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IFocusManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFocusManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFocusManagerStaticsVtbl {
        unsafe extern "system" fn GetFocusedElement<Impl: IFocusManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IFocusManagerStatics2Impl: Sized {
    fn TryMoveFocus(&mut self, focusnavigationdirection: FocusNavigationDirection) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFocusManagerStatics2 {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IFocusManagerStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IFocusManagerStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFocusManagerStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFocusManagerStatics2Vtbl {
        unsafe extern "system" fn TryMoveFocus<Impl: IFocusManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, focusnavigationdirection: FocusNavigationDirection, result__: *mut bool) -> ::windows::core::HRESULT {
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
pub trait IFocusManagerStatics3Impl: Sized {
    fn FindNextFocusableElement(&mut self, focusnavigationdirection: FocusNavigationDirection) -> ::windows::core::Result<super::UIElement>;
    fn FindNextFocusableElementWithHint(&mut self, focusnavigationdirection: FocusNavigationDirection, hintrect: &super::super::super::Foundation::Rect) -> ::windows::core::Result<super::UIElement>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFocusManagerStatics3 {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IFocusManagerStatics3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IFocusManagerStatics3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFocusManagerStatics3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFocusManagerStatics3Vtbl {
        unsafe extern "system" fn FindNextFocusableElement<Impl: IFocusManagerStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, focusnavigationdirection: FocusNavigationDirection, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindNextFocusableElementWithHint<Impl: IFocusManagerStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, focusnavigationdirection: FocusNavigationDirection, hintrect: super::super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IFocusManagerStatics4Impl: Sized {
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
impl IFocusManagerStatics4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFocusManagerStatics4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFocusManagerStatics4Vtbl {
        unsafe extern "system" fn TryMoveFocusWithOptions<Impl: IFocusManagerStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, focusnavigationdirection: FocusNavigationDirection, focusnavigationoptions: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindNextElement<Impl: IFocusManagerStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, focusnavigationdirection: FocusNavigationDirection, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindFirstFocusableElement<Impl: IFocusManagerStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, searchscope: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindLastFocusableElement<Impl: IFocusManagerStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, searchscope: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FindNextElementWithOptions<Impl: IFocusManagerStatics4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, focusnavigationdirection: FocusNavigationDirection, focusnavigationoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IFocusManagerStatics5Impl: Sized {
    fn TryFocusAsync(&mut self, element: &::core::option::Option<super::DependencyObject>, value: super::FocusState) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<FocusMovementResult>>;
    fn TryMoveFocusAsync(&mut self, focusnavigationdirection: FocusNavigationDirection) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<FocusMovementResult>>;
    fn TryMoveFocusWithOptionsAsync(&mut self, focusnavigationdirection: FocusNavigationDirection, focusnavigationoptions: &::core::option::Option<FindNextElementOptions>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<FocusMovementResult>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IFocusManagerStatics5 {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IFocusManagerStatics5";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IFocusManagerStatics5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFocusManagerStatics5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFocusManagerStatics5Vtbl {
        unsafe extern "system" fn TryFocusAsync<Impl: IFocusManagerStatics5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: super::FocusState, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryMoveFocusAsync<Impl: IFocusManagerStatics5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, focusnavigationdirection: FocusNavigationDirection, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TryMoveFocusWithOptionsAsync<Impl: IFocusManagerStatics5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, focusnavigationdirection: FocusNavigationDirection, focusnavigationoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IFocusManagerStatics6Impl: Sized {
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
impl IFocusManagerStatics6Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFocusManagerStatics6Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFocusManagerStatics6Vtbl {
        unsafe extern "system" fn GotFocus<Impl: IFocusManagerStatics6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveGotFocus<Impl: IFocusManagerStatics6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveGotFocus(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LostFocus<Impl: IFocusManagerStatics6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveLostFocus<Impl: IFocusManagerStatics6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveLostFocus(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GettingFocus<Impl: IFocusManagerStatics6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveGettingFocus<Impl: IFocusManagerStatics6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveGettingFocus(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LosingFocus<Impl: IFocusManagerStatics6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveLosingFocus<Impl: IFocusManagerStatics6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
pub trait IFocusManagerStatics7Impl: Sized {
    fn GetFocusedElement(&mut self, xamlroot: &::core::option::Option<super::XamlRoot>) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFocusManagerStatics7 {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IFocusManagerStatics7";
}
#[cfg(feature = "implement_exclusive")]
impl IFocusManagerStatics7Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFocusManagerStatics7Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFocusManagerStatics7Vtbl {
        unsafe extern "system" fn GetFocusedElement<Impl: IFocusManagerStatics7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xamlroot: ::windows::core::RawPtr, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IFocusMovementResultImpl: Sized {
    fn Succeeded(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IFocusMovementResult {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IFocusMovementResult";
}
#[cfg(feature = "implement_exclusive")]
impl IFocusMovementResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFocusMovementResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFocusMovementResultVtbl {
        unsafe extern "system" fn Succeeded<Impl: IFocusMovementResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
pub trait IGettingFocusEventArgsImpl: Sized {
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
impl IGettingFocusEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGettingFocusEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGettingFocusEventArgsVtbl {
        unsafe extern "system" fn OldFocusedElement<Impl: IGettingFocusEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NewFocusedElement<Impl: IGettingFocusEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetNewFocusedElement<Impl: IGettingFocusEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNewFocusedElement(&*(&value as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FocusState<Impl: IGettingFocusEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::FocusState) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Direction<Impl: IGettingFocusEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FocusNavigationDirection) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Handled<Impl: IGettingFocusEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHandled<Impl: IGettingFocusEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn InputDevice<Impl: IGettingFocusEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FocusInputDeviceKind) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Cancel<Impl: IGettingFocusEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCancel<Impl: IGettingFocusEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
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
pub trait IGettingFocusEventArgs2Impl: Sized {
    fn TryCancel(&mut self) -> ::windows::core::Result<bool>;
    fn TrySetNewFocusedElement(&mut self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGettingFocusEventArgs2 {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IGettingFocusEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IGettingFocusEventArgs2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGettingFocusEventArgs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGettingFocusEventArgs2Vtbl {
        unsafe extern "system" fn TryCancel<Impl: IGettingFocusEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TrySetNewFocusedElement<Impl: IGettingFocusEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
pub trait IGettingFocusEventArgs3Impl: Sized {
    fn CorrelationId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGettingFocusEventArgs3 {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IGettingFocusEventArgs3";
}
#[cfg(feature = "implement_exclusive")]
impl IGettingFocusEventArgs3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGettingFocusEventArgs3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGettingFocusEventArgs3Vtbl {
        unsafe extern "system" fn CorrelationId<Impl: IGettingFocusEventArgs3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
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
pub trait IHoldingRoutedEventArgsImpl: Sized {
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
impl IHoldingRoutedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHoldingRoutedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHoldingRoutedEventArgsVtbl {
        unsafe extern "system" fn PointerDeviceType<Impl: IHoldingRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HoldingState<Impl: IHoldingRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Input::HoldingState) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Handled<Impl: IHoldingRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHandled<Impl: IHoldingRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn GetPosition<Impl: IHoldingRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relativeto: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
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
pub trait IInertiaExpansionBehaviorImpl: Sized {
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
impl IInertiaExpansionBehaviorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaExpansionBehaviorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInertiaExpansionBehaviorVtbl {
        unsafe extern "system" fn DesiredDeceleration<Impl: IInertiaExpansionBehaviorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDesiredDeceleration<Impl: IInertiaExpansionBehaviorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDesiredDeceleration(value).into()
        }
        unsafe extern "system" fn DesiredExpansion<Impl: IInertiaExpansionBehaviorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDesiredExpansion<Impl: IInertiaExpansionBehaviorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
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
pub trait IInertiaRotationBehaviorImpl: Sized {
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
impl IInertiaRotationBehaviorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaRotationBehaviorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInertiaRotationBehaviorVtbl {
        unsafe extern "system" fn DesiredDeceleration<Impl: IInertiaRotationBehaviorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDesiredDeceleration<Impl: IInertiaRotationBehaviorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDesiredDeceleration(value).into()
        }
        unsafe extern "system" fn DesiredRotation<Impl: IInertiaRotationBehaviorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDesiredRotation<Impl: IInertiaRotationBehaviorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
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
pub trait IInertiaTranslationBehaviorImpl: Sized {
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
impl IInertiaTranslationBehaviorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInertiaTranslationBehaviorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInertiaTranslationBehaviorVtbl {
        unsafe extern "system" fn DesiredDeceleration<Impl: IInertiaTranslationBehaviorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDesiredDeceleration<Impl: IInertiaTranslationBehaviorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDesiredDeceleration(value).into()
        }
        unsafe extern "system" fn DesiredDisplacement<Impl: IInertiaTranslationBehaviorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDesiredDisplacement<Impl: IInertiaTranslationBehaviorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
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
pub trait IInputScopeImpl: Sized {
    fn Names(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<InputScopeName>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IInputScope {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IInputScope";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IInputScopeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInputScopeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInputScopeVtbl {
        unsafe extern "system" fn Names<Impl: IInputScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IInputScopeNameImpl: Sized {
    fn NameValue(&mut self) -> ::windows::core::Result<InputScopeNameValue>;
    fn SetNameValue(&mut self, value: InputScopeNameValue) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInputScopeName {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IInputScopeName";
}
#[cfg(feature = "implement_exclusive")]
impl IInputScopeNameVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInputScopeNameImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInputScopeNameVtbl {
        unsafe extern "system" fn NameValue<Impl: IInputScopeNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut InputScopeNameValue) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetNameValue<Impl: IInputScopeNameImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: InputScopeNameValue) -> ::windows::core::HRESULT {
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
pub trait IInputScopeNameFactoryImpl: Sized {
    fn CreateInstance(&mut self, namevalue: InputScopeNameValue) -> ::windows::core::Result<InputScopeName>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IInputScopeNameFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IInputScopeNameFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IInputScopeNameFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInputScopeNameFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInputScopeNameFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IInputScopeNameFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, namevalue: InputScopeNameValue, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IKeyRoutedEventArgsImpl: Sized {
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
impl IKeyRoutedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeyRoutedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeyRoutedEventArgsVtbl {
        unsafe extern "system" fn Key<Impl: IKeyRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::System::VirtualKey) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn KeyStatus<Impl: IKeyRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Core::CorePhysicalKeyStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Handled<Impl: IKeyRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHandled<Impl: IKeyRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
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
pub trait IKeyRoutedEventArgs2Impl: Sized {
    fn OriginalKey(&mut self) -> ::windows::core::Result<super::super::super::System::VirtualKey>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IKeyRoutedEventArgs2 {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IKeyRoutedEventArgs2";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl IKeyRoutedEventArgs2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeyRoutedEventArgs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeyRoutedEventArgs2Vtbl {
        unsafe extern "system" fn OriginalKey<Impl: IKeyRoutedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::System::VirtualKey) -> ::windows::core::HRESULT {
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
pub trait IKeyRoutedEventArgs3Impl: Sized {
    fn DeviceId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeyRoutedEventArgs3 {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IKeyRoutedEventArgs3";
}
#[cfg(feature = "implement_exclusive")]
impl IKeyRoutedEventArgs3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeyRoutedEventArgs3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeyRoutedEventArgs3Vtbl {
        unsafe extern "system" fn DeviceId<Impl: IKeyRoutedEventArgs3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
pub trait IKeyboardAcceleratorImpl: Sized {
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
impl IKeyboardAcceleratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeyboardAcceleratorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeyboardAcceleratorVtbl {
        unsafe extern "system" fn Key<Impl: IKeyboardAcceleratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::System::VirtualKey) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetKey<Impl: IKeyboardAcceleratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::System::VirtualKey) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKey(value).into()
        }
        unsafe extern "system" fn Modifiers<Impl: IKeyboardAcceleratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetModifiers<Impl: IKeyboardAcceleratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetModifiers(value).into()
        }
        unsafe extern "system" fn IsEnabled<Impl: IKeyboardAcceleratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsEnabled<Impl: IKeyboardAcceleratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsEnabled(value).into()
        }
        unsafe extern "system" fn ScopeOwner<Impl: IKeyboardAcceleratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetScopeOwner<Impl: IKeyboardAcceleratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScopeOwner(&*(&value as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Invoked<Impl: IKeyboardAcceleratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveInvoked<Impl: IKeyboardAcceleratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
pub trait IKeyboardAcceleratorFactoryImpl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<KeyboardAccelerator>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeyboardAcceleratorFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IKeyboardAcceleratorFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IKeyboardAcceleratorFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeyboardAcceleratorFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeyboardAcceleratorFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IKeyboardAcceleratorFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IKeyboardAcceleratorInvokedEventArgsImpl: Sized {
    fn Handled(&mut self) -> ::windows::core::Result<bool>;
    fn SetHandled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Element(&mut self) -> ::windows::core::Result<super::DependencyObject>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeyboardAcceleratorInvokedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IKeyboardAcceleratorInvokedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IKeyboardAcceleratorInvokedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeyboardAcceleratorInvokedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeyboardAcceleratorInvokedEventArgsVtbl {
        unsafe extern "system" fn Handled<Impl: IKeyboardAcceleratorInvokedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHandled<Impl: IKeyboardAcceleratorInvokedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn Element<Impl: IKeyboardAcceleratorInvokedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IKeyboardAcceleratorInvokedEventArgs2Impl: Sized {
    fn KeyboardAccelerator(&mut self) -> ::windows::core::Result<KeyboardAccelerator>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IKeyboardAcceleratorInvokedEventArgs2 {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IKeyboardAcceleratorInvokedEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IKeyboardAcceleratorInvokedEventArgs2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeyboardAcceleratorInvokedEventArgs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeyboardAcceleratorInvokedEventArgs2Vtbl {
        unsafe extern "system" fn KeyboardAccelerator<Impl: IKeyboardAcceleratorInvokedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IKeyboardAcceleratorStaticsImpl: Sized {
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
impl IKeyboardAcceleratorStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKeyboardAcceleratorStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKeyboardAcceleratorStaticsVtbl {
        unsafe extern "system" fn KeyProperty<Impl: IKeyboardAcceleratorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ModifiersProperty<Impl: IKeyboardAcceleratorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsEnabledProperty<Impl: IKeyboardAcceleratorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ScopeOwnerProperty<Impl: IKeyboardAcceleratorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ILosingFocusEventArgsImpl: Sized {
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
impl ILosingFocusEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILosingFocusEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILosingFocusEventArgsVtbl {
        unsafe extern "system" fn OldFocusedElement<Impl: ILosingFocusEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn NewFocusedElement<Impl: ILosingFocusEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetNewFocusedElement<Impl: ILosingFocusEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNewFocusedElement(&*(&value as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FocusState<Impl: ILosingFocusEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::FocusState) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Direction<Impl: ILosingFocusEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FocusNavigationDirection) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Handled<Impl: ILosingFocusEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHandled<Impl: ILosingFocusEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn InputDevice<Impl: ILosingFocusEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FocusInputDeviceKind) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Cancel<Impl: ILosingFocusEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCancel<Impl: ILosingFocusEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
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
pub trait ILosingFocusEventArgs2Impl: Sized {
    fn TryCancel(&mut self) -> ::windows::core::Result<bool>;
    fn TrySetNewFocusedElement(&mut self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILosingFocusEventArgs2 {
    const NAME: &'static str = "Windows.UI.Xaml.Input.ILosingFocusEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl ILosingFocusEventArgs2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILosingFocusEventArgs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILosingFocusEventArgs2Vtbl {
        unsafe extern "system" fn TryCancel<Impl: ILosingFocusEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TrySetNewFocusedElement<Impl: ILosingFocusEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
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
pub trait ILosingFocusEventArgs3Impl: Sized {
    fn CorrelationId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ILosingFocusEventArgs3 {
    const NAME: &'static str = "Windows.UI.Xaml.Input.ILosingFocusEventArgs3";
}
#[cfg(feature = "implement_exclusive")]
impl ILosingFocusEventArgs3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILosingFocusEventArgs3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILosingFocusEventArgs3Vtbl {
        unsafe extern "system" fn CorrelationId<Impl: ILosingFocusEventArgs3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
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
pub trait IManipulationCompletedRoutedEventArgsImpl: Sized {
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
impl IManipulationCompletedRoutedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IManipulationCompletedRoutedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IManipulationCompletedRoutedEventArgsVtbl {
        unsafe extern "system" fn Container<Impl: IManipulationCompletedRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Position<Impl: IManipulationCompletedRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsInertial<Impl: IManipulationCompletedRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Cumulative<Impl: IManipulationCompletedRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Input::ManipulationDelta) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Velocities<Impl: IManipulationCompletedRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Input::ManipulationVelocities) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Handled<Impl: IManipulationCompletedRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHandled<Impl: IManipulationCompletedRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn PointerDeviceType<Impl: IManipulationCompletedRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT {
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
pub trait IManipulationDeltaRoutedEventArgsImpl: Sized {
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
impl IManipulationDeltaRoutedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IManipulationDeltaRoutedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IManipulationDeltaRoutedEventArgsVtbl {
        unsafe extern "system" fn Container<Impl: IManipulationDeltaRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Position<Impl: IManipulationDeltaRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsInertial<Impl: IManipulationDeltaRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Delta<Impl: IManipulationDeltaRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Input::ManipulationDelta) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Cumulative<Impl: IManipulationDeltaRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Input::ManipulationDelta) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Velocities<Impl: IManipulationDeltaRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Input::ManipulationVelocities) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Handled<Impl: IManipulationDeltaRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHandled<Impl: IManipulationDeltaRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn PointerDeviceType<Impl: IManipulationDeltaRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Complete<Impl: IManipulationDeltaRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IManipulationInertiaStartingRoutedEventArgsImpl: Sized {
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
impl IManipulationInertiaStartingRoutedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IManipulationInertiaStartingRoutedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IManipulationInertiaStartingRoutedEventArgsVtbl {
        unsafe extern "system" fn Container<Impl: IManipulationInertiaStartingRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExpansionBehavior<Impl: IManipulationInertiaStartingRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetExpansionBehavior<Impl: IManipulationInertiaStartingRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExpansionBehavior(&*(&value as *const <InertiaExpansionBehavior as ::windows::core::Abi>::Abi as *const <InertiaExpansionBehavior as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RotationBehavior<Impl: IManipulationInertiaStartingRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRotationBehavior<Impl: IManipulationInertiaStartingRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRotationBehavior(&*(&value as *const <InertiaRotationBehavior as ::windows::core::Abi>::Abi as *const <InertiaRotationBehavior as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TranslationBehavior<Impl: IManipulationInertiaStartingRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTranslationBehavior<Impl: IManipulationInertiaStartingRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTranslationBehavior(&*(&value as *const <InertiaTranslationBehavior as ::windows::core::Abi>::Abi as *const <InertiaTranslationBehavior as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Handled<Impl: IManipulationInertiaStartingRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHandled<Impl: IManipulationInertiaStartingRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn PointerDeviceType<Impl: IManipulationInertiaStartingRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Delta<Impl: IManipulationInertiaStartingRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Input::ManipulationDelta) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Cumulative<Impl: IManipulationInertiaStartingRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Input::ManipulationDelta) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Velocities<Impl: IManipulationInertiaStartingRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Input::ManipulationVelocities) -> ::windows::core::HRESULT {
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
pub trait IManipulationPivotImpl: Sized {
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
impl IManipulationPivotVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IManipulationPivotImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IManipulationPivotVtbl {
        unsafe extern "system" fn Center<Impl: IManipulationPivotImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCenter<Impl: IManipulationPivotImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCenter(&*(&value as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Radius<Impl: IManipulationPivotImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRadius<Impl: IManipulationPivotImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT {
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
pub trait IManipulationPivotFactoryImpl: Sized {
    fn CreateInstanceWithCenterAndRadius(&mut self, center: &super::super::super::Foundation::Point, radius: f64) -> ::windows::core::Result<ManipulationPivot>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IManipulationPivotFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IManipulationPivotFactory";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IManipulationPivotFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IManipulationPivotFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IManipulationPivotFactoryVtbl {
        unsafe extern "system" fn CreateInstanceWithCenterAndRadius<Impl: IManipulationPivotFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, center: super::super::super::Foundation::Point, radius: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IManipulationStartedRoutedEventArgsImpl: Sized {
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
impl IManipulationStartedRoutedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IManipulationStartedRoutedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IManipulationStartedRoutedEventArgsVtbl {
        unsafe extern "system" fn Container<Impl: IManipulationStartedRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Position<Impl: IManipulationStartedRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Handled<Impl: IManipulationStartedRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHandled<Impl: IManipulationStartedRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn PointerDeviceType<Impl: IManipulationStartedRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Cumulative<Impl: IManipulationStartedRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Input::ManipulationDelta) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Complete<Impl: IManipulationStartedRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IManipulationStartedRoutedEventArgsFactoryImpl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<ManipulationStartedRoutedEventArgs>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IManipulationStartedRoutedEventArgsFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IManipulationStartedRoutedEventArgsFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IManipulationStartedRoutedEventArgsFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IManipulationStartedRoutedEventArgsFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IManipulationStartedRoutedEventArgsFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IManipulationStartedRoutedEventArgsFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IManipulationStartingRoutedEventArgsImpl: Sized {
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
impl IManipulationStartingRoutedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IManipulationStartingRoutedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IManipulationStartingRoutedEventArgsVtbl {
        unsafe extern "system" fn Mode<Impl: IManipulationStartingRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ManipulationModes) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMode<Impl: IManipulationStartingRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ManipulationModes) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMode(value).into()
        }
        unsafe extern "system" fn Container<Impl: IManipulationStartingRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetContainer<Impl: IManipulationStartingRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContainer(&*(&value as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Pivot<Impl: IManipulationStartingRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPivot<Impl: IManipulationStartingRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPivot(&*(&value as *const <ManipulationPivot as ::windows::core::Abi>::Abi as *const <ManipulationPivot as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Handled<Impl: IManipulationStartingRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHandled<Impl: IManipulationStartingRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
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
pub trait INoFocusCandidateFoundEventArgsImpl: Sized {
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
impl INoFocusCandidateFoundEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INoFocusCandidateFoundEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INoFocusCandidateFoundEventArgsVtbl {
        unsafe extern "system" fn Direction<Impl: INoFocusCandidateFoundEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FocusNavigationDirection) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Handled<Impl: INoFocusCandidateFoundEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHandled<Impl: INoFocusCandidateFoundEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn InputDevice<Impl: INoFocusCandidateFoundEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut FocusInputDeviceKind) -> ::windows::core::HRESULT {
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
pub trait IPointerImpl: Sized {
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
impl IPointerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPointerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPointerVtbl {
        unsafe extern "system" fn PointerId<Impl: IPointerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PointerDeviceType<Impl: IPointerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsInContact<Impl: IPointerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsInRange<Impl: IPointerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
pub trait IPointerRoutedEventArgsImpl: Sized {
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
impl IPointerRoutedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPointerRoutedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPointerRoutedEventArgsVtbl {
        unsafe extern "system" fn Pointer<Impl: IPointerRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn KeyModifiers<Impl: IPointerRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Handled<Impl: IPointerRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHandled<Impl: IPointerRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn GetCurrentPoint<Impl: IPointerRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relativeto: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetIntermediatePoints<Impl: IPointerRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relativeto: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IPointerRoutedEventArgs2Impl: Sized {
    fn IsGenerated(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPointerRoutedEventArgs2 {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IPointerRoutedEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IPointerRoutedEventArgs2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPointerRoutedEventArgs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPointerRoutedEventArgs2Vtbl {
        unsafe extern "system" fn IsGenerated<Impl: IPointerRoutedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
pub trait IProcessKeyboardAcceleratorEventArgsImpl: Sized {
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
impl IProcessKeyboardAcceleratorEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProcessKeyboardAcceleratorEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProcessKeyboardAcceleratorEventArgsVtbl {
        unsafe extern "system" fn Key<Impl: IProcessKeyboardAcceleratorEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::System::VirtualKey) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Modifiers<Impl: IProcessKeyboardAcceleratorEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::System::VirtualKeyModifiers) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Handled<Impl: IProcessKeyboardAcceleratorEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHandled<Impl: IProcessKeyboardAcceleratorEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
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
pub trait IRightTappedRoutedEventArgsImpl: Sized {
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
impl IRightTappedRoutedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRightTappedRoutedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRightTappedRoutedEventArgsVtbl {
        unsafe extern "system" fn PointerDeviceType<Impl: IRightTappedRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Handled<Impl: IRightTappedRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHandled<Impl: IRightTappedRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn GetPosition<Impl: IRightTappedRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relativeto: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
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
pub trait IStandardUICommandImpl: Sized {
    fn Kind(&mut self) -> ::windows::core::Result<StandardUICommandKind>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStandardUICommand {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IStandardUICommand";
}
#[cfg(feature = "implement_exclusive")]
impl IStandardUICommandVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStandardUICommandImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStandardUICommandVtbl {
        unsafe extern "system" fn Kind<Impl: IStandardUICommandImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut StandardUICommandKind) -> ::windows::core::HRESULT {
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
pub trait IStandardUICommand2Impl: Sized {
    fn SetKind(&mut self, value: StandardUICommandKind) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStandardUICommand2 {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IStandardUICommand2";
}
#[cfg(feature = "implement_exclusive")]
impl IStandardUICommand2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStandardUICommand2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStandardUICommand2Vtbl {
        unsafe extern "system" fn SetKind<Impl: IStandardUICommand2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: StandardUICommandKind) -> ::windows::core::HRESULT {
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
pub trait IStandardUICommandFactoryImpl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<StandardUICommand>;
    fn CreateInstanceWithKind(&mut self, kind: StandardUICommandKind, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<StandardUICommand>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStandardUICommandFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IStandardUICommandFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IStandardUICommandFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStandardUICommandFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStandardUICommandFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IStandardUICommandFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateInstanceWithKind<Impl: IStandardUICommandFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, kind: StandardUICommandKind, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IStandardUICommandStaticsImpl: Sized {
    fn KindProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStandardUICommandStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IStandardUICommandStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IStandardUICommandStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStandardUICommandStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStandardUICommandStaticsVtbl {
        unsafe extern "system" fn KindProperty<Impl: IStandardUICommandStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait ITappedRoutedEventArgsImpl: Sized {
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
impl ITappedRoutedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITappedRoutedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITappedRoutedEventArgsVtbl {
        unsafe extern "system" fn PointerDeviceType<Impl: ITappedRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Devices::Input::PointerDeviceType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Handled<Impl: ITappedRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHandled<Impl: ITappedRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHandled(value).into()
        }
        unsafe extern "system" fn GetPosition<Impl: ITappedRoutedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relativeto: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
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
pub trait IXamlUICommandImpl: Sized {
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
impl IXamlUICommandVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlUICommandImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlUICommandVtbl {
        unsafe extern "system" fn Label<Impl: IXamlUICommandImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetLabel<Impl: IXamlUICommandImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLabel(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IconSource<Impl: IXamlUICommandImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIconSource<Impl: IXamlUICommandImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIconSource(&*(&value as *const <super::Controls::IconSource as ::windows::core::Abi>::Abi as *const <super::Controls::IconSource as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn KeyboardAccelerators<Impl: IXamlUICommandImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AccessKey<Impl: IXamlUICommandImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAccessKey<Impl: IXamlUICommandImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAccessKey(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Description<Impl: IXamlUICommandImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDescription<Impl: IXamlUICommandImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Command<Impl: IXamlUICommandImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCommand<Impl: IXamlUICommandImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCommand(&*(&value as *const <ICommand as ::windows::core::Abi>::Abi as *const <ICommand as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ExecuteRequested<Impl: IXamlUICommandImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveExecuteRequested<Impl: IXamlUICommandImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveExecuteRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CanExecuteRequested<Impl: IXamlUICommandImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveCanExecuteRequested<Impl: IXamlUICommandImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCanExecuteRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NotifyCanExecuteChanged<Impl: IXamlUICommandImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IXamlUICommandFactoryImpl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<XamlUICommand>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXamlUICommandFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Input.IXamlUICommandFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IXamlUICommandFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlUICommandFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlUICommandFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IXamlUICommandFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IXamlUICommandStaticsImpl: Sized {
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
impl IXamlUICommandStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlUICommandStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlUICommandStaticsVtbl {
        unsafe extern "system" fn LabelProperty<Impl: IXamlUICommandStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IconSourceProperty<Impl: IXamlUICommandStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn KeyboardAcceleratorsProperty<Impl: IXamlUICommandStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AccessKeyProperty<Impl: IXamlUICommandStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DescriptionProperty<Impl: IXamlUICommandStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CommandProperty<Impl: IXamlUICommandStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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

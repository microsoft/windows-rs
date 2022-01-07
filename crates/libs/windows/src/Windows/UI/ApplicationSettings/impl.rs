#[cfg(feature = "implement_exclusive")]
pub trait IAccountsSettingsPaneImpl: Sized {
    fn AccountCommandsRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AccountsSettingsPane, AccountsSettingsPaneCommandsRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAccountCommandsRequested(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAccountsSettingsPane {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.IAccountsSettingsPane";
}
#[cfg(feature = "implement_exclusive")]
impl IAccountsSettingsPaneVtbl {
    pub const fn new<Impl: IAccountsSettingsPaneImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAccountsSettingsPaneVtbl {
        unsafe extern "system" fn AccountCommandsRequested<Impl: IAccountsSettingsPaneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AccountCommandsRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AccountsSettingsPane, AccountsSettingsPaneCommandsRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AccountsSettingsPane, AccountsSettingsPaneCommandsRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAccountCommandsRequested<Impl: IAccountsSettingsPaneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveAccountCommandsRequested(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAccountsSettingsPane>, base.5, AccountCommandsRequested::<Impl, OFFSET>, RemoveAccountCommandsRequested::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccountsSettingsPaneCommandsRequestedEventArgsImpl: Sized {
    fn WebAccountProviderCommands(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<WebAccountProviderCommand>>;
    fn WebAccountCommands(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<WebAccountCommand>>;
    fn CredentialCommands(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<CredentialCommand>>;
    fn Commands(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SettingsCommand>>;
    fn HeaderText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetHeaderText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<AccountsSettingsPaneEventDeferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAccountsSettingsPaneCommandsRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.IAccountsSettingsPaneCommandsRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAccountsSettingsPaneCommandsRequestedEventArgsVtbl {
    pub const fn new<Impl: IAccountsSettingsPaneCommandsRequestedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAccountsSettingsPaneCommandsRequestedEventArgsVtbl {
        unsafe extern "system" fn WebAccountProviderCommands<Impl: IAccountsSettingsPaneCommandsRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WebAccountProviderCommands() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WebAccountCommands<Impl: IAccountsSettingsPaneCommandsRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WebAccountCommands() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CredentialCommands<Impl: IAccountsSettingsPaneCommandsRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CredentialCommands() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Commands<Impl: IAccountsSettingsPaneCommandsRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Commands() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HeaderText<Impl: IAccountsSettingsPaneCommandsRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHeaderText<Impl: IAccountsSettingsPaneCommandsRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetHeaderText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IAccountsSettingsPaneCommandsRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAccountsSettingsPaneCommandsRequestedEventArgs>, base.5, WebAccountProviderCommands::<Impl, OFFSET>, WebAccountCommands::<Impl, OFFSET>, CredentialCommands::<Impl, OFFSET>, Commands::<Impl, OFFSET>, HeaderText::<Impl, OFFSET>, SetHeaderText::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccountsSettingsPaneCommandsRequestedEventArgs2Impl: Sized {
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAccountsSettingsPaneCommandsRequestedEventArgs2 {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.IAccountsSettingsPaneCommandsRequestedEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IAccountsSettingsPaneCommandsRequestedEventArgs2Vtbl {
    pub const fn new<Impl: IAccountsSettingsPaneCommandsRequestedEventArgs2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAccountsSettingsPaneCommandsRequestedEventArgs2Vtbl {
        unsafe extern "system" fn User<Impl: IAccountsSettingsPaneCommandsRequestedEventArgs2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).User() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAccountsSettingsPaneCommandsRequestedEventArgs2>, base.5, User::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccountsSettingsPaneEventDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAccountsSettingsPaneEventDeferral {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.IAccountsSettingsPaneEventDeferral";
}
#[cfg(feature = "implement_exclusive")]
impl IAccountsSettingsPaneEventDeferralVtbl {
    pub const fn new<Impl: IAccountsSettingsPaneEventDeferralImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAccountsSettingsPaneEventDeferralVtbl {
        unsafe extern "system" fn Complete<Impl: IAccountsSettingsPaneEventDeferralImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAccountsSettingsPaneEventDeferral>, base.5, Complete::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccountsSettingsPaneStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<AccountsSettingsPane>;
    fn Show(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAccountsSettingsPaneStatics {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.IAccountsSettingsPaneStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAccountsSettingsPaneStaticsVtbl {
    pub const fn new<Impl: IAccountsSettingsPaneStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAccountsSettingsPaneStaticsVtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: IAccountsSettingsPaneStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForCurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Show<Impl: IAccountsSettingsPaneStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Show().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAccountsSettingsPaneStatics>, base.5, GetForCurrentView::<Impl, OFFSET>, Show::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccountsSettingsPaneStatics2Impl: Sized + IAccountsSettingsPaneStaticsImpl {
    fn ShowManageAccountsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ShowAddAccountAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAccountsSettingsPaneStatics2 {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.IAccountsSettingsPaneStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IAccountsSettingsPaneStatics2Vtbl {
    pub const fn new<Impl: IAccountsSettingsPaneStatics2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAccountsSettingsPaneStatics2Vtbl {
        unsafe extern "system" fn ShowManageAccountsAsync<Impl: IAccountsSettingsPaneStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShowManageAccountsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowAddAccountAsync<Impl: IAccountsSettingsPaneStatics2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShowAddAccountAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAccountsSettingsPaneStatics2>, base.5, ShowManageAccountsAsync::<Impl, OFFSET>, ShowAddAccountAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccountsSettingsPaneStatics3Impl: Sized {
    fn ShowManageAccountsForUserAsync(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ShowAddAccountForUserAsync(&self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAccountsSettingsPaneStatics3 {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.IAccountsSettingsPaneStatics3";
}
#[cfg(feature = "implement_exclusive")]
impl IAccountsSettingsPaneStatics3Vtbl {
    pub const fn new<Impl: IAccountsSettingsPaneStatics3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAccountsSettingsPaneStatics3Vtbl {
        unsafe extern "system" fn ShowManageAccountsForUserAsync<Impl: IAccountsSettingsPaneStatics3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShowManageAccountsForUserAsync(&*(&user as *const <super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::System::User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowAddAccountForUserAsync<Impl: IAccountsSettingsPaneStatics3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShowAddAccountForUserAsync(&*(&user as *const <super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::System::User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAccountsSettingsPaneStatics3>, base.5, ShowManageAccountsForUserAsync::<Impl, OFFSET>, ShowAddAccountForUserAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICredentialCommandImpl: Sized {
    fn PasswordCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential>;
    fn CredentialDeleted(&self) -> ::windows::core::Result<CredentialCommandCredentialDeletedHandler>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICredentialCommand {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.ICredentialCommand";
}
#[cfg(feature = "implement_exclusive")]
impl ICredentialCommandVtbl {
    pub const fn new<Impl: ICredentialCommandImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICredentialCommandVtbl {
        unsafe extern "system" fn PasswordCredential<Impl: ICredentialCommandImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PasswordCredential() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CredentialDeleted<Impl: ICredentialCommandImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CredentialDeleted() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICredentialCommand>, base.5, PasswordCredential::<Impl, OFFSET>, CredentialDeleted::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICredentialCommandFactoryImpl: Sized {
    fn CreateCredentialCommand(&self, passwordcredential: &::core::option::Option<super::super::Security::Credentials::PasswordCredential>) -> ::windows::core::Result<CredentialCommand>;
    fn CreateCredentialCommandWithHandler(&self, passwordcredential: &::core::option::Option<super::super::Security::Credentials::PasswordCredential>, deleted: &::core::option::Option<CredentialCommandCredentialDeletedHandler>) -> ::windows::core::Result<CredentialCommand>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICredentialCommandFactory {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.ICredentialCommandFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ICredentialCommandFactoryVtbl {
    pub const fn new<Impl: ICredentialCommandFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICredentialCommandFactoryVtbl {
        unsafe extern "system" fn CreateCredentialCommand<Impl: ICredentialCommandFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, passwordcredential: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateCredentialCommand(&*(&passwordcredential as *const <super::super::Security::Credentials::PasswordCredential as ::windows::core::Abi>::Abi as *const <super::super::Security::Credentials::PasswordCredential as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCredentialCommandWithHandler<Impl: ICredentialCommandFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, passwordcredential: ::windows::core::RawPtr, deleted: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateCredentialCommandWithHandler(&*(&passwordcredential as *const <super::super::Security::Credentials::PasswordCredential as ::windows::core::Abi>::Abi as *const <super::super::Security::Credentials::PasswordCredential as ::windows::core::DefaultType>::DefaultType), &*(&deleted as *const <CredentialCommandCredentialDeletedHandler as ::windows::core::Abi>::Abi as *const <CredentialCommandCredentialDeletedHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICredentialCommandFactory>, base.5, CreateCredentialCommand::<Impl, OFFSET>, CreateCredentialCommandWithHandler::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISettingsCommandFactoryImpl: Sized {
    fn CreateSettingsCommand(&self, settingscommandid: &::core::option::Option<::windows::core::IInspectable>, label: &::windows::core::HSTRING, handler: &::core::option::Option<super::Popups::UICommandInvokedHandler>) -> ::windows::core::Result<SettingsCommand>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISettingsCommandFactory {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.ISettingsCommandFactory";
}
#[cfg(feature = "implement_exclusive")]
impl ISettingsCommandFactoryVtbl {
    pub const fn new<Impl: ISettingsCommandFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISettingsCommandFactoryVtbl {
        unsafe extern "system" fn CreateSettingsCommand<Impl: ISettingsCommandFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, settingscommandid: *mut ::core::ffi::c_void, label: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, handler: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateSettingsCommand(
                &*(&settingscommandid as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                &*(&label as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&handler as *const <super::Popups::UICommandInvokedHandler as ::windows::core::Abi>::Abi as *const <super::Popups::UICommandInvokedHandler as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISettingsCommandFactory>, base.5, CreateSettingsCommand::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISettingsCommandStaticsImpl: Sized {
    fn AccountsCommand(&self) -> ::windows::core::Result<SettingsCommand>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISettingsCommandStatics {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.ISettingsCommandStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISettingsCommandStaticsVtbl {
    pub const fn new<Impl: ISettingsCommandStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISettingsCommandStaticsVtbl {
        unsafe extern "system" fn AccountsCommand<Impl: ISettingsCommandStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AccountsCommand() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISettingsCommandStatics>, base.5, AccountsCommand::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISettingsPaneImpl: Sized {
    fn CommandsRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SettingsPane, SettingsPaneCommandsRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCommandsRequested(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISettingsPane {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.ISettingsPane";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ISettingsPaneVtbl {
    pub const fn new<Impl: ISettingsPaneImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISettingsPaneVtbl {
        unsafe extern "system" fn CommandsRequested<Impl: ISettingsPaneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CommandsRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<SettingsPane, SettingsPaneCommandsRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<SettingsPane, SettingsPaneCommandsRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCommandsRequested<Impl: ISettingsPaneImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveCommandsRequested(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISettingsPane>, base.5, CommandsRequested::<Impl, OFFSET>, RemoveCommandsRequested::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISettingsPaneCommandsRequestImpl: Sized {
    fn ApplicationCommands(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SettingsCommand>>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISettingsPaneCommandsRequest {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.ISettingsPaneCommandsRequest";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ISettingsPaneCommandsRequestVtbl {
    pub const fn new<Impl: ISettingsPaneCommandsRequestImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISettingsPaneCommandsRequestVtbl {
        unsafe extern "system" fn ApplicationCommands<Impl: ISettingsPaneCommandsRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ApplicationCommands() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISettingsPaneCommandsRequest>, base.5, ApplicationCommands::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISettingsPaneCommandsRequestedEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<SettingsPaneCommandsRequest>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISettingsPaneCommandsRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.ISettingsPaneCommandsRequestedEventArgs";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ISettingsPaneCommandsRequestedEventArgsVtbl {
    pub const fn new<Impl: ISettingsPaneCommandsRequestedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISettingsPaneCommandsRequestedEventArgsVtbl {
        unsafe extern "system" fn Request<Impl: ISettingsPaneCommandsRequestedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISettingsPaneCommandsRequestedEventArgs>, base.5, Request::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISettingsPaneStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<SettingsPane>;
    fn Show(&self) -> ::windows::core::Result<()>;
    fn Edge(&self) -> ::windows::core::Result<SettingsEdgeLocation>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISettingsPaneStatics {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.ISettingsPaneStatics";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ISettingsPaneStaticsVtbl {
    pub const fn new<Impl: ISettingsPaneStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISettingsPaneStaticsVtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: ISettingsPaneStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForCurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Show<Impl: ISettingsPaneStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Show().into()
        }
        unsafe extern "system" fn Edge<Impl: ISettingsPaneStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SettingsEdgeLocation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Edge() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISettingsPaneStatics>, base.5, GetForCurrentView::<Impl, OFFSET>, Show::<Impl, OFFSET>, Edge::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountCommandImpl: Sized {
    fn WebAccount(&self) -> ::windows::core::Result<super::super::Security::Credentials::WebAccount>;
    fn Invoked(&self) -> ::windows::core::Result<WebAccountCommandInvokedHandler>;
    fn Actions(&self) -> ::windows::core::Result<SupportedWebAccountActions>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebAccountCommand {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.IWebAccountCommand";
}
#[cfg(feature = "implement_exclusive")]
impl IWebAccountCommandVtbl {
    pub const fn new<Impl: IWebAccountCommandImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebAccountCommandVtbl {
        unsafe extern "system" fn WebAccount<Impl: IWebAccountCommandImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WebAccount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Invoked<Impl: IWebAccountCommandImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Invoked() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Actions<Impl: IWebAccountCommandImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SupportedWebAccountActions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Actions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebAccountCommand>, base.5, WebAccount::<Impl, OFFSET>, Invoked::<Impl, OFFSET>, Actions::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountCommandFactoryImpl: Sized {
    fn CreateWebAccountCommand(&self, webaccount: &::core::option::Option<super::super::Security::Credentials::WebAccount>, invoked: &::core::option::Option<WebAccountCommandInvokedHandler>, actions: SupportedWebAccountActions) -> ::windows::core::Result<WebAccountCommand>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebAccountCommandFactory {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.IWebAccountCommandFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IWebAccountCommandFactoryVtbl {
    pub const fn new<Impl: IWebAccountCommandFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebAccountCommandFactoryVtbl {
        unsafe extern "system" fn CreateWebAccountCommand<Impl: IWebAccountCommandFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, webaccount: ::windows::core::RawPtr, invoked: ::windows::core::RawPtr, actions: SupportedWebAccountActions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateWebAccountCommand(&*(&webaccount as *const <super::super::Security::Credentials::WebAccount as ::windows::core::Abi>::Abi as *const <super::super::Security::Credentials::WebAccount as ::windows::core::DefaultType>::DefaultType), &*(&invoked as *const <WebAccountCommandInvokedHandler as ::windows::core::Abi>::Abi as *const <WebAccountCommandInvokedHandler as ::windows::core::DefaultType>::DefaultType), actions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebAccountCommandFactory>, base.5, CreateWebAccountCommand::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountInvokedArgsImpl: Sized {
    fn Action(&self) -> ::windows::core::Result<WebAccountAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebAccountInvokedArgs {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.IWebAccountInvokedArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IWebAccountInvokedArgsVtbl {
    pub const fn new<Impl: IWebAccountInvokedArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebAccountInvokedArgsVtbl {
        unsafe extern "system" fn Action<Impl: IWebAccountInvokedArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut WebAccountAction) -> ::windows::core::HRESULT {
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebAccountInvokedArgs>, base.5, Action::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountProviderCommandImpl: Sized {
    fn WebAccountProvider(&self) -> ::windows::core::Result<super::super::Security::Credentials::WebAccountProvider>;
    fn Invoked(&self) -> ::windows::core::Result<WebAccountProviderCommandInvokedHandler>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebAccountProviderCommand {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.IWebAccountProviderCommand";
}
#[cfg(feature = "implement_exclusive")]
impl IWebAccountProviderCommandVtbl {
    pub const fn new<Impl: IWebAccountProviderCommandImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebAccountProviderCommandVtbl {
        unsafe extern "system" fn WebAccountProvider<Impl: IWebAccountProviderCommandImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WebAccountProvider() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Invoked<Impl: IWebAccountProviderCommandImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Invoked() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebAccountProviderCommand>, base.5, WebAccountProvider::<Impl, OFFSET>, Invoked::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountProviderCommandFactoryImpl: Sized {
    fn CreateWebAccountProviderCommand(&self, webaccountprovider: &::core::option::Option<super::super::Security::Credentials::WebAccountProvider>, invoked: &::core::option::Option<WebAccountProviderCommandInvokedHandler>) -> ::windows::core::Result<WebAccountProviderCommand>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebAccountProviderCommandFactory {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.IWebAccountProviderCommandFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IWebAccountProviderCommandFactoryVtbl {
    pub const fn new<Impl: IWebAccountProviderCommandFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebAccountProviderCommandFactoryVtbl {
        unsafe extern "system" fn CreateWebAccountProviderCommand<Impl: IWebAccountProviderCommandFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, webaccountprovider: ::windows::core::RawPtr, invoked: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateWebAccountProviderCommand(&*(&webaccountprovider as *const <super::super::Security::Credentials::WebAccountProvider as ::windows::core::Abi>::Abi as *const <super::super::Security::Credentials::WebAccountProvider as ::windows::core::DefaultType>::DefaultType), &*(&invoked as *const <WebAccountProviderCommandInvokedHandler as ::windows::core::Abi>::Abi as *const <WebAccountProviderCommandInvokedHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebAccountProviderCommandFactory>, base.5, CreateWebAccountProviderCommand::<Impl, OFFSET>)
    }
}

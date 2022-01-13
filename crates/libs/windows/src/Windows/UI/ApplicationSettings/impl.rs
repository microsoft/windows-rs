#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAccountsSettingsPaneImpl: Sized {
    fn AccountCommandsRequested(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AccountsSettingsPane, AccountsSettingsPaneCommandsRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAccountCommandsRequested(&mut self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAccountsSettingsPane {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.IAccountsSettingsPane";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAccountsSettingsPaneVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccountsSettingsPaneImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccountsSettingsPaneVtbl {
        unsafe extern "system" fn AccountCommandsRequested<Impl: IAccountsSettingsPaneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccountCommandsRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AccountsSettingsPane, AccountsSettingsPaneCommandsRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AccountsSettingsPane, AccountsSettingsPaneCommandsRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAccountCommandsRequested<Impl: IAccountsSettingsPaneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAccountCommandsRequested(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAccountsSettingsPane, BASE_OFFSET>(),
            AccountCommandsRequested: AccountCommandsRequested::<Impl, IMPL_OFFSET>,
            RemoveAccountCommandsRequested: RemoveAccountCommandsRequested::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccountsSettingsPane as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAccountsSettingsPaneCommandsRequestedEventArgsImpl: Sized {
    fn WebAccountProviderCommands(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<WebAccountProviderCommand>>;
    fn WebAccountCommands(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<WebAccountCommand>>;
    fn CredentialCommands(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<CredentialCommand>>;
    fn Commands(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SettingsCommand>>;
    fn HeaderText(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetHeaderText(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<AccountsSettingsPaneEventDeferral>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAccountsSettingsPaneCommandsRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.IAccountsSettingsPaneCommandsRequestedEventArgs";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAccountsSettingsPaneCommandsRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccountsSettingsPaneCommandsRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccountsSettingsPaneCommandsRequestedEventArgsVtbl {
        unsafe extern "system" fn WebAccountProviderCommands<Impl: IAccountsSettingsPaneCommandsRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WebAccountProviderCommands() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WebAccountCommands<Impl: IAccountsSettingsPaneCommandsRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WebAccountCommands() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CredentialCommands<Impl: IAccountsSettingsPaneCommandsRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CredentialCommands() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Commands<Impl: IAccountsSettingsPaneCommandsRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Commands() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HeaderText<Impl: IAccountsSettingsPaneCommandsRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetHeaderText<Impl: IAccountsSettingsPaneCommandsRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHeaderText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IAccountsSettingsPaneCommandsRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAccountsSettingsPaneCommandsRequestedEventArgs, BASE_OFFSET>(),
            WebAccountProviderCommands: WebAccountProviderCommands::<Impl, IMPL_OFFSET>,
            WebAccountCommands: WebAccountCommands::<Impl, IMPL_OFFSET>,
            CredentialCommands: CredentialCommands::<Impl, IMPL_OFFSET>,
            Commands: Commands::<Impl, IMPL_OFFSET>,
            HeaderText: HeaderText::<Impl, IMPL_OFFSET>,
            SetHeaderText: SetHeaderText::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccountsSettingsPaneCommandsRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait IAccountsSettingsPaneCommandsRequestedEventArgs2Impl: Sized {
    fn User(&mut self) -> ::windows::core::Result<super::super::System::User>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAccountsSettingsPaneCommandsRequestedEventArgs2 {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.IAccountsSettingsPaneCommandsRequestedEventArgs2";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl IAccountsSettingsPaneCommandsRequestedEventArgs2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccountsSettingsPaneCommandsRequestedEventArgs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccountsSettingsPaneCommandsRequestedEventArgs2Vtbl {
        unsafe extern "system" fn User<Impl: IAccountsSettingsPaneCommandsRequestedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).User() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAccountsSettingsPaneCommandsRequestedEventArgs2, BASE_OFFSET>(),
            User: User::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccountsSettingsPaneCommandsRequestedEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccountsSettingsPaneEventDeferralImpl: Sized {
    fn Complete(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAccountsSettingsPaneEventDeferral {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.IAccountsSettingsPaneEventDeferral";
}
#[cfg(feature = "implement_exclusive")]
impl IAccountsSettingsPaneEventDeferralVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccountsSettingsPaneEventDeferralImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccountsSettingsPaneEventDeferralVtbl {
        unsafe extern "system" fn Complete<Impl: IAccountsSettingsPaneEventDeferralImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAccountsSettingsPaneEventDeferral, BASE_OFFSET>(),
            Complete: Complete::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccountsSettingsPaneEventDeferral as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAccountsSettingsPaneStaticsImpl: Sized {
    fn GetForCurrentView(&mut self) -> ::windows::core::Result<AccountsSettingsPane>;
    fn Show(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAccountsSettingsPaneStatics {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.IAccountsSettingsPaneStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IAccountsSettingsPaneStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccountsSettingsPaneStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccountsSettingsPaneStaticsVtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: IAccountsSettingsPaneStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForCurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Show<Impl: IAccountsSettingsPaneStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Show().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAccountsSettingsPaneStatics, BASE_OFFSET>(),
            GetForCurrentView: GetForCurrentView::<Impl, IMPL_OFFSET>,
            Show: Show::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccountsSettingsPaneStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAccountsSettingsPaneStatics2Impl: Sized + IAccountsSettingsPaneStaticsImpl {
    fn ShowManageAccountsAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ShowAddAccountAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAccountsSettingsPaneStatics2 {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.IAccountsSettingsPaneStatics2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAccountsSettingsPaneStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccountsSettingsPaneStatics2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccountsSettingsPaneStatics2Vtbl {
        unsafe extern "system" fn ShowManageAccountsAsync<Impl: IAccountsSettingsPaneStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowManageAccountsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowAddAccountAsync<Impl: IAccountsSettingsPaneStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowAddAccountAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAccountsSettingsPaneStatics2, BASE_OFFSET>(),
            ShowManageAccountsAsync: ShowManageAccountsAsync::<Impl, IMPL_OFFSET>,
            ShowAddAccountAsync: ShowAddAccountAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccountsSettingsPaneStatics2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
pub trait IAccountsSettingsPaneStatics3Impl: Sized {
    fn ShowManageAccountsForUserAsync(&mut self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ShowAddAccountForUserAsync(&mut self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAccountsSettingsPaneStatics3 {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.IAccountsSettingsPaneStatics3";
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl IAccountsSettingsPaneStatics3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccountsSettingsPaneStatics3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccountsSettingsPaneStatics3Vtbl {
        unsafe extern "system" fn ShowManageAccountsForUserAsync<Impl: IAccountsSettingsPaneStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowManageAccountsForUserAsync(&*(&user as *const <super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::System::User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowAddAccountForUserAsync<Impl: IAccountsSettingsPaneStatics3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowAddAccountForUserAsync(&*(&user as *const <super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::System::User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAccountsSettingsPaneStatics3, BASE_OFFSET>(),
            ShowManageAccountsForUserAsync: ShowManageAccountsForUserAsync::<Impl, IMPL_OFFSET>,
            ShowAddAccountForUserAsync: ShowAddAccountForUserAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccountsSettingsPaneStatics3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
pub trait ICredentialCommandImpl: Sized {
    fn PasswordCredential(&mut self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential>;
    fn CredentialDeleted(&mut self) -> ::windows::core::Result<CredentialCommandCredentialDeletedHandler>;
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICredentialCommand {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.ICredentialCommand";
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
impl ICredentialCommandVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICredentialCommandImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICredentialCommandVtbl {
        unsafe extern "system" fn PasswordCredential<Impl: ICredentialCommandImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PasswordCredential() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CredentialDeleted<Impl: ICredentialCommandImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CredentialDeleted() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICredentialCommand, BASE_OFFSET>(),
            PasswordCredential: PasswordCredential::<Impl, IMPL_OFFSET>,
            CredentialDeleted: CredentialDeleted::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICredentialCommand as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
pub trait ICredentialCommandFactoryImpl: Sized {
    fn CreateCredentialCommand(&mut self, passwordcredential: &::core::option::Option<super::super::Security::Credentials::PasswordCredential>) -> ::windows::core::Result<CredentialCommand>;
    fn CreateCredentialCommandWithHandler(&mut self, passwordcredential: &::core::option::Option<super::super::Security::Credentials::PasswordCredential>, deleted: &::core::option::Option<CredentialCommandCredentialDeletedHandler>) -> ::windows::core::Result<CredentialCommand>;
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ICredentialCommandFactory {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.ICredentialCommandFactory";
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
impl ICredentialCommandFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICredentialCommandFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICredentialCommandFactoryVtbl {
        unsafe extern "system" fn CreateCredentialCommand<Impl: ICredentialCommandFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, passwordcredential: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCredentialCommand(&*(&passwordcredential as *const <super::super::Security::Credentials::PasswordCredential as ::windows::core::Abi>::Abi as *const <super::super::Security::Credentials::PasswordCredential as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCredentialCommandWithHandler<Impl: ICredentialCommandFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, passwordcredential: ::windows::core::RawPtr, deleted: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateCredentialCommandWithHandler(&*(&passwordcredential as *const <super::super::Security::Credentials::PasswordCredential as ::windows::core::Abi>::Abi as *const <super::super::Security::Credentials::PasswordCredential as ::windows::core::DefaultType>::DefaultType), &*(&deleted as *const <CredentialCommandCredentialDeletedHandler as ::windows::core::Abi>::Abi as *const <CredentialCommandCredentialDeletedHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICredentialCommandFactory, BASE_OFFSET>(),
            CreateCredentialCommand: CreateCredentialCommand::<Impl, IMPL_OFFSET>,
            CreateCredentialCommandWithHandler: CreateCredentialCommandWithHandler::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICredentialCommandFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Popups", feature = "implement_exclusive"))]
pub trait ISettingsCommandFactoryImpl: Sized {
    fn CreateSettingsCommand(&mut self, settingscommandid: &::core::option::Option<::windows::core::IInspectable>, label: &::windows::core::HSTRING, handler: &::core::option::Option<super::Popups::UICommandInvokedHandler>) -> ::windows::core::Result<SettingsCommand>;
}
#[cfg(all(feature = "UI_Popups", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISettingsCommandFactory {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.ISettingsCommandFactory";
}
#[cfg(all(feature = "UI_Popups", feature = "implement_exclusive"))]
impl ISettingsCommandFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISettingsCommandFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISettingsCommandFactoryVtbl {
        unsafe extern "system" fn CreateSettingsCommand<Impl: ISettingsCommandFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, settingscommandid: *mut ::core::ffi::c_void, label: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, handler: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISettingsCommandFactory, BASE_OFFSET>(),
            CreateSettingsCommand: CreateSettingsCommand::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISettingsCommandFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "UI_Popups", feature = "implement_exclusive"))]
pub trait ISettingsCommandStaticsImpl: Sized {
    fn AccountsCommand(&mut self) -> ::windows::core::Result<SettingsCommand>;
}
#[cfg(all(feature = "UI_Popups", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISettingsCommandStatics {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.ISettingsCommandStatics";
}
#[cfg(all(feature = "UI_Popups", feature = "implement_exclusive"))]
impl ISettingsCommandStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISettingsCommandStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISettingsCommandStaticsVtbl {
        unsafe extern "system" fn AccountsCommand<Impl: ISettingsCommandStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccountsCommand() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISettingsCommandStatics, BASE_OFFSET>(),
            AccountsCommand: AccountsCommand::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISettingsCommandStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISettingsPaneImpl: Sized {
    fn CommandsRequested(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<SettingsPane, SettingsPaneCommandsRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCommandsRequested(&mut self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISettingsPane {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.ISettingsPane";
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
impl ISettingsPaneVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISettingsPaneImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISettingsPaneVtbl {
        unsafe extern "system" fn CommandsRequested<Impl: ISettingsPaneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommandsRequested(&*(&handler as *const <super::super::Foundation::TypedEventHandler<SettingsPane, SettingsPaneCommandsRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<SettingsPane, SettingsPaneCommandsRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCommandsRequested<Impl: ISettingsPaneImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCommandsRequested(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISettingsPane, BASE_OFFSET>(),
            CommandsRequested: CommandsRequested::<Impl, IMPL_OFFSET>,
            RemoveCommandsRequested: RemoveCommandsRequested::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISettingsPane as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Popups", feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISettingsPaneCommandsRequestImpl: Sized {
    fn ApplicationCommands(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SettingsCommand>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Popups", feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISettingsPaneCommandsRequest {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.ISettingsPaneCommandsRequest";
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Popups", feature = "deprecated", feature = "implement_exclusive"))]
impl ISettingsPaneCommandsRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISettingsPaneCommandsRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISettingsPaneCommandsRequestVtbl {
        unsafe extern "system" fn ApplicationCommands<Impl: ISettingsPaneCommandsRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplicationCommands() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISettingsPaneCommandsRequest, BASE_OFFSET>(),
            ApplicationCommands: ApplicationCommands::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISettingsPaneCommandsRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISettingsPaneCommandsRequestedEventArgsImpl: Sized {
    fn Request(&mut self) -> ::windows::core::Result<SettingsPaneCommandsRequest>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISettingsPaneCommandsRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.ISettingsPaneCommandsRequestedEventArgs";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ISettingsPaneCommandsRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISettingsPaneCommandsRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISettingsPaneCommandsRequestedEventArgsVtbl {
        unsafe extern "system" fn Request<Impl: ISettingsPaneCommandsRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISettingsPaneCommandsRequestedEventArgs, BASE_OFFSET>(),
            Request: Request::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISettingsPaneCommandsRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISettingsPaneStaticsImpl: Sized {
    fn GetForCurrentView(&mut self) -> ::windows::core::Result<SettingsPane>;
    fn Show(&mut self) -> ::windows::core::Result<()>;
    fn Edge(&mut self) -> ::windows::core::Result<SettingsEdgeLocation>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISettingsPaneStatics {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.ISettingsPaneStatics";
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
impl ISettingsPaneStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISettingsPaneStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISettingsPaneStaticsVtbl {
        unsafe extern "system" fn GetForCurrentView<Impl: ISettingsPaneStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForCurrentView() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Show<Impl: ISettingsPaneStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Show().into()
        }
        unsafe extern "system" fn Edge<Impl: ISettingsPaneStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SettingsEdgeLocation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Edge() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISettingsPaneStatics, BASE_OFFSET>(),
            GetForCurrentView: GetForCurrentView::<Impl, IMPL_OFFSET>,
            Show: Show::<Impl, IMPL_OFFSET>,
            Edge: Edge::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISettingsPaneStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
pub trait IWebAccountCommandImpl: Sized {
    fn WebAccount(&mut self) -> ::windows::core::Result<super::super::Security::Credentials::WebAccount>;
    fn Invoked(&mut self) -> ::windows::core::Result<WebAccountCommandInvokedHandler>;
    fn Actions(&mut self) -> ::windows::core::Result<SupportedWebAccountActions>;
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebAccountCommand {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.IWebAccountCommand";
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
impl IWebAccountCommandVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountCommandImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountCommandVtbl {
        unsafe extern "system" fn WebAccount<Impl: IWebAccountCommandImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WebAccount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Invoked<Impl: IWebAccountCommandImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Invoked() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Actions<Impl: IWebAccountCommandImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SupportedWebAccountActions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Actions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebAccountCommand, BASE_OFFSET>(),
            WebAccount: WebAccount::<Impl, IMPL_OFFSET>,
            Invoked: Invoked::<Impl, IMPL_OFFSET>,
            Actions: Actions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAccountCommand as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
pub trait IWebAccountCommandFactoryImpl: Sized {
    fn CreateWebAccountCommand(&mut self, webaccount: &::core::option::Option<super::super::Security::Credentials::WebAccount>, invoked: &::core::option::Option<WebAccountCommandInvokedHandler>, actions: SupportedWebAccountActions) -> ::windows::core::Result<WebAccountCommand>;
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebAccountCommandFactory {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.IWebAccountCommandFactory";
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
impl IWebAccountCommandFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountCommandFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountCommandFactoryVtbl {
        unsafe extern "system" fn CreateWebAccountCommand<Impl: IWebAccountCommandFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webaccount: ::windows::core::RawPtr, invoked: ::windows::core::RawPtr, actions: SupportedWebAccountActions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWebAccountCommand(&*(&webaccount as *const <super::super::Security::Credentials::WebAccount as ::windows::core::Abi>::Abi as *const <super::super::Security::Credentials::WebAccount as ::windows::core::DefaultType>::DefaultType), &*(&invoked as *const <WebAccountCommandInvokedHandler as ::windows::core::Abi>::Abi as *const <WebAccountCommandInvokedHandler as ::windows::core::DefaultType>::DefaultType), actions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebAccountCommandFactory, BASE_OFFSET>(),
            CreateWebAccountCommand: CreateWebAccountCommand::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAccountCommandFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebAccountInvokedArgsImpl: Sized {
    fn Action(&mut self) -> ::windows::core::Result<WebAccountAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebAccountInvokedArgs {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.IWebAccountInvokedArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IWebAccountInvokedArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountInvokedArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountInvokedArgsVtbl {
        unsafe extern "system" fn Action<Impl: IWebAccountInvokedArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut WebAccountAction) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IWebAccountInvokedArgs, BASE_OFFSET>(), Action: Action::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAccountInvokedArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
pub trait IWebAccountProviderCommandImpl: Sized {
    fn WebAccountProvider(&mut self) -> ::windows::core::Result<super::super::Security::Credentials::WebAccountProvider>;
    fn Invoked(&mut self) -> ::windows::core::Result<WebAccountProviderCommandInvokedHandler>;
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebAccountProviderCommand {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.IWebAccountProviderCommand";
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
impl IWebAccountProviderCommandVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountProviderCommandImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountProviderCommandVtbl {
        unsafe extern "system" fn WebAccountProvider<Impl: IWebAccountProviderCommandImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WebAccountProvider() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Invoked<Impl: IWebAccountProviderCommandImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Invoked() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebAccountProviderCommand, BASE_OFFSET>(),
            WebAccountProvider: WebAccountProvider::<Impl, IMPL_OFFSET>,
            Invoked: Invoked::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAccountProviderCommand as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
pub trait IWebAccountProviderCommandFactoryImpl: Sized {
    fn CreateWebAccountProviderCommand(&mut self, webaccountprovider: &::core::option::Option<super::super::Security::Credentials::WebAccountProvider>, invoked: &::core::option::Option<WebAccountProviderCommandInvokedHandler>) -> ::windows::core::Result<WebAccountProviderCommand>;
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebAccountProviderCommandFactory {
    const NAME: &'static str = "Windows.UI.ApplicationSettings.IWebAccountProviderCommandFactory";
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
impl IWebAccountProviderCommandFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAccountProviderCommandFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAccountProviderCommandFactoryVtbl {
        unsafe extern "system" fn CreateWebAccountProviderCommand<Impl: IWebAccountProviderCommandFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, webaccountprovider: ::windows::core::RawPtr, invoked: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWebAccountProviderCommand(&*(&webaccountprovider as *const <super::super::Security::Credentials::WebAccountProvider as ::windows::core::Abi>::Abi as *const <super::super::Security::Credentials::WebAccountProvider as ::windows::core::DefaultType>::DefaultType), &*(&invoked as *const <WebAccountProviderCommandInvokedHandler as ::windows::core::Abi>::Abi as *const <WebAccountProviderCommandInvokedHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebAccountProviderCommandFactory, BASE_OFFSET>(),
            CreateWebAccountProviderCommand: CreateWebAccountProviderCommand::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAccountProviderCommandFactory as ::windows::core::Interface>::IID
    }
}

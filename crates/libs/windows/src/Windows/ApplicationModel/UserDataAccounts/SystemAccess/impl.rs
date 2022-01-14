#[cfg(feature = "implement_exclusive")]
pub trait IDeviceAccountConfiguration_Impl: Sized {
    fn AccountName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAccountName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DeviceAccountTypeId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDeviceAccountTypeId(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ServerType(&mut self) -> ::windows::core::Result<DeviceAccountServerType>;
    fn SetServerType(&mut self, value: DeviceAccountServerType) -> ::windows::core::Result<()>;
    fn EmailAddress(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetEmailAddress(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Domain(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDomain(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn EmailSyncEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetEmailSyncEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn ContactsSyncEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetContactsSyncEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn CalendarSyncEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetCalendarSyncEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IncomingServerAddress(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetIncomingServerAddress(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IncomingServerPort(&mut self) -> ::windows::core::Result<i32>;
    fn SetIncomingServerPort(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn IncomingServerRequiresSsl(&mut self) -> ::windows::core::Result<bool>;
    fn SetIncomingServerRequiresSsl(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IncomingServerUsername(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetIncomingServerUsername(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn OutgoingServerAddress(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetOutgoingServerAddress(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn OutgoingServerPort(&mut self) -> ::windows::core::Result<i32>;
    fn SetOutgoingServerPort(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn OutgoingServerRequiresSsl(&mut self) -> ::windows::core::Result<bool>;
    fn SetOutgoingServerRequiresSsl(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn OutgoingServerUsername(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetOutgoingServerUsername(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeviceAccountConfiguration {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.SystemAccess.IDeviceAccountConfiguration";
}
#[cfg(feature = "implement_exclusive")]
impl IDeviceAccountConfiguration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceAccountConfiguration_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDeviceAccountConfiguration_Vtbl {
        unsafe extern "system" fn AccountName<Impl: IDeviceAccountConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccountName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccountName<Impl: IDeviceAccountConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAccountName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DeviceAccountTypeId<Impl: IDeviceAccountConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceAccountTypeId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDeviceAccountTypeId<Impl: IDeviceAccountConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDeviceAccountTypeId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ServerType<Impl: IDeviceAccountConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DeviceAccountServerType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServerType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServerType<Impl: IDeviceAccountConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: DeviceAccountServerType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetServerType(value).into()
        }
        unsafe extern "system" fn EmailAddress<Impl: IDeviceAccountConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EmailAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEmailAddress<Impl: IDeviceAccountConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEmailAddress(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Domain<Impl: IDeviceAccountConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Domain() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDomain<Impl: IDeviceAccountConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDomain(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EmailSyncEnabled<Impl: IDeviceAccountConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EmailSyncEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEmailSyncEnabled<Impl: IDeviceAccountConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEmailSyncEnabled(value).into()
        }
        unsafe extern "system" fn ContactsSyncEnabled<Impl: IDeviceAccountConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContactsSyncEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContactsSyncEnabled<Impl: IDeviceAccountConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContactsSyncEnabled(value).into()
        }
        unsafe extern "system" fn CalendarSyncEnabled<Impl: IDeviceAccountConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CalendarSyncEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCalendarSyncEnabled<Impl: IDeviceAccountConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCalendarSyncEnabled(value).into()
        }
        unsafe extern "system" fn IncomingServerAddress<Impl: IDeviceAccountConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IncomingServerAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIncomingServerAddress<Impl: IDeviceAccountConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIncomingServerAddress(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IncomingServerPort<Impl: IDeviceAccountConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IncomingServerPort() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIncomingServerPort<Impl: IDeviceAccountConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIncomingServerPort(value).into()
        }
        unsafe extern "system" fn IncomingServerRequiresSsl<Impl: IDeviceAccountConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IncomingServerRequiresSsl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIncomingServerRequiresSsl<Impl: IDeviceAccountConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIncomingServerRequiresSsl(value).into()
        }
        unsafe extern "system" fn IncomingServerUsername<Impl: IDeviceAccountConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IncomingServerUsername() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIncomingServerUsername<Impl: IDeviceAccountConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIncomingServerUsername(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OutgoingServerAddress<Impl: IDeviceAccountConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutgoingServerAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutgoingServerAddress<Impl: IDeviceAccountConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutgoingServerAddress(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OutgoingServerPort<Impl: IDeviceAccountConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutgoingServerPort() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutgoingServerPort<Impl: IDeviceAccountConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutgoingServerPort(value).into()
        }
        unsafe extern "system" fn OutgoingServerRequiresSsl<Impl: IDeviceAccountConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutgoingServerRequiresSsl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutgoingServerRequiresSsl<Impl: IDeviceAccountConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutgoingServerRequiresSsl(value).into()
        }
        unsafe extern "system" fn OutgoingServerUsername<Impl: IDeviceAccountConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutgoingServerUsername() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutgoingServerUsername<Impl: IDeviceAccountConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutgoingServerUsername(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDeviceAccountConfiguration, BASE_OFFSET>(),
            AccountName: AccountName::<Impl, IMPL_OFFSET>,
            SetAccountName: SetAccountName::<Impl, IMPL_OFFSET>,
            DeviceAccountTypeId: DeviceAccountTypeId::<Impl, IMPL_OFFSET>,
            SetDeviceAccountTypeId: SetDeviceAccountTypeId::<Impl, IMPL_OFFSET>,
            ServerType: ServerType::<Impl, IMPL_OFFSET>,
            SetServerType: SetServerType::<Impl, IMPL_OFFSET>,
            EmailAddress: EmailAddress::<Impl, IMPL_OFFSET>,
            SetEmailAddress: SetEmailAddress::<Impl, IMPL_OFFSET>,
            Domain: Domain::<Impl, IMPL_OFFSET>,
            SetDomain: SetDomain::<Impl, IMPL_OFFSET>,
            EmailSyncEnabled: EmailSyncEnabled::<Impl, IMPL_OFFSET>,
            SetEmailSyncEnabled: SetEmailSyncEnabled::<Impl, IMPL_OFFSET>,
            ContactsSyncEnabled: ContactsSyncEnabled::<Impl, IMPL_OFFSET>,
            SetContactsSyncEnabled: SetContactsSyncEnabled::<Impl, IMPL_OFFSET>,
            CalendarSyncEnabled: CalendarSyncEnabled::<Impl, IMPL_OFFSET>,
            SetCalendarSyncEnabled: SetCalendarSyncEnabled::<Impl, IMPL_OFFSET>,
            IncomingServerAddress: IncomingServerAddress::<Impl, IMPL_OFFSET>,
            SetIncomingServerAddress: SetIncomingServerAddress::<Impl, IMPL_OFFSET>,
            IncomingServerPort: IncomingServerPort::<Impl, IMPL_OFFSET>,
            SetIncomingServerPort: SetIncomingServerPort::<Impl, IMPL_OFFSET>,
            IncomingServerRequiresSsl: IncomingServerRequiresSsl::<Impl, IMPL_OFFSET>,
            SetIncomingServerRequiresSsl: SetIncomingServerRequiresSsl::<Impl, IMPL_OFFSET>,
            IncomingServerUsername: IncomingServerUsername::<Impl, IMPL_OFFSET>,
            SetIncomingServerUsername: SetIncomingServerUsername::<Impl, IMPL_OFFSET>,
            OutgoingServerAddress: OutgoingServerAddress::<Impl, IMPL_OFFSET>,
            SetOutgoingServerAddress: SetOutgoingServerAddress::<Impl, IMPL_OFFSET>,
            OutgoingServerPort: OutgoingServerPort::<Impl, IMPL_OFFSET>,
            SetOutgoingServerPort: SetOutgoingServerPort::<Impl, IMPL_OFFSET>,
            OutgoingServerRequiresSsl: OutgoingServerRequiresSsl::<Impl, IMPL_OFFSET>,
            SetOutgoingServerRequiresSsl: SetOutgoingServerRequiresSsl::<Impl, IMPL_OFFSET>,
            OutgoingServerUsername: OutgoingServerUsername::<Impl, IMPL_OFFSET>,
            SetOutgoingServerUsername: SetOutgoingServerUsername::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDeviceAccountConfiguration as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "implement_exclusive"))]
pub trait IDeviceAccountConfiguration2_Impl: Sized {
    fn IncomingServerCredential(&mut self) -> ::windows::core::Result<super::super::super::Security::Credentials::PasswordCredential>;
    fn SetIncomingServerCredential(&mut self, value: &::core::option::Option<super::super::super::Security::Credentials::PasswordCredential>) -> ::windows::core::Result<()>;
    fn OutgoingServerCredential(&mut self) -> ::windows::core::Result<super::super::super::Security::Credentials::PasswordCredential>;
    fn SetOutgoingServerCredential(&mut self, value: &::core::option::Option<super::super::super::Security::Credentials::PasswordCredential>) -> ::windows::core::Result<()>;
    fn OAuthRefreshToken(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetOAuthRefreshToken(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsExternallyManaged(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsExternallyManaged(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn AccountIconId(&mut self) -> ::windows::core::Result<DeviceAccountIconId>;
    fn SetAccountIconId(&mut self, value: DeviceAccountIconId) -> ::windows::core::Result<()>;
    fn AuthenticationType(&mut self) -> ::windows::core::Result<DeviceAccountAuthenticationType>;
    fn SetAuthenticationType(&mut self, value: DeviceAccountAuthenticationType) -> ::windows::core::Result<()>;
    fn IsSsoAuthenticationSupported(&mut self) -> ::windows::core::Result<bool>;
    fn SsoAccountId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSsoAccountId(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AlwaysDownloadFullMessage(&mut self) -> ::windows::core::Result<bool>;
    fn SetAlwaysDownloadFullMessage(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn DoesPolicyAllowMailSync(&mut self) -> ::windows::core::Result<bool>;
    fn SyncScheduleKind(&mut self) -> ::windows::core::Result<DeviceAccountSyncScheduleKind>;
    fn SetSyncScheduleKind(&mut self, value: DeviceAccountSyncScheduleKind) -> ::windows::core::Result<()>;
    fn MailAgeFilter(&mut self) -> ::windows::core::Result<DeviceAccountMailAgeFilter>;
    fn SetMailAgeFilter(&mut self, value: DeviceAccountMailAgeFilter) -> ::windows::core::Result<()>;
    fn IsClientAuthenticationCertificateRequired(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsClientAuthenticationCertificateRequired(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn AutoSelectAuthenticationCertificate(&mut self) -> ::windows::core::Result<bool>;
    fn SetAutoSelectAuthenticationCertificate(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn AuthenticationCertificateId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAuthenticationCertificateId(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn CardDavSyncScheduleKind(&mut self) -> ::windows::core::Result<DeviceAccountSyncScheduleKind>;
    fn SetCardDavSyncScheduleKind(&mut self, value: DeviceAccountSyncScheduleKind) -> ::windows::core::Result<()>;
    fn CalDavSyncScheduleKind(&mut self) -> ::windows::core::Result<DeviceAccountSyncScheduleKind>;
    fn SetCalDavSyncScheduleKind(&mut self, value: DeviceAccountSyncScheduleKind) -> ::windows::core::Result<()>;
    fn CardDavServerUrl(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn SetCardDavServerUrl(&mut self, value: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn CardDavRequiresSsl(&mut self) -> ::windows::core::Result<bool>;
    fn SetCardDavRequiresSsl(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn CalDavServerUrl(&mut self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn SetCalDavServerUrl(&mut self, value: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn CalDavRequiresSsl(&mut self) -> ::windows::core::Result<bool>;
    fn SetCalDavRequiresSsl(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn WasModifiedByUser(&mut self) -> ::windows::core::Result<bool>;
    fn SetWasModifiedByUser(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn WasIncomingServerCertificateHashConfirmed(&mut self) -> ::windows::core::Result<bool>;
    fn SetWasIncomingServerCertificateHashConfirmed(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IncomingServerCertificateHash(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetIncomingServerCertificateHash(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsOutgoingServerAuthenticationRequired(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsOutgoingServerAuthenticationRequired(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsOutgoingServerAuthenticationEnabled(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsOutgoingServerAuthenticationEnabled(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn WasOutgoingServerCertificateHashConfirmed(&mut self) -> ::windows::core::Result<bool>;
    fn SetWasOutgoingServerCertificateHashConfirmed(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn OutgoingServerCertificateHash(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetOutgoingServerCertificateHash(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsSyncScheduleManagedBySystem(&mut self) -> ::windows::core::Result<bool>;
    fn SetIsSyncScheduleManagedBySystem(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDeviceAccountConfiguration2 {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.SystemAccess.IDeviceAccountConfiguration2";
}
#[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "implement_exclusive"))]
impl IDeviceAccountConfiguration2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceAccountConfiguration2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDeviceAccountConfiguration2_Vtbl {
        unsafe extern "system" fn IncomingServerCredential<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IncomingServerCredential() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIncomingServerCredential<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIncomingServerCredential(&*(&value as *const <super::super::super::Security::Credentials::PasswordCredential as ::windows::core::Abi>::Abi as *const <super::super::super::Security::Credentials::PasswordCredential as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OutgoingServerCredential<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutgoingServerCredential() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutgoingServerCredential<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutgoingServerCredential(&*(&value as *const <super::super::super::Security::Credentials::PasswordCredential as ::windows::core::Abi>::Abi as *const <super::super::super::Security::Credentials::PasswordCredential as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OAuthRefreshToken<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OAuthRefreshToken() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOAuthRefreshToken<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOAuthRefreshToken(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsExternallyManaged<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsExternallyManaged() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsExternallyManaged<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsExternallyManaged(value).into()
        }
        unsafe extern "system" fn AccountIconId<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DeviceAccountIconId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccountIconId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAccountIconId<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: DeviceAccountIconId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAccountIconId(value).into()
        }
        unsafe extern "system" fn AuthenticationType<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DeviceAccountAuthenticationType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticationType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationType<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: DeviceAccountAuthenticationType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAuthenticationType(value).into()
        }
        unsafe extern "system" fn IsSsoAuthenticationSupported<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSsoAuthenticationSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SsoAccountId<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SsoAccountId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSsoAccountId<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSsoAccountId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AlwaysDownloadFullMessage<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlwaysDownloadFullMessage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlwaysDownloadFullMessage<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlwaysDownloadFullMessage(value).into()
        }
        unsafe extern "system" fn DoesPolicyAllowMailSync<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DoesPolicyAllowMailSync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SyncScheduleKind<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DeviceAccountSyncScheduleKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SyncScheduleKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSyncScheduleKind<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: DeviceAccountSyncScheduleKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSyncScheduleKind(value).into()
        }
        unsafe extern "system" fn MailAgeFilter<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DeviceAccountMailAgeFilter) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MailAgeFilter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMailAgeFilter<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: DeviceAccountMailAgeFilter) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMailAgeFilter(value).into()
        }
        unsafe extern "system" fn IsClientAuthenticationCertificateRequired<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsClientAuthenticationCertificateRequired() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsClientAuthenticationCertificateRequired<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsClientAuthenticationCertificateRequired(value).into()
        }
        unsafe extern "system" fn AutoSelectAuthenticationCertificate<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoSelectAuthenticationCertificate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoSelectAuthenticationCertificate<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoSelectAuthenticationCertificate(value).into()
        }
        unsafe extern "system" fn AuthenticationCertificateId<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthenticationCertificateId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthenticationCertificateId<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAuthenticationCertificateId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CardDavSyncScheduleKind<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DeviceAccountSyncScheduleKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CardDavSyncScheduleKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCardDavSyncScheduleKind<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: DeviceAccountSyncScheduleKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCardDavSyncScheduleKind(value).into()
        }
        unsafe extern "system" fn CalDavSyncScheduleKind<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DeviceAccountSyncScheduleKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CalDavSyncScheduleKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCalDavSyncScheduleKind<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: DeviceAccountSyncScheduleKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCalDavSyncScheduleKind(value).into()
        }
        unsafe extern "system" fn CardDavServerUrl<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CardDavServerUrl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCardDavServerUrl<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCardDavServerUrl(&*(&value as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CardDavRequiresSsl<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CardDavRequiresSsl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCardDavRequiresSsl<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCardDavRequiresSsl(value).into()
        }
        unsafe extern "system" fn CalDavServerUrl<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CalDavServerUrl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCalDavServerUrl<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCalDavServerUrl(&*(&value as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CalDavRequiresSsl<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CalDavRequiresSsl() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCalDavRequiresSsl<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCalDavRequiresSsl(value).into()
        }
        unsafe extern "system" fn WasModifiedByUser<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WasModifiedByUser() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWasModifiedByUser<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWasModifiedByUser(value).into()
        }
        unsafe extern "system" fn WasIncomingServerCertificateHashConfirmed<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WasIncomingServerCertificateHashConfirmed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWasIncomingServerCertificateHashConfirmed<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWasIncomingServerCertificateHashConfirmed(value).into()
        }
        unsafe extern "system" fn IncomingServerCertificateHash<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IncomingServerCertificateHash() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIncomingServerCertificateHash<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIncomingServerCertificateHash(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsOutgoingServerAuthenticationRequired<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOutgoingServerAuthenticationRequired() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsOutgoingServerAuthenticationRequired<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsOutgoingServerAuthenticationRequired(value).into()
        }
        unsafe extern "system" fn IsOutgoingServerAuthenticationEnabled<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOutgoingServerAuthenticationEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsOutgoingServerAuthenticationEnabled<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsOutgoingServerAuthenticationEnabled(value).into()
        }
        unsafe extern "system" fn WasOutgoingServerCertificateHashConfirmed<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WasOutgoingServerCertificateHashConfirmed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWasOutgoingServerCertificateHashConfirmed<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWasOutgoingServerCertificateHashConfirmed(value).into()
        }
        unsafe extern "system" fn OutgoingServerCertificateHash<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutgoingServerCertificateHash() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutgoingServerCertificateHash<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutgoingServerCertificateHash(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsSyncScheduleManagedBySystem<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSyncScheduleManagedBySystem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsSyncScheduleManagedBySystem<Impl: IDeviceAccountConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsSyncScheduleManagedBySystem(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDeviceAccountConfiguration2, BASE_OFFSET>(),
            IncomingServerCredential: IncomingServerCredential::<Impl, IMPL_OFFSET>,
            SetIncomingServerCredential: SetIncomingServerCredential::<Impl, IMPL_OFFSET>,
            OutgoingServerCredential: OutgoingServerCredential::<Impl, IMPL_OFFSET>,
            SetOutgoingServerCredential: SetOutgoingServerCredential::<Impl, IMPL_OFFSET>,
            OAuthRefreshToken: OAuthRefreshToken::<Impl, IMPL_OFFSET>,
            SetOAuthRefreshToken: SetOAuthRefreshToken::<Impl, IMPL_OFFSET>,
            IsExternallyManaged: IsExternallyManaged::<Impl, IMPL_OFFSET>,
            SetIsExternallyManaged: SetIsExternallyManaged::<Impl, IMPL_OFFSET>,
            AccountIconId: AccountIconId::<Impl, IMPL_OFFSET>,
            SetAccountIconId: SetAccountIconId::<Impl, IMPL_OFFSET>,
            AuthenticationType: AuthenticationType::<Impl, IMPL_OFFSET>,
            SetAuthenticationType: SetAuthenticationType::<Impl, IMPL_OFFSET>,
            IsSsoAuthenticationSupported: IsSsoAuthenticationSupported::<Impl, IMPL_OFFSET>,
            SsoAccountId: SsoAccountId::<Impl, IMPL_OFFSET>,
            SetSsoAccountId: SetSsoAccountId::<Impl, IMPL_OFFSET>,
            AlwaysDownloadFullMessage: AlwaysDownloadFullMessage::<Impl, IMPL_OFFSET>,
            SetAlwaysDownloadFullMessage: SetAlwaysDownloadFullMessage::<Impl, IMPL_OFFSET>,
            DoesPolicyAllowMailSync: DoesPolicyAllowMailSync::<Impl, IMPL_OFFSET>,
            SyncScheduleKind: SyncScheduleKind::<Impl, IMPL_OFFSET>,
            SetSyncScheduleKind: SetSyncScheduleKind::<Impl, IMPL_OFFSET>,
            MailAgeFilter: MailAgeFilter::<Impl, IMPL_OFFSET>,
            SetMailAgeFilter: SetMailAgeFilter::<Impl, IMPL_OFFSET>,
            IsClientAuthenticationCertificateRequired: IsClientAuthenticationCertificateRequired::<Impl, IMPL_OFFSET>,
            SetIsClientAuthenticationCertificateRequired: SetIsClientAuthenticationCertificateRequired::<Impl, IMPL_OFFSET>,
            AutoSelectAuthenticationCertificate: AutoSelectAuthenticationCertificate::<Impl, IMPL_OFFSET>,
            SetAutoSelectAuthenticationCertificate: SetAutoSelectAuthenticationCertificate::<Impl, IMPL_OFFSET>,
            AuthenticationCertificateId: AuthenticationCertificateId::<Impl, IMPL_OFFSET>,
            SetAuthenticationCertificateId: SetAuthenticationCertificateId::<Impl, IMPL_OFFSET>,
            CardDavSyncScheduleKind: CardDavSyncScheduleKind::<Impl, IMPL_OFFSET>,
            SetCardDavSyncScheduleKind: SetCardDavSyncScheduleKind::<Impl, IMPL_OFFSET>,
            CalDavSyncScheduleKind: CalDavSyncScheduleKind::<Impl, IMPL_OFFSET>,
            SetCalDavSyncScheduleKind: SetCalDavSyncScheduleKind::<Impl, IMPL_OFFSET>,
            CardDavServerUrl: CardDavServerUrl::<Impl, IMPL_OFFSET>,
            SetCardDavServerUrl: SetCardDavServerUrl::<Impl, IMPL_OFFSET>,
            CardDavRequiresSsl: CardDavRequiresSsl::<Impl, IMPL_OFFSET>,
            SetCardDavRequiresSsl: SetCardDavRequiresSsl::<Impl, IMPL_OFFSET>,
            CalDavServerUrl: CalDavServerUrl::<Impl, IMPL_OFFSET>,
            SetCalDavServerUrl: SetCalDavServerUrl::<Impl, IMPL_OFFSET>,
            CalDavRequiresSsl: CalDavRequiresSsl::<Impl, IMPL_OFFSET>,
            SetCalDavRequiresSsl: SetCalDavRequiresSsl::<Impl, IMPL_OFFSET>,
            WasModifiedByUser: WasModifiedByUser::<Impl, IMPL_OFFSET>,
            SetWasModifiedByUser: SetWasModifiedByUser::<Impl, IMPL_OFFSET>,
            WasIncomingServerCertificateHashConfirmed: WasIncomingServerCertificateHashConfirmed::<Impl, IMPL_OFFSET>,
            SetWasIncomingServerCertificateHashConfirmed: SetWasIncomingServerCertificateHashConfirmed::<Impl, IMPL_OFFSET>,
            IncomingServerCertificateHash: IncomingServerCertificateHash::<Impl, IMPL_OFFSET>,
            SetIncomingServerCertificateHash: SetIncomingServerCertificateHash::<Impl, IMPL_OFFSET>,
            IsOutgoingServerAuthenticationRequired: IsOutgoingServerAuthenticationRequired::<Impl, IMPL_OFFSET>,
            SetIsOutgoingServerAuthenticationRequired: SetIsOutgoingServerAuthenticationRequired::<Impl, IMPL_OFFSET>,
            IsOutgoingServerAuthenticationEnabled: IsOutgoingServerAuthenticationEnabled::<Impl, IMPL_OFFSET>,
            SetIsOutgoingServerAuthenticationEnabled: SetIsOutgoingServerAuthenticationEnabled::<Impl, IMPL_OFFSET>,
            WasOutgoingServerCertificateHashConfirmed: WasOutgoingServerCertificateHashConfirmed::<Impl, IMPL_OFFSET>,
            SetWasOutgoingServerCertificateHashConfirmed: SetWasOutgoingServerCertificateHashConfirmed::<Impl, IMPL_OFFSET>,
            OutgoingServerCertificateHash: OutgoingServerCertificateHash::<Impl, IMPL_OFFSET>,
            SetOutgoingServerCertificateHash: SetOutgoingServerCertificateHash::<Impl, IMPL_OFFSET>,
            IsSyncScheduleManagedBySystem: IsSyncScheduleManagedBySystem::<Impl, IMPL_OFFSET>,
            SetIsSyncScheduleManagedBySystem: SetIsSyncScheduleManagedBySystem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDeviceAccountConfiguration2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IUserDataAccountSystemAccessManagerStatics_Impl: Sized {
    fn AddAndShowDeviceAccountsAsync(&mut self, accounts: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<DeviceAccountConfiguration>>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserDataAccountSystemAccessManagerStatics {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.SystemAccess.IUserDataAccountSystemAccessManagerStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IUserDataAccountSystemAccessManagerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserDataAccountSystemAccessManagerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserDataAccountSystemAccessManagerStatics_Vtbl {
        unsafe extern "system" fn AddAndShowDeviceAccountsAsync<Impl: IUserDataAccountSystemAccessManagerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accounts: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddAndShowDeviceAccountsAsync(&*(&accounts as *const <super::super::super::Foundation::Collections::IIterable<DeviceAccountConfiguration> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<DeviceAccountConfiguration> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserDataAccountSystemAccessManagerStatics, BASE_OFFSET>(),
            AddAndShowDeviceAccountsAsync: AddAndShowDeviceAccountsAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserDataAccountSystemAccessManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IUserDataAccountSystemAccessManagerStatics2_Impl: Sized {
    fn SuppressLocalAccountWithAccountAsync(&mut self, userdataaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn CreateDeviceAccountAsync(&mut self, account: &::core::option::Option<DeviceAccountConfiguration>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn DeleteDeviceAccountAsync(&mut self, accountid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn GetDeviceAccountConfigurationAsync(&mut self, accountid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<DeviceAccountConfiguration>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserDataAccountSystemAccessManagerStatics2 {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.SystemAccess.IUserDataAccountSystemAccessManagerStatics2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUserDataAccountSystemAccessManagerStatics2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserDataAccountSystemAccessManagerStatics2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserDataAccountSystemAccessManagerStatics2_Vtbl {
        unsafe extern "system" fn SuppressLocalAccountWithAccountAsync<Impl: IUserDataAccountSystemAccessManagerStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, userdataaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SuppressLocalAccountWithAccountAsync(&*(&userdataaccountid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDeviceAccountAsync<Impl: IUserDataAccountSystemAccessManagerStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, account: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDeviceAccountAsync(&*(&account as *const <DeviceAccountConfiguration as ::windows::core::Abi>::Abi as *const <DeviceAccountConfiguration as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteDeviceAccountAsync<Impl: IUserDataAccountSystemAccessManagerStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteDeviceAccountAsync(&*(&accountid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceAccountConfigurationAsync<Impl: IUserDataAccountSystemAccessManagerStatics2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceAccountConfigurationAsync(&*(&accountid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserDataAccountSystemAccessManagerStatics2, BASE_OFFSET>(),
            SuppressLocalAccountWithAccountAsync: SuppressLocalAccountWithAccountAsync::<Impl, IMPL_OFFSET>,
            CreateDeviceAccountAsync: CreateDeviceAccountAsync::<Impl, IMPL_OFFSET>,
            DeleteDeviceAccountAsync: DeleteDeviceAccountAsync::<Impl, IMPL_OFFSET>,
            GetDeviceAccountConfigurationAsync: GetDeviceAccountConfigurationAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserDataAccountSystemAccessManagerStatics2 as ::windows::core::Interface>::IID
    }
}

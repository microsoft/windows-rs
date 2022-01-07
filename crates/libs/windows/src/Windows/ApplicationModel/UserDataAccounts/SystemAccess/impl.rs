#[cfg(feature = "implement_exclusive")]
pub trait IDeviceAccountConfigurationImpl: Sized {
    fn AccountName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAccountName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DeviceAccountTypeId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDeviceAccountTypeId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ServerType(&self) -> ::windows::core::Result<DeviceAccountServerType>;
    fn SetServerType(&self, value: DeviceAccountServerType) -> ::windows::core::Result<()>;
    fn EmailAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetEmailAddress(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Domain(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDomain(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn EmailSyncEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetEmailSyncEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn ContactsSyncEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetContactsSyncEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn CalendarSyncEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetCalendarSyncEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IncomingServerAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetIncomingServerAddress(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IncomingServerPort(&self) -> ::windows::core::Result<i32>;
    fn SetIncomingServerPort(&self, value: i32) -> ::windows::core::Result<()>;
    fn IncomingServerRequiresSsl(&self) -> ::windows::core::Result<bool>;
    fn SetIncomingServerRequiresSsl(&self, value: bool) -> ::windows::core::Result<()>;
    fn IncomingServerUsername(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetIncomingServerUsername(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn OutgoingServerAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetOutgoingServerAddress(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn OutgoingServerPort(&self) -> ::windows::core::Result<i32>;
    fn SetOutgoingServerPort(&self, value: i32) -> ::windows::core::Result<()>;
    fn OutgoingServerRequiresSsl(&self) -> ::windows::core::Result<bool>;
    fn SetOutgoingServerRequiresSsl(&self, value: bool) -> ::windows::core::Result<()>;
    fn OutgoingServerUsername(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetOutgoingServerUsername(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeviceAccountConfiguration {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.SystemAccess.IDeviceAccountConfiguration";
}
#[cfg(feature = "implement_exclusive")]
impl IDeviceAccountConfigurationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceAccountConfigurationImpl, const OFFSET: isize>() -> IDeviceAccountConfigurationVtbl {
        unsafe extern "system" fn AccountName<Impl: IDeviceAccountConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAccountName<Impl: IDeviceAccountConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAccountName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DeviceAccountTypeId<Impl: IDeviceAccountConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDeviceAccountTypeId<Impl: IDeviceAccountConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDeviceAccountTypeId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ServerType<Impl: IDeviceAccountConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DeviceAccountServerType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetServerType<Impl: IDeviceAccountConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: DeviceAccountServerType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetServerType(value).into()
        }
        unsafe extern "system" fn EmailAddress<Impl: IDeviceAccountConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetEmailAddress<Impl: IDeviceAccountConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEmailAddress(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Domain<Impl: IDeviceAccountConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDomain<Impl: IDeviceAccountConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDomain(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EmailSyncEnabled<Impl: IDeviceAccountConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetEmailSyncEnabled<Impl: IDeviceAccountConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEmailSyncEnabled(value).into()
        }
        unsafe extern "system" fn ContactsSyncEnabled<Impl: IDeviceAccountConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetContactsSyncEnabled<Impl: IDeviceAccountConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContactsSyncEnabled(value).into()
        }
        unsafe extern "system" fn CalendarSyncEnabled<Impl: IDeviceAccountConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCalendarSyncEnabled<Impl: IDeviceAccountConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCalendarSyncEnabled(value).into()
        }
        unsafe extern "system" fn IncomingServerAddress<Impl: IDeviceAccountConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIncomingServerAddress<Impl: IDeviceAccountConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIncomingServerAddress(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IncomingServerPort<Impl: IDeviceAccountConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIncomingServerPort<Impl: IDeviceAccountConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIncomingServerPort(value).into()
        }
        unsafe extern "system" fn IncomingServerRequiresSsl<Impl: IDeviceAccountConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIncomingServerRequiresSsl<Impl: IDeviceAccountConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIncomingServerRequiresSsl(value).into()
        }
        unsafe extern "system" fn IncomingServerUsername<Impl: IDeviceAccountConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIncomingServerUsername<Impl: IDeviceAccountConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIncomingServerUsername(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OutgoingServerAddress<Impl: IDeviceAccountConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOutgoingServerAddress<Impl: IDeviceAccountConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutgoingServerAddress(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OutgoingServerPort<Impl: IDeviceAccountConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOutgoingServerPort<Impl: IDeviceAccountConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutgoingServerPort(value).into()
        }
        unsafe extern "system" fn OutgoingServerRequiresSsl<Impl: IDeviceAccountConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOutgoingServerRequiresSsl<Impl: IDeviceAccountConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutgoingServerRequiresSsl(value).into()
        }
        unsafe extern "system" fn OutgoingServerUsername<Impl: IDeviceAccountConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOutgoingServerUsername<Impl: IDeviceAccountConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutgoingServerUsername(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDeviceAccountConfiguration>,
            ::windows::core::GetTrustLevel,
            AccountName::<Impl, OFFSET>,
            SetAccountName::<Impl, OFFSET>,
            DeviceAccountTypeId::<Impl, OFFSET>,
            SetDeviceAccountTypeId::<Impl, OFFSET>,
            ServerType::<Impl, OFFSET>,
            SetServerType::<Impl, OFFSET>,
            EmailAddress::<Impl, OFFSET>,
            SetEmailAddress::<Impl, OFFSET>,
            Domain::<Impl, OFFSET>,
            SetDomain::<Impl, OFFSET>,
            EmailSyncEnabled::<Impl, OFFSET>,
            SetEmailSyncEnabled::<Impl, OFFSET>,
            ContactsSyncEnabled::<Impl, OFFSET>,
            SetContactsSyncEnabled::<Impl, OFFSET>,
            CalendarSyncEnabled::<Impl, OFFSET>,
            SetCalendarSyncEnabled::<Impl, OFFSET>,
            IncomingServerAddress::<Impl, OFFSET>,
            SetIncomingServerAddress::<Impl, OFFSET>,
            IncomingServerPort::<Impl, OFFSET>,
            SetIncomingServerPort::<Impl, OFFSET>,
            IncomingServerRequiresSsl::<Impl, OFFSET>,
            SetIncomingServerRequiresSsl::<Impl, OFFSET>,
            IncomingServerUsername::<Impl, OFFSET>,
            SetIncomingServerUsername::<Impl, OFFSET>,
            OutgoingServerAddress::<Impl, OFFSET>,
            SetOutgoingServerAddress::<Impl, OFFSET>,
            OutgoingServerPort::<Impl, OFFSET>,
            SetOutgoingServerPort::<Impl, OFFSET>,
            OutgoingServerRequiresSsl::<Impl, OFFSET>,
            SetOutgoingServerRequiresSsl::<Impl, OFFSET>,
            OutgoingServerUsername::<Impl, OFFSET>,
            SetOutgoingServerUsername::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceAccountConfiguration2Impl: Sized {
    fn IncomingServerCredential(&self) -> ::windows::core::Result<super::super::super::Security::Credentials::PasswordCredential>;
    fn SetIncomingServerCredential(&self, value: &::core::option::Option<super::super::super::Security::Credentials::PasswordCredential>) -> ::windows::core::Result<()>;
    fn OutgoingServerCredential(&self) -> ::windows::core::Result<super::super::super::Security::Credentials::PasswordCredential>;
    fn SetOutgoingServerCredential(&self, value: &::core::option::Option<super::super::super::Security::Credentials::PasswordCredential>) -> ::windows::core::Result<()>;
    fn OAuthRefreshToken(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetOAuthRefreshToken(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsExternallyManaged(&self) -> ::windows::core::Result<bool>;
    fn SetIsExternallyManaged(&self, value: bool) -> ::windows::core::Result<()>;
    fn AccountIconId(&self) -> ::windows::core::Result<DeviceAccountIconId>;
    fn SetAccountIconId(&self, value: DeviceAccountIconId) -> ::windows::core::Result<()>;
    fn AuthenticationType(&self) -> ::windows::core::Result<DeviceAccountAuthenticationType>;
    fn SetAuthenticationType(&self, value: DeviceAccountAuthenticationType) -> ::windows::core::Result<()>;
    fn IsSsoAuthenticationSupported(&self) -> ::windows::core::Result<bool>;
    fn SsoAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSsoAccountId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AlwaysDownloadFullMessage(&self) -> ::windows::core::Result<bool>;
    fn SetAlwaysDownloadFullMessage(&self, value: bool) -> ::windows::core::Result<()>;
    fn DoesPolicyAllowMailSync(&self) -> ::windows::core::Result<bool>;
    fn SyncScheduleKind(&self) -> ::windows::core::Result<DeviceAccountSyncScheduleKind>;
    fn SetSyncScheduleKind(&self, value: DeviceAccountSyncScheduleKind) -> ::windows::core::Result<()>;
    fn MailAgeFilter(&self) -> ::windows::core::Result<DeviceAccountMailAgeFilter>;
    fn SetMailAgeFilter(&self, value: DeviceAccountMailAgeFilter) -> ::windows::core::Result<()>;
    fn IsClientAuthenticationCertificateRequired(&self) -> ::windows::core::Result<bool>;
    fn SetIsClientAuthenticationCertificateRequired(&self, value: bool) -> ::windows::core::Result<()>;
    fn AutoSelectAuthenticationCertificate(&self) -> ::windows::core::Result<bool>;
    fn SetAutoSelectAuthenticationCertificate(&self, value: bool) -> ::windows::core::Result<()>;
    fn AuthenticationCertificateId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAuthenticationCertificateId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn CardDavSyncScheduleKind(&self) -> ::windows::core::Result<DeviceAccountSyncScheduleKind>;
    fn SetCardDavSyncScheduleKind(&self, value: DeviceAccountSyncScheduleKind) -> ::windows::core::Result<()>;
    fn CalDavSyncScheduleKind(&self) -> ::windows::core::Result<DeviceAccountSyncScheduleKind>;
    fn SetCalDavSyncScheduleKind(&self, value: DeviceAccountSyncScheduleKind) -> ::windows::core::Result<()>;
    fn CardDavServerUrl(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn SetCardDavServerUrl(&self, value: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn CardDavRequiresSsl(&self) -> ::windows::core::Result<bool>;
    fn SetCardDavRequiresSsl(&self, value: bool) -> ::windows::core::Result<()>;
    fn CalDavServerUrl(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn SetCalDavServerUrl(&self, value: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn CalDavRequiresSsl(&self) -> ::windows::core::Result<bool>;
    fn SetCalDavRequiresSsl(&self, value: bool) -> ::windows::core::Result<()>;
    fn WasModifiedByUser(&self) -> ::windows::core::Result<bool>;
    fn SetWasModifiedByUser(&self, value: bool) -> ::windows::core::Result<()>;
    fn WasIncomingServerCertificateHashConfirmed(&self) -> ::windows::core::Result<bool>;
    fn SetWasIncomingServerCertificateHashConfirmed(&self, value: bool) -> ::windows::core::Result<()>;
    fn IncomingServerCertificateHash(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetIncomingServerCertificateHash(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsOutgoingServerAuthenticationRequired(&self) -> ::windows::core::Result<bool>;
    fn SetIsOutgoingServerAuthenticationRequired(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsOutgoingServerAuthenticationEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsOutgoingServerAuthenticationEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn WasOutgoingServerCertificateHashConfirmed(&self) -> ::windows::core::Result<bool>;
    fn SetWasOutgoingServerCertificateHashConfirmed(&self, value: bool) -> ::windows::core::Result<()>;
    fn OutgoingServerCertificateHash(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetOutgoingServerCertificateHash(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsSyncScheduleManagedBySystem(&self) -> ::windows::core::Result<bool>;
    fn SetIsSyncScheduleManagedBySystem(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeviceAccountConfiguration2 {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.SystemAccess.IDeviceAccountConfiguration2";
}
#[cfg(feature = "implement_exclusive")]
impl IDeviceAccountConfiguration2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>() -> IDeviceAccountConfiguration2Vtbl {
        unsafe extern "system" fn IncomingServerCredential<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIncomingServerCredential<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIncomingServerCredential(&*(&value as *const <super::super::super::Security::Credentials::PasswordCredential as ::windows::core::Abi>::Abi as *const <super::super::super::Security::Credentials::PasswordCredential as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OutgoingServerCredential<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOutgoingServerCredential<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutgoingServerCredential(&*(&value as *const <super::super::super::Security::Credentials::PasswordCredential as ::windows::core::Abi>::Abi as *const <super::super::super::Security::Credentials::PasswordCredential as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OAuthRefreshToken<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOAuthRefreshToken<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOAuthRefreshToken(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsExternallyManaged<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsExternallyManaged<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsExternallyManaged(value).into()
        }
        unsafe extern "system" fn AccountIconId<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DeviceAccountIconId) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAccountIconId<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: DeviceAccountIconId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAccountIconId(value).into()
        }
        unsafe extern "system" fn AuthenticationType<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DeviceAccountAuthenticationType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAuthenticationType<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: DeviceAccountAuthenticationType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAuthenticationType(value).into()
        }
        unsafe extern "system" fn IsSsoAuthenticationSupported<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SsoAccountId<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSsoAccountId<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSsoAccountId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AlwaysDownloadFullMessage<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAlwaysDownloadFullMessage<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlwaysDownloadFullMessage(value).into()
        }
        unsafe extern "system" fn DoesPolicyAllowMailSync<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SyncScheduleKind<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DeviceAccountSyncScheduleKind) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSyncScheduleKind<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: DeviceAccountSyncScheduleKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSyncScheduleKind(value).into()
        }
        unsafe extern "system" fn MailAgeFilter<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DeviceAccountMailAgeFilter) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMailAgeFilter<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: DeviceAccountMailAgeFilter) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMailAgeFilter(value).into()
        }
        unsafe extern "system" fn IsClientAuthenticationCertificateRequired<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsClientAuthenticationCertificateRequired<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsClientAuthenticationCertificateRequired(value).into()
        }
        unsafe extern "system" fn AutoSelectAuthenticationCertificate<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAutoSelectAuthenticationCertificate<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoSelectAuthenticationCertificate(value).into()
        }
        unsafe extern "system" fn AuthenticationCertificateId<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAuthenticationCertificateId<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAuthenticationCertificateId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CardDavSyncScheduleKind<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DeviceAccountSyncScheduleKind) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCardDavSyncScheduleKind<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: DeviceAccountSyncScheduleKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCardDavSyncScheduleKind(value).into()
        }
        unsafe extern "system" fn CalDavSyncScheduleKind<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DeviceAccountSyncScheduleKind) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCalDavSyncScheduleKind<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: DeviceAccountSyncScheduleKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCalDavSyncScheduleKind(value).into()
        }
        unsafe extern "system" fn CardDavServerUrl<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCardDavServerUrl<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCardDavServerUrl(&*(&value as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CardDavRequiresSsl<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCardDavRequiresSsl<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCardDavRequiresSsl(value).into()
        }
        unsafe extern "system" fn CalDavServerUrl<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCalDavServerUrl<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCalDavServerUrl(&*(&value as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CalDavRequiresSsl<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCalDavRequiresSsl<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCalDavRequiresSsl(value).into()
        }
        unsafe extern "system" fn WasModifiedByUser<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetWasModifiedByUser<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWasModifiedByUser(value).into()
        }
        unsafe extern "system" fn WasIncomingServerCertificateHashConfirmed<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetWasIncomingServerCertificateHashConfirmed<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWasIncomingServerCertificateHashConfirmed(value).into()
        }
        unsafe extern "system" fn IncomingServerCertificateHash<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIncomingServerCertificateHash<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIncomingServerCertificateHash(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsOutgoingServerAuthenticationRequired<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsOutgoingServerAuthenticationRequired<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsOutgoingServerAuthenticationRequired(value).into()
        }
        unsafe extern "system" fn IsOutgoingServerAuthenticationEnabled<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsOutgoingServerAuthenticationEnabled<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsOutgoingServerAuthenticationEnabled(value).into()
        }
        unsafe extern "system" fn WasOutgoingServerCertificateHashConfirmed<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetWasOutgoingServerCertificateHashConfirmed<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWasOutgoingServerCertificateHashConfirmed(value).into()
        }
        unsafe extern "system" fn OutgoingServerCertificateHash<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOutgoingServerCertificateHash<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutgoingServerCertificateHash(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn IsSyncScheduleManagedBySystem<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsSyncScheduleManagedBySystem<Impl: IDeviceAccountConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsSyncScheduleManagedBySystem(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDeviceAccountConfiguration2>,
            ::windows::core::GetTrustLevel,
            IncomingServerCredential::<Impl, OFFSET>,
            SetIncomingServerCredential::<Impl, OFFSET>,
            OutgoingServerCredential::<Impl, OFFSET>,
            SetOutgoingServerCredential::<Impl, OFFSET>,
            OAuthRefreshToken::<Impl, OFFSET>,
            SetOAuthRefreshToken::<Impl, OFFSET>,
            IsExternallyManaged::<Impl, OFFSET>,
            SetIsExternallyManaged::<Impl, OFFSET>,
            AccountIconId::<Impl, OFFSET>,
            SetAccountIconId::<Impl, OFFSET>,
            AuthenticationType::<Impl, OFFSET>,
            SetAuthenticationType::<Impl, OFFSET>,
            IsSsoAuthenticationSupported::<Impl, OFFSET>,
            SsoAccountId::<Impl, OFFSET>,
            SetSsoAccountId::<Impl, OFFSET>,
            AlwaysDownloadFullMessage::<Impl, OFFSET>,
            SetAlwaysDownloadFullMessage::<Impl, OFFSET>,
            DoesPolicyAllowMailSync::<Impl, OFFSET>,
            SyncScheduleKind::<Impl, OFFSET>,
            SetSyncScheduleKind::<Impl, OFFSET>,
            MailAgeFilter::<Impl, OFFSET>,
            SetMailAgeFilter::<Impl, OFFSET>,
            IsClientAuthenticationCertificateRequired::<Impl, OFFSET>,
            SetIsClientAuthenticationCertificateRequired::<Impl, OFFSET>,
            AutoSelectAuthenticationCertificate::<Impl, OFFSET>,
            SetAutoSelectAuthenticationCertificate::<Impl, OFFSET>,
            AuthenticationCertificateId::<Impl, OFFSET>,
            SetAuthenticationCertificateId::<Impl, OFFSET>,
            CardDavSyncScheduleKind::<Impl, OFFSET>,
            SetCardDavSyncScheduleKind::<Impl, OFFSET>,
            CalDavSyncScheduleKind::<Impl, OFFSET>,
            SetCalDavSyncScheduleKind::<Impl, OFFSET>,
            CardDavServerUrl::<Impl, OFFSET>,
            SetCardDavServerUrl::<Impl, OFFSET>,
            CardDavRequiresSsl::<Impl, OFFSET>,
            SetCardDavRequiresSsl::<Impl, OFFSET>,
            CalDavServerUrl::<Impl, OFFSET>,
            SetCalDavServerUrl::<Impl, OFFSET>,
            CalDavRequiresSsl::<Impl, OFFSET>,
            SetCalDavRequiresSsl::<Impl, OFFSET>,
            WasModifiedByUser::<Impl, OFFSET>,
            SetWasModifiedByUser::<Impl, OFFSET>,
            WasIncomingServerCertificateHashConfirmed::<Impl, OFFSET>,
            SetWasIncomingServerCertificateHashConfirmed::<Impl, OFFSET>,
            IncomingServerCertificateHash::<Impl, OFFSET>,
            SetIncomingServerCertificateHash::<Impl, OFFSET>,
            IsOutgoingServerAuthenticationRequired::<Impl, OFFSET>,
            SetIsOutgoingServerAuthenticationRequired::<Impl, OFFSET>,
            IsOutgoingServerAuthenticationEnabled::<Impl, OFFSET>,
            SetIsOutgoingServerAuthenticationEnabled::<Impl, OFFSET>,
            WasOutgoingServerCertificateHashConfirmed::<Impl, OFFSET>,
            SetWasOutgoingServerCertificateHashConfirmed::<Impl, OFFSET>,
            OutgoingServerCertificateHash::<Impl, OFFSET>,
            SetOutgoingServerCertificateHash::<Impl, OFFSET>,
            IsSyncScheduleManagedBySystem::<Impl, OFFSET>,
            SetIsSyncScheduleManagedBySystem::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataAccountSystemAccessManagerStaticsImpl: Sized {
    fn AddAndShowDeviceAccountsAsync(&self, accounts: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<DeviceAccountConfiguration>>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserDataAccountSystemAccessManagerStatics {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.SystemAccess.IUserDataAccountSystemAccessManagerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IUserDataAccountSystemAccessManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserDataAccountSystemAccessManagerStaticsImpl, const OFFSET: isize>() -> IUserDataAccountSystemAccessManagerStaticsVtbl {
        unsafe extern "system" fn AddAndShowDeviceAccountsAsync<Impl: IUserDataAccountSystemAccessManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accounts: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUserDataAccountSystemAccessManagerStatics>, ::windows::core::GetTrustLevel, AddAndShowDeviceAccountsAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataAccountSystemAccessManagerStatics2Impl: Sized {
    fn SuppressLocalAccountWithAccountAsync(&self, userdataaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn CreateDeviceAccountAsync(&self, account: &::core::option::Option<DeviceAccountConfiguration>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn DeleteDeviceAccountAsync(&self, accountid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn GetDeviceAccountConfigurationAsync(&self, accountid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<DeviceAccountConfiguration>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserDataAccountSystemAccessManagerStatics2 {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.SystemAccess.IUserDataAccountSystemAccessManagerStatics2";
}
#[cfg(feature = "implement_exclusive")]
impl IUserDataAccountSystemAccessManagerStatics2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserDataAccountSystemAccessManagerStatics2Impl, const OFFSET: isize>() -> IUserDataAccountSystemAccessManagerStatics2Vtbl {
        unsafe extern "system" fn SuppressLocalAccountWithAccountAsync<Impl: IUserDataAccountSystemAccessManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, userdataaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateDeviceAccountAsync<Impl: IUserDataAccountSystemAccessManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, account: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DeleteDeviceAccountAsync<Impl: IUserDataAccountSystemAccessManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDeviceAccountConfigurationAsync<Impl: IUserDataAccountSystemAccessManagerStatics2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IUserDataAccountSystemAccessManagerStatics2>, ::windows::core::GetTrustLevel, SuppressLocalAccountWithAccountAsync::<Impl, OFFSET>, CreateDeviceAccountAsync::<Impl, OFFSET>, DeleteDeviceAccountAsync::<Impl, OFFSET>, GetDeviceAccountConfigurationAsync::<Impl, OFFSET>)
    }
}

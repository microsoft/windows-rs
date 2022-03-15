#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DeviceAccountAuthenticationType(pub i32);
impl DeviceAccountAuthenticationType {
    pub const Basic: Self = Self(0i32);
    pub const OAuth: Self = Self(1i32);
    pub const SingleSignOn: Self = Self(2i32);
}
impl ::core::marker::Copy for DeviceAccountAuthenticationType {}
impl ::core::clone::Clone for DeviceAccountAuthenticationType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DeviceAccountAuthenticationType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DeviceAccountAuthenticationType {
    type Abi = Self;
}
impl ::core::fmt::Debug for DeviceAccountAuthenticationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceAccountAuthenticationType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DeviceAccountAuthenticationType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataAccounts.SystemAccess.DeviceAccountAuthenticationType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
#[repr(transparent)]
pub struct DeviceAccountConfiguration(::windows::core::IUnknown);
impl DeviceAccountConfiguration {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<DeviceAccountConfiguration, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn AccountName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccountName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn SetAccountName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAccountName)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn DeviceAccountTypeId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceAccountTypeId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn SetDeviceAccountTypeId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDeviceAccountTypeId)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn ServerType(&self) -> ::windows::core::Result<DeviceAccountServerType> {
        let this = self;
        unsafe {
            let mut result__: DeviceAccountServerType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ServerType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DeviceAccountServerType>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn SetServerType(&self, value: DeviceAccountServerType) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetServerType)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn EmailAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EmailAddress)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn SetEmailAddress<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetEmailAddress)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn Domain(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Domain)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn SetDomain<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDomain)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn EmailSyncEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EmailSyncEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn SetEmailSyncEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetEmailSyncEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn ContactsSyncEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContactsSyncEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn SetContactsSyncEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetContactsSyncEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn CalendarSyncEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CalendarSyncEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn SetCalendarSyncEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCalendarSyncEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn IncomingServerAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IncomingServerAddress)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn SetIncomingServerAddress<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIncomingServerAddress)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn IncomingServerPort(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IncomingServerPort)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn SetIncomingServerPort(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIncomingServerPort)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn IncomingServerRequiresSsl(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IncomingServerRequiresSsl)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn SetIncomingServerRequiresSsl(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIncomingServerRequiresSsl)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn IncomingServerUsername(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IncomingServerUsername)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn SetIncomingServerUsername<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIncomingServerUsername)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn OutgoingServerAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OutgoingServerAddress)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn SetOutgoingServerAddress<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetOutgoingServerAddress)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn OutgoingServerPort(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OutgoingServerPort)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn SetOutgoingServerPort(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetOutgoingServerPort)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn OutgoingServerRequiresSsl(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OutgoingServerRequiresSsl)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn SetOutgoingServerRequiresSsl(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetOutgoingServerRequiresSsl)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn OutgoingServerUsername(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OutgoingServerUsername)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn SetOutgoingServerUsername<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetOutgoingServerUsername)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`, `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn IncomingServerCredential(&self) -> ::windows::core::Result<super::super::super::Security::Credentials::PasswordCredential> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IncomingServerCredential)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`, `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn SetIncomingServerCredential<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Security::Credentials::PasswordCredential>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetIncomingServerCredential)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`, `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn OutgoingServerCredential(&self) -> ::windows::core::Result<super::super::super::Security::Credentials::PasswordCredential> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OutgoingServerCredential)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`, `\"Security_Credentials\"`*"]
    #[cfg(feature = "Security_Credentials")]
    pub fn SetOutgoingServerCredential<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Security::Credentials::PasswordCredential>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetOutgoingServerCredential)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn OAuthRefreshToken(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OAuthRefreshToken)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn SetOAuthRefreshToken<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetOAuthRefreshToken)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn IsExternallyManaged(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsExternallyManaged)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn SetIsExternallyManaged(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetIsExternallyManaged)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn AccountIconId(&self) -> ::windows::core::Result<DeviceAccountIconId> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: DeviceAccountIconId = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccountIconId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DeviceAccountIconId>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn SetAccountIconId(&self, value: DeviceAccountIconId) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetAccountIconId)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn AuthenticationType(&self) -> ::windows::core::Result<DeviceAccountAuthenticationType> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: DeviceAccountAuthenticationType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AuthenticationType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DeviceAccountAuthenticationType>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn SetAuthenticationType(&self, value: DeviceAccountAuthenticationType) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetAuthenticationType)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn IsSsoAuthenticationSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsSsoAuthenticationSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn SsoAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SsoAccountId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn SetSsoAccountId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetSsoAccountId)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn AlwaysDownloadFullMessage(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AlwaysDownloadFullMessage)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn SetAlwaysDownloadFullMessage(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetAlwaysDownloadFullMessage)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn DoesPolicyAllowMailSync(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DoesPolicyAllowMailSync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn SyncScheduleKind(&self) -> ::windows::core::Result<DeviceAccountSyncScheduleKind> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: DeviceAccountSyncScheduleKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SyncScheduleKind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DeviceAccountSyncScheduleKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn SetSyncScheduleKind(&self, value: DeviceAccountSyncScheduleKind) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetSyncScheduleKind)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn MailAgeFilter(&self) -> ::windows::core::Result<DeviceAccountMailAgeFilter> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: DeviceAccountMailAgeFilter = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MailAgeFilter)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DeviceAccountMailAgeFilter>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn SetMailAgeFilter(&self, value: DeviceAccountMailAgeFilter) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetMailAgeFilter)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn IsClientAuthenticationCertificateRequired(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsClientAuthenticationCertificateRequired)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn SetIsClientAuthenticationCertificateRequired(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetIsClientAuthenticationCertificateRequired)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn AutoSelectAuthenticationCertificate(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AutoSelectAuthenticationCertificate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn SetAutoSelectAuthenticationCertificate(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetAutoSelectAuthenticationCertificate)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn AuthenticationCertificateId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AuthenticationCertificateId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn SetAuthenticationCertificateId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetAuthenticationCertificateId)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn CardDavSyncScheduleKind(&self) -> ::windows::core::Result<DeviceAccountSyncScheduleKind> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: DeviceAccountSyncScheduleKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CardDavSyncScheduleKind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DeviceAccountSyncScheduleKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn SetCardDavSyncScheduleKind(&self, value: DeviceAccountSyncScheduleKind) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetCardDavSyncScheduleKind)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn CalDavSyncScheduleKind(&self) -> ::windows::core::Result<DeviceAccountSyncScheduleKind> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: DeviceAccountSyncScheduleKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CalDavSyncScheduleKind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DeviceAccountSyncScheduleKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn SetCalDavSyncScheduleKind(&self, value: DeviceAccountSyncScheduleKind) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetCalDavSyncScheduleKind)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CardDavServerUrl(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CardDavServerUrl)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetCardDavServerUrl<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetCardDavServerUrl)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn CardDavRequiresSsl(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CardDavRequiresSsl)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn SetCardDavRequiresSsl(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetCardDavRequiresSsl)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CalDavServerUrl(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CalDavServerUrl)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetCalDavServerUrl<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetCalDavServerUrl)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn CalDavRequiresSsl(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CalDavRequiresSsl)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn SetCalDavRequiresSsl(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetCalDavRequiresSsl)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn WasModifiedByUser(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WasModifiedByUser)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn SetWasModifiedByUser(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetWasModifiedByUser)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn WasIncomingServerCertificateHashConfirmed(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WasIncomingServerCertificateHashConfirmed)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn SetWasIncomingServerCertificateHashConfirmed(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetWasIncomingServerCertificateHashConfirmed)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn IncomingServerCertificateHash(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IncomingServerCertificateHash)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn SetIncomingServerCertificateHash<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetIncomingServerCertificateHash)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn IsOutgoingServerAuthenticationRequired(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsOutgoingServerAuthenticationRequired)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn SetIsOutgoingServerAuthenticationRequired(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetIsOutgoingServerAuthenticationRequired)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn IsOutgoingServerAuthenticationEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsOutgoingServerAuthenticationEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn SetIsOutgoingServerAuthenticationEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetIsOutgoingServerAuthenticationEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn WasOutgoingServerCertificateHashConfirmed(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).WasOutgoingServerCertificateHashConfirmed)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn SetWasOutgoingServerCertificateHashConfirmed(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetWasOutgoingServerCertificateHashConfirmed)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn OutgoingServerCertificateHash(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OutgoingServerCertificateHash)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn SetOutgoingServerCertificateHash<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetOutgoingServerCertificateHash)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn IsSyncScheduleManagedBySystem(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsSyncScheduleManagedBySystem)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
    pub fn SetIsSyncScheduleManagedBySystem(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetIsSyncScheduleManagedBySystem)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for DeviceAccountConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for DeviceAccountConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceAccountConfiguration {}
impl ::core::fmt::Debug for DeviceAccountConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceAccountConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DeviceAccountConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataAccounts.SystemAccess.DeviceAccountConfiguration;{ad0123a3-fbdc-4d1b-be43-5a27ea4a1b63})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for DeviceAccountConfiguration {
    type Vtable = IDeviceAccountConfiguration_Vtbl;
    const IID: ::windows::core::GUID = <IDeviceAccountConfiguration as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for DeviceAccountConfiguration {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.SystemAccess.DeviceAccountConfiguration";
}
impl ::core::convert::From<DeviceAccountConfiguration> for ::windows::core::IUnknown {
    fn from(value: DeviceAccountConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeviceAccountConfiguration> for ::windows::core::IUnknown {
    fn from(value: &DeviceAccountConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for DeviceAccountConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a DeviceAccountConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<DeviceAccountConfiguration> for ::windows::core::IInspectable {
    fn from(value: DeviceAccountConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&DeviceAccountConfiguration> for ::windows::core::IInspectable {
    fn from(value: &DeviceAccountConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for DeviceAccountConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a DeviceAccountConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for DeviceAccountConfiguration {}
unsafe impl ::core::marker::Sync for DeviceAccountConfiguration {}
#[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DeviceAccountIconId(pub i32);
impl DeviceAccountIconId {
    pub const Exchange: Self = Self(0i32);
    pub const Msa: Self = Self(1i32);
    pub const Outlook: Self = Self(2i32);
    pub const Generic: Self = Self(3i32);
}
impl ::core::marker::Copy for DeviceAccountIconId {}
impl ::core::clone::Clone for DeviceAccountIconId {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DeviceAccountIconId {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DeviceAccountIconId {
    type Abi = Self;
}
impl ::core::fmt::Debug for DeviceAccountIconId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceAccountIconId").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DeviceAccountIconId {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataAccounts.SystemAccess.DeviceAccountIconId;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DeviceAccountMailAgeFilter(pub i32);
impl DeviceAccountMailAgeFilter {
    pub const All: Self = Self(0i32);
    pub const Last1Day: Self = Self(1i32);
    pub const Last3Days: Self = Self(2i32);
    pub const Last7Days: Self = Self(3i32);
    pub const Last14Days: Self = Self(4i32);
    pub const Last30Days: Self = Self(5i32);
    pub const Last90Days: Self = Self(6i32);
}
impl ::core::marker::Copy for DeviceAccountMailAgeFilter {}
impl ::core::clone::Clone for DeviceAccountMailAgeFilter {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DeviceAccountMailAgeFilter {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DeviceAccountMailAgeFilter {
    type Abi = Self;
}
impl ::core::fmt::Debug for DeviceAccountMailAgeFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceAccountMailAgeFilter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DeviceAccountMailAgeFilter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataAccounts.SystemAccess.DeviceAccountMailAgeFilter;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DeviceAccountServerType(pub i32);
impl DeviceAccountServerType {
    pub const Exchange: Self = Self(0i32);
    pub const Pop: Self = Self(1i32);
    pub const Imap: Self = Self(2i32);
}
impl ::core::marker::Copy for DeviceAccountServerType {}
impl ::core::clone::Clone for DeviceAccountServerType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DeviceAccountServerType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DeviceAccountServerType {
    type Abi = Self;
}
impl ::core::fmt::Debug for DeviceAccountServerType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceAccountServerType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DeviceAccountServerType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataAccounts.SystemAccess.DeviceAccountServerType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DeviceAccountSyncScheduleKind(pub i32);
impl DeviceAccountSyncScheduleKind {
    pub const Manual: Self = Self(0i32);
    pub const Every15Minutes: Self = Self(1i32);
    pub const Every30Minutes: Self = Self(2i32);
    pub const Every60Minutes: Self = Self(3i32);
    pub const Every2Hours: Self = Self(4i32);
    pub const Daily: Self = Self(5i32);
    pub const AsItemsArrive: Self = Self(6i32);
}
impl ::core::marker::Copy for DeviceAccountSyncScheduleKind {}
impl ::core::clone::Clone for DeviceAccountSyncScheduleKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DeviceAccountSyncScheduleKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DeviceAccountSyncScheduleKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for DeviceAccountSyncScheduleKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceAccountSyncScheduleKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DeviceAccountSyncScheduleKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataAccounts.SystemAccess.DeviceAccountSyncScheduleKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceAccountConfiguration(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDeviceAccountConfiguration {
    type Vtable = IDeviceAccountConfiguration_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xad0123a3_fbdc_4d1b_be43_5a27ea4a1b63);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceAccountConfiguration_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AccountName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetAccountName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DeviceAccountTypeId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDeviceAccountTypeId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ServerType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DeviceAccountServerType) -> ::windows::core::HRESULT,
    pub SetServerType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: DeviceAccountServerType) -> ::windows::core::HRESULT,
    pub EmailAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetEmailAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Domain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDomain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub EmailSyncEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetEmailSyncEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub ContactsSyncEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetContactsSyncEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub CalendarSyncEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetCalendarSyncEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IncomingServerAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetIncomingServerAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IncomingServerPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetIncomingServerPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub IncomingServerRequiresSsl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIncomingServerRequiresSsl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IncomingServerUsername: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetIncomingServerUsername: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub OutgoingServerAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetOutgoingServerAddress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub OutgoingServerPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetOutgoingServerPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub OutgoingServerRequiresSsl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetOutgoingServerRequiresSsl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub OutgoingServerUsername: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetOutgoingServerUsername: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IDeviceAccountConfiguration2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IDeviceAccountConfiguration2 {
    type Vtable = IDeviceAccountConfiguration2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2b2e5a6_728d_4a4a_8945_2bf8580136de);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceAccountConfiguration2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Security_Credentials")]
    pub IncomingServerCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    IncomingServerCredential: usize,
    #[cfg(feature = "Security_Credentials")]
    pub SetIncomingServerCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    SetIncomingServerCredential: usize,
    #[cfg(feature = "Security_Credentials")]
    pub OutgoingServerCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    OutgoingServerCredential: usize,
    #[cfg(feature = "Security_Credentials")]
    pub SetOutgoingServerCredential: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))]
    SetOutgoingServerCredential: usize,
    pub OAuthRefreshToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetOAuthRefreshToken: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsExternallyManaged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsExternallyManaged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub AccountIconId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DeviceAccountIconId) -> ::windows::core::HRESULT,
    pub SetAccountIconId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: DeviceAccountIconId) -> ::windows::core::HRESULT,
    pub AuthenticationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DeviceAccountAuthenticationType) -> ::windows::core::HRESULT,
    pub SetAuthenticationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: DeviceAccountAuthenticationType) -> ::windows::core::HRESULT,
    pub IsSsoAuthenticationSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SsoAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetSsoAccountId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub AlwaysDownloadFullMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAlwaysDownloadFullMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub DoesPolicyAllowMailSync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SyncScheduleKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DeviceAccountSyncScheduleKind) -> ::windows::core::HRESULT,
    pub SetSyncScheduleKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: DeviceAccountSyncScheduleKind) -> ::windows::core::HRESULT,
    pub MailAgeFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DeviceAccountMailAgeFilter) -> ::windows::core::HRESULT,
    pub SetMailAgeFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: DeviceAccountMailAgeFilter) -> ::windows::core::HRESULT,
    pub IsClientAuthenticationCertificateRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsClientAuthenticationCertificateRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub AutoSelectAuthenticationCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAutoSelectAuthenticationCertificate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub AuthenticationCertificateId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetAuthenticationCertificateId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub CardDavSyncScheduleKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DeviceAccountSyncScheduleKind) -> ::windows::core::HRESULT,
    pub SetCardDavSyncScheduleKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: DeviceAccountSyncScheduleKind) -> ::windows::core::HRESULT,
    pub CalDavSyncScheduleKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DeviceAccountSyncScheduleKind) -> ::windows::core::HRESULT,
    pub SetCalDavSyncScheduleKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: DeviceAccountSyncScheduleKind) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CardDavServerUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CardDavServerUrl: usize,
    #[cfg(feature = "Foundation")]
    pub SetCardDavServerUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetCardDavServerUrl: usize,
    pub CardDavRequiresSsl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetCardDavRequiresSsl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CalDavServerUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CalDavServerUrl: usize,
    #[cfg(feature = "Foundation")]
    pub SetCalDavServerUrl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetCalDavServerUrl: usize,
    pub CalDavRequiresSsl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetCalDavRequiresSsl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub WasModifiedByUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetWasModifiedByUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub WasIncomingServerCertificateHashConfirmed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetWasIncomingServerCertificateHashConfirmed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IncomingServerCertificateHash: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetIncomingServerCertificateHash: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsOutgoingServerAuthenticationRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsOutgoingServerAuthenticationRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsOutgoingServerAuthenticationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsOutgoingServerAuthenticationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub WasOutgoingServerCertificateHashConfirmed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetWasOutgoingServerCertificateHashConfirmed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub OutgoingServerCertificateHash: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetOutgoingServerCertificateHash: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsSyncScheduleManagedBySystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsSyncScheduleManagedBySystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataAccountSystemAccessManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserDataAccountSystemAccessManagerStatics {
    type Vtable = IUserDataAccountSystemAccessManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9d6b11b9_cbe5_45f5_822b_c267b81dbdb6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountSystemAccessManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub AddAndShowDeviceAccountsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accounts: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AddAndShowDeviceAccountsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataAccountSystemAccessManagerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserDataAccountSystemAccessManagerStatics2 {
    type Vtable = IUserDataAccountSystemAccessManagerStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x943f854d_4b4e_439f_83d3_979b27c05ac7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountSystemAccessManagerStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub SuppressLocalAccountWithAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, userdataaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SuppressLocalAccountWithAccountAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreateDeviceAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, account: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateDeviceAccountAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteDeviceAccountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteDeviceAccountAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetDeviceAccountConfigurationAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, accountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeviceAccountConfigurationAsync: usize,
}
#[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`*"]
pub struct UserDataAccountSystemAccessManager {}
impl UserDataAccountSystemAccessManager {
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AddAndShowDeviceAccountsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<DeviceAccountConfiguration>>>(accounts: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>> {
        Self::IUserDataAccountSystemAccessManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AddAndShowDeviceAccountsAsync)(::core::mem::transmute_copy(this), accounts.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SuppressLocalAccountWithAccountAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(userdataaccountid: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        Self::IUserDataAccountSystemAccessManagerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SuppressLocalAccountWithAccountAsync)(::core::mem::transmute_copy(this), userdataaccountid.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateDeviceAccountAsync<'a, Param0: ::windows::core::IntoParam<'a, DeviceAccountConfiguration>>(account: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        Self::IUserDataAccountSystemAccessManagerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateDeviceAccountAsync)(::core::mem::transmute_copy(this), account.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteDeviceAccountAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(accountid: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        Self::IUserDataAccountSystemAccessManagerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeleteDeviceAccountAsync)(::core::mem::transmute_copy(this), accountid.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_UserDataAccounts_SystemAccess\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeviceAccountConfigurationAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(accountid: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<DeviceAccountConfiguration>> {
        Self::IUserDataAccountSystemAccessManagerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeviceAccountConfigurationAsync)(::core::mem::transmute_copy(this), accountid.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<DeviceAccountConfiguration>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IUserDataAccountSystemAccessManagerStatics<R, F: FnOnce(&IUserDataAccountSystemAccessManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UserDataAccountSystemAccessManager, IUserDataAccountSystemAccessManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IUserDataAccountSystemAccessManagerStatics2<R, F: FnOnce(&IUserDataAccountSystemAccessManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UserDataAccountSystemAccessManager, IUserDataAccountSystemAccessManagerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for UserDataAccountSystemAccessManager {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.SystemAccess.UserDataAccountSystemAccessManager";
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

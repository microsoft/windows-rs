#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DeviceAccountAuthenticationType(pub i32);
impl DeviceAccountAuthenticationType {
    pub const Basic: DeviceAccountAuthenticationType = DeviceAccountAuthenticationType(0i32);
    pub const OAuth: DeviceAccountAuthenticationType = DeviceAccountAuthenticationType(1i32);
    pub const SingleSignOn: DeviceAccountAuthenticationType = DeviceAccountAuthenticationType(2i32);
}
impl ::std::convert::From<i32> for DeviceAccountAuthenticationType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DeviceAccountAuthenticationType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DeviceAccountAuthenticationType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataAccounts.SystemAccess.DeviceAccountAuthenticationType;i4)");
}
impl ::windows::runtime::DefaultType for DeviceAccountAuthenticationType {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct DeviceAccountConfiguration(::windows::runtime::IInspectable);
impl DeviceAccountConfiguration {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<DeviceAccountConfiguration, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn AccountName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn SetAccountName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn DeviceAccountTypeId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn SetDeviceAccountTypeId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn ServerType(&self) -> ::windows::runtime::Result<DeviceAccountServerType> {
        let this = self;
        unsafe {
            let mut result__: DeviceAccountServerType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<DeviceAccountServerType>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn SetServerType(&self, value: DeviceAccountServerType) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn EmailAddress(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn SetEmailAddress<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn Domain(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn SetDomain<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn EmailSyncEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn SetEmailSyncEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn ContactsSyncEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn SetContactsSyncEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn CalendarSyncEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn SetCalendarSyncEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn IncomingServerAddress(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn SetIncomingServerAddress<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn IncomingServerPort(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn SetIncomingServerPort(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn IncomingServerRequiresSsl(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn SetIncomingServerRequiresSsl(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn IncomingServerUsername(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn SetIncomingServerUsername<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn OutgoingServerAddress(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn SetOutgoingServerAddress<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).31)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn OutgoingServerPort(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).32)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn SetOutgoingServerPort(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).33)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn OutgoingServerRequiresSsl(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).34)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn SetOutgoingServerRequiresSsl(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).35)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn OutgoingServerUsername(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).36)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn SetOutgoingServerUsername<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).37)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`, `Security_Credentials`*"]
    pub fn IncomingServerCredential(&self) -> ::windows::runtime::Result<super::super::super::Security::Credentials::PasswordCredential> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`, `Security_Credentials`*"]
    pub fn SetIncomingServerCredential<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Security::Credentials::PasswordCredential>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`, `Security_Credentials`*"]
    pub fn OutgoingServerCredential(&self) -> ::windows::runtime::Result<super::super::super::Security::Credentials::PasswordCredential> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Security::Credentials::PasswordCredential>(result__)
        }
    }
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`, `Security_Credentials`*"]
    pub fn SetOutgoingServerCredential<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Security::Credentials::PasswordCredential>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn OAuthRefreshToken(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn SetOAuthRefreshToken<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn IsExternallyManaged(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn SetIsExternallyManaged(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn AccountIconId(&self) -> ::windows::runtime::Result<DeviceAccountIconId> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: DeviceAccountIconId = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<DeviceAccountIconId>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn SetAccountIconId(&self, value: DeviceAccountIconId) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn AuthenticationType(&self) -> ::windows::runtime::Result<DeviceAccountAuthenticationType> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: DeviceAccountAuthenticationType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<DeviceAccountAuthenticationType>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn SetAuthenticationType(&self, value: DeviceAccountAuthenticationType) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn IsSsoAuthenticationSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn SsoAccountId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn SetSsoAccountId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn AlwaysDownloadFullMessage(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn SetAlwaysDownloadFullMessage(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn DoesPolicyAllowMailSync(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn SyncScheduleKind(&self) -> ::windows::runtime::Result<DeviceAccountSyncScheduleKind> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: DeviceAccountSyncScheduleKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<DeviceAccountSyncScheduleKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn SetSyncScheduleKind(&self, value: DeviceAccountSyncScheduleKind) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn MailAgeFilter(&self) -> ::windows::runtime::Result<DeviceAccountMailAgeFilter> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: DeviceAccountMailAgeFilter = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), &mut result__).from_abi::<DeviceAccountMailAgeFilter>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn SetMailAgeFilter(&self, value: DeviceAccountMailAgeFilter) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn IsClientAuthenticationCertificateRequired(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn SetIsClientAuthenticationCertificateRequired(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn AutoSelectAuthenticationCertificate(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn SetAutoSelectAuthenticationCertificate(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).31)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn AuthenticationCertificateId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).32)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn SetAuthenticationCertificateId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).33)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn CardDavSyncScheduleKind(&self) -> ::windows::runtime::Result<DeviceAccountSyncScheduleKind> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: DeviceAccountSyncScheduleKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).34)(::std::mem::transmute_copy(this), &mut result__).from_abi::<DeviceAccountSyncScheduleKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn SetCardDavSyncScheduleKind(&self, value: DeviceAccountSyncScheduleKind) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).35)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn CalDavSyncScheduleKind(&self) -> ::windows::runtime::Result<DeviceAccountSyncScheduleKind> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: DeviceAccountSyncScheduleKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).36)(::std::mem::transmute_copy(this), &mut result__).from_abi::<DeviceAccountSyncScheduleKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn SetCalDavSyncScheduleKind(&self, value: DeviceAccountSyncScheduleKind) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).37)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`, `Foundation`*"]
    pub fn CardDavServerUrl(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Uri> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).38)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`, `Foundation`*"]
    pub fn SetCardDavServerUrl<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).39)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn CardDavRequiresSsl(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).40)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn SetCardDavRequiresSsl(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).41)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`, `Foundation`*"]
    pub fn CalDavServerUrl(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Uri> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).42)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`, `Foundation`*"]
    pub fn SetCalDavServerUrl<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).43)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn CalDavRequiresSsl(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).44)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn SetCalDavRequiresSsl(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).45)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn WasModifiedByUser(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).46)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn SetWasModifiedByUser(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).47)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn WasIncomingServerCertificateHashConfirmed(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).48)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn SetWasIncomingServerCertificateHashConfirmed(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).49)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn IncomingServerCertificateHash(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).50)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn SetIncomingServerCertificateHash<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).51)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn IsOutgoingServerAuthenticationRequired(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).52)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn SetIsOutgoingServerAuthenticationRequired(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).53)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn IsOutgoingServerAuthenticationEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).54)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn SetIsOutgoingServerAuthenticationEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).55)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn WasOutgoingServerCertificateHashConfirmed(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).56)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn SetWasOutgoingServerCertificateHashConfirmed(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).57)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn OutgoingServerCertificateHash(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).58)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn SetOutgoingServerCertificateHash<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).59)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn IsSyncScheduleManagedBySystem(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).60)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
    pub fn SetIsSyncScheduleManagedBySystem(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IDeviceAccountConfiguration2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).61)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DeviceAccountConfiguration {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataAccounts.SystemAccess.DeviceAccountConfiguration;{ad0123a3-fbdc-4d1b-be43-5a27ea4a1b63})");
}
unsafe impl ::windows::runtime::Interface for DeviceAccountConfiguration {
    type Vtable = IDeviceAccountConfiguration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2902533027, 64476, 19739, [190, 67, 90, 39, 234, 74, 27, 99]);
}
impl ::windows::runtime::RuntimeName for DeviceAccountConfiguration {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.SystemAccess.DeviceAccountConfiguration";
}
impl ::std::convert::From<DeviceAccountConfiguration> for ::windows::runtime::IUnknown {
    fn from(value: DeviceAccountConfiguration) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&DeviceAccountConfiguration> for ::windows::runtime::IUnknown {
    fn from(value: &DeviceAccountConfiguration) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DeviceAccountConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &DeviceAccountConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<DeviceAccountConfiguration> for ::windows::runtime::IInspectable {
    fn from(value: DeviceAccountConfiguration) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DeviceAccountConfiguration> for ::windows::runtime::IInspectable {
    fn from(value: &DeviceAccountConfiguration) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DeviceAccountConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DeviceAccountConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for DeviceAccountConfiguration {}
unsafe impl ::std::marker::Sync for DeviceAccountConfiguration {}
#[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DeviceAccountIconId(pub i32);
impl DeviceAccountIconId {
    pub const Exchange: DeviceAccountIconId = DeviceAccountIconId(0i32);
    pub const Msa: DeviceAccountIconId = DeviceAccountIconId(1i32);
    pub const Outlook: DeviceAccountIconId = DeviceAccountIconId(2i32);
    pub const Generic: DeviceAccountIconId = DeviceAccountIconId(3i32);
}
impl ::std::convert::From<i32> for DeviceAccountIconId {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DeviceAccountIconId {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DeviceAccountIconId {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataAccounts.SystemAccess.DeviceAccountIconId;i4)");
}
impl ::windows::runtime::DefaultType for DeviceAccountIconId {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DeviceAccountMailAgeFilter(pub i32);
impl DeviceAccountMailAgeFilter {
    pub const All: DeviceAccountMailAgeFilter = DeviceAccountMailAgeFilter(0i32);
    pub const Last1Day: DeviceAccountMailAgeFilter = DeviceAccountMailAgeFilter(1i32);
    pub const Last3Days: DeviceAccountMailAgeFilter = DeviceAccountMailAgeFilter(2i32);
    pub const Last7Days: DeviceAccountMailAgeFilter = DeviceAccountMailAgeFilter(3i32);
    pub const Last14Days: DeviceAccountMailAgeFilter = DeviceAccountMailAgeFilter(4i32);
    pub const Last30Days: DeviceAccountMailAgeFilter = DeviceAccountMailAgeFilter(5i32);
    pub const Last90Days: DeviceAccountMailAgeFilter = DeviceAccountMailAgeFilter(6i32);
}
impl ::std::convert::From<i32> for DeviceAccountMailAgeFilter {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DeviceAccountMailAgeFilter {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DeviceAccountMailAgeFilter {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataAccounts.SystemAccess.DeviceAccountMailAgeFilter;i4)");
}
impl ::windows::runtime::DefaultType for DeviceAccountMailAgeFilter {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DeviceAccountServerType(pub i32);
impl DeviceAccountServerType {
    pub const Exchange: DeviceAccountServerType = DeviceAccountServerType(0i32);
    pub const Pop: DeviceAccountServerType = DeviceAccountServerType(1i32);
    pub const Imap: DeviceAccountServerType = DeviceAccountServerType(2i32);
}
impl ::std::convert::From<i32> for DeviceAccountServerType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DeviceAccountServerType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DeviceAccountServerType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataAccounts.SystemAccess.DeviceAccountServerType;i4)");
}
impl ::windows::runtime::DefaultType for DeviceAccountServerType {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DeviceAccountSyncScheduleKind(pub i32);
impl DeviceAccountSyncScheduleKind {
    pub const Manual: DeviceAccountSyncScheduleKind = DeviceAccountSyncScheduleKind(0i32);
    pub const Every15Minutes: DeviceAccountSyncScheduleKind = DeviceAccountSyncScheduleKind(1i32);
    pub const Every30Minutes: DeviceAccountSyncScheduleKind = DeviceAccountSyncScheduleKind(2i32);
    pub const Every60Minutes: DeviceAccountSyncScheduleKind = DeviceAccountSyncScheduleKind(3i32);
    pub const Every2Hours: DeviceAccountSyncScheduleKind = DeviceAccountSyncScheduleKind(4i32);
    pub const Daily: DeviceAccountSyncScheduleKind = DeviceAccountSyncScheduleKind(5i32);
    pub const AsItemsArrive: DeviceAccountSyncScheduleKind = DeviceAccountSyncScheduleKind(6i32);
}
impl ::std::convert::From<i32> for DeviceAccountSyncScheduleKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DeviceAccountSyncScheduleKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DeviceAccountSyncScheduleKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataAccounts.SystemAccess.DeviceAccountSyncScheduleKind;i4)");
}
impl ::windows::runtime::DefaultType for DeviceAccountSyncScheduleKind {
    type DefaultType = Self;
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeviceAccountConfiguration(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDeviceAccountConfiguration {
    type Vtable = IDeviceAccountConfiguration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2902533027, 64476, 19739, [190, 67, 90, 39, 234, 74, 27, 99]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceAccountConfiguration_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut DeviceAccountServerType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: DeviceAccountServerType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeviceAccountConfiguration2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDeviceAccountConfiguration2 {
    type Vtable = IDeviceAccountConfiguration2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4071810470, 29325, 19018, [137, 69, 43, 248, 88, 1, 54, 222]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeviceAccountConfiguration2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut DeviceAccountIconId) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: DeviceAccountIconId) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut DeviceAccountAuthenticationType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: DeviceAccountAuthenticationType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut DeviceAccountSyncScheduleKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: DeviceAccountSyncScheduleKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut DeviceAccountMailAgeFilter) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: DeviceAccountMailAgeFilter) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut DeviceAccountSyncScheduleKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: DeviceAccountSyncScheduleKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut DeviceAccountSyncScheduleKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: DeviceAccountSyncScheduleKind) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserDataAccountSystemAccessManagerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserDataAccountSystemAccessManagerStatics {
    type Vtable = IUserDataAccountSystemAccessManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2641039801, 52197, 17909, [130, 43, 194, 103, 184, 29, 189, 182]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountSystemAccessManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, accounts: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserDataAccountSystemAccessManagerStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserDataAccountSystemAccessManagerStatics2 {
    type Vtable = IUserDataAccountSystemAccessManagerStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2487190861, 19278, 17311, [131, 211, 151, 155, 39, 192, 90, 199]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataAccountSystemAccessManagerStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, userdataaccountid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, account: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, accountid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, accountid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`*"]
pub struct UserDataAccountSystemAccessManager {}
impl UserDataAccountSystemAccessManager {
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`, `Foundation`, `Foundation_Collections`*"]
    pub fn AddAndShowDeviceAccountsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<DeviceAccountConfiguration>>>(accounts: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>> {
        Self::IUserDataAccountSystemAccessManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), accounts.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`, `Foundation`*"]
    pub fn SuppressLocalAccountWithAccountAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(userdataaccountid: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        Self::IUserDataAccountSystemAccessManagerStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), userdataaccountid.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`, `Foundation`*"]
    pub fn CreateDeviceAccountAsync<'a, Param0: ::windows::runtime::IntoParam<'a, DeviceAccountConfiguration>>(account: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>> {
        Self::IUserDataAccountSystemAccessManagerStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), account.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`, `Foundation`*"]
    pub fn DeleteDeviceAccountAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(accountid: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        Self::IUserDataAccountSystemAccessManagerStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), accountid.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataAccounts_SystemAccess`, `Foundation`*"]
    pub fn GetDeviceAccountConfigurationAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(accountid: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<DeviceAccountConfiguration>> {
        Self::IUserDataAccountSystemAccessManagerStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), accountid.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<DeviceAccountConfiguration>>(result__)
        })
    }
    pub fn IUserDataAccountSystemAccessManagerStatics<R, F: FnOnce(&IUserDataAccountSystemAccessManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<UserDataAccountSystemAccessManager, IUserDataAccountSystemAccessManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IUserDataAccountSystemAccessManagerStatics2<R, F: FnOnce(&IUserDataAccountSystemAccessManagerStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<UserDataAccountSystemAccessManager, IUserDataAccountSystemAccessManagerStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for UserDataAccountSystemAccessManager {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataAccounts.SystemAccess.UserDataAccountSystemAccessManager";
}

#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Security_Authentication_OnlineId`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CredentialPromptType(pub i32);
impl CredentialPromptType {
    pub const PromptIfNeeded: CredentialPromptType = CredentialPromptType(0i32);
    pub const RetypeCredentials: CredentialPromptType = CredentialPromptType(1i32);
    pub const DoNotPrompt: CredentialPromptType = CredentialPromptType(2i32);
}
impl ::core::convert::From<i32> for CredentialPromptType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CredentialPromptType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CredentialPromptType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.OnlineId.CredentialPromptType;i4)");
}
impl ::windows::core::DefaultType for CredentialPromptType {
    type DefaultType = Self;
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IOnlineIdAuthenticator(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IOnlineIdAuthenticator {
    type Vtable = IOnlineIdAuthenticator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa003f58a_29ab_4817_b884_d7516dad18b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOnlineIdAuthenticator_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, request: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, requests: ::windows::core::RawPtr, credentialprompttype: CredentialPromptType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IOnlineIdServiceTicket(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IOnlineIdServiceTicket {
    type Vtable = IOnlineIdServiceTicket_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc95c547f_d781_4a94_acb8_c59874238c26);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOnlineIdServiceTicket_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IOnlineIdServiceTicketRequest(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IOnlineIdServiceTicketRequest {
    type Vtable = IOnlineIdServiceTicketRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x297445d3_fb63_4135_8909_4e354c061466);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOnlineIdServiceTicketRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IOnlineIdServiceTicketRequestFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IOnlineIdServiceTicketRequestFactory {
    type Vtable = IOnlineIdServiceTicketRequestFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbebb0a08_9e73_4077_9614_08614c0bc245);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOnlineIdServiceTicketRequestFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, service: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, policy: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, service: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IOnlineIdSystemAuthenticatorForUser(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IOnlineIdSystemAuthenticatorForUser {
    type Vtable = IOnlineIdSystemAuthenticatorForUser_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5798befb_1de4_4186_a2e6_b563f86aaf44);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOnlineIdSystemAuthenticatorForUser_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, request: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IOnlineIdSystemAuthenticatorStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IOnlineIdSystemAuthenticatorStatics {
    type Vtable = IOnlineIdSystemAuthenticatorStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x85047792_f634_41e3_96a4_5164e902c740);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOnlineIdSystemAuthenticatorStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IOnlineIdSystemIdentity(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IOnlineIdSystemIdentity {
    type Vtable = IOnlineIdSystemIdentity_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x743cd20d_b6ca_434d_8124_53ea12685307);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOnlineIdSystemIdentity_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IOnlineIdSystemTicketResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IOnlineIdSystemTicketResult {
    type Vtable = IOnlineIdSystemTicketResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb0a5ff8_b098_4acd_9d13_9e640652b5b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOnlineIdSystemTicketResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut OnlineIdSystemTicketStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserIdentity(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUserIdentity {
    type Vtable = IUserIdentity_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2146d9cd_0742_4be3_8a1c_7c7ae679aa88);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserIdentity_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Security_Authentication_OnlineId`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct OnlineIdAuthenticator(pub ::windows::core::IInspectable);
impl OnlineIdAuthenticator {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<OnlineIdAuthenticator, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_OnlineId`, `Foundation`*"]
    pub fn AuthenticateUserAsync<'a, Param0: ::windows::core::IntoParam<'a, OnlineIdServiceTicketRequest>>(&self, request: Param0) -> ::windows::core::Result<UserAuthenticationOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), request.into_param().abi(), &mut result__).from_abi::<UserAuthenticationOperation>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Security_Authentication_OnlineId`, `Foundation`, `Foundation_Collections`*"]
    pub fn AuthenticateUserAsyncAdvanced<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<OnlineIdServiceTicketRequest>>>(&self, requests: Param0, credentialprompttype: CredentialPromptType) -> ::windows::core::Result<UserAuthenticationOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), requests.into_param().abi(), credentialprompttype, &mut result__).from_abi::<UserAuthenticationOperation>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_OnlineId`, `Foundation`*"]
    pub fn SignOutUserAsync(&self) -> ::windows::core::Result<SignOutUserOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SignOutUserOperation>(result__)
        }
    }
    #[doc = "*Required features: `Security_Authentication_OnlineId`*"]
    pub fn SetApplicationId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Security_Authentication_OnlineId`*"]
    pub fn ApplicationId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Security_Authentication_OnlineId`*"]
    pub fn CanSignOut(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Security_Authentication_OnlineId`*"]
    pub fn AuthenticatedSafeCustomerId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for OnlineIdAuthenticator {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.OnlineId.OnlineIdAuthenticator;{a003f58a-29ab-4817-b884-d7516dad18b9})");
}
unsafe impl ::windows::core::Interface for OnlineIdAuthenticator {
    type Vtable = IOnlineIdAuthenticator_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa003f58a_29ab_4817_b884_d7516dad18b9);
}
impl ::windows::core::RuntimeName for OnlineIdAuthenticator {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.OnlineIdAuthenticator";
}
impl ::core::convert::From<OnlineIdAuthenticator> for ::windows::core::IUnknown {
    fn from(value: OnlineIdAuthenticator) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&OnlineIdAuthenticator> for ::windows::core::IUnknown {
    fn from(value: &OnlineIdAuthenticator) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for OnlineIdAuthenticator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a OnlineIdAuthenticator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<OnlineIdAuthenticator> for ::windows::core::IInspectable {
    fn from(value: OnlineIdAuthenticator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&OnlineIdAuthenticator> for ::windows::core::IInspectable {
    fn from(value: &OnlineIdAuthenticator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for OnlineIdAuthenticator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a OnlineIdAuthenticator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for OnlineIdAuthenticator {}
unsafe impl ::core::marker::Sync for OnlineIdAuthenticator {}
#[doc = "*Required features: `Security_Authentication_OnlineId`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct OnlineIdServiceTicket(pub ::windows::core::IInspectable);
impl OnlineIdServiceTicket {
    #[doc = "*Required features: `Security_Authentication_OnlineId`*"]
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_Authentication_OnlineId`*"]
    pub fn Request(&self) -> ::windows::core::Result<OnlineIdServiceTicketRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<OnlineIdServiceTicketRequest>(result__)
        }
    }
    #[doc = "*Required features: `Security_Authentication_OnlineId`*"]
    pub fn ErrorCode(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for OnlineIdServiceTicket {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.OnlineId.OnlineIdServiceTicket;{c95c547f-d781-4a94-acb8-c59874238c26})");
}
unsafe impl ::windows::core::Interface for OnlineIdServiceTicket {
    type Vtable = IOnlineIdServiceTicket_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc95c547f_d781_4a94_acb8_c59874238c26);
}
impl ::windows::core::RuntimeName for OnlineIdServiceTicket {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.OnlineIdServiceTicket";
}
impl ::core::convert::From<OnlineIdServiceTicket> for ::windows::core::IUnknown {
    fn from(value: OnlineIdServiceTicket) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&OnlineIdServiceTicket> for ::windows::core::IUnknown {
    fn from(value: &OnlineIdServiceTicket) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for OnlineIdServiceTicket {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a OnlineIdServiceTicket {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<OnlineIdServiceTicket> for ::windows::core::IInspectable {
    fn from(value: OnlineIdServiceTicket) -> Self {
        value.0
    }
}
impl ::core::convert::From<&OnlineIdServiceTicket> for ::windows::core::IInspectable {
    fn from(value: &OnlineIdServiceTicket) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for OnlineIdServiceTicket {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a OnlineIdServiceTicket {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for OnlineIdServiceTicket {}
unsafe impl ::core::marker::Sync for OnlineIdServiceTicket {}
#[doc = "*Required features: `Security_Authentication_OnlineId`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct OnlineIdServiceTicketRequest(pub ::windows::core::IInspectable);
impl OnlineIdServiceTicketRequest {
    #[doc = "*Required features: `Security_Authentication_OnlineId`*"]
    pub fn Service(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_Authentication_OnlineId`*"]
    pub fn Policy(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_Authentication_OnlineId`*"]
    pub fn CreateOnlineIdServiceTicketRequest<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(service: Param0, policy: Param1) -> ::windows::core::Result<OnlineIdServiceTicketRequest> {
        Self::IOnlineIdServiceTicketRequestFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), service.into_param().abi(), policy.into_param().abi(), &mut result__).from_abi::<OnlineIdServiceTicketRequest>(result__)
        })
    }
    #[doc = "*Required features: `Security_Authentication_OnlineId`*"]
    pub fn CreateOnlineIdServiceTicketRequestAdvanced<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(service: Param0) -> ::windows::core::Result<OnlineIdServiceTicketRequest> {
        Self::IOnlineIdServiceTicketRequestFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), service.into_param().abi(), &mut result__).from_abi::<OnlineIdServiceTicketRequest>(result__)
        })
    }
    pub fn IOnlineIdServiceTicketRequestFactory<R, F: FnOnce(&IOnlineIdServiceTicketRequestFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<OnlineIdServiceTicketRequest, IOnlineIdServiceTicketRequestFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for OnlineIdServiceTicketRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.OnlineId.OnlineIdServiceTicketRequest;{297445d3-fb63-4135-8909-4e354c061466})");
}
unsafe impl ::windows::core::Interface for OnlineIdServiceTicketRequest {
    type Vtable = IOnlineIdServiceTicketRequest_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x297445d3_fb63_4135_8909_4e354c061466);
}
impl ::windows::core::RuntimeName for OnlineIdServiceTicketRequest {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.OnlineIdServiceTicketRequest";
}
impl ::core::convert::From<OnlineIdServiceTicketRequest> for ::windows::core::IUnknown {
    fn from(value: OnlineIdServiceTicketRequest) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&OnlineIdServiceTicketRequest> for ::windows::core::IUnknown {
    fn from(value: &OnlineIdServiceTicketRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for OnlineIdServiceTicketRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a OnlineIdServiceTicketRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<OnlineIdServiceTicketRequest> for ::windows::core::IInspectable {
    fn from(value: OnlineIdServiceTicketRequest) -> Self {
        value.0
    }
}
impl ::core::convert::From<&OnlineIdServiceTicketRequest> for ::windows::core::IInspectable {
    fn from(value: &OnlineIdServiceTicketRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for OnlineIdServiceTicketRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a OnlineIdServiceTicketRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for OnlineIdServiceTicketRequest {}
unsafe impl ::core::marker::Sync for OnlineIdServiceTicketRequest {}
#[doc = "*Required features: `Security_Authentication_OnlineId`*"]
pub struct OnlineIdSystemAuthenticator {}
impl OnlineIdSystemAuthenticator {
    #[doc = "*Required features: `Security_Authentication_OnlineId`*"]
    pub fn Default() -> ::windows::core::Result<OnlineIdSystemAuthenticatorForUser> {
        Self::IOnlineIdSystemAuthenticatorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<OnlineIdSystemAuthenticatorForUser>(result__)
        })
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `Security_Authentication_OnlineId`, `System`*"]
    pub fn GetForUser<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::User>>(user: Param0) -> ::windows::core::Result<OnlineIdSystemAuthenticatorForUser> {
        Self::IOnlineIdSystemAuthenticatorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<OnlineIdSystemAuthenticatorForUser>(result__)
        })
    }
    pub fn IOnlineIdSystemAuthenticatorStatics<R, F: FnOnce(&IOnlineIdSystemAuthenticatorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<OnlineIdSystemAuthenticator, IOnlineIdSystemAuthenticatorStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for OnlineIdSystemAuthenticator {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.OnlineIdSystemAuthenticator";
}
#[doc = "*Required features: `Security_Authentication_OnlineId`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct OnlineIdSystemAuthenticatorForUser(pub ::windows::core::IInspectable);
impl OnlineIdSystemAuthenticatorForUser {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_OnlineId`, `Foundation`*"]
    pub fn GetTicketAsync<'a, Param0: ::windows::core::IntoParam<'a, OnlineIdServiceTicketRequest>>(&self, request: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<OnlineIdSystemTicketResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), request.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<OnlineIdSystemTicketResult>>(result__)
        }
    }
    #[doc = "*Required features: `Security_Authentication_OnlineId`*"]
    pub fn SetApplicationId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Security_Authentication_OnlineId`*"]
    pub fn ApplicationId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `Security_Authentication_OnlineId`, `System`*"]
    pub fn User(&self) -> ::windows::core::Result<super::super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::System::User>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for OnlineIdSystemAuthenticatorForUser {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.OnlineId.OnlineIdSystemAuthenticatorForUser;{5798befb-1de4-4186-a2e6-b563f86aaf44})");
}
unsafe impl ::windows::core::Interface for OnlineIdSystemAuthenticatorForUser {
    type Vtable = IOnlineIdSystemAuthenticatorForUser_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5798befb_1de4_4186_a2e6_b563f86aaf44);
}
impl ::windows::core::RuntimeName for OnlineIdSystemAuthenticatorForUser {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.OnlineIdSystemAuthenticatorForUser";
}
impl ::core::convert::From<OnlineIdSystemAuthenticatorForUser> for ::windows::core::IUnknown {
    fn from(value: OnlineIdSystemAuthenticatorForUser) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&OnlineIdSystemAuthenticatorForUser> for ::windows::core::IUnknown {
    fn from(value: &OnlineIdSystemAuthenticatorForUser) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for OnlineIdSystemAuthenticatorForUser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a OnlineIdSystemAuthenticatorForUser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<OnlineIdSystemAuthenticatorForUser> for ::windows::core::IInspectable {
    fn from(value: OnlineIdSystemAuthenticatorForUser) -> Self {
        value.0
    }
}
impl ::core::convert::From<&OnlineIdSystemAuthenticatorForUser> for ::windows::core::IInspectable {
    fn from(value: &OnlineIdSystemAuthenticatorForUser) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for OnlineIdSystemAuthenticatorForUser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a OnlineIdSystemAuthenticatorForUser {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for OnlineIdSystemAuthenticatorForUser {}
unsafe impl ::core::marker::Sync for OnlineIdSystemAuthenticatorForUser {}
#[doc = "*Required features: `Security_Authentication_OnlineId`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct OnlineIdSystemIdentity(pub ::windows::core::IInspectable);
impl OnlineIdSystemIdentity {
    #[doc = "*Required features: `Security_Authentication_OnlineId`*"]
    pub fn Ticket(&self) -> ::windows::core::Result<OnlineIdServiceTicket> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<OnlineIdServiceTicket>(result__)
        }
    }
    #[doc = "*Required features: `Security_Authentication_OnlineId`*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for OnlineIdSystemIdentity {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.OnlineId.OnlineIdSystemIdentity;{743cd20d-b6ca-434d-8124-53ea12685307})");
}
unsafe impl ::windows::core::Interface for OnlineIdSystemIdentity {
    type Vtable = IOnlineIdSystemIdentity_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x743cd20d_b6ca_434d_8124_53ea12685307);
}
impl ::windows::core::RuntimeName for OnlineIdSystemIdentity {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.OnlineIdSystemIdentity";
}
impl ::core::convert::From<OnlineIdSystemIdentity> for ::windows::core::IUnknown {
    fn from(value: OnlineIdSystemIdentity) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&OnlineIdSystemIdentity> for ::windows::core::IUnknown {
    fn from(value: &OnlineIdSystemIdentity) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for OnlineIdSystemIdentity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a OnlineIdSystemIdentity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<OnlineIdSystemIdentity> for ::windows::core::IInspectable {
    fn from(value: OnlineIdSystemIdentity) -> Self {
        value.0
    }
}
impl ::core::convert::From<&OnlineIdSystemIdentity> for ::windows::core::IInspectable {
    fn from(value: &OnlineIdSystemIdentity) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for OnlineIdSystemIdentity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a OnlineIdSystemIdentity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for OnlineIdSystemIdentity {}
unsafe impl ::core::marker::Sync for OnlineIdSystemIdentity {}
#[doc = "*Required features: `Security_Authentication_OnlineId`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct OnlineIdSystemTicketResult(pub ::windows::core::IInspectable);
impl OnlineIdSystemTicketResult {
    #[doc = "*Required features: `Security_Authentication_OnlineId`*"]
    pub fn Identity(&self) -> ::windows::core::Result<OnlineIdSystemIdentity> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<OnlineIdSystemIdentity>(result__)
        }
    }
    #[doc = "*Required features: `Security_Authentication_OnlineId`*"]
    pub fn Status(&self) -> ::windows::core::Result<OnlineIdSystemTicketStatus> {
        let this = self;
        unsafe {
            let mut result__: OnlineIdSystemTicketStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<OnlineIdSystemTicketStatus>(result__)
        }
    }
    #[doc = "*Required features: `Security_Authentication_OnlineId`*"]
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for OnlineIdSystemTicketResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.OnlineId.OnlineIdSystemTicketResult;{db0a5ff8-b098-4acd-9d13-9e640652b5b6})");
}
unsafe impl ::windows::core::Interface for OnlineIdSystemTicketResult {
    type Vtable = IOnlineIdSystemTicketResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb0a5ff8_b098_4acd_9d13_9e640652b5b6);
}
impl ::windows::core::RuntimeName for OnlineIdSystemTicketResult {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.OnlineIdSystemTicketResult";
}
impl ::core::convert::From<OnlineIdSystemTicketResult> for ::windows::core::IUnknown {
    fn from(value: OnlineIdSystemTicketResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&OnlineIdSystemTicketResult> for ::windows::core::IUnknown {
    fn from(value: &OnlineIdSystemTicketResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for OnlineIdSystemTicketResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a OnlineIdSystemTicketResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<OnlineIdSystemTicketResult> for ::windows::core::IInspectable {
    fn from(value: OnlineIdSystemTicketResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&OnlineIdSystemTicketResult> for ::windows::core::IInspectable {
    fn from(value: &OnlineIdSystemTicketResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for OnlineIdSystemTicketResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a OnlineIdSystemTicketResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for OnlineIdSystemTicketResult {}
unsafe impl ::core::marker::Sync for OnlineIdSystemTicketResult {}
#[doc = "*Required features: `Security_Authentication_OnlineId`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct OnlineIdSystemTicketStatus(pub i32);
impl OnlineIdSystemTicketStatus {
    pub const Success: OnlineIdSystemTicketStatus = OnlineIdSystemTicketStatus(0i32);
    pub const Error: OnlineIdSystemTicketStatus = OnlineIdSystemTicketStatus(1i32);
    pub const ServiceConnectionError: OnlineIdSystemTicketStatus = OnlineIdSystemTicketStatus(2i32);
}
impl ::core::convert::From<i32> for OnlineIdSystemTicketStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for OnlineIdSystemTicketStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for OnlineIdSystemTicketStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.OnlineId.OnlineIdSystemTicketStatus;i4)");
}
impl ::windows::core::DefaultType for OnlineIdSystemTicketStatus {
    type DefaultType = Self;
}
#[cfg(feature = "Foundation")]
#[doc = "*Required features: `Security_Authentication_OnlineId`, `Foundation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SignOutUserOperation(pub ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl SignOutUserOperation {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_OnlineId`, `Foundation`*"]
    pub fn SetCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::AsyncActionCompletedHandler>>(&self, handler: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_OnlineId`, `Foundation`*"]
    pub fn Completed(&self) -> ::windows::core::Result<super::super::super::Foundation::AsyncActionCompletedHandler> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::AsyncActionCompletedHandler>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_OnlineId`, `Foundation`*"]
    pub fn GetResults(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_OnlineId`, `Foundation`*"]
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_OnlineId`, `Foundation`*"]
    pub fn Status(&self) -> ::windows::core::Result<super::super::super::Foundation::AsyncStatus> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::AsyncStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::AsyncStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_OnlineId`, `Foundation`*"]
    pub fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_OnlineId`, `Foundation`*"]
    pub fn Cancel(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_OnlineId`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn get(&self) -> ::windows::core::Result<()> {
        if self.Status()? == super::super::super::Foundation::AsyncStatus::Started {
            let (waiter, signaler) = ::windows::core::Waiter::new();
            self.SetCompleted(super::super::super::Foundation::AsyncActionCompletedHandler::new(move |_sender, _args| {
                unsafe {
                    signaler.signal();
                }
                Ok(())
            }))?;
        }
        self.GetResults()
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::RuntimeType for SignOutUserOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.OnlineId.SignOutUserOperation;{5a648006-843a-4da9-865b-9d26e5dfad7b})");
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::Interface for SignOutUserOperation {
    type Vtable = super::super::super::Foundation::IAsyncAction_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5a648006_843a_4da9_865b_9d26e5dfad7b);
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for SignOutUserOperation {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.SignOutUserOperation";
}
#[cfg(feature = "Foundation")]
#[cfg(feature = "std")]
impl ::std::future::Future for SignOutUserOperation {
    type Output = ::windows::core::Result<()>;
    fn poll(self: ::std::pin::Pin<&mut Self>, context: &mut ::std::task::Context) -> ::std::task::Poll<Self::Output> {
        if self.Status()? == super::super::super::Foundation::AsyncStatus::Started {
            let waker = context.waker().clone();
            let _ = self.SetCompleted(super::super::super::Foundation::AsyncActionCompletedHandler::new(move |_sender, _args| {
                waker.wake_by_ref();
                Ok(())
            }));
            ::std::task::Poll::Pending
        } else {
            ::std::task::Poll::Ready(self.GetResults())
        }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::From<SignOutUserOperation> for ::windows::core::IUnknown {
    fn from(value: SignOutUserOperation) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::From<&SignOutUserOperation> for ::windows::core::IUnknown {
    fn from(value: &SignOutUserOperation) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SignOutUserOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SignOutUserOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::From<SignOutUserOperation> for ::windows::core::IInspectable {
    fn from(value: SignOutUserOperation) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::From<&SignOutUserOperation> for ::windows::core::IInspectable {
    fn from(value: &SignOutUserOperation) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SignOutUserOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SignOutUserOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::From<SignOutUserOperation> for super::super::super::Foundation::IAsyncAction {
    fn from(value: SignOutUserOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::From<&SignOutUserOperation> for super::super::super::Foundation::IAsyncAction {
    fn from(value: &SignOutUserOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IAsyncAction> for SignOutUserOperation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IAsyncAction> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IAsyncAction> for &SignOutUserOperation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IAsyncAction> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<SignOutUserOperation> for super::super::super::Foundation::IAsyncInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: SignOutUserOperation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&SignOutUserOperation> for super::super::super::Foundation::IAsyncInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: &SignOutUserOperation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IAsyncInfo> for SignOutUserOperation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IAsyncInfo> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IAsyncInfo> for &SignOutUserOperation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IAsyncInfo> {
        ::core::convert::TryInto::<super::super::super::Foundation::IAsyncInfo>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::core::marker::Send for SignOutUserOperation {}
#[cfg(feature = "Foundation")]
unsafe impl ::core::marker::Sync for SignOutUserOperation {}
#[cfg(feature = "Foundation")]
#[doc = "*Required features: `Security_Authentication_OnlineId`, `Foundation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct UserAuthenticationOperation(pub ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl UserAuthenticationOperation {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_OnlineId`, `Foundation`*"]
    pub fn SetCompleted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::AsyncOperationCompletedHandler<UserIdentity>>>(&self, handler: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_OnlineId`, `Foundation`*"]
    pub fn Completed(&self) -> ::windows::core::Result<super::super::super::Foundation::AsyncOperationCompletedHandler<UserIdentity>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::AsyncOperationCompletedHandler<UserIdentity>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_OnlineId`, `Foundation`*"]
    pub fn GetResults(&self) -> ::windows::core::Result<UserIdentity> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserIdentity>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_OnlineId`, `Foundation`*"]
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_OnlineId`, `Foundation`*"]
    pub fn Status(&self) -> ::windows::core::Result<super::super::super::Foundation::AsyncStatus> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::AsyncStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::AsyncStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_OnlineId`, `Foundation`*"]
    pub fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_OnlineId`, `Foundation`*"]
    pub fn Cancel(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_OnlineId`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn get(&self) -> ::windows::core::Result<UserIdentity> {
        if self.Status()? == super::super::super::Foundation::AsyncStatus::Started {
            let (waiter, signaler) = ::windows::core::Waiter::new();
            self.SetCompleted(super::super::super::Foundation::AsyncOperationCompletedHandler::new(move |_sender, _args| {
                unsafe {
                    signaler.signal();
                }
                Ok(())
            }))?;
        }
        self.GetResults()
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::RuntimeType for UserAuthenticationOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.OnlineId.UserAuthenticationOperation;pinterface({9fc2b0bb-e446-44e2-aa61-9cab8f636af2};rc(Windows.Security.Authentication.OnlineId.UserIdentity;{2146d9cd-0742-4be3-8a1c-7c7ae679aa88})))");
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::Interface for UserAuthenticationOperation {
    type Vtable = super::super::super::Foundation::IAsyncOperation_abi<UserIdentity>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<super::super::super::Foundation::IAsyncOperation<UserIdentity> as ::windows::core::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for UserAuthenticationOperation {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.UserAuthenticationOperation";
}
#[cfg(feature = "Foundation")]
#[cfg(feature = "std")]
impl ::std::future::Future for UserAuthenticationOperation {
    type Output = ::windows::core::Result<UserIdentity>;
    fn poll(self: ::std::pin::Pin<&mut Self>, context: &mut ::std::task::Context) -> ::std::task::Poll<Self::Output> {
        if self.Status()? == super::super::super::Foundation::AsyncStatus::Started {
            let waker = context.waker().clone();
            let _ = self.SetCompleted(super::super::super::Foundation::AsyncOperationCompletedHandler::new(move |_sender, _args| {
                waker.wake_by_ref();
                Ok(())
            }));
            ::std::task::Poll::Pending
        } else {
            ::std::task::Poll::Ready(self.GetResults())
        }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::From<UserAuthenticationOperation> for ::windows::core::IUnknown {
    fn from(value: UserAuthenticationOperation) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::From<&UserAuthenticationOperation> for ::windows::core::IUnknown {
    fn from(value: &UserAuthenticationOperation) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserAuthenticationOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserAuthenticationOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::From<UserAuthenticationOperation> for ::windows::core::IInspectable {
    fn from(value: UserAuthenticationOperation) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::From<&UserAuthenticationOperation> for ::windows::core::IInspectable {
    fn from(value: &UserAuthenticationOperation) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserAuthenticationOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserAuthenticationOperation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::From<UserAuthenticationOperation> for super::super::super::Foundation::IAsyncOperation<UserIdentity> {
    fn from(value: UserAuthenticationOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::From<&UserAuthenticationOperation> for super::super::super::Foundation::IAsyncOperation<UserIdentity> {
    fn from(value: &UserAuthenticationOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IAsyncOperation<UserIdentity>> for UserAuthenticationOperation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IAsyncOperation<UserIdentity>> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IAsyncOperation<UserIdentity>> for &UserAuthenticationOperation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IAsyncOperation<UserIdentity>> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<UserAuthenticationOperation> for super::super::super::Foundation::IAsyncInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: UserAuthenticationOperation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&UserAuthenticationOperation> for super::super::super::Foundation::IAsyncInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: &UserAuthenticationOperation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IAsyncInfo> for UserAuthenticationOperation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IAsyncInfo> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IAsyncInfo> for &UserAuthenticationOperation {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IAsyncInfo> {
        ::core::convert::TryInto::<super::super::super::Foundation::IAsyncInfo>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::core::marker::Send for UserAuthenticationOperation {}
#[cfg(feature = "Foundation")]
unsafe impl ::core::marker::Sync for UserAuthenticationOperation {}
#[doc = "*Required features: `Security_Authentication_OnlineId`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct UserIdentity(pub ::windows::core::IInspectable);
impl UserIdentity {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Security_Authentication_OnlineId`, `Foundation_Collections`*"]
    pub fn Tickets(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<OnlineIdServiceTicket>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<OnlineIdServiceTicket>>(result__)
        }
    }
    #[doc = "*Required features: `Security_Authentication_OnlineId`*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_Authentication_OnlineId`*"]
    pub fn SafeCustomerId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_Authentication_OnlineId`*"]
    pub fn SignInName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_Authentication_OnlineId`*"]
    pub fn FirstName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_Authentication_OnlineId`*"]
    pub fn LastName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_Authentication_OnlineId`*"]
    pub fn IsBetaAccount(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Security_Authentication_OnlineId`*"]
    pub fn IsConfirmedPC(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for UserIdentity {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.OnlineId.UserIdentity;{2146d9cd-0742-4be3-8a1c-7c7ae679aa88})");
}
unsafe impl ::windows::core::Interface for UserIdentity {
    type Vtable = IUserIdentity_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2146d9cd_0742_4be3_8a1c_7c7ae679aa88);
}
impl ::windows::core::RuntimeName for UserIdentity {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.UserIdentity";
}
impl ::core::convert::From<UserIdentity> for ::windows::core::IUnknown {
    fn from(value: UserIdentity) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&UserIdentity> for ::windows::core::IUnknown {
    fn from(value: &UserIdentity) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserIdentity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserIdentity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<UserIdentity> for ::windows::core::IInspectable {
    fn from(value: UserIdentity) -> Self {
        value.0
    }
}
impl ::core::convert::From<&UserIdentity> for ::windows::core::IInspectable {
    fn from(value: &UserIdentity) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserIdentity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserIdentity {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for UserIdentity {}
unsafe impl ::core::marker::Sync for UserIdentity {}

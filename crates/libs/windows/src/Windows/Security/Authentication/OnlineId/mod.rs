#[doc(hidden)]
#[repr(transparent)]
pub struct IOnlineIdAuthenticator(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOnlineIdAuthenticator {
    type Vtable = IOnlineIdAuthenticator_Vtbl;
}
impl ::core::clone::Clone for IOnlineIdAuthenticator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOnlineIdAuthenticator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa003f58a_29ab_4817_b884_d7516dad18b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOnlineIdAuthenticator_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub AuthenticateUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AuthenticateUserAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub AuthenticateUserAsyncAdvanced: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requests: *mut ::core::ffi::c_void, credentialprompttype: CredentialPromptType, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AuthenticateUserAsyncAdvanced: usize,
    #[cfg(feature = "Foundation")]
    pub SignOutUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SignOutUserAsync: usize,
    pub SetApplicationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub ApplicationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub CanSignOut: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub AuthenticatedSafeCustomerId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOnlineIdServiceTicket(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOnlineIdServiceTicket {
    type Vtable = IOnlineIdServiceTicket_Vtbl;
}
impl ::core::clone::Clone for IOnlineIdServiceTicket {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOnlineIdServiceTicket {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc95c547f_d781_4a94_acb8_c59874238c26);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOnlineIdServiceTicket_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOnlineIdServiceTicketRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOnlineIdServiceTicketRequest {
    type Vtable = IOnlineIdServiceTicketRequest_Vtbl;
}
impl ::core::clone::Clone for IOnlineIdServiceTicketRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOnlineIdServiceTicketRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x297445d3_fb63_4135_8909_4e354c061466);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOnlineIdServiceTicketRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Service: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Policy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOnlineIdServiceTicketRequestFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOnlineIdServiceTicketRequestFactory {
    type Vtable = IOnlineIdServiceTicketRequestFactory_Vtbl;
}
impl ::core::clone::Clone for IOnlineIdServiceTicketRequestFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOnlineIdServiceTicketRequestFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbebb0a08_9e73_4077_9614_08614c0bc245);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOnlineIdServiceTicketRequestFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateOnlineIdServiceTicketRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, service: ::std::mem::MaybeUninit<::windows::core::HSTRING>, policy: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateOnlineIdServiceTicketRequestAdvanced: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, service: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOnlineIdSystemAuthenticatorForUser(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOnlineIdSystemAuthenticatorForUser {
    type Vtable = IOnlineIdSystemAuthenticatorForUser_Vtbl;
}
impl ::core::clone::Clone for IOnlineIdSystemAuthenticatorForUser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOnlineIdSystemAuthenticatorForUser {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5798befb_1de4_4186_a2e6_b563f86aaf44);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOnlineIdSystemAuthenticatorForUser_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetTicketAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, request: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetTicketAsync: usize,
    pub SetApplicationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub ApplicationId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")]
    pub User: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    User: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOnlineIdSystemAuthenticatorStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOnlineIdSystemAuthenticatorStatics {
    type Vtable = IOnlineIdSystemAuthenticatorStatics_Vtbl;
}
impl ::core::clone::Clone for IOnlineIdSystemAuthenticatorStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOnlineIdSystemAuthenticatorStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x85047792_f634_41e3_96a4_5164e902c740);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOnlineIdSystemAuthenticatorStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Default: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")]
    pub GetForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetForUser: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOnlineIdSystemIdentity(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOnlineIdSystemIdentity {
    type Vtable = IOnlineIdSystemIdentity_Vtbl;
}
impl ::core::clone::Clone for IOnlineIdSystemIdentity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOnlineIdSystemIdentity {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x743cd20d_b6ca_434d_8124_53ea12685307);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOnlineIdSystemIdentity_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Ticket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IOnlineIdSystemTicketResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IOnlineIdSystemTicketResult {
    type Vtable = IOnlineIdSystemTicketResult_Vtbl;
}
impl ::core::clone::Clone for IOnlineIdSystemTicketResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IOnlineIdSystemTicketResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb0a5ff8_b098_4acd_9d13_9e640652b5b6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOnlineIdSystemTicketResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Identity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut OnlineIdSystemTicketStatus) -> ::windows::core::HRESULT,
    pub ExtendedError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserIdentity(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUserIdentity {
    type Vtable = IUserIdentity_Vtbl;
}
impl ::core::clone::Clone for IUserIdentity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IUserIdentity {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2146d9cd_0742_4be3_8a1c_7c7ae679aa88);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserIdentity_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Tickets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Tickets: usize,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SafeCustomerId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SignInName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub FirstName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub LastName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsBetaAccount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsConfirmedPC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Security_Authentication_OnlineId\"`*"]
#[repr(transparent)]
pub struct OnlineIdAuthenticator(::windows::core::IUnknown);
impl OnlineIdAuthenticator {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<OnlineIdAuthenticator, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AuthenticateUserAsync(&self, request: &OnlineIdServiceTicketRequest) -> ::windows::core::Result<UserAuthenticationOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<UserAuthenticationOperation>();
            (::windows::core::Interface::vtable(this).AuthenticateUserAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(request), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AuthenticateUserAsyncAdvanced<P0>(&self, requests: P0, credentialprompttype: CredentialPromptType) -> ::windows::core::Result<UserAuthenticationOperation>
    where
        P0: ::windows::core::TryIntoParam<super::super::super::Foundation::Collections::IIterable<OnlineIdServiceTicketRequest>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<UserAuthenticationOperation>();
            (::windows::core::Interface::vtable(this).AuthenticateUserAsyncAdvanced)(::windows::core::Interface::as_raw(this), requests.try_into_param()?.abi(), credentialprompttype, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SignOutUserAsync(&self) -> ::windows::core::Result<SignOutUserOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<SignOutUserOperation>();
            (::windows::core::Interface::vtable(this).SignOutUserAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetApplicationId(&self, value: ::windows::core::GUID) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetApplicationId)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ApplicationId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).ApplicationId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CanSignOut(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).CanSignOut)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AuthenticatedSafeCustomerId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).AuthenticatedSafeCustomerId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for OnlineIdAuthenticator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OnlineIdAuthenticator {}
impl ::core::fmt::Debug for OnlineIdAuthenticator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OnlineIdAuthenticator").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for OnlineIdAuthenticator {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.OnlineId.OnlineIdAuthenticator;{a003f58a-29ab-4817-b884-d7516dad18b9})");
}
impl ::core::clone::Clone for OnlineIdAuthenticator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for OnlineIdAuthenticator {
    type Vtable = IOnlineIdAuthenticator_Vtbl;
}
unsafe impl ::windows::core::ComInterface for OnlineIdAuthenticator {
    const IID: ::windows::core::GUID = <IOnlineIdAuthenticator as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for OnlineIdAuthenticator {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.OnlineIdAuthenticator";
}
::windows::imp::interface_hierarchy!(OnlineIdAuthenticator, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for OnlineIdAuthenticator {}
unsafe impl ::core::marker::Sync for OnlineIdAuthenticator {}
#[doc = "*Required features: `\"Security_Authentication_OnlineId\"`*"]
#[repr(transparent)]
pub struct OnlineIdServiceTicket(::windows::core::IUnknown);
impl OnlineIdServiceTicket {
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Value)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Request(&self) -> ::windows::core::Result<OnlineIdServiceTicketRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<OnlineIdServiceTicketRequest>();
            (::windows::core::Interface::vtable(this).Request)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ErrorCode(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).ErrorCode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for OnlineIdServiceTicket {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OnlineIdServiceTicket {}
impl ::core::fmt::Debug for OnlineIdServiceTicket {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OnlineIdServiceTicket").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for OnlineIdServiceTicket {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.OnlineId.OnlineIdServiceTicket;{c95c547f-d781-4a94-acb8-c59874238c26})");
}
impl ::core::clone::Clone for OnlineIdServiceTicket {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for OnlineIdServiceTicket {
    type Vtable = IOnlineIdServiceTicket_Vtbl;
}
unsafe impl ::windows::core::ComInterface for OnlineIdServiceTicket {
    const IID: ::windows::core::GUID = <IOnlineIdServiceTicket as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for OnlineIdServiceTicket {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.OnlineIdServiceTicket";
}
::windows::imp::interface_hierarchy!(OnlineIdServiceTicket, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for OnlineIdServiceTicket {}
unsafe impl ::core::marker::Sync for OnlineIdServiceTicket {}
#[doc = "*Required features: `\"Security_Authentication_OnlineId\"`*"]
#[repr(transparent)]
pub struct OnlineIdServiceTicketRequest(::windows::core::IUnknown);
impl OnlineIdServiceTicketRequest {
    pub fn Service(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Service)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Policy(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Policy)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateOnlineIdServiceTicketRequest(service: &::windows::core::HSTRING, policy: &::windows::core::HSTRING) -> ::windows::core::Result<OnlineIdServiceTicketRequest> {
        Self::IOnlineIdServiceTicketRequestFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<OnlineIdServiceTicketRequest>();
            (::windows::core::Interface::vtable(this).CreateOnlineIdServiceTicketRequest)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(service), ::core::mem::transmute_copy(policy), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateOnlineIdServiceTicketRequestAdvanced(service: &::windows::core::HSTRING) -> ::windows::core::Result<OnlineIdServiceTicketRequest> {
        Self::IOnlineIdServiceTicketRequestFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<OnlineIdServiceTicketRequest>();
            (::windows::core::Interface::vtable(this).CreateOnlineIdServiceTicketRequestAdvanced)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(service), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IOnlineIdServiceTicketRequestFactory<R, F: FnOnce(&IOnlineIdServiceTicketRequestFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<OnlineIdServiceTicketRequest, IOnlineIdServiceTicketRequestFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for OnlineIdServiceTicketRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OnlineIdServiceTicketRequest {}
impl ::core::fmt::Debug for OnlineIdServiceTicketRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OnlineIdServiceTicketRequest").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for OnlineIdServiceTicketRequest {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.OnlineId.OnlineIdServiceTicketRequest;{297445d3-fb63-4135-8909-4e354c061466})");
}
impl ::core::clone::Clone for OnlineIdServiceTicketRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for OnlineIdServiceTicketRequest {
    type Vtable = IOnlineIdServiceTicketRequest_Vtbl;
}
unsafe impl ::windows::core::ComInterface for OnlineIdServiceTicketRequest {
    const IID: ::windows::core::GUID = <IOnlineIdServiceTicketRequest as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for OnlineIdServiceTicketRequest {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.OnlineIdServiceTicketRequest";
}
::windows::imp::interface_hierarchy!(OnlineIdServiceTicketRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for OnlineIdServiceTicketRequest {}
unsafe impl ::core::marker::Sync for OnlineIdServiceTicketRequest {}
#[doc = "*Required features: `\"Security_Authentication_OnlineId\"`*"]
pub struct OnlineIdSystemAuthenticator;
impl OnlineIdSystemAuthenticator {
    pub fn Default() -> ::windows::core::Result<OnlineIdSystemAuthenticatorForUser> {
        Self::IOnlineIdSystemAuthenticatorStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<OnlineIdSystemAuthenticatorForUser>();
            (::windows::core::Interface::vtable(this).Default)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn GetForUser(user: &super::super::super::System::User) -> ::windows::core::Result<OnlineIdSystemAuthenticatorForUser> {
        Self::IOnlineIdSystemAuthenticatorStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<OnlineIdSystemAuthenticatorForUser>();
            (::windows::core::Interface::vtable(this).GetForUser)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(user), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IOnlineIdSystemAuthenticatorStatics<R, F: FnOnce(&IOnlineIdSystemAuthenticatorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<OnlineIdSystemAuthenticator, IOnlineIdSystemAuthenticatorStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for OnlineIdSystemAuthenticator {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.OnlineIdSystemAuthenticator";
}
#[doc = "*Required features: `\"Security_Authentication_OnlineId\"`*"]
#[repr(transparent)]
pub struct OnlineIdSystemAuthenticatorForUser(::windows::core::IUnknown);
impl OnlineIdSystemAuthenticatorForUser {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetTicketAsync(&self, request: &OnlineIdServiceTicketRequest) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<OnlineIdSystemTicketResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<OnlineIdSystemTicketResult>>();
            (::windows::core::Interface::vtable(this).GetTicketAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(request), &mut result__).from_abi(result__)
        }
    }
    pub fn SetApplicationId(&self, value: ::windows::core::GUID) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetApplicationId)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ApplicationId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
            (::windows::core::Interface::vtable(this).ApplicationId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::System::User>();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for OnlineIdSystemAuthenticatorForUser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OnlineIdSystemAuthenticatorForUser {}
impl ::core::fmt::Debug for OnlineIdSystemAuthenticatorForUser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OnlineIdSystemAuthenticatorForUser").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for OnlineIdSystemAuthenticatorForUser {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.OnlineId.OnlineIdSystemAuthenticatorForUser;{5798befb-1de4-4186-a2e6-b563f86aaf44})");
}
impl ::core::clone::Clone for OnlineIdSystemAuthenticatorForUser {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for OnlineIdSystemAuthenticatorForUser {
    type Vtable = IOnlineIdSystemAuthenticatorForUser_Vtbl;
}
unsafe impl ::windows::core::ComInterface for OnlineIdSystemAuthenticatorForUser {
    const IID: ::windows::core::GUID = <IOnlineIdSystemAuthenticatorForUser as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for OnlineIdSystemAuthenticatorForUser {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.OnlineIdSystemAuthenticatorForUser";
}
::windows::imp::interface_hierarchy!(OnlineIdSystemAuthenticatorForUser, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for OnlineIdSystemAuthenticatorForUser {}
unsafe impl ::core::marker::Sync for OnlineIdSystemAuthenticatorForUser {}
#[doc = "*Required features: `\"Security_Authentication_OnlineId\"`*"]
#[repr(transparent)]
pub struct OnlineIdSystemIdentity(::windows::core::IUnknown);
impl OnlineIdSystemIdentity {
    pub fn Ticket(&self) -> ::windows::core::Result<OnlineIdServiceTicket> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<OnlineIdServiceTicket>();
            (::windows::core::Interface::vtable(this).Ticket)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for OnlineIdSystemIdentity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OnlineIdSystemIdentity {}
impl ::core::fmt::Debug for OnlineIdSystemIdentity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OnlineIdSystemIdentity").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for OnlineIdSystemIdentity {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.OnlineId.OnlineIdSystemIdentity;{743cd20d-b6ca-434d-8124-53ea12685307})");
}
impl ::core::clone::Clone for OnlineIdSystemIdentity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for OnlineIdSystemIdentity {
    type Vtable = IOnlineIdSystemIdentity_Vtbl;
}
unsafe impl ::windows::core::ComInterface for OnlineIdSystemIdentity {
    const IID: ::windows::core::GUID = <IOnlineIdSystemIdentity as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for OnlineIdSystemIdentity {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.OnlineIdSystemIdentity";
}
::windows::imp::interface_hierarchy!(OnlineIdSystemIdentity, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for OnlineIdSystemIdentity {}
unsafe impl ::core::marker::Sync for OnlineIdSystemIdentity {}
#[doc = "*Required features: `\"Security_Authentication_OnlineId\"`*"]
#[repr(transparent)]
pub struct OnlineIdSystemTicketResult(::windows::core::IUnknown);
impl OnlineIdSystemTicketResult {
    pub fn Identity(&self) -> ::windows::core::Result<OnlineIdSystemIdentity> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<OnlineIdSystemIdentity>();
            (::windows::core::Interface::vtable(this).Identity)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Status(&self) -> ::windows::core::Result<OnlineIdSystemTicketStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<OnlineIdSystemTicketStatus>();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HRESULT>();
            (::windows::core::Interface::vtable(this).ExtendedError)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for OnlineIdSystemTicketResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OnlineIdSystemTicketResult {}
impl ::core::fmt::Debug for OnlineIdSystemTicketResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OnlineIdSystemTicketResult").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for OnlineIdSystemTicketResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.OnlineId.OnlineIdSystemTicketResult;{db0a5ff8-b098-4acd-9d13-9e640652b5b6})");
}
impl ::core::clone::Clone for OnlineIdSystemTicketResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for OnlineIdSystemTicketResult {
    type Vtable = IOnlineIdSystemTicketResult_Vtbl;
}
unsafe impl ::windows::core::ComInterface for OnlineIdSystemTicketResult {
    const IID: ::windows::core::GUID = <IOnlineIdSystemTicketResult as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for OnlineIdSystemTicketResult {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.OnlineIdSystemTicketResult";
}
::windows::imp::interface_hierarchy!(OnlineIdSystemTicketResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for OnlineIdSystemTicketResult {}
unsafe impl ::core::marker::Sync for OnlineIdSystemTicketResult {}
#[doc = "*Required features: `\"Security_Authentication_OnlineId\"`, `\"Foundation\"`*"]
#[cfg(feature = "Foundation")]
#[repr(transparent)]
pub struct SignOutUserOperation(::windows::core::IUnknown);
#[cfg(feature = "Foundation")]
impl SignOutUserOperation {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetCompleted(&self, handler: &super::super::super::Foundation::AsyncActionCompletedHandler) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCompleted)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Completed(&self) -> ::windows::core::Result<super::super::super::Foundation::AsyncActionCompletedHandler> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::AsyncActionCompletedHandler>();
            (::windows::core::Interface::vtable(this).Completed)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetResults(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).GetResults)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<super::super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Status(&self) -> ::windows::core::Result<super::super::super::Foundation::AsyncStatus> {
        let this = &::windows::core::ComInterface::cast::<super::super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::AsyncStatus>();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::ComInterface::cast::<super::super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HRESULT>();
            (::windows::core::Interface::vtable(this).ErrorCode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Cancel(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Cancel)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::PartialEq for SignOutUserOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::Eq for SignOutUserOperation {}
#[cfg(feature = "Foundation")]
impl ::core::fmt::Debug for SignOutUserOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SignOutUserOperation").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeType for SignOutUserOperation {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.OnlineId.SignOutUserOperation;{5a648006-843a-4da9-865b-9d26e5dfad7b})");
}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for SignOutUserOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::Interface for SignOutUserOperation {
    type Vtable = super::super::super::Foundation::IAsyncAction_Vtbl;
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::ComInterface for SignOutUserOperation {
    const IID: ::windows::core::GUID = <super::super::super::Foundation::IAsyncAction as ::windows::core::ComInterface>::IID;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for SignOutUserOperation {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.SignOutUserOperation";
}
#[cfg(feature = "Foundation")]
impl SignOutUserOperation {
    pub fn get(&self) -> ::windows::core::Result<()> {
        if self.Status()? == super::super::super::Foundation::AsyncStatus::Started {
            let (_waiter, signaler) = ::windows::imp::Waiter::new()?;
            self.SetCompleted(&super::super::super::Foundation::AsyncActionCompletedHandler::new(move |_sender, _args| {
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
impl ::std::future::Future for SignOutUserOperation {
    type Output = ::windows::core::Result<()>;
    fn poll(self: ::std::pin::Pin<&mut Self>, context: &mut ::std::task::Context) -> ::std::task::Poll<Self::Output> {
        if self.Status()? == super::super::super::Foundation::AsyncStatus::Started {
            let waker = context.waker().clone();
            let _ = self.SetCompleted(&super::super::super::Foundation::AsyncActionCompletedHandler::new(move |_sender, _args| {
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
::windows::imp::interface_hierarchy!(SignOutUserOperation, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::super::Foundation::IAsyncAction> for SignOutUserOperation {}
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::super::Foundation::IAsyncInfo> for SignOutUserOperation {}
#[cfg(feature = "Foundation")]
unsafe impl ::core::marker::Send for SignOutUserOperation {}
#[cfg(feature = "Foundation")]
unsafe impl ::core::marker::Sync for SignOutUserOperation {}
#[doc = "*Required features: `\"Security_Authentication_OnlineId\"`, `\"Foundation\"`*"]
#[cfg(feature = "Foundation")]
#[repr(transparent)]
pub struct UserAuthenticationOperation(::windows::core::IUnknown);
#[cfg(feature = "Foundation")]
impl UserAuthenticationOperation {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<super::super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Status(&self) -> ::windows::core::Result<super::super::super::Foundation::AsyncStatus> {
        let this = &::windows::core::ComInterface::cast::<super::super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::AsyncStatus>();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::ComInterface::cast::<super::super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HRESULT>();
            (::windows::core::Interface::vtable(this).ErrorCode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Cancel(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Cancel)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<super::super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetCompleted(&self, handler: &super::super::super::Foundation::AsyncOperationCompletedHandler<UserIdentity>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCompleted)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Completed(&self) -> ::windows::core::Result<super::super::super::Foundation::AsyncOperationCompletedHandler<UserIdentity>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::AsyncOperationCompletedHandler<UserIdentity>>();
            (::windows::core::Interface::vtable(this).Completed)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetResults(&self) -> ::windows::core::Result<UserIdentity> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<UserIdentity>();
            (::windows::core::Interface::vtable(this).GetResults)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::PartialEq for UserAuthenticationOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::Eq for UserAuthenticationOperation {}
#[cfg(feature = "Foundation")]
impl ::core::fmt::Debug for UserAuthenticationOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserAuthenticationOperation").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeType for UserAuthenticationOperation {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.OnlineId.UserAuthenticationOperation;pinterface({9fc2b0bb-e446-44e2-aa61-9cab8f636af2};rc(Windows.Security.Authentication.OnlineId.UserIdentity;{2146d9cd-0742-4be3-8a1c-7c7ae679aa88})))");
}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for UserAuthenticationOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::Interface for UserAuthenticationOperation {
    type Vtable = super::super::super::Foundation::IAsyncOperation_Vtbl<UserIdentity>;
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::ComInterface for UserAuthenticationOperation {
    const IID: ::windows::core::GUID = <super::super::super::Foundation::IAsyncOperation<UserIdentity> as ::windows::core::ComInterface>::IID;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for UserAuthenticationOperation {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.UserAuthenticationOperation";
}
#[cfg(feature = "Foundation")]
impl UserAuthenticationOperation {
    pub fn get(&self) -> ::windows::core::Result<UserIdentity> {
        if self.Status()? == super::super::super::Foundation::AsyncStatus::Started {
            let (_waiter, signaler) = ::windows::imp::Waiter::new()?;
            self.SetCompleted(&super::super::super::Foundation::AsyncOperationCompletedHandler::new(move |_sender, _args| {
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
impl ::std::future::Future for UserAuthenticationOperation {
    type Output = ::windows::core::Result<UserIdentity>;
    fn poll(self: ::std::pin::Pin<&mut Self>, context: &mut ::std::task::Context) -> ::std::task::Poll<Self::Output> {
        if self.Status()? == super::super::super::Foundation::AsyncStatus::Started {
            let waker = context.waker().clone();
            let _ = self.SetCompleted(&super::super::super::Foundation::AsyncOperationCompletedHandler::new(move |_sender, _args| {
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
::windows::imp::interface_hierarchy!(UserAuthenticationOperation, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::super::Foundation::IAsyncInfo> for UserAuthenticationOperation {}
#[cfg(feature = "Foundation")]
impl ::windows::core::CanTryInto<super::super::super::Foundation::IAsyncOperation<UserIdentity>> for UserAuthenticationOperation {}
#[cfg(feature = "Foundation")]
unsafe impl ::core::marker::Send for UserAuthenticationOperation {}
#[cfg(feature = "Foundation")]
unsafe impl ::core::marker::Sync for UserAuthenticationOperation {}
#[doc = "*Required features: `\"Security_Authentication_OnlineId\"`*"]
#[repr(transparent)]
pub struct UserIdentity(::windows::core::IUnknown);
impl UserIdentity {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Tickets(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<OnlineIdServiceTicket>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Collections::IVectorView<OnlineIdServiceTicket>>();
            (::windows::core::Interface::vtable(this).Tickets)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SafeCustomerId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).SafeCustomerId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SignInName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).SignInName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FirstName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).FirstName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LastName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).LastName)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsBetaAccount(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsBetaAccount)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsConfirmedPC(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsConfirmedPC)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for UserIdentity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserIdentity {}
impl ::core::fmt::Debug for UserIdentity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserIdentity").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for UserIdentity {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.OnlineId.UserIdentity;{2146d9cd-0742-4be3-8a1c-7c7ae679aa88})");
}
impl ::core::clone::Clone for UserIdentity {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for UserIdentity {
    type Vtable = IUserIdentity_Vtbl;
}
unsafe impl ::windows::core::ComInterface for UserIdentity {
    const IID: ::windows::core::GUID = <IUserIdentity as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for UserIdentity {
    const NAME: &'static str = "Windows.Security.Authentication.OnlineId.UserIdentity";
}
::windows::imp::interface_hierarchy!(UserIdentity, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for UserIdentity {}
unsafe impl ::core::marker::Sync for UserIdentity {}
#[doc = "*Required features: `\"Security_Authentication_OnlineId\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CredentialPromptType(pub i32);
impl CredentialPromptType {
    pub const PromptIfNeeded: Self = Self(0i32);
    pub const RetypeCredentials: Self = Self(1i32);
    pub const DoNotPrompt: Self = Self(2i32);
}
impl ::core::marker::Copy for CredentialPromptType {}
impl ::core::clone::Clone for CredentialPromptType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CredentialPromptType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CredentialPromptType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CredentialPromptType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CredentialPromptType").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for CredentialPromptType {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.OnlineId.CredentialPromptType;i4)");
}
#[doc = "*Required features: `\"Security_Authentication_OnlineId\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OnlineIdSystemTicketStatus(pub i32);
impl OnlineIdSystemTicketStatus {
    pub const Success: Self = Self(0i32);
    pub const Error: Self = Self(1i32);
    pub const ServiceConnectionError: Self = Self(2i32);
}
impl ::core::marker::Copy for OnlineIdSystemTicketStatus {}
impl ::core::clone::Clone for OnlineIdSystemTicketStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OnlineIdSystemTicketStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for OnlineIdSystemTicketStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for OnlineIdSystemTicketStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OnlineIdSystemTicketStatus").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for OnlineIdSystemTicketStatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.OnlineId.OnlineIdSystemTicketStatus;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

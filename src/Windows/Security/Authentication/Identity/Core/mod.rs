#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct IMicrosoftAccountMultiFactorAuthenticationManager(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMicrosoftAccountMultiFactorAuthenticationManager {
    type Vtable = IMicrosoftAccountMultiFactorAuthenticationManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0fd340a5_f574_4320_a08e_0a19a82322aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMicrosoftAccountMultiFactorAuthenticationManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, useraccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, codelength: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, useraccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, authenticationtoken: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, wnschannelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, useraccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, useraccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, channeluri: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, useraccountidlist: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, useraccountidlist: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sessionauthentictionstatus: MicrosoftAccountMultiFactorSessionAuthenticationStatus, authenticationsessioninfo: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sessionauthentictionstatus: MicrosoftAccountMultiFactorSessionAuthenticationStatus, useraccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sessionid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sessionauthenticationtype: MicrosoftAccountMultiFactorAuthenticationType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, authenticationsessioninfo: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, useraccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sessionid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sessionauthenticationtype: MicrosoftAccountMultiFactorAuthenticationType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMicrosoftAccountMultiFactorAuthenticatorStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMicrosoftAccountMultiFactorAuthenticatorStatics {
    type Vtable = IMicrosoftAccountMultiFactorAuthenticatorStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd964c2e6_f446_4c71_8b79_6ea4024aa9b8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMicrosoftAccountMultiFactorAuthenticatorStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMicrosoftAccountMultiFactorGetSessionsResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMicrosoftAccountMultiFactorGetSessionsResult {
    type Vtable = IMicrosoftAccountMultiFactorGetSessionsResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e23a9a0_e9fa_497a_95de_6d5747bf974c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMicrosoftAccountMultiFactorGetSessionsResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut MicrosoftAccountMultiFactorServiceResponse) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMicrosoftAccountMultiFactorOneTimeCodedInfo(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMicrosoftAccountMultiFactorOneTimeCodedInfo {
    type Vtable = IMicrosoftAccountMultiFactorOneTimeCodedInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82ba264b_d87c_4668_a976_40cfae547d08);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMicrosoftAccountMultiFactorOneTimeCodedInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut MicrosoftAccountMultiFactorServiceResponse) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMicrosoftAccountMultiFactorSessionInfo(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMicrosoftAccountMultiFactorSessionInfo {
    type Vtable = IMicrosoftAccountMultiFactorSessionInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f7eabb4_a278_4635_b765_b494eb260af4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMicrosoftAccountMultiFactorSessionInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut MicrosoftAccountMultiFactorSessionApprovalStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut MicrosoftAccountMultiFactorAuthenticationType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    type Vtable = IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa7ec5fb_da3f_4088_a20d_5618afadb2e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut MicrosoftAccountMultiFactorServiceResponse) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MicrosoftAccountMultiFactorAuthenticationManager(pub ::windows::core::IInspectable);
impl MicrosoftAccountMultiFactorAuthenticationManager {
    #[cfg(feature = "Foundation")]
    pub fn GetOneTimePassCodeAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, useraccountid: Param0, codelength: u32) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorOneTimeCodedInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), useraccountid.into_param().abi(), codelength, &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorOneTimeCodedInfo>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn AddDeviceAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, useraccountid: Param0, authenticationtoken: Param1, wnschannelid: Param2) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), useraccountid.into_param().abi(), authenticationtoken.into_param().abi(), wnschannelid.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveDeviceAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, useraccountid: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), useraccountid.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn UpdateWnsChannelAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, useraccountid: Param0, channeluri: Param1) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), useraccountid.into_param().abi(), channeluri.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetSessionsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, useraccountidlist: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorGetSessionsResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), useraccountidlist.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorGetSessionsResult>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetSessionsAndUnregisteredAccountsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, useraccountidlist: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), useraccountidlist.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ApproveSessionUsingAuthSessionInfoAsync<'a, Param1: ::windows::core::IntoParam<'a, MicrosoftAccountMultiFactorSessionInfo>>(&self, sessionauthentictionstatus: MicrosoftAccountMultiFactorSessionAuthenticationStatus, authenticationsessioninfo: Param1) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), sessionauthentictionstatus, authenticationsessioninfo.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ApproveSessionAsync<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, sessionauthentictionstatus: MicrosoftAccountMultiFactorSessionAuthenticationStatus, useraccountid: Param1, sessionid: Param2, sessionauthenticationtype: MicrosoftAccountMultiFactorAuthenticationType) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), sessionauthentictionstatus, useraccountid.into_param().abi(), sessionid.into_param().abi(), sessionauthenticationtype, &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn DenySessionUsingAuthSessionInfoAsync<'a, Param0: ::windows::core::IntoParam<'a, MicrosoftAccountMultiFactorSessionInfo>>(&self, authenticationsessioninfo: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), authenticationsessioninfo.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn DenySessionAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, useraccountid: Param0, sessionid: Param1, sessionauthenticationtype: MicrosoftAccountMultiFactorAuthenticationType) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), useraccountid.into_param().abi(), sessionid.into_param().abi(), sessionauthenticationtype, &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>>(result__)
        }
    }
    pub fn Current() -> ::windows::core::Result<MicrosoftAccountMultiFactorAuthenticationManager> {
        Self::IMicrosoftAccountMultiFactorAuthenticatorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MicrosoftAccountMultiFactorAuthenticationManager>(result__)
        })
    }
    pub fn IMicrosoftAccountMultiFactorAuthenticatorStatics<R, F: FnOnce(&IMicrosoftAccountMultiFactorAuthenticatorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MicrosoftAccountMultiFactorAuthenticationManager, IMicrosoftAccountMultiFactorAuthenticatorStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for MicrosoftAccountMultiFactorAuthenticationManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorAuthenticationManager;{0fd340a5-f574-4320-a08e-0a19a82322aa})");
}
unsafe impl ::windows::core::Interface for MicrosoftAccountMultiFactorAuthenticationManager {
    type Vtable = IMicrosoftAccountMultiFactorAuthenticationManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0fd340a5_f574_4320_a08e_0a19a82322aa);
}
impl ::windows::core::RuntimeName for MicrosoftAccountMultiFactorAuthenticationManager {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorAuthenticationManager";
}
impl ::core::convert::From<MicrosoftAccountMultiFactorAuthenticationManager> for ::windows::core::IUnknown {
    fn from(value: MicrosoftAccountMultiFactorAuthenticationManager) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MicrosoftAccountMultiFactorAuthenticationManager> for ::windows::core::IUnknown {
    fn from(value: &MicrosoftAccountMultiFactorAuthenticationManager) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MicrosoftAccountMultiFactorAuthenticationManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MicrosoftAccountMultiFactorAuthenticationManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MicrosoftAccountMultiFactorAuthenticationManager> for ::windows::core::IInspectable {
    fn from(value: MicrosoftAccountMultiFactorAuthenticationManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MicrosoftAccountMultiFactorAuthenticationManager> for ::windows::core::IInspectable {
    fn from(value: &MicrosoftAccountMultiFactorAuthenticationManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MicrosoftAccountMultiFactorAuthenticationManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MicrosoftAccountMultiFactorAuthenticationManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MicrosoftAccountMultiFactorAuthenticationManager {}
unsafe impl ::core::marker::Sync for MicrosoftAccountMultiFactorAuthenticationManager {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MicrosoftAccountMultiFactorAuthenticationType(pub i32);
impl MicrosoftAccountMultiFactorAuthenticationType {
    pub const User: MicrosoftAccountMultiFactorAuthenticationType = MicrosoftAccountMultiFactorAuthenticationType(0i32);
    pub const Device: MicrosoftAccountMultiFactorAuthenticationType = MicrosoftAccountMultiFactorAuthenticationType(1i32);
}
impl ::core::convert::From<i32> for MicrosoftAccountMultiFactorAuthenticationType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MicrosoftAccountMultiFactorAuthenticationType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for MicrosoftAccountMultiFactorAuthenticationType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorAuthenticationType;i4)");
}
impl ::windows::core::DefaultType for MicrosoftAccountMultiFactorAuthenticationType {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MicrosoftAccountMultiFactorGetSessionsResult(pub ::windows::core::IInspectable);
impl MicrosoftAccountMultiFactorGetSessionsResult {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Sessions(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<MicrosoftAccountMultiFactorSessionInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<MicrosoftAccountMultiFactorSessionInfo>>(result__)
        }
    }
    pub fn ServiceResponse(&self) -> ::windows::core::Result<MicrosoftAccountMultiFactorServiceResponse> {
        let this = self;
        unsafe {
            let mut result__: MicrosoftAccountMultiFactorServiceResponse = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MicrosoftAccountMultiFactorServiceResponse>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MicrosoftAccountMultiFactorGetSessionsResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorGetSessionsResult;{4e23a9a0-e9fa-497a-95de-6d5747bf974c})");
}
unsafe impl ::windows::core::Interface for MicrosoftAccountMultiFactorGetSessionsResult {
    type Vtable = IMicrosoftAccountMultiFactorGetSessionsResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e23a9a0_e9fa_497a_95de_6d5747bf974c);
}
impl ::windows::core::RuntimeName for MicrosoftAccountMultiFactorGetSessionsResult {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorGetSessionsResult";
}
impl ::core::convert::From<MicrosoftAccountMultiFactorGetSessionsResult> for ::windows::core::IUnknown {
    fn from(value: MicrosoftAccountMultiFactorGetSessionsResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MicrosoftAccountMultiFactorGetSessionsResult> for ::windows::core::IUnknown {
    fn from(value: &MicrosoftAccountMultiFactorGetSessionsResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MicrosoftAccountMultiFactorGetSessionsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MicrosoftAccountMultiFactorGetSessionsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MicrosoftAccountMultiFactorGetSessionsResult> for ::windows::core::IInspectable {
    fn from(value: MicrosoftAccountMultiFactorGetSessionsResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MicrosoftAccountMultiFactorGetSessionsResult> for ::windows::core::IInspectable {
    fn from(value: &MicrosoftAccountMultiFactorGetSessionsResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MicrosoftAccountMultiFactorGetSessionsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MicrosoftAccountMultiFactorGetSessionsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MicrosoftAccountMultiFactorGetSessionsResult {}
unsafe impl ::core::marker::Sync for MicrosoftAccountMultiFactorGetSessionsResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MicrosoftAccountMultiFactorOneTimeCodedInfo(pub ::windows::core::IInspectable);
impl MicrosoftAccountMultiFactorOneTimeCodedInfo {
    pub fn Code(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn TimeInterval(&self) -> ::windows::core::Result<super::super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn TimeToLive(&self) -> ::windows::core::Result<super::super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    pub fn ServiceResponse(&self) -> ::windows::core::Result<MicrosoftAccountMultiFactorServiceResponse> {
        let this = self;
        unsafe {
            let mut result__: MicrosoftAccountMultiFactorServiceResponse = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MicrosoftAccountMultiFactorServiceResponse>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MicrosoftAccountMultiFactorOneTimeCodedInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorOneTimeCodedInfo;{82ba264b-d87c-4668-a976-40cfae547d08})");
}
unsafe impl ::windows::core::Interface for MicrosoftAccountMultiFactorOneTimeCodedInfo {
    type Vtable = IMicrosoftAccountMultiFactorOneTimeCodedInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82ba264b_d87c_4668_a976_40cfae547d08);
}
impl ::windows::core::RuntimeName for MicrosoftAccountMultiFactorOneTimeCodedInfo {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorOneTimeCodedInfo";
}
impl ::core::convert::From<MicrosoftAccountMultiFactorOneTimeCodedInfo> for ::windows::core::IUnknown {
    fn from(value: MicrosoftAccountMultiFactorOneTimeCodedInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MicrosoftAccountMultiFactorOneTimeCodedInfo> for ::windows::core::IUnknown {
    fn from(value: &MicrosoftAccountMultiFactorOneTimeCodedInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MicrosoftAccountMultiFactorOneTimeCodedInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MicrosoftAccountMultiFactorOneTimeCodedInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MicrosoftAccountMultiFactorOneTimeCodedInfo> for ::windows::core::IInspectable {
    fn from(value: MicrosoftAccountMultiFactorOneTimeCodedInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MicrosoftAccountMultiFactorOneTimeCodedInfo> for ::windows::core::IInspectable {
    fn from(value: &MicrosoftAccountMultiFactorOneTimeCodedInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MicrosoftAccountMultiFactorOneTimeCodedInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MicrosoftAccountMultiFactorOneTimeCodedInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MicrosoftAccountMultiFactorOneTimeCodedInfo {}
unsafe impl ::core::marker::Sync for MicrosoftAccountMultiFactorOneTimeCodedInfo {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MicrosoftAccountMultiFactorServiceResponse(pub i32);
impl MicrosoftAccountMultiFactorServiceResponse {
    pub const Success: MicrosoftAccountMultiFactorServiceResponse = MicrosoftAccountMultiFactorServiceResponse(0i32);
    pub const Error: MicrosoftAccountMultiFactorServiceResponse = MicrosoftAccountMultiFactorServiceResponse(1i32);
    pub const NoNetworkConnection: MicrosoftAccountMultiFactorServiceResponse = MicrosoftAccountMultiFactorServiceResponse(2i32);
    pub const ServiceUnavailable: MicrosoftAccountMultiFactorServiceResponse = MicrosoftAccountMultiFactorServiceResponse(3i32);
    pub const TotpSetupDenied: MicrosoftAccountMultiFactorServiceResponse = MicrosoftAccountMultiFactorServiceResponse(4i32);
    pub const NgcNotSetup: MicrosoftAccountMultiFactorServiceResponse = MicrosoftAccountMultiFactorServiceResponse(5i32);
    pub const SessionAlreadyDenied: MicrosoftAccountMultiFactorServiceResponse = MicrosoftAccountMultiFactorServiceResponse(6i32);
    pub const SessionAlreadyApproved: MicrosoftAccountMultiFactorServiceResponse = MicrosoftAccountMultiFactorServiceResponse(7i32);
    pub const SessionExpired: MicrosoftAccountMultiFactorServiceResponse = MicrosoftAccountMultiFactorServiceResponse(8i32);
    pub const NgcNonceExpired: MicrosoftAccountMultiFactorServiceResponse = MicrosoftAccountMultiFactorServiceResponse(9i32);
    pub const InvalidSessionId: MicrosoftAccountMultiFactorServiceResponse = MicrosoftAccountMultiFactorServiceResponse(10i32);
    pub const InvalidSessionType: MicrosoftAccountMultiFactorServiceResponse = MicrosoftAccountMultiFactorServiceResponse(11i32);
    pub const InvalidOperation: MicrosoftAccountMultiFactorServiceResponse = MicrosoftAccountMultiFactorServiceResponse(12i32);
    pub const InvalidStateTransition: MicrosoftAccountMultiFactorServiceResponse = MicrosoftAccountMultiFactorServiceResponse(13i32);
    pub const DeviceNotFound: MicrosoftAccountMultiFactorServiceResponse = MicrosoftAccountMultiFactorServiceResponse(14i32);
    pub const FlowDisabled: MicrosoftAccountMultiFactorServiceResponse = MicrosoftAccountMultiFactorServiceResponse(15i32);
    pub const SessionNotApproved: MicrosoftAccountMultiFactorServiceResponse = MicrosoftAccountMultiFactorServiceResponse(16i32);
    pub const OperationCanceledByUser: MicrosoftAccountMultiFactorServiceResponse = MicrosoftAccountMultiFactorServiceResponse(17i32);
    pub const NgcDisabledByServer: MicrosoftAccountMultiFactorServiceResponse = MicrosoftAccountMultiFactorServiceResponse(18i32);
    pub const NgcKeyNotFoundOnServer: MicrosoftAccountMultiFactorServiceResponse = MicrosoftAccountMultiFactorServiceResponse(19i32);
    pub const UIRequired: MicrosoftAccountMultiFactorServiceResponse = MicrosoftAccountMultiFactorServiceResponse(20i32);
    pub const DeviceIdChanged: MicrosoftAccountMultiFactorServiceResponse = MicrosoftAccountMultiFactorServiceResponse(21i32);
}
impl ::core::convert::From<i32> for MicrosoftAccountMultiFactorServiceResponse {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MicrosoftAccountMultiFactorServiceResponse {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for MicrosoftAccountMultiFactorServiceResponse {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorServiceResponse;i4)");
}
impl ::windows::core::DefaultType for MicrosoftAccountMultiFactorServiceResponse {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MicrosoftAccountMultiFactorSessionApprovalStatus(pub i32);
impl MicrosoftAccountMultiFactorSessionApprovalStatus {
    pub const Pending: MicrosoftAccountMultiFactorSessionApprovalStatus = MicrosoftAccountMultiFactorSessionApprovalStatus(0i32);
    pub const Approved: MicrosoftAccountMultiFactorSessionApprovalStatus = MicrosoftAccountMultiFactorSessionApprovalStatus(1i32);
    pub const Denied: MicrosoftAccountMultiFactorSessionApprovalStatus = MicrosoftAccountMultiFactorSessionApprovalStatus(2i32);
}
impl ::core::convert::From<i32> for MicrosoftAccountMultiFactorSessionApprovalStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MicrosoftAccountMultiFactorSessionApprovalStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for MicrosoftAccountMultiFactorSessionApprovalStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorSessionApprovalStatus;i4)");
}
impl ::windows::core::DefaultType for MicrosoftAccountMultiFactorSessionApprovalStatus {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MicrosoftAccountMultiFactorSessionAuthenticationStatus(pub i32);
impl MicrosoftAccountMultiFactorSessionAuthenticationStatus {
    pub const Authenticated: MicrosoftAccountMultiFactorSessionAuthenticationStatus = MicrosoftAccountMultiFactorSessionAuthenticationStatus(0i32);
    pub const Unauthenticated: MicrosoftAccountMultiFactorSessionAuthenticationStatus = MicrosoftAccountMultiFactorSessionAuthenticationStatus(1i32);
}
impl ::core::convert::From<i32> for MicrosoftAccountMultiFactorSessionAuthenticationStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MicrosoftAccountMultiFactorSessionAuthenticationStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for MicrosoftAccountMultiFactorSessionAuthenticationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorSessionAuthenticationStatus;i4)");
}
impl ::windows::core::DefaultType for MicrosoftAccountMultiFactorSessionAuthenticationStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MicrosoftAccountMultiFactorSessionInfo(pub ::windows::core::IInspectable);
impl MicrosoftAccountMultiFactorSessionInfo {
    pub fn UserAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SessionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn DisplaySessionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ApprovalStatus(&self) -> ::windows::core::Result<MicrosoftAccountMultiFactorSessionApprovalStatus> {
        let this = self;
        unsafe {
            let mut result__: MicrosoftAccountMultiFactorSessionApprovalStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MicrosoftAccountMultiFactorSessionApprovalStatus>(result__)
        }
    }
    pub fn AuthenticationType(&self) -> ::windows::core::Result<MicrosoftAccountMultiFactorAuthenticationType> {
        let this = self;
        unsafe {
            let mut result__: MicrosoftAccountMultiFactorAuthenticationType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MicrosoftAccountMultiFactorAuthenticationType>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestTime(&self) -> ::windows::core::Result<super::super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ExpirationTime(&self) -> ::windows::core::Result<super::super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::DateTime>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MicrosoftAccountMultiFactorSessionInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorSessionInfo;{5f7eabb4-a278-4635-b765-b494eb260af4})");
}
unsafe impl ::windows::core::Interface for MicrosoftAccountMultiFactorSessionInfo {
    type Vtable = IMicrosoftAccountMultiFactorSessionInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f7eabb4_a278_4635_b765_b494eb260af4);
}
impl ::windows::core::RuntimeName for MicrosoftAccountMultiFactorSessionInfo {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorSessionInfo";
}
impl ::core::convert::From<MicrosoftAccountMultiFactorSessionInfo> for ::windows::core::IUnknown {
    fn from(value: MicrosoftAccountMultiFactorSessionInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MicrosoftAccountMultiFactorSessionInfo> for ::windows::core::IUnknown {
    fn from(value: &MicrosoftAccountMultiFactorSessionInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MicrosoftAccountMultiFactorSessionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MicrosoftAccountMultiFactorSessionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MicrosoftAccountMultiFactorSessionInfo> for ::windows::core::IInspectable {
    fn from(value: MicrosoftAccountMultiFactorSessionInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MicrosoftAccountMultiFactorSessionInfo> for ::windows::core::IInspectable {
    fn from(value: &MicrosoftAccountMultiFactorSessionInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MicrosoftAccountMultiFactorSessionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MicrosoftAccountMultiFactorSessionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MicrosoftAccountMultiFactorSessionInfo {}
unsafe impl ::core::marker::Sync for MicrosoftAccountMultiFactorSessionInfo {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo(pub ::windows::core::IInspectable);
impl MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Sessions(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<MicrosoftAccountMultiFactorSessionInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<MicrosoftAccountMultiFactorSessionInfo>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn UnregisteredAccounts(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    pub fn ServiceResponse(&self) -> ::windows::core::Result<MicrosoftAccountMultiFactorServiceResponse> {
        let this = self;
        unsafe {
            let mut result__: MicrosoftAccountMultiFactorServiceResponse = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MicrosoftAccountMultiFactorServiceResponse>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo;{aa7ec5fb-da3f-4088-a20d-5618afadb2e5})");
}
unsafe impl ::windows::core::Interface for MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    type Vtable = IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa7ec5fb_da3f_4088_a20d_5618afadb2e5);
}
impl ::windows::core::RuntimeName for MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo";
}
impl ::core::convert::From<MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo> for ::windows::core::IUnknown {
    fn from(value: MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo> for ::windows::core::IUnknown {
    fn from(value: &MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo> for ::windows::core::IInspectable {
    fn from(value: MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo> for ::windows::core::IInspectable {
    fn from(value: &MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {}
unsafe impl ::core::marker::Sync for MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {}

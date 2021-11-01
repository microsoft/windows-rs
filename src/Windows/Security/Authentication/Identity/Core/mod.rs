#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMicrosoftAccountMultiFactorAuthenticationManager(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMicrosoftAccountMultiFactorAuthenticationManager {
    type Vtable = IMicrosoftAccountMultiFactorAuthenticationManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(265502885, 62836, 17184, [160, 142, 10, 25, 168, 35, 34, 170]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMicrosoftAccountMultiFactorAuthenticationManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, useraccountid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, codelength: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, useraccountid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, authenticationtoken: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, wnschannelid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, useraccountid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, useraccountid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, channeluri: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, useraccountidlist: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, useraccountidlist: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sessionauthentictionstatus: MicrosoftAccountMultiFactorSessionAuthenticationStatus, authenticationsessioninfo: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sessionauthentictionstatus: MicrosoftAccountMultiFactorSessionAuthenticationStatus, useraccountid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, sessionid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, sessionauthenticationtype: MicrosoftAccountMultiFactorAuthenticationType, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, authenticationsessioninfo: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, useraccountid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, sessionid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, sessionauthenticationtype: MicrosoftAccountMultiFactorAuthenticationType, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMicrosoftAccountMultiFactorAuthenticatorStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMicrosoftAccountMultiFactorAuthenticatorStatics {
    type Vtable = IMicrosoftAccountMultiFactorAuthenticatorStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3647259366, 62534, 19569, [139, 121, 110, 164, 2, 74, 169, 184]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMicrosoftAccountMultiFactorAuthenticatorStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMicrosoftAccountMultiFactorGetSessionsResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMicrosoftAccountMultiFactorGetSessionsResult {
    type Vtable = IMicrosoftAccountMultiFactorGetSessionsResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1310960032, 59898, 18810, [149, 222, 109, 87, 71, 191, 151, 76]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMicrosoftAccountMultiFactorGetSessionsResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MicrosoftAccountMultiFactorServiceResponse) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMicrosoftAccountMultiFactorOneTimeCodedInfo(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMicrosoftAccountMultiFactorOneTimeCodedInfo {
    type Vtable = IMicrosoftAccountMultiFactorOneTimeCodedInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2193237579, 55420, 18024, [169, 118, 64, 207, 174, 84, 125, 8]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMicrosoftAccountMultiFactorOneTimeCodedInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MicrosoftAccountMultiFactorServiceResponse) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMicrosoftAccountMultiFactorSessionInfo(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMicrosoftAccountMultiFactorSessionInfo {
    type Vtable = IMicrosoftAccountMultiFactorSessionInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1602137012, 41592, 17973, [183, 101, 180, 148, 235, 38, 10, 244]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMicrosoftAccountMultiFactorSessionInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MicrosoftAccountMultiFactorSessionApprovalStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MicrosoftAccountMultiFactorAuthenticationType) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    type Vtable = IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2860434939, 55871, 16520, [162, 13, 86, 24, 175, 173, 178, 229]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MicrosoftAccountMultiFactorServiceResponse) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Security_Authentication_Identity_Core`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MicrosoftAccountMultiFactorAuthenticationManager(::windows::runtime::IInspectable);
impl MicrosoftAccountMultiFactorAuthenticationManager {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_Identity_Core`, `Foundation`*"]
    pub fn GetOneTimePassCodeAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, useraccountid: Param0, codelength: u32) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorOneTimeCodedInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), useraccountid.into_param().abi(), codelength, &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorOneTimeCodedInfo>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_Identity_Core`, `Foundation`*"]
    pub fn AddDeviceAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, useraccountid: Param0, authenticationtoken: Param1, wnschannelid: Param2) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), useraccountid.into_param().abi(), authenticationtoken.into_param().abi(), wnschannelid.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_Identity_Core`, `Foundation`*"]
    pub fn RemoveDeviceAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, useraccountid: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), useraccountid.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_Identity_Core`, `Foundation`*"]
    pub fn UpdateWnsChannelAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, useraccountid: Param0, channeluri: Param1) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), useraccountid.into_param().abi(), channeluri.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Security_Authentication_Identity_Core`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetSessionsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(&self, useraccountidlist: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorGetSessionsResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), useraccountidlist.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorGetSessionsResult>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Security_Authentication_Identity_Core`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetSessionsAndUnregisteredAccountsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(&self, useraccountidlist: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), useraccountidlist.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_Identity_Core`, `Foundation`*"]
    pub fn ApproveSessionUsingAuthSessionInfoAsync<'a, Param1: ::windows::runtime::IntoParam<'a, MicrosoftAccountMultiFactorSessionInfo>>(&self, sessionauthentictionstatus: MicrosoftAccountMultiFactorSessionAuthenticationStatus, authenticationsessioninfo: Param1) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), sessionauthentictionstatus, authenticationsessioninfo.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_Identity_Core`, `Foundation`*"]
    pub fn ApproveSessionAsync<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(
        &self,
        sessionauthentictionstatus: MicrosoftAccountMultiFactorSessionAuthenticationStatus,
        useraccountid: Param1,
        sessionid: Param2,
        sessionauthenticationtype: MicrosoftAccountMultiFactorAuthenticationType,
    ) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), sessionauthentictionstatus, useraccountid.into_param().abi(), sessionid.into_param().abi(), sessionauthenticationtype, &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_Identity_Core`, `Foundation`*"]
    pub fn DenySessionUsingAuthSessionInfoAsync<'a, Param0: ::windows::runtime::IntoParam<'a, MicrosoftAccountMultiFactorSessionInfo>>(&self, authenticationsessioninfo: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), authenticationsessioninfo.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_Identity_Core`, `Foundation`*"]
    pub fn DenySessionAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, useraccountid: Param0, sessionid: Param1, sessionauthenticationtype: MicrosoftAccountMultiFactorAuthenticationType) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), useraccountid.into_param().abi(), sessionid.into_param().abi(), sessionauthenticationtype, &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>>(result__)
        }
    }
    #[doc = "*Required features: `Security_Authentication_Identity_Core`*"]
    pub fn Current() -> ::windows::runtime::Result<MicrosoftAccountMultiFactorAuthenticationManager> {
        Self::IMicrosoftAccountMultiFactorAuthenticatorStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MicrosoftAccountMultiFactorAuthenticationManager>(result__)
        })
    }
    pub fn IMicrosoftAccountMultiFactorAuthenticatorStatics<R, F: FnOnce(&IMicrosoftAccountMultiFactorAuthenticatorStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MicrosoftAccountMultiFactorAuthenticationManager, IMicrosoftAccountMultiFactorAuthenticatorStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MicrosoftAccountMultiFactorAuthenticationManager {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorAuthenticationManager;{0fd340a5-f574-4320-a08e-0a19a82322aa})");
}
unsafe impl ::windows::runtime::Interface for MicrosoftAccountMultiFactorAuthenticationManager {
    type Vtable = IMicrosoftAccountMultiFactorAuthenticationManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(265502885, 62836, 17184, [160, 142, 10, 25, 168, 35, 34, 170]);
}
impl ::windows::runtime::RuntimeName for MicrosoftAccountMultiFactorAuthenticationManager {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorAuthenticationManager";
}
impl ::std::convert::From<MicrosoftAccountMultiFactorAuthenticationManager> for ::windows::runtime::IUnknown {
    fn from(value: MicrosoftAccountMultiFactorAuthenticationManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MicrosoftAccountMultiFactorAuthenticationManager> for ::windows::runtime::IUnknown {
    fn from(value: &MicrosoftAccountMultiFactorAuthenticationManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MicrosoftAccountMultiFactorAuthenticationManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MicrosoftAccountMultiFactorAuthenticationManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MicrosoftAccountMultiFactorAuthenticationManager> for ::windows::runtime::IInspectable {
    fn from(value: MicrosoftAccountMultiFactorAuthenticationManager) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MicrosoftAccountMultiFactorAuthenticationManager> for ::windows::runtime::IInspectable {
    fn from(value: &MicrosoftAccountMultiFactorAuthenticationManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MicrosoftAccountMultiFactorAuthenticationManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MicrosoftAccountMultiFactorAuthenticationManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MicrosoftAccountMultiFactorAuthenticationManager {}
unsafe impl ::std::marker::Sync for MicrosoftAccountMultiFactorAuthenticationManager {}
#[doc = "*Required features: `Security_Authentication_Identity_Core`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MicrosoftAccountMultiFactorAuthenticationType(pub i32);
impl MicrosoftAccountMultiFactorAuthenticationType {
    pub const User: MicrosoftAccountMultiFactorAuthenticationType = MicrosoftAccountMultiFactorAuthenticationType(0i32);
    pub const Device: MicrosoftAccountMultiFactorAuthenticationType = MicrosoftAccountMultiFactorAuthenticationType(1i32);
}
impl ::std::convert::From<i32> for MicrosoftAccountMultiFactorAuthenticationType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MicrosoftAccountMultiFactorAuthenticationType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MicrosoftAccountMultiFactorAuthenticationType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorAuthenticationType;i4)");
}
#[doc = "*Required features: `Security_Authentication_Identity_Core`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MicrosoftAccountMultiFactorGetSessionsResult(::windows::runtime::IInspectable);
impl MicrosoftAccountMultiFactorGetSessionsResult {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Security_Authentication_Identity_Core`, `Foundation_Collections`*"]
    pub fn Sessions(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Collections::IVectorView<MicrosoftAccountMultiFactorSessionInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<MicrosoftAccountMultiFactorSessionInfo>>(result__)
        }
    }
    #[doc = "*Required features: `Security_Authentication_Identity_Core`*"]
    pub fn ServiceResponse(&self) -> ::windows::runtime::Result<MicrosoftAccountMultiFactorServiceResponse> {
        let this = self;
        unsafe {
            let mut result__: MicrosoftAccountMultiFactorServiceResponse = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MicrosoftAccountMultiFactorServiceResponse>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MicrosoftAccountMultiFactorGetSessionsResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorGetSessionsResult;{4e23a9a0-e9fa-497a-95de-6d5747bf974c})");
}
unsafe impl ::windows::runtime::Interface for MicrosoftAccountMultiFactorGetSessionsResult {
    type Vtable = IMicrosoftAccountMultiFactorGetSessionsResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1310960032, 59898, 18810, [149, 222, 109, 87, 71, 191, 151, 76]);
}
impl ::windows::runtime::RuntimeName for MicrosoftAccountMultiFactorGetSessionsResult {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorGetSessionsResult";
}
impl ::std::convert::From<MicrosoftAccountMultiFactorGetSessionsResult> for ::windows::runtime::IUnknown {
    fn from(value: MicrosoftAccountMultiFactorGetSessionsResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MicrosoftAccountMultiFactorGetSessionsResult> for ::windows::runtime::IUnknown {
    fn from(value: &MicrosoftAccountMultiFactorGetSessionsResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MicrosoftAccountMultiFactorGetSessionsResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MicrosoftAccountMultiFactorGetSessionsResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MicrosoftAccountMultiFactorGetSessionsResult> for ::windows::runtime::IInspectable {
    fn from(value: MicrosoftAccountMultiFactorGetSessionsResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MicrosoftAccountMultiFactorGetSessionsResult> for ::windows::runtime::IInspectable {
    fn from(value: &MicrosoftAccountMultiFactorGetSessionsResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MicrosoftAccountMultiFactorGetSessionsResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MicrosoftAccountMultiFactorGetSessionsResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MicrosoftAccountMultiFactorGetSessionsResult {}
unsafe impl ::std::marker::Sync for MicrosoftAccountMultiFactorGetSessionsResult {}
#[doc = "*Required features: `Security_Authentication_Identity_Core`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MicrosoftAccountMultiFactorOneTimeCodedInfo(::windows::runtime::IInspectable);
impl MicrosoftAccountMultiFactorOneTimeCodedInfo {
    #[doc = "*Required features: `Security_Authentication_Identity_Core`*"]
    pub fn Code(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_Identity_Core`, `Foundation`*"]
    pub fn TimeInterval(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_Identity_Core`, `Foundation`*"]
    pub fn TimeToLive(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `Security_Authentication_Identity_Core`*"]
    pub fn ServiceResponse(&self) -> ::windows::runtime::Result<MicrosoftAccountMultiFactorServiceResponse> {
        let this = self;
        unsafe {
            let mut result__: MicrosoftAccountMultiFactorServiceResponse = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MicrosoftAccountMultiFactorServiceResponse>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MicrosoftAccountMultiFactorOneTimeCodedInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorOneTimeCodedInfo;{82ba264b-d87c-4668-a976-40cfae547d08})");
}
unsafe impl ::windows::runtime::Interface for MicrosoftAccountMultiFactorOneTimeCodedInfo {
    type Vtable = IMicrosoftAccountMultiFactorOneTimeCodedInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2193237579, 55420, 18024, [169, 118, 64, 207, 174, 84, 125, 8]);
}
impl ::windows::runtime::RuntimeName for MicrosoftAccountMultiFactorOneTimeCodedInfo {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorOneTimeCodedInfo";
}
impl ::std::convert::From<MicrosoftAccountMultiFactorOneTimeCodedInfo> for ::windows::runtime::IUnknown {
    fn from(value: MicrosoftAccountMultiFactorOneTimeCodedInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MicrosoftAccountMultiFactorOneTimeCodedInfo> for ::windows::runtime::IUnknown {
    fn from(value: &MicrosoftAccountMultiFactorOneTimeCodedInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MicrosoftAccountMultiFactorOneTimeCodedInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MicrosoftAccountMultiFactorOneTimeCodedInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MicrosoftAccountMultiFactorOneTimeCodedInfo> for ::windows::runtime::IInspectable {
    fn from(value: MicrosoftAccountMultiFactorOneTimeCodedInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MicrosoftAccountMultiFactorOneTimeCodedInfo> for ::windows::runtime::IInspectable {
    fn from(value: &MicrosoftAccountMultiFactorOneTimeCodedInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MicrosoftAccountMultiFactorOneTimeCodedInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MicrosoftAccountMultiFactorOneTimeCodedInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MicrosoftAccountMultiFactorOneTimeCodedInfo {}
unsafe impl ::std::marker::Sync for MicrosoftAccountMultiFactorOneTimeCodedInfo {}
#[doc = "*Required features: `Security_Authentication_Identity_Core`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for MicrosoftAccountMultiFactorServiceResponse {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MicrosoftAccountMultiFactorServiceResponse {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MicrosoftAccountMultiFactorServiceResponse {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorServiceResponse;i4)");
}
#[doc = "*Required features: `Security_Authentication_Identity_Core`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MicrosoftAccountMultiFactorSessionApprovalStatus(pub i32);
impl MicrosoftAccountMultiFactorSessionApprovalStatus {
    pub const Pending: MicrosoftAccountMultiFactorSessionApprovalStatus = MicrosoftAccountMultiFactorSessionApprovalStatus(0i32);
    pub const Approved: MicrosoftAccountMultiFactorSessionApprovalStatus = MicrosoftAccountMultiFactorSessionApprovalStatus(1i32);
    pub const Denied: MicrosoftAccountMultiFactorSessionApprovalStatus = MicrosoftAccountMultiFactorSessionApprovalStatus(2i32);
}
impl ::std::convert::From<i32> for MicrosoftAccountMultiFactorSessionApprovalStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MicrosoftAccountMultiFactorSessionApprovalStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MicrosoftAccountMultiFactorSessionApprovalStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorSessionApprovalStatus;i4)");
}
#[doc = "*Required features: `Security_Authentication_Identity_Core`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MicrosoftAccountMultiFactorSessionAuthenticationStatus(pub i32);
impl MicrosoftAccountMultiFactorSessionAuthenticationStatus {
    pub const Authenticated: MicrosoftAccountMultiFactorSessionAuthenticationStatus = MicrosoftAccountMultiFactorSessionAuthenticationStatus(0i32);
    pub const Unauthenticated: MicrosoftAccountMultiFactorSessionAuthenticationStatus = MicrosoftAccountMultiFactorSessionAuthenticationStatus(1i32);
}
impl ::std::convert::From<i32> for MicrosoftAccountMultiFactorSessionAuthenticationStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MicrosoftAccountMultiFactorSessionAuthenticationStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MicrosoftAccountMultiFactorSessionAuthenticationStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorSessionAuthenticationStatus;i4)");
}
#[doc = "*Required features: `Security_Authentication_Identity_Core`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MicrosoftAccountMultiFactorSessionInfo(::windows::runtime::IInspectable);
impl MicrosoftAccountMultiFactorSessionInfo {
    #[doc = "*Required features: `Security_Authentication_Identity_Core`*"]
    pub fn UserAccountId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_Authentication_Identity_Core`*"]
    pub fn SessionId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_Authentication_Identity_Core`*"]
    pub fn DisplaySessionId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_Authentication_Identity_Core`*"]
    pub fn ApprovalStatus(&self) -> ::windows::runtime::Result<MicrosoftAccountMultiFactorSessionApprovalStatus> {
        let this = self;
        unsafe {
            let mut result__: MicrosoftAccountMultiFactorSessionApprovalStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MicrosoftAccountMultiFactorSessionApprovalStatus>(result__)
        }
    }
    #[doc = "*Required features: `Security_Authentication_Identity_Core`*"]
    pub fn AuthenticationType(&self) -> ::windows::runtime::Result<MicrosoftAccountMultiFactorAuthenticationType> {
        let this = self;
        unsafe {
            let mut result__: MicrosoftAccountMultiFactorAuthenticationType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MicrosoftAccountMultiFactorAuthenticationType>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_Identity_Core`, `Foundation`*"]
    pub fn RequestTime(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_Identity_Core`, `Foundation`*"]
    pub fn ExpirationTime(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::DateTime>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MicrosoftAccountMultiFactorSessionInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorSessionInfo;{5f7eabb4-a278-4635-b765-b494eb260af4})");
}
unsafe impl ::windows::runtime::Interface for MicrosoftAccountMultiFactorSessionInfo {
    type Vtable = IMicrosoftAccountMultiFactorSessionInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1602137012, 41592, 17973, [183, 101, 180, 148, 235, 38, 10, 244]);
}
impl ::windows::runtime::RuntimeName for MicrosoftAccountMultiFactorSessionInfo {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorSessionInfo";
}
impl ::std::convert::From<MicrosoftAccountMultiFactorSessionInfo> for ::windows::runtime::IUnknown {
    fn from(value: MicrosoftAccountMultiFactorSessionInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MicrosoftAccountMultiFactorSessionInfo> for ::windows::runtime::IUnknown {
    fn from(value: &MicrosoftAccountMultiFactorSessionInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MicrosoftAccountMultiFactorSessionInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MicrosoftAccountMultiFactorSessionInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MicrosoftAccountMultiFactorSessionInfo> for ::windows::runtime::IInspectable {
    fn from(value: MicrosoftAccountMultiFactorSessionInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MicrosoftAccountMultiFactorSessionInfo> for ::windows::runtime::IInspectable {
    fn from(value: &MicrosoftAccountMultiFactorSessionInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MicrosoftAccountMultiFactorSessionInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MicrosoftAccountMultiFactorSessionInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MicrosoftAccountMultiFactorSessionInfo {}
unsafe impl ::std::marker::Sync for MicrosoftAccountMultiFactorSessionInfo {}
#[doc = "*Required features: `Security_Authentication_Identity_Core`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo(::windows::runtime::IInspectable);
impl MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Security_Authentication_Identity_Core`, `Foundation_Collections`*"]
    pub fn Sessions(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Collections::IVectorView<MicrosoftAccountMultiFactorSessionInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<MicrosoftAccountMultiFactorSessionInfo>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Security_Authentication_Identity_Core`, `Foundation_Collections`*"]
    pub fn UnregisteredAccounts(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `Security_Authentication_Identity_Core`*"]
    pub fn ServiceResponse(&self) -> ::windows::runtime::Result<MicrosoftAccountMultiFactorServiceResponse> {
        let this = self;
        unsafe {
            let mut result__: MicrosoftAccountMultiFactorServiceResponse = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MicrosoftAccountMultiFactorServiceResponse>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo;{aa7ec5fb-da3f-4088-a20d-5618afadb2e5})");
}
unsafe impl ::windows::runtime::Interface for MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    type Vtable = IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2860434939, 55871, 16520, [162, 13, 86, 24, 175, 173, 178, 229]);
}
impl ::windows::runtime::RuntimeName for MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo";
}
impl ::std::convert::From<MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo> for ::windows::runtime::IUnknown {
    fn from(value: MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo> for ::windows::runtime::IUnknown {
    fn from(value: &MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo> for ::windows::runtime::IInspectable {
    fn from(value: MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo> for ::windows::runtime::IInspectable {
    fn from(value: &MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {}
unsafe impl ::std::marker::Sync for MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {}

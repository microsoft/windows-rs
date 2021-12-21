#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct IMicrosoftAccountMultiFactorAuthenticationManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMicrosoftAccountMultiFactorAuthenticationManager {
    type Vtable = IMicrosoftAccountMultiFactorAuthenticationManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0fd340a5_f574_4320_a08e_0a19a82322aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMicrosoftAccountMultiFactorAuthenticationManagerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, useraccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, codelength: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, useraccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, authenticationtoken: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, wnschannelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, useraccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, useraccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, channeluri: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, useraccountidlist: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, useraccountidlist: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionauthentictionstatus: MicrosoftAccountMultiFactorSessionAuthenticationStatus, authenticationsessioninfo: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sessionauthentictionstatus: MicrosoftAccountMultiFactorSessionAuthenticationStatus, useraccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sessionid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sessionauthenticationtype: MicrosoftAccountMultiFactorAuthenticationType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, authenticationsessioninfo: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, useraccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sessionid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sessionauthenticationtype: MicrosoftAccountMultiFactorAuthenticationType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMicrosoftAccountMultiFactorAuthenticatorStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMicrosoftAccountMultiFactorAuthenticatorStatics {
    type Vtable = IMicrosoftAccountMultiFactorAuthenticatorStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd964c2e6_f446_4c71_8b79_6ea4024aa9b8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMicrosoftAccountMultiFactorAuthenticatorStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMicrosoftAccountMultiFactorGetSessionsResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMicrosoftAccountMultiFactorGetSessionsResult {
    type Vtable = IMicrosoftAccountMultiFactorGetSessionsResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e23a9a0_e9fa_497a_95de_6d5747bf974c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMicrosoftAccountMultiFactorGetSessionsResultVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MicrosoftAccountMultiFactorServiceResponse) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMicrosoftAccountMultiFactorOneTimeCodedInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMicrosoftAccountMultiFactorOneTimeCodedInfo {
    type Vtable = IMicrosoftAccountMultiFactorOneTimeCodedInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82ba264b_d87c_4668_a976_40cfae547d08);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMicrosoftAccountMultiFactorOneTimeCodedInfoVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MicrosoftAccountMultiFactorServiceResponse) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMicrosoftAccountMultiFactorSessionInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMicrosoftAccountMultiFactorSessionInfo {
    type Vtable = IMicrosoftAccountMultiFactorSessionInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f7eabb4_a278_4635_b765_b494eb260af4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMicrosoftAccountMultiFactorSessionInfoVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MicrosoftAccountMultiFactorSessionApprovalStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MicrosoftAccountMultiFactorAuthenticationType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    type Vtable = IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa7ec5fb_da3f_4088_a20d_5618afadb2e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfoVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MicrosoftAccountMultiFactorServiceResponse) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Security_Authentication_Identity_Core'*"]
#[repr(transparent)]
pub struct MicrosoftAccountMultiFactorAuthenticationManager(::windows::core::IUnknown);
impl MicrosoftAccountMultiFactorAuthenticationManager {
    #[doc = "*Required features: 'Security_Authentication_Identity_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetOneTimePassCodeAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, useraccountid: Param0, codelength: u32) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorOneTimeCodedInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), useraccountid.into_param().abi(), codelength, &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorOneTimeCodedInfo>>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authentication_Identity_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn AddDeviceAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, useraccountid: Param0, authenticationtoken: Param1, wnschannelid: Param2) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), useraccountid.into_param().abi(), authenticationtoken.into_param().abi(), wnschannelid.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authentication_Identity_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDeviceAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, useraccountid: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), useraccountid.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authentication_Identity_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateWnsChannelAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, useraccountid: Param0, channeluri: Param1) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), useraccountid.into_param().abi(), channeluri.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authentication_Identity_Core', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetSessionsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, useraccountidlist: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorGetSessionsResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), useraccountidlist.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorGetSessionsResult>>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authentication_Identity_Core', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetSessionsAndUnregisteredAccountsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, useraccountidlist: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), useraccountidlist.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo>>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authentication_Identity_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ApproveSessionUsingAuthSessionInfoAsync<'a, Param1: ::windows::core::IntoParam<'a, MicrosoftAccountMultiFactorSessionInfo>>(&self, sessionauthentictionstatus: MicrosoftAccountMultiFactorSessionAuthenticationStatus, authenticationsessioninfo: Param1) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), sessionauthentictionstatus, authenticationsessioninfo.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authentication_Identity_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ApproveSessionAsync<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, sessionauthentictionstatus: MicrosoftAccountMultiFactorSessionAuthenticationStatus, useraccountid: Param1, sessionid: Param2, sessionauthenticationtype: MicrosoftAccountMultiFactorAuthenticationType) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), sessionauthentictionstatus, useraccountid.into_param().abi(), sessionid.into_param().abi(), sessionauthenticationtype, &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authentication_Identity_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn DenySessionUsingAuthSessionInfoAsync<'a, Param0: ::windows::core::IntoParam<'a, MicrosoftAccountMultiFactorSessionInfo>>(&self, authenticationsessioninfo: Param0) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), authenticationsessioninfo.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authentication_Identity_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn DenySessionAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, useraccountid: Param0, sessionid: Param1, sessionauthenticationtype: MicrosoftAccountMultiFactorAuthenticationType) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), useraccountid.into_param().abi(), sessionid.into_param().abi(), sessionauthenticationtype, &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<MicrosoftAccountMultiFactorServiceResponse>>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authentication_Identity_Core'*"]
    pub fn Current() -> ::windows::core::Result<MicrosoftAccountMultiFactorAuthenticationManager> {
        Self::IMicrosoftAccountMultiFactorAuthenticatorStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MicrosoftAccountMultiFactorAuthenticationManager>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMicrosoftAccountMultiFactorAuthenticatorStatics<R, F: FnOnce(&IMicrosoftAccountMultiFactorAuthenticatorStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MicrosoftAccountMultiFactorAuthenticationManager, IMicrosoftAccountMultiFactorAuthenticatorStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MicrosoftAccountMultiFactorAuthenticationManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MicrosoftAccountMultiFactorAuthenticationManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MicrosoftAccountMultiFactorAuthenticationManager {}
impl ::core::fmt::Debug for MicrosoftAccountMultiFactorAuthenticationManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MicrosoftAccountMultiFactorAuthenticationManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MicrosoftAccountMultiFactorAuthenticationManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorAuthenticationManager;{0fd340a5-f574-4320-a08e-0a19a82322aa})");
}
unsafe impl ::windows::core::Interface for MicrosoftAccountMultiFactorAuthenticationManager {
    type Vtable = IMicrosoftAccountMultiFactorAuthenticationManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0fd340a5_f574_4320_a08e_0a19a82322aa);
}
impl ::windows::core::RuntimeName for MicrosoftAccountMultiFactorAuthenticationManager {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorAuthenticationManager";
}
impl ::core::convert::From<MicrosoftAccountMultiFactorAuthenticationManager> for ::windows::core::IUnknown {
    fn from(value: MicrosoftAccountMultiFactorAuthenticationManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MicrosoftAccountMultiFactorAuthenticationManager> for ::windows::core::IUnknown {
    fn from(value: &MicrosoftAccountMultiFactorAuthenticationManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MicrosoftAccountMultiFactorAuthenticationManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MicrosoftAccountMultiFactorAuthenticationManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MicrosoftAccountMultiFactorAuthenticationManager> for ::windows::core::IInspectable {
    fn from(value: MicrosoftAccountMultiFactorAuthenticationManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MicrosoftAccountMultiFactorAuthenticationManager> for ::windows::core::IInspectable {
    fn from(value: &MicrosoftAccountMultiFactorAuthenticationManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MicrosoftAccountMultiFactorAuthenticationManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MicrosoftAccountMultiFactorAuthenticationManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MicrosoftAccountMultiFactorAuthenticationManager {}
unsafe impl ::core::marker::Sync for MicrosoftAccountMultiFactorAuthenticationManager {}
#[doc = "*Required features: 'Security_Authentication_Identity_Core'*"]
#[repr(transparent)]
pub struct MicrosoftAccountMultiFactorAuthenticationType(pub i32);
impl MicrosoftAccountMultiFactorAuthenticationType {
    pub const User: Self = Self(0i32);
    pub const Device: Self = Self(1i32);
}
impl ::core::marker::Copy for MicrosoftAccountMultiFactorAuthenticationType {}
impl ::core::clone::Clone for MicrosoftAccountMultiFactorAuthenticationType {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MicrosoftAccountMultiFactorAuthenticationType {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MicrosoftAccountMultiFactorAuthenticationType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MicrosoftAccountMultiFactorAuthenticationType {}
impl ::core::fmt::Debug for MicrosoftAccountMultiFactorAuthenticationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MicrosoftAccountMultiFactorAuthenticationType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MicrosoftAccountMultiFactorAuthenticationType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorAuthenticationType;i4)");
}
impl ::windows::core::DefaultType for MicrosoftAccountMultiFactorAuthenticationType {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Security_Authentication_Identity_Core'*"]
#[repr(transparent)]
pub struct MicrosoftAccountMultiFactorGetSessionsResult(::windows::core::IUnknown);
impl MicrosoftAccountMultiFactorGetSessionsResult {
    #[doc = "*Required features: 'Security_Authentication_Identity_Core', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Sessions(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<MicrosoftAccountMultiFactorSessionInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<MicrosoftAccountMultiFactorSessionInfo>>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authentication_Identity_Core'*"]
    pub fn ServiceResponse(&self) -> ::windows::core::Result<MicrosoftAccountMultiFactorServiceResponse> {
        let this = self;
        unsafe {
            let mut result__: MicrosoftAccountMultiFactorServiceResponse = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MicrosoftAccountMultiFactorServiceResponse>(result__)
        }
    }
}
impl ::core::clone::Clone for MicrosoftAccountMultiFactorGetSessionsResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MicrosoftAccountMultiFactorGetSessionsResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MicrosoftAccountMultiFactorGetSessionsResult {}
impl ::core::fmt::Debug for MicrosoftAccountMultiFactorGetSessionsResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MicrosoftAccountMultiFactorGetSessionsResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MicrosoftAccountMultiFactorGetSessionsResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorGetSessionsResult;{4e23a9a0-e9fa-497a-95de-6d5747bf974c})");
}
unsafe impl ::windows::core::Interface for MicrosoftAccountMultiFactorGetSessionsResult {
    type Vtable = IMicrosoftAccountMultiFactorGetSessionsResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e23a9a0_e9fa_497a_95de_6d5747bf974c);
}
impl ::windows::core::RuntimeName for MicrosoftAccountMultiFactorGetSessionsResult {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorGetSessionsResult";
}
impl ::core::convert::From<MicrosoftAccountMultiFactorGetSessionsResult> for ::windows::core::IUnknown {
    fn from(value: MicrosoftAccountMultiFactorGetSessionsResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MicrosoftAccountMultiFactorGetSessionsResult> for ::windows::core::IUnknown {
    fn from(value: &MicrosoftAccountMultiFactorGetSessionsResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MicrosoftAccountMultiFactorGetSessionsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MicrosoftAccountMultiFactorGetSessionsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MicrosoftAccountMultiFactorGetSessionsResult> for ::windows::core::IInspectable {
    fn from(value: MicrosoftAccountMultiFactorGetSessionsResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MicrosoftAccountMultiFactorGetSessionsResult> for ::windows::core::IInspectable {
    fn from(value: &MicrosoftAccountMultiFactorGetSessionsResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MicrosoftAccountMultiFactorGetSessionsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MicrosoftAccountMultiFactorGetSessionsResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MicrosoftAccountMultiFactorGetSessionsResult {}
unsafe impl ::core::marker::Sync for MicrosoftAccountMultiFactorGetSessionsResult {}
#[doc = "*Required features: 'Security_Authentication_Identity_Core'*"]
#[repr(transparent)]
pub struct MicrosoftAccountMultiFactorOneTimeCodedInfo(::windows::core::IUnknown);
impl MicrosoftAccountMultiFactorOneTimeCodedInfo {
    #[doc = "*Required features: 'Security_Authentication_Identity_Core'*"]
    pub fn Code(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authentication_Identity_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn TimeInterval(&self) -> ::windows::core::Result<super::super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authentication_Identity_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn TimeToLive(&self) -> ::windows::core::Result<super::super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authentication_Identity_Core'*"]
    pub fn ServiceResponse(&self) -> ::windows::core::Result<MicrosoftAccountMultiFactorServiceResponse> {
        let this = self;
        unsafe {
            let mut result__: MicrosoftAccountMultiFactorServiceResponse = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MicrosoftAccountMultiFactorServiceResponse>(result__)
        }
    }
}
impl ::core::clone::Clone for MicrosoftAccountMultiFactorOneTimeCodedInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MicrosoftAccountMultiFactorOneTimeCodedInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MicrosoftAccountMultiFactorOneTimeCodedInfo {}
impl ::core::fmt::Debug for MicrosoftAccountMultiFactorOneTimeCodedInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MicrosoftAccountMultiFactorOneTimeCodedInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MicrosoftAccountMultiFactorOneTimeCodedInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorOneTimeCodedInfo;{82ba264b-d87c-4668-a976-40cfae547d08})");
}
unsafe impl ::windows::core::Interface for MicrosoftAccountMultiFactorOneTimeCodedInfo {
    type Vtable = IMicrosoftAccountMultiFactorOneTimeCodedInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82ba264b_d87c_4668_a976_40cfae547d08);
}
impl ::windows::core::RuntimeName for MicrosoftAccountMultiFactorOneTimeCodedInfo {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorOneTimeCodedInfo";
}
impl ::core::convert::From<MicrosoftAccountMultiFactorOneTimeCodedInfo> for ::windows::core::IUnknown {
    fn from(value: MicrosoftAccountMultiFactorOneTimeCodedInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MicrosoftAccountMultiFactorOneTimeCodedInfo> for ::windows::core::IUnknown {
    fn from(value: &MicrosoftAccountMultiFactorOneTimeCodedInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MicrosoftAccountMultiFactorOneTimeCodedInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MicrosoftAccountMultiFactorOneTimeCodedInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MicrosoftAccountMultiFactorOneTimeCodedInfo> for ::windows::core::IInspectable {
    fn from(value: MicrosoftAccountMultiFactorOneTimeCodedInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MicrosoftAccountMultiFactorOneTimeCodedInfo> for ::windows::core::IInspectable {
    fn from(value: &MicrosoftAccountMultiFactorOneTimeCodedInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MicrosoftAccountMultiFactorOneTimeCodedInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MicrosoftAccountMultiFactorOneTimeCodedInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MicrosoftAccountMultiFactorOneTimeCodedInfo {}
unsafe impl ::core::marker::Sync for MicrosoftAccountMultiFactorOneTimeCodedInfo {}
#[doc = "*Required features: 'Security_Authentication_Identity_Core'*"]
#[repr(transparent)]
pub struct MicrosoftAccountMultiFactorServiceResponse(pub i32);
impl MicrosoftAccountMultiFactorServiceResponse {
    pub const Success: Self = Self(0i32);
    pub const Error: Self = Self(1i32);
    pub const NoNetworkConnection: Self = Self(2i32);
    pub const ServiceUnavailable: Self = Self(3i32);
    pub const TotpSetupDenied: Self = Self(4i32);
    pub const NgcNotSetup: Self = Self(5i32);
    pub const SessionAlreadyDenied: Self = Self(6i32);
    pub const SessionAlreadyApproved: Self = Self(7i32);
    pub const SessionExpired: Self = Self(8i32);
    pub const NgcNonceExpired: Self = Self(9i32);
    pub const InvalidSessionId: Self = Self(10i32);
    pub const InvalidSessionType: Self = Self(11i32);
    pub const InvalidOperation: Self = Self(12i32);
    pub const InvalidStateTransition: Self = Self(13i32);
    pub const DeviceNotFound: Self = Self(14i32);
    pub const FlowDisabled: Self = Self(15i32);
    pub const SessionNotApproved: Self = Self(16i32);
    pub const OperationCanceledByUser: Self = Self(17i32);
    pub const NgcDisabledByServer: Self = Self(18i32);
    pub const NgcKeyNotFoundOnServer: Self = Self(19i32);
    pub const UIRequired: Self = Self(20i32);
    pub const DeviceIdChanged: Self = Self(21i32);
}
impl ::core::marker::Copy for MicrosoftAccountMultiFactorServiceResponse {}
impl ::core::clone::Clone for MicrosoftAccountMultiFactorServiceResponse {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MicrosoftAccountMultiFactorServiceResponse {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MicrosoftAccountMultiFactorServiceResponse {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MicrosoftAccountMultiFactorServiceResponse {}
impl ::core::fmt::Debug for MicrosoftAccountMultiFactorServiceResponse {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MicrosoftAccountMultiFactorServiceResponse").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MicrosoftAccountMultiFactorServiceResponse {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorServiceResponse;i4)");
}
impl ::windows::core::DefaultType for MicrosoftAccountMultiFactorServiceResponse {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Security_Authentication_Identity_Core'*"]
#[repr(transparent)]
pub struct MicrosoftAccountMultiFactorSessionApprovalStatus(pub i32);
impl MicrosoftAccountMultiFactorSessionApprovalStatus {
    pub const Pending: Self = Self(0i32);
    pub const Approved: Self = Self(1i32);
    pub const Denied: Self = Self(2i32);
}
impl ::core::marker::Copy for MicrosoftAccountMultiFactorSessionApprovalStatus {}
impl ::core::clone::Clone for MicrosoftAccountMultiFactorSessionApprovalStatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MicrosoftAccountMultiFactorSessionApprovalStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MicrosoftAccountMultiFactorSessionApprovalStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MicrosoftAccountMultiFactorSessionApprovalStatus {}
impl ::core::fmt::Debug for MicrosoftAccountMultiFactorSessionApprovalStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MicrosoftAccountMultiFactorSessionApprovalStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MicrosoftAccountMultiFactorSessionApprovalStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorSessionApprovalStatus;i4)");
}
impl ::windows::core::DefaultType for MicrosoftAccountMultiFactorSessionApprovalStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Security_Authentication_Identity_Core'*"]
#[repr(transparent)]
pub struct MicrosoftAccountMultiFactorSessionAuthenticationStatus(pub i32);
impl MicrosoftAccountMultiFactorSessionAuthenticationStatus {
    pub const Authenticated: Self = Self(0i32);
    pub const Unauthenticated: Self = Self(1i32);
}
impl ::core::marker::Copy for MicrosoftAccountMultiFactorSessionAuthenticationStatus {}
impl ::core::clone::Clone for MicrosoftAccountMultiFactorSessionAuthenticationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MicrosoftAccountMultiFactorSessionAuthenticationStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MicrosoftAccountMultiFactorSessionAuthenticationStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MicrosoftAccountMultiFactorSessionAuthenticationStatus {}
impl ::core::fmt::Debug for MicrosoftAccountMultiFactorSessionAuthenticationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MicrosoftAccountMultiFactorSessionAuthenticationStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MicrosoftAccountMultiFactorSessionAuthenticationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorSessionAuthenticationStatus;i4)");
}
impl ::windows::core::DefaultType for MicrosoftAccountMultiFactorSessionAuthenticationStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Security_Authentication_Identity_Core'*"]
#[repr(transparent)]
pub struct MicrosoftAccountMultiFactorSessionInfo(::windows::core::IUnknown);
impl MicrosoftAccountMultiFactorSessionInfo {
    #[doc = "*Required features: 'Security_Authentication_Identity_Core'*"]
    pub fn UserAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authentication_Identity_Core'*"]
    pub fn SessionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authentication_Identity_Core'*"]
    pub fn DisplaySessionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authentication_Identity_Core'*"]
    pub fn ApprovalStatus(&self) -> ::windows::core::Result<MicrosoftAccountMultiFactorSessionApprovalStatus> {
        let this = self;
        unsafe {
            let mut result__: MicrosoftAccountMultiFactorSessionApprovalStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MicrosoftAccountMultiFactorSessionApprovalStatus>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authentication_Identity_Core'*"]
    pub fn AuthenticationType(&self) -> ::windows::core::Result<MicrosoftAccountMultiFactorAuthenticationType> {
        let this = self;
        unsafe {
            let mut result__: MicrosoftAccountMultiFactorAuthenticationType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MicrosoftAccountMultiFactorAuthenticationType>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authentication_Identity_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestTime(&self) -> ::windows::core::Result<super::super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authentication_Identity_Core', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ExpirationTime(&self) -> ::windows::core::Result<super::super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::DateTime>(result__)
        }
    }
}
impl ::core::clone::Clone for MicrosoftAccountMultiFactorSessionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MicrosoftAccountMultiFactorSessionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MicrosoftAccountMultiFactorSessionInfo {}
impl ::core::fmt::Debug for MicrosoftAccountMultiFactorSessionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MicrosoftAccountMultiFactorSessionInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MicrosoftAccountMultiFactorSessionInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorSessionInfo;{5f7eabb4-a278-4635-b765-b494eb260af4})");
}
unsafe impl ::windows::core::Interface for MicrosoftAccountMultiFactorSessionInfo {
    type Vtable = IMicrosoftAccountMultiFactorSessionInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f7eabb4_a278_4635_b765_b494eb260af4);
}
impl ::windows::core::RuntimeName for MicrosoftAccountMultiFactorSessionInfo {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorSessionInfo";
}
impl ::core::convert::From<MicrosoftAccountMultiFactorSessionInfo> for ::windows::core::IUnknown {
    fn from(value: MicrosoftAccountMultiFactorSessionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MicrosoftAccountMultiFactorSessionInfo> for ::windows::core::IUnknown {
    fn from(value: &MicrosoftAccountMultiFactorSessionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MicrosoftAccountMultiFactorSessionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MicrosoftAccountMultiFactorSessionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MicrosoftAccountMultiFactorSessionInfo> for ::windows::core::IInspectable {
    fn from(value: MicrosoftAccountMultiFactorSessionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MicrosoftAccountMultiFactorSessionInfo> for ::windows::core::IInspectable {
    fn from(value: &MicrosoftAccountMultiFactorSessionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MicrosoftAccountMultiFactorSessionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MicrosoftAccountMultiFactorSessionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MicrosoftAccountMultiFactorSessionInfo {}
unsafe impl ::core::marker::Sync for MicrosoftAccountMultiFactorSessionInfo {}
#[doc = "*Required features: 'Security_Authentication_Identity_Core'*"]
#[repr(transparent)]
pub struct MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo(::windows::core::IUnknown);
impl MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    #[doc = "*Required features: 'Security_Authentication_Identity_Core', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Sessions(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<MicrosoftAccountMultiFactorSessionInfo>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<MicrosoftAccountMultiFactorSessionInfo>>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authentication_Identity_Core', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn UnregisteredAccounts(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authentication_Identity_Core'*"]
    pub fn ServiceResponse(&self) -> ::windows::core::Result<MicrosoftAccountMultiFactorServiceResponse> {
        let this = self;
        unsafe {
            let mut result__: MicrosoftAccountMultiFactorServiceResponse = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MicrosoftAccountMultiFactorServiceResponse>(result__)
        }
    }
}
impl ::core::clone::Clone for MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {}
impl ::core::fmt::Debug for MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo;{aa7ec5fb-da3f-4088-a20d-5618afadb2e5})");
}
unsafe impl ::windows::core::Interface for MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    type Vtable = IMicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfoVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa7ec5fb_da3f_4088_a20d_5618afadb2e5);
}
impl ::windows::core::RuntimeName for MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    const NAME: &'static str = "Windows.Security.Authentication.Identity.Core.MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo";
}
impl ::core::convert::From<MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo> for ::windows::core::IUnknown {
    fn from(value: MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo> for ::windows::core::IUnknown {
    fn from(value: &MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo> for ::windows::core::IInspectable {
    fn from(value: MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo> for ::windows::core::IInspectable {
    fn from(value: &MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {}
unsafe impl ::core::marker::Sync for MicrosoftAccountMultiFactorUnregisteredAccountsAndSessionInfo {}

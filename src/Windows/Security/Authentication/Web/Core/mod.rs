#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Security_Authentication_Web_Core`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct FindAllAccountsResult(pub ::windows::runtime::IInspectable);
impl FindAllAccountsResult {
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    #[doc = "*Required features: `Security_Authentication_Web_Core`, `Foundation_Collections`, `Security_Credentials`*"]
    pub fn Accounts(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::Credentials::WebAccount>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::Credentials::WebAccount>>(result__)
        }
    }
    #[doc = "*Required features: `Security_Authentication_Web_Core`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<FindAllWebAccountsStatus> {
        let this = self;
        unsafe {
            let mut result__: FindAllWebAccountsStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<FindAllWebAccountsStatus>(result__)
        }
    }
    #[doc = "*Required features: `Security_Authentication_Web_Core`*"]
    pub fn ProviderError(&self) -> ::windows::runtime::Result<WebProviderError> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<WebProviderError>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for FindAllAccountsResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Web.Core.FindAllAccountsResult;{a5812b5d-b72e-420c-86ab-aac0d7b7261f})");
}
unsafe impl ::windows::runtime::Interface for FindAllAccountsResult {
    type Vtable = IFindAllAccountsResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2776705885, 46894, 16908, [134, 171, 170, 192, 215, 183, 38, 31]);
}
impl ::windows::runtime::RuntimeName for FindAllAccountsResult {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.FindAllAccountsResult";
}
impl ::std::convert::From<FindAllAccountsResult> for ::windows::runtime::IUnknown {
    fn from(value: FindAllAccountsResult) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&FindAllAccountsResult> for ::windows::runtime::IUnknown {
    fn from(value: &FindAllAccountsResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for FindAllAccountsResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a FindAllAccountsResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<FindAllAccountsResult> for ::windows::runtime::IInspectable {
    fn from(value: FindAllAccountsResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FindAllAccountsResult> for ::windows::runtime::IInspectable {
    fn from(value: &FindAllAccountsResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for FindAllAccountsResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a FindAllAccountsResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for FindAllAccountsResult {}
unsafe impl ::std::marker::Sync for FindAllAccountsResult {}
#[doc = "*Required features: `Security_Authentication_Web_Core`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FindAllWebAccountsStatus(pub i32);
impl FindAllWebAccountsStatus {
    pub const Success: FindAllWebAccountsStatus = FindAllWebAccountsStatus(0i32);
    pub const NotAllowedByProvider: FindAllWebAccountsStatus = FindAllWebAccountsStatus(1i32);
    pub const NotSupportedByProvider: FindAllWebAccountsStatus = FindAllWebAccountsStatus(2i32);
    pub const ProviderError: FindAllWebAccountsStatus = FindAllWebAccountsStatus(3i32);
}
impl ::std::convert::From<i32> for FindAllWebAccountsStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FindAllWebAccountsStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for FindAllWebAccountsStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Web.Core.FindAllWebAccountsStatus;i4)");
}
impl ::windows::runtime::DefaultType for FindAllWebAccountsStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IFindAllAccountsResult(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFindAllAccountsResult {
    type Vtable = IFindAllAccountsResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2776705885, 46894, 16908, [134, 171, 170, 192, 215, 183, 38, 31]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFindAllAccountsResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut FindAllWebAccountsStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWebAccountEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebAccountEventArgs {
    type Vtable = IWebAccountEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1874264957, 16974, 17644, [151, 124, 239, 36, 21, 70, 42, 90]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWebAccountMonitor(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebAccountMonitor {
    type Vtable = IWebAccountMonitor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1950742013, 43677, 17945, [141, 93, 193, 56, 164, 237, 227, 229]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountMonitor_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWebAccountMonitor2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebAccountMonitor2 {
    type Vtable = IWebAccountMonitor2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2813182456, 9400, 20225, [154, 229, 36, 84, 94, 113, 35, 58]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAccountMonitor2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWebAuthenticationCoreManagerStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebAuthenticationCoreManagerStatics {
    type Vtable = IWebAuthenticationCoreManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1791655058, 42369, 17529, [156, 16, 117, 46, 255, 68, 253, 52]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAuthenticationCoreManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, request: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, request: ::windows::runtime::RawPtr, webaccount: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, request: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, request: ::windows::runtime::RawPtr, webaccount: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, provider: ::windows::runtime::RawPtr, webaccountid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, webaccountproviderid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, webaccountproviderid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, authority: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWebAuthenticationCoreManagerStatics2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebAuthenticationCoreManagerStatics2 {
    type Vtable = IWebAuthenticationCoreManagerStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4119074890, 35671, 18464, [182, 164, 112, 165, 182, 252, 244, 74]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAuthenticationCoreManagerStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "System"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, webaccountproviderid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, authority: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, user: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials", feature = "System")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWebAuthenticationCoreManagerStatics3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebAuthenticationCoreManagerStatics3 {
    type Vtable = IWebAuthenticationCoreManagerStatics3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(604303026, 35108, 19859, [171, 58, 153, 104, 139, 65, 157, 86]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAuthenticationCoreManagerStatics3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, webaccounts: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Security_Credentials")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWebAuthenticationCoreManagerStatics4(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebAuthenticationCoreManagerStatics4 {
    type Vtable = IWebAuthenticationCoreManagerStatics4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1424372734, 38624, 16872, [152, 50, 18, 152, 137, 124, 42, 175]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAuthenticationCoreManagerStatics4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, provider: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, provider: ::windows::runtime::RawPtr, clientid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, webaccountproviderid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, webaccountproviderid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, authority: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "System"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, webaccountproviderid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, authority: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, user: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Security_Credentials", feature = "System")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWebProviderError(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebProviderError {
    type Vtable = IWebProviderError_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3675855793, 20677, 18441, [141, 202, 9, 201, 148, 16, 36, 92]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebProviderError_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWebProviderErrorFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebProviderErrorFactory {
    type Vtable = IWebProviderErrorFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3821275693, 35311, 20023, [132, 127, 168, 185, 213, 163, 41, 16]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebProviderErrorFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, errorcode: u32, errormessage: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWebTokenRequest(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebTokenRequest {
    type Vtable = IWebTokenRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3078311272, 44491, 18035, [179, 100, 12, 247, 179, 92, 175, 151]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebTokenRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut WebTokenRequestPromptType) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWebTokenRequest2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebTokenRequest2 {
    type Vtable = IWebTokenRequest2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3607150713, 12488, 17303, [150, 84, 150, 28, 59, 232, 184, 85]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebTokenRequest2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWebTokenRequest3(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebTokenRequest3 {
    type Vtable = IWebTokenRequest3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1517640529, 15281, 16805, [166, 61, 144, 188, 50, 199, 219, 154]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebTokenRequest3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWebTokenRequestFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebTokenRequestFactory {
    type Vtable = IWebTokenRequestFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1827804188, 4080, 19559, [184, 79, 153, 221, 190, 74, 114, 201]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebTokenRequestFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, provider: ::windows::runtime::RawPtr, scope: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, clientid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, provider: ::windows::runtime::RawPtr, scope: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, clientid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, prompttype: WebTokenRequestPromptType, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, provider: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, provider: ::windows::runtime::RawPtr, scope: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWebTokenRequestResult(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebTokenRequestResult {
    type Vtable = IWebTokenRequestResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3240788741, 53752, 17539, [141, 84, 56, 254, 41, 39, 132, 255]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebTokenRequestResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut WebTokenRequestStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWebTokenResponse(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebTokenResponse {
    type Vtable = IWebTokenResponse_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1739048394, 33782, 17606, [163, 177, 14, 182, 158, 65, 250, 138]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebTokenResponse_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWebTokenResponseFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebTokenResponseFactory {
    type Vtable = IWebTokenResponseFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2875979768, 21584, 20214, [151, 247, 5, 43, 4, 49, 192, 240]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebTokenResponseFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, webaccount: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
    #[cfg(feature = "Security_Credentials")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, webaccount: ::windows::runtime::RawPtr, error: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
);
#[doc = "*Required features: `Security_Authentication_Web_Core`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct WebAccountEventArgs(pub ::windows::runtime::IInspectable);
impl WebAccountEventArgs {
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `Security_Authentication_Web_Core`, `Security_Credentials`*"]
    pub fn Account(&self) -> ::windows::runtime::Result<super::super::super::Credentials::WebAccount> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Credentials::WebAccount>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for WebAccountEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Web.Core.WebAccountEventArgs;{6fb7037d-424e-44ec-977c-ef2415462a5a})");
}
unsafe impl ::windows::runtime::Interface for WebAccountEventArgs {
    type Vtable = IWebAccountEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1874264957, 16974, 17644, [151, 124, 239, 36, 21, 70, 42, 90]);
}
impl ::windows::runtime::RuntimeName for WebAccountEventArgs {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.WebAccountEventArgs";
}
impl ::std::convert::From<WebAccountEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: WebAccountEventArgs) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&WebAccountEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &WebAccountEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for WebAccountEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a WebAccountEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<WebAccountEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: WebAccountEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&WebAccountEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &WebAccountEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for WebAccountEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a WebAccountEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for WebAccountEventArgs {}
unsafe impl ::std::marker::Sync for WebAccountEventArgs {}
#[doc = "*Required features: `Security_Authentication_Web_Core`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct WebAccountMonitor(pub ::windows::runtime::IInspectable);
impl WebAccountMonitor {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_Web_Core`, `Foundation`*"]
    pub fn Updated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<WebAccountMonitor, WebAccountEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_Web_Core`, `Foundation`*"]
    pub fn RemoveUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_Web_Core`, `Foundation`*"]
    pub fn Removed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<WebAccountMonitor, WebAccountEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_Web_Core`, `Foundation`*"]
    pub fn RemoveRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_Web_Core`, `Foundation`*"]
    pub fn DefaultSignInAccountChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<WebAccountMonitor, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_Web_Core`, `Foundation`*"]
    pub fn RemoveDefaultSignInAccountChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_Web_Core`, `Foundation`*"]
    pub fn AccountPictureUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::TypedEventHandler<WebAccountMonitor, WebAccountEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IWebAccountMonitor2>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_Web_Core`, `Foundation`*"]
    pub fn RemoveAccountPictureUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IWebAccountMonitor2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for WebAccountMonitor {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Web.Core.WebAccountMonitor;{7445f5fd-aa9d-4619-8d5d-c138a4ede3e5})");
}
unsafe impl ::windows::runtime::Interface for WebAccountMonitor {
    type Vtable = IWebAccountMonitor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1950742013, 43677, 17945, [141, 93, 193, 56, 164, 237, 227, 229]);
}
impl ::windows::runtime::RuntimeName for WebAccountMonitor {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.WebAccountMonitor";
}
impl ::std::convert::From<WebAccountMonitor> for ::windows::runtime::IUnknown {
    fn from(value: WebAccountMonitor) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&WebAccountMonitor> for ::windows::runtime::IUnknown {
    fn from(value: &WebAccountMonitor) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for WebAccountMonitor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a WebAccountMonitor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<WebAccountMonitor> for ::windows::runtime::IInspectable {
    fn from(value: WebAccountMonitor) -> Self {
        value.0
    }
}
impl ::std::convert::From<&WebAccountMonitor> for ::windows::runtime::IInspectable {
    fn from(value: &WebAccountMonitor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for WebAccountMonitor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a WebAccountMonitor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for WebAccountMonitor {}
unsafe impl ::std::marker::Sync for WebAccountMonitor {}
#[doc = "*Required features: `Security_Authentication_Web_Core`*"]
pub struct WebAuthenticationCoreManager {}
impl WebAuthenticationCoreManager {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_Web_Core`, `Foundation`*"]
    pub fn GetTokenSilentlyAsync<'a, Param0: ::windows::runtime::IntoParam<'a, WebTokenRequest>>(request: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<WebTokenRequestResult>> {
        Self::IWebAuthenticationCoreManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), request.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<WebTokenRequestResult>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    #[doc = "*Required features: `Security_Authentication_Web_Core`, `Foundation`, `Security_Credentials`*"]
    pub fn GetTokenSilentlyWithWebAccountAsync<'a, Param0: ::windows::runtime::IntoParam<'a, WebTokenRequest>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Credentials::WebAccount>>(request: Param0, webaccount: Param1) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<WebTokenRequestResult>> {
        Self::IWebAuthenticationCoreManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), request.into_param().abi(), webaccount.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<WebTokenRequestResult>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_Web_Core`, `Foundation`*"]
    pub fn RequestTokenAsync<'a, Param0: ::windows::runtime::IntoParam<'a, WebTokenRequest>>(request: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<WebTokenRequestResult>> {
        Self::IWebAuthenticationCoreManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), request.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<WebTokenRequestResult>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    #[doc = "*Required features: `Security_Authentication_Web_Core`, `Foundation`, `Security_Credentials`*"]
    pub fn RequestTokenWithWebAccountAsync<'a, Param0: ::windows::runtime::IntoParam<'a, WebTokenRequest>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Credentials::WebAccount>>(request: Param0, webaccount: Param1) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<WebTokenRequestResult>> {
        Self::IWebAuthenticationCoreManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), request.into_param().abi(), webaccount.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<WebTokenRequestResult>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    #[doc = "*Required features: `Security_Authentication_Web_Core`, `Foundation`, `Security_Credentials`*"]
    pub fn FindAccountAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Credentials::WebAccountProvider>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(provider: Param0, webaccountid: Param1) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>> {
        Self::IWebAuthenticationCoreManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), provider.into_param().abi(), webaccountid.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccount>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    #[doc = "*Required features: `Security_Authentication_Web_Core`, `Foundation`, `Security_Credentials`*"]
    pub fn FindAccountProviderAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(webaccountproviderid: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>> {
        Self::IWebAuthenticationCoreManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), webaccountproviderid.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    #[doc = "*Required features: `Security_Authentication_Web_Core`, `Foundation`, `Security_Credentials`*"]
    pub fn FindAccountProviderWithAuthorityAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(webaccountproviderid: Param0, authority: Param1) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>> {
        Self::IWebAuthenticationCoreManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), webaccountproviderid.into_param().abi(), authority.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "System"))]
    #[doc = "*Required features: `Security_Authentication_Web_Core`, `Foundation`, `Security_Credentials`, `System`*"]
    pub fn FindAccountProviderWithAuthorityForUserAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::super::System::User>>(webaccountproviderid: Param0, authority: Param1, user: Param2) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>> {
        Self::IWebAuthenticationCoreManagerStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), webaccountproviderid.into_param().abi(), authority.into_param().abi(), user.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
    #[doc = "*Required features: `Security_Authentication_Web_Core`, `Foundation_Collections`, `Security_Credentials`*"]
    pub fn CreateWebAccountMonitor<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::Collections::IIterable<super::super::super::Credentials::WebAccount>>>(webaccounts: Param0) -> ::windows::runtime::Result<WebAccountMonitor> {
        Self::IWebAuthenticationCoreManagerStatics3(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), webaccounts.into_param().abi(), &mut result__).from_abi::<WebAccountMonitor>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    #[doc = "*Required features: `Security_Authentication_Web_Core`, `Foundation`, `Security_Credentials`*"]
    pub fn FindAllAccountsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Credentials::WebAccountProvider>>(provider: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<FindAllAccountsResult>> {
        Self::IWebAuthenticationCoreManagerStatics4(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), provider.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<FindAllAccountsResult>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    #[doc = "*Required features: `Security_Authentication_Web_Core`, `Foundation`, `Security_Credentials`*"]
    pub fn FindAllAccountsWithClientIdAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Credentials::WebAccountProvider>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(provider: Param0, clientid: Param1) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<FindAllAccountsResult>> {
        Self::IWebAuthenticationCoreManagerStatics4(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), provider.into_param().abi(), clientid.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<FindAllAccountsResult>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    #[doc = "*Required features: `Security_Authentication_Web_Core`, `Foundation`, `Security_Credentials`*"]
    pub fn FindSystemAccountProviderAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(webaccountproviderid: Param0) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>> {
        Self::IWebAuthenticationCoreManagerStatics4(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), webaccountproviderid.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
    #[doc = "*Required features: `Security_Authentication_Web_Core`, `Foundation`, `Security_Credentials`*"]
    pub fn FindSystemAccountProviderWithAuthorityAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(webaccountproviderid: Param0, authority: Param1) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>> {
        Self::IWebAuthenticationCoreManagerStatics4(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), webaccountproviderid.into_param().abi(), authority.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "System"))]
    #[doc = "*Required features: `Security_Authentication_Web_Core`, `Foundation`, `Security_Credentials`, `System`*"]
    pub fn FindSystemAccountProviderWithAuthorityForUserAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::super::System::User>>(webaccountproviderid: Param0, authority: Param1, user: Param2) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>> {
        Self::IWebAuthenticationCoreManagerStatics4(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), webaccountproviderid.into_param().abi(), authority.into_param().abi(), user.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<super::super::super::Credentials::WebAccountProvider>>(result__)
        })
    }
    pub fn IWebAuthenticationCoreManagerStatics<R, F: FnOnce(&IWebAuthenticationCoreManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<WebAuthenticationCoreManager, IWebAuthenticationCoreManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IWebAuthenticationCoreManagerStatics2<R, F: FnOnce(&IWebAuthenticationCoreManagerStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<WebAuthenticationCoreManager, IWebAuthenticationCoreManagerStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IWebAuthenticationCoreManagerStatics3<R, F: FnOnce(&IWebAuthenticationCoreManagerStatics3) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<WebAuthenticationCoreManager, IWebAuthenticationCoreManagerStatics3> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IWebAuthenticationCoreManagerStatics4<R, F: FnOnce(&IWebAuthenticationCoreManagerStatics4) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<WebAuthenticationCoreManager, IWebAuthenticationCoreManagerStatics4> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for WebAuthenticationCoreManager {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.WebAuthenticationCoreManager";
}
#[doc = "*Required features: `Security_Authentication_Web_Core`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct WebProviderError(pub ::windows::runtime::IInspectable);
impl WebProviderError {
    #[doc = "*Required features: `Security_Authentication_Web_Core`*"]
    pub fn ErrorCode(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Security_Authentication_Web_Core`*"]
    pub fn ErrorMessage(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Security_Authentication_Web_Core`, `Foundation_Collections`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `Security_Authentication_Web_Core`*"]
    pub fn Create<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(errorcode: u32, errormessage: Param1) -> ::windows::runtime::Result<WebProviderError> {
        Self::IWebProviderErrorFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), errorcode, errormessage.into_param().abi(), &mut result__).from_abi::<WebProviderError>(result__)
        })
    }
    pub fn IWebProviderErrorFactory<R, F: FnOnce(&IWebProviderErrorFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<WebProviderError, IWebProviderErrorFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for WebProviderError {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Web.Core.WebProviderError;{db191bb1-50c5-4809-8dca-09c99410245c})");
}
unsafe impl ::windows::runtime::Interface for WebProviderError {
    type Vtable = IWebProviderError_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3675855793, 20677, 18441, [141, 202, 9, 201, 148, 16, 36, 92]);
}
impl ::windows::runtime::RuntimeName for WebProviderError {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.WebProviderError";
}
impl ::std::convert::From<WebProviderError> for ::windows::runtime::IUnknown {
    fn from(value: WebProviderError) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&WebProviderError> for ::windows::runtime::IUnknown {
    fn from(value: &WebProviderError) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for WebProviderError {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a WebProviderError {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<WebProviderError> for ::windows::runtime::IInspectable {
    fn from(value: WebProviderError) -> Self {
        value.0
    }
}
impl ::std::convert::From<&WebProviderError> for ::windows::runtime::IInspectable {
    fn from(value: &WebProviderError) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for WebProviderError {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a WebProviderError {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for WebProviderError {}
unsafe impl ::std::marker::Sync for WebProviderError {}
#[doc = "*Required features: `Security_Authentication_Web_Core`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct WebTokenRequest(pub ::windows::runtime::IInspectable);
impl WebTokenRequest {
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `Security_Authentication_Web_Core`, `Security_Credentials`*"]
    pub fn WebAccountProvider(&self) -> ::windows::runtime::Result<super::super::super::Credentials::WebAccountProvider> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Credentials::WebAccountProvider>(result__)
        }
    }
    #[doc = "*Required features: `Security_Authentication_Web_Core`*"]
    pub fn Scope(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_Authentication_Web_Core`*"]
    pub fn ClientId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_Authentication_Web_Core`*"]
    pub fn PromptType(&self) -> ::windows::runtime::Result<WebTokenRequestPromptType> {
        let this = self;
        unsafe {
            let mut result__: WebTokenRequestPromptType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<WebTokenRequestPromptType>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Security_Authentication_Web_Core`, `Foundation_Collections`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `Security_Authentication_Web_Core`, `Security_Credentials`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Credentials::WebAccountProvider>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(provider: Param0, scope: Param1, clientid: Param2) -> ::windows::runtime::Result<WebTokenRequest> {
        Self::IWebTokenRequestFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), provider.into_param().abi(), scope.into_param().abi(), clientid.into_param().abi(), &mut result__).from_abi::<WebTokenRequest>(result__)
        })
    }
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `Security_Authentication_Web_Core`, `Security_Credentials`*"]
    pub fn CreateWithPromptType<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Credentials::WebAccountProvider>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(provider: Param0, scope: Param1, clientid: Param2, prompttype: WebTokenRequestPromptType) -> ::windows::runtime::Result<WebTokenRequest> {
        Self::IWebTokenRequestFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), provider.into_param().abi(), scope.into_param().abi(), clientid.into_param().abi(), prompttype, &mut result__).from_abi::<WebTokenRequest>(result__)
        })
    }
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `Security_Authentication_Web_Core`, `Security_Credentials`*"]
    pub fn CreateWithProvider<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Credentials::WebAccountProvider>>(provider: Param0) -> ::windows::runtime::Result<WebTokenRequest> {
        Self::IWebTokenRequestFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), provider.into_param().abi(), &mut result__).from_abi::<WebTokenRequest>(result__)
        })
    }
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `Security_Authentication_Web_Core`, `Security_Credentials`*"]
    pub fn CreateWithScope<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Credentials::WebAccountProvider>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(provider: Param0, scope: Param1) -> ::windows::runtime::Result<WebTokenRequest> {
        Self::IWebTokenRequestFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), provider.into_param().abi(), scope.into_param().abi(), &mut result__).from_abi::<WebTokenRequest>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Security_Authentication_Web_Core`, `Foundation_Collections`*"]
    pub fn AppProperties(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> {
        let this = &::windows::runtime::Interface::cast::<IWebTokenRequest2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `Security_Authentication_Web_Core`*"]
    pub fn CorrelationId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IWebTokenRequest3>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_Authentication_Web_Core`*"]
    pub fn SetCorrelationId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IWebTokenRequest3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn IWebTokenRequestFactory<R, F: FnOnce(&IWebTokenRequestFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<WebTokenRequest, IWebTokenRequestFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for WebTokenRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Web.Core.WebTokenRequest;{b77b4d68-adcb-4673-b364-0cf7b35caf97})");
}
unsafe impl ::windows::runtime::Interface for WebTokenRequest {
    type Vtable = IWebTokenRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3078311272, 44491, 18035, [179, 100, 12, 247, 179, 92, 175, 151]);
}
impl ::windows::runtime::RuntimeName for WebTokenRequest {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.WebTokenRequest";
}
impl ::std::convert::From<WebTokenRequest> for ::windows::runtime::IUnknown {
    fn from(value: WebTokenRequest) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&WebTokenRequest> for ::windows::runtime::IUnknown {
    fn from(value: &WebTokenRequest) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for WebTokenRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a WebTokenRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<WebTokenRequest> for ::windows::runtime::IInspectable {
    fn from(value: WebTokenRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<&WebTokenRequest> for ::windows::runtime::IInspectable {
    fn from(value: &WebTokenRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for WebTokenRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a WebTokenRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for WebTokenRequest {}
unsafe impl ::std::marker::Sync for WebTokenRequest {}
#[doc = "*Required features: `Security_Authentication_Web_Core`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WebTokenRequestPromptType(pub i32);
impl WebTokenRequestPromptType {
    pub const Default: WebTokenRequestPromptType = WebTokenRequestPromptType(0i32);
    pub const ForceAuthentication: WebTokenRequestPromptType = WebTokenRequestPromptType(1i32);
}
impl ::std::convert::From<i32> for WebTokenRequestPromptType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WebTokenRequestPromptType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for WebTokenRequestPromptType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Web.Core.WebTokenRequestPromptType;i4)");
}
impl ::windows::runtime::DefaultType for WebTokenRequestPromptType {
    type DefaultType = Self;
}
#[doc = "*Required features: `Security_Authentication_Web_Core`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct WebTokenRequestResult(pub ::windows::runtime::IInspectable);
impl WebTokenRequestResult {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Security_Authentication_Web_Core`, `Foundation_Collections`*"]
    pub fn ResponseData(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Collections::IVectorView<WebTokenResponse>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<WebTokenResponse>>(result__)
        }
    }
    #[doc = "*Required features: `Security_Authentication_Web_Core`*"]
    pub fn ResponseStatus(&self) -> ::windows::runtime::Result<WebTokenRequestStatus> {
        let this = self;
        unsafe {
            let mut result__: WebTokenRequestStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<WebTokenRequestStatus>(result__)
        }
    }
    #[doc = "*Required features: `Security_Authentication_Web_Core`*"]
    pub fn ResponseError(&self) -> ::windows::runtime::Result<WebProviderError> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<WebProviderError>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_Web_Core`, `Foundation`*"]
    pub fn InvalidateCacheAsync(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for WebTokenRequestResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Web.Core.WebTokenRequestResult;{c12a8305-d1f8-4483-8d54-38fe292784ff})");
}
unsafe impl ::windows::runtime::Interface for WebTokenRequestResult {
    type Vtable = IWebTokenRequestResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3240788741, 53752, 17539, [141, 84, 56, 254, 41, 39, 132, 255]);
}
impl ::windows::runtime::RuntimeName for WebTokenRequestResult {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.WebTokenRequestResult";
}
impl ::std::convert::From<WebTokenRequestResult> for ::windows::runtime::IUnknown {
    fn from(value: WebTokenRequestResult) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&WebTokenRequestResult> for ::windows::runtime::IUnknown {
    fn from(value: &WebTokenRequestResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for WebTokenRequestResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a WebTokenRequestResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<WebTokenRequestResult> for ::windows::runtime::IInspectable {
    fn from(value: WebTokenRequestResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&WebTokenRequestResult> for ::windows::runtime::IInspectable {
    fn from(value: &WebTokenRequestResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for WebTokenRequestResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a WebTokenRequestResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for WebTokenRequestResult {}
unsafe impl ::std::marker::Sync for WebTokenRequestResult {}
#[doc = "*Required features: `Security_Authentication_Web_Core`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WebTokenRequestStatus(pub i32);
impl WebTokenRequestStatus {
    pub const Success: WebTokenRequestStatus = WebTokenRequestStatus(0i32);
    pub const UserCancel: WebTokenRequestStatus = WebTokenRequestStatus(1i32);
    pub const AccountSwitch: WebTokenRequestStatus = WebTokenRequestStatus(2i32);
    pub const UserInteractionRequired: WebTokenRequestStatus = WebTokenRequestStatus(3i32);
    pub const AccountProviderNotAvailable: WebTokenRequestStatus = WebTokenRequestStatus(4i32);
    pub const ProviderError: WebTokenRequestStatus = WebTokenRequestStatus(5i32);
}
impl ::std::convert::From<i32> for WebTokenRequestStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WebTokenRequestStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for WebTokenRequestStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Web.Core.WebTokenRequestStatus;i4)");
}
impl ::windows::runtime::DefaultType for WebTokenRequestStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Security_Authentication_Web_Core`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct WebTokenResponse(pub ::windows::runtime::IInspectable);
impl WebTokenResponse {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<WebTokenResponse, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Security_Authentication_Web_Core`*"]
    pub fn Token(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_Authentication_Web_Core`*"]
    pub fn ProviderError(&self) -> ::windows::runtime::Result<WebProviderError> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<WebProviderError>(result__)
        }
    }
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `Security_Authentication_Web_Core`, `Security_Credentials`*"]
    pub fn WebAccount(&self) -> ::windows::runtime::Result<super::super::super::Credentials::WebAccount> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Credentials::WebAccount>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Security_Authentication_Web_Core`, `Foundation_Collections`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `Security_Authentication_Web_Core`*"]
    pub fn CreateWithToken<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(token: Param0) -> ::windows::runtime::Result<WebTokenResponse> {
        Self::IWebTokenResponseFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), token.into_param().abi(), &mut result__).from_abi::<WebTokenResponse>(result__)
        })
    }
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `Security_Authentication_Web_Core`, `Security_Credentials`*"]
    pub fn CreateWithTokenAndAccount<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Credentials::WebAccount>>(token: Param0, webaccount: Param1) -> ::windows::runtime::Result<WebTokenResponse> {
        Self::IWebTokenResponseFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi(), webaccount.into_param().abi(), &mut result__).from_abi::<WebTokenResponse>(result__)
        })
    }
    #[cfg(feature = "Security_Credentials")]
    #[doc = "*Required features: `Security_Authentication_Web_Core`, `Security_Credentials`*"]
    pub fn CreateWithTokenAccountAndError<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Credentials::WebAccount>, Param2: ::windows::runtime::IntoParam<'a, WebProviderError>>(token: Param0, webaccount: Param1, error: Param2) -> ::windows::runtime::Result<WebTokenResponse> {
        Self::IWebTokenResponseFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), token.into_param().abi(), webaccount.into_param().abi(), error.into_param().abi(), &mut result__).from_abi::<WebTokenResponse>(result__)
        })
    }
    pub fn IWebTokenResponseFactory<R, F: FnOnce(&IWebTokenResponseFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<WebTokenResponse, IWebTokenResponseFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for WebTokenResponse {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Web.Core.WebTokenResponse;{67a7c5ca-83f6-44c6-a3b1-0eb69e41fa8a})");
}
unsafe impl ::windows::runtime::Interface for WebTokenResponse {
    type Vtable = IWebTokenResponse_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1739048394, 33782, 17606, [163, 177, 14, 182, 158, 65, 250, 138]);
}
impl ::windows::runtime::RuntimeName for WebTokenResponse {
    const NAME: &'static str = "Windows.Security.Authentication.Web.Core.WebTokenResponse";
}
impl ::std::convert::From<WebTokenResponse> for ::windows::runtime::IUnknown {
    fn from(value: WebTokenResponse) -> Self {
        value.0 .0
    }
}
impl ::std::convert::From<&WebTokenResponse> for ::windows::runtime::IUnknown {
    fn from(value: &WebTokenResponse) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for WebTokenResponse {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a WebTokenResponse {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::std::convert::From<WebTokenResponse> for ::windows::runtime::IInspectable {
    fn from(value: WebTokenResponse) -> Self {
        value.0
    }
}
impl ::std::convert::From<&WebTokenResponse> for ::windows::runtime::IInspectable {
    fn from(value: &WebTokenResponse) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for WebTokenResponse {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a WebTokenResponse {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for WebTokenResponse {}
unsafe impl ::std::marker::Sync for WebTokenResponse {}

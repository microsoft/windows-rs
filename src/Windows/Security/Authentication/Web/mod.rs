#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Security_Authentication_Web_Core")]
pub mod Core;
#[cfg(feature = "Security_Authentication_Web_Provider")]
pub mod Provider;
#[repr(transparent)]
#[doc(hidden)]
pub struct IWebAuthenticationBrokerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebAuthenticationBrokerStatics {
    type Vtable = IWebAuthenticationBrokerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(789880602, 58995, 16565, [188, 34, 32, 26, 104, 100, 163, 123]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAuthenticationBrokerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, options: WebAuthenticationOptions, requesturi: ::windows::runtime::RawPtr, callbackuri: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, options: WebAuthenticationOptions, requesturi: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWebAuthenticationBrokerStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebAuthenticationBrokerStatics2 {
    type Vtable = IWebAuthenticationBrokerStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1942879134, 5351, 16858, [169, 113, 170, 244, 65, 11, 98, 30]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAuthenticationBrokerStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, requesturi: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, requesturi: ::windows::runtime::RawPtr, callbackuri: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, requesturi: ::windows::runtime::RawPtr, callbackuri: ::windows::runtime::RawPtr, continuationdata: ::windows::runtime::RawPtr, options: WebAuthenticationOptions) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, requesturi: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, requesturi: ::windows::runtime::RawPtr, options: WebAuthenticationOptions, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWebAuthenticationResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebAuthenticationResult {
    type Vtable = IWebAuthenticationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1677732683, 60905, 18186, [165, 205, 3, 35, 250, 246, 226, 98]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAuthenticationResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut WebAuthenticationStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Security_Authentication_Web`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TokenBindingKeyType(pub i32);
impl TokenBindingKeyType {
    pub const Rsa2048: TokenBindingKeyType = TokenBindingKeyType(0i32);
    pub const EcdsaP256: TokenBindingKeyType = TokenBindingKeyType(1i32);
    pub const AnyExisting: TokenBindingKeyType = TokenBindingKeyType(2i32);
}
impl ::std::convert::From<i32> for TokenBindingKeyType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TokenBindingKeyType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for TokenBindingKeyType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Web.TokenBindingKeyType;i4)");
}
impl ::windows::runtime::DefaultType for TokenBindingKeyType {
    type DefaultType = Self;
}
#[doc = "*Required features: `Security_Authentication_Web`*"]
pub struct WebAuthenticationBroker {}
impl WebAuthenticationBroker {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_Web`, `Foundation`*"]
    pub fn AuthenticateWithCallbackUriAsync<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>>(options: WebAuthenticationOptions, requesturi: Param1, callbackuri: Param2) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<WebAuthenticationResult>> {
        Self::IWebAuthenticationBrokerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), options, requesturi.into_param().abi(), callbackuri.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<WebAuthenticationResult>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_Web`, `Foundation`*"]
    pub fn AuthenticateWithoutCallbackUriAsync<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>>(options: WebAuthenticationOptions, requesturi: Param1) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<WebAuthenticationResult>> {
        Self::IWebAuthenticationBrokerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), options, requesturi.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<WebAuthenticationResult>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_Web`, `Foundation`*"]
    pub fn GetCurrentApplicationCallbackUri() -> ::windows::runtime::Result<super::super::super::Foundation::Uri> {
        Self::IWebAuthenticationBrokerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_Web`, `Foundation`*"]
    pub fn AuthenticateAndContinue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>>(requesturi: Param0) -> ::windows::runtime::Result<()> {
        Self::IWebAuthenticationBrokerStatics2(|this| unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), requesturi.into_param().abi()).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_Web`, `Foundation`*"]
    pub fn AuthenticateWithCallbackUriAndContinue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>>(requesturi: Param0, callbackuri: Param1) -> ::windows::runtime::Result<()> {
        Self::IWebAuthenticationBrokerStatics2(|this| unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), requesturi.into_param().abi(), callbackuri.into_param().abi()).ok() })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Security_Authentication_Web`, `Foundation`, `Foundation_Collections`*"]
    pub fn AuthenticateWithCallbackUriContinuationDataAndOptionsAndContinue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::ValueSet>>(requesturi: Param0, callbackuri: Param1, continuationdata: Param2, options: WebAuthenticationOptions) -> ::windows::runtime::Result<()> {
        Self::IWebAuthenticationBrokerStatics2(|this| unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), requesturi.into_param().abi(), callbackuri.into_param().abi(), continuationdata.into_param().abi(), options).ok() })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_Web`, `Foundation`*"]
    pub fn AuthenticateSilentlyAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>>(requesturi: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<WebAuthenticationResult>> {
        Self::IWebAuthenticationBrokerStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), requesturi.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<WebAuthenticationResult>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Security_Authentication_Web`, `Foundation`*"]
    pub fn AuthenticateSilentlyWithOptionsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Uri>>(requesturi: Param0, options: WebAuthenticationOptions) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<WebAuthenticationResult>> {
        Self::IWebAuthenticationBrokerStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), requesturi.into_param().abi(), options, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<WebAuthenticationResult>>(result__)
        })
    }
    pub fn IWebAuthenticationBrokerStatics<R, F: FnOnce(&IWebAuthenticationBrokerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<WebAuthenticationBroker, IWebAuthenticationBrokerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IWebAuthenticationBrokerStatics2<R, F: FnOnce(&IWebAuthenticationBrokerStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<WebAuthenticationBroker, IWebAuthenticationBrokerStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for WebAuthenticationBroker {
    const NAME: &'static str = "Windows.Security.Authentication.Web.WebAuthenticationBroker";
}
#[doc = "*Required features: `Security_Authentication_Web`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WebAuthenticationOptions(pub u32);
impl WebAuthenticationOptions {
    pub const None: WebAuthenticationOptions = WebAuthenticationOptions(0u32);
    pub const SilentMode: WebAuthenticationOptions = WebAuthenticationOptions(1u32);
    pub const UseTitle: WebAuthenticationOptions = WebAuthenticationOptions(2u32);
    pub const UseHttpPost: WebAuthenticationOptions = WebAuthenticationOptions(4u32);
    pub const UseCorporateNetwork: WebAuthenticationOptions = WebAuthenticationOptions(8u32);
}
impl ::std::convert::From<u32> for WebAuthenticationOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WebAuthenticationOptions {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for WebAuthenticationOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Web.WebAuthenticationOptions;u4)");
}
impl ::windows::runtime::DefaultType for WebAuthenticationOptions {
    type DefaultType = Self;
}
impl ::std::ops::BitOr for WebAuthenticationOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for WebAuthenticationOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for WebAuthenticationOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for WebAuthenticationOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for WebAuthenticationOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Security_Authentication_Web`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct WebAuthenticationResult(::windows::runtime::IInspectable);
impl WebAuthenticationResult {
    #[doc = "*Required features: `Security_Authentication_Web`*"]
    pub fn ResponseData(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Security_Authentication_Web`*"]
    pub fn ResponseStatus(&self) -> ::windows::runtime::Result<WebAuthenticationStatus> {
        let this = self;
        unsafe {
            let mut result__: WebAuthenticationStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<WebAuthenticationStatus>(result__)
        }
    }
    #[doc = "*Required features: `Security_Authentication_Web`*"]
    pub fn ResponseErrorDetail(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for WebAuthenticationResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Web.WebAuthenticationResult;{64002b4b-ede9-470a-a5cd-0323faf6e262})");
}
unsafe impl ::windows::runtime::Interface for WebAuthenticationResult {
    type Vtable = IWebAuthenticationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1677732683, 60905, 18186, [165, 205, 3, 35, 250, 246, 226, 98]);
}
impl ::windows::runtime::RuntimeName for WebAuthenticationResult {
    const NAME: &'static str = "Windows.Security.Authentication.Web.WebAuthenticationResult";
}
#[doc = "*Required features: `Security_Authentication_Web`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct WebAuthenticationStatus(pub i32);
impl WebAuthenticationStatus {
    pub const Success: WebAuthenticationStatus = WebAuthenticationStatus(0i32);
    pub const UserCancel: WebAuthenticationStatus = WebAuthenticationStatus(1i32);
    pub const ErrorHttp: WebAuthenticationStatus = WebAuthenticationStatus(2i32);
}
impl ::std::convert::From<i32> for WebAuthenticationStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WebAuthenticationStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for WebAuthenticationStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Web.WebAuthenticationStatus;i4)");
}
impl ::windows::runtime::DefaultType for WebAuthenticationStatus {
    type DefaultType = Self;
}

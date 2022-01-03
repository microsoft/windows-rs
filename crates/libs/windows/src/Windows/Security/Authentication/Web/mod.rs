#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Security_Authentication_Web_Core")]
pub mod Core;
#[cfg(feature = "Security_Authentication_Web_Provider")]
pub mod Provider;
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAuthenticationBrokerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAuthenticationBrokerStatics {
    type Vtable = IWebAuthenticationBrokerStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2f149f1a_e673_40b5_bc22_201a6864a37b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAuthenticationBrokerStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: WebAuthenticationOptions, requesturi: ::windows::core::RawPtr, callbackuri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: WebAuthenticationOptions, requesturi: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAuthenticationBrokerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAuthenticationBrokerStatics2 {
    type Vtable = IWebAuthenticationBrokerStatics2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73cdfb9e_14e7_41da_a971_aaf4410b621e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAuthenticationBrokerStatics2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requesturi: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requesturi: ::windows::core::RawPtr, callbackuri: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requesturi: ::windows::core::RawPtr, callbackuri: ::windows::core::RawPtr, continuationdata: ::windows::core::RawPtr, options: WebAuthenticationOptions) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requesturi: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requesturi: ::windows::core::RawPtr, options: WebAuthenticationOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAuthenticationResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAuthenticationResult {
    type Vtable = IWebAuthenticationResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64002b4b_ede9_470a_a5cd_0323faf6e262);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAuthenticationResultVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WebAuthenticationStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Security_Authentication_Web'*"]
#[repr(transparent)]
pub struct TokenBindingKeyType(pub i32);
impl TokenBindingKeyType {
    pub const Rsa2048: Self = Self(0i32);
    pub const EcdsaP256: Self = Self(1i32);
    pub const AnyExisting: Self = Self(2i32);
}
impl ::core::marker::Copy for TokenBindingKeyType {}
impl ::core::clone::Clone for TokenBindingKeyType {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TokenBindingKeyType {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TokenBindingKeyType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TokenBindingKeyType {}
impl ::core::fmt::Debug for TokenBindingKeyType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TokenBindingKeyType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TokenBindingKeyType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Web.TokenBindingKeyType;i4)");
}
impl ::windows::core::DefaultType for TokenBindingKeyType {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Security_Authentication_Web'*"]
pub struct WebAuthenticationBroker {}
impl WebAuthenticationBroker {
    #[doc = "*Required features: 'Security_Authentication_Web', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn AuthenticateWithCallbackUriAsync<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(options: WebAuthenticationOptions, requesturi: Param1, callbackuri: Param2) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WebAuthenticationResult>> {
        Self::IWebAuthenticationBrokerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), options, requesturi.into_param().abi(), callbackuri.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<WebAuthenticationResult>>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Authentication_Web', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn AuthenticateWithoutCallbackUriAsync<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(options: WebAuthenticationOptions, requesturi: Param1) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WebAuthenticationResult>> {
        Self::IWebAuthenticationBrokerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), options, requesturi.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<WebAuthenticationResult>>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Authentication_Web', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn GetCurrentApplicationCallbackUri() -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        Self::IWebAuthenticationBrokerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Authentication_Web', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn AuthenticateAndContinue<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(requesturi: Param0) -> ::windows::core::Result<()> {
        Self::IWebAuthenticationBrokerStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), requesturi.into_param().abi()).ok() })
    }
    #[doc = "*Required features: 'Security_Authentication_Web', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn AuthenticateWithCallbackUriAndContinue<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(requesturi: Param0, callbackuri: Param1) -> ::windows::core::Result<()> {
        Self::IWebAuthenticationBrokerStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), requesturi.into_param().abi(), callbackuri.into_param().abi()).ok() })
    }
    #[doc = "*Required features: 'Security_Authentication_Web', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn AuthenticateWithCallbackUriContinuationDataAndOptionsAndContinue<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::ValueSet>>(requesturi: Param0, callbackuri: Param1, continuationdata: Param2, options: WebAuthenticationOptions) -> ::windows::core::Result<()> {
        Self::IWebAuthenticationBrokerStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), requesturi.into_param().abi(), callbackuri.into_param().abi(), continuationdata.into_param().abi(), options).ok() })
    }
    #[doc = "*Required features: 'Security_Authentication_Web', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn AuthenticateSilentlyAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(requesturi: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WebAuthenticationResult>> {
        Self::IWebAuthenticationBrokerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), requesturi.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<WebAuthenticationResult>>(result__)
        })
    }
    #[doc = "*Required features: 'Security_Authentication_Web', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn AuthenticateSilentlyWithOptionsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(requesturi: Param0, options: WebAuthenticationOptions) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WebAuthenticationResult>> {
        Self::IWebAuthenticationBrokerStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), requesturi.into_param().abi(), options, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<WebAuthenticationResult>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWebAuthenticationBrokerStatics<R, F: FnOnce(&IWebAuthenticationBrokerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WebAuthenticationBroker, IWebAuthenticationBrokerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IWebAuthenticationBrokerStatics2<R, F: FnOnce(&IWebAuthenticationBrokerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WebAuthenticationBroker, IWebAuthenticationBrokerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for WebAuthenticationBroker {
    const NAME: &'static str = "Windows.Security.Authentication.Web.WebAuthenticationBroker";
}
#[doc = "*Required features: 'Security_Authentication_Web'*"]
#[repr(transparent)]
pub struct WebAuthenticationOptions(pub u32);
impl WebAuthenticationOptions {
    pub const None: Self = Self(0u32);
    pub const SilentMode: Self = Self(1u32);
    pub const UseTitle: Self = Self(2u32);
    pub const UseHttpPost: Self = Self(4u32);
    pub const UseCorporateNetwork: Self = Self(8u32);
}
impl ::core::marker::Copy for WebAuthenticationOptions {}
impl ::core::clone::Clone for WebAuthenticationOptions {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WebAuthenticationOptions {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WebAuthenticationOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebAuthenticationOptions {}
impl ::core::fmt::Debug for WebAuthenticationOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebAuthenticationOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for WebAuthenticationOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for WebAuthenticationOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for WebAuthenticationOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for WebAuthenticationOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for WebAuthenticationOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for WebAuthenticationOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Web.WebAuthenticationOptions;u4)");
}
impl ::windows::core::DefaultType for WebAuthenticationOptions {
    type DefaultType = Self;
}
#[doc = "*Required features: 'Security_Authentication_Web'*"]
#[repr(transparent)]
pub struct WebAuthenticationResult(::windows::core::IUnknown);
impl WebAuthenticationResult {
    #[doc = "*Required features: 'Security_Authentication_Web'*"]
    pub fn ResponseData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authentication_Web'*"]
    pub fn ResponseStatus(&self) -> ::windows::core::Result<WebAuthenticationStatus> {
        let this = self;
        unsafe {
            let mut result__: WebAuthenticationStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WebAuthenticationStatus>(result__)
        }
    }
    #[doc = "*Required features: 'Security_Authentication_Web'*"]
    pub fn ResponseErrorDetail(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for WebAuthenticationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebAuthenticationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebAuthenticationResult {}
impl ::core::fmt::Debug for WebAuthenticationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebAuthenticationResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebAuthenticationResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Web.WebAuthenticationResult;{64002b4b-ede9-470a-a5cd-0323faf6e262})");
}
unsafe impl ::windows::core::Interface for WebAuthenticationResult {
    type Vtable = IWebAuthenticationResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64002b4b_ede9_470a_a5cd_0323faf6e262);
}
impl ::windows::core::RuntimeName for WebAuthenticationResult {
    const NAME: &'static str = "Windows.Security.Authentication.Web.WebAuthenticationResult";
}
impl ::core::convert::From<WebAuthenticationResult> for ::windows::core::IUnknown {
    fn from(value: WebAuthenticationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebAuthenticationResult> for ::windows::core::IUnknown {
    fn from(value: &WebAuthenticationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebAuthenticationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &WebAuthenticationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebAuthenticationResult> for ::windows::core::IInspectable {
    fn from(value: WebAuthenticationResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebAuthenticationResult> for ::windows::core::IInspectable {
    fn from(value: &WebAuthenticationResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebAuthenticationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &WebAuthenticationResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: 'Security_Authentication_Web'*"]
#[repr(transparent)]
pub struct WebAuthenticationStatus(pub i32);
impl WebAuthenticationStatus {
    pub const Success: Self = Self(0i32);
    pub const UserCancel: Self = Self(1i32);
    pub const ErrorHttp: Self = Self(2i32);
}
impl ::core::marker::Copy for WebAuthenticationStatus {}
impl ::core::clone::Clone for WebAuthenticationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WebAuthenticationStatus {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WebAuthenticationStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebAuthenticationStatus {}
impl ::core::fmt::Debug for WebAuthenticationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebAuthenticationStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebAuthenticationStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Web.WebAuthenticationStatus;i4)");
}
impl ::windows::core::DefaultType for WebAuthenticationStatus {
    type DefaultType = Self;
}

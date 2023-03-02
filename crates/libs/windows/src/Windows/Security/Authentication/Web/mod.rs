#[cfg(feature = "Security_Authentication_Web_Core")]
pub mod Core;
#[cfg(feature = "Security_Authentication_Web_Provider")]
pub mod Provider;
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAuthenticationBrokerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAuthenticationBrokerStatics {
    type Vtable = IWebAuthenticationBrokerStatics_Vtbl;
}
impl ::core::clone::Clone for IWebAuthenticationBrokerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWebAuthenticationBrokerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2f149f1a_e673_40b5_bc22_201a6864a37b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAuthenticationBrokerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub AuthenticateWithCallbackUriAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: WebAuthenticationOptions, requesturi: *mut ::core::ffi::c_void, callbackuri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AuthenticateWithCallbackUriAsync: usize,
    #[cfg(feature = "Foundation")]
    pub AuthenticateWithoutCallbackUriAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: WebAuthenticationOptions, requesturi: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AuthenticateWithoutCallbackUriAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetCurrentApplicationCallbackUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCurrentApplicationCallbackUri: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAuthenticationBrokerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAuthenticationBrokerStatics2 {
    type Vtable = IWebAuthenticationBrokerStatics2_Vtbl;
}
impl ::core::clone::Clone for IWebAuthenticationBrokerStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWebAuthenticationBrokerStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73cdfb9e_14e7_41da_a971_aaf4410b621e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAuthenticationBrokerStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub AuthenticateAndContinue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requesturi: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AuthenticateAndContinue: usize,
    #[cfg(feature = "Foundation")]
    pub AuthenticateWithCallbackUriAndContinue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requesturi: *mut ::core::ffi::c_void, callbackuri: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AuthenticateWithCallbackUriAndContinue: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub AuthenticateWithCallbackUriContinuationDataAndOptionsAndContinue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requesturi: *mut ::core::ffi::c_void, callbackuri: *mut ::core::ffi::c_void, continuationdata: *mut ::core::ffi::c_void, options: WebAuthenticationOptions) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AuthenticateWithCallbackUriContinuationDataAndOptionsAndContinue: usize,
    #[cfg(feature = "Foundation")]
    pub AuthenticateSilentlyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requesturi: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AuthenticateSilentlyAsync: usize,
    #[cfg(feature = "Foundation")]
    pub AuthenticateSilentlyWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requesturi: *mut ::core::ffi::c_void, options: WebAuthenticationOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AuthenticateSilentlyWithOptionsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebAuthenticationResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebAuthenticationResult {
    type Vtable = IWebAuthenticationResult_Vtbl;
}
impl ::core::clone::Clone for IWebAuthenticationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWebAuthenticationResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64002b4b_ede9_470a_a5cd_0323faf6e262);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAuthenticationResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ResponseData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ResponseStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WebAuthenticationStatus) -> ::windows::core::HRESULT,
    pub ResponseErrorDetail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Security_Authentication_Web\"`*"]
pub struct WebAuthenticationBroker;
impl WebAuthenticationBroker {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AuthenticateWithCallbackUriAsync(options: WebAuthenticationOptions, requesturi: &super::super::super::Foundation::Uri, callbackuri: &super::super::super::Foundation::Uri) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WebAuthenticationResult>> {
        Self::IWebAuthenticationBrokerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<WebAuthenticationResult>>();
            (::windows::core::Interface::vtable(this).AuthenticateWithCallbackUriAsync)(::windows::core::Interface::as_raw(this), options, ::core::mem::transmute_copy(requesturi), ::core::mem::transmute_copy(callbackuri), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AuthenticateWithoutCallbackUriAsync(options: WebAuthenticationOptions, requesturi: &super::super::super::Foundation::Uri) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WebAuthenticationResult>> {
        Self::IWebAuthenticationBrokerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<WebAuthenticationResult>>();
            (::windows::core::Interface::vtable(this).AuthenticateWithoutCallbackUriAsync)(::windows::core::Interface::as_raw(this), options, ::core::mem::transmute_copy(requesturi), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetCurrentApplicationCallbackUri() -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        Self::IWebAuthenticationBrokerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Uri>();
            (::windows::core::Interface::vtable(this).GetCurrentApplicationCallbackUri)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AuthenticateAndContinue(requesturi: &super::super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        Self::IWebAuthenticationBrokerStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).AuthenticateAndContinue)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(requesturi)).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AuthenticateWithCallbackUriAndContinue(requesturi: &super::super::super::Foundation::Uri, callbackuri: &super::super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        Self::IWebAuthenticationBrokerStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).AuthenticateWithCallbackUriAndContinue)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(requesturi), ::core::mem::transmute_copy(callbackuri)).ok() })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AuthenticateWithCallbackUriContinuationDataAndOptionsAndContinue(requesturi: &super::super::super::Foundation::Uri, callbackuri: &super::super::super::Foundation::Uri, continuationdata: &super::super::super::Foundation::Collections::ValueSet, options: WebAuthenticationOptions) -> ::windows::core::Result<()> {
        Self::IWebAuthenticationBrokerStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).AuthenticateWithCallbackUriContinuationDataAndOptionsAndContinue)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(requesturi), ::core::mem::transmute_copy(callbackuri), ::core::mem::transmute_copy(continuationdata), options).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AuthenticateSilentlyAsync(requesturi: &super::super::super::Foundation::Uri) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WebAuthenticationResult>> {
        Self::IWebAuthenticationBrokerStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<WebAuthenticationResult>>();
            (::windows::core::Interface::vtable(this).AuthenticateSilentlyAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(requesturi), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AuthenticateSilentlyWithOptionsAsync(requesturi: &super::super::super::Foundation::Uri, options: WebAuthenticationOptions) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<WebAuthenticationResult>> {
        Self::IWebAuthenticationBrokerStatics2(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::IAsyncOperation<WebAuthenticationResult>>();
            (::windows::core::Interface::vtable(this).AuthenticateSilentlyWithOptionsAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(requesturi), options, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWebAuthenticationBrokerStatics<R, F: FnOnce(&IWebAuthenticationBrokerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<WebAuthenticationBroker, IWebAuthenticationBrokerStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IWebAuthenticationBrokerStatics2<R, F: FnOnce(&IWebAuthenticationBrokerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<WebAuthenticationBroker, IWebAuthenticationBrokerStatics2> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for WebAuthenticationBroker {
    const NAME: &'static str = "Windows.Security.Authentication.Web.WebAuthenticationBroker";
}
#[doc = "*Required features: `\"Security_Authentication_Web\"`*"]
#[repr(transparent)]
pub struct WebAuthenticationResult(::windows::core::IUnknown);
impl WebAuthenticationResult {
    pub fn ResponseData(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).ResponseData)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ResponseStatus(&self) -> ::windows::core::Result<WebAuthenticationStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<WebAuthenticationStatus>();
            (::windows::core::Interface::vtable(this).ResponseStatus)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ResponseErrorDetail(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).ResponseErrorDetail)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows::core::RuntimeType for WebAuthenticationResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Security.Authentication.Web.WebAuthenticationResult;{64002b4b-ede9-470a-a5cd-0323faf6e262})");
}
impl ::core::clone::Clone for WebAuthenticationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for WebAuthenticationResult {
    type Vtable = IWebAuthenticationResult_Vtbl;
}
unsafe impl ::windows::core::ComInterface for WebAuthenticationResult {
    const IID: ::windows::core::GUID = <IWebAuthenticationResult as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for WebAuthenticationResult {
    const NAME: &'static str = "Windows.Security.Authentication.Web.WebAuthenticationResult";
}
::windows::imp::interface_hierarchy!(WebAuthenticationResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"Security_Authentication_Web\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for TokenBindingKeyType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for TokenBindingKeyType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TokenBindingKeyType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TokenBindingKeyType").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for TokenBindingKeyType {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Web.TokenBindingKeyType;i4)");
}
#[doc = "*Required features: `\"Security_Authentication_Web\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for WebAuthenticationOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WebAuthenticationOptions {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WebAuthenticationOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebAuthenticationOptions").field(&self.0).finish()
    }
}
impl WebAuthenticationOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
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
impl ::windows::core::RuntimeType for WebAuthenticationOptions {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Web.WebAuthenticationOptions;u4)");
}
#[doc = "*Required features: `\"Security_Authentication_Web\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for WebAuthenticationStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WebAuthenticationStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WebAuthenticationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebAuthenticationStatus").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for WebAuthenticationStatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Security.Authentication.Web.WebAuthenticationStatus;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

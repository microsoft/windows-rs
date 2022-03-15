#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "ApplicationModel_ExtendedExecution_Foreground")]
pub mod Foreground;
#[doc = "*Required features: `\"ApplicationModel_ExtendedExecution\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ExtendedExecutionReason(pub i32);
impl ExtendedExecutionReason {
    pub const Unspecified: Self = Self(0i32);
    pub const LocationTracking: Self = Self(1i32);
    pub const SavingData: Self = Self(2i32);
}
impl ::core::marker::Copy for ExtendedExecutionReason {}
impl ::core::clone::Clone for ExtendedExecutionReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ExtendedExecutionReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ExtendedExecutionReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for ExtendedExecutionReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExtendedExecutionReason").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ExtendedExecutionReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ExtendedExecution.ExtendedExecutionReason;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_ExtendedExecution\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ExtendedExecutionResult(pub i32);
impl ExtendedExecutionResult {
    pub const Allowed: Self = Self(0i32);
    pub const Denied: Self = Self(1i32);
}
impl ::core::marker::Copy for ExtendedExecutionResult {}
impl ::core::clone::Clone for ExtendedExecutionResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ExtendedExecutionResult {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ExtendedExecutionResult {
    type Abi = Self;
}
impl ::core::fmt::Debug for ExtendedExecutionResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExtendedExecutionResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ExtendedExecutionResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ExtendedExecution.ExtendedExecutionResult;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_ExtendedExecution\"`*"]
#[repr(transparent)]
pub struct ExtendedExecutionRevokedEventArgs(::windows::core::IUnknown);
impl ExtendedExecutionRevokedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_ExtendedExecution\"`*"]
    pub fn Reason(&self) -> ::windows::core::Result<ExtendedExecutionRevokedReason> {
        let this = self;
        unsafe {
            let mut result__: ExtendedExecutionRevokedReason = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Reason)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ExtendedExecutionRevokedReason>(result__)
        }
    }
}
impl ::core::clone::Clone for ExtendedExecutionRevokedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ExtendedExecutionRevokedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ExtendedExecutionRevokedEventArgs {}
impl ::core::fmt::Debug for ExtendedExecutionRevokedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExtendedExecutionRevokedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ExtendedExecutionRevokedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ExtendedExecution.ExtendedExecutionRevokedEventArgs;{bfbc9f16-63b5-4c0b-aad6-828af5373ec3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ExtendedExecutionRevokedEventArgs {
    type Vtable = IExtendedExecutionRevokedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IExtendedExecutionRevokedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ExtendedExecutionRevokedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.ExtendedExecution.ExtendedExecutionRevokedEventArgs";
}
impl ::core::convert::From<ExtendedExecutionRevokedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ExtendedExecutionRevokedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ExtendedExecutionRevokedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ExtendedExecutionRevokedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ExtendedExecutionRevokedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ExtendedExecutionRevokedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ExtendedExecutionRevokedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ExtendedExecutionRevokedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ExtendedExecutionRevokedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ExtendedExecutionRevokedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ExtendedExecutionRevokedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ExtendedExecutionRevokedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ExtendedExecutionRevokedEventArgs {}
unsafe impl ::core::marker::Sync for ExtendedExecutionRevokedEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_ExtendedExecution\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ExtendedExecutionRevokedReason(pub i32);
impl ExtendedExecutionRevokedReason {
    pub const Resumed: Self = Self(0i32);
    pub const SystemPolicy: Self = Self(1i32);
}
impl ::core::marker::Copy for ExtendedExecutionRevokedReason {}
impl ::core::clone::Clone for ExtendedExecutionRevokedReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ExtendedExecutionRevokedReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ExtendedExecutionRevokedReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for ExtendedExecutionRevokedReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExtendedExecutionRevokedReason").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ExtendedExecutionRevokedReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ExtendedExecution.ExtendedExecutionRevokedReason;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"ApplicationModel_ExtendedExecution\"`*"]
#[repr(transparent)]
pub struct ExtendedExecutionSession(::windows::core::IUnknown);
impl ExtendedExecutionSession {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ExtendedExecutionSession, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"ApplicationModel_ExtendedExecution\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_ExtendedExecution\"`*"]
    pub fn Reason(&self) -> ::windows::core::Result<ExtendedExecutionReason> {
        let this = self;
        unsafe {
            let mut result__: ExtendedExecutionReason = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Reason)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ExtendedExecutionReason>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ExtendedExecution\"`*"]
    pub fn SetReason(&self, value: ExtendedExecutionReason) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetReason)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_ExtendedExecution\"`*"]
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Description)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ExtendedExecution\"`*"]
    pub fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDescription)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_ExtendedExecution\"`*"]
    pub fn PercentProgress(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PercentProgress)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ExtendedExecution\"`*"]
    pub fn SetPercentProgress(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPercentProgress)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_ExtendedExecution\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Revoked<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, ExtendedExecutionRevokedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Revoked)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_ExtendedExecution\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveRevoked<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveRevoked)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_ExtendedExecution\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RequestExtensionAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ExtendedExecutionResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RequestExtensionAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ExtendedExecutionResult>>(result__)
        }
    }
}
impl ::core::clone::Clone for ExtendedExecutionSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ExtendedExecutionSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ExtendedExecutionSession {}
impl ::core::fmt::Debug for ExtendedExecutionSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExtendedExecutionSession").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ExtendedExecutionSession {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ExtendedExecution.ExtendedExecutionSession;{af908a2d-118b-48f1-9308-0c4fc41e200f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ExtendedExecutionSession {
    type Vtable = IExtendedExecutionSession_Vtbl;
    const IID: ::windows::core::GUID = <IExtendedExecutionSession as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ExtendedExecutionSession {
    const NAME: &'static str = "Windows.ApplicationModel.ExtendedExecution.ExtendedExecutionSession";
}
impl ::core::convert::From<ExtendedExecutionSession> for ::windows::core::IUnknown {
    fn from(value: ExtendedExecutionSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ExtendedExecutionSession> for ::windows::core::IUnknown {
    fn from(value: &ExtendedExecutionSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ExtendedExecutionSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ExtendedExecutionSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ExtendedExecutionSession> for ::windows::core::IInspectable {
    fn from(value: ExtendedExecutionSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ExtendedExecutionSession> for ::windows::core::IInspectable {
    fn from(value: &ExtendedExecutionSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ExtendedExecutionSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ExtendedExecutionSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<ExtendedExecutionSession> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: ExtendedExecutionSession) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&ExtendedExecutionSession> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &ExtendedExecutionSession) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for ExtendedExecutionSession {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &ExtendedExecutionSession {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ExtendedExecutionSession {}
unsafe impl ::core::marker::Sync for ExtendedExecutionSession {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IExtendedExecutionRevokedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IExtendedExecutionRevokedEventArgs {
    type Vtable = IExtendedExecutionRevokedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbfbc9f16_63b5_4c0b_aad6_828af5373ec3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExtendedExecutionRevokedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Reason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ExtendedExecutionRevokedReason) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IExtendedExecutionSession(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IExtendedExecutionSession {
    type Vtable = IExtendedExecutionSession_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf908a2d_118b_48f1_9308_0c4fc41e200f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExtendedExecutionSession_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Reason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ExtendedExecutionReason) -> ::windows::core::HRESULT,
    pub SetReason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ExtendedExecutionReason) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PercentProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetPercentProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Revoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Revoked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveRevoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveRevoked: usize,
    #[cfg(feature = "Foundation")]
    pub RequestExtensionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RequestExtensionAsync: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

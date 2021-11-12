#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "ApplicationModel_ExtendedExecution_Foreground")]
pub mod Foreground;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ExtendedExecutionReason(pub i32);
impl ExtendedExecutionReason {
    pub const Unspecified: ExtendedExecutionReason = ExtendedExecutionReason(0i32);
    pub const LocationTracking: ExtendedExecutionReason = ExtendedExecutionReason(1i32);
    pub const SavingData: ExtendedExecutionReason = ExtendedExecutionReason(2i32);
}
impl ::core::convert::From<i32> for ExtendedExecutionReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ExtendedExecutionReason {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ExtendedExecutionReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ExtendedExecution.ExtendedExecutionReason;i4)");
}
impl ::windows::core::DefaultType for ExtendedExecutionReason {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ExtendedExecutionResult(pub i32);
impl ExtendedExecutionResult {
    pub const Allowed: ExtendedExecutionResult = ExtendedExecutionResult(0i32);
    pub const Denied: ExtendedExecutionResult = ExtendedExecutionResult(1i32);
}
impl ::core::convert::From<i32> for ExtendedExecutionResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ExtendedExecutionResult {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ExtendedExecutionResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ExtendedExecution.ExtendedExecutionResult;i4)");
}
impl ::windows::core::DefaultType for ExtendedExecutionResult {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ExtendedExecutionRevokedEventArgs(pub ::windows::core::IInspectable);
impl ExtendedExecutionRevokedEventArgs {
    pub fn Reason(&self) -> ::windows::core::Result<ExtendedExecutionRevokedReason> {
        let this = self;
        unsafe {
            let mut result__: ExtendedExecutionRevokedReason = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ExtendedExecutionRevokedReason>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ExtendedExecutionRevokedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ExtendedExecution.ExtendedExecutionRevokedEventArgs;{bfbc9f16-63b5-4c0b-aad6-828af5373ec3})");
}
unsafe impl ::windows::core::Interface for ExtendedExecutionRevokedEventArgs {
    type Vtable = IExtendedExecutionRevokedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbfbc9f16_63b5_4c0b_aad6_828af5373ec3);
}
impl ::windows::core::RuntimeName for ExtendedExecutionRevokedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.ExtendedExecution.ExtendedExecutionRevokedEventArgs";
}
impl ::core::convert::From<ExtendedExecutionRevokedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ExtendedExecutionRevokedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ExtendedExecutionRevokedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ExtendedExecutionRevokedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ExtendedExecutionRevokedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ExtendedExecutionRevokedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ExtendedExecutionRevokedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ExtendedExecutionRevokedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ExtendedExecutionRevokedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ExtendedExecutionRevokedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ExtendedExecutionRevokedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ExtendedExecutionRevokedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ExtendedExecutionRevokedEventArgs {}
unsafe impl ::core::marker::Sync for ExtendedExecutionRevokedEventArgs {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ExtendedExecutionRevokedReason(pub i32);
impl ExtendedExecutionRevokedReason {
    pub const Resumed: ExtendedExecutionRevokedReason = ExtendedExecutionRevokedReason(0i32);
    pub const SystemPolicy: ExtendedExecutionRevokedReason = ExtendedExecutionRevokedReason(1i32);
}
impl ::core::convert::From<i32> for ExtendedExecutionRevokedReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ExtendedExecutionRevokedReason {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ExtendedExecutionRevokedReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ExtendedExecution.ExtendedExecutionRevokedReason;i4)");
}
impl ::windows::core::DefaultType for ExtendedExecutionRevokedReason {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ExtendedExecutionSession(pub ::windows::core::IInspectable);
impl ExtendedExecutionSession {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ExtendedExecutionSession, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Reason(&self) -> ::windows::core::Result<ExtendedExecutionReason> {
        let this = self;
        unsafe {
            let mut result__: ExtendedExecutionReason = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ExtendedExecutionReason>(result__)
        }
    }
    pub fn SetReason(&self, value: ExtendedExecutionReason) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn PercentProgress(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetPercentProgress(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Revoked<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, ExtendedExecutionRevokedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveRevoked<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestExtensionAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ExtendedExecutionResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ExtendedExecutionResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for ExtendedExecutionSession {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ExtendedExecution.ExtendedExecutionSession;{af908a2d-118b-48f1-9308-0c4fc41e200f})");
}
unsafe impl ::windows::core::Interface for ExtendedExecutionSession {
    type Vtable = IExtendedExecutionSession_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf908a2d_118b_48f1_9308_0c4fc41e200f);
}
impl ::windows::core::RuntimeName for ExtendedExecutionSession {
    const NAME: &'static str = "Windows.ApplicationModel.ExtendedExecution.ExtendedExecutionSession";
}
impl ::core::convert::From<ExtendedExecutionSession> for ::windows::core::IUnknown {
    fn from(value: ExtendedExecutionSession) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ExtendedExecutionSession> for ::windows::core::IUnknown {
    fn from(value: &ExtendedExecutionSession) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ExtendedExecutionSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ExtendedExecutionSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ExtendedExecutionSession> for ::windows::core::IInspectable {
    fn from(value: ExtendedExecutionSession) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ExtendedExecutionSession> for ::windows::core::IInspectable {
    fn from(value: &ExtendedExecutionSession) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ExtendedExecutionSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ExtendedExecutionSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
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
#[repr(transparent)]
#[doc(hidden)]
pub struct IExtendedExecutionRevokedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IExtendedExecutionRevokedEventArgs {
    type Vtable = IExtendedExecutionRevokedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbfbc9f16_63b5_4c0b_aad6_828af5373ec3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExtendedExecutionRevokedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ExtendedExecutionRevokedReason) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IExtendedExecutionSession(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IExtendedExecutionSession {
    type Vtable = IExtendedExecutionSession_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf908a2d_118b_48f1_9308_0c4fc41e200f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExtendedExecutionSession_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ExtendedExecutionReason) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ExtendedExecutionReason) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);

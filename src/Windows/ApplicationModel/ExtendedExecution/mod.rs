#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "ApplicationModel_ExtendedExecution_Foreground")]
pub mod Foreground;
#[doc = "*Required features: `ApplicationModel_ExtendedExecution`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ExtendedExecutionReason(pub i32);
impl ExtendedExecutionReason {
    pub const Unspecified: ExtendedExecutionReason = ExtendedExecutionReason(0i32);
    pub const LocationTracking: ExtendedExecutionReason = ExtendedExecutionReason(1i32);
    pub const SavingData: ExtendedExecutionReason = ExtendedExecutionReason(2i32);
}
impl ::std::convert::From<i32> for ExtendedExecutionReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ExtendedExecutionReason {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ExtendedExecutionReason {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ExtendedExecution.ExtendedExecutionReason;i4)");
}
impl ::windows::runtime::DefaultType for ExtendedExecutionReason {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_ExtendedExecution`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ExtendedExecutionResult(pub i32);
impl ExtendedExecutionResult {
    pub const Allowed: ExtendedExecutionResult = ExtendedExecutionResult(0i32);
    pub const Denied: ExtendedExecutionResult = ExtendedExecutionResult(1i32);
}
impl ::std::convert::From<i32> for ExtendedExecutionResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ExtendedExecutionResult {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ExtendedExecutionResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ExtendedExecution.ExtendedExecutionResult;i4)");
}
impl ::windows::runtime::DefaultType for ExtendedExecutionResult {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_ExtendedExecution`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ExtendedExecutionRevokedEventArgs(::windows::runtime::IInspectable);
impl ExtendedExecutionRevokedEventArgs {
    #[doc = "*Required features: `ApplicationModel_ExtendedExecution`*"]
    pub fn Reason(&self) -> ::windows::runtime::Result<ExtendedExecutionRevokedReason> {
        let this = self;
        unsafe {
            let mut result__: ExtendedExecutionRevokedReason = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ExtendedExecutionRevokedReason>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ExtendedExecutionRevokedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ExtendedExecution.ExtendedExecutionRevokedEventArgs;{bfbc9f16-63b5-4c0b-aad6-828af5373ec3})");
}
unsafe impl ::windows::runtime::Interface for ExtendedExecutionRevokedEventArgs {
    type Vtable = IExtendedExecutionRevokedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3216809750, 25525, 19467, [170, 214, 130, 138, 245, 55, 62, 195]);
}
impl ::windows::runtime::RuntimeName for ExtendedExecutionRevokedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.ExtendedExecution.ExtendedExecutionRevokedEventArgs";
}
unsafe impl ::std::marker::Send for ExtendedExecutionRevokedEventArgs {}
unsafe impl ::std::marker::Sync for ExtendedExecutionRevokedEventArgs {}
#[doc = "*Required features: `ApplicationModel_ExtendedExecution`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ExtendedExecutionRevokedReason(pub i32);
impl ExtendedExecutionRevokedReason {
    pub const Resumed: ExtendedExecutionRevokedReason = ExtendedExecutionRevokedReason(0i32);
    pub const SystemPolicy: ExtendedExecutionRevokedReason = ExtendedExecutionRevokedReason(1i32);
}
impl ::std::convert::From<i32> for ExtendedExecutionRevokedReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ExtendedExecutionRevokedReason {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ExtendedExecutionRevokedReason {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ExtendedExecution.ExtendedExecutionRevokedReason;i4)");
}
impl ::windows::runtime::DefaultType for ExtendedExecutionRevokedReason {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_ExtendedExecution`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ExtendedExecutionSession(::windows::runtime::IInspectable);
impl ExtendedExecutionSession {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ExtendedExecutionSession, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `ApplicationModel_ExtendedExecution`*"]
    pub fn Reason(&self) -> ::windows::runtime::Result<ExtendedExecutionReason> {
        let this = self;
        unsafe {
            let mut result__: ExtendedExecutionReason = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ExtendedExecutionReason>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_ExtendedExecution`*"]
    pub fn SetReason(&self, value: ExtendedExecutionReason) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_ExtendedExecution`*"]
    pub fn Description(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_ExtendedExecution`*"]
    pub fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_ExtendedExecution`*"]
    pub fn PercentProgress(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_ExtendedExecution`*"]
    pub fn SetPercentProgress(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_ExtendedExecution`, `Foundation`*"]
    pub fn Revoked<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<::windows::runtime::IInspectable, ExtendedExecutionRevokedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_ExtendedExecution`, `Foundation`*"]
    pub fn RemoveRevoked<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_ExtendedExecution`, `Foundation`*"]
    pub fn RequestExtensionAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ExtendedExecutionResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ExtendedExecutionResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_ExtendedExecution`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ExtendedExecutionSession {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ExtendedExecution.ExtendedExecutionSession;{af908a2d-118b-48f1-9308-0c4fc41e200f})");
}
unsafe impl ::windows::runtime::Interface for ExtendedExecutionSession {
    type Vtable = IExtendedExecutionSession_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2945485357, 4491, 18673, [147, 8, 12, 79, 196, 30, 32, 15]);
}
impl ::windows::runtime::RuntimeName for ExtendedExecutionSession {
    const NAME: &'static str = "Windows.ApplicationModel.ExtendedExecution.ExtendedExecutionSession";
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<ExtendedExecutionSession> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ExtendedExecutionSession) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&ExtendedExecutionSession> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ExtendedExecutionSession) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for ExtendedExecutionSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &ExtendedExecutionSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for ExtendedExecutionSession {}
unsafe impl ::std::marker::Sync for ExtendedExecutionSession {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IExtendedExecutionRevokedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IExtendedExecutionRevokedEventArgs {
    type Vtable = IExtendedExecutionRevokedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3216809750, 25525, 19467, [170, 214, 130, 138, 245, 55, 62, 195]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExtendedExecutionRevokedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ExtendedExecutionRevokedReason) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IExtendedExecutionSession(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IExtendedExecutionSession {
    type Vtable = IExtendedExecutionSession_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2945485357, 4491, 18673, [147, 8, 12, 79, 196, 30, 32, 15]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExtendedExecutionSession_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ExtendedExecutionReason) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ExtendedExecutionReason) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);

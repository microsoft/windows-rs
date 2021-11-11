#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `ApplicationModel_ExtendedExecution_Foreground`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ExtendedExecutionForegroundReason(pub i32);
impl ExtendedExecutionForegroundReason {
    pub const Unspecified: ExtendedExecutionForegroundReason = ExtendedExecutionForegroundReason(0i32);
    pub const SavingData: ExtendedExecutionForegroundReason = ExtendedExecutionForegroundReason(1i32);
    pub const BackgroundAudio: ExtendedExecutionForegroundReason = ExtendedExecutionForegroundReason(2i32);
    pub const Unconstrained: ExtendedExecutionForegroundReason = ExtendedExecutionForegroundReason(3i32);
}
impl ::core::convert::From<i32> for ExtendedExecutionForegroundReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ExtendedExecutionForegroundReason {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ExtendedExecutionForegroundReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ExtendedExecution.Foreground.ExtendedExecutionForegroundReason;i4)");
}
impl ::windows::core::DefaultType for ExtendedExecutionForegroundReason {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_ExtendedExecution_Foreground`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ExtendedExecutionForegroundResult(pub i32);
impl ExtendedExecutionForegroundResult {
    pub const Allowed: ExtendedExecutionForegroundResult = ExtendedExecutionForegroundResult(0i32);
    pub const Denied: ExtendedExecutionForegroundResult = ExtendedExecutionForegroundResult(1i32);
}
impl ::core::convert::From<i32> for ExtendedExecutionForegroundResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ExtendedExecutionForegroundResult {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ExtendedExecutionForegroundResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ExtendedExecution.Foreground.ExtendedExecutionForegroundResult;i4)");
}
impl ::windows::core::DefaultType for ExtendedExecutionForegroundResult {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_ExtendedExecution_Foreground`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ExtendedExecutionForegroundRevokedEventArgs(pub ::windows::core::IInspectable);
impl ExtendedExecutionForegroundRevokedEventArgs {
    #[doc = "*Required features: `ApplicationModel_ExtendedExecution_Foreground`*"]
    pub fn Reason(&self) -> ::windows::core::Result<ExtendedExecutionForegroundRevokedReason> {
        let this = self;
        unsafe {
            let mut result__: ExtendedExecutionForegroundRevokedReason = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ExtendedExecutionForegroundRevokedReason>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ExtendedExecutionForegroundRevokedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ExtendedExecution.Foreground.ExtendedExecutionForegroundRevokedEventArgs;{b07cd940-9557-aea4-2c99-bdd56d9be461})");
}
unsafe impl ::windows::core::Interface for ExtendedExecutionForegroundRevokedEventArgs {
    type Vtable = IExtendedExecutionForegroundRevokedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb07cd940_9557_aea4_2c99_bdd56d9be461);
}
impl ::windows::core::RuntimeName for ExtendedExecutionForegroundRevokedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.ExtendedExecution.Foreground.ExtendedExecutionForegroundRevokedEventArgs";
}
impl ::core::convert::From<ExtendedExecutionForegroundRevokedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ExtendedExecutionForegroundRevokedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ExtendedExecutionForegroundRevokedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ExtendedExecutionForegroundRevokedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ExtendedExecutionForegroundRevokedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ExtendedExecutionForegroundRevokedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ExtendedExecutionForegroundRevokedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ExtendedExecutionForegroundRevokedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ExtendedExecutionForegroundRevokedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ExtendedExecutionForegroundRevokedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ExtendedExecutionForegroundRevokedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ExtendedExecutionForegroundRevokedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ExtendedExecutionForegroundRevokedEventArgs {}
unsafe impl ::core::marker::Sync for ExtendedExecutionForegroundRevokedEventArgs {}
#[doc = "*Required features: `ApplicationModel_ExtendedExecution_Foreground`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ExtendedExecutionForegroundRevokedReason(pub i32);
impl ExtendedExecutionForegroundRevokedReason {
    pub const Resumed: ExtendedExecutionForegroundRevokedReason = ExtendedExecutionForegroundRevokedReason(0i32);
    pub const SystemPolicy: ExtendedExecutionForegroundRevokedReason = ExtendedExecutionForegroundRevokedReason(1i32);
}
impl ::core::convert::From<i32> for ExtendedExecutionForegroundRevokedReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ExtendedExecutionForegroundRevokedReason {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ExtendedExecutionForegroundRevokedReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ExtendedExecution.Foreground.ExtendedExecutionForegroundRevokedReason;i4)");
}
impl ::windows::core::DefaultType for ExtendedExecutionForegroundRevokedReason {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_ExtendedExecution_Foreground`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ExtendedExecutionForegroundSession(pub ::windows::core::IInspectable);
impl ExtendedExecutionForegroundSession {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ExtendedExecutionForegroundSession, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `ApplicationModel_ExtendedExecution_Foreground`*"]
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_ExtendedExecution_Foreground`*"]
    pub fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_ExtendedExecution_Foreground`, `Foundation`*"]
    pub fn Revoked<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, ExtendedExecutionForegroundRevokedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_ExtendedExecution_Foreground`, `Foundation`*"]
    pub fn RemoveRevoked<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_ExtendedExecution_Foreground`, `Foundation`*"]
    pub fn RequestExtensionAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<ExtendedExecutionForegroundResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<ExtendedExecutionForegroundResult>>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_ExtendedExecution_Foreground`*"]
    pub fn Reason(&self) -> ::windows::core::Result<ExtendedExecutionForegroundReason> {
        let this = self;
        unsafe {
            let mut result__: ExtendedExecutionForegroundReason = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ExtendedExecutionForegroundReason>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_ExtendedExecution_Foreground`*"]
    pub fn SetReason(&self, value: ExtendedExecutionForegroundReason) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_ExtendedExecution_Foreground`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for ExtendedExecutionForegroundSession {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ExtendedExecution.Foreground.ExtendedExecutionForegroundSession;{fbf440e1-9d10-4201-b01e-c83275296f2e})");
}
unsafe impl ::windows::core::Interface for ExtendedExecutionForegroundSession {
    type Vtable = IExtendedExecutionForegroundSession_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbf440e1_9d10_4201_b01e_c83275296f2e);
}
impl ::windows::core::RuntimeName for ExtendedExecutionForegroundSession {
    const NAME: &'static str = "Windows.ApplicationModel.ExtendedExecution.Foreground.ExtendedExecutionForegroundSession";
}
impl ::core::convert::From<ExtendedExecutionForegroundSession> for ::windows::core::IUnknown {
    fn from(value: ExtendedExecutionForegroundSession) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ExtendedExecutionForegroundSession> for ::windows::core::IUnknown {
    fn from(value: &ExtendedExecutionForegroundSession) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ExtendedExecutionForegroundSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ExtendedExecutionForegroundSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ExtendedExecutionForegroundSession> for ::windows::core::IInspectable {
    fn from(value: ExtendedExecutionForegroundSession) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ExtendedExecutionForegroundSession> for ::windows::core::IInspectable {
    fn from(value: &ExtendedExecutionForegroundSession) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ExtendedExecutionForegroundSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ExtendedExecutionForegroundSession {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<ExtendedExecutionForegroundSession> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: ExtendedExecutionForegroundSession) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&ExtendedExecutionForegroundSession> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &ExtendedExecutionForegroundSession) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for ExtendedExecutionForegroundSession {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for &ExtendedExecutionForegroundSession {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ExtendedExecutionForegroundSession {}
unsafe impl ::core::marker::Sync for ExtendedExecutionForegroundSession {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IExtendedExecutionForegroundRevokedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IExtendedExecutionForegroundRevokedEventArgs {
    type Vtable = IExtendedExecutionForegroundRevokedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb07cd940_9557_aea4_2c99_bdd56d9be461);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExtendedExecutionForegroundRevokedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ExtendedExecutionForegroundRevokedReason) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IExtendedExecutionForegroundSession(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IExtendedExecutionForegroundSession {
    type Vtable = IExtendedExecutionForegroundSession_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbf440e1_9d10_4201_b01e_c83275296f2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExtendedExecutionForegroundSession_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ExtendedExecutionForegroundReason) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ExtendedExecutionForegroundReason) -> ::windows::core::HRESULT,
);

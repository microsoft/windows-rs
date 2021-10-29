#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ExtendedExecutionForegroundReason(pub i32);
impl ExtendedExecutionForegroundReason {
    pub const Unspecified: ExtendedExecutionForegroundReason = ExtendedExecutionForegroundReason(0i32);
    pub const SavingData: ExtendedExecutionForegroundReason = ExtendedExecutionForegroundReason(1i32);
    pub const BackgroundAudio: ExtendedExecutionForegroundReason = ExtendedExecutionForegroundReason(2i32);
    pub const Unconstrained: ExtendedExecutionForegroundReason = ExtendedExecutionForegroundReason(3i32);
}
impl ::std::convert::From<i32> for ExtendedExecutionForegroundReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ExtendedExecutionForegroundReason {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ExtendedExecutionForegroundReason {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ExtendedExecution.Foreground.ExtendedExecutionForegroundReason;i4)");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ExtendedExecutionForegroundResult(pub i32);
impl ExtendedExecutionForegroundResult {
    pub const Allowed: ExtendedExecutionForegroundResult = ExtendedExecutionForegroundResult(0i32);
    pub const Denied: ExtendedExecutionForegroundResult = ExtendedExecutionForegroundResult(1i32);
}
impl ::std::convert::From<i32> for ExtendedExecutionForegroundResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ExtendedExecutionForegroundResult {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ExtendedExecutionForegroundResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ExtendedExecution.Foreground.ExtendedExecutionForegroundResult;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ExtendedExecutionForegroundRevokedEventArgs(::windows::runtime::IInspectable);
impl ExtendedExecutionForegroundRevokedEventArgs {
    pub fn Reason(&self) -> ::windows::runtime::Result<ExtendedExecutionForegroundRevokedReason> {
        let this = self;
        unsafe {
            let mut result__: ExtendedExecutionForegroundRevokedReason = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ExtendedExecutionForegroundRevokedReason>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ExtendedExecutionForegroundRevokedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ExtendedExecution.Foreground.ExtendedExecutionForegroundRevokedEventArgs;{b07cd940-9557-aea4-2c99-bdd56d9be461})");
}
unsafe impl ::windows::runtime::Interface for ExtendedExecutionForegroundRevokedEventArgs {
    type Vtable = IExtendedExecutionForegroundRevokedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2960972096, 38231, 44708, [44, 153, 189, 213, 109, 155, 228, 97]);
}
impl ::windows::runtime::RuntimeName for ExtendedExecutionForegroundRevokedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.ExtendedExecution.Foreground.ExtendedExecutionForegroundRevokedEventArgs";
}
impl ::std::convert::From<ExtendedExecutionForegroundRevokedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ExtendedExecutionForegroundRevokedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ExtendedExecutionForegroundRevokedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ExtendedExecutionForegroundRevokedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ExtendedExecutionForegroundRevokedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ExtendedExecutionForegroundRevokedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ExtendedExecutionForegroundRevokedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ExtendedExecutionForegroundRevokedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ExtendedExecutionForegroundRevokedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ExtendedExecutionForegroundRevokedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ExtendedExecutionForegroundRevokedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ExtendedExecutionForegroundRevokedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ExtendedExecutionForegroundRevokedEventArgs {}
unsafe impl ::std::marker::Sync for ExtendedExecutionForegroundRevokedEventArgs {}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ExtendedExecutionForegroundRevokedReason(pub i32);
impl ExtendedExecutionForegroundRevokedReason {
    pub const Resumed: ExtendedExecutionForegroundRevokedReason = ExtendedExecutionForegroundRevokedReason(0i32);
    pub const SystemPolicy: ExtendedExecutionForegroundRevokedReason = ExtendedExecutionForegroundRevokedReason(1i32);
}
impl ::std::convert::From<i32> for ExtendedExecutionForegroundRevokedReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ExtendedExecutionForegroundRevokedReason {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ExtendedExecutionForegroundRevokedReason {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.ExtendedExecution.Foreground.ExtendedExecutionForegroundRevokedReason;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ExtendedExecutionForegroundSession(::windows::runtime::IInspectable);
impl ExtendedExecutionForegroundSession {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ExtendedExecutionForegroundSession, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Description(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn SetDescription<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Revoked<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<::windows::runtime::IInspectable, ExtendedExecutionForegroundRevokedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveRevoked<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestExtensionAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<ExtendedExecutionForegroundResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<ExtendedExecutionForegroundResult>>(result__)
        }
    }
    pub fn Reason(&self) -> ::windows::runtime::Result<ExtendedExecutionForegroundReason> {
        let this = self;
        unsafe {
            let mut result__: ExtendedExecutionForegroundReason = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ExtendedExecutionForegroundReason>(result__)
        }
    }
    pub fn SetReason(&self, value: ExtendedExecutionForegroundReason) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ExtendedExecutionForegroundSession {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.ExtendedExecution.Foreground.ExtendedExecutionForegroundSession;{fbf440e1-9d10-4201-b01e-c83275296f2e})");
}
unsafe impl ::windows::runtime::Interface for ExtendedExecutionForegroundSession {
    type Vtable = IExtendedExecutionForegroundSession_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4227088609, 40208, 16897, [176, 30, 200, 50, 117, 41, 111, 46]);
}
impl ::windows::runtime::RuntimeName for ExtendedExecutionForegroundSession {
    const NAME: &'static str = "Windows.ApplicationModel.ExtendedExecution.Foreground.ExtendedExecutionForegroundSession";
}
impl ::std::convert::From<ExtendedExecutionForegroundSession> for ::windows::runtime::IUnknown {
    fn from(value: ExtendedExecutionForegroundSession) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ExtendedExecutionForegroundSession> for ::windows::runtime::IUnknown {
    fn from(value: &ExtendedExecutionForegroundSession) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ExtendedExecutionForegroundSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ExtendedExecutionForegroundSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ExtendedExecutionForegroundSession> for ::windows::runtime::IInspectable {
    fn from(value: ExtendedExecutionForegroundSession) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ExtendedExecutionForegroundSession> for ::windows::runtime::IInspectable {
    fn from(value: &ExtendedExecutionForegroundSession) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ExtendedExecutionForegroundSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ExtendedExecutionForegroundSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<ExtendedExecutionForegroundSession> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ExtendedExecutionForegroundSession) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&ExtendedExecutionForegroundSession> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ExtendedExecutionForegroundSession) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for ExtendedExecutionForegroundSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for &ExtendedExecutionForegroundSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for ExtendedExecutionForegroundSession {}
unsafe impl ::std::marker::Sync for ExtendedExecutionForegroundSession {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IExtendedExecutionForegroundRevokedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IExtendedExecutionForegroundRevokedEventArgs {
    type Vtable = IExtendedExecutionForegroundRevokedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2960972096, 38231, 44708, [44, 153, 189, 213, 109, 155, 228, 97]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExtendedExecutionForegroundRevokedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ExtendedExecutionForegroundRevokedReason) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IExtendedExecutionForegroundSession(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IExtendedExecutionForegroundSession {
    type Vtable = IExtendedExecutionForegroundSession_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4227088609, 40208, 16897, [176, 30, 200, 50, 117, 41, 111, 46]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExtendedExecutionForegroundSession_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ExtendedExecutionForegroundReason) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ExtendedExecutionForegroundReason) -> ::windows::runtime::HRESULT,
);

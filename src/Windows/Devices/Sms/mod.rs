#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Devices_Sms`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CellularClass(pub i32);
impl CellularClass {
    pub const None: CellularClass = CellularClass(0i32);
    pub const Gsm: CellularClass = CellularClass(1i32);
    pub const Cdma: CellularClass = CellularClass(2i32);
}
impl ::std::convert::From<i32> for CellularClass {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CellularClass {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CellularClass {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Sms.CellularClass;i4)");
}
impl ::windows::runtime::DefaultType for CellularClass {
    type DefaultType = Self;
}
#[cfg(feature = "Foundation")]
#[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct DeleteSmsMessageOperation(::windows::runtime::IInspectable);
#[cfg(feature = "Foundation")]
impl DeleteSmsMessageOperation {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn SetCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::AsyncActionCompletedHandler>>(&self, handler: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn Completed(&self) -> ::windows::runtime::Result<super::super::Foundation::AsyncActionCompletedHandler> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::AsyncActionCompletedHandler>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn GetResults(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<super::super::Foundation::AsyncStatus> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::AsyncStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::AsyncStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn ErrorCode(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn Cancel(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this)).ok() }
    }
    pub fn get(&self) -> ::windows::runtime::Result<()> {
        if self.Status()? == super::super::Foundation::AsyncStatus::Started {
            let (waiter, signaler) = ::windows::runtime::Waiter::new();
            self.SetCompleted(super::super::Foundation::AsyncActionCompletedHandler::new(move |_sender, _args| {
                unsafe {
                    signaler.signal();
                }
                Ok(())
            }))?;
        }
        self.GetResults()
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::runtime::RuntimeType for DeleteSmsMessageOperation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.DeleteSmsMessageOperation;{5a648006-843a-4da9-865b-9d26e5dfad7b})");
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::runtime::Interface for DeleteSmsMessageOperation {
    type Vtable = super::super::Foundation::IAsyncAction_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1516535814, 33850, 19881, [134, 91, 157, 38, 229, 223, 173, 123]);
}
#[cfg(feature = "Foundation")]
impl ::windows::runtime::RuntimeName for DeleteSmsMessageOperation {
    const NAME: &'static str = "Windows.Devices.Sms.DeleteSmsMessageOperation";
}
impl ::std::future::Future for DeleteSmsMessageOperation {
    type Output = ::windows::runtime::Result<()>;
    fn poll(self: ::std::pin::Pin<&mut Self>, context: &mut ::std::task::Context) -> ::std::task::Poll<Self::Output> {
        if self.Status()? == super::super::Foundation::AsyncStatus::Started {
            let waker = context.waker().clone();
            let _ = self.SetCompleted(super::super::Foundation::AsyncActionCompletedHandler::new(move |_sender, _args| {
                waker.wake_by_ref();
                Ok(())
            }));
            ::std::task::Poll::Pending
        } else {
            ::std::task::Poll::Ready(self.GetResults())
        }
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<DeleteSmsMessageOperation> for ::windows::runtime::IUnknown {
    fn from(value: DeleteSmsMessageOperation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<&DeleteSmsMessageOperation> for ::windows::runtime::IUnknown {
    fn from(value: &DeleteSmsMessageOperation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DeleteSmsMessageOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &DeleteSmsMessageOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<DeleteSmsMessageOperation> for ::windows::runtime::IInspectable {
    fn from(value: DeleteSmsMessageOperation) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<&DeleteSmsMessageOperation> for ::windows::runtime::IInspectable {
    fn from(value: &DeleteSmsMessageOperation) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DeleteSmsMessageOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DeleteSmsMessageOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<DeleteSmsMessageOperation> for super::super::Foundation::IAsyncAction {
    fn from(value: DeleteSmsMessageOperation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<&DeleteSmsMessageOperation> for super::super::Foundation::IAsyncAction {
    fn from(value: &DeleteSmsMessageOperation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IAsyncAction> for DeleteSmsMessageOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IAsyncAction> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::Foundation::IAsyncAction>::into(self))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IAsyncAction> for &DeleteSmsMessageOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IAsyncAction> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::Foundation::IAsyncAction>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<DeleteSmsMessageOperation> for super::super::Foundation::IAsyncInfo {
    type Error = ::windows::runtime::Error;
    fn try_from(value: DeleteSmsMessageOperation) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&DeleteSmsMessageOperation> for super::super::Foundation::IAsyncInfo {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &DeleteSmsMessageOperation) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IAsyncInfo> for DeleteSmsMessageOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IAsyncInfo> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IAsyncInfo> for &DeleteSmsMessageOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IAsyncInfo> {
        ::std::convert::TryInto::<super::super::Foundation::IAsyncInfo>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation")]
#[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct DeleteSmsMessagesOperation(::windows::runtime::IInspectable);
#[cfg(feature = "Foundation")]
impl DeleteSmsMessagesOperation {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn SetCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::AsyncActionCompletedHandler>>(&self, handler: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn Completed(&self) -> ::windows::runtime::Result<super::super::Foundation::AsyncActionCompletedHandler> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::AsyncActionCompletedHandler>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn GetResults(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<super::super::Foundation::AsyncStatus> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::AsyncStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::AsyncStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn ErrorCode(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn Cancel(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this)).ok() }
    }
    pub fn get(&self) -> ::windows::runtime::Result<()> {
        if self.Status()? == super::super::Foundation::AsyncStatus::Started {
            let (waiter, signaler) = ::windows::runtime::Waiter::new();
            self.SetCompleted(super::super::Foundation::AsyncActionCompletedHandler::new(move |_sender, _args| {
                unsafe {
                    signaler.signal();
                }
                Ok(())
            }))?;
        }
        self.GetResults()
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::runtime::RuntimeType for DeleteSmsMessagesOperation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.DeleteSmsMessagesOperation;{5a648006-843a-4da9-865b-9d26e5dfad7b})");
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::runtime::Interface for DeleteSmsMessagesOperation {
    type Vtable = super::super::Foundation::IAsyncAction_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1516535814, 33850, 19881, [134, 91, 157, 38, 229, 223, 173, 123]);
}
#[cfg(feature = "Foundation")]
impl ::windows::runtime::RuntimeName for DeleteSmsMessagesOperation {
    const NAME: &'static str = "Windows.Devices.Sms.DeleteSmsMessagesOperation";
}
impl ::std::future::Future for DeleteSmsMessagesOperation {
    type Output = ::windows::runtime::Result<()>;
    fn poll(self: ::std::pin::Pin<&mut Self>, context: &mut ::std::task::Context) -> ::std::task::Poll<Self::Output> {
        if self.Status()? == super::super::Foundation::AsyncStatus::Started {
            let waker = context.waker().clone();
            let _ = self.SetCompleted(super::super::Foundation::AsyncActionCompletedHandler::new(move |_sender, _args| {
                waker.wake_by_ref();
                Ok(())
            }));
            ::std::task::Poll::Pending
        } else {
            ::std::task::Poll::Ready(self.GetResults())
        }
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<DeleteSmsMessagesOperation> for ::windows::runtime::IUnknown {
    fn from(value: DeleteSmsMessagesOperation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<&DeleteSmsMessagesOperation> for ::windows::runtime::IUnknown {
    fn from(value: &DeleteSmsMessagesOperation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DeleteSmsMessagesOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &DeleteSmsMessagesOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<DeleteSmsMessagesOperation> for ::windows::runtime::IInspectable {
    fn from(value: DeleteSmsMessagesOperation) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<&DeleteSmsMessagesOperation> for ::windows::runtime::IInspectable {
    fn from(value: &DeleteSmsMessagesOperation) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DeleteSmsMessagesOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a DeleteSmsMessagesOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<DeleteSmsMessagesOperation> for super::super::Foundation::IAsyncAction {
    fn from(value: DeleteSmsMessagesOperation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<&DeleteSmsMessagesOperation> for super::super::Foundation::IAsyncAction {
    fn from(value: &DeleteSmsMessagesOperation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IAsyncAction> for DeleteSmsMessagesOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IAsyncAction> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::Foundation::IAsyncAction>::into(self))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IAsyncAction> for &DeleteSmsMessagesOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IAsyncAction> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::Foundation::IAsyncAction>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<DeleteSmsMessagesOperation> for super::super::Foundation::IAsyncInfo {
    type Error = ::windows::runtime::Error;
    fn try_from(value: DeleteSmsMessagesOperation) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&DeleteSmsMessagesOperation> for super::super::Foundation::IAsyncInfo {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &DeleteSmsMessagesOperation) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IAsyncInfo> for DeleteSmsMessagesOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IAsyncInfo> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IAsyncInfo> for &DeleteSmsMessagesOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IAsyncInfo> {
        ::std::convert::TryInto::<super::super::Foundation::IAsyncInfo>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation")]
#[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct GetSmsDeviceOperation(::windows::runtime::IInspectable);
#[cfg(feature = "Foundation")]
impl GetSmsDeviceOperation {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn SetCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::AsyncOperationCompletedHandler<SmsDevice>>>(&self, handler: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn Completed(&self) -> ::windows::runtime::Result<super::super::Foundation::AsyncOperationCompletedHandler<SmsDevice>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::AsyncOperationCompletedHandler<SmsDevice>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn GetResults(&self) -> ::windows::runtime::Result<SmsDevice> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SmsDevice>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<super::super::Foundation::AsyncStatus> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::AsyncStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::AsyncStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn ErrorCode(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn Cancel(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this)).ok() }
    }
    pub fn get(&self) -> ::windows::runtime::Result<SmsDevice> {
        if self.Status()? == super::super::Foundation::AsyncStatus::Started {
            let (waiter, signaler) = ::windows::runtime::Waiter::new();
            self.SetCompleted(super::super::Foundation::AsyncOperationCompletedHandler::new(move |_sender, _args| {
                unsafe {
                    signaler.signal();
                }
                Ok(())
            }))?;
        }
        self.GetResults()
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::runtime::RuntimeType for GetSmsDeviceOperation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.GetSmsDeviceOperation;pinterface({9fc2b0bb-e446-44e2-aa61-9cab8f636af2};rc(Windows.Devices.Sms.SmsDevice;{091791ed-872b-4eec-9c72-ab11627b34ec})))");
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::runtime::Interface for GetSmsDeviceOperation {
    type Vtable = super::super::Foundation::IAsyncOperation_abi<SmsDevice>;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_signature(<super::super::Foundation::IAsyncOperation<SmsDevice> as ::windows::runtime::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation")]
impl ::windows::runtime::RuntimeName for GetSmsDeviceOperation {
    const NAME: &'static str = "Windows.Devices.Sms.GetSmsDeviceOperation";
}
impl ::std::future::Future for GetSmsDeviceOperation {
    type Output = ::windows::runtime::Result<SmsDevice>;
    fn poll(self: ::std::pin::Pin<&mut Self>, context: &mut ::std::task::Context) -> ::std::task::Poll<Self::Output> {
        if self.Status()? == super::super::Foundation::AsyncStatus::Started {
            let waker = context.waker().clone();
            let _ = self.SetCompleted(super::super::Foundation::AsyncOperationCompletedHandler::new(move |_sender, _args| {
                waker.wake_by_ref();
                Ok(())
            }));
            ::std::task::Poll::Pending
        } else {
            ::std::task::Poll::Ready(self.GetResults())
        }
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<GetSmsDeviceOperation> for ::windows::runtime::IUnknown {
    fn from(value: GetSmsDeviceOperation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<&GetSmsDeviceOperation> for ::windows::runtime::IUnknown {
    fn from(value: &GetSmsDeviceOperation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for GetSmsDeviceOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &GetSmsDeviceOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<GetSmsDeviceOperation> for ::windows::runtime::IInspectable {
    fn from(value: GetSmsDeviceOperation) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<&GetSmsDeviceOperation> for ::windows::runtime::IInspectable {
    fn from(value: &GetSmsDeviceOperation) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for GetSmsDeviceOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a GetSmsDeviceOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<GetSmsDeviceOperation> for super::super::Foundation::IAsyncOperation<SmsDevice> {
    fn from(value: GetSmsDeviceOperation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<&GetSmsDeviceOperation> for super::super::Foundation::IAsyncOperation<SmsDevice> {
    fn from(value: &GetSmsDeviceOperation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IAsyncOperation<SmsDevice>> for GetSmsDeviceOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IAsyncOperation<SmsDevice>> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::Foundation::IAsyncOperation<SmsDevice>>::into(self))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IAsyncOperation<SmsDevice>> for &GetSmsDeviceOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IAsyncOperation<SmsDevice>> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::Foundation::IAsyncOperation<SmsDevice>>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<GetSmsDeviceOperation> for super::super::Foundation::IAsyncInfo {
    type Error = ::windows::runtime::Error;
    fn try_from(value: GetSmsDeviceOperation) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&GetSmsDeviceOperation> for super::super::Foundation::IAsyncInfo {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &GetSmsDeviceOperation) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IAsyncInfo> for GetSmsDeviceOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IAsyncInfo> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IAsyncInfo> for &GetSmsDeviceOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IAsyncInfo> {
        ::std::convert::TryInto::<super::super::Foundation::IAsyncInfo>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation")]
#[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct GetSmsMessageOperation(::windows::runtime::IInspectable);
#[cfg(feature = "Foundation")]
impl GetSmsMessageOperation {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn SetCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::AsyncOperationCompletedHandler<ISmsMessage>>>(&self, handler: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn Completed(&self) -> ::windows::runtime::Result<super::super::Foundation::AsyncOperationCompletedHandler<ISmsMessage>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::AsyncOperationCompletedHandler<ISmsMessage>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn GetResults(&self) -> ::windows::runtime::Result<ISmsMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ISmsMessage>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<super::super::Foundation::AsyncStatus> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::AsyncStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::AsyncStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn ErrorCode(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn Cancel(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this)).ok() }
    }
    pub fn get(&self) -> ::windows::runtime::Result<ISmsMessage> {
        if self.Status()? == super::super::Foundation::AsyncStatus::Started {
            let (waiter, signaler) = ::windows::runtime::Waiter::new();
            self.SetCompleted(super::super::Foundation::AsyncOperationCompletedHandler::new(move |_sender, _args| {
                unsafe {
                    signaler.signal();
                }
                Ok(())
            }))?;
        }
        self.GetResults()
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::runtime::RuntimeType for GetSmsMessageOperation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.GetSmsMessageOperation;pinterface({9fc2b0bb-e446-44e2-aa61-9cab8f636af2};{ed3c5e28-6984-4b07-811d-8d5906ed3cea}))");
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::runtime::Interface for GetSmsMessageOperation {
    type Vtable = super::super::Foundation::IAsyncOperation_abi<ISmsMessage>;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_signature(<super::super::Foundation::IAsyncOperation<ISmsMessage> as ::windows::runtime::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation")]
impl ::windows::runtime::RuntimeName for GetSmsMessageOperation {
    const NAME: &'static str = "Windows.Devices.Sms.GetSmsMessageOperation";
}
impl ::std::future::Future for GetSmsMessageOperation {
    type Output = ::windows::runtime::Result<ISmsMessage>;
    fn poll(self: ::std::pin::Pin<&mut Self>, context: &mut ::std::task::Context) -> ::std::task::Poll<Self::Output> {
        if self.Status()? == super::super::Foundation::AsyncStatus::Started {
            let waker = context.waker().clone();
            let _ = self.SetCompleted(super::super::Foundation::AsyncOperationCompletedHandler::new(move |_sender, _args| {
                waker.wake_by_ref();
                Ok(())
            }));
            ::std::task::Poll::Pending
        } else {
            ::std::task::Poll::Ready(self.GetResults())
        }
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<GetSmsMessageOperation> for ::windows::runtime::IUnknown {
    fn from(value: GetSmsMessageOperation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<&GetSmsMessageOperation> for ::windows::runtime::IUnknown {
    fn from(value: &GetSmsMessageOperation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for GetSmsMessageOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &GetSmsMessageOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<GetSmsMessageOperation> for ::windows::runtime::IInspectable {
    fn from(value: GetSmsMessageOperation) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<&GetSmsMessageOperation> for ::windows::runtime::IInspectable {
    fn from(value: &GetSmsMessageOperation) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for GetSmsMessageOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a GetSmsMessageOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<GetSmsMessageOperation> for super::super::Foundation::IAsyncOperation<ISmsMessage> {
    fn from(value: GetSmsMessageOperation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<&GetSmsMessageOperation> for super::super::Foundation::IAsyncOperation<ISmsMessage> {
    fn from(value: &GetSmsMessageOperation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IAsyncOperation<ISmsMessage>> for GetSmsMessageOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IAsyncOperation<ISmsMessage>> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::Foundation::IAsyncOperation<ISmsMessage>>::into(self))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IAsyncOperation<ISmsMessage>> for &GetSmsMessageOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IAsyncOperation<ISmsMessage>> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::Foundation::IAsyncOperation<ISmsMessage>>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<GetSmsMessageOperation> for super::super::Foundation::IAsyncInfo {
    type Error = ::windows::runtime::Error;
    fn try_from(value: GetSmsMessageOperation) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&GetSmsMessageOperation> for super::super::Foundation::IAsyncInfo {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &GetSmsMessageOperation) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IAsyncInfo> for GetSmsMessageOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IAsyncInfo> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IAsyncInfo> for &GetSmsMessageOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IAsyncInfo> {
        ::std::convert::TryInto::<super::super::Foundation::IAsyncInfo>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation")]
#[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct GetSmsMessagesOperation(::windows::runtime::IInspectable);
#[cfg(feature = "Foundation")]
impl GetSmsMessagesOperation {
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`, `Foundation_Collections`*"]
    pub fn SetProgress<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::AsyncOperationProgressHandler<super::super::Foundation::Collections::IVectorView<ISmsMessage>, i32>>>(&self, handler: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`, `Foundation_Collections`*"]
    pub fn Progress(&self) -> ::windows::runtime::Result<super::super::Foundation::AsyncOperationProgressHandler<super::super::Foundation::Collections::IVectorView<ISmsMessage>, i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::AsyncOperationProgressHandler<super::super::Foundation::Collections::IVectorView<ISmsMessage>, i32>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`, `Foundation_Collections`*"]
    pub fn SetCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::AsyncOperationWithProgressCompletedHandler<super::super::Foundation::Collections::IVectorView<ISmsMessage>, i32>>>(&self, handler: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), handler.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`, `Foundation_Collections`*"]
    pub fn Completed(&self) -> ::windows::runtime::Result<super::super::Foundation::AsyncOperationWithProgressCompletedHandler<super::super::Foundation::Collections::IVectorView<ISmsMessage>, i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::AsyncOperationWithProgressCompletedHandler<super::super::Foundation::Collections::IVectorView<ISmsMessage>, i32>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetResults(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<ISmsMessage>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ISmsMessage>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<super::super::Foundation::AsyncStatus> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::AsyncStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::AsyncStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn ErrorCode(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn Cancel(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this)).ok() }
    }
    pub fn get(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<ISmsMessage>> {
        if self.Status()? == super::super::Foundation::AsyncStatus::Started {
            let (waiter, signaler) = ::windows::runtime::Waiter::new();
            self.SetCompleted(super::super::Foundation::AsyncOperationWithProgressCompletedHandler::new(move |_sender, _args| {
                unsafe {
                    signaler.signal();
                }
                Ok(())
            }))?;
        }
        self.GetResults()
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::runtime::RuntimeType for GetSmsMessagesOperation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.GetSmsMessagesOperation;pinterface({b5d036d7-e297-498f-ba60-0289e76e23dd};pinterface({bbe1fa4c-b0e3-4583-baef-1f1b2e483e56};{ed3c5e28-6984-4b07-811d-8d5906ed3cea});i4))");
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::runtime::Interface for GetSmsMessagesOperation {
    type Vtable = super::super::Foundation::IAsyncOperationWithProgress_abi<super::super::Foundation::Collections::IVectorView<ISmsMessage>, i32>;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_signature(<super::super::Foundation::IAsyncOperationWithProgress<super::super::Foundation::Collections::IVectorView<ISmsMessage>, i32> as ::windows::runtime::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation")]
impl ::windows::runtime::RuntimeName for GetSmsMessagesOperation {
    const NAME: &'static str = "Windows.Devices.Sms.GetSmsMessagesOperation";
}
impl ::std::future::Future for GetSmsMessagesOperation {
    type Output = ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<ISmsMessage>>;
    fn poll(self: ::std::pin::Pin<&mut Self>, context: &mut ::std::task::Context) -> ::std::task::Poll<Self::Output> {
        if self.Status()? == super::super::Foundation::AsyncStatus::Started {
            let waker = context.waker().clone();
            let _ = self.SetCompleted(super::super::Foundation::AsyncOperationWithProgressCompletedHandler::new(move |_sender, _args| {
                waker.wake_by_ref();
                Ok(())
            }));
            ::std::task::Poll::Pending
        } else {
            ::std::task::Poll::Ready(self.GetResults())
        }
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<GetSmsMessagesOperation> for ::windows::runtime::IUnknown {
    fn from(value: GetSmsMessagesOperation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<&GetSmsMessagesOperation> for ::windows::runtime::IUnknown {
    fn from(value: &GetSmsMessagesOperation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for GetSmsMessagesOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &GetSmsMessagesOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<GetSmsMessagesOperation> for ::windows::runtime::IInspectable {
    fn from(value: GetSmsMessagesOperation) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<&GetSmsMessagesOperation> for ::windows::runtime::IInspectable {
    fn from(value: &GetSmsMessagesOperation) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for GetSmsMessagesOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a GetSmsMessagesOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<GetSmsMessagesOperation> for super::super::Foundation::IAsyncOperationWithProgress<super::super::Foundation::Collections::IVectorView<ISmsMessage>, i32> {
    fn from(value: GetSmsMessagesOperation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<&GetSmsMessagesOperation> for super::super::Foundation::IAsyncOperationWithProgress<super::super::Foundation::Collections::IVectorView<ISmsMessage>, i32> {
    fn from(value: &GetSmsMessagesOperation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IAsyncOperationWithProgress<super::super::Foundation::Collections::IVectorView<ISmsMessage>, i32>> for GetSmsMessagesOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IAsyncOperationWithProgress<super::super::Foundation::Collections::IVectorView<ISmsMessage>, i32>> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::Foundation::IAsyncOperationWithProgress<super::super::Foundation::Collections::IVectorView<ISmsMessage>, i32>>::into(self))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IAsyncOperationWithProgress<super::super::Foundation::Collections::IVectorView<ISmsMessage>, i32>> for &GetSmsMessagesOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IAsyncOperationWithProgress<super::super::Foundation::Collections::IVectorView<ISmsMessage>, i32>> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::Foundation::IAsyncOperationWithProgress<super::super::Foundation::Collections::IVectorView<ISmsMessage>, i32>>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<GetSmsMessagesOperation> for super::super::Foundation::IAsyncInfo {
    type Error = ::windows::runtime::Error;
    fn try_from(value: GetSmsMessagesOperation) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&GetSmsMessagesOperation> for super::super::Foundation::IAsyncInfo {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &GetSmsMessagesOperation) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IAsyncInfo> for GetSmsMessagesOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IAsyncInfo> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IAsyncInfo> for &GetSmsMessagesOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IAsyncInfo> {
        ::std::convert::TryInto::<super::super::Foundation::IAsyncInfo>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmsAppMessage(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmsAppMessage {
    type Vtable = ISmsAppMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3904603284, 54176, 18954, [134, 215, 41, 16, 51, 168, 207, 84]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsAppMessage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SmsEncoding) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: SmsEncoding) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `Devices_Sms`*"]
pub struct ISmsBinaryMessage(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmsBinaryMessage {
    type Vtable = ISmsBinaryMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1542776851, 15187, 19566, [182, 26, 216, 106, 99, 117, 86, 80]);
}
impl ISmsBinaryMessage {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn Format(&self) -> ::windows::runtime::Result<SmsDataFormat> {
        let this = self;
        unsafe {
            let mut result__: SmsDataFormat = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SmsDataFormat>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn SetFormat(&self, value: SmsDataFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn GetData(&self) -> ::windows::runtime::Result<::windows::runtime::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::Array<u8> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), ::windows::runtime::Array::<u8>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn SetData(&self, value: &[<u8 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.len() as u32, ::std::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<ISmsMessage>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn MessageClass(&self) -> ::windows::runtime::Result<SmsMessageClass> {
        let this = &::windows::runtime::Interface::cast::<ISmsMessage>(self)?;
        unsafe {
            let mut result__: SmsMessageClass = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SmsMessageClass>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ISmsBinaryMessage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{5bf4e813-3b53-4c6e-b61a-d86a63755650}");
}
impl ::std::convert::From<ISmsBinaryMessage> for ::windows::runtime::IUnknown {
    fn from(value: ISmsBinaryMessage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISmsBinaryMessage> for ::windows::runtime::IUnknown {
    fn from(value: &ISmsBinaryMessage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISmsBinaryMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ISmsBinaryMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ISmsBinaryMessage> for ::windows::runtime::IInspectable {
    fn from(value: ISmsBinaryMessage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISmsBinaryMessage> for ::windows::runtime::IInspectable {
    fn from(value: &ISmsBinaryMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ISmsBinaryMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ISmsBinaryMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<ISmsBinaryMessage> for ISmsMessage {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ISmsBinaryMessage) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&ISmsBinaryMessage> for ISmsMessage {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ISmsBinaryMessage) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISmsMessage> for ISmsBinaryMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISmsMessage> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISmsMessage> for &ISmsBinaryMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISmsMessage> {
        ::std::convert::TryInto::<ISmsMessage>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsBinaryMessage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SmsDataFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: SmsDataFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: u32, value: *const u8) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmsBroadcastMessage(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmsBroadcastMessage {
    type Vtable = ISmsBroadcastMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1974385649, 58551, 18548, [160, 156, 41, 86, 229, 146, 249, 87]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsBroadcastMessage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SmsGeographicalScope) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SmsBroadcastType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `Devices_Sms`*"]
pub struct ISmsDevice(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmsDevice {
    type Vtable = ISmsDevice_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(152539629, 34603, 20204, [156, 114, 171, 17, 98, 123, 52, 236]);
}
impl ISmsDevice {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn SendMessageAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ISmsMessage>>(&self, message: Param0) -> ::windows::runtime::Result<SendSmsMessageOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), message.into_param().abi(), &mut result__).from_abi::<SendSmsMessageOperation>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn CalculateLength<'a, Param0: ::windows::runtime::IntoParam<'a, SmsTextMessage>>(&self, message: Param0) -> ::windows::runtime::Result<SmsEncodedLength> {
        let this = self;
        unsafe {
            let mut result__: SmsEncodedLength = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), message.into_param().abi(), &mut result__).from_abi::<SmsEncodedLength>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn AccountPhoneNumber(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn CellularClass(&self) -> ::windows::runtime::Result<CellularClass> {
        let this = self;
        unsafe {
            let mut result__: CellularClass = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CellularClass>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn MessageStore(&self) -> ::windows::runtime::Result<SmsDeviceMessageStore> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SmsDeviceMessageStore>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn DeviceStatus(&self) -> ::windows::runtime::Result<SmsDeviceStatus> {
        let this = self;
        unsafe {
            let mut result__: SmsDeviceStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SmsDeviceStatus>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn SmsMessageReceived<'a, Param0: ::windows::runtime::IntoParam<'a, SmsMessageReceivedEventHandler>>(&self, eventhandler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), eventhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn RemoveSmsMessageReceived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn SmsDeviceStatusChanged<'a, Param0: ::windows::runtime::IntoParam<'a, SmsDeviceStatusChangedEventHandler>>(&self, eventhandler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), eventhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn RemoveSmsDeviceStatusChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ISmsDevice {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{091791ed-872b-4eec-9c72-ab11627b34ec}");
}
impl ::std::convert::From<ISmsDevice> for ::windows::runtime::IUnknown {
    fn from(value: ISmsDevice) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISmsDevice> for ::windows::runtime::IUnknown {
    fn from(value: &ISmsDevice) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISmsDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ISmsDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ISmsDevice> for ::windows::runtime::IInspectable {
    fn from(value: ISmsDevice) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISmsDevice> for ::windows::runtime::IInspectable {
    fn from(value: &ISmsDevice) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ISmsDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ISmsDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsDevice_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, message: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, message: ::windows::runtime::RawPtr, result__: *mut SmsEncodedLength) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut CellularClass) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SmsDeviceStatus) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventhandler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventhandler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmsDevice2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmsDevice2 {
    type Vtable = ISmsDevice2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3179961363, 58658, 18123, [184, 213, 158, 173, 48, 251, 108, 71]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsDevice2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut CellularClass) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SmsDeviceStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, message: ::windows::runtime::RawPtr, result__: *mut SmsEncodedLength) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, message: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventhandler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmsDevice2Statics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmsDevice2Statics {
    type Vtable = ISmsDevice2Statics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1707574053, 4145, 18718, [143, 182, 239, 153, 145, 175, 227, 99]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsDevice2Statics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parentdeviceid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmsDeviceMessageStore(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmsDeviceMessageStore {
    type Vtable = ISmsDeviceMessageStore_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2559177299, 61832, 17447, [141, 84, 206, 12, 36, 35, 197, 193]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsDeviceMessageStore_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, messageid: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, messagefilter: SmsMessageFilter, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, messageid: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, messagefilter: SmsMessageFilter, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmsDeviceStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmsDeviceStatics {
    type Vtable = ISmsDeviceStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4169992170, 55317, 19921, [162, 52, 69, 32, 206, 70, 4, 164]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsDeviceStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmsDeviceStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmsDeviceStatics2 {
    type Vtable = ISmsDeviceStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(748756103, 2163, 19631, [138, 125, 189, 71, 30, 133, 134, 209]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsDeviceStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, networkaccountid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmsFilterRule(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmsFilterRule {
    type Vtable = ISmsFilterRule_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1088630702, 45129, 20412, [175, 233, 226, 166, 16, 239, 245, 92]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsFilterRule_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SmsMessageType) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut CellularClass) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: CellularClass) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmsFilterRuleFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmsFilterRuleFactory {
    type Vtable = ISmsFilterRuleFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(12805384, 25238, 20265, [154, 173, 137, 32, 206, 186, 60, 232]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsFilterRuleFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, messagetype: SmsMessageType, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmsFilterRules(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmsFilterRules {
    type Vtable = ISmsFilterRules_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1313336059, 31181, 18561, [152, 148, 85, 164, 19, 91, 35, 250]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsFilterRules_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SmsFilterActionType) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmsFilterRulesFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmsFilterRulesFactory {
    type Vtable = ISmsFilterRulesFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2694391021, 28206, 17712, [159, 222, 70, 93, 2, 238, 208, 14]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsFilterRulesFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, actiontype: SmsFilterActionType, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `Devices_Sms`*"]
pub struct ISmsMessage(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmsMessage {
    type Vtable = ISmsMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3980156456, 27012, 19207, [129, 29, 141, 89, 6, 237, 60, 234]);
}
impl ISmsMessage {
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn MessageClass(&self) -> ::windows::runtime::Result<SmsMessageClass> {
        let this = self;
        unsafe {
            let mut result__: SmsMessageClass = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SmsMessageClass>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ISmsMessage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{ed3c5e28-6984-4b07-811d-8d5906ed3cea}");
}
impl ::std::convert::From<ISmsMessage> for ::windows::runtime::IUnknown {
    fn from(value: ISmsMessage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISmsMessage> for ::windows::runtime::IUnknown {
    fn from(value: &ISmsMessage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISmsMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ISmsMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ISmsMessage> for ::windows::runtime::IInspectable {
    fn from(value: ISmsMessage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISmsMessage> for ::windows::runtime::IInspectable {
    fn from(value: &ISmsMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ISmsMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ISmsMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsMessage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SmsMessageClass) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `Devices_Sms`*"]
pub struct ISmsMessageBase(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmsMessageBase {
    type Vtable = ISmsMessageBase_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(753991216, 65104, 20422, [170, 136, 76, 207, 226, 122, 41, 234]);
}
impl ISmsMessageBase {
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn MessageType(&self) -> ::windows::runtime::Result<SmsMessageType> {
        let this = self;
        unsafe {
            let mut result__: SmsMessageType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SmsMessageType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn CellularClass(&self) -> ::windows::runtime::Result<CellularClass> {
        let this = self;
        unsafe {
            let mut result__: CellularClass = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CellularClass>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn MessageClass(&self) -> ::windows::runtime::Result<SmsMessageClass> {
        let this = self;
        unsafe {
            let mut result__: SmsMessageClass = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SmsMessageClass>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn SimIccId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ISmsMessageBase {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{2cf0fe30-fe50-4fc6-aa88-4ccfe27a29ea}");
}
impl ::std::convert::From<ISmsMessageBase> for ::windows::runtime::IUnknown {
    fn from(value: ISmsMessageBase) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISmsMessageBase> for ::windows::runtime::IUnknown {
    fn from(value: &ISmsMessageBase) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISmsMessageBase {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ISmsMessageBase {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ISmsMessageBase> for ::windows::runtime::IInspectable {
    fn from(value: ISmsMessageBase) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISmsMessageBase> for ::windows::runtime::IInspectable {
    fn from(value: &ISmsMessageBase) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ISmsMessageBase {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ISmsMessageBase {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsMessageBase_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SmsMessageType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut CellularClass) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SmsMessageClass) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmsMessageReceivedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmsMessageReceivedEventArgs {
    type Vtable = ISmsMessageReceivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(149424792, 47333, 16833, [163, 216, 211, 171, 250, 226, 38, 117]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsMessageReceivedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmsMessageReceivedTriggerDetails(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmsMessageReceivedTriggerDetails {
    type Vtable = ISmsMessageReceivedTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(735038420, 9815, 16680, [173, 95, 227, 135, 113, 50, 189, 177]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsMessageReceivedTriggerDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SmsMessageType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmsMessageRegistration(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmsMessageRegistration {
    type Vtable = ISmsMessageRegistration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(387993662, 62287, 17515, [131, 179, 15, 241, 153, 35, 180, 9]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsMessageRegistration_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventhandler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmsMessageRegistrationStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmsMessageRegistrationStatics {
    type Vtable = ISmsMessageRegistrationStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1671451748, 10392, 18296, [160, 60, 111, 153, 73, 7, 214, 58]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsMessageRegistrationStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, filterrules: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmsReceivedEventDetails(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmsReceivedEventDetails {
    type Vtable = ISmsReceivedEventDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1538592533, 58477, 19586, [132, 125, 90, 3, 4, 193, 213, 61]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsReceivedEventDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmsReceivedEventDetails2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmsReceivedEventDetails2 {
    type Vtable = ISmsReceivedEventDetails2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1088445574, 42932, 18289, [154, 231, 11, 95, 251, 18, 192, 58]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsReceivedEventDetails2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SmsMessageClass) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmsSendMessageResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmsSendMessageResult {
    type Vtable = ISmsSendMessageResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3675495154, 30921, 20459, [150, 34, 69, 35, 40, 8, 141, 98]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsSendMessageResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut CellularClass) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SmsModemErrorCode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmsStatusMessage(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmsStatusMessage {
    type Vtable = ISmsStatusMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3872555842, 46859, 18039, [147, 121, 201, 120, 63, 223, 248, 244]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsStatusMessage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `Devices_Sms`*"]
pub struct ISmsTextMessage(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmsTextMessage {
    type Vtable = ISmsTextMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3592196172, 42133, 18559, [154, 111, 151, 21, 72, 197, 188, 159]);
}
impl ISmsTextMessage {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn PartReferenceId(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn PartNumber(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn PartCount(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn To(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn SetTo<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn From(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn SetFrom<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn Body(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn SetBody<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn Encoding(&self) -> ::windows::runtime::Result<SmsEncoding> {
        let this = self;
        unsafe {
            let mut result__: SmsEncoding = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SmsEncoding>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn SetEncoding(&self, value: SmsEncoding) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation_Collections`*"]
    pub fn ToBinaryMessages(&self, format: SmsDataFormat) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<ISmsBinaryMessage>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), format, &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ISmsBinaryMessage>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<ISmsMessage>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn MessageClass(&self) -> ::windows::runtime::Result<SmsMessageClass> {
        let this = &::windows::runtime::Interface::cast::<ISmsMessage>(self)?;
        unsafe {
            let mut result__: SmsMessageClass = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SmsMessageClass>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ISmsTextMessage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{d61c904c-a495-487f-9a6f-971548c5bc9f}");
}
impl ::std::convert::From<ISmsTextMessage> for ::windows::runtime::IUnknown {
    fn from(value: ISmsTextMessage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISmsTextMessage> for ::windows::runtime::IUnknown {
    fn from(value: &ISmsTextMessage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISmsTextMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ISmsTextMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ISmsTextMessage> for ::windows::runtime::IInspectable {
    fn from(value: ISmsTextMessage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISmsTextMessage> for ::windows::runtime::IInspectable {
    fn from(value: &ISmsTextMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ISmsTextMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ISmsTextMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<ISmsTextMessage> for ISmsMessage {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ISmsTextMessage) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&ISmsTextMessage> for ISmsMessage {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ISmsTextMessage) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISmsMessage> for ISmsTextMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISmsMessage> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISmsMessage> for &ISmsTextMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISmsMessage> {
        ::std::convert::TryInto::<ISmsMessage>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsTextMessage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SmsEncoding) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: SmsEncoding) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, format: SmsDataFormat, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmsTextMessage2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmsTextMessage2 {
    type Vtable = ISmsTextMessage2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(580966547, 17749, 18261, [181, 161, 231, 253, 132, 149, 95, 141]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsTextMessage2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SmsEncoding) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: SmsEncoding) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmsTextMessageStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmsTextMessageStatics {
    type Vtable = ISmsTextMessageStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2137572845, 15564, 18339, [140, 85, 56, 13, 59, 1, 8, 146]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsTextMessageStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, binarymessage: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, format: SmsDataFormat, value_array_size: u32, value: *const u8, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmsVoicemailMessage(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmsVoicemailMessage {
    type Vtable = ISmsVoicemailMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(656056486, 38321, 17663, [188, 184, 184, 253, 215, 224, 139, 195]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsVoicemailMessage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISmsWapMessage(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISmsWapMessage {
    type Vtable = ISmsWapMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3448993603, 31317, 19771, [144, 33, 242, 46, 2, 45, 9, 197]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISmsWapMessage_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct LegacySmsApiContract(pub u8);
#[cfg(feature = "Foundation")]
#[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SendSmsMessageOperation(::windows::runtime::IInspectable);
#[cfg(feature = "Foundation")]
impl SendSmsMessageOperation {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn SetCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::AsyncActionCompletedHandler>>(&self, handler: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn Completed(&self) -> ::windows::runtime::Result<super::super::Foundation::AsyncActionCompletedHandler> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::AsyncActionCompletedHandler>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn GetResults(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<super::super::Foundation::AsyncStatus> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::AsyncStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::AsyncStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn ErrorCode(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn Cancel(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IAsyncInfo>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this)).ok() }
    }
    pub fn get(&self) -> ::windows::runtime::Result<()> {
        if self.Status()? == super::super::Foundation::AsyncStatus::Started {
            let (waiter, signaler) = ::windows::runtime::Waiter::new();
            self.SetCompleted(super::super::Foundation::AsyncActionCompletedHandler::new(move |_sender, _args| {
                unsafe {
                    signaler.signal();
                }
                Ok(())
            }))?;
        }
        self.GetResults()
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::runtime::RuntimeType for SendSmsMessageOperation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SendSmsMessageOperation;{5a648006-843a-4da9-865b-9d26e5dfad7b})");
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::runtime::Interface for SendSmsMessageOperation {
    type Vtable = super::super::Foundation::IAsyncAction_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1516535814, 33850, 19881, [134, 91, 157, 38, 229, 223, 173, 123]);
}
#[cfg(feature = "Foundation")]
impl ::windows::runtime::RuntimeName for SendSmsMessageOperation {
    const NAME: &'static str = "Windows.Devices.Sms.SendSmsMessageOperation";
}
impl ::std::future::Future for SendSmsMessageOperation {
    type Output = ::windows::runtime::Result<()>;
    fn poll(self: ::std::pin::Pin<&mut Self>, context: &mut ::std::task::Context) -> ::std::task::Poll<Self::Output> {
        if self.Status()? == super::super::Foundation::AsyncStatus::Started {
            let waker = context.waker().clone();
            let _ = self.SetCompleted(super::super::Foundation::AsyncActionCompletedHandler::new(move |_sender, _args| {
                waker.wake_by_ref();
                Ok(())
            }));
            ::std::task::Poll::Pending
        } else {
            ::std::task::Poll::Ready(self.GetResults())
        }
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<SendSmsMessageOperation> for ::windows::runtime::IUnknown {
    fn from(value: SendSmsMessageOperation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<&SendSmsMessageOperation> for ::windows::runtime::IUnknown {
    fn from(value: &SendSmsMessageOperation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SendSmsMessageOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SendSmsMessageOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<SendSmsMessageOperation> for ::windows::runtime::IInspectable {
    fn from(value: SendSmsMessageOperation) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<&SendSmsMessageOperation> for ::windows::runtime::IInspectable {
    fn from(value: &SendSmsMessageOperation) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SendSmsMessageOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SendSmsMessageOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<SendSmsMessageOperation> for super::super::Foundation::IAsyncAction {
    fn from(value: SendSmsMessageOperation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::From<&SendSmsMessageOperation> for super::super::Foundation::IAsyncAction {
    fn from(value: &SendSmsMessageOperation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IAsyncAction> for SendSmsMessageOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IAsyncAction> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::Foundation::IAsyncAction>::into(self))
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IAsyncAction> for &SendSmsMessageOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IAsyncAction> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::Foundation::IAsyncAction>::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<SendSmsMessageOperation> for super::super::Foundation::IAsyncInfo {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SendSmsMessageOperation) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&SendSmsMessageOperation> for super::super::Foundation::IAsyncInfo {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SendSmsMessageOperation) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IAsyncInfo> for SendSmsMessageOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IAsyncInfo> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IAsyncInfo> for &SendSmsMessageOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IAsyncInfo> {
        ::std::convert::TryInto::<super::super::Foundation::IAsyncInfo>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[doc = "*Required features: `Devices_Sms`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SmsAppMessage(::windows::runtime::IInspectable);
impl SmsAppMessage {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SmsAppMessage, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn To(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn SetTo<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn From(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn Body(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn SetBody<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn CallbackNumber(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn SetCallbackNumber<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn IsDeliveryNotificationEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn SetIsDeliveryNotificationEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn RetryAttemptCount(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn SetRetryAttemptCount(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn Encoding(&self) -> ::windows::runtime::Result<SmsEncoding> {
        let this = self;
        unsafe {
            let mut result__: SmsEncoding = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SmsEncoding>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn SetEncoding(&self, value: SmsEncoding) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn PortNumber(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn SetPortNumber(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn TeleserviceId(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn SetTeleserviceId(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn ProtocolId(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn SetProtocolId(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Sms`, `Storage_Streams`*"]
    pub fn BinaryBody(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Sms`, `Storage_Streams`*"]
    pub fn SetBinaryBody<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn MessageType(&self) -> ::windows::runtime::Result<SmsMessageType> {
        let this = &::windows::runtime::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__: SmsMessageType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SmsMessageType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn CellularClass(&self) -> ::windows::runtime::Result<CellularClass> {
        let this = &::windows::runtime::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__: CellularClass = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CellularClass>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn MessageClass(&self) -> ::windows::runtime::Result<SmsMessageClass> {
        let this = &::windows::runtime::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__: SmsMessageClass = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SmsMessageClass>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn SimIccId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SmsAppMessage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SmsAppMessage;{e8bb8494-d3a0-4a0a-86d7-291033a8cf54})");
}
unsafe impl ::windows::runtime::Interface for SmsAppMessage {
    type Vtable = ISmsAppMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3904603284, 54176, 18954, [134, 215, 41, 16, 51, 168, 207, 84]);
}
impl ::windows::runtime::RuntimeName for SmsAppMessage {
    const NAME: &'static str = "Windows.Devices.Sms.SmsAppMessage";
}
impl ::std::convert::From<SmsAppMessage> for ::windows::runtime::IUnknown {
    fn from(value: SmsAppMessage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SmsAppMessage> for ::windows::runtime::IUnknown {
    fn from(value: &SmsAppMessage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SmsAppMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SmsAppMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SmsAppMessage> for ::windows::runtime::IInspectable {
    fn from(value: SmsAppMessage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SmsAppMessage> for ::windows::runtime::IInspectable {
    fn from(value: &SmsAppMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SmsAppMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SmsAppMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<SmsAppMessage> for ISmsMessageBase {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SmsAppMessage) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SmsAppMessage> for ISmsMessageBase {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SmsAppMessage) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISmsMessageBase> for SmsAppMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISmsMessageBase> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISmsMessageBase> for &SmsAppMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISmsMessageBase> {
        ::std::convert::TryInto::<ISmsMessageBase>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for SmsAppMessage {}
unsafe impl ::std::marker::Sync for SmsAppMessage {}
#[doc = "*Required features: `Devices_Sms`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SmsBinaryMessage(::windows::runtime::IInspectable);
impl SmsBinaryMessage {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SmsBinaryMessage, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn Format(&self) -> ::windows::runtime::Result<SmsDataFormat> {
        let this = self;
        unsafe {
            let mut result__: SmsDataFormat = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SmsDataFormat>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn SetFormat(&self, value: SmsDataFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn GetData(&self) -> ::windows::runtime::Result<::windows::runtime::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::Array<u8> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), ::windows::runtime::Array::<u8>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn SetData(&self, value: &[<u8 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.len() as u32, ::std::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<ISmsMessage>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn MessageClass(&self) -> ::windows::runtime::Result<SmsMessageClass> {
        let this = &::windows::runtime::Interface::cast::<ISmsMessage>(self)?;
        unsafe {
            let mut result__: SmsMessageClass = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SmsMessageClass>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SmsBinaryMessage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SmsBinaryMessage;{5bf4e813-3b53-4c6e-b61a-d86a63755650})");
}
unsafe impl ::windows::runtime::Interface for SmsBinaryMessage {
    type Vtable = ISmsBinaryMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1542776851, 15187, 19566, [182, 26, 216, 106, 99, 117, 86, 80]);
}
impl ::windows::runtime::RuntimeName for SmsBinaryMessage {
    const NAME: &'static str = "Windows.Devices.Sms.SmsBinaryMessage";
}
impl ::std::convert::From<SmsBinaryMessage> for ::windows::runtime::IUnknown {
    fn from(value: SmsBinaryMessage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SmsBinaryMessage> for ::windows::runtime::IUnknown {
    fn from(value: &SmsBinaryMessage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SmsBinaryMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SmsBinaryMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SmsBinaryMessage> for ::windows::runtime::IInspectable {
    fn from(value: SmsBinaryMessage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SmsBinaryMessage> for ::windows::runtime::IInspectable {
    fn from(value: &SmsBinaryMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SmsBinaryMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SmsBinaryMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<SmsBinaryMessage> for ISmsBinaryMessage {
    fn from(value: SmsBinaryMessage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SmsBinaryMessage> for ISmsBinaryMessage {
    fn from(value: &SmsBinaryMessage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISmsBinaryMessage> for SmsBinaryMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISmsBinaryMessage> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISmsBinaryMessage>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISmsBinaryMessage> for &SmsBinaryMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISmsBinaryMessage> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISmsBinaryMessage>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::TryFrom<SmsBinaryMessage> for ISmsMessage {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SmsBinaryMessage) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SmsBinaryMessage> for ISmsMessage {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SmsBinaryMessage) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISmsMessage> for SmsBinaryMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISmsMessage> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISmsMessage> for &SmsBinaryMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISmsMessage> {
        ::std::convert::TryInto::<ISmsMessage>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for SmsBinaryMessage {}
unsafe impl ::std::marker::Sync for SmsBinaryMessage {}
#[doc = "*Required features: `Devices_Sms`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SmsBroadcastMessage(::windows::runtime::IInspectable);
impl SmsBroadcastMessage {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn To(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn Body(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn Channel(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn GeographicalScope(&self) -> ::windows::runtime::Result<SmsGeographicalScope> {
        let this = self;
        unsafe {
            let mut result__: SmsGeographicalScope = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SmsGeographicalScope>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn MessageCode(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn UpdateNumber(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn BroadcastType(&self) -> ::windows::runtime::Result<SmsBroadcastType> {
        let this = self;
        unsafe {
            let mut result__: SmsBroadcastType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SmsBroadcastType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn IsEmergencyAlert(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn IsUserPopupRequested(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn MessageType(&self) -> ::windows::runtime::Result<SmsMessageType> {
        let this = &::windows::runtime::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__: SmsMessageType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SmsMessageType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn CellularClass(&self) -> ::windows::runtime::Result<CellularClass> {
        let this = &::windows::runtime::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__: CellularClass = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CellularClass>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn MessageClass(&self) -> ::windows::runtime::Result<SmsMessageClass> {
        let this = &::windows::runtime::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__: SmsMessageClass = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SmsMessageClass>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn SimIccId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SmsBroadcastMessage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SmsBroadcastMessage;{75aebbf1-e4b7-4874-a09c-2956e592f957})");
}
unsafe impl ::windows::runtime::Interface for SmsBroadcastMessage {
    type Vtable = ISmsBroadcastMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1974385649, 58551, 18548, [160, 156, 41, 86, 229, 146, 249, 87]);
}
impl ::windows::runtime::RuntimeName for SmsBroadcastMessage {
    const NAME: &'static str = "Windows.Devices.Sms.SmsBroadcastMessage";
}
impl ::std::convert::From<SmsBroadcastMessage> for ::windows::runtime::IUnknown {
    fn from(value: SmsBroadcastMessage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SmsBroadcastMessage> for ::windows::runtime::IUnknown {
    fn from(value: &SmsBroadcastMessage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SmsBroadcastMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SmsBroadcastMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SmsBroadcastMessage> for ::windows::runtime::IInspectable {
    fn from(value: SmsBroadcastMessage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SmsBroadcastMessage> for ::windows::runtime::IInspectable {
    fn from(value: &SmsBroadcastMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SmsBroadcastMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SmsBroadcastMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<SmsBroadcastMessage> for ISmsMessageBase {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SmsBroadcastMessage) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SmsBroadcastMessage> for ISmsMessageBase {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SmsBroadcastMessage) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISmsMessageBase> for SmsBroadcastMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISmsMessageBase> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISmsMessageBase> for &SmsBroadcastMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISmsMessageBase> {
        ::std::convert::TryInto::<ISmsMessageBase>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for SmsBroadcastMessage {}
unsafe impl ::std::marker::Sync for SmsBroadcastMessage {}
#[doc = "*Required features: `Devices_Sms`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SmsBroadcastType(pub i32);
impl SmsBroadcastType {
    pub const Other: SmsBroadcastType = SmsBroadcastType(0i32);
    pub const CmasPresidential: SmsBroadcastType = SmsBroadcastType(1i32);
    pub const CmasExtreme: SmsBroadcastType = SmsBroadcastType(2i32);
    pub const CmasSevere: SmsBroadcastType = SmsBroadcastType(3i32);
    pub const CmasAmber: SmsBroadcastType = SmsBroadcastType(4i32);
    pub const CmasTest: SmsBroadcastType = SmsBroadcastType(5i32);
    pub const EUAlert1: SmsBroadcastType = SmsBroadcastType(6i32);
    pub const EUAlert2: SmsBroadcastType = SmsBroadcastType(7i32);
    pub const EUAlert3: SmsBroadcastType = SmsBroadcastType(8i32);
    pub const EUAlertAmber: SmsBroadcastType = SmsBroadcastType(9i32);
    pub const EUAlertInfo: SmsBroadcastType = SmsBroadcastType(10i32);
    pub const EtwsEarthquake: SmsBroadcastType = SmsBroadcastType(11i32);
    pub const EtwsTsunami: SmsBroadcastType = SmsBroadcastType(12i32);
    pub const EtwsTsunamiAndEarthquake: SmsBroadcastType = SmsBroadcastType(13i32);
    pub const LatAlertLocal: SmsBroadcastType = SmsBroadcastType(14i32);
}
impl ::std::convert::From<i32> for SmsBroadcastType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SmsBroadcastType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SmsBroadcastType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Sms.SmsBroadcastType;i4)");
}
impl ::windows::runtime::DefaultType for SmsBroadcastType {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Sms`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SmsDataFormat(pub i32);
impl SmsDataFormat {
    pub const Unknown: SmsDataFormat = SmsDataFormat(0i32);
    pub const CdmaSubmit: SmsDataFormat = SmsDataFormat(1i32);
    pub const GsmSubmit: SmsDataFormat = SmsDataFormat(2i32);
    pub const CdmaDeliver: SmsDataFormat = SmsDataFormat(3i32);
    pub const GsmDeliver: SmsDataFormat = SmsDataFormat(4i32);
}
impl ::std::convert::From<i32> for SmsDataFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SmsDataFormat {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SmsDataFormat {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Sms.SmsDataFormat;i4)");
}
impl ::windows::runtime::DefaultType for SmsDataFormat {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Sms`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SmsDevice(::windows::runtime::IInspectable);
impl SmsDevice {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn SendMessageAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ISmsMessage>>(&self, message: Param0) -> ::windows::runtime::Result<SendSmsMessageOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), message.into_param().abi(), &mut result__).from_abi::<SendSmsMessageOperation>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn CalculateLength<'a, Param0: ::windows::runtime::IntoParam<'a, SmsTextMessage>>(&self, message: Param0) -> ::windows::runtime::Result<SmsEncodedLength> {
        let this = self;
        unsafe {
            let mut result__: SmsEncodedLength = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), message.into_param().abi(), &mut result__).from_abi::<SmsEncodedLength>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn AccountPhoneNumber(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn CellularClass(&self) -> ::windows::runtime::Result<CellularClass> {
        let this = self;
        unsafe {
            let mut result__: CellularClass = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CellularClass>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn MessageStore(&self) -> ::windows::runtime::Result<SmsDeviceMessageStore> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SmsDeviceMessageStore>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn DeviceStatus(&self) -> ::windows::runtime::Result<SmsDeviceStatus> {
        let this = self;
        unsafe {
            let mut result__: SmsDeviceStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SmsDeviceStatus>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn SmsMessageReceived<'a, Param0: ::windows::runtime::IntoParam<'a, SmsMessageReceivedEventHandler>>(&self, eventhandler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), eventhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn RemoveSmsMessageReceived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn SmsDeviceStatusChanged<'a, Param0: ::windows::runtime::IntoParam<'a, SmsDeviceStatusChangedEventHandler>>(&self, eventhandler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), eventhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn RemoveSmsDeviceStatusChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn GetDeviceSelector() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::ISmsDeviceStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn FromIdAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(deviceid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<SmsDevice>> {
        Self::ISmsDeviceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SmsDevice>>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn GetDefaultAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<SmsDevice>> {
        Self::ISmsDeviceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SmsDevice>>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn FromNetworkAccountIdAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(networkaccountid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<SmsDevice>> {
        Self::ISmsDeviceStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), networkaccountid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SmsDevice>>(result__)
        })
    }
    pub fn ISmsDeviceStatics<R, F: FnOnce(&ISmsDeviceStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SmsDevice, ISmsDeviceStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ISmsDeviceStatics2<R, F: FnOnce(&ISmsDeviceStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SmsDevice, ISmsDeviceStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SmsDevice {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SmsDevice;{091791ed-872b-4eec-9c72-ab11627b34ec})");
}
unsafe impl ::windows::runtime::Interface for SmsDevice {
    type Vtable = ISmsDevice_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(152539629, 34603, 20204, [156, 114, 171, 17, 98, 123, 52, 236]);
}
impl ::windows::runtime::RuntimeName for SmsDevice {
    const NAME: &'static str = "Windows.Devices.Sms.SmsDevice";
}
impl ::std::convert::From<SmsDevice> for ::windows::runtime::IUnknown {
    fn from(value: SmsDevice) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SmsDevice> for ::windows::runtime::IUnknown {
    fn from(value: &SmsDevice) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SmsDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SmsDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SmsDevice> for ::windows::runtime::IInspectable {
    fn from(value: SmsDevice) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SmsDevice> for ::windows::runtime::IInspectable {
    fn from(value: &SmsDevice) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SmsDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SmsDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<SmsDevice> for ISmsDevice {
    fn from(value: SmsDevice) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SmsDevice> for ISmsDevice {
    fn from(value: &SmsDevice) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISmsDevice> for SmsDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISmsDevice> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISmsDevice>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISmsDevice> for &SmsDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISmsDevice> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISmsDevice>::into(::std::clone::Clone::clone(self)))
    }
}
#[doc = "*Required features: `Devices_Sms`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SmsDevice2(::windows::runtime::IInspectable);
impl SmsDevice2 {
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn SmscAddress(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn SetSmscAddress<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn ParentDeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn AccountPhoneNumber(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn CellularClass(&self) -> ::windows::runtime::Result<CellularClass> {
        let this = self;
        unsafe {
            let mut result__: CellularClass = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CellularClass>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn DeviceStatus(&self) -> ::windows::runtime::Result<SmsDeviceStatus> {
        let this = self;
        unsafe {
            let mut result__: SmsDeviceStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SmsDeviceStatus>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn CalculateLength<'a, Param0: ::windows::runtime::IntoParam<'a, ISmsMessageBase>>(&self, message: Param0) -> ::windows::runtime::Result<SmsEncodedLength> {
        let this = self;
        unsafe {
            let mut result__: SmsEncodedLength = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), message.into_param().abi(), &mut result__).from_abi::<SmsEncodedLength>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn SendMessageAndGetResultAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ISmsMessageBase>>(&self, message: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<SmsSendMessageResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), message.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SmsSendMessageResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn DeviceStatusChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<SmsDevice2, ::windows::runtime::IInspectable>>>(&self, eventhandler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), eventhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn RemoveDeviceStatusChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn GetDeviceSelector() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::ISmsDevice2Statics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn FromId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(deviceid: Param0) -> ::windows::runtime::Result<SmsDevice2> {
        Self::ISmsDevice2Statics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<SmsDevice2>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn GetDefault() -> ::windows::runtime::Result<SmsDevice2> {
        Self::ISmsDevice2Statics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SmsDevice2>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn FromParentId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(parentdeviceid: Param0) -> ::windows::runtime::Result<SmsDevice2> {
        Self::ISmsDevice2Statics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), parentdeviceid.into_param().abi(), &mut result__).from_abi::<SmsDevice2>(result__)
        })
    }
    pub fn ISmsDevice2Statics<R, F: FnOnce(&ISmsDevice2Statics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SmsDevice2, ISmsDevice2Statics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SmsDevice2 {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SmsDevice2;{bd8a5c13-e522-46cb-b8d5-9ead30fb6c47})");
}
unsafe impl ::windows::runtime::Interface for SmsDevice2 {
    type Vtable = ISmsDevice2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3179961363, 58658, 18123, [184, 213, 158, 173, 48, 251, 108, 71]);
}
impl ::windows::runtime::RuntimeName for SmsDevice2 {
    const NAME: &'static str = "Windows.Devices.Sms.SmsDevice2";
}
impl ::std::convert::From<SmsDevice2> for ::windows::runtime::IUnknown {
    fn from(value: SmsDevice2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SmsDevice2> for ::windows::runtime::IUnknown {
    fn from(value: &SmsDevice2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SmsDevice2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SmsDevice2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SmsDevice2> for ::windows::runtime::IInspectable {
    fn from(value: SmsDevice2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SmsDevice2> for ::windows::runtime::IInspectable {
    fn from(value: &SmsDevice2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SmsDevice2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SmsDevice2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[doc = "*Required features: `Devices_Sms`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SmsDeviceMessageStore(::windows::runtime::IInspectable);
impl SmsDeviceMessageStore {
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn DeleteMessageAsync(&self, messageid: u32) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), messageid, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn DeleteMessagesAsync(&self, messagefilter: SmsMessageFilter) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), messagefilter, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn GetMessageAsync(&self, messageid: u32) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ISmsMessage>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), messageid, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ISmsMessage>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetMessagesAsync(&self, messagefilter: SmsMessageFilter) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Foundation::Collections::IVectorView<ISmsMessage>, i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), messagefilter, &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<super::super::Foundation::Collections::IVectorView<ISmsMessage>, i32>>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn MaxMessages(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SmsDeviceMessageStore {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SmsDeviceMessageStore;{9889f253-f188-4427-8d54-ce0c2423c5c1})");
}
unsafe impl ::windows::runtime::Interface for SmsDeviceMessageStore {
    type Vtable = ISmsDeviceMessageStore_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2559177299, 61832, 17447, [141, 84, 206, 12, 36, 35, 197, 193]);
}
impl ::windows::runtime::RuntimeName for SmsDeviceMessageStore {
    const NAME: &'static str = "Windows.Devices.Sms.SmsDeviceMessageStore";
}
impl ::std::convert::From<SmsDeviceMessageStore> for ::windows::runtime::IUnknown {
    fn from(value: SmsDeviceMessageStore) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SmsDeviceMessageStore> for ::windows::runtime::IUnknown {
    fn from(value: &SmsDeviceMessageStore) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SmsDeviceMessageStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SmsDeviceMessageStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SmsDeviceMessageStore> for ::windows::runtime::IInspectable {
    fn from(value: SmsDeviceMessageStore) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SmsDeviceMessageStore> for ::windows::runtime::IInspectable {
    fn from(value: &SmsDeviceMessageStore) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SmsDeviceMessageStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SmsDeviceMessageStore {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[doc = "*Required features: `Devices_Sms`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SmsDeviceStatus(pub i32);
impl SmsDeviceStatus {
    pub const Off: SmsDeviceStatus = SmsDeviceStatus(0i32);
    pub const Ready: SmsDeviceStatus = SmsDeviceStatus(1i32);
    pub const SimNotInserted: SmsDeviceStatus = SmsDeviceStatus(2i32);
    pub const BadSim: SmsDeviceStatus = SmsDeviceStatus(3i32);
    pub const DeviceFailure: SmsDeviceStatus = SmsDeviceStatus(4i32);
    pub const SubscriptionNotActivated: SmsDeviceStatus = SmsDeviceStatus(5i32);
    pub const DeviceLocked: SmsDeviceStatus = SmsDeviceStatus(6i32);
    pub const DeviceBlocked: SmsDeviceStatus = SmsDeviceStatus(7i32);
}
impl ::std::convert::From<i32> for SmsDeviceStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SmsDeviceStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SmsDeviceStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Sms.SmsDeviceStatus;i4)");
}
impl ::windows::runtime::DefaultType for SmsDeviceStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Sms`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SmsDeviceStatusChangedEventHandler(::windows::runtime::IUnknown);
impl SmsDeviceStatusChangedEventHandler {
    pub fn new<F: FnMut(&::std::option::Option<SmsDevice>) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = SmsDeviceStatusChangedEventHandler_box::<F> {
            vtable: &SmsDeviceStatusChangedEventHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, SmsDevice>>(&self, sender: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::std::mem::transmute_copy(this), sender.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SmsDeviceStatusChangedEventHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({982b1162-3dd7-4618-af89-0c272d5d06d8})");
}
unsafe impl ::windows::runtime::Interface for SmsDeviceStatusChangedEventHandler {
    type Vtable = SmsDeviceStatusChangedEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2552959330, 15831, 17944, [175, 137, 12, 39, 45, 93, 6, 216]);
}
#[repr(C)]
#[doc(hidden)]
pub struct SmsDeviceStatusChangedEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct SmsDeviceStatusChangedEventHandler_box<F: FnMut(&::std::option::Option<SmsDevice>) -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const SmsDeviceStatusChangedEventHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<F: FnMut(&::std::option::Option<SmsDevice>) -> ::windows::runtime::Result<()> + 'static> SmsDeviceStatusChangedEventHandler_box<F> {
    const VTABLE: SmsDeviceStatusChangedEventHandler_abi = SmsDeviceStatusChangedEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<SmsDeviceStatusChangedEventHandler as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::std::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(&*(&sender as *const <SmsDevice as ::windows::runtime::Abi>::Abi as *const <SmsDevice as ::windows::runtime::DefaultType>::DefaultType)).into()
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Devices_Sms`*"]
pub struct SmsEncodedLength {
    pub SegmentCount: u32,
    pub CharacterCountLastSegment: u32,
    pub CharactersPerSegment: u32,
    pub ByteCountLastSegment: u32,
    pub BytesPerSegment: u32,
}
impl SmsEncodedLength {}
impl ::std::default::Default for SmsEncodedLength {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SmsEncodedLength {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SmsEncodedLength")
            .field("SegmentCount", &self.SegmentCount)
            .field("CharacterCountLastSegment", &self.CharacterCountLastSegment)
            .field("CharactersPerSegment", &self.CharactersPerSegment)
            .field("ByteCountLastSegment", &self.ByteCountLastSegment)
            .field("BytesPerSegment", &self.BytesPerSegment)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SmsEncodedLength {
    fn eq(&self, other: &Self) -> bool {
        self.SegmentCount == other.SegmentCount && self.CharacterCountLastSegment == other.CharacterCountLastSegment && self.CharactersPerSegment == other.CharactersPerSegment && self.ByteCountLastSegment == other.ByteCountLastSegment && self.BytesPerSegment == other.BytesPerSegment
    }
}
impl ::std::cmp::Eq for SmsEncodedLength {}
unsafe impl ::windows::runtime::Abi for SmsEncodedLength {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SmsEncodedLength {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Devices.Sms.SmsEncodedLength;u4;u4;u4;u4;u4)");
}
impl ::windows::runtime::DefaultType for SmsEncodedLength {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Sms`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SmsEncoding(pub i32);
impl SmsEncoding {
    pub const Unknown: SmsEncoding = SmsEncoding(0i32);
    pub const Optimal: SmsEncoding = SmsEncoding(1i32);
    pub const SevenBitAscii: SmsEncoding = SmsEncoding(2i32);
    pub const Unicode: SmsEncoding = SmsEncoding(3i32);
    pub const GsmSevenBit: SmsEncoding = SmsEncoding(4i32);
    pub const EightBit: SmsEncoding = SmsEncoding(5i32);
    pub const Latin: SmsEncoding = SmsEncoding(6i32);
    pub const Korean: SmsEncoding = SmsEncoding(7i32);
    pub const IA5: SmsEncoding = SmsEncoding(8i32);
    pub const ShiftJis: SmsEncoding = SmsEncoding(9i32);
    pub const LatinHebrew: SmsEncoding = SmsEncoding(10i32);
}
impl ::std::convert::From<i32> for SmsEncoding {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SmsEncoding {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SmsEncoding {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Sms.SmsEncoding;i4)");
}
impl ::windows::runtime::DefaultType for SmsEncoding {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Sms`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SmsFilterActionType(pub i32);
impl SmsFilterActionType {
    pub const AcceptImmediately: SmsFilterActionType = SmsFilterActionType(0i32);
    pub const Drop: SmsFilterActionType = SmsFilterActionType(1i32);
    pub const Peek: SmsFilterActionType = SmsFilterActionType(2i32);
    pub const Accept: SmsFilterActionType = SmsFilterActionType(3i32);
}
impl ::std::convert::From<i32> for SmsFilterActionType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SmsFilterActionType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SmsFilterActionType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Sms.SmsFilterActionType;i4)");
}
impl ::windows::runtime::DefaultType for SmsFilterActionType {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Sms`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SmsFilterRule(::windows::runtime::IInspectable);
impl SmsFilterRule {
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn MessageType(&self) -> ::windows::runtime::Result<SmsMessageType> {
        let this = self;
        unsafe {
            let mut result__: SmsMessageType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SmsMessageType>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation_Collections`*"]
    pub fn ImsiPrefixes(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation_Collections`*"]
    pub fn DeviceIds(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation_Collections`*"]
    pub fn SenderNumbers(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation_Collections`*"]
    pub fn TextMessagePrefixes(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation_Collections`*"]
    pub fn PortNumbers(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<i32>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn CellularClass(&self) -> ::windows::runtime::Result<CellularClass> {
        let this = self;
        unsafe {
            let mut result__: CellularClass = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CellularClass>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn SetCellularClass(&self, value: CellularClass) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation_Collections`*"]
    pub fn ProtocolIds(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation_Collections`*"]
    pub fn TeleserviceIds(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation_Collections`*"]
    pub fn WapApplicationIds(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation_Collections`*"]
    pub fn WapContentTypes(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation_Collections`*"]
    pub fn BroadcastTypes(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<SmsBroadcastType>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<SmsBroadcastType>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation_Collections`*"]
    pub fn BroadcastChannels(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<i32>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn CreateFilterRule(messagetype: SmsMessageType) -> ::windows::runtime::Result<SmsFilterRule> {
        Self::ISmsFilterRuleFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), messagetype, &mut result__).from_abi::<SmsFilterRule>(result__)
        })
    }
    pub fn ISmsFilterRuleFactory<R, F: FnOnce(&ISmsFilterRuleFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SmsFilterRule, ISmsFilterRuleFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SmsFilterRule {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SmsFilterRule;{40e32fae-b049-4fbc-afe9-e2a610eff55c})");
}
unsafe impl ::windows::runtime::Interface for SmsFilterRule {
    type Vtable = ISmsFilterRule_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1088630702, 45129, 20412, [175, 233, 226, 166, 16, 239, 245, 92]);
}
impl ::windows::runtime::RuntimeName for SmsFilterRule {
    const NAME: &'static str = "Windows.Devices.Sms.SmsFilterRule";
}
impl ::std::convert::From<SmsFilterRule> for ::windows::runtime::IUnknown {
    fn from(value: SmsFilterRule) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SmsFilterRule> for ::windows::runtime::IUnknown {
    fn from(value: &SmsFilterRule) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SmsFilterRule {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SmsFilterRule {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SmsFilterRule> for ::windows::runtime::IInspectable {
    fn from(value: SmsFilterRule) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SmsFilterRule> for ::windows::runtime::IInspectable {
    fn from(value: &SmsFilterRule) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SmsFilterRule {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SmsFilterRule {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SmsFilterRule {}
unsafe impl ::std::marker::Sync for SmsFilterRule {}
#[doc = "*Required features: `Devices_Sms`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SmsFilterRules(::windows::runtime::IInspectable);
impl SmsFilterRules {
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn ActionType(&self) -> ::windows::runtime::Result<SmsFilterActionType> {
        let this = self;
        unsafe {
            let mut result__: SmsFilterActionType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SmsFilterActionType>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation_Collections`*"]
    pub fn Rules(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<SmsFilterRule>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<SmsFilterRule>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn CreateFilterRules(actiontype: SmsFilterActionType) -> ::windows::runtime::Result<SmsFilterRules> {
        Self::ISmsFilterRulesFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), actiontype, &mut result__).from_abi::<SmsFilterRules>(result__)
        })
    }
    pub fn ISmsFilterRulesFactory<R, F: FnOnce(&ISmsFilterRulesFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SmsFilterRules, ISmsFilterRulesFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SmsFilterRules {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SmsFilterRules;{4e47eafb-79cd-4881-9894-55a4135b23fa})");
}
unsafe impl ::windows::runtime::Interface for SmsFilterRules {
    type Vtable = ISmsFilterRules_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1313336059, 31181, 18561, [152, 148, 85, 164, 19, 91, 35, 250]);
}
impl ::windows::runtime::RuntimeName for SmsFilterRules {
    const NAME: &'static str = "Windows.Devices.Sms.SmsFilterRules";
}
impl ::std::convert::From<SmsFilterRules> for ::windows::runtime::IUnknown {
    fn from(value: SmsFilterRules) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SmsFilterRules> for ::windows::runtime::IUnknown {
    fn from(value: &SmsFilterRules) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SmsFilterRules {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SmsFilterRules {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SmsFilterRules> for ::windows::runtime::IInspectable {
    fn from(value: SmsFilterRules) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SmsFilterRules> for ::windows::runtime::IInspectable {
    fn from(value: &SmsFilterRules) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SmsFilterRules {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SmsFilterRules {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SmsFilterRules {}
unsafe impl ::std::marker::Sync for SmsFilterRules {}
#[doc = "*Required features: `Devices_Sms`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SmsGeographicalScope(pub i32);
impl SmsGeographicalScope {
    pub const None: SmsGeographicalScope = SmsGeographicalScope(0i32);
    pub const CellWithImmediateDisplay: SmsGeographicalScope = SmsGeographicalScope(1i32);
    pub const LocationArea: SmsGeographicalScope = SmsGeographicalScope(2i32);
    pub const Plmn: SmsGeographicalScope = SmsGeographicalScope(3i32);
    pub const Cell: SmsGeographicalScope = SmsGeographicalScope(4i32);
}
impl ::std::convert::From<i32> for SmsGeographicalScope {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SmsGeographicalScope {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SmsGeographicalScope {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Sms.SmsGeographicalScope;i4)");
}
impl ::windows::runtime::DefaultType for SmsGeographicalScope {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Sms`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SmsMessageClass(pub i32);
impl SmsMessageClass {
    pub const None: SmsMessageClass = SmsMessageClass(0i32);
    pub const Class0: SmsMessageClass = SmsMessageClass(1i32);
    pub const Class1: SmsMessageClass = SmsMessageClass(2i32);
    pub const Class2: SmsMessageClass = SmsMessageClass(3i32);
    pub const Class3: SmsMessageClass = SmsMessageClass(4i32);
}
impl ::std::convert::From<i32> for SmsMessageClass {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SmsMessageClass {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SmsMessageClass {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Sms.SmsMessageClass;i4)");
}
impl ::windows::runtime::DefaultType for SmsMessageClass {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Sms`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SmsMessageFilter(pub i32);
impl SmsMessageFilter {
    pub const All: SmsMessageFilter = SmsMessageFilter(0i32);
    pub const Unread: SmsMessageFilter = SmsMessageFilter(1i32);
    pub const Read: SmsMessageFilter = SmsMessageFilter(2i32);
    pub const Sent: SmsMessageFilter = SmsMessageFilter(3i32);
    pub const Draft: SmsMessageFilter = SmsMessageFilter(4i32);
}
impl ::std::convert::From<i32> for SmsMessageFilter {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SmsMessageFilter {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SmsMessageFilter {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Sms.SmsMessageFilter;i4)");
}
impl ::windows::runtime::DefaultType for SmsMessageFilter {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Sms`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SmsMessageReceivedEventArgs(::windows::runtime::IInspectable);
impl SmsMessageReceivedEventArgs {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn TextMessage(&self) -> ::windows::runtime::Result<SmsTextMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SmsTextMessage>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn BinaryMessage(&self) -> ::windows::runtime::Result<SmsBinaryMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SmsBinaryMessage>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SmsMessageReceivedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SmsMessageReceivedEventArgs;{08e80a98-b8e5-41c1-a3d8-d3abfae22675})");
}
unsafe impl ::windows::runtime::Interface for SmsMessageReceivedEventArgs {
    type Vtable = ISmsMessageReceivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(149424792, 47333, 16833, [163, 216, 211, 171, 250, 226, 38, 117]);
}
impl ::windows::runtime::RuntimeName for SmsMessageReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.Sms.SmsMessageReceivedEventArgs";
}
impl ::std::convert::From<SmsMessageReceivedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SmsMessageReceivedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SmsMessageReceivedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SmsMessageReceivedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SmsMessageReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SmsMessageReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SmsMessageReceivedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SmsMessageReceivedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SmsMessageReceivedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &SmsMessageReceivedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SmsMessageReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SmsMessageReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[doc = "*Required features: `Devices_Sms`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SmsMessageReceivedEventHandler(::windows::runtime::IUnknown);
impl SmsMessageReceivedEventHandler {
    pub fn new<F: FnMut(&::std::option::Option<SmsDevice>, &::std::option::Option<SmsMessageReceivedEventArgs>) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = SmsMessageReceivedEventHandler_box::<F> {
            vtable: &SmsMessageReceivedEventHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, SmsDevice>, Param1: ::windows::runtime::IntoParam<'a, SmsMessageReceivedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::std::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SmsMessageReceivedEventHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({0b7ad409-ec2d-47ce-a253-732beeebcacd})");
}
unsafe impl ::windows::runtime::Interface for SmsMessageReceivedEventHandler {
    type Vtable = SmsMessageReceivedEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(192599049, 60461, 18382, [162, 83, 115, 43, 238, 235, 202, 205]);
}
#[repr(C)]
#[doc(hidden)]
pub struct SmsMessageReceivedEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, e: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct SmsMessageReceivedEventHandler_box<F: FnMut(&::std::option::Option<SmsDevice>, &::std::option::Option<SmsMessageReceivedEventArgs>) -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const SmsMessageReceivedEventHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<F: FnMut(&::std::option::Option<SmsDevice>, &::std::option::Option<SmsMessageReceivedEventArgs>) -> ::windows::runtime::Result<()> + 'static> SmsMessageReceivedEventHandler_box<F> {
    const VTABLE: SmsMessageReceivedEventHandler_abi = SmsMessageReceivedEventHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<SmsMessageReceivedEventHandler as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::std::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, e: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&sender as *const <SmsDevice as ::windows::runtime::Abi>::Abi as *const <SmsDevice as ::windows::runtime::DefaultType>::DefaultType),
            &*(&e as *const <SmsMessageReceivedEventArgs as ::windows::runtime::Abi>::Abi as *const <SmsMessageReceivedEventArgs as ::windows::runtime::DefaultType>::DefaultType),
        )
        .into()
    }
}
#[doc = "*Required features: `Devices_Sms`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SmsMessageReceivedTriggerDetails(::windows::runtime::IInspectable);
impl SmsMessageReceivedTriggerDetails {
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn MessageType(&self) -> ::windows::runtime::Result<SmsMessageType> {
        let this = self;
        unsafe {
            let mut result__: SmsMessageType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SmsMessageType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn TextMessage(&self) -> ::windows::runtime::Result<SmsTextMessage2> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SmsTextMessage2>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn WapMessage(&self) -> ::windows::runtime::Result<SmsWapMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SmsWapMessage>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn AppMessage(&self) -> ::windows::runtime::Result<SmsAppMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SmsAppMessage>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn BroadcastMessage(&self) -> ::windows::runtime::Result<SmsBroadcastMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SmsBroadcastMessage>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn VoicemailMessage(&self) -> ::windows::runtime::Result<SmsVoicemailMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SmsVoicemailMessage>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn StatusMessage(&self) -> ::windows::runtime::Result<SmsStatusMessage> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SmsStatusMessage>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn Drop(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn Accept(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SmsMessageReceivedTriggerDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SmsMessageReceivedTriggerDetails;{2bcfcbd4-2657-4128-ad5f-e3877132bdb1})");
}
unsafe impl ::windows::runtime::Interface for SmsMessageReceivedTriggerDetails {
    type Vtable = ISmsMessageReceivedTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(735038420, 9815, 16680, [173, 95, 227, 135, 113, 50, 189, 177]);
}
impl ::windows::runtime::RuntimeName for SmsMessageReceivedTriggerDetails {
    const NAME: &'static str = "Windows.Devices.Sms.SmsMessageReceivedTriggerDetails";
}
impl ::std::convert::From<SmsMessageReceivedTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: SmsMessageReceivedTriggerDetails) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SmsMessageReceivedTriggerDetails> for ::windows::runtime::IUnknown {
    fn from(value: &SmsMessageReceivedTriggerDetails) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SmsMessageReceivedTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SmsMessageReceivedTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SmsMessageReceivedTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: SmsMessageReceivedTriggerDetails) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SmsMessageReceivedTriggerDetails> for ::windows::runtime::IInspectable {
    fn from(value: &SmsMessageReceivedTriggerDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SmsMessageReceivedTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SmsMessageReceivedTriggerDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SmsMessageReceivedTriggerDetails {}
unsafe impl ::std::marker::Sync for SmsMessageReceivedTriggerDetails {}
#[doc = "*Required features: `Devices_Sms`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SmsMessageRegistration(::windows::runtime::IInspectable);
impl SmsMessageRegistration {
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn Unregister(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn MessageReceived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<SmsMessageRegistration, SmsMessageReceivedTriggerDetails>>>(&self, eventhandler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), eventhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn RemoveMessageReceived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation_Collections`*"]
    pub fn AllRegistrations() -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<SmsMessageRegistration>> {
        Self::ISmsMessageRegistrationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<SmsMessageRegistration>>(result__)
        })
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn Register<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, SmsFilterRules>>(id: Param0, filterrules: Param1) -> ::windows::runtime::Result<SmsMessageRegistration> {
        Self::ISmsMessageRegistrationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), id.into_param().abi(), filterrules.into_param().abi(), &mut result__).from_abi::<SmsMessageRegistration>(result__)
        })
    }
    pub fn ISmsMessageRegistrationStatics<R, F: FnOnce(&ISmsMessageRegistrationStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SmsMessageRegistration, ISmsMessageRegistrationStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SmsMessageRegistration {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SmsMessageRegistration;{1720503e-f34f-446b-83b3-0ff19923b409})");
}
unsafe impl ::windows::runtime::Interface for SmsMessageRegistration {
    type Vtable = ISmsMessageRegistration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(387993662, 62287, 17515, [131, 179, 15, 241, 153, 35, 180, 9]);
}
impl ::windows::runtime::RuntimeName for SmsMessageRegistration {
    const NAME: &'static str = "Windows.Devices.Sms.SmsMessageRegistration";
}
impl ::std::convert::From<SmsMessageRegistration> for ::windows::runtime::IUnknown {
    fn from(value: SmsMessageRegistration) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SmsMessageRegistration> for ::windows::runtime::IUnknown {
    fn from(value: &SmsMessageRegistration) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SmsMessageRegistration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SmsMessageRegistration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SmsMessageRegistration> for ::windows::runtime::IInspectable {
    fn from(value: SmsMessageRegistration) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SmsMessageRegistration> for ::windows::runtime::IInspectable {
    fn from(value: &SmsMessageRegistration) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SmsMessageRegistration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SmsMessageRegistration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[doc = "*Required features: `Devices_Sms`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SmsMessageType(pub i32);
impl SmsMessageType {
    pub const Binary: SmsMessageType = SmsMessageType(0i32);
    pub const Text: SmsMessageType = SmsMessageType(1i32);
    pub const Wap: SmsMessageType = SmsMessageType(2i32);
    pub const App: SmsMessageType = SmsMessageType(3i32);
    pub const Broadcast: SmsMessageType = SmsMessageType(4i32);
    pub const Voicemail: SmsMessageType = SmsMessageType(5i32);
    pub const Status: SmsMessageType = SmsMessageType(6i32);
}
impl ::std::convert::From<i32> for SmsMessageType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SmsMessageType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SmsMessageType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Sms.SmsMessageType;i4)");
}
impl ::windows::runtime::DefaultType for SmsMessageType {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Sms`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SmsModemErrorCode(pub i32);
impl SmsModemErrorCode {
    pub const Other: SmsModemErrorCode = SmsModemErrorCode(0i32);
    pub const MessagingNetworkError: SmsModemErrorCode = SmsModemErrorCode(1i32);
    pub const SmsOperationNotSupportedByDevice: SmsModemErrorCode = SmsModemErrorCode(2i32);
    pub const SmsServiceNotSupportedByNetwork: SmsModemErrorCode = SmsModemErrorCode(3i32);
    pub const DeviceFailure: SmsModemErrorCode = SmsModemErrorCode(4i32);
    pub const MessageNotEncodedProperly: SmsModemErrorCode = SmsModemErrorCode(5i32);
    pub const MessageTooLarge: SmsModemErrorCode = SmsModemErrorCode(6i32);
    pub const DeviceNotReady: SmsModemErrorCode = SmsModemErrorCode(7i32);
    pub const NetworkNotReady: SmsModemErrorCode = SmsModemErrorCode(8i32);
    pub const InvalidSmscAddress: SmsModemErrorCode = SmsModemErrorCode(9i32);
    pub const NetworkFailure: SmsModemErrorCode = SmsModemErrorCode(10i32);
    pub const FixedDialingNumberRestricted: SmsModemErrorCode = SmsModemErrorCode(11i32);
}
impl ::std::convert::From<i32> for SmsModemErrorCode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SmsModemErrorCode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SmsModemErrorCode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.Sms.SmsModemErrorCode;i4)");
}
impl ::windows::runtime::DefaultType for SmsModemErrorCode {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_Sms`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SmsReceivedEventDetails(::windows::runtime::IInspectable);
impl SmsReceivedEventDetails {
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn MessageIndex(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn MessageClass(&self) -> ::windows::runtime::Result<SmsMessageClass> {
        let this = &::windows::runtime::Interface::cast::<ISmsReceivedEventDetails2>(self)?;
        unsafe {
            let mut result__: SmsMessageClass = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SmsMessageClass>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn BinaryMessage(&self) -> ::windows::runtime::Result<SmsBinaryMessage> {
        let this = &::windows::runtime::Interface::cast::<ISmsReceivedEventDetails2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SmsBinaryMessage>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SmsReceivedEventDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SmsReceivedEventDetails;{5bb50f15-e46d-4c82-847d-5a0304c1d53d})");
}
unsafe impl ::windows::runtime::Interface for SmsReceivedEventDetails {
    type Vtable = ISmsReceivedEventDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1538592533, 58477, 19586, [132, 125, 90, 3, 4, 193, 213, 61]);
}
impl ::windows::runtime::RuntimeName for SmsReceivedEventDetails {
    const NAME: &'static str = "Windows.Devices.Sms.SmsReceivedEventDetails";
}
impl ::std::convert::From<SmsReceivedEventDetails> for ::windows::runtime::IUnknown {
    fn from(value: SmsReceivedEventDetails) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SmsReceivedEventDetails> for ::windows::runtime::IUnknown {
    fn from(value: &SmsReceivedEventDetails) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SmsReceivedEventDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SmsReceivedEventDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SmsReceivedEventDetails> for ::windows::runtime::IInspectable {
    fn from(value: SmsReceivedEventDetails) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SmsReceivedEventDetails> for ::windows::runtime::IInspectable {
    fn from(value: &SmsReceivedEventDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SmsReceivedEventDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SmsReceivedEventDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SmsReceivedEventDetails {}
unsafe impl ::std::marker::Sync for SmsReceivedEventDetails {}
#[doc = "*Required features: `Devices_Sms`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SmsSendMessageResult(::windows::runtime::IInspectable);
impl SmsSendMessageResult {
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn IsSuccessful(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation_Collections`*"]
    pub fn MessageReferenceNumbers(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<i32>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn CellularClass(&self) -> ::windows::runtime::Result<CellularClass> {
        let this = self;
        unsafe {
            let mut result__: CellularClass = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CellularClass>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn ModemErrorCode(&self) -> ::windows::runtime::Result<SmsModemErrorCode> {
        let this = self;
        unsafe {
            let mut result__: SmsModemErrorCode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SmsModemErrorCode>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn IsErrorTransient(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn NetworkCauseCode(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn TransportFailureCause(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SmsSendMessageResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SmsSendMessageResult;{db139af2-78c9-4feb-9622-452328088d62})");
}
unsafe impl ::windows::runtime::Interface for SmsSendMessageResult {
    type Vtable = ISmsSendMessageResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3675495154, 30921, 20459, [150, 34, 69, 35, 40, 8, 141, 98]);
}
impl ::windows::runtime::RuntimeName for SmsSendMessageResult {
    const NAME: &'static str = "Windows.Devices.Sms.SmsSendMessageResult";
}
impl ::std::convert::From<SmsSendMessageResult> for ::windows::runtime::IUnknown {
    fn from(value: SmsSendMessageResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SmsSendMessageResult> for ::windows::runtime::IUnknown {
    fn from(value: &SmsSendMessageResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SmsSendMessageResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SmsSendMessageResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SmsSendMessageResult> for ::windows::runtime::IInspectable {
    fn from(value: SmsSendMessageResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SmsSendMessageResult> for ::windows::runtime::IInspectable {
    fn from(value: &SmsSendMessageResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SmsSendMessageResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SmsSendMessageResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for SmsSendMessageResult {}
unsafe impl ::std::marker::Sync for SmsSendMessageResult {}
#[doc = "*Required features: `Devices_Sms`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SmsStatusMessage(::windows::runtime::IInspectable);
impl SmsStatusMessage {
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn To(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn From(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn Body(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn MessageReferenceNumber(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn ServiceCenterTimestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn DischargeTime(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn MessageType(&self) -> ::windows::runtime::Result<SmsMessageType> {
        let this = &::windows::runtime::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__: SmsMessageType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SmsMessageType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn CellularClass(&self) -> ::windows::runtime::Result<CellularClass> {
        let this = &::windows::runtime::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__: CellularClass = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CellularClass>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn MessageClass(&self) -> ::windows::runtime::Result<SmsMessageClass> {
        let this = &::windows::runtime::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__: SmsMessageClass = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SmsMessageClass>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn SimIccId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SmsStatusMessage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SmsStatusMessage;{e6d28342-b70b-4677-9379-c9783fdff8f4})");
}
unsafe impl ::windows::runtime::Interface for SmsStatusMessage {
    type Vtable = ISmsStatusMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3872555842, 46859, 18039, [147, 121, 201, 120, 63, 223, 248, 244]);
}
impl ::windows::runtime::RuntimeName for SmsStatusMessage {
    const NAME: &'static str = "Windows.Devices.Sms.SmsStatusMessage";
}
impl ::std::convert::From<SmsStatusMessage> for ::windows::runtime::IUnknown {
    fn from(value: SmsStatusMessage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SmsStatusMessage> for ::windows::runtime::IUnknown {
    fn from(value: &SmsStatusMessage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SmsStatusMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SmsStatusMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SmsStatusMessage> for ::windows::runtime::IInspectable {
    fn from(value: SmsStatusMessage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SmsStatusMessage> for ::windows::runtime::IInspectable {
    fn from(value: &SmsStatusMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SmsStatusMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SmsStatusMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<SmsStatusMessage> for ISmsMessageBase {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SmsStatusMessage) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SmsStatusMessage> for ISmsMessageBase {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SmsStatusMessage) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISmsMessageBase> for SmsStatusMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISmsMessageBase> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISmsMessageBase> for &SmsStatusMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISmsMessageBase> {
        ::std::convert::TryInto::<ISmsMessageBase>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for SmsStatusMessage {}
unsafe impl ::std::marker::Sync for SmsStatusMessage {}
#[doc = "*Required features: `Devices_Sms`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SmsTextMessage(::windows::runtime::IInspectable);
impl SmsTextMessage {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SmsTextMessage, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn PartReferenceId(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn PartNumber(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn PartCount(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn To(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn SetTo<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn From(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn SetFrom<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn Body(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn SetBody<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn Encoding(&self) -> ::windows::runtime::Result<SmsEncoding> {
        let this = self;
        unsafe {
            let mut result__: SmsEncoding = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SmsEncoding>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn SetEncoding(&self, value: SmsEncoding) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "deprecated")]
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation_Collections`*"]
    pub fn ToBinaryMessages(&self, format: SmsDataFormat) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<ISmsBinaryMessage>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), format, &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<ISmsBinaryMessage>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<ISmsMessage>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn MessageClass(&self) -> ::windows::runtime::Result<SmsMessageClass> {
        let this = &::windows::runtime::Interface::cast::<ISmsMessage>(self)?;
        unsafe {
            let mut result__: SmsMessageClass = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SmsMessageClass>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn FromBinaryMessage<'a, Param0: ::windows::runtime::IntoParam<'a, SmsBinaryMessage>>(binarymessage: Param0) -> ::windows::runtime::Result<SmsTextMessage> {
        Self::ISmsTextMessageStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), binarymessage.into_param().abi(), &mut result__).from_abi::<SmsTextMessage>(result__)
        })
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn FromBinaryData(format: SmsDataFormat, value: &[<u8 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<SmsTextMessage> {
        Self::ISmsTextMessageStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), format, value.len() as u32, ::std::mem::transmute(value.as_ptr()), &mut result__).from_abi::<SmsTextMessage>(result__)
        })
    }
    pub fn ISmsTextMessageStatics<R, F: FnOnce(&ISmsTextMessageStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SmsTextMessage, ISmsTextMessageStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SmsTextMessage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SmsTextMessage;{d61c904c-a495-487f-9a6f-971548c5bc9f})");
}
unsafe impl ::windows::runtime::Interface for SmsTextMessage {
    type Vtable = ISmsTextMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3592196172, 42133, 18559, [154, 111, 151, 21, 72, 197, 188, 159]);
}
impl ::windows::runtime::RuntimeName for SmsTextMessage {
    const NAME: &'static str = "Windows.Devices.Sms.SmsTextMessage";
}
impl ::std::convert::From<SmsTextMessage> for ::windows::runtime::IUnknown {
    fn from(value: SmsTextMessage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SmsTextMessage> for ::windows::runtime::IUnknown {
    fn from(value: &SmsTextMessage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SmsTextMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SmsTextMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SmsTextMessage> for ::windows::runtime::IInspectable {
    fn from(value: SmsTextMessage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SmsTextMessage> for ::windows::runtime::IInspectable {
    fn from(value: &SmsTextMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SmsTextMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SmsTextMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<SmsTextMessage> for ISmsTextMessage {
    fn from(value: SmsTextMessage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SmsTextMessage> for ISmsTextMessage {
    fn from(value: &SmsTextMessage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISmsTextMessage> for SmsTextMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISmsTextMessage> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISmsTextMessage>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISmsTextMessage> for &SmsTextMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISmsTextMessage> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISmsTextMessage>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::TryFrom<SmsTextMessage> for ISmsMessage {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SmsTextMessage) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SmsTextMessage> for ISmsMessage {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SmsTextMessage) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISmsMessage> for SmsTextMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISmsMessage> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISmsMessage> for &SmsTextMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISmsMessage> {
        ::std::convert::TryInto::<ISmsMessage>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for SmsTextMessage {}
unsafe impl ::std::marker::Sync for SmsTextMessage {}
#[doc = "*Required features: `Devices_Sms`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SmsTextMessage2(::windows::runtime::IInspectable);
impl SmsTextMessage2 {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<SmsTextMessage2, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn To(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn SetTo<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn From(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn Body(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn SetBody<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn Encoding(&self) -> ::windows::runtime::Result<SmsEncoding> {
        let this = self;
        unsafe {
            let mut result__: SmsEncoding = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SmsEncoding>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn SetEncoding(&self, value: SmsEncoding) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn CallbackNumber(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn SetCallbackNumber<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn IsDeliveryNotificationEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn SetIsDeliveryNotificationEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn RetryAttemptCount(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn SetRetryAttemptCount(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn TeleserviceId(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn ProtocolId(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn MessageType(&self) -> ::windows::runtime::Result<SmsMessageType> {
        let this = &::windows::runtime::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__: SmsMessageType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SmsMessageType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn CellularClass(&self) -> ::windows::runtime::Result<CellularClass> {
        let this = &::windows::runtime::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__: CellularClass = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CellularClass>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn MessageClass(&self) -> ::windows::runtime::Result<SmsMessageClass> {
        let this = &::windows::runtime::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__: SmsMessageClass = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SmsMessageClass>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn SimIccId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SmsTextMessage2 {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SmsTextMessage2;{22a0d893-4555-4755-b5a1-e7fd84955f8d})");
}
unsafe impl ::windows::runtime::Interface for SmsTextMessage2 {
    type Vtable = ISmsTextMessage2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(580966547, 17749, 18261, [181, 161, 231, 253, 132, 149, 95, 141]);
}
impl ::windows::runtime::RuntimeName for SmsTextMessage2 {
    const NAME: &'static str = "Windows.Devices.Sms.SmsTextMessage2";
}
impl ::std::convert::From<SmsTextMessage2> for ::windows::runtime::IUnknown {
    fn from(value: SmsTextMessage2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SmsTextMessage2> for ::windows::runtime::IUnknown {
    fn from(value: &SmsTextMessage2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SmsTextMessage2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SmsTextMessage2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SmsTextMessage2> for ::windows::runtime::IInspectable {
    fn from(value: SmsTextMessage2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SmsTextMessage2> for ::windows::runtime::IInspectable {
    fn from(value: &SmsTextMessage2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SmsTextMessage2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SmsTextMessage2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<SmsTextMessage2> for ISmsMessageBase {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SmsTextMessage2) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SmsTextMessage2> for ISmsMessageBase {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SmsTextMessage2) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISmsMessageBase> for SmsTextMessage2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISmsMessageBase> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISmsMessageBase> for &SmsTextMessage2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISmsMessageBase> {
        ::std::convert::TryInto::<ISmsMessageBase>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for SmsTextMessage2 {}
unsafe impl ::std::marker::Sync for SmsTextMessage2 {}
#[doc = "*Required features: `Devices_Sms`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SmsVoicemailMessage(::windows::runtime::IInspectable);
impl SmsVoicemailMessage {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn To(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn Body(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn MessageCount(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn MessageType(&self) -> ::windows::runtime::Result<SmsMessageType> {
        let this = &::windows::runtime::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__: SmsMessageType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SmsMessageType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn CellularClass(&self) -> ::windows::runtime::Result<CellularClass> {
        let this = &::windows::runtime::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__: CellularClass = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CellularClass>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn MessageClass(&self) -> ::windows::runtime::Result<SmsMessageClass> {
        let this = &::windows::runtime::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__: SmsMessageClass = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SmsMessageClass>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn SimIccId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SmsVoicemailMessage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SmsVoicemailMessage;{271aa0a6-95b1-44ff-bcb8-b8fdd7e08bc3})");
}
unsafe impl ::windows::runtime::Interface for SmsVoicemailMessage {
    type Vtable = ISmsVoicemailMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(656056486, 38321, 17663, [188, 184, 184, 253, 215, 224, 139, 195]);
}
impl ::windows::runtime::RuntimeName for SmsVoicemailMessage {
    const NAME: &'static str = "Windows.Devices.Sms.SmsVoicemailMessage";
}
impl ::std::convert::From<SmsVoicemailMessage> for ::windows::runtime::IUnknown {
    fn from(value: SmsVoicemailMessage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SmsVoicemailMessage> for ::windows::runtime::IUnknown {
    fn from(value: &SmsVoicemailMessage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SmsVoicemailMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SmsVoicemailMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SmsVoicemailMessage> for ::windows::runtime::IInspectable {
    fn from(value: SmsVoicemailMessage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SmsVoicemailMessage> for ::windows::runtime::IInspectable {
    fn from(value: &SmsVoicemailMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SmsVoicemailMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SmsVoicemailMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<SmsVoicemailMessage> for ISmsMessageBase {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SmsVoicemailMessage) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SmsVoicemailMessage> for ISmsMessageBase {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SmsVoicemailMessage) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISmsMessageBase> for SmsVoicemailMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISmsMessageBase> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISmsMessageBase> for &SmsVoicemailMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISmsMessageBase> {
        ::std::convert::TryInto::<ISmsMessageBase>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for SmsVoicemailMessage {}
unsafe impl ::std::marker::Sync for SmsVoicemailMessage {}
#[doc = "*Required features: `Devices_Sms`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SmsWapMessage(::windows::runtime::IInspectable);
impl SmsWapMessage {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn To(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn From(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn ApplicationId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn ContentType(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_Sms`, `Storage_Streams`*"]
    pub fn BinaryBody(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_Sms`, `Foundation_Collections`*"]
    pub fn Headers(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMap<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn MessageType(&self) -> ::windows::runtime::Result<SmsMessageType> {
        let this = &::windows::runtime::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__: SmsMessageType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SmsMessageType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn CellularClass(&self) -> ::windows::runtime::Result<CellularClass> {
        let this = &::windows::runtime::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__: CellularClass = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CellularClass>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn MessageClass(&self) -> ::windows::runtime::Result<SmsMessageClass> {
        let this = &::windows::runtime::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__: SmsMessageClass = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SmsMessageClass>(result__)
        }
    }
    #[doc = "*Required features: `Devices_Sms`*"]
    pub fn SimIccId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<ISmsMessageBase>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SmsWapMessage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.Sms.SmsWapMessage;{cd937743-7a55-4d3b-9021-f22e022d09c5})");
}
unsafe impl ::windows::runtime::Interface for SmsWapMessage {
    type Vtable = ISmsWapMessage_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3448993603, 31317, 19771, [144, 33, 242, 46, 2, 45, 9, 197]);
}
impl ::windows::runtime::RuntimeName for SmsWapMessage {
    const NAME: &'static str = "Windows.Devices.Sms.SmsWapMessage";
}
impl ::std::convert::From<SmsWapMessage> for ::windows::runtime::IUnknown {
    fn from(value: SmsWapMessage) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SmsWapMessage> for ::windows::runtime::IUnknown {
    fn from(value: &SmsWapMessage) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SmsWapMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SmsWapMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SmsWapMessage> for ::windows::runtime::IInspectable {
    fn from(value: SmsWapMessage) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SmsWapMessage> for ::windows::runtime::IInspectable {
    fn from(value: &SmsWapMessage) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SmsWapMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SmsWapMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<SmsWapMessage> for ISmsMessageBase {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SmsWapMessage) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SmsWapMessage> for ISmsMessageBase {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SmsWapMessage) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISmsMessageBase> for SmsWapMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISmsMessageBase> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISmsMessageBase> for &SmsWapMessage {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISmsMessageBase> {
        ::std::convert::TryInto::<ISmsMessageBase>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for SmsWapMessage {}
unsafe impl ::std::marker::Sync for SmsWapMessage {}

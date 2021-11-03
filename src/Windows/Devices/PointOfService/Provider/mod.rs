#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Devices_PointOfService_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct BarcodeScannerDisableScannerRequest(::windows::runtime::IInspectable);
impl BarcodeScannerDisableScannerRequest {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn ReportCompletedAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn ReportFailedAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IBarcodeScannerDisableScannerRequest2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), reason, &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn ReportFailedWithFailedReasonAndDescriptionAsync<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, reason: i32, failedreasondescription: Param1) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IBarcodeScannerDisableScannerRequest2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), reason, failedreasondescription.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BarcodeScannerDisableScannerRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerDisableScannerRequest;{88ecf7c0-37b9-4275-8e77-c8e52ae5a9c8})");
}
unsafe impl ::windows::runtime::Interface for BarcodeScannerDisableScannerRequest {
    type Vtable = IBarcodeScannerDisableScannerRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2297231296, 14265, 17013, [142, 119, 200, 229, 42, 229, 169, 200]);
}
impl ::windows::runtime::RuntimeName for BarcodeScannerDisableScannerRequest {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerDisableScannerRequest";
}
unsafe impl ::std::marker::Send for BarcodeScannerDisableScannerRequest {}
unsafe impl ::std::marker::Sync for BarcodeScannerDisableScannerRequest {}
#[doc = "*Required features: `Devices_PointOfService_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct BarcodeScannerDisableScannerRequestEventArgs(::windows::runtime::IInspectable);
impl BarcodeScannerDisableScannerRequestEventArgs {
    #[doc = "*Required features: `Devices_PointOfService_Provider`*"]
    pub fn Request(&self) -> ::windows::runtime::Result<BarcodeScannerDisableScannerRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BarcodeScannerDisableScannerRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BarcodeScannerDisableScannerRequestEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerDisableScannerRequestEventArgs;{7006e142-e802-46f5-b604-352a15ce9232})");
}
unsafe impl ::windows::runtime::Interface for BarcodeScannerDisableScannerRequestEventArgs {
    type Vtable = IBarcodeScannerDisableScannerRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1879499074, 59394, 18165, [182, 4, 53, 42, 21, 206, 146, 50]);
}
impl ::windows::runtime::RuntimeName for BarcodeScannerDisableScannerRequestEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerDisableScannerRequestEventArgs";
}
unsafe impl ::std::marker::Send for BarcodeScannerDisableScannerRequestEventArgs {}
unsafe impl ::std::marker::Sync for BarcodeScannerDisableScannerRequestEventArgs {}
#[doc = "*Required features: `Devices_PointOfService_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct BarcodeScannerEnableScannerRequest(::windows::runtime::IInspectable);
impl BarcodeScannerEnableScannerRequest {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn ReportCompletedAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn ReportFailedAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IBarcodeScannerEnableScannerRequest2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), reason, &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn ReportFailedWithFailedReasonAndDescriptionAsync<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, reason: i32, failedreasondescription: Param1) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IBarcodeScannerEnableScannerRequest2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), reason, failedreasondescription.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BarcodeScannerEnableScannerRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerEnableScannerRequest;{c0b3e9ba-816a-452b-bd77-b7e453ec446d})");
}
unsafe impl ::windows::runtime::Interface for BarcodeScannerEnableScannerRequest {
    type Vtable = IBarcodeScannerEnableScannerRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3233016250, 33130, 17707, [189, 119, 183, 228, 83, 236, 68, 109]);
}
impl ::windows::runtime::RuntimeName for BarcodeScannerEnableScannerRequest {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerEnableScannerRequest";
}
unsafe impl ::std::marker::Send for BarcodeScannerEnableScannerRequest {}
unsafe impl ::std::marker::Sync for BarcodeScannerEnableScannerRequest {}
#[doc = "*Required features: `Devices_PointOfService_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct BarcodeScannerEnableScannerRequestEventArgs(::windows::runtime::IInspectable);
impl BarcodeScannerEnableScannerRequestEventArgs {
    #[doc = "*Required features: `Devices_PointOfService_Provider`*"]
    pub fn Request(&self) -> ::windows::runtime::Result<BarcodeScannerEnableScannerRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BarcodeScannerEnableScannerRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BarcodeScannerEnableScannerRequestEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerEnableScannerRequestEventArgs;{956c9419-7b4e-4451-8c41-8e10cfbc5b41})");
}
unsafe impl ::windows::runtime::Interface for BarcodeScannerEnableScannerRequestEventArgs {
    type Vtable = IBarcodeScannerEnableScannerRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2506920985, 31566, 17489, [140, 65, 142, 16, 207, 188, 91, 65]);
}
impl ::windows::runtime::RuntimeName for BarcodeScannerEnableScannerRequestEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerEnableScannerRequestEventArgs";
}
unsafe impl ::std::marker::Send for BarcodeScannerEnableScannerRequestEventArgs {}
unsafe impl ::std::marker::Sync for BarcodeScannerEnableScannerRequestEventArgs {}
#[doc = "*Required features: `Devices_PointOfService_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct BarcodeScannerFrameReader(::windows::runtime::IInspectable);
impl BarcodeScannerFrameReader {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn StartAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn StopAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn TryAcquireLatestFrameAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<BarcodeScannerVideoFrame>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<BarcodeScannerVideoFrame>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService_Provider`*"]
    pub fn Connection(&self) -> ::windows::runtime::Result<BarcodeScannerProviderConnection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BarcodeScannerProviderConnection>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn FrameArrived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<BarcodeScannerFrameReader, BarcodeScannerFrameReaderFrameArrivedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn RemoveFrameArrived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BarcodeScannerFrameReader {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerFrameReader;{dbc72b07-64c3-482b-93c8-65fb33c22208})");
}
unsafe impl ::windows::runtime::Interface for BarcodeScannerFrameReader {
    type Vtable = IBarcodeScannerFrameReader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3687262983, 25795, 18475, [147, 200, 101, 251, 51, 194, 34, 8]);
}
impl ::windows::runtime::RuntimeName for BarcodeScannerFrameReader {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerFrameReader";
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<BarcodeScannerFrameReader> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: BarcodeScannerFrameReader) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&BarcodeScannerFrameReader> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &BarcodeScannerFrameReader) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for BarcodeScannerFrameReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for &BarcodeScannerFrameReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for BarcodeScannerFrameReader {}
unsafe impl ::std::marker::Sync for BarcodeScannerFrameReader {}
#[doc = "*Required features: `Devices_PointOfService_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct BarcodeScannerFrameReaderFrameArrivedEventArgs(::windows::runtime::IInspectable);
impl BarcodeScannerFrameReaderFrameArrivedEventArgs {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BarcodeScannerFrameReaderFrameArrivedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerFrameReaderFrameArrivedEventArgs;{b0bbd604-54fd-436d-8629-712e787223dd})");
}
unsafe impl ::windows::runtime::Interface for BarcodeScannerFrameReaderFrameArrivedEventArgs {
    type Vtable = IBarcodeScannerFrameReaderFrameArrivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2965100036, 21757, 17261, [134, 41, 113, 46, 120, 114, 35, 221]);
}
impl ::windows::runtime::RuntimeName for BarcodeScannerFrameReaderFrameArrivedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerFrameReaderFrameArrivedEventArgs";
}
unsafe impl ::std::marker::Send for BarcodeScannerFrameReaderFrameArrivedEventArgs {}
unsafe impl ::std::marker::Sync for BarcodeScannerFrameReaderFrameArrivedEventArgs {}
#[doc = "*Required features: `Devices_PointOfService_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct BarcodeScannerGetSymbologyAttributesRequest(::windows::runtime::IInspectable);
impl BarcodeScannerGetSymbologyAttributesRequest {
    #[doc = "*Required features: `Devices_PointOfService_Provider`*"]
    pub fn Symbology(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn ReportCompletedAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::BarcodeSymbologyAttributes>>(&self, attributes: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), attributes.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn ReportFailedAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IBarcodeScannerGetSymbologyAttributesRequest2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), reason, &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn ReportFailedWithFailedReasonAndDescriptionAsync<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, reason: i32, failedreasondescription: Param1) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IBarcodeScannerGetSymbologyAttributesRequest2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), reason, failedreasondescription.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BarcodeScannerGetSymbologyAttributesRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerGetSymbologyAttributesRequest;{9774c46a-58e4-4c5f-b8e9-e41467632700})");
}
unsafe impl ::windows::runtime::Interface for BarcodeScannerGetSymbologyAttributesRequest {
    type Vtable = IBarcodeScannerGetSymbologyAttributesRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2541012074, 22756, 19551, [184, 233, 228, 20, 103, 99, 39, 0]);
}
impl ::windows::runtime::RuntimeName for BarcodeScannerGetSymbologyAttributesRequest {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerGetSymbologyAttributesRequest";
}
unsafe impl ::std::marker::Send for BarcodeScannerGetSymbologyAttributesRequest {}
unsafe impl ::std::marker::Sync for BarcodeScannerGetSymbologyAttributesRequest {}
#[doc = "*Required features: `Devices_PointOfService_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct BarcodeScannerGetSymbologyAttributesRequestEventArgs(::windows::runtime::IInspectable);
impl BarcodeScannerGetSymbologyAttributesRequestEventArgs {
    #[doc = "*Required features: `Devices_PointOfService_Provider`*"]
    pub fn Request(&self) -> ::windows::runtime::Result<BarcodeScannerGetSymbologyAttributesRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BarcodeScannerGetSymbologyAttributesRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BarcodeScannerGetSymbologyAttributesRequestEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerGetSymbologyAttributesRequestEventArgs;{7f89de3e-fb5d-493c-b402-356b24d574a6})");
}
unsafe impl ::windows::runtime::Interface for BarcodeScannerGetSymbologyAttributesRequestEventArgs {
    type Vtable = IBarcodeScannerGetSymbologyAttributesRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2139741758, 64349, 18748, [180, 2, 53, 107, 36, 213, 116, 166]);
}
impl ::windows::runtime::RuntimeName for BarcodeScannerGetSymbologyAttributesRequestEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerGetSymbologyAttributesRequestEventArgs";
}
unsafe impl ::std::marker::Send for BarcodeScannerGetSymbologyAttributesRequestEventArgs {}
unsafe impl ::std::marker::Sync for BarcodeScannerGetSymbologyAttributesRequestEventArgs {}
#[doc = "*Required features: `Devices_PointOfService_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct BarcodeScannerHideVideoPreviewRequest(::windows::runtime::IInspectable);
impl BarcodeScannerHideVideoPreviewRequest {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn ReportCompletedAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn ReportFailedAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IBarcodeScannerHideVideoPreviewRequest2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), reason, &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn ReportFailedWithFailedReasonAndDescriptionAsync<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, reason: i32, failedreasondescription: Param1) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IBarcodeScannerHideVideoPreviewRequest2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), reason, failedreasondescription.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BarcodeScannerHideVideoPreviewRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerHideVideoPreviewRequest;{fa4ebe7f-6670-40e1-b90b-bb10d8d425fa})");
}
unsafe impl ::windows::runtime::Interface for BarcodeScannerHideVideoPreviewRequest {
    type Vtable = IBarcodeScannerHideVideoPreviewRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4199464575, 26224, 16609, [185, 11, 187, 16, 216, 212, 37, 250]);
}
impl ::windows::runtime::RuntimeName for BarcodeScannerHideVideoPreviewRequest {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerHideVideoPreviewRequest";
}
unsafe impl ::std::marker::Send for BarcodeScannerHideVideoPreviewRequest {}
unsafe impl ::std::marker::Sync for BarcodeScannerHideVideoPreviewRequest {}
#[doc = "*Required features: `Devices_PointOfService_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct BarcodeScannerHideVideoPreviewRequestEventArgs(::windows::runtime::IInspectable);
impl BarcodeScannerHideVideoPreviewRequestEventArgs {
    #[doc = "*Required features: `Devices_PointOfService_Provider`*"]
    pub fn Request(&self) -> ::windows::runtime::Result<BarcodeScannerHideVideoPreviewRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BarcodeScannerHideVideoPreviewRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BarcodeScannerHideVideoPreviewRequestEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerHideVideoPreviewRequestEventArgs;{16a281fc-d6be-4bc7-9df1-33741f3eadea})");
}
unsafe impl ::windows::runtime::Interface for BarcodeScannerHideVideoPreviewRequestEventArgs {
    type Vtable = IBarcodeScannerHideVideoPreviewRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(379748860, 54974, 19399, [157, 241, 51, 116, 31, 62, 173, 234]);
}
impl ::windows::runtime::RuntimeName for BarcodeScannerHideVideoPreviewRequestEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerHideVideoPreviewRequestEventArgs";
}
unsafe impl ::std::marker::Send for BarcodeScannerHideVideoPreviewRequestEventArgs {}
unsafe impl ::std::marker::Sync for BarcodeScannerHideVideoPreviewRequestEventArgs {}
#[doc = "*Required features: `Devices_PointOfService_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct BarcodeScannerProviderConnection(::windows::runtime::IInspectable);
impl BarcodeScannerProviderConnection {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService_Provider`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService_Provider`*"]
    pub fn VideoDeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation_Collections`*"]
    pub fn SupportedSymbologies(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVector<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVector<u32>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService_Provider`*"]
    pub fn CompanyName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService_Provider`*"]
    pub fn SetCompanyName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService_Provider`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService_Provider`*"]
    pub fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService_Provider`*"]
    pub fn Version(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService_Provider`*"]
    pub fn SetVersion<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService_Provider`*"]
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn ReportScannedDataAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::BarcodeScannerReport>>(&self, report: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), report.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn ReportTriggerStateAsync(&self, state: BarcodeScannerTriggerState) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), state, &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn ReportErrorAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::UnifiedPosErrorData>>(&self, errordata: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), errordata.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn ReportErrorAsyncWithScanReport<'a, Param0: ::windows::runtime::IntoParam<'a, super::UnifiedPosErrorData>, Param2: ::windows::runtime::IntoParam<'a, super::BarcodeScannerReport>>(&self, errordata: Param0, isretriable: bool, scanreport: Param2) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), errordata.into_param().abi(), isretriable, scanreport.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn EnableScannerRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerEnableScannerRequestEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn RemoveEnableScannerRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn DisableScannerRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerDisableScannerRequestEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn RemoveDisableScannerRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn SetActiveSymbologiesRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerSetActiveSymbologiesRequestEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn RemoveSetActiveSymbologiesRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn StartSoftwareTriggerRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerStartSoftwareTriggerRequestEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn RemoveStartSoftwareTriggerRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn StopSoftwareTriggerRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerStopSoftwareTriggerRequestEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn RemoveStopSoftwareTriggerRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn GetBarcodeSymbologyAttributesRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerGetSymbologyAttributesRequestEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn RemoveGetBarcodeSymbologyAttributesRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).31)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn SetBarcodeSymbologyAttributesRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerSetSymbologyAttributesRequestEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).32)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn RemoveSetBarcodeSymbologyAttributesRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).33)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn HideVideoPreviewRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerHideVideoPreviewRequestEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).34)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn RemoveHideVideoPreviewRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).35)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn CreateFrameReaderAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<BarcodeScannerFrameReader>> {
        let this = &::windows::runtime::Interface::cast::<IBarcodeScannerProviderConnection2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<BarcodeScannerFrameReader>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`, `Graphics_Imaging`*"]
    pub fn CreateFrameReaderWithFormatAsync(&self, preferredformat: super::super::super::Graphics::Imaging::BitmapPixelFormat) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<BarcodeScannerFrameReader>> {
        let this = &::windows::runtime::Interface::cast::<IBarcodeScannerProviderConnection2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), preferredformat, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<BarcodeScannerFrameReader>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`, `Graphics_Imaging`*"]
    pub fn CreateFrameReaderWithFormatAndSizeAsync<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Imaging::BitmapSize>>(&self, preferredformat: super::super::super::Graphics::Imaging::BitmapPixelFormat, preferredsize: Param1) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<BarcodeScannerFrameReader>> {
        let this = &::windows::runtime::Interface::cast::<IBarcodeScannerProviderConnection2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), preferredformat, preferredsize.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<BarcodeScannerFrameReader>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BarcodeScannerProviderConnection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerProviderConnection;{b44acbed-0b3a-4fa3-86c5-491ea30780eb})");
}
unsafe impl ::windows::runtime::Interface for BarcodeScannerProviderConnection {
    type Vtable = IBarcodeScannerProviderConnection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3024800749, 2874, 20387, [134, 197, 73, 30, 163, 7, 128, 235]);
}
impl ::windows::runtime::RuntimeName for BarcodeScannerProviderConnection {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerProviderConnection";
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<BarcodeScannerProviderConnection> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: BarcodeScannerProviderConnection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&BarcodeScannerProviderConnection> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &BarcodeScannerProviderConnection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for BarcodeScannerProviderConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for &BarcodeScannerProviderConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for BarcodeScannerProviderConnection {}
unsafe impl ::std::marker::Sync for BarcodeScannerProviderConnection {}
#[doc = "*Required features: `Devices_PointOfService_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct BarcodeScannerProviderTriggerDetails(::windows::runtime::IInspectable);
impl BarcodeScannerProviderTriggerDetails {
    #[doc = "*Required features: `Devices_PointOfService_Provider`*"]
    pub fn Connection(&self) -> ::windows::runtime::Result<BarcodeScannerProviderConnection> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BarcodeScannerProviderConnection>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BarcodeScannerProviderTriggerDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerProviderTriggerDetails;{50856d82-24e3-48ce-99c7-70aac1cbc9f7})");
}
unsafe impl ::windows::runtime::Interface for BarcodeScannerProviderTriggerDetails {
    type Vtable = IBarcodeScannerProviderTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1350921602, 9443, 18638, [153, 199, 112, 170, 193, 203, 201, 247]);
}
impl ::windows::runtime::RuntimeName for BarcodeScannerProviderTriggerDetails {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerProviderTriggerDetails";
}
unsafe impl ::std::marker::Send for BarcodeScannerProviderTriggerDetails {}
unsafe impl ::std::marker::Sync for BarcodeScannerProviderTriggerDetails {}
#[doc = "*Required features: `Devices_PointOfService_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct BarcodeScannerSetActiveSymbologiesRequest(::windows::runtime::IInspectable);
impl BarcodeScannerSetActiveSymbologiesRequest {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation_Collections`*"]
    pub fn Symbologies(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn ReportCompletedAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn ReportFailedAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IBarcodeScannerSetActiveSymbologiesRequest2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), reason, &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn ReportFailedWithFailedReasonAndDescriptionAsync<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, reason: i32, failedreasondescription: Param1) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IBarcodeScannerSetActiveSymbologiesRequest2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), reason, failedreasondescription.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BarcodeScannerSetActiveSymbologiesRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerSetActiveSymbologiesRequest;{db3f32b9-f7da-41a1-9f79-07bcd95f0bdf})");
}
unsafe impl ::windows::runtime::Interface for BarcodeScannerSetActiveSymbologiesRequest {
    type Vtable = IBarcodeScannerSetActiveSymbologiesRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3678352057, 63450, 16801, [159, 121, 7, 188, 217, 95, 11, 223]);
}
impl ::windows::runtime::RuntimeName for BarcodeScannerSetActiveSymbologiesRequest {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerSetActiveSymbologiesRequest";
}
unsafe impl ::std::marker::Send for BarcodeScannerSetActiveSymbologiesRequest {}
unsafe impl ::std::marker::Sync for BarcodeScannerSetActiveSymbologiesRequest {}
#[doc = "*Required features: `Devices_PointOfService_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct BarcodeScannerSetActiveSymbologiesRequestEventArgs(::windows::runtime::IInspectable);
impl BarcodeScannerSetActiveSymbologiesRequestEventArgs {
    #[doc = "*Required features: `Devices_PointOfService_Provider`*"]
    pub fn Request(&self) -> ::windows::runtime::Result<BarcodeScannerSetActiveSymbologiesRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BarcodeScannerSetActiveSymbologiesRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BarcodeScannerSetActiveSymbologiesRequestEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerSetActiveSymbologiesRequestEventArgs;{06305afa-7bf6-4d52-801a-330272f60ae1})");
}
unsafe impl ::windows::runtime::Interface for BarcodeScannerSetActiveSymbologiesRequestEventArgs {
    type Vtable = IBarcodeScannerSetActiveSymbologiesRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(103832314, 31734, 19794, [128, 26, 51, 2, 114, 246, 10, 225]);
}
impl ::windows::runtime::RuntimeName for BarcodeScannerSetActiveSymbologiesRequestEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerSetActiveSymbologiesRequestEventArgs";
}
unsafe impl ::std::marker::Send for BarcodeScannerSetActiveSymbologiesRequestEventArgs {}
unsafe impl ::std::marker::Sync for BarcodeScannerSetActiveSymbologiesRequestEventArgs {}
#[doc = "*Required features: `Devices_PointOfService_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct BarcodeScannerSetSymbologyAttributesRequest(::windows::runtime::IInspectable);
impl BarcodeScannerSetSymbologyAttributesRequest {
    #[doc = "*Required features: `Devices_PointOfService_Provider`*"]
    pub fn Symbology(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService_Provider`*"]
    pub fn Attributes(&self) -> ::windows::runtime::Result<super::BarcodeSymbologyAttributes> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::BarcodeSymbologyAttributes>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn ReportCompletedAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn ReportFailedAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IBarcodeScannerSetSymbologyAttributesRequest2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), reason, &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn ReportFailedWithFailedReasonAndDescriptionAsync<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, reason: i32, failedreasondescription: Param1) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IBarcodeScannerSetSymbologyAttributesRequest2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), reason, failedreasondescription.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BarcodeScannerSetSymbologyAttributesRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerSetSymbologyAttributesRequest;{32fb814f-a37f-48b0-acea-dce1480f12ae})");
}
unsafe impl ::windows::runtime::Interface for BarcodeScannerSetSymbologyAttributesRequest {
    type Vtable = IBarcodeScannerSetSymbologyAttributesRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(855343439, 41855, 18608, [172, 234, 220, 225, 72, 15, 18, 174]);
}
impl ::windows::runtime::RuntimeName for BarcodeScannerSetSymbologyAttributesRequest {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerSetSymbologyAttributesRequest";
}
unsafe impl ::std::marker::Send for BarcodeScannerSetSymbologyAttributesRequest {}
unsafe impl ::std::marker::Sync for BarcodeScannerSetSymbologyAttributesRequest {}
#[doc = "*Required features: `Devices_PointOfService_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct BarcodeScannerSetSymbologyAttributesRequestEventArgs(::windows::runtime::IInspectable);
impl BarcodeScannerSetSymbologyAttributesRequestEventArgs {
    #[doc = "*Required features: `Devices_PointOfService_Provider`*"]
    pub fn Request(&self) -> ::windows::runtime::Result<BarcodeScannerSetSymbologyAttributesRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BarcodeScannerSetSymbologyAttributesRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BarcodeScannerSetSymbologyAttributesRequestEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerSetSymbologyAttributesRequestEventArgs;{b2b89809-9824-47d4-85bd-d0077baa7bd2})");
}
unsafe impl ::windows::runtime::Interface for BarcodeScannerSetSymbologyAttributesRequestEventArgs {
    type Vtable = IBarcodeScannerSetSymbologyAttributesRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2998441993, 38948, 18388, [133, 189, 208, 7, 123, 170, 123, 210]);
}
impl ::windows::runtime::RuntimeName for BarcodeScannerSetSymbologyAttributesRequestEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerSetSymbologyAttributesRequestEventArgs";
}
unsafe impl ::std::marker::Send for BarcodeScannerSetSymbologyAttributesRequestEventArgs {}
unsafe impl ::std::marker::Sync for BarcodeScannerSetSymbologyAttributesRequestEventArgs {}
#[doc = "*Required features: `Devices_PointOfService_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct BarcodeScannerStartSoftwareTriggerRequest(::windows::runtime::IInspectable);
impl BarcodeScannerStartSoftwareTriggerRequest {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn ReportCompletedAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn ReportFailedAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IBarcodeScannerStartSoftwareTriggerRequest2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), reason, &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn ReportFailedWithFailedReasonAndDescriptionAsync<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, reason: i32, failedreasondescription: Param1) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IBarcodeScannerStartSoftwareTriggerRequest2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), reason, failedreasondescription.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BarcodeScannerStartSoftwareTriggerRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerStartSoftwareTriggerRequest;{e3fa7b27-ff62-4454-af4a-cb6144a3e3f7})");
}
unsafe impl ::windows::runtime::Interface for BarcodeScannerStartSoftwareTriggerRequest {
    type Vtable = IBarcodeScannerStartSoftwareTriggerRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3824843559, 65378, 17492, [175, 74, 203, 97, 68, 163, 227, 247]);
}
impl ::windows::runtime::RuntimeName for BarcodeScannerStartSoftwareTriggerRequest {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerStartSoftwareTriggerRequest";
}
unsafe impl ::std::marker::Send for BarcodeScannerStartSoftwareTriggerRequest {}
unsafe impl ::std::marker::Sync for BarcodeScannerStartSoftwareTriggerRequest {}
#[doc = "*Required features: `Devices_PointOfService_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct BarcodeScannerStartSoftwareTriggerRequestEventArgs(::windows::runtime::IInspectable);
impl BarcodeScannerStartSoftwareTriggerRequestEventArgs {
    #[doc = "*Required features: `Devices_PointOfService_Provider`*"]
    pub fn Request(&self) -> ::windows::runtime::Result<BarcodeScannerStartSoftwareTriggerRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BarcodeScannerStartSoftwareTriggerRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BarcodeScannerStartSoftwareTriggerRequestEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerStartSoftwareTriggerRequestEventArgs;{2305d843-c88f-4f3b-8c3b-d3df071051ec})");
}
unsafe impl ::windows::runtime::Interface for BarcodeScannerStartSoftwareTriggerRequestEventArgs {
    type Vtable = IBarcodeScannerStartSoftwareTriggerRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(587585603, 51343, 20283, [140, 59, 211, 223, 7, 16, 81, 236]);
}
impl ::windows::runtime::RuntimeName for BarcodeScannerStartSoftwareTriggerRequestEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerStartSoftwareTriggerRequestEventArgs";
}
unsafe impl ::std::marker::Send for BarcodeScannerStartSoftwareTriggerRequestEventArgs {}
unsafe impl ::std::marker::Sync for BarcodeScannerStartSoftwareTriggerRequestEventArgs {}
#[doc = "*Required features: `Devices_PointOfService_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct BarcodeScannerStopSoftwareTriggerRequest(::windows::runtime::IInspectable);
impl BarcodeScannerStopSoftwareTriggerRequest {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn ReportCompletedAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn ReportFailedAsync(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IBarcodeScannerStopSoftwareTriggerRequest2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), reason, &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn ReportFailedWithFailedReasonAndDescriptionAsync<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, reason: i32, failedreasondescription: Param1) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IBarcodeScannerStopSoftwareTriggerRequest2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), reason, failedreasondescription.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BarcodeScannerStopSoftwareTriggerRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerStopSoftwareTriggerRequest;{6f9faf35-e287-4ca8-b70d-5a91d694f668})");
}
unsafe impl ::windows::runtime::Interface for BarcodeScannerStopSoftwareTriggerRequest {
    type Vtable = IBarcodeScannerStopSoftwareTriggerRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1872736053, 57991, 19624, [183, 13, 90, 145, 214, 148, 246, 104]);
}
impl ::windows::runtime::RuntimeName for BarcodeScannerStopSoftwareTriggerRequest {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerStopSoftwareTriggerRequest";
}
unsafe impl ::std::marker::Send for BarcodeScannerStopSoftwareTriggerRequest {}
unsafe impl ::std::marker::Sync for BarcodeScannerStopSoftwareTriggerRequest {}
#[doc = "*Required features: `Devices_PointOfService_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct BarcodeScannerStopSoftwareTriggerRequestEventArgs(::windows::runtime::IInspectable);
impl BarcodeScannerStopSoftwareTriggerRequestEventArgs {
    #[doc = "*Required features: `Devices_PointOfService_Provider`*"]
    pub fn Request(&self) -> ::windows::runtime::Result<BarcodeScannerStopSoftwareTriggerRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BarcodeScannerStopSoftwareTriggerRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BarcodeScannerStopSoftwareTriggerRequestEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerStopSoftwareTriggerRequestEventArgs;{eac34450-4eb7-481a-9273-147a273b99b8})");
}
unsafe impl ::windows::runtime::Interface for BarcodeScannerStopSoftwareTriggerRequestEventArgs {
    type Vtable = IBarcodeScannerStopSoftwareTriggerRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3938665552, 20151, 18458, [146, 115, 20, 122, 39, 59, 153, 184]);
}
impl ::windows::runtime::RuntimeName for BarcodeScannerStopSoftwareTriggerRequestEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerStopSoftwareTriggerRequestEventArgs";
}
unsafe impl ::std::marker::Send for BarcodeScannerStopSoftwareTriggerRequestEventArgs {}
unsafe impl ::std::marker::Sync for BarcodeScannerStopSoftwareTriggerRequestEventArgs {}
#[doc = "*Required features: `Devices_PointOfService_Provider`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct BarcodeScannerTriggerState(pub i32);
impl BarcodeScannerTriggerState {
    pub const Released: BarcodeScannerTriggerState = BarcodeScannerTriggerState(0i32);
    pub const Pressed: BarcodeScannerTriggerState = BarcodeScannerTriggerState(1i32);
}
impl ::std::convert::From<i32> for BarcodeScannerTriggerState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for BarcodeScannerTriggerState {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for BarcodeScannerTriggerState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.Provider.BarcodeScannerTriggerState;i4)");
}
impl ::windows::runtime::DefaultType for BarcodeScannerTriggerState {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_PointOfService_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct BarcodeScannerVideoFrame(::windows::runtime::IInspectable);
impl BarcodeScannerVideoFrame {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Graphics_Imaging`*"]
    pub fn Format(&self) -> ::windows::runtime::Result<super::super::super::Graphics::Imaging::BitmapPixelFormat> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Graphics::Imaging::BitmapPixelFormat = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Graphics::Imaging::BitmapPixelFormat>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService_Provider`*"]
    pub fn Width(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService_Provider`*"]
    pub fn Height(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_PointOfService_Provider`, `Storage_Streams`*"]
    pub fn PixelData(&self) -> ::windows::runtime::Result<super::super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Storage::Streams::IBuffer>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BarcodeScannerVideoFrame {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeScannerVideoFrame;{7e585248-9df7-4121-a175-801d8000112e})");
}
unsafe impl ::windows::runtime::Interface for BarcodeScannerVideoFrame {
    type Vtable = IBarcodeScannerVideoFrame_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2119717448, 40439, 16673, [161, 117, 128, 29, 128, 0, 17, 46]);
}
impl ::windows::runtime::RuntimeName for BarcodeScannerVideoFrame {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeScannerVideoFrame";
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<BarcodeScannerVideoFrame> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: BarcodeScannerVideoFrame) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&BarcodeScannerVideoFrame> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &BarcodeScannerVideoFrame) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for BarcodeScannerVideoFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for &BarcodeScannerVideoFrame {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for BarcodeScannerVideoFrame {}
unsafe impl ::std::marker::Sync for BarcodeScannerVideoFrame {}
#[doc = "*Required features: `Devices_PointOfService_Provider`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct BarcodeSymbologyAttributesBuilder(::windows::runtime::IInspectable);
impl BarcodeSymbologyAttributesBuilder {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BarcodeSymbologyAttributesBuilder, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Devices_PointOfService_Provider`*"]
    pub fn IsCheckDigitValidationSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService_Provider`*"]
    pub fn SetIsCheckDigitValidationSupported(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService_Provider`*"]
    pub fn IsCheckDigitTransmissionSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService_Provider`*"]
    pub fn SetIsCheckDigitTransmissionSupported(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService_Provider`*"]
    pub fn IsDecodeLengthSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService_Provider`*"]
    pub fn SetIsDecodeLengthSupported(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService_Provider`*"]
    pub fn CreateAttributes(&self) -> ::windows::runtime::Result<super::BarcodeSymbologyAttributes> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::BarcodeSymbologyAttributes>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BarcodeSymbologyAttributesBuilder {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.Provider.BarcodeSymbologyAttributesBuilder;{c57b0cbf-e4f5-40b9-84cf-e63fbaea42b4})");
}
unsafe impl ::windows::runtime::Interface for BarcodeSymbologyAttributesBuilder {
    type Vtable = IBarcodeSymbologyAttributesBuilder_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3313175743, 58613, 16569, [132, 207, 230, 63, 186, 234, 66, 180]);
}
impl ::windows::runtime::RuntimeName for BarcodeSymbologyAttributesBuilder {
    const NAME: &'static str = "Windows.Devices.PointOfService.Provider.BarcodeSymbologyAttributesBuilder";
}
unsafe impl ::std::marker::Send for BarcodeSymbologyAttributesBuilder {}
unsafe impl ::std::marker::Sync for BarcodeSymbologyAttributesBuilder {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IBarcodeScannerDisableScannerRequest(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBarcodeScannerDisableScannerRequest {
    type Vtable = IBarcodeScannerDisableScannerRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2297231296, 14265, 17013, [142, 119, 200, 229, 42, 229, 169, 200]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerDisableScannerRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBarcodeScannerDisableScannerRequest2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBarcodeScannerDisableScannerRequest2 {
    type Vtable = IBarcodeScannerDisableScannerRequest2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3437225509, 26051, 19660, [180, 87, 243, 156, 122, 158, 166, 13]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerDisableScannerRequest2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reason: i32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reason: i32, failedreasondescription: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBarcodeScannerDisableScannerRequestEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBarcodeScannerDisableScannerRequestEventArgs {
    type Vtable = IBarcodeScannerDisableScannerRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1879499074, 59394, 18165, [182, 4, 53, 42, 21, 206, 146, 50]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerDisableScannerRequestEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBarcodeScannerEnableScannerRequest(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBarcodeScannerEnableScannerRequest {
    type Vtable = IBarcodeScannerEnableScannerRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3233016250, 33130, 17707, [189, 119, 183, 228, 83, 236, 68, 109]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerEnableScannerRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBarcodeScannerEnableScannerRequest2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBarcodeScannerEnableScannerRequest2 {
    type Vtable = IBarcodeScannerEnableScannerRequest2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1906635432, 39173, 16812, [145, 33, 182, 69, 145, 106, 132, 161]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerEnableScannerRequest2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reason: i32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reason: i32, failedreasondescription: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBarcodeScannerEnableScannerRequestEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBarcodeScannerEnableScannerRequestEventArgs {
    type Vtable = IBarcodeScannerEnableScannerRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2506920985, 31566, 17489, [140, 65, 142, 16, 207, 188, 91, 65]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerEnableScannerRequestEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBarcodeScannerFrameReader(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBarcodeScannerFrameReader {
    type Vtable = IBarcodeScannerFrameReader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3687262983, 25795, 18475, [147, 200, 101, 251, 51, 194, 34, 8]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerFrameReader_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBarcodeScannerFrameReaderFrameArrivedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBarcodeScannerFrameReaderFrameArrivedEventArgs {
    type Vtable = IBarcodeScannerFrameReaderFrameArrivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2965100036, 21757, 17261, [134, 41, 113, 46, 120, 114, 35, 221]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerFrameReaderFrameArrivedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBarcodeScannerGetSymbologyAttributesRequest(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBarcodeScannerGetSymbologyAttributesRequest {
    type Vtable = IBarcodeScannerGetSymbologyAttributesRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2541012074, 22756, 19551, [184, 233, 228, 20, 103, 99, 39, 0]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerGetSymbologyAttributesRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, attributes: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBarcodeScannerGetSymbologyAttributesRequest2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBarcodeScannerGetSymbologyAttributesRequest2 {
    type Vtable = IBarcodeScannerGetSymbologyAttributesRequest2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1785342739, 30120, 18939, [184, 82, 191, 185, 61, 118, 10, 247]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerGetSymbologyAttributesRequest2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reason: i32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reason: i32, failedreasondescription: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBarcodeScannerGetSymbologyAttributesRequestEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBarcodeScannerGetSymbologyAttributesRequestEventArgs {
    type Vtable = IBarcodeScannerGetSymbologyAttributesRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2139741758, 64349, 18748, [180, 2, 53, 107, 36, 213, 116, 166]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerGetSymbologyAttributesRequestEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBarcodeScannerHideVideoPreviewRequest(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBarcodeScannerHideVideoPreviewRequest {
    type Vtable = IBarcodeScannerHideVideoPreviewRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4199464575, 26224, 16609, [185, 11, 187, 16, 216, 212, 37, 250]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerHideVideoPreviewRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBarcodeScannerHideVideoPreviewRequest2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBarcodeScannerHideVideoPreviewRequest2 {
    type Vtable = IBarcodeScannerHideVideoPreviewRequest2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2116567901, 38932, 17181, [162, 242, 214, 36, 140, 90, 212, 181]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerHideVideoPreviewRequest2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reason: i32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reason: i32, failedreasondescription: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBarcodeScannerHideVideoPreviewRequestEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBarcodeScannerHideVideoPreviewRequestEventArgs {
    type Vtable = IBarcodeScannerHideVideoPreviewRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(379748860, 54974, 19399, [157, 241, 51, 116, 31, 62, 173, 234]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerHideVideoPreviewRequestEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBarcodeScannerProviderConnection(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBarcodeScannerProviderConnection {
    type Vtable = IBarcodeScannerProviderConnection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3024800749, 2874, 20387, [134, 197, 73, 30, 163, 7, 128, 235]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerProviderConnection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, report: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, state: BarcodeScannerTriggerState, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, errordata: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, errordata: ::windows::runtime::RawPtr, isretriable: bool, scanreport: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBarcodeScannerProviderConnection2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBarcodeScannerProviderConnection2 {
    type Vtable = IBarcodeScannerProviderConnection2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3197850573, 4404, 16780, [160, 107, 4, 66, 58, 115, 243, 215]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerProviderConnection2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, preferredformat: super::super::super::Graphics::Imaging::BitmapPixelFormat, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Imaging")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, preferredformat: super::super::super::Graphics::Imaging::BitmapPixelFormat, preferredsize: super::super::super::Graphics::Imaging::BitmapSize, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Imaging")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBarcodeScannerProviderTriggerDetails(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBarcodeScannerProviderTriggerDetails {
    type Vtable = IBarcodeScannerProviderTriggerDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1350921602, 9443, 18638, [153, 199, 112, 170, 193, 203, 201, 247]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerProviderTriggerDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBarcodeScannerSetActiveSymbologiesRequest(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBarcodeScannerSetActiveSymbologiesRequest {
    type Vtable = IBarcodeScannerSetActiveSymbologiesRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3678352057, 63450, 16801, [159, 121, 7, 188, 217, 95, 11, 223]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerSetActiveSymbologiesRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBarcodeScannerSetActiveSymbologiesRequest2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBarcodeScannerSetActiveSymbologiesRequest2 {
    type Vtable = IBarcodeScannerSetActiveSymbologiesRequest2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4127157983, 64154, 18249, [177, 27, 232, 252, 203, 117, 188, 107]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerSetActiveSymbologiesRequest2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reason: i32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reason: i32, failedreasondescription: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBarcodeScannerSetActiveSymbologiesRequestEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBarcodeScannerSetActiveSymbologiesRequestEventArgs {
    type Vtable = IBarcodeScannerSetActiveSymbologiesRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(103832314, 31734, 19794, [128, 26, 51, 2, 114, 246, 10, 225]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerSetActiveSymbologiesRequestEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBarcodeScannerSetSymbologyAttributesRequest(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBarcodeScannerSetSymbologyAttributesRequest {
    type Vtable = IBarcodeScannerSetSymbologyAttributesRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(855343439, 41855, 18608, [172, 234, 220, 225, 72, 15, 18, 174]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerSetSymbologyAttributesRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBarcodeScannerSetSymbologyAttributesRequest2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBarcodeScannerSetSymbologyAttributesRequest2 {
    type Vtable = IBarcodeScannerSetSymbologyAttributesRequest2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3757817793, 56232, 19319, [190, 30, 181, 108, 215, 47, 101, 179]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerSetSymbologyAttributesRequest2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reason: i32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reason: i32, failedreasondescription: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBarcodeScannerSetSymbologyAttributesRequestEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBarcodeScannerSetSymbologyAttributesRequestEventArgs {
    type Vtable = IBarcodeScannerSetSymbologyAttributesRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2998441993, 38948, 18388, [133, 189, 208, 7, 123, 170, 123, 210]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerSetSymbologyAttributesRequestEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBarcodeScannerStartSoftwareTriggerRequest(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBarcodeScannerStartSoftwareTriggerRequest {
    type Vtable = IBarcodeScannerStartSoftwareTriggerRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3824843559, 65378, 17492, [175, 74, 203, 97, 68, 163, 227, 247]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerStartSoftwareTriggerRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBarcodeScannerStartSoftwareTriggerRequest2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBarcodeScannerStartSoftwareTriggerRequest2 {
    type Vtable = IBarcodeScannerStartSoftwareTriggerRequest2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3950158428, 26200, 18277, [166, 142, 50, 116, 130, 101, 61, 235]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerStartSoftwareTriggerRequest2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reason: i32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reason: i32, failedreasondescription: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBarcodeScannerStartSoftwareTriggerRequestEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBarcodeScannerStartSoftwareTriggerRequestEventArgs {
    type Vtable = IBarcodeScannerStartSoftwareTriggerRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(587585603, 51343, 20283, [140, 59, 211, 223, 7, 16, 81, 236]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerStartSoftwareTriggerRequestEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBarcodeScannerStopSoftwareTriggerRequest(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBarcodeScannerStopSoftwareTriggerRequest {
    type Vtable = IBarcodeScannerStopSoftwareTriggerRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1872736053, 57991, 19624, [183, 13, 90, 145, 214, 148, 246, 104]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerStopSoftwareTriggerRequest_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBarcodeScannerStopSoftwareTriggerRequest2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBarcodeScannerStopSoftwareTriggerRequest2 {
    type Vtable = IBarcodeScannerStopSoftwareTriggerRequest2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3411527133, 65104, 18936, [160, 180, 189, 194, 48, 129, 77, 162]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerStopSoftwareTriggerRequest2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reason: i32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reason: i32, failedreasondescription: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBarcodeScannerStopSoftwareTriggerRequestEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBarcodeScannerStopSoftwareTriggerRequestEventArgs {
    type Vtable = IBarcodeScannerStopSoftwareTriggerRequestEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3938665552, 20151, 18458, [146, 115, 20, 122, 39, 59, 153, 184]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerStopSoftwareTriggerRequestEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBarcodeScannerVideoFrame(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBarcodeScannerVideoFrame {
    type Vtable = IBarcodeScannerVideoFrame_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2119717448, 40439, 16673, [161, 117, 128, 29, 128, 0, 17, 46]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerVideoFrame_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Graphics::Imaging::BitmapPixelFormat) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBarcodeSymbologyAttributesBuilder(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBarcodeSymbologyAttributesBuilder {
    type Vtable = IBarcodeSymbologyAttributesBuilder_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3313175743, 58613, 16569, [132, 207, 230, 63, 186, 234, 66, 180]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeSymbologyAttributesBuilder_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);

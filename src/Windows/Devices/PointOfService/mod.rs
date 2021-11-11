#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Devices_PointOfService_Provider")]
pub mod Provider;
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BarcodeScanner(pub ::windows::core::IInspectable);
impl BarcodeScanner {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Capabilities(&self) -> ::windows::core::Result<BarcodeScannerCapabilities> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BarcodeScannerCapabilities>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn ClaimScannerAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ClaimedBarcodeScanner>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ClaimedBarcodeScanner>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn CheckHealthAsync(&self, level: UnifiedPosHealthCheckLevel) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), level, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetSupportedSymbologiesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<u32>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<u32>>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn IsSymbologySupportedAsync(&self, barcodesymbology: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), barcodesymbology, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`, `Foundation_Collections`, `Storage_Streams`*"]
    pub fn RetrieveStatisticsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, statisticscategories: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), statisticscategories.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation_Collections`*"]
    pub fn GetSupportedProfiles(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsProfileSupported<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, profile: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), profile.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn StatusUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<BarcodeScanner, BarcodeScannerStatusUpdatedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveStatusUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn GetDefaultAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BarcodeScanner>> {
        Self::IBarcodeScannerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BarcodeScanner>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BarcodeScanner>> {
        Self::IBarcodeScannerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BarcodeScanner>>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBarcodeScannerStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn VideoDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IBarcodeScanner2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn GetDeviceSelectorWithConnectionTypes(connectiontypes: PosConnectionTypes) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBarcodeScannerStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), connectiontypes, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn IBarcodeScannerStatics<R, F: FnOnce(&IBarcodeScannerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BarcodeScanner, IBarcodeScannerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IBarcodeScannerStatics2<R, F: FnOnce(&IBarcodeScannerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BarcodeScanner, IBarcodeScannerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for BarcodeScanner {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.BarcodeScanner;{bea33e06-b264-4f03-a9c1-45b20f01134f})");
}
unsafe impl ::windows::core::Interface for BarcodeScanner {
    type Vtable = IBarcodeScanner_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbea33e06_b264_4f03_a9c1_45b20f01134f);
}
impl ::windows::core::RuntimeName for BarcodeScanner {
    const NAME: &'static str = "Windows.Devices.PointOfService.BarcodeScanner";
}
impl ::core::convert::From<BarcodeScanner> for ::windows::core::IUnknown {
    fn from(value: BarcodeScanner) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BarcodeScanner> for ::windows::core::IUnknown {
    fn from(value: &BarcodeScanner) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BarcodeScanner {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BarcodeScanner {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BarcodeScanner> for ::windows::core::IInspectable {
    fn from(value: BarcodeScanner) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BarcodeScanner> for ::windows::core::IInspectable {
    fn from(value: &BarcodeScanner) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BarcodeScanner {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BarcodeScanner {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<BarcodeScanner> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: BarcodeScanner) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&BarcodeScanner> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &BarcodeScanner) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for BarcodeScanner {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &BarcodeScanner {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for BarcodeScanner {}
unsafe impl ::core::marker::Sync for BarcodeScanner {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BarcodeScannerCapabilities(pub ::windows::core::IInspectable);
impl BarcodeScannerCapabilities {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PowerReportingType(&self) -> ::windows::core::Result<UnifiedPosPowerReportingType> {
        let this = self;
        unsafe {
            let mut result__: UnifiedPosPowerReportingType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UnifiedPosPowerReportingType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsStatisticsReportingSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsStatisticsUpdatingSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsImagePreviewSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsSoftwareTriggerSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IBarcodeScannerCapabilities1>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsVideoPreviewSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IBarcodeScannerCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for BarcodeScannerCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.BarcodeScannerCapabilities;{c60691e4-f2c8-4420-a307-b12ef6622857})");
}
unsafe impl ::windows::core::Interface for BarcodeScannerCapabilities {
    type Vtable = IBarcodeScannerCapabilities_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc60691e4_f2c8_4420_a307_b12ef6622857);
}
impl ::windows::core::RuntimeName for BarcodeScannerCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.BarcodeScannerCapabilities";
}
impl ::core::convert::From<BarcodeScannerCapabilities> for ::windows::core::IUnknown {
    fn from(value: BarcodeScannerCapabilities) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BarcodeScannerCapabilities> for ::windows::core::IUnknown {
    fn from(value: &BarcodeScannerCapabilities) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BarcodeScannerCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BarcodeScannerCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BarcodeScannerCapabilities> for ::windows::core::IInspectable {
    fn from(value: BarcodeScannerCapabilities) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BarcodeScannerCapabilities> for ::windows::core::IInspectable {
    fn from(value: &BarcodeScannerCapabilities) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BarcodeScannerCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BarcodeScannerCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerCapabilities {}
unsafe impl ::core::marker::Sync for BarcodeScannerCapabilities {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BarcodeScannerDataReceivedEventArgs(pub ::windows::core::IInspectable);
impl BarcodeScannerDataReceivedEventArgs {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Report(&self) -> ::windows::core::Result<BarcodeScannerReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BarcodeScannerReport>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for BarcodeScannerDataReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.BarcodeScannerDataReceivedEventArgs;{4234a7e2-ed97-467d-ad2b-01e44313a929})");
}
unsafe impl ::windows::core::Interface for BarcodeScannerDataReceivedEventArgs {
    type Vtable = IBarcodeScannerDataReceivedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4234a7e2_ed97_467d_ad2b_01e44313a929);
}
impl ::windows::core::RuntimeName for BarcodeScannerDataReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.BarcodeScannerDataReceivedEventArgs";
}
impl ::core::convert::From<BarcodeScannerDataReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: BarcodeScannerDataReceivedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BarcodeScannerDataReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &BarcodeScannerDataReceivedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BarcodeScannerDataReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BarcodeScannerDataReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BarcodeScannerDataReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: BarcodeScannerDataReceivedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BarcodeScannerDataReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &BarcodeScannerDataReceivedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BarcodeScannerDataReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BarcodeScannerDataReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerDataReceivedEventArgs {}
unsafe impl ::core::marker::Sync for BarcodeScannerDataReceivedEventArgs {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BarcodeScannerErrorOccurredEventArgs(pub ::windows::core::IInspectable);
impl BarcodeScannerErrorOccurredEventArgs {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PartialInputData(&self) -> ::windows::core::Result<BarcodeScannerReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BarcodeScannerReport>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsRetriable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ErrorData(&self) -> ::windows::core::Result<UnifiedPosErrorData> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UnifiedPosErrorData>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for BarcodeScannerErrorOccurredEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.BarcodeScannerErrorOccurredEventArgs;{2cd2602f-cf3a-4002-a75a-c5ec468f0a20})");
}
unsafe impl ::windows::core::Interface for BarcodeScannerErrorOccurredEventArgs {
    type Vtable = IBarcodeScannerErrorOccurredEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2cd2602f_cf3a_4002_a75a_c5ec468f0a20);
}
impl ::windows::core::RuntimeName for BarcodeScannerErrorOccurredEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.BarcodeScannerErrorOccurredEventArgs";
}
impl ::core::convert::From<BarcodeScannerErrorOccurredEventArgs> for ::windows::core::IUnknown {
    fn from(value: BarcodeScannerErrorOccurredEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BarcodeScannerErrorOccurredEventArgs> for ::windows::core::IUnknown {
    fn from(value: &BarcodeScannerErrorOccurredEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BarcodeScannerErrorOccurredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BarcodeScannerErrorOccurredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BarcodeScannerErrorOccurredEventArgs> for ::windows::core::IInspectable {
    fn from(value: BarcodeScannerErrorOccurredEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BarcodeScannerErrorOccurredEventArgs> for ::windows::core::IInspectable {
    fn from(value: &BarcodeScannerErrorOccurredEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BarcodeScannerErrorOccurredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BarcodeScannerErrorOccurredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerErrorOccurredEventArgs {}
unsafe impl ::core::marker::Sync for BarcodeScannerErrorOccurredEventArgs {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BarcodeScannerImagePreviewReceivedEventArgs(pub ::windows::core::IInspectable);
impl BarcodeScannerImagePreviewReceivedEventArgs {
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_PointOfService`, `Storage_Streams`*"]
    pub fn Preview(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamWithContentType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStreamWithContentType>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for BarcodeScannerImagePreviewReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.BarcodeScannerImagePreviewReceivedEventArgs;{f3b7de85-6e8b-434e-9f58-06ef26bc4baf})");
}
unsafe impl ::windows::core::Interface for BarcodeScannerImagePreviewReceivedEventArgs {
    type Vtable = IBarcodeScannerImagePreviewReceivedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf3b7de85_6e8b_434e_9f58_06ef26bc4baf);
}
impl ::windows::core::RuntimeName for BarcodeScannerImagePreviewReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.BarcodeScannerImagePreviewReceivedEventArgs";
}
impl ::core::convert::From<BarcodeScannerImagePreviewReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: BarcodeScannerImagePreviewReceivedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BarcodeScannerImagePreviewReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &BarcodeScannerImagePreviewReceivedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BarcodeScannerImagePreviewReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BarcodeScannerImagePreviewReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BarcodeScannerImagePreviewReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: BarcodeScannerImagePreviewReceivedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BarcodeScannerImagePreviewReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &BarcodeScannerImagePreviewReceivedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BarcodeScannerImagePreviewReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BarcodeScannerImagePreviewReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerImagePreviewReceivedEventArgs {}
unsafe impl ::core::marker::Sync for BarcodeScannerImagePreviewReceivedEventArgs {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BarcodeScannerReport(pub ::windows::core::IInspectable);
impl BarcodeScannerReport {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ScanDataType(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_PointOfService`, `Storage_Streams`*"]
    pub fn ScanData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_PointOfService`, `Storage_Streams`*"]
    pub fn ScanDataLabel(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_PointOfService`, `Storage_Streams`*"]
    pub fn CreateInstance<'a, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>, Param2: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(scandatatype: u32, scandata: Param1, scandatalabel: Param2) -> ::windows::core::Result<BarcodeScannerReport> {
        Self::IBarcodeScannerReportFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), scandatatype, scandata.into_param().abi(), scandatalabel.into_param().abi(), &mut result__).from_abi::<BarcodeScannerReport>(result__)
        })
    }
    pub fn IBarcodeScannerReportFactory<R, F: FnOnce(&IBarcodeScannerReportFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BarcodeScannerReport, IBarcodeScannerReportFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for BarcodeScannerReport {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.BarcodeScannerReport;{5ce4d8b0-a489-4b96-86c4-f0bf8a37753d})");
}
unsafe impl ::windows::core::Interface for BarcodeScannerReport {
    type Vtable = IBarcodeScannerReport_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ce4d8b0_a489_4b96_86c4_f0bf8a37753d);
}
impl ::windows::core::RuntimeName for BarcodeScannerReport {
    const NAME: &'static str = "Windows.Devices.PointOfService.BarcodeScannerReport";
}
impl ::core::convert::From<BarcodeScannerReport> for ::windows::core::IUnknown {
    fn from(value: BarcodeScannerReport) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BarcodeScannerReport> for ::windows::core::IUnknown {
    fn from(value: &BarcodeScannerReport) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BarcodeScannerReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BarcodeScannerReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BarcodeScannerReport> for ::windows::core::IInspectable {
    fn from(value: BarcodeScannerReport) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BarcodeScannerReport> for ::windows::core::IInspectable {
    fn from(value: &BarcodeScannerReport) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BarcodeScannerReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BarcodeScannerReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerReport {}
unsafe impl ::core::marker::Sync for BarcodeScannerReport {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct BarcodeScannerStatus(pub i32);
impl BarcodeScannerStatus {
    pub const Online: BarcodeScannerStatus = BarcodeScannerStatus(0i32);
    pub const Off: BarcodeScannerStatus = BarcodeScannerStatus(1i32);
    pub const Offline: BarcodeScannerStatus = BarcodeScannerStatus(2i32);
    pub const OffOrOffline: BarcodeScannerStatus = BarcodeScannerStatus(3i32);
    pub const Extended: BarcodeScannerStatus = BarcodeScannerStatus(4i32);
}
impl ::core::convert::From<i32> for BarcodeScannerStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for BarcodeScannerStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for BarcodeScannerStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.BarcodeScannerStatus;i4)");
}
impl ::windows::core::DefaultType for BarcodeScannerStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BarcodeScannerStatusUpdatedEventArgs(pub ::windows::core::IInspectable);
impl BarcodeScannerStatusUpdatedEventArgs {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Status(&self) -> ::windows::core::Result<BarcodeScannerStatus> {
        let this = self;
        unsafe {
            let mut result__: BarcodeScannerStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BarcodeScannerStatus>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ExtendedStatus(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for BarcodeScannerStatusUpdatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.BarcodeScannerStatusUpdatedEventArgs;{355d8586-9c43-462b-a91a-816dc97f452c})");
}
unsafe impl ::windows::core::Interface for BarcodeScannerStatusUpdatedEventArgs {
    type Vtable = IBarcodeScannerStatusUpdatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x355d8586_9c43_462b_a91a_816dc97f452c);
}
impl ::windows::core::RuntimeName for BarcodeScannerStatusUpdatedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.BarcodeScannerStatusUpdatedEventArgs";
}
impl ::core::convert::From<BarcodeScannerStatusUpdatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: BarcodeScannerStatusUpdatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BarcodeScannerStatusUpdatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &BarcodeScannerStatusUpdatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BarcodeScannerStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BarcodeScannerStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BarcodeScannerStatusUpdatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: BarcodeScannerStatusUpdatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BarcodeScannerStatusUpdatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &BarcodeScannerStatusUpdatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BarcodeScannerStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BarcodeScannerStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerStatusUpdatedEventArgs {}
unsafe impl ::core::marker::Sync for BarcodeScannerStatusUpdatedEventArgs {}
#[doc = "*Required features: `Devices_PointOfService`*"]
pub struct BarcodeSymbologies {}
impl BarcodeSymbologies {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Unknown() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Ean8() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Ean8Add2() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Ean8Add5() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Eanv() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn EanvAdd2() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn EanvAdd5() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Ean13() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Ean13Add2() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Ean13Add5() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Isbn() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsbnAdd5() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Ismn() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsmnAdd2() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsmnAdd5() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Issn() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IssnAdd2() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IssnAdd5() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Ean99() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Ean99Add2() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Ean99Add5() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Upca() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn UpcaAdd2() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn UpcaAdd5() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Upce() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn UpceAdd2() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn UpceAdd5() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn UpcCoupon() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).33)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn TfStd() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).34)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn TfDis() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).35)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn TfInt() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).36)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn TfInd() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).37)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn TfMat() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).38)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn TfIata() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).39)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Gs1DatabarType1() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).40)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Gs1DatabarType2() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).41)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Gs1DatabarType3() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).42)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Code39() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).43)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Code39Ex() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).44)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Trioptic39() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).45)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Code32() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).46)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Pzn() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).47)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Code93() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).48)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Code93Ex() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).49)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Code128() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).50)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Gs1128() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).51)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Gs1128Coupon() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).52)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn UccEan128() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).53)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Sisac() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).54)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Isbt() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).55)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Codabar() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).56)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Code11() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).57)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Msi() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).58)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Plessey() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).59)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Telepen() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).60)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Code16k() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).61)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CodablockA() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).62)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CodablockF() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).63)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Codablock128() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).64)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Code49() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).65)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Aztec() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).66)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DataCode() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).67)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DataMatrix() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).68)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn HanXin() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).69)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Maxicode() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).70)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn MicroPdf417() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).71)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn MicroQr() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).72)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Pdf417() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).73)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Qr() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).74)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn MsTag() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).75)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Ccab() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).76)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Ccc() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).77)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Tlc39() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).78)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn AusPost() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).79)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CanPost() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).80)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ChinaPost() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).81)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DutchKix() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).82)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn InfoMail() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).83)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ItalianPost25() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).84)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ItalianPost39() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).85)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn JapanPost() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).86)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn KoreanPost() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).87)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SwedenPost() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).88)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn UkPost() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).89)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn UsIntelligent() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).90)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn UsIntelligentPkg() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).91)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn UsPlanet() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).92)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn UsPostNet() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).93)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Us4StateFics() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).94)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn OcrA() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).95)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn OcrB() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).96)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Micr() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).97)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ExtendedBase() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).98)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn GetName(scandatatype: u32) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).99)(::core::mem::transmute_copy(this), scandatatype, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Gs1DWCode() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics2(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    pub fn IBarcodeSymbologiesStatics<R, F: FnOnce(&IBarcodeSymbologiesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BarcodeSymbologies, IBarcodeSymbologiesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IBarcodeSymbologiesStatics2<R, F: FnOnce(&IBarcodeSymbologiesStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BarcodeSymbologies, IBarcodeSymbologiesStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for BarcodeSymbologies {
    const NAME: &'static str = "Windows.Devices.PointOfService.BarcodeSymbologies";
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BarcodeSymbologyAttributes(pub ::windows::core::IInspectable);
impl BarcodeSymbologyAttributes {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsCheckDigitValidationEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetIsCheckDigitValidationEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsCheckDigitValidationSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsCheckDigitTransmissionEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetIsCheckDigitTransmissionEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsCheckDigitTransmissionSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DecodeLength1(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetDecodeLength1(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DecodeLength2(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetDecodeLength2(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DecodeLengthKind(&self) -> ::windows::core::Result<BarcodeSymbologyDecodeLengthKind> {
        let this = self;
        unsafe {
            let mut result__: BarcodeSymbologyDecodeLengthKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BarcodeSymbologyDecodeLengthKind>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetDecodeLengthKind(&self, value: BarcodeSymbologyDecodeLengthKind) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDecodeLengthSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for BarcodeSymbologyAttributes {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.BarcodeSymbologyAttributes;{66413a78-ab7a-4ada-8ece-936014b2ead7})");
}
unsafe impl ::windows::core::Interface for BarcodeSymbologyAttributes {
    type Vtable = IBarcodeSymbologyAttributes_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66413a78_ab7a_4ada_8ece_936014b2ead7);
}
impl ::windows::core::RuntimeName for BarcodeSymbologyAttributes {
    const NAME: &'static str = "Windows.Devices.PointOfService.BarcodeSymbologyAttributes";
}
impl ::core::convert::From<BarcodeSymbologyAttributes> for ::windows::core::IUnknown {
    fn from(value: BarcodeSymbologyAttributes) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BarcodeSymbologyAttributes> for ::windows::core::IUnknown {
    fn from(value: &BarcodeSymbologyAttributes) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BarcodeSymbologyAttributes {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BarcodeSymbologyAttributes {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BarcodeSymbologyAttributes> for ::windows::core::IInspectable {
    fn from(value: BarcodeSymbologyAttributes) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BarcodeSymbologyAttributes> for ::windows::core::IInspectable {
    fn from(value: &BarcodeSymbologyAttributes) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BarcodeSymbologyAttributes {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BarcodeSymbologyAttributes {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BarcodeSymbologyAttributes {}
unsafe impl ::core::marker::Sync for BarcodeSymbologyAttributes {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct BarcodeSymbologyDecodeLengthKind(pub i32);
impl BarcodeSymbologyDecodeLengthKind {
    pub const AnyLength: BarcodeSymbologyDecodeLengthKind = BarcodeSymbologyDecodeLengthKind(0i32);
    pub const Discrete: BarcodeSymbologyDecodeLengthKind = BarcodeSymbologyDecodeLengthKind(1i32);
    pub const Range: BarcodeSymbologyDecodeLengthKind = BarcodeSymbologyDecodeLengthKind(2i32);
}
impl ::core::convert::From<i32> for BarcodeSymbologyDecodeLengthKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for BarcodeSymbologyDecodeLengthKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for BarcodeSymbologyDecodeLengthKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.BarcodeSymbologyDecodeLengthKind;i4)");
}
impl ::windows::core::DefaultType for BarcodeSymbologyDecodeLengthKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CashDrawer(pub ::windows::core::IInspectable);
impl CashDrawer {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Capabilities(&self) -> ::windows::core::Result<CashDrawerCapabilities> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CashDrawerCapabilities>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Status(&self) -> ::windows::core::Result<CashDrawerStatus> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CashDrawerStatus>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDrawerOpen(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DrawerEventSource(&self) -> ::windows::core::Result<CashDrawerEventSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CashDrawerEventSource>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn ClaimDrawerAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ClaimedCashDrawer>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ClaimedCashDrawer>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn CheckHealthAsync(&self, level: UnifiedPosHealthCheckLevel) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), level, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetStatisticsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, statisticscategories: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), statisticscategories.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn StatusUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CashDrawer, CashDrawerStatusUpdatedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveStatusUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn GetDefaultAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CashDrawer>> {
        Self::ICashDrawerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<CashDrawer>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CashDrawer>> {
        Self::ICashDrawerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<CashDrawer>>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICashDrawerStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn GetDeviceSelectorWithConnectionTypes(connectiontypes: PosConnectionTypes) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICashDrawerStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), connectiontypes, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn ICashDrawerStatics<R, F: FnOnce(&ICashDrawerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CashDrawer, ICashDrawerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ICashDrawerStatics2<R, F: FnOnce(&ICashDrawerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CashDrawer, ICashDrawerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for CashDrawer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.CashDrawer;{9f88f5c8-de54-4aee-a890-920bcbfe30fc})");
}
unsafe impl ::windows::core::Interface for CashDrawer {
    type Vtable = ICashDrawer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f88f5c8_de54_4aee_a890_920bcbfe30fc);
}
impl ::windows::core::RuntimeName for CashDrawer {
    const NAME: &'static str = "Windows.Devices.PointOfService.CashDrawer";
}
impl ::core::convert::From<CashDrawer> for ::windows::core::IUnknown {
    fn from(value: CashDrawer) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CashDrawer> for ::windows::core::IUnknown {
    fn from(value: &CashDrawer) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CashDrawer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CashDrawer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CashDrawer> for ::windows::core::IInspectable {
    fn from(value: CashDrawer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CashDrawer> for ::windows::core::IInspectable {
    fn from(value: &CashDrawer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CashDrawer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CashDrawer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<CashDrawer> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: CashDrawer) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&CashDrawer> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &CashDrawer) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for CashDrawer {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &CashDrawer {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for CashDrawer {}
unsafe impl ::core::marker::Sync for CashDrawer {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CashDrawerCapabilities(pub ::windows::core::IInspectable);
impl CashDrawerCapabilities {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PowerReportingType(&self) -> ::windows::core::Result<UnifiedPosPowerReportingType> {
        let this = self;
        unsafe {
            let mut result__: UnifiedPosPowerReportingType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UnifiedPosPowerReportingType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsStatisticsReportingSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsStatisticsUpdatingSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsStatusReportingSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsStatusMultiDrawerDetectSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDrawerOpenSensorAvailable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CashDrawerCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.CashDrawerCapabilities;{0bc6de0b-e8e7-4b1f-b1d1-3e501ad08247})");
}
unsafe impl ::windows::core::Interface for CashDrawerCapabilities {
    type Vtable = ICashDrawerCapabilities_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0bc6de0b_e8e7_4b1f_b1d1_3e501ad08247);
}
impl ::windows::core::RuntimeName for CashDrawerCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.CashDrawerCapabilities";
}
impl ::core::convert::From<CashDrawerCapabilities> for ::windows::core::IUnknown {
    fn from(value: CashDrawerCapabilities) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CashDrawerCapabilities> for ::windows::core::IUnknown {
    fn from(value: &CashDrawerCapabilities) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CashDrawerCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CashDrawerCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CashDrawerCapabilities> for ::windows::core::IInspectable {
    fn from(value: CashDrawerCapabilities) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CashDrawerCapabilities> for ::windows::core::IInspectable {
    fn from(value: &CashDrawerCapabilities) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CashDrawerCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CashDrawerCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CashDrawerCapabilities {}
unsafe impl ::core::marker::Sync for CashDrawerCapabilities {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CashDrawerCloseAlarm(pub ::windows::core::IInspectable);
impl CashDrawerCloseAlarm {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn SetAlarmTimeout<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn AlarmTimeout(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetBeepFrequency(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn BeepFrequency(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn SetBeepDuration<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn BeepDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn SetBeepDelay<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn BeepDelay(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn AlarmTimeoutExpired<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CashDrawerCloseAlarm, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveAlarmTimeoutExpired<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn StartAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CashDrawerCloseAlarm {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.CashDrawerCloseAlarm;{6bf88cc7-6f63-430e-ab3b-95d75ffbe87f})");
}
unsafe impl ::windows::core::Interface for CashDrawerCloseAlarm {
    type Vtable = ICashDrawerCloseAlarm_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6bf88cc7_6f63_430e_ab3b_95d75ffbe87f);
}
impl ::windows::core::RuntimeName for CashDrawerCloseAlarm {
    const NAME: &'static str = "Windows.Devices.PointOfService.CashDrawerCloseAlarm";
}
impl ::core::convert::From<CashDrawerCloseAlarm> for ::windows::core::IUnknown {
    fn from(value: CashDrawerCloseAlarm) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CashDrawerCloseAlarm> for ::windows::core::IUnknown {
    fn from(value: &CashDrawerCloseAlarm) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CashDrawerCloseAlarm {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CashDrawerCloseAlarm {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CashDrawerCloseAlarm> for ::windows::core::IInspectable {
    fn from(value: CashDrawerCloseAlarm) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CashDrawerCloseAlarm> for ::windows::core::IInspectable {
    fn from(value: &CashDrawerCloseAlarm) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CashDrawerCloseAlarm {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CashDrawerCloseAlarm {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CashDrawerCloseAlarm {}
unsafe impl ::core::marker::Sync for CashDrawerCloseAlarm {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CashDrawerClosedEventArgs(pub ::windows::core::IInspectable);
impl CashDrawerClosedEventArgs {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CashDrawer(&self) -> ::windows::core::Result<CashDrawer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CashDrawer>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CashDrawerClosedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.CashDrawerClosedEventArgs;{69cb3bc1-147f-421c-9c23-090123bb786c})");
}
unsafe impl ::windows::core::Interface for CashDrawerClosedEventArgs {
    type Vtable = ICashDrawerEventSourceEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69cb3bc1_147f_421c_9c23_090123bb786c);
}
impl ::windows::core::RuntimeName for CashDrawerClosedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.CashDrawerClosedEventArgs";
}
impl ::core::convert::From<CashDrawerClosedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CashDrawerClosedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CashDrawerClosedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CashDrawerClosedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CashDrawerClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CashDrawerClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CashDrawerClosedEventArgs> for ::windows::core::IInspectable {
    fn from(value: CashDrawerClosedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CashDrawerClosedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CashDrawerClosedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CashDrawerClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CashDrawerClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<CashDrawerClosedEventArgs> for ICashDrawerEventSourceEventArgs {
    fn from(value: CashDrawerClosedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CashDrawerClosedEventArgs> for ICashDrawerEventSourceEventArgs {
    fn from(value: &CashDrawerClosedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICashDrawerEventSourceEventArgs> for CashDrawerClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ICashDrawerEventSourceEventArgs> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICashDrawerEventSourceEventArgs> for &CashDrawerClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ICashDrawerEventSourceEventArgs> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CashDrawerClosedEventArgs {}
unsafe impl ::core::marker::Sync for CashDrawerClosedEventArgs {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CashDrawerEventSource(pub ::windows::core::IInspectable);
impl CashDrawerEventSource {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn DrawerClosed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CashDrawerEventSource, CashDrawerClosedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveDrawerClosed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn DrawerOpened<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CashDrawerEventSource, CashDrawerOpenedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveDrawerOpened<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for CashDrawerEventSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.CashDrawerEventSource;{e006e46c-f2f9-442f-8dd6-06c10a4227ba})");
}
unsafe impl ::windows::core::Interface for CashDrawerEventSource {
    type Vtable = ICashDrawerEventSource_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe006e46c_f2f9_442f_8dd6_06c10a4227ba);
}
impl ::windows::core::RuntimeName for CashDrawerEventSource {
    const NAME: &'static str = "Windows.Devices.PointOfService.CashDrawerEventSource";
}
impl ::core::convert::From<CashDrawerEventSource> for ::windows::core::IUnknown {
    fn from(value: CashDrawerEventSource) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CashDrawerEventSource> for ::windows::core::IUnknown {
    fn from(value: &CashDrawerEventSource) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CashDrawerEventSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CashDrawerEventSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CashDrawerEventSource> for ::windows::core::IInspectable {
    fn from(value: CashDrawerEventSource) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CashDrawerEventSource> for ::windows::core::IInspectable {
    fn from(value: &CashDrawerEventSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CashDrawerEventSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CashDrawerEventSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CashDrawerEventSource {}
unsafe impl ::core::marker::Sync for CashDrawerEventSource {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CashDrawerOpenedEventArgs(pub ::windows::core::IInspectable);
impl CashDrawerOpenedEventArgs {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CashDrawer(&self) -> ::windows::core::Result<CashDrawer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CashDrawer>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CashDrawerOpenedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.CashDrawerOpenedEventArgs;{69cb3bc1-147f-421c-9c23-090123bb786c})");
}
unsafe impl ::windows::core::Interface for CashDrawerOpenedEventArgs {
    type Vtable = ICashDrawerEventSourceEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69cb3bc1_147f_421c_9c23_090123bb786c);
}
impl ::windows::core::RuntimeName for CashDrawerOpenedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.CashDrawerOpenedEventArgs";
}
impl ::core::convert::From<CashDrawerOpenedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CashDrawerOpenedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CashDrawerOpenedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CashDrawerOpenedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CashDrawerOpenedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CashDrawerOpenedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CashDrawerOpenedEventArgs> for ::windows::core::IInspectable {
    fn from(value: CashDrawerOpenedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CashDrawerOpenedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CashDrawerOpenedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CashDrawerOpenedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CashDrawerOpenedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<CashDrawerOpenedEventArgs> for ICashDrawerEventSourceEventArgs {
    fn from(value: CashDrawerOpenedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CashDrawerOpenedEventArgs> for ICashDrawerEventSourceEventArgs {
    fn from(value: &CashDrawerOpenedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICashDrawerEventSourceEventArgs> for CashDrawerOpenedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ICashDrawerEventSourceEventArgs> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICashDrawerEventSourceEventArgs> for &CashDrawerOpenedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ICashDrawerEventSourceEventArgs> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CashDrawerOpenedEventArgs {}
unsafe impl ::core::marker::Sync for CashDrawerOpenedEventArgs {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CashDrawerStatus(pub ::windows::core::IInspectable);
impl CashDrawerStatus {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn StatusKind(&self) -> ::windows::core::Result<CashDrawerStatusKind> {
        let this = self;
        unsafe {
            let mut result__: CashDrawerStatusKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CashDrawerStatusKind>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ExtendedStatus(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CashDrawerStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.CashDrawerStatus;{6bbd78bf-dca1-4e06-99eb-5af6a5aec108})");
}
unsafe impl ::windows::core::Interface for CashDrawerStatus {
    type Vtable = ICashDrawerStatus_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6bbd78bf_dca1_4e06_99eb_5af6a5aec108);
}
impl ::windows::core::RuntimeName for CashDrawerStatus {
    const NAME: &'static str = "Windows.Devices.PointOfService.CashDrawerStatus";
}
impl ::core::convert::From<CashDrawerStatus> for ::windows::core::IUnknown {
    fn from(value: CashDrawerStatus) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CashDrawerStatus> for ::windows::core::IUnknown {
    fn from(value: &CashDrawerStatus) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CashDrawerStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CashDrawerStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CashDrawerStatus> for ::windows::core::IInspectable {
    fn from(value: CashDrawerStatus) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CashDrawerStatus> for ::windows::core::IInspectable {
    fn from(value: &CashDrawerStatus) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CashDrawerStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CashDrawerStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CashDrawerStatus {}
unsafe impl ::core::marker::Sync for CashDrawerStatus {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CashDrawerStatusKind(pub i32);
impl CashDrawerStatusKind {
    pub const Online: CashDrawerStatusKind = CashDrawerStatusKind(0i32);
    pub const Off: CashDrawerStatusKind = CashDrawerStatusKind(1i32);
    pub const Offline: CashDrawerStatusKind = CashDrawerStatusKind(2i32);
    pub const OffOrOffline: CashDrawerStatusKind = CashDrawerStatusKind(3i32);
    pub const Extended: CashDrawerStatusKind = CashDrawerStatusKind(4i32);
}
impl ::core::convert::From<i32> for CashDrawerStatusKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CashDrawerStatusKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for CashDrawerStatusKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.CashDrawerStatusKind;i4)");
}
impl ::windows::core::DefaultType for CashDrawerStatusKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct CashDrawerStatusUpdatedEventArgs(pub ::windows::core::IInspectable);
impl CashDrawerStatusUpdatedEventArgs {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Status(&self) -> ::windows::core::Result<CashDrawerStatus> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CashDrawerStatus>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for CashDrawerStatusUpdatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.CashDrawerStatusUpdatedEventArgs;{30aae98a-0d70-459c-9553-87e124c52488})");
}
unsafe impl ::windows::core::Interface for CashDrawerStatusUpdatedEventArgs {
    type Vtable = ICashDrawerStatusUpdatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30aae98a_0d70_459c_9553_87e124c52488);
}
impl ::windows::core::RuntimeName for CashDrawerStatusUpdatedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.CashDrawerStatusUpdatedEventArgs";
}
impl ::core::convert::From<CashDrawerStatusUpdatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CashDrawerStatusUpdatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&CashDrawerStatusUpdatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CashDrawerStatusUpdatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CashDrawerStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CashDrawerStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<CashDrawerStatusUpdatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: CashDrawerStatusUpdatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&CashDrawerStatusUpdatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CashDrawerStatusUpdatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CashDrawerStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CashDrawerStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for CashDrawerStatusUpdatedEventArgs {}
unsafe impl ::core::marker::Sync for CashDrawerStatusUpdatedEventArgs {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ClaimedBarcodeScanner(pub ::windows::core::IInspectable);
impl ClaimedBarcodeScanner {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetIsDisabledOnDataReceived(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDisabledOnDataReceived(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetIsDecodeDataEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDecodeDataEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn EnableAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn DisableAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn RetainDevice(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`, `Foundation_Collections`*"]
    pub fn SetActiveSymbologiesAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<u32>>>(&self, symbologies: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), symbologies.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`, `Foundation_Collections`*"]
    pub fn ResetStatisticsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, statisticscategories: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), statisticscategories.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`, `Foundation_Collections`*"]
    pub fn UpdateStatisticsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>>(&self, statistics: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), statistics.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn SetActiveProfileAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, profile: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), profile.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn DataReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ClaimedBarcodeScanner, BarcodeScannerDataReceivedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveDataReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn TriggerPressed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<ClaimedBarcodeScanner>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveTriggerPressed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn TriggerReleased<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<ClaimedBarcodeScanner>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveTriggerReleased<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn ReleaseDeviceRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<ClaimedBarcodeScanner>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveReleaseDeviceRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn ImagePreviewReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ClaimedBarcodeScanner, BarcodeScannerImagePreviewReceivedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveImagePreviewReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn ErrorOccurred<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ClaimedBarcodeScanner, BarcodeScannerErrorOccurredEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveErrorOccurred<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn StartSoftwareTriggerAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IClaimedBarcodeScanner1>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn StopSoftwareTriggerAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IClaimedBarcodeScanner1>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn GetSymbologyAttributesAsync(&self, barcodesymbology: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BarcodeSymbologyAttributes>> {
        let this = &::windows::core::Interface::cast::<IClaimedBarcodeScanner2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), barcodesymbology, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BarcodeSymbologyAttributes>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn SetSymbologyAttributesAsync<'a, Param1: ::windows::core::IntoParam<'a, BarcodeSymbologyAttributes>>(&self, barcodesymbology: u32, attributes: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IClaimedBarcodeScanner2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), barcodesymbology, attributes.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn ShowVideoPreviewAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IClaimedBarcodeScanner3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn HideVideoPreview(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IClaimedBarcodeScanner3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetIsVideoPreviewShownOnEnable(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IClaimedBarcodeScanner3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsVideoPreviewShownOnEnable(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IClaimedBarcodeScanner3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn Closed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ClaimedBarcodeScanner, ClaimedBarcodeScannerClosedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IClaimedBarcodeScanner4>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveClosed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IClaimedBarcodeScanner4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for ClaimedBarcodeScanner {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedBarcodeScanner;{4a63b49c-8fa4-4332-bb26-945d11d81e0f})");
}
unsafe impl ::windows::core::Interface for ClaimedBarcodeScanner {
    type Vtable = IClaimedBarcodeScanner_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4a63b49c_8fa4_4332_bb26_945d11d81e0f);
}
impl ::windows::core::RuntimeName for ClaimedBarcodeScanner {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedBarcodeScanner";
}
impl ::core::convert::From<ClaimedBarcodeScanner> for ::windows::core::IUnknown {
    fn from(value: ClaimedBarcodeScanner) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ClaimedBarcodeScanner> for ::windows::core::IUnknown {
    fn from(value: &ClaimedBarcodeScanner) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ClaimedBarcodeScanner {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ClaimedBarcodeScanner {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ClaimedBarcodeScanner> for ::windows::core::IInspectable {
    fn from(value: ClaimedBarcodeScanner) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ClaimedBarcodeScanner> for ::windows::core::IInspectable {
    fn from(value: &ClaimedBarcodeScanner) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ClaimedBarcodeScanner {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ClaimedBarcodeScanner {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<ClaimedBarcodeScanner> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: ClaimedBarcodeScanner) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&ClaimedBarcodeScanner> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &ClaimedBarcodeScanner) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for ClaimedBarcodeScanner {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &ClaimedBarcodeScanner {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ClaimedBarcodeScanner {}
unsafe impl ::core::marker::Sync for ClaimedBarcodeScanner {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ClaimedBarcodeScannerClosedEventArgs(pub ::windows::core::IInspectable);
impl ClaimedBarcodeScannerClosedEventArgs {}
unsafe impl ::windows::core::RuntimeType for ClaimedBarcodeScannerClosedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedBarcodeScannerClosedEventArgs;{cf7d5489-a22c-4c65-a901-88d77d833954})");
}
unsafe impl ::windows::core::Interface for ClaimedBarcodeScannerClosedEventArgs {
    type Vtable = IClaimedBarcodeScannerClosedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf7d5489_a22c_4c65_a901_88d77d833954);
}
impl ::windows::core::RuntimeName for ClaimedBarcodeScannerClosedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedBarcodeScannerClosedEventArgs";
}
impl ::core::convert::From<ClaimedBarcodeScannerClosedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ClaimedBarcodeScannerClosedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ClaimedBarcodeScannerClosedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ClaimedBarcodeScannerClosedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ClaimedBarcodeScannerClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ClaimedBarcodeScannerClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ClaimedBarcodeScannerClosedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ClaimedBarcodeScannerClosedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ClaimedBarcodeScannerClosedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ClaimedBarcodeScannerClosedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ClaimedBarcodeScannerClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ClaimedBarcodeScannerClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ClaimedBarcodeScannerClosedEventArgs {}
unsafe impl ::core::marker::Sync for ClaimedBarcodeScannerClosedEventArgs {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ClaimedCashDrawer(pub ::windows::core::IInspectable);
impl ClaimedCashDrawer {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDrawerOpen(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CloseAlarm(&self) -> ::windows::core::Result<CashDrawerCloseAlarm> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CashDrawerCloseAlarm>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn OpenDrawerAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn EnableAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn DisableAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RetainDeviceAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`, `Foundation_Collections`*"]
    pub fn ResetStatisticsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, statisticscategories: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), statisticscategories.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`, `Foundation_Collections`*"]
    pub fn UpdateStatisticsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>>(&self, statistics: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), statistics.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn ReleaseDeviceRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ClaimedCashDrawer, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveReleaseDeviceRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn Closed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ClaimedCashDrawer, ClaimedCashDrawerClosedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IClaimedCashDrawer2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveClosed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IClaimedCashDrawer2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for ClaimedCashDrawer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedCashDrawer;{ca3f99af-abb8-42c1-8a84-5c66512f5a75})");
}
unsafe impl ::windows::core::Interface for ClaimedCashDrawer {
    type Vtable = IClaimedCashDrawer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca3f99af_abb8_42c1_8a84_5c66512f5a75);
}
impl ::windows::core::RuntimeName for ClaimedCashDrawer {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedCashDrawer";
}
impl ::core::convert::From<ClaimedCashDrawer> for ::windows::core::IUnknown {
    fn from(value: ClaimedCashDrawer) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ClaimedCashDrawer> for ::windows::core::IUnknown {
    fn from(value: &ClaimedCashDrawer) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ClaimedCashDrawer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ClaimedCashDrawer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ClaimedCashDrawer> for ::windows::core::IInspectable {
    fn from(value: ClaimedCashDrawer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ClaimedCashDrawer> for ::windows::core::IInspectable {
    fn from(value: &ClaimedCashDrawer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ClaimedCashDrawer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ClaimedCashDrawer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<ClaimedCashDrawer> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: ClaimedCashDrawer) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&ClaimedCashDrawer> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &ClaimedCashDrawer) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for ClaimedCashDrawer {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &ClaimedCashDrawer {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ClaimedCashDrawer {}
unsafe impl ::core::marker::Sync for ClaimedCashDrawer {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ClaimedCashDrawerClosedEventArgs(pub ::windows::core::IInspectable);
impl ClaimedCashDrawerClosedEventArgs {}
unsafe impl ::windows::core::RuntimeType for ClaimedCashDrawerClosedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedCashDrawerClosedEventArgs;{cc573f33-3f34-4c5c-baae-deadf16cd7fa})");
}
unsafe impl ::windows::core::Interface for ClaimedCashDrawerClosedEventArgs {
    type Vtable = IClaimedCashDrawerClosedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcc573f33_3f34_4c5c_baae_deadf16cd7fa);
}
impl ::windows::core::RuntimeName for ClaimedCashDrawerClosedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedCashDrawerClosedEventArgs";
}
impl ::core::convert::From<ClaimedCashDrawerClosedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ClaimedCashDrawerClosedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ClaimedCashDrawerClosedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ClaimedCashDrawerClosedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ClaimedCashDrawerClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ClaimedCashDrawerClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ClaimedCashDrawerClosedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ClaimedCashDrawerClosedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ClaimedCashDrawerClosedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ClaimedCashDrawerClosedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ClaimedCashDrawerClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ClaimedCashDrawerClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ClaimedCashDrawerClosedEventArgs {}
unsafe impl ::core::marker::Sync for ClaimedCashDrawerClosedEventArgs {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ClaimedJournalPrinter(pub ::windows::core::IInspectable);
impl ClaimedJournalPrinter {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CreateJob(&self) -> ::windows::core::Result<JournalPrintJob> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<JournalPrintJob>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetCharactersPerLine(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CharactersPerLine(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetLineHeight(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn LineHeight(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetLineSpacing(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn LineSpacing(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn LineWidth(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetIsLetterQuality(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsLetterQuality(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsPaperNearEnd(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetColorCartridge(&self, value: PosPrinterColorCartridge) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ColorCartridge(&self) -> ::windows::core::Result<PosPrinterColorCartridge> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: PosPrinterColorCartridge = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterColorCartridge>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsCoverOpen(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsCartridgeRemoved(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsCartridgeEmpty(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsHeadCleaning(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsPaperEmpty(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsReadyToPrint(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ValidateData<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, data: Param0) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), data.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ClaimedJournalPrinter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedJournalPrinter;{67ea0630-517d-487f-9fdf-d2e0a0a264a5})");
}
unsafe impl ::windows::core::Interface for ClaimedJournalPrinter {
    type Vtable = IClaimedJournalPrinter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x67ea0630_517d_487f_9fdf_d2e0a0a264a5);
}
impl ::windows::core::RuntimeName for ClaimedJournalPrinter {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedJournalPrinter";
}
impl ::core::convert::From<ClaimedJournalPrinter> for ::windows::core::IUnknown {
    fn from(value: ClaimedJournalPrinter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ClaimedJournalPrinter> for ::windows::core::IUnknown {
    fn from(value: &ClaimedJournalPrinter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ClaimedJournalPrinter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ClaimedJournalPrinter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ClaimedJournalPrinter> for ::windows::core::IInspectable {
    fn from(value: ClaimedJournalPrinter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ClaimedJournalPrinter> for ::windows::core::IInspectable {
    fn from(value: &ClaimedJournalPrinter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ClaimedJournalPrinter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ClaimedJournalPrinter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<ClaimedJournalPrinter> for ICommonClaimedPosPrinterStation {
    type Error = ::windows::core::Error;
    fn try_from(value: ClaimedJournalPrinter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ClaimedJournalPrinter> for ICommonClaimedPosPrinterStation {
    type Error = ::windows::core::Error;
    fn try_from(value: &ClaimedJournalPrinter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICommonClaimedPosPrinterStation> for ClaimedJournalPrinter {
    fn into_param(self) -> ::windows::core::Param<'a, ICommonClaimedPosPrinterStation> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICommonClaimedPosPrinterStation> for &ClaimedJournalPrinter {
    fn into_param(self) -> ::windows::core::Param<'a, ICommonClaimedPosPrinterStation> {
        ::core::convert::TryInto::<ICommonClaimedPosPrinterStation>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ClaimedJournalPrinter {}
unsafe impl ::core::marker::Sync for ClaimedJournalPrinter {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ClaimedLineDisplay(pub ::windows::core::IInspectable);
impl ClaimedLineDisplay {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Capabilities(&self) -> ::windows::core::Result<LineDisplayCapabilities> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LineDisplayCapabilities>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PhysicalDeviceName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PhysicalDeviceDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DeviceControlDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DeviceControlVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DeviceServiceVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DefaultWindow(&self) -> ::windows::core::Result<LineDisplayWindow> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LineDisplayWindow>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn RetainDevice(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn ReleaseDeviceRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ClaimedLineDisplay, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveReleaseDeviceRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ClaimedLineDisplay>> {
        Self::IClaimedLineDisplayStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ClaimedLineDisplay>>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IClaimedLineDisplayStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn GetDeviceSelectorWithConnectionTypes(connectiontypes: PosConnectionTypes) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IClaimedLineDisplayStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), connectiontypes, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetStatisticsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, statisticscategories: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), statisticscategories.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn CheckHealthAsync(&self, level: UnifiedPosHealthCheckLevel) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), level, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn CheckPowerStatusAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LineDisplayPowerStatus>> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LineDisplayPowerStatus>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn StatusUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ClaimedLineDisplay, LineDisplayStatusUpdatedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveStatusUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`, `Foundation_Collections`*"]
    pub fn SupportedScreenSizesInCharacters(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Foundation::Size>> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Foundation::Size>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn MaxBitmapSizeInPixels(&self) -> ::windows::core::Result<super::super::Foundation::Size> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::Size = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation_Collections`*"]
    pub fn SupportedCharacterSets(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i32>> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<i32>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CustomGlyphs(&self) -> ::windows::core::Result<LineDisplayCustomGlyphs> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LineDisplayCustomGlyphs>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn GetAttributes(&self) -> ::windows::core::Result<LineDisplayAttributes> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LineDisplayAttributes>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn TryUpdateAttributesAsync<'a, Param0: ::windows::core::IntoParam<'a, LineDisplayAttributes>>(&self, attributes: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), attributes.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn TrySetDescriptorAsync(&self, descriptor: u32, descriptorstate: LineDisplayDescriptorState) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), descriptor, descriptorstate, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn TryClearDescriptorsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn TryCreateWindowAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Size>>(&self, viewport: Param0, windowsize: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LineDisplayWindow>> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), viewport.into_param().abi(), windowsize.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LineDisplayWindow>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`, `Storage`*"]
    pub fn TryStoreStorageFileBitmapAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::StorageFile>>(&self, bitmap: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LineDisplayStoredBitmap>> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), bitmap.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LineDisplayStoredBitmap>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`, `Storage`*"]
    pub fn TryStoreStorageFileBitmapWithAlignmentAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::StorageFile>>(&self, bitmap: Param0, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LineDisplayStoredBitmap>> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), bitmap.into_param().abi(), horizontalalignment, verticalalignment, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LineDisplayStoredBitmap>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`, `Storage`*"]
    pub fn TryStoreStorageFileBitmapWithAlignmentAndWidthAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::StorageFile>>(&self, bitmap: Param0, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment, widthinpixels: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LineDisplayStoredBitmap>> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), bitmap.into_param().abi(), horizontalalignment, verticalalignment, widthinpixels, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LineDisplayStoredBitmap>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn Closed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ClaimedLineDisplay, ClaimedLineDisplayClosedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay3>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveClosed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn IClaimedLineDisplayStatics<R, F: FnOnce(&IClaimedLineDisplayStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ClaimedLineDisplay, IClaimedLineDisplayStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ClaimedLineDisplay {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedLineDisplay;{120ac970-9a75-4acf-aae7-09972bcf8794})");
}
unsafe impl ::windows::core::Interface for ClaimedLineDisplay {
    type Vtable = IClaimedLineDisplay_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x120ac970_9a75_4acf_aae7_09972bcf8794);
}
impl ::windows::core::RuntimeName for ClaimedLineDisplay {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedLineDisplay";
}
impl ::core::convert::From<ClaimedLineDisplay> for ::windows::core::IUnknown {
    fn from(value: ClaimedLineDisplay) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ClaimedLineDisplay> for ::windows::core::IUnknown {
    fn from(value: &ClaimedLineDisplay) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ClaimedLineDisplay {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ClaimedLineDisplay {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ClaimedLineDisplay> for ::windows::core::IInspectable {
    fn from(value: ClaimedLineDisplay) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ClaimedLineDisplay> for ::windows::core::IInspectable {
    fn from(value: &ClaimedLineDisplay) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ClaimedLineDisplay {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ClaimedLineDisplay {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<ClaimedLineDisplay> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: ClaimedLineDisplay) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&ClaimedLineDisplay> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &ClaimedLineDisplay) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for ClaimedLineDisplay {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &ClaimedLineDisplay {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ClaimedLineDisplay {}
unsafe impl ::core::marker::Sync for ClaimedLineDisplay {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ClaimedLineDisplayClosedEventArgs(pub ::windows::core::IInspectable);
impl ClaimedLineDisplayClosedEventArgs {}
unsafe impl ::windows::core::RuntimeType for ClaimedLineDisplayClosedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedLineDisplayClosedEventArgs;{f915f364-d3d5-4f10-b511-90939edfacd8})");
}
unsafe impl ::windows::core::Interface for ClaimedLineDisplayClosedEventArgs {
    type Vtable = IClaimedLineDisplayClosedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf915f364_d3d5_4f10_b511_90939edfacd8);
}
impl ::windows::core::RuntimeName for ClaimedLineDisplayClosedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedLineDisplayClosedEventArgs";
}
impl ::core::convert::From<ClaimedLineDisplayClosedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ClaimedLineDisplayClosedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ClaimedLineDisplayClosedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ClaimedLineDisplayClosedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ClaimedLineDisplayClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ClaimedLineDisplayClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ClaimedLineDisplayClosedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ClaimedLineDisplayClosedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ClaimedLineDisplayClosedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ClaimedLineDisplayClosedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ClaimedLineDisplayClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ClaimedLineDisplayClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ClaimedLineDisplayClosedEventArgs {}
unsafe impl ::core::marker::Sync for ClaimedLineDisplayClosedEventArgs {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ClaimedMagneticStripeReader(pub ::windows::core::IInspectable);
impl ClaimedMagneticStripeReader {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetIsDisabledOnDataReceived(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDisabledOnDataReceived(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetIsDecodeDataEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDecodeDataEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDeviceAuthenticated(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetDataEncryptionAlgorithm(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DataEncryptionAlgorithm(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetTracksToRead(&self, value: MagneticStripeReaderTrackIds) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn TracksToRead(&self) -> ::windows::core::Result<MagneticStripeReaderTrackIds> {
        let this = self;
        unsafe {
            let mut result__: MagneticStripeReaderTrackIds = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MagneticStripeReaderTrackIds>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetIsTransmitSentinelsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsTransmitSentinelsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn EnableAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn DisableAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn RetainDevice(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetErrorReportingType(&self, value: MagneticStripeReaderErrorReportingType) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`, `Storage_Streams`*"]
    pub fn RetrieveDeviceAuthenticationDataAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn AuthenticateDeviceAsync(&self, responsetoken: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), responsetoken.len() as u32, ::core::mem::transmute(responsetoken.as_ptr()), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn DeAuthenticateDeviceAsync(&self, responsetoken: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), responsetoken.len() as u32, ::core::mem::transmute(responsetoken.as_ptr()), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn UpdateKeyAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, key: Param0, keyname: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), key.into_param().abi(), keyname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`, `Foundation_Collections`*"]
    pub fn ResetStatisticsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, statisticscategories: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), statisticscategories.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`, `Foundation_Collections`*"]
    pub fn UpdateStatisticsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>>(&self, statistics: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), statistics.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn BankCardDataReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, MagneticStripeReaderBankCardDataReceivedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveBankCardDataReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn AamvaCardDataReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, MagneticStripeReaderAamvaCardDataReceivedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveAamvaCardDataReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn VendorSpecificDataReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).33)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveVendorSpecificDataReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).34)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn ReleaseDeviceRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<ClaimedMagneticStripeReader>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).35)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveReleaseDeviceRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).36)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn ErrorOccurred<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, MagneticStripeReaderErrorOccurredEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).37)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveErrorOccurred<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).38)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn Closed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, ClaimedMagneticStripeReaderClosedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IClaimedMagneticStripeReader2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveClosed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IClaimedMagneticStripeReader2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for ClaimedMagneticStripeReader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedMagneticStripeReader;{475ca8f3-9417-48bc-b9d7-4163a7844c02})");
}
unsafe impl ::windows::core::Interface for ClaimedMagneticStripeReader {
    type Vtable = IClaimedMagneticStripeReader_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x475ca8f3_9417_48bc_b9d7_4163a7844c02);
}
impl ::windows::core::RuntimeName for ClaimedMagneticStripeReader {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedMagneticStripeReader";
}
impl ::core::convert::From<ClaimedMagneticStripeReader> for ::windows::core::IUnknown {
    fn from(value: ClaimedMagneticStripeReader) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ClaimedMagneticStripeReader> for ::windows::core::IUnknown {
    fn from(value: &ClaimedMagneticStripeReader) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ClaimedMagneticStripeReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ClaimedMagneticStripeReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ClaimedMagneticStripeReader> for ::windows::core::IInspectable {
    fn from(value: ClaimedMagneticStripeReader) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ClaimedMagneticStripeReader> for ::windows::core::IInspectable {
    fn from(value: &ClaimedMagneticStripeReader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ClaimedMagneticStripeReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ClaimedMagneticStripeReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<ClaimedMagneticStripeReader> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: ClaimedMagneticStripeReader) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&ClaimedMagneticStripeReader> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &ClaimedMagneticStripeReader) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for ClaimedMagneticStripeReader {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &ClaimedMagneticStripeReader {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ClaimedMagneticStripeReader {}
unsafe impl ::core::marker::Sync for ClaimedMagneticStripeReader {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ClaimedMagneticStripeReaderClosedEventArgs(pub ::windows::core::IInspectable);
impl ClaimedMagneticStripeReaderClosedEventArgs {}
unsafe impl ::windows::core::RuntimeType for ClaimedMagneticStripeReaderClosedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedMagneticStripeReaderClosedEventArgs;{14ada93a-adcd-4c80-acda-c3eaed2647e1})");
}
unsafe impl ::windows::core::Interface for ClaimedMagneticStripeReaderClosedEventArgs {
    type Vtable = IClaimedMagneticStripeReaderClosedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x14ada93a_adcd_4c80_acda_c3eaed2647e1);
}
impl ::windows::core::RuntimeName for ClaimedMagneticStripeReaderClosedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedMagneticStripeReaderClosedEventArgs";
}
impl ::core::convert::From<ClaimedMagneticStripeReaderClosedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ClaimedMagneticStripeReaderClosedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ClaimedMagneticStripeReaderClosedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ClaimedMagneticStripeReaderClosedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ClaimedMagneticStripeReaderClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ClaimedMagneticStripeReaderClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ClaimedMagneticStripeReaderClosedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ClaimedMagneticStripeReaderClosedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ClaimedMagneticStripeReaderClosedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ClaimedMagneticStripeReaderClosedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ClaimedMagneticStripeReaderClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ClaimedMagneticStripeReaderClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ClaimedMagneticStripeReaderClosedEventArgs {}
unsafe impl ::core::marker::Sync for ClaimedMagneticStripeReaderClosedEventArgs {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ClaimedPosPrinter(pub ::windows::core::IInspectable);
impl ClaimedPosPrinter {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetCharacterSet(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CharacterSet(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsCoverOpen(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetIsCharacterSetMappingEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsCharacterSetMappingEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetMapMode(&self, value: PosPrinterMapMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn MapMode(&self) -> ::windows::core::Result<PosPrinterMapMode> {
        let this = self;
        unsafe {
            let mut result__: PosPrinterMapMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterMapMode>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Receipt(&self) -> ::windows::core::Result<ClaimedReceiptPrinter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ClaimedReceiptPrinter>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Slip(&self) -> ::windows::core::Result<ClaimedSlipPrinter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ClaimedSlipPrinter>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Journal(&self) -> ::windows::core::Result<ClaimedJournalPrinter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ClaimedJournalPrinter>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn EnableAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn DisableAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RetainDeviceAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`, `Foundation_Collections`*"]
    pub fn ResetStatisticsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, statisticscategories: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), statisticscategories.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`, `Foundation_Collections`*"]
    pub fn UpdateStatisticsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>>(&self, statistics: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), statistics.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn ReleaseDeviceRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ClaimedPosPrinter, PosPrinterReleaseDeviceRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveReleaseDeviceRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn Closed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ClaimedPosPrinter, ClaimedPosPrinterClosedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IClaimedPosPrinter2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveClosed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IClaimedPosPrinter2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for ClaimedPosPrinter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedPosPrinter;{6d64ce0c-e03e-4b14-a38e-c28c34b86353})");
}
unsafe impl ::windows::core::Interface for ClaimedPosPrinter {
    type Vtable = IClaimedPosPrinter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d64ce0c_e03e_4b14_a38e_c28c34b86353);
}
impl ::windows::core::RuntimeName for ClaimedPosPrinter {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedPosPrinter";
}
impl ::core::convert::From<ClaimedPosPrinter> for ::windows::core::IUnknown {
    fn from(value: ClaimedPosPrinter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ClaimedPosPrinter> for ::windows::core::IUnknown {
    fn from(value: &ClaimedPosPrinter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ClaimedPosPrinter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ClaimedPosPrinter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ClaimedPosPrinter> for ::windows::core::IInspectable {
    fn from(value: ClaimedPosPrinter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ClaimedPosPrinter> for ::windows::core::IInspectable {
    fn from(value: &ClaimedPosPrinter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ClaimedPosPrinter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ClaimedPosPrinter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<ClaimedPosPrinter> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: ClaimedPosPrinter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&ClaimedPosPrinter> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &ClaimedPosPrinter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for ClaimedPosPrinter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &ClaimedPosPrinter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ClaimedPosPrinter {}
unsafe impl ::core::marker::Sync for ClaimedPosPrinter {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ClaimedPosPrinterClosedEventArgs(pub ::windows::core::IInspectable);
impl ClaimedPosPrinterClosedEventArgs {}
unsafe impl ::windows::core::RuntimeType for ClaimedPosPrinterClosedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedPosPrinterClosedEventArgs;{e2b7a27b-4d40-471d-92ed-63375b18c788})");
}
unsafe impl ::windows::core::Interface for ClaimedPosPrinterClosedEventArgs {
    type Vtable = IClaimedPosPrinterClosedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2b7a27b_4d40_471d_92ed_63375b18c788);
}
impl ::windows::core::RuntimeName for ClaimedPosPrinterClosedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedPosPrinterClosedEventArgs";
}
impl ::core::convert::From<ClaimedPosPrinterClosedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ClaimedPosPrinterClosedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ClaimedPosPrinterClosedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ClaimedPosPrinterClosedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ClaimedPosPrinterClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ClaimedPosPrinterClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ClaimedPosPrinterClosedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ClaimedPosPrinterClosedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ClaimedPosPrinterClosedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ClaimedPosPrinterClosedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ClaimedPosPrinterClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ClaimedPosPrinterClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ClaimedPosPrinterClosedEventArgs {}
unsafe impl ::core::marker::Sync for ClaimedPosPrinterClosedEventArgs {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ClaimedReceiptPrinter(pub ::windows::core::IInspectable);
impl ClaimedReceiptPrinter {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SidewaysMaxLines(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SidewaysMaxChars(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn LinesToPaperCut(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn PageSize(&self) -> ::windows::core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Size = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn PrintArea(&self) -> ::windows::core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CreateJob(&self) -> ::windows::core::Result<ReceiptPrintJob> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ReceiptPrintJob>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetCharactersPerLine(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CharactersPerLine(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetLineHeight(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn LineHeight(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetLineSpacing(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn LineSpacing(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn LineWidth(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetIsLetterQuality(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsLetterQuality(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsPaperNearEnd(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetColorCartridge(&self, value: PosPrinterColorCartridge) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ColorCartridge(&self) -> ::windows::core::Result<PosPrinterColorCartridge> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: PosPrinterColorCartridge = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterColorCartridge>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsCoverOpen(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsCartridgeRemoved(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsCartridgeEmpty(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsHeadCleaning(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsPaperEmpty(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsReadyToPrint(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ValidateData<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, data: Param0) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), data.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ClaimedReceiptPrinter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedReceiptPrinter;{9ad27a74-dd61-4ee2-9837-5b5d72d538b9})");
}
unsafe impl ::windows::core::Interface for ClaimedReceiptPrinter {
    type Vtable = IClaimedReceiptPrinter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ad27a74_dd61_4ee2_9837_5b5d72d538b9);
}
impl ::windows::core::RuntimeName for ClaimedReceiptPrinter {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedReceiptPrinter";
}
impl ::core::convert::From<ClaimedReceiptPrinter> for ::windows::core::IUnknown {
    fn from(value: ClaimedReceiptPrinter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ClaimedReceiptPrinter> for ::windows::core::IUnknown {
    fn from(value: &ClaimedReceiptPrinter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ClaimedReceiptPrinter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ClaimedReceiptPrinter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ClaimedReceiptPrinter> for ::windows::core::IInspectable {
    fn from(value: ClaimedReceiptPrinter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ClaimedReceiptPrinter> for ::windows::core::IInspectable {
    fn from(value: &ClaimedReceiptPrinter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ClaimedReceiptPrinter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ClaimedReceiptPrinter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<ClaimedReceiptPrinter> for ICommonClaimedPosPrinterStation {
    type Error = ::windows::core::Error;
    fn try_from(value: ClaimedReceiptPrinter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ClaimedReceiptPrinter> for ICommonClaimedPosPrinterStation {
    type Error = ::windows::core::Error;
    fn try_from(value: &ClaimedReceiptPrinter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICommonClaimedPosPrinterStation> for ClaimedReceiptPrinter {
    fn into_param(self) -> ::windows::core::Param<'a, ICommonClaimedPosPrinterStation> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICommonClaimedPosPrinterStation> for &ClaimedReceiptPrinter {
    fn into_param(self) -> ::windows::core::Param<'a, ICommonClaimedPosPrinterStation> {
        ::core::convert::TryInto::<ICommonClaimedPosPrinterStation>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ClaimedReceiptPrinter {}
unsafe impl ::core::marker::Sync for ClaimedReceiptPrinter {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ClaimedSlipPrinter(pub ::windows::core::IInspectable);
impl ClaimedSlipPrinter {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SidewaysMaxLines(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SidewaysMaxChars(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn MaxLines(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn LinesNearEndToEnd(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PrintSide(&self) -> ::windows::core::Result<PosPrinterPrintSide> {
        let this = self;
        unsafe {
            let mut result__: PosPrinterPrintSide = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterPrintSide>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn PageSize(&self) -> ::windows::core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Size = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn PrintArea(&self) -> ::windows::core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn OpenJaws(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CloseJaws(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn InsertSlipAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, timeout: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), timeout.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveSlipAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, timeout: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), timeout.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ChangePrintSide(&self, printside: PosPrinterPrintSide) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), printside).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CreateJob(&self) -> ::windows::core::Result<SlipPrintJob> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SlipPrintJob>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetCharactersPerLine(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CharactersPerLine(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetLineHeight(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn LineHeight(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetLineSpacing(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn LineSpacing(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn LineWidth(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetIsLetterQuality(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsLetterQuality(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsPaperNearEnd(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetColorCartridge(&self, value: PosPrinterColorCartridge) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ColorCartridge(&self) -> ::windows::core::Result<PosPrinterColorCartridge> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: PosPrinterColorCartridge = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterColorCartridge>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsCoverOpen(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsCartridgeRemoved(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsCartridgeEmpty(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsHeadCleaning(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsPaperEmpty(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsReadyToPrint(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ValidateData<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, data: Param0) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), data.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ClaimedSlipPrinter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedSlipPrinter;{bd5deff2-af90-4e8a-b77b-e3ae9ca63a7f})");
}
unsafe impl ::windows::core::Interface for ClaimedSlipPrinter {
    type Vtable = IClaimedSlipPrinter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbd5deff2_af90_4e8a_b77b_e3ae9ca63a7f);
}
impl ::windows::core::RuntimeName for ClaimedSlipPrinter {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedSlipPrinter";
}
impl ::core::convert::From<ClaimedSlipPrinter> for ::windows::core::IUnknown {
    fn from(value: ClaimedSlipPrinter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ClaimedSlipPrinter> for ::windows::core::IUnknown {
    fn from(value: &ClaimedSlipPrinter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ClaimedSlipPrinter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ClaimedSlipPrinter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ClaimedSlipPrinter> for ::windows::core::IInspectable {
    fn from(value: ClaimedSlipPrinter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ClaimedSlipPrinter> for ::windows::core::IInspectable {
    fn from(value: &ClaimedSlipPrinter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ClaimedSlipPrinter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ClaimedSlipPrinter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<ClaimedSlipPrinter> for ICommonClaimedPosPrinterStation {
    type Error = ::windows::core::Error;
    fn try_from(value: ClaimedSlipPrinter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ClaimedSlipPrinter> for ICommonClaimedPosPrinterStation {
    type Error = ::windows::core::Error;
    fn try_from(value: &ClaimedSlipPrinter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICommonClaimedPosPrinterStation> for ClaimedSlipPrinter {
    fn into_param(self) -> ::windows::core::Param<'a, ICommonClaimedPosPrinterStation> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICommonClaimedPosPrinterStation> for &ClaimedSlipPrinter {
    fn into_param(self) -> ::windows::core::Param<'a, ICommonClaimedPosPrinterStation> {
        ::core::convert::TryInto::<ICommonClaimedPosPrinterStation>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ClaimedSlipPrinter {}
unsafe impl ::core::marker::Sync for ClaimedSlipPrinter {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IBarcodeScanner(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBarcodeScanner {
    type Vtable = IBarcodeScanner_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbea33e06_b264_4f03_a9c1_45b20f01134f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScanner_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, level: UnifiedPosHealthCheckLevel, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, barcodesymbology: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, statisticscategories: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, profile: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBarcodeScanner2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBarcodeScanner2 {
    type Vtable = IBarcodeScanner2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x89215167_8cee_436d_89ab_8dfb43bb4286);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScanner2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBarcodeScannerCapabilities(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBarcodeScannerCapabilities {
    type Vtable = IBarcodeScannerCapabilities_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc60691e4_f2c8_4420_a307_b12ef6622857);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerCapabilities_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut UnifiedPosPowerReportingType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBarcodeScannerCapabilities1(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBarcodeScannerCapabilities1 {
    type Vtable = IBarcodeScannerCapabilities1_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8e5ab3e9_0e2c_472f_a1cc_ee8054b6a684);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerCapabilities1_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBarcodeScannerCapabilities2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBarcodeScannerCapabilities2 {
    type Vtable = IBarcodeScannerCapabilities2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf211cfec_e1a1_4ea8_9abc_92b1596270ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerCapabilities2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBarcodeScannerDataReceivedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBarcodeScannerDataReceivedEventArgs {
    type Vtable = IBarcodeScannerDataReceivedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4234a7e2_ed97_467d_ad2b_01e44313a929);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerDataReceivedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBarcodeScannerErrorOccurredEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBarcodeScannerErrorOccurredEventArgs {
    type Vtable = IBarcodeScannerErrorOccurredEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2cd2602f_cf3a_4002_a75a_c5ec468f0a20);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerErrorOccurredEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBarcodeScannerImagePreviewReceivedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBarcodeScannerImagePreviewReceivedEventArgs {
    type Vtable = IBarcodeScannerImagePreviewReceivedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf3b7de85_6e8b_434e_9f58_06ef26bc4baf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerImagePreviewReceivedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBarcodeScannerReport(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBarcodeScannerReport {
    type Vtable = IBarcodeScannerReport_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ce4d8b0_a489_4b96_86c4_f0bf8a37753d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerReport_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBarcodeScannerReportFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBarcodeScannerReportFactory {
    type Vtable = IBarcodeScannerReportFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa2547326_2013_457c_8963_49c15dca78ce);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerReportFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, scandatatype: u32, scandata: ::windows::core::RawPtr, scandatalabel: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBarcodeScannerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBarcodeScannerStatics {
    type Vtable = IBarcodeScannerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d115f6f_da49_41e8_8c8c_f0cb62a9c4fc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBarcodeScannerStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBarcodeScannerStatics2 {
    type Vtable = IBarcodeScannerStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb8652473_a36f_4007_b1d0_279ebe92a656);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, connectiontypes: PosConnectionTypes, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBarcodeScannerStatusUpdatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBarcodeScannerStatusUpdatedEventArgs {
    type Vtable = IBarcodeScannerStatusUpdatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x355d8586_9c43_462b_a91a_816dc97f452c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerStatusUpdatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut BarcodeScannerStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBarcodeSymbologiesStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBarcodeSymbologiesStatics {
    type Vtable = IBarcodeSymbologiesStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca8549bb_06d2_43f4_a44b_c620679fd8d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeSymbologiesStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, scandatatype: u32, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBarcodeSymbologiesStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBarcodeSymbologiesStatics2 {
    type Vtable = IBarcodeSymbologiesStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8b7518f4_99d0_40bf_9424_b91d6dd4c6e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeSymbologiesStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBarcodeSymbologyAttributes(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBarcodeSymbologyAttributes {
    type Vtable = IBarcodeSymbologyAttributes_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66413a78_ab7a_4ada_8ece_936014b2ead7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeSymbologyAttributes_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut BarcodeSymbologyDecodeLengthKind) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: BarcodeSymbologyDecodeLengthKind) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICashDrawer(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICashDrawer {
    type Vtable = ICashDrawer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f88f5c8_de54_4aee_a890_920bcbfe30fc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICashDrawer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, level: UnifiedPosHealthCheckLevel, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, statisticscategories: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICashDrawerCapabilities(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICashDrawerCapabilities {
    type Vtable = ICashDrawerCapabilities_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0bc6de0b_e8e7_4b1f_b1d1_3e501ad08247);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICashDrawerCapabilities_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut UnifiedPosPowerReportingType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICashDrawerCloseAlarm(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICashDrawerCloseAlarm {
    type Vtable = ICashDrawerCloseAlarm_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6bf88cc7_6f63_430e_ab3b_95d75ffbe87f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICashDrawerCloseAlarm_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICashDrawerEventSource(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICashDrawerEventSource {
    type Vtable = ICashDrawerEventSource_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe006e46c_f2f9_442f_8dd6_06c10a4227ba);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICashDrawerEventSource_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Devices_PointOfService`*"]
pub struct ICashDrawerEventSourceEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICashDrawerEventSourceEventArgs {
    type Vtable = ICashDrawerEventSourceEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69cb3bc1_147f_421c_9c23_090123bb786c);
}
impl ICashDrawerEventSourceEventArgs {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CashDrawer(&self) -> ::windows::core::Result<CashDrawer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CashDrawer>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ICashDrawerEventSourceEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{69cb3bc1-147f-421c-9c23-090123bb786c}");
}
impl ::core::convert::From<ICashDrawerEventSourceEventArgs> for ::windows::core::IUnknown {
    fn from(value: ICashDrawerEventSourceEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ICashDrawerEventSourceEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ICashDrawerEventSourceEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICashDrawerEventSourceEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICashDrawerEventSourceEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ICashDrawerEventSourceEventArgs> for ::windows::core::IInspectable {
    fn from(value: ICashDrawerEventSourceEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICashDrawerEventSourceEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ICashDrawerEventSourceEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ICashDrawerEventSourceEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ICashDrawerEventSourceEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICashDrawerEventSourceEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICashDrawerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICashDrawerStatics {
    type Vtable = ICashDrawerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdfa0955a_d437_4fff_b547_dda969a4f883);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICashDrawerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICashDrawerStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICashDrawerStatics2 {
    type Vtable = ICashDrawerStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3e818121_8c42_40e8_9c0e_40297048104c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICashDrawerStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, connectiontypes: PosConnectionTypes, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICashDrawerStatus(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICashDrawerStatus {
    type Vtable = ICashDrawerStatus_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6bbd78bf_dca1_4e06_99eb_5af6a5aec108);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICashDrawerStatus_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut CashDrawerStatusKind) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ICashDrawerStatusUpdatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICashDrawerStatusUpdatedEventArgs {
    type Vtable = ICashDrawerStatusUpdatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30aae98a_0d70_459c_9553_87e124c52488);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICashDrawerStatusUpdatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IClaimedBarcodeScanner(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IClaimedBarcodeScanner {
    type Vtable = IClaimedBarcodeScanner_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4a63b49c_8fa4_4332_bb26_945d11d81e0f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedBarcodeScanner_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, symbologies: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, statisticscategories: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, statistics: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, profile: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IClaimedBarcodeScanner1(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IClaimedBarcodeScanner1 {
    type Vtable = IClaimedBarcodeScanner1_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf61aad0c_8551_42b4_998c_970c20210a22);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedBarcodeScanner1_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IClaimedBarcodeScanner2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IClaimedBarcodeScanner2 {
    type Vtable = IClaimedBarcodeScanner2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3b59e8c_2d8b_4f70_8af3_3448bedd5fe2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedBarcodeScanner2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, barcodesymbology: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, barcodesymbology: u32, attributes: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IClaimedBarcodeScanner3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IClaimedBarcodeScanner3 {
    type Vtable = IClaimedBarcodeScanner3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe6ceb430_712e_45fc_8b86_cd55f5aef79d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedBarcodeScanner3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IClaimedBarcodeScanner4(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IClaimedBarcodeScanner4 {
    type Vtable = IClaimedBarcodeScanner4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d501f97_376a_41a8_a230_2f37c1949dde);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedBarcodeScanner4_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IClaimedBarcodeScannerClosedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IClaimedBarcodeScannerClosedEventArgs {
    type Vtable = IClaimedBarcodeScannerClosedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf7d5489_a22c_4c65_a901_88d77d833954);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedBarcodeScannerClosedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IClaimedCashDrawer(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IClaimedCashDrawer {
    type Vtable = IClaimedCashDrawer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca3f99af_abb8_42c1_8a84_5c66512f5a75);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedCashDrawer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, statisticscategories: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, statistics: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IClaimedCashDrawer2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IClaimedCashDrawer2 {
    type Vtable = IClaimedCashDrawer2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9cbab5a2_de42_4d5b_b0c1_9b57a2ba89c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedCashDrawer2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IClaimedCashDrawerClosedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IClaimedCashDrawerClosedEventArgs {
    type Vtable = IClaimedCashDrawerClosedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcc573f33_3f34_4c5c_baae_deadf16cd7fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedCashDrawerClosedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IClaimedJournalPrinter(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IClaimedJournalPrinter {
    type Vtable = IClaimedJournalPrinter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x67ea0630_517d_487f_9fdf_d2e0a0a264a5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedJournalPrinter_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IClaimedLineDisplay(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IClaimedLineDisplay {
    type Vtable = IClaimedLineDisplay_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x120ac970_9a75_4acf_aae7_09972bcf8794);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedLineDisplay_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IClaimedLineDisplay2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IClaimedLineDisplay2 {
    type Vtable = IClaimedLineDisplay2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa31c75ed_41f5_4e76_a074_795e47a46e97);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedLineDisplay2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, statisticscategories: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, level: UnifiedPosHealthCheckLevel, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, attributes: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, descriptor: u32, descriptorstate: LineDisplayDescriptorState, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, viewport: super::super::Foundation::Rect, windowsize: super::super::Foundation::Size, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bitmap: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bitmap: ::windows::core::RawPtr, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bitmap: ::windows::core::RawPtr, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment, widthinpixels: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IClaimedLineDisplay3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IClaimedLineDisplay3 {
    type Vtable = IClaimedLineDisplay3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x642ecd92_e9d4_4ecc_af75_329c274cd18f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedLineDisplay3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IClaimedLineDisplayClosedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IClaimedLineDisplayClosedEventArgs {
    type Vtable = IClaimedLineDisplayClosedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf915f364_d3d5_4f10_b511_90939edfacd8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedLineDisplayClosedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IClaimedLineDisplayStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IClaimedLineDisplayStatics {
    type Vtable = IClaimedLineDisplayStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x78ca98fb_8b6b_4973_86f0_3e570c351825);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedLineDisplayStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, connectiontypes: PosConnectionTypes, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IClaimedMagneticStripeReader(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IClaimedMagneticStripeReader {
    type Vtable = IClaimedMagneticStripeReader_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x475ca8f3_9417_48bc_b9d7_4163a7844c02);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedMagneticStripeReader_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: MagneticStripeReaderTrackIds) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut MagneticStripeReaderTrackIds) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: MagneticStripeReaderErrorReportingType) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, responseToken_array_size: u32, responsetoken: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, responseToken_array_size: u32, responsetoken: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, keyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, statisticscategories: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, statistics: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IClaimedMagneticStripeReader2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IClaimedMagneticStripeReader2 {
    type Vtable = IClaimedMagneticStripeReader2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x236fafdf_e2dc_4d7d_9c78_060df2bf2928);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedMagneticStripeReader2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IClaimedMagneticStripeReaderClosedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IClaimedMagneticStripeReaderClosedEventArgs {
    type Vtable = IClaimedMagneticStripeReaderClosedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x14ada93a_adcd_4c80_acda_c3eaed2647e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedMagneticStripeReaderClosedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IClaimedPosPrinter(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IClaimedPosPrinter {
    type Vtable = IClaimedPosPrinter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d64ce0c_e03e_4b14_a38e_c28c34b86353);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedPosPrinter_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: PosPrinterMapMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PosPrinterMapMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, statisticscategories: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, statistics: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IClaimedPosPrinter2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IClaimedPosPrinter2 {
    type Vtable = IClaimedPosPrinter2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5bf7a3d5_5198_437a_82df_589993fa77e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedPosPrinter2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IClaimedPosPrinterClosedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IClaimedPosPrinterClosedEventArgs {
    type Vtable = IClaimedPosPrinterClosedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2b7a27b_4d40_471d_92ed_63375b18c788);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedPosPrinterClosedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IClaimedReceiptPrinter(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IClaimedReceiptPrinter {
    type Vtable = IClaimedReceiptPrinter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ad27a74_dd61_4ee2_9837_5b5d72d538b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedReceiptPrinter_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IClaimedSlipPrinter(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IClaimedSlipPrinter {
    type Vtable = IClaimedSlipPrinter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbd5deff2_af90_4e8a_b77b_e3ae9ca63a7f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedSlipPrinter_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PosPrinterPrintSide) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, timeout: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, timeout: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, printside: PosPrinterPrintSide) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Devices_PointOfService`*"]
pub struct ICommonClaimedPosPrinterStation(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICommonClaimedPosPrinterStation {
    type Vtable = ICommonClaimedPosPrinterStation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb7eb66a8_fe8a_4cfb_8b42_e35b280cb27c);
}
impl ICommonClaimedPosPrinterStation {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetCharactersPerLine(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CharactersPerLine(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetLineHeight(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn LineHeight(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetLineSpacing(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn LineSpacing(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn LineWidth(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetIsLetterQuality(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsLetterQuality(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsPaperNearEnd(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetColorCartridge(&self, value: PosPrinterColorCartridge) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ColorCartridge(&self) -> ::windows::core::Result<PosPrinterColorCartridge> {
        let this = self;
        unsafe {
            let mut result__: PosPrinterColorCartridge = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterColorCartridge>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsCoverOpen(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsCartridgeRemoved(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsCartridgeEmpty(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsHeadCleaning(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsPaperEmpty(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsReadyToPrint(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ValidateData<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, data: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), data.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ICommonClaimedPosPrinterStation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{b7eb66a8-fe8a-4cfb-8b42-e35b280cb27c}");
}
impl ::core::convert::From<ICommonClaimedPosPrinterStation> for ::windows::core::IUnknown {
    fn from(value: ICommonClaimedPosPrinterStation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ICommonClaimedPosPrinterStation> for ::windows::core::IUnknown {
    fn from(value: &ICommonClaimedPosPrinterStation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICommonClaimedPosPrinterStation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICommonClaimedPosPrinterStation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ICommonClaimedPosPrinterStation> for ::windows::core::IInspectable {
    fn from(value: ICommonClaimedPosPrinterStation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICommonClaimedPosPrinterStation> for ::windows::core::IInspectable {
    fn from(value: &ICommonClaimedPosPrinterStation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ICommonClaimedPosPrinterStation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ICommonClaimedPosPrinterStation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommonClaimedPosPrinterStation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: PosPrinterColorCartridge) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PosPrinterColorCartridge) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Devices_PointOfService`*"]
pub struct ICommonPosPrintStationCapabilities(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICommonPosPrintStationCapabilities {
    type Vtable = ICommonPosPrintStationCapabilities_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde5b52ca_e02e_40e9_9e5e_1b488e6aacfc);
}
impl ICommonPosPrintStationCapabilities {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsPrinterPresent(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDualColorSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ColorCartridgeCapabilities(&self) -> ::windows::core::Result<PosPrinterColorCapabilities> {
        let this = self;
        unsafe {
            let mut result__: PosPrinterColorCapabilities = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterColorCapabilities>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CartridgeSensors(&self) -> ::windows::core::Result<PosPrinterCartridgeSensors> {
        let this = self;
        unsafe {
            let mut result__: PosPrinterCartridgeSensors = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterCartridgeSensors>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsBoldSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsItalicSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsUnderlineSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDoubleHighPrintSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDoubleWidePrintSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDoubleHighDoubleWidePrintSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsPaperEmptySensorSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsPaperNearEndSensorSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation_Collections`*"]
    pub fn SupportedCharactersPerLine(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ICommonPosPrintStationCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{de5b52ca-e02e-40e9-9e5e-1b488e6aacfc}");
}
impl ::core::convert::From<ICommonPosPrintStationCapabilities> for ::windows::core::IUnknown {
    fn from(value: ICommonPosPrintStationCapabilities) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ICommonPosPrintStationCapabilities> for ::windows::core::IUnknown {
    fn from(value: &ICommonPosPrintStationCapabilities) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICommonPosPrintStationCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICommonPosPrintStationCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ICommonPosPrintStationCapabilities> for ::windows::core::IInspectable {
    fn from(value: ICommonPosPrintStationCapabilities) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICommonPosPrintStationCapabilities> for ::windows::core::IInspectable {
    fn from(value: &ICommonPosPrintStationCapabilities) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ICommonPosPrintStationCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ICommonPosPrintStationCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommonPosPrintStationCapabilities_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PosPrinterColorCapabilities) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PosPrinterCartridgeSensors) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Devices_PointOfService`*"]
pub struct ICommonReceiptSlipCapabilities(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ICommonReceiptSlipCapabilities {
    type Vtable = ICommonReceiptSlipCapabilities_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09286b8b_9873_4d05_bfbe_4727a6038f69);
}
impl ICommonReceiptSlipCapabilities {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsBarcodeSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsBitmapSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsLeft90RotationSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsRight90RotationSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Is180RotationSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsPrintAreaSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn RuledLineCapabilities(&self) -> ::windows::core::Result<PosPrinterRuledLineCapabilities> {
        let this = self;
        unsafe {
            let mut result__: PosPrinterRuledLineCapabilities = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterRuledLineCapabilities>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation_Collections`*"]
    pub fn SupportedBarcodeRotations(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation_Collections`*"]
    pub fn SupportedBitmapRotations(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsPrinterPresent(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDualColorSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ColorCartridgeCapabilities(&self) -> ::windows::core::Result<PosPrinterColorCapabilities> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: PosPrinterColorCapabilities = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterColorCapabilities>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CartridgeSensors(&self) -> ::windows::core::Result<PosPrinterCartridgeSensors> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: PosPrinterCartridgeSensors = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterCartridgeSensors>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsBoldSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsItalicSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsUnderlineSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDoubleHighPrintSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDoubleWidePrintSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDoubleHighDoubleWidePrintSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsPaperEmptySensorSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsPaperNearEndSensorSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation_Collections`*"]
    pub fn SupportedCharactersPerLine(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ICommonReceiptSlipCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{09286b8b-9873-4d05-bfbe-4727a6038f69}");
}
impl ::core::convert::From<ICommonReceiptSlipCapabilities> for ::windows::core::IUnknown {
    fn from(value: ICommonReceiptSlipCapabilities) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ICommonReceiptSlipCapabilities> for ::windows::core::IUnknown {
    fn from(value: &ICommonReceiptSlipCapabilities) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICommonReceiptSlipCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICommonReceiptSlipCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ICommonReceiptSlipCapabilities> for ::windows::core::IInspectable {
    fn from(value: ICommonReceiptSlipCapabilities) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICommonReceiptSlipCapabilities> for ::windows::core::IInspectable {
    fn from(value: &ICommonReceiptSlipCapabilities) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ICommonReceiptSlipCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ICommonReceiptSlipCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<ICommonReceiptSlipCapabilities> for ICommonPosPrintStationCapabilities {
    type Error = ::windows::core::Error;
    fn try_from(value: ICommonReceiptSlipCapabilities) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ICommonReceiptSlipCapabilities> for ICommonPosPrintStationCapabilities {
    type Error = ::windows::core::Error;
    fn try_from(value: &ICommonReceiptSlipCapabilities) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICommonPosPrintStationCapabilities> for ICommonReceiptSlipCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ICommonPosPrintStationCapabilities> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICommonPosPrintStationCapabilities> for &ICommonReceiptSlipCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ICommonPosPrintStationCapabilities> {
        ::core::convert::TryInto::<ICommonPosPrintStationCapabilities>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommonReceiptSlipCapabilities_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PosPrinterRuledLineCapabilities) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IJournalPrintJob(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IJournalPrintJob {
    type Vtable = IJournalPrintJob_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f4f2864_f3f0_55d0_8c39_74cc91783eed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJournalPrintJob_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, printoptions: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, linecount: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, distance: i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IJournalPrinterCapabilities(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IJournalPrinterCapabilities {
    type Vtable = IJournalPrinterCapabilities_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b5ccc43_e047_4463_bb58_17b5ba1d8056);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJournalPrinterCapabilities_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IJournalPrinterCapabilities2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IJournalPrinterCapabilities2 {
    type Vtable = IJournalPrinterCapabilities2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03b0b645_33b8_533b_baaa_a4389283ab0a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJournalPrinterCapabilities2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILineDisplay(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILineDisplay {
    type Vtable = ILineDisplay_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24f5df4e_3c99_44e2_b73f_e51be3637a8c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplay_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILineDisplay2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILineDisplay2 {
    type Vtable = ILineDisplay2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc296a628_ef44_40f3_bd1c_b04c6a5cdc7d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplay2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILineDisplayAttributes(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILineDisplayAttributes {
    type Vtable = ILineDisplayAttributes_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc17de99c_229a_4c14_a6f1_b4e4b1fead92);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayAttributes_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILineDisplayCapabilities(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILineDisplayCapabilities {
    type Vtable = ILineDisplayCapabilities_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5a15b5d1_8dc5_4b9c_9172_303e47b70c55);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayCapabilities_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut UnifiedPosPowerReportingType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut LineDisplayTextAttributeGranularity) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut LineDisplayTextAttributeGranularity) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILineDisplayCursor(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILineDisplayCursor {
    type Vtable = ILineDisplayCursor_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecdffc45_754a_4e3b_ab2b_151181085605);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayCursor_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, attributes: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILineDisplayCursorAttributes(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILineDisplayCursorAttributes {
    type Vtable = ILineDisplayCursorAttributes_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e2d54fe_4ffd_4190_aae1_ce285f20c896);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayCursorAttributes_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut LineDisplayCursorType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: LineDisplayCursorType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILineDisplayCustomGlyphs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILineDisplayCustomGlyphs {
    type Vtable = ILineDisplayCustomGlyphs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2257f63c_f263_44f1_a1a0_e750a6a0ec54);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayCustomGlyphs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, glyphcode: u32, glyphdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILineDisplayMarquee(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILineDisplayMarquee {
    type Vtable = ILineDisplayMarquee_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa3d33e3e_f46a_4b7a_bc21_53eb3b57f8b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayMarquee_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut LineDisplayMarqueeFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: LineDisplayMarqueeFormat) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, direction: LineDisplayScrollDirection, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILineDisplayStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILineDisplayStatics {
    type Vtable = ILineDisplayStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x022dc0b6_11b0_4690_9547_0b39c5af2114);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, connectiontypes: PosConnectionTypes, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILineDisplayStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILineDisplayStatics2 {
    type Vtable = ILineDisplayStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x600c3f1c_77ab_4968_a7de_c02ff169f2cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILineDisplayStatisticsCategorySelector(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILineDisplayStatisticsCategorySelector {
    type Vtable = ILineDisplayStatisticsCategorySelector_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb521c46b_9274_4d24_94f3_b6017b832444);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayStatisticsCategorySelector_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILineDisplayStatusUpdatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILineDisplayStatusUpdatedEventArgs {
    type Vtable = ILineDisplayStatusUpdatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xddd57c1a_86fb_4eba_93d1_6f5eda52b752);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayStatusUpdatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut LineDisplayPowerStatus) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILineDisplayStoredBitmap(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILineDisplayStoredBitmap {
    type Vtable = ILineDisplayStoredBitmap_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf621515b_d81e_43ba_bf1b_bcfa3c785ba0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayStoredBitmap_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILineDisplayWindow(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILineDisplayWindow {
    type Vtable = ILineDisplayWindow_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd21feef4_2364_4be5_bee1_851680af4964);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayWindow_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayattribute: LineDisplayTextAttribute, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayattribute: LineDisplayTextAttribute, startposition: super::super::Foundation::Point, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, direction: LineDisplayScrollDirection, numberofcolumnsorrows: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILineDisplayWindow2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ILineDisplayWindow2 {
    type Vtable = ILineDisplayWindow2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa95ce2e6_bdd8_4365_8e11_de94de8dff02);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayWindow2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bitmap: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bitmap: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bitmap: ::windows::core::RawPtr, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bitmap: ::windows::core::RawPtr, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment, widthinpixels: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bitmap: ::windows::core::RawPtr, offsetinpixels: super::super::Foundation::Point, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bitmap: ::windows::core::RawPtr, offsetinpixels: super::super::Foundation::Point, widthinpixels: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMagneticStripeReader(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMagneticStripeReader {
    type Vtable = IMagneticStripeReader_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a92b015_47c3_468a_9333_0c6517574883);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReader_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result_size__: *mut u32, result__: *mut *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut MagneticStripeReaderAuthenticationProtocol) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, level: UnifiedPosHealthCheckLevel, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, statisticscategories: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut MagneticStripeReaderErrorReportingType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMagneticStripeReaderAamvaCardDataReceivedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMagneticStripeReaderAamvaCardDataReceivedEventArgs {
    type Vtable = IMagneticStripeReaderAamvaCardDataReceivedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a4bbd51_c316_4910_87f3_7a62ba862d31);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderAamvaCardDataReceivedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMagneticStripeReaderBankCardDataReceivedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMagneticStripeReaderBankCardDataReceivedEventArgs {
    type Vtable = IMagneticStripeReaderBankCardDataReceivedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e958823_a31a_4763_882c_23725e39b08e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderBankCardDataReceivedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMagneticStripeReaderCapabilities(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMagneticStripeReaderCapabilities {
    type Vtable = IMagneticStripeReaderCapabilities_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7128809c_c440_44a2_a467_469175d02896);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderCapabilities_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut MagneticStripeReaderAuthenticationLevel) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut UnifiedPosPowerReportingType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMagneticStripeReaderCardTypesStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMagneticStripeReaderCardTypesStatics {
    type Vtable = IMagneticStripeReaderCardTypesStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x528f2c5d_2986_474f_8454_7ccd05928d5f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderCardTypesStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMagneticStripeReaderEncryptionAlgorithmsStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMagneticStripeReaderEncryptionAlgorithmsStatics {
    type Vtable = IMagneticStripeReaderEncryptionAlgorithmsStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53b57350_c3db_4754_9c00_41392374a109);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderEncryptionAlgorithmsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMagneticStripeReaderErrorOccurredEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMagneticStripeReaderErrorOccurredEventArgs {
    type Vtable = IMagneticStripeReaderErrorOccurredEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1fedf95d_2c84_41ad_b778_f2356a789ab1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderErrorOccurredEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut MagneticStripeReaderTrackErrorType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut MagneticStripeReaderTrackErrorType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut MagneticStripeReaderTrackErrorType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut MagneticStripeReaderTrackErrorType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMagneticStripeReaderReport(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMagneticStripeReaderReport {
    type Vtable = IMagneticStripeReaderReport_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a5b6047_99b0_4188_bef1_eddf79f78fe6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderReport_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMagneticStripeReaderStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMagneticStripeReaderStatics {
    type Vtable = IMagneticStripeReaderStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc45fab4a_efd7_4760_a5ce_15b0e47e94eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMagneticStripeReaderStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMagneticStripeReaderStatics2 {
    type Vtable = IMagneticStripeReaderStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8cadc362_d667_48fa_86bc_f5ae1189262b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, connectiontypes: PosConnectionTypes, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMagneticStripeReaderStatusUpdatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMagneticStripeReaderStatusUpdatedEventArgs {
    type Vtable = IMagneticStripeReaderStatusUpdatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09cc6bb0_3262_401d_9e8a_e80d6358906b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderStatusUpdatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut MagneticStripeReaderStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMagneticStripeReaderTrackData(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMagneticStripeReaderTrackData {
    type Vtable = IMagneticStripeReaderTrackData_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x104cf671_4a9d_446e_abc5_20402307ba36);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderTrackData_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    type Vtable = IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf0a5514_59cc_4a60_99e8_99a53dace5aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPosPrinter(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPosPrinter {
    type Vtable = IPosPrinter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a03c10e_9a19_4a01_994f_12dfad6adcbf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinter_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, level: UnifiedPosHealthCheckLevel, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, statisticscategories: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPosPrinter2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPosPrinter2 {
    type Vtable = IPosPrinter2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x248475e8_8b98_5517_8e48_760e86f68987);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinter2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, typeface: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPosPrinterCapabilities(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPosPrinterCapabilities {
    type Vtable = IPosPrinterCapabilities_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcde95721_4380_4985_adc5_39db30cd93bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinterCapabilities_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut UnifiedPosPowerReportingType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPosPrinterCharacterSetIdsStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPosPrinterCharacterSetIdsStatics {
    type Vtable = IPosPrinterCharacterSetIdsStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c709eff_709a_4fe7_b215_06a748a38b39);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinterCharacterSetIdsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPosPrinterFontProperty(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPosPrinterFontProperty {
    type Vtable = IPosPrinterFontProperty_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7f4e93a_f8ac_5f04_84d2_29b16d8a633c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinterFontProperty_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Devices_PointOfService`*"]
pub struct IPosPrinterJob(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPosPrinterJob {
    type Vtable = IPosPrinterJob_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a94005c_0615_4591_a58f_30f87edfe2e4);
}
impl IPosPrinterJob {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Print<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, data: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PrintLine<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, data: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PrintNewline(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn ExecuteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IPosPrinterJob {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{9a94005c-0615-4591-a58f-30f87edfe2e4}");
}
impl ::core::convert::From<IPosPrinterJob> for ::windows::core::IUnknown {
    fn from(value: IPosPrinterJob) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IPosPrinterJob> for ::windows::core::IUnknown {
    fn from(value: &IPosPrinterJob) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPosPrinterJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPosPrinterJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IPosPrinterJob> for ::windows::core::IInspectable {
    fn from(value: IPosPrinterJob) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPosPrinterJob> for ::windows::core::IInspectable {
    fn from(value: &IPosPrinterJob) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IPosPrinterJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IPosPrinterJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinterJob_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPosPrinterPrintOptions(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPosPrinterPrintOptions {
    type Vtable = IPosPrinterPrintOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a2e16fd_1d02_5a58_9d59_bfcde76fde86);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinterPrintOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PosPrinterAlignment) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: PosPrinterAlignment) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPosPrinterReleaseDeviceRequestedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPosPrinterReleaseDeviceRequestedEventArgs {
    type Vtable = IPosPrinterReleaseDeviceRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2bcba359_1cef_40b2_9ecb_f927f856ae3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinterReleaseDeviceRequestedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPosPrinterStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPosPrinterStatics {
    type Vtable = IPosPrinterStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8ce0d4ea_132f_4cdf_a64a_2d0d7c96a85b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinterStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPosPrinterStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPosPrinterStatics2 {
    type Vtable = IPosPrinterStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeecd2c1c_b0d0_42e7_b137_b89b16244d41);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinterStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, connectiontypes: PosConnectionTypes, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPosPrinterStatus(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPosPrinterStatus {
    type Vtable = IPosPrinterStatus_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd1f0c730_da40_4328_bf76_5156fa33b747);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinterStatus_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PosPrinterStatusKind) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPosPrinterStatusUpdatedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPosPrinterStatusUpdatedEventArgs {
    type Vtable = IPosPrinterStatusUpdatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2edb87df_13a6_428d_ba81_b0e7c3e5a3cd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinterStatusUpdatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Devices_PointOfService`*"]
pub struct IReceiptOrSlipJob(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IReceiptOrSlipJob {
    type Vtable = IReceiptOrSlipJob_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x532199be_c8c3_4dc2_89e9_5c4a37b34ddc);
}
impl IReceiptOrSlipJob {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetBarcodeRotation(&self, value: PosPrinterRotation) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetPrintRotation(&self, value: PosPrinterRotation, includebitmaps: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value, includebitmaps).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn SetPrintArea<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Devices_PointOfService`, `Graphics_Imaging`*"]
    pub fn SetBitmap<'a, Param1: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmapnumber: u32, bitmap: Param1, alignment: PosPrinterAlignment) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), bitmapnumber, bitmap.into_param().abi(), alignment).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Devices_PointOfService`, `Graphics_Imaging`*"]
    pub fn SetBitmapCustomWidthStandardAlign<'a, Param1: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmapnumber: u32, bitmap: Param1, alignment: PosPrinterAlignment, width: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), bitmapnumber, bitmap.into_param().abi(), alignment, width).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Devices_PointOfService`, `Graphics_Imaging`*"]
    pub fn SetCustomAlignedBitmap<'a, Param1: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmapnumber: u32, bitmap: Param1, alignmentdistance: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), bitmapnumber, bitmap.into_param().abi(), alignmentdistance).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Devices_PointOfService`, `Graphics_Imaging`*"]
    pub fn SetBitmapCustomWidthCustomAlign<'a, Param1: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmapnumber: u32, bitmap: Param1, alignmentdistance: u32, width: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), bitmapnumber, bitmap.into_param().abi(), alignmentdistance, width).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PrintSavedBitmap(&self, bitmapnumber: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), bitmapnumber).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DrawRuledLine<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, positionlist: Param0, linedirection: PosPrinterLineDirection, linewidth: u32, linestyle: PosPrinterLineStyle, linecolor: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), positionlist.into_param().abi(), linedirection, linewidth, linestyle, linecolor).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PrintBarcode<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, data: Param0, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignment: PosPrinterAlignment) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), data.into_param().abi(), symbology, height, width, textposition, alignment).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PrintBarcodeCustomAlign<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, data: Param0, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignmentdistance: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), data.into_param().abi(), symbology, height, width, textposition, alignmentdistance).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Devices_PointOfService`, `Graphics_Imaging`*"]
    pub fn PrintBitmap<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmap: Param0, alignment: PosPrinterAlignment) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), bitmap.into_param().abi(), alignment).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Devices_PointOfService`, `Graphics_Imaging`*"]
    pub fn PrintBitmapCustomWidthStandardAlign<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmap: Param0, alignment: PosPrinterAlignment, width: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), bitmap.into_param().abi(), alignment, width).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Devices_PointOfService`, `Graphics_Imaging`*"]
    pub fn PrintCustomAlignedBitmap<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmap: Param0, alignmentdistance: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), bitmap.into_param().abi(), alignmentdistance).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Devices_PointOfService`, `Graphics_Imaging`*"]
    pub fn PrintBitmapCustomWidthCustomAlign<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmap: Param0, alignmentdistance: u32, width: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), bitmap.into_param().abi(), alignmentdistance, width).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Print<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, data: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PrintLine<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, data: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PrintNewline(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn ExecuteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IReceiptOrSlipJob {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{532199be-c8c3-4dc2-89e9-5c4a37b34ddc}");
}
impl ::core::convert::From<IReceiptOrSlipJob> for ::windows::core::IUnknown {
    fn from(value: IReceiptOrSlipJob) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IReceiptOrSlipJob> for ::windows::core::IUnknown {
    fn from(value: &IReceiptOrSlipJob) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IReceiptOrSlipJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IReceiptOrSlipJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IReceiptOrSlipJob> for ::windows::core::IInspectable {
    fn from(value: IReceiptOrSlipJob) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IReceiptOrSlipJob> for ::windows::core::IInspectable {
    fn from(value: &IReceiptOrSlipJob) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IReceiptOrSlipJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IReceiptOrSlipJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IReceiptOrSlipJob> for IPosPrinterJob {
    type Error = ::windows::core::Error;
    fn try_from(value: IReceiptOrSlipJob) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IReceiptOrSlipJob> for IPosPrinterJob {
    type Error = ::windows::core::Error;
    fn try_from(value: &IReceiptOrSlipJob) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPosPrinterJob> for IReceiptOrSlipJob {
    fn into_param(self) -> ::windows::core::Param<'a, IPosPrinterJob> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPosPrinterJob> for &IReceiptOrSlipJob {
    fn into_param(self) -> ::windows::core::Param<'a, IPosPrinterJob> {
        ::core::convert::TryInto::<IPosPrinterJob>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IReceiptOrSlipJob_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: PosPrinterRotation) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: PosPrinterRotation, includebitmaps: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bitmapnumber: u32, bitmap: ::windows::core::RawPtr, alignment: PosPrinterAlignment) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))] usize,
    #[cfg(feature = "Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bitmapnumber: u32, bitmap: ::windows::core::RawPtr, alignment: PosPrinterAlignment, width: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))] usize,
    #[cfg(feature = "Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bitmapnumber: u32, bitmap: ::windows::core::RawPtr, alignmentdistance: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))] usize,
    #[cfg(feature = "Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bitmapnumber: u32, bitmap: ::windows::core::RawPtr, alignmentdistance: u32, width: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bitmapnumber: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, positionlist: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, linedirection: PosPrinterLineDirection, linewidth: u32, linestyle: PosPrinterLineStyle, linecolor: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignment: PosPrinterAlignment) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignmentdistance: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bitmap: ::windows::core::RawPtr, alignment: PosPrinterAlignment) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))] usize,
    #[cfg(feature = "Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bitmap: ::windows::core::RawPtr, alignment: PosPrinterAlignment, width: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))] usize,
    #[cfg(feature = "Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bitmap: ::windows::core::RawPtr, alignmentdistance: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))] usize,
    #[cfg(feature = "Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bitmap: ::windows::core::RawPtr, alignmentdistance: u32, width: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IReceiptPrintJob(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IReceiptPrintJob {
    type Vtable = IReceiptPrintJob_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa96066e_acad_4b79_9d0f_c0cfc08dc77b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReceiptPrintJob_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, kind: PosPrinterMarkFeedKind) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, percentage: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IReceiptPrintJob2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IReceiptPrintJob2 {
    type Vtable = IReceiptPrintJob2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cbc12e3_9e29_5179_bcd8_1811d3b9a10e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReceiptPrintJob2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, printoptions: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, linecount: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, distance: i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IReceiptPrinterCapabilities(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IReceiptPrinterCapabilities {
    type Vtable = IReceiptPrinterCapabilities_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb8f0b58f_51a8_43fc_9bd5_8de272a6415b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReceiptPrinterCapabilities_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PosPrinterMarkFeedCapabilities) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IReceiptPrinterCapabilities2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IReceiptPrinterCapabilities2 {
    type Vtable = IReceiptPrinterCapabilities2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20030638_8a2c_55ac_9a7b_7576d8869e99);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReceiptPrinterCapabilities2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISlipPrintJob(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISlipPrintJob {
    type Vtable = ISlipPrintJob_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d88f95d_6131_5a4b_b7d5_8ef2da7b4165);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISlipPrintJob_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, printoptions: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, linecount: i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, distance: i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISlipPrinterCapabilities(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISlipPrinterCapabilities {
    type Vtable = ISlipPrinterCapabilities_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x99b16399_488c_4157_8ac2_9f57f708d3db);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISlipPrinterCapabilities_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISlipPrinterCapabilities2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISlipPrinterCapabilities2 {
    type Vtable = ISlipPrinterCapabilities2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6ff89671_2d1a_5000_87c2_b0851bfdf07e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISlipPrinterCapabilities2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUnifiedPosErrorData(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUnifiedPosErrorData {
    type Vtable = IUnifiedPosErrorData_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2b998c3a_555c_4889_8ed8_c599bb3a712a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnifiedPosErrorData_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut UnifiedPosErrorSeverity) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut UnifiedPosErrorReason) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUnifiedPosErrorDataFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUnifiedPosErrorDataFactory {
    type Vtable = IUnifiedPosErrorDataFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b982551_1ffe_451b_a368_63e0ce465f5a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnifiedPosErrorDataFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, severity: UnifiedPosErrorSeverity, reason: UnifiedPosErrorReason, extendedreason: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct JournalPrintJob(pub ::windows::core::IInspectable);
impl JournalPrintJob {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Print<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, data: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PrintLine<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, data: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PrintNewline(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn ExecuteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Print2<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, PosPrinterPrintOptions>>(&self, data: Param0, printoptions: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IJournalPrintJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), data.into_param().abi(), printoptions.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn FeedPaperByLine(&self, linecount: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IJournalPrintJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), linecount).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn FeedPaperByMapModeUnit(&self, distance: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IJournalPrintJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), distance).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for JournalPrintJob {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.JournalPrintJob;{9a94005c-0615-4591-a58f-30f87edfe2e4})");
}
unsafe impl ::windows::core::Interface for JournalPrintJob {
    type Vtable = IPosPrinterJob_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a94005c_0615_4591_a58f_30f87edfe2e4);
}
impl ::windows::core::RuntimeName for JournalPrintJob {
    const NAME: &'static str = "Windows.Devices.PointOfService.JournalPrintJob";
}
impl ::core::convert::From<JournalPrintJob> for ::windows::core::IUnknown {
    fn from(value: JournalPrintJob) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&JournalPrintJob> for ::windows::core::IUnknown {
    fn from(value: &JournalPrintJob) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for JournalPrintJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a JournalPrintJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<JournalPrintJob> for ::windows::core::IInspectable {
    fn from(value: JournalPrintJob) -> Self {
        value.0
    }
}
impl ::core::convert::From<&JournalPrintJob> for ::windows::core::IInspectable {
    fn from(value: &JournalPrintJob) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for JournalPrintJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a JournalPrintJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<JournalPrintJob> for IPosPrinterJob {
    fn from(value: JournalPrintJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&JournalPrintJob> for IPosPrinterJob {
    fn from(value: &JournalPrintJob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPosPrinterJob> for JournalPrintJob {
    fn into_param(self) -> ::windows::core::Param<'a, IPosPrinterJob> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPosPrinterJob> for &JournalPrintJob {
    fn into_param(self) -> ::windows::core::Param<'a, IPosPrinterJob> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for JournalPrintJob {}
unsafe impl ::core::marker::Sync for JournalPrintJob {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct JournalPrinterCapabilities(pub ::windows::core::IInspectable);
impl JournalPrinterCapabilities {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsPrinterPresent(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDualColorSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ColorCartridgeCapabilities(&self) -> ::windows::core::Result<PosPrinterColorCapabilities> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: PosPrinterColorCapabilities = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterColorCapabilities>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CartridgeSensors(&self) -> ::windows::core::Result<PosPrinterCartridgeSensors> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: PosPrinterCartridgeSensors = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterCartridgeSensors>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsBoldSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsItalicSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsUnderlineSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDoubleHighPrintSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDoubleWidePrintSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDoubleHighDoubleWidePrintSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsPaperEmptySensorSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsPaperNearEndSensorSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation_Collections`*"]
    pub fn SupportedCharactersPerLine(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsReverseVideoSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IJournalPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsStrikethroughSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IJournalPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsSuperscriptSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IJournalPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsSubscriptSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IJournalPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsReversePaperFeedByLineSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IJournalPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsReversePaperFeedByMapModeUnitSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IJournalPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for JournalPrinterCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.JournalPrinterCapabilities;{3b5ccc43-e047-4463-bb58-17b5ba1d8056})");
}
unsafe impl ::windows::core::Interface for JournalPrinterCapabilities {
    type Vtable = IJournalPrinterCapabilities_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b5ccc43_e047_4463_bb58_17b5ba1d8056);
}
impl ::windows::core::RuntimeName for JournalPrinterCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.JournalPrinterCapabilities";
}
impl ::core::convert::From<JournalPrinterCapabilities> for ::windows::core::IUnknown {
    fn from(value: JournalPrinterCapabilities) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&JournalPrinterCapabilities> for ::windows::core::IUnknown {
    fn from(value: &JournalPrinterCapabilities) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for JournalPrinterCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a JournalPrinterCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<JournalPrinterCapabilities> for ::windows::core::IInspectable {
    fn from(value: JournalPrinterCapabilities) -> Self {
        value.0
    }
}
impl ::core::convert::From<&JournalPrinterCapabilities> for ::windows::core::IInspectable {
    fn from(value: &JournalPrinterCapabilities) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for JournalPrinterCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a JournalPrinterCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<JournalPrinterCapabilities> for ICommonPosPrintStationCapabilities {
    type Error = ::windows::core::Error;
    fn try_from(value: JournalPrinterCapabilities) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&JournalPrinterCapabilities> for ICommonPosPrintStationCapabilities {
    type Error = ::windows::core::Error;
    fn try_from(value: &JournalPrinterCapabilities) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICommonPosPrintStationCapabilities> for JournalPrinterCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ICommonPosPrintStationCapabilities> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICommonPosPrintStationCapabilities> for &JournalPrinterCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ICommonPosPrintStationCapabilities> {
        ::core::convert::TryInto::<ICommonPosPrintStationCapabilities>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for JournalPrinterCapabilities {}
unsafe impl ::core::marker::Sync for JournalPrinterCapabilities {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LineDisplay(pub ::windows::core::IInspectable);
impl LineDisplay {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Capabilities(&self) -> ::windows::core::Result<LineDisplayCapabilities> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LineDisplayCapabilities>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PhysicalDeviceName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PhysicalDeviceDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DeviceControlDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DeviceControlVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DeviceServiceVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn ClaimAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ClaimedLineDisplay>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ClaimedLineDisplay>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LineDisplay>> {
        Self::ILineDisplayStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LineDisplay>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn GetDefaultAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LineDisplay>> {
        Self::ILineDisplayStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LineDisplay>>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ILineDisplayStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn GetDeviceSelectorWithConnectionTypes(connectiontypes: PosConnectionTypes) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ILineDisplayStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), connectiontypes, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn CheckPowerStatusAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LineDisplayPowerStatus>> {
        let this = &::windows::core::Interface::cast::<ILineDisplay2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LineDisplayPowerStatus>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn StatisticsCategorySelector() -> ::windows::core::Result<LineDisplayStatisticsCategorySelector> {
        Self::ILineDisplayStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LineDisplayStatisticsCategorySelector>(result__)
        })
    }
    pub fn ILineDisplayStatics<R, F: FnOnce(&ILineDisplayStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LineDisplay, ILineDisplayStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ILineDisplayStatics2<R, F: FnOnce(&ILineDisplayStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LineDisplay, ILineDisplayStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for LineDisplay {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplay;{24f5df4e-3c99-44e2-b73f-e51be3637a8c})");
}
unsafe impl ::windows::core::Interface for LineDisplay {
    type Vtable = ILineDisplay_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24f5df4e_3c99_44e2_b73f_e51be3637a8c);
}
impl ::windows::core::RuntimeName for LineDisplay {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplay";
}
impl ::core::convert::From<LineDisplay> for ::windows::core::IUnknown {
    fn from(value: LineDisplay) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LineDisplay> for ::windows::core::IUnknown {
    fn from(value: &LineDisplay) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LineDisplay {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LineDisplay {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LineDisplay> for ::windows::core::IInspectable {
    fn from(value: LineDisplay) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LineDisplay> for ::windows::core::IInspectable {
    fn from(value: &LineDisplay) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LineDisplay {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LineDisplay {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<LineDisplay> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: LineDisplay) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&LineDisplay> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &LineDisplay) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for LineDisplay {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &LineDisplay {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for LineDisplay {}
unsafe impl ::core::marker::Sync for LineDisplay {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LineDisplayAttributes(pub ::windows::core::IInspectable);
impl LineDisplayAttributes {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsPowerNotifyEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetIsPowerNotifyEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Brightness(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetBrightness(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn BlinkRate(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn SetBlinkRate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn ScreenSizeInCharacters(&self) -> ::windows::core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Size = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn SetScreenSizeInCharacters<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Size>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CharacterSet(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetCharacterSet(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsCharacterSetMappingEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetIsCharacterSetMappingEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CurrentWindow(&self) -> ::windows::core::Result<LineDisplayWindow> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LineDisplayWindow>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetCurrentWindow<'a, Param0: ::windows::core::IntoParam<'a, LineDisplayWindow>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for LineDisplayAttributes {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplayAttributes;{c17de99c-229a-4c14-a6f1-b4e4b1fead92})");
}
unsafe impl ::windows::core::Interface for LineDisplayAttributes {
    type Vtable = ILineDisplayAttributes_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc17de99c_229a_4c14_a6f1_b4e4b1fead92);
}
impl ::windows::core::RuntimeName for LineDisplayAttributes {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayAttributes";
}
impl ::core::convert::From<LineDisplayAttributes> for ::windows::core::IUnknown {
    fn from(value: LineDisplayAttributes) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LineDisplayAttributes> for ::windows::core::IUnknown {
    fn from(value: &LineDisplayAttributes) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LineDisplayAttributes {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LineDisplayAttributes {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LineDisplayAttributes> for ::windows::core::IInspectable {
    fn from(value: LineDisplayAttributes) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LineDisplayAttributes> for ::windows::core::IInspectable {
    fn from(value: &LineDisplayAttributes) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LineDisplayAttributes {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LineDisplayAttributes {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for LineDisplayAttributes {}
unsafe impl ::core::marker::Sync for LineDisplayAttributes {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LineDisplayCapabilities(pub ::windows::core::IInspectable);
impl LineDisplayCapabilities {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsStatisticsReportingSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsStatisticsUpdatingSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PowerReportingType(&self) -> ::windows::core::Result<UnifiedPosPowerReportingType> {
        let this = self;
        unsafe {
            let mut result__: UnifiedPosPowerReportingType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UnifiedPosPowerReportingType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CanChangeScreenSize(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CanDisplayBitmaps(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CanReadCharacterAtCursor(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CanMapCharacterSets(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CanDisplayCustomGlyphs(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CanReverse(&self) -> ::windows::core::Result<LineDisplayTextAttributeGranularity> {
        let this = self;
        unsafe {
            let mut result__: LineDisplayTextAttributeGranularity = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LineDisplayTextAttributeGranularity>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CanBlink(&self) -> ::windows::core::Result<LineDisplayTextAttributeGranularity> {
        let this = self;
        unsafe {
            let mut result__: LineDisplayTextAttributeGranularity = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LineDisplayTextAttributeGranularity>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CanChangeBlinkRate(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsBrightnessSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsCursorSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsHorizontalMarqueeSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsVerticalMarqueeSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsInterCharacterWaitSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SupportedDescriptors(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SupportedWindows(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for LineDisplayCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplayCapabilities;{5a15b5d1-8dc5-4b9c-9172-303e47b70c55})");
}
unsafe impl ::windows::core::Interface for LineDisplayCapabilities {
    type Vtable = ILineDisplayCapabilities_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5a15b5d1_8dc5_4b9c_9172_303e47b70c55);
}
impl ::windows::core::RuntimeName for LineDisplayCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayCapabilities";
}
impl ::core::convert::From<LineDisplayCapabilities> for ::windows::core::IUnknown {
    fn from(value: LineDisplayCapabilities) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LineDisplayCapabilities> for ::windows::core::IUnknown {
    fn from(value: &LineDisplayCapabilities) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LineDisplayCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LineDisplayCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LineDisplayCapabilities> for ::windows::core::IInspectable {
    fn from(value: LineDisplayCapabilities) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LineDisplayCapabilities> for ::windows::core::IInspectable {
    fn from(value: &LineDisplayCapabilities) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LineDisplayCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LineDisplayCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for LineDisplayCapabilities {}
unsafe impl ::core::marker::Sync for LineDisplayCapabilities {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LineDisplayCursor(pub ::windows::core::IInspectable);
impl LineDisplayCursor {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CanCustomize(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsBlinkSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsBlockSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsHalfBlockSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsUnderlineSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsReverseSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsOtherSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn GetAttributes(&self) -> ::windows::core::Result<LineDisplayCursorAttributes> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LineDisplayCursorAttributes>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn TryUpdateAttributesAsync<'a, Param0: ::windows::core::IntoParam<'a, LineDisplayCursorAttributes>>(&self, attributes: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), attributes.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for LineDisplayCursor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplayCursor;{ecdffc45-754a-4e3b-ab2b-151181085605})");
}
unsafe impl ::windows::core::Interface for LineDisplayCursor {
    type Vtable = ILineDisplayCursor_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecdffc45_754a_4e3b_ab2b_151181085605);
}
impl ::windows::core::RuntimeName for LineDisplayCursor {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayCursor";
}
impl ::core::convert::From<LineDisplayCursor> for ::windows::core::IUnknown {
    fn from(value: LineDisplayCursor) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LineDisplayCursor> for ::windows::core::IUnknown {
    fn from(value: &LineDisplayCursor) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LineDisplayCursor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LineDisplayCursor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LineDisplayCursor> for ::windows::core::IInspectable {
    fn from(value: LineDisplayCursor) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LineDisplayCursor> for ::windows::core::IInspectable {
    fn from(value: &LineDisplayCursor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LineDisplayCursor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LineDisplayCursor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for LineDisplayCursor {}
unsafe impl ::core::marker::Sync for LineDisplayCursor {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LineDisplayCursorAttributes(pub ::windows::core::IInspectable);
impl LineDisplayCursorAttributes {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsBlinkEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetIsBlinkEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CursorType(&self) -> ::windows::core::Result<LineDisplayCursorType> {
        let this = self;
        unsafe {
            let mut result__: LineDisplayCursorType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LineDisplayCursorType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetCursorType(&self, value: LineDisplayCursorType) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsAutoAdvanceEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetIsAutoAdvanceEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn SetPosition<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Point>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for LineDisplayCursorAttributes {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplayCursorAttributes;{4e2d54fe-4ffd-4190-aae1-ce285f20c896})");
}
unsafe impl ::windows::core::Interface for LineDisplayCursorAttributes {
    type Vtable = ILineDisplayCursorAttributes_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e2d54fe_4ffd_4190_aae1_ce285f20c896);
}
impl ::windows::core::RuntimeName for LineDisplayCursorAttributes {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayCursorAttributes";
}
impl ::core::convert::From<LineDisplayCursorAttributes> for ::windows::core::IUnknown {
    fn from(value: LineDisplayCursorAttributes) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LineDisplayCursorAttributes> for ::windows::core::IUnknown {
    fn from(value: &LineDisplayCursorAttributes) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LineDisplayCursorAttributes {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LineDisplayCursorAttributes {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LineDisplayCursorAttributes> for ::windows::core::IInspectable {
    fn from(value: LineDisplayCursorAttributes) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LineDisplayCursorAttributes> for ::windows::core::IInspectable {
    fn from(value: &LineDisplayCursorAttributes) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LineDisplayCursorAttributes {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LineDisplayCursorAttributes {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for LineDisplayCursorAttributes {}
unsafe impl ::core::marker::Sync for LineDisplayCursorAttributes {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct LineDisplayCursorType(pub i32);
impl LineDisplayCursorType {
    pub const None: LineDisplayCursorType = LineDisplayCursorType(0i32);
    pub const Block: LineDisplayCursorType = LineDisplayCursorType(1i32);
    pub const HalfBlock: LineDisplayCursorType = LineDisplayCursorType(2i32);
    pub const Underline: LineDisplayCursorType = LineDisplayCursorType(3i32);
    pub const Reverse: LineDisplayCursorType = LineDisplayCursorType(4i32);
    pub const Other: LineDisplayCursorType = LineDisplayCursorType(5i32);
}
impl ::core::convert::From<i32> for LineDisplayCursorType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for LineDisplayCursorType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for LineDisplayCursorType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.LineDisplayCursorType;i4)");
}
impl ::windows::core::DefaultType for LineDisplayCursorType {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LineDisplayCustomGlyphs(pub ::windows::core::IInspectable);
impl LineDisplayCustomGlyphs {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn SizeInPixels(&self) -> ::windows::core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Size = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation_Collections`*"]
    pub fn SupportedGlyphCodes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`, `Storage_Streams`*"]
    pub fn TryRedefineAsync<'a, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, glyphcode: u32, glyphdata: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), glyphcode, glyphdata.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for LineDisplayCustomGlyphs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplayCustomGlyphs;{2257f63c-f263-44f1-a1a0-e750a6a0ec54})");
}
unsafe impl ::windows::core::Interface for LineDisplayCustomGlyphs {
    type Vtable = ILineDisplayCustomGlyphs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2257f63c_f263_44f1_a1a0_e750a6a0ec54);
}
impl ::windows::core::RuntimeName for LineDisplayCustomGlyphs {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayCustomGlyphs";
}
impl ::core::convert::From<LineDisplayCustomGlyphs> for ::windows::core::IUnknown {
    fn from(value: LineDisplayCustomGlyphs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LineDisplayCustomGlyphs> for ::windows::core::IUnknown {
    fn from(value: &LineDisplayCustomGlyphs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LineDisplayCustomGlyphs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LineDisplayCustomGlyphs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LineDisplayCustomGlyphs> for ::windows::core::IInspectable {
    fn from(value: LineDisplayCustomGlyphs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LineDisplayCustomGlyphs> for ::windows::core::IInspectable {
    fn from(value: &LineDisplayCustomGlyphs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LineDisplayCustomGlyphs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LineDisplayCustomGlyphs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for LineDisplayCustomGlyphs {}
unsafe impl ::core::marker::Sync for LineDisplayCustomGlyphs {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct LineDisplayDescriptorState(pub i32);
impl LineDisplayDescriptorState {
    pub const Off: LineDisplayDescriptorState = LineDisplayDescriptorState(0i32);
    pub const On: LineDisplayDescriptorState = LineDisplayDescriptorState(1i32);
    pub const Blink: LineDisplayDescriptorState = LineDisplayDescriptorState(2i32);
}
impl ::core::convert::From<i32> for LineDisplayDescriptorState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for LineDisplayDescriptorState {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for LineDisplayDescriptorState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.LineDisplayDescriptorState;i4)");
}
impl ::windows::core::DefaultType for LineDisplayDescriptorState {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct LineDisplayHorizontalAlignment(pub i32);
impl LineDisplayHorizontalAlignment {
    pub const Left: LineDisplayHorizontalAlignment = LineDisplayHorizontalAlignment(0i32);
    pub const Center: LineDisplayHorizontalAlignment = LineDisplayHorizontalAlignment(1i32);
    pub const Right: LineDisplayHorizontalAlignment = LineDisplayHorizontalAlignment(2i32);
}
impl ::core::convert::From<i32> for LineDisplayHorizontalAlignment {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for LineDisplayHorizontalAlignment {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for LineDisplayHorizontalAlignment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.LineDisplayHorizontalAlignment;i4)");
}
impl ::windows::core::DefaultType for LineDisplayHorizontalAlignment {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LineDisplayMarquee(pub ::windows::core::IInspectable);
impl LineDisplayMarquee {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Format(&self) -> ::windows::core::Result<LineDisplayMarqueeFormat> {
        let this = self;
        unsafe {
            let mut result__: LineDisplayMarqueeFormat = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LineDisplayMarqueeFormat>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetFormat(&self, value: LineDisplayMarqueeFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RepeatWaitInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn SetRepeatWaitInterval<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn ScrollWaitInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn SetScrollWaitInterval<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn TryStartScrollingAsync(&self, direction: LineDisplayScrollDirection) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), direction, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn TryStopScrollingAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for LineDisplayMarquee {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplayMarquee;{a3d33e3e-f46a-4b7a-bc21-53eb3b57f8b4})");
}
unsafe impl ::windows::core::Interface for LineDisplayMarquee {
    type Vtable = ILineDisplayMarquee_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa3d33e3e_f46a_4b7a_bc21_53eb3b57f8b4);
}
impl ::windows::core::RuntimeName for LineDisplayMarquee {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayMarquee";
}
impl ::core::convert::From<LineDisplayMarquee> for ::windows::core::IUnknown {
    fn from(value: LineDisplayMarquee) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LineDisplayMarquee> for ::windows::core::IUnknown {
    fn from(value: &LineDisplayMarquee) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LineDisplayMarquee {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LineDisplayMarquee {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LineDisplayMarquee> for ::windows::core::IInspectable {
    fn from(value: LineDisplayMarquee) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LineDisplayMarquee> for ::windows::core::IInspectable {
    fn from(value: &LineDisplayMarquee) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LineDisplayMarquee {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LineDisplayMarquee {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for LineDisplayMarquee {}
unsafe impl ::core::marker::Sync for LineDisplayMarquee {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct LineDisplayMarqueeFormat(pub i32);
impl LineDisplayMarqueeFormat {
    pub const None: LineDisplayMarqueeFormat = LineDisplayMarqueeFormat(0i32);
    pub const Walk: LineDisplayMarqueeFormat = LineDisplayMarqueeFormat(1i32);
    pub const Place: LineDisplayMarqueeFormat = LineDisplayMarqueeFormat(2i32);
}
impl ::core::convert::From<i32> for LineDisplayMarqueeFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for LineDisplayMarqueeFormat {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for LineDisplayMarqueeFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.LineDisplayMarqueeFormat;i4)");
}
impl ::windows::core::DefaultType for LineDisplayMarqueeFormat {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct LineDisplayPowerStatus(pub i32);
impl LineDisplayPowerStatus {
    pub const Unknown: LineDisplayPowerStatus = LineDisplayPowerStatus(0i32);
    pub const Online: LineDisplayPowerStatus = LineDisplayPowerStatus(1i32);
    pub const Off: LineDisplayPowerStatus = LineDisplayPowerStatus(2i32);
    pub const Offline: LineDisplayPowerStatus = LineDisplayPowerStatus(3i32);
    pub const OffOrOffline: LineDisplayPowerStatus = LineDisplayPowerStatus(4i32);
}
impl ::core::convert::From<i32> for LineDisplayPowerStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for LineDisplayPowerStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for LineDisplayPowerStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.LineDisplayPowerStatus;i4)");
}
impl ::windows::core::DefaultType for LineDisplayPowerStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct LineDisplayScrollDirection(pub i32);
impl LineDisplayScrollDirection {
    pub const Up: LineDisplayScrollDirection = LineDisplayScrollDirection(0i32);
    pub const Down: LineDisplayScrollDirection = LineDisplayScrollDirection(1i32);
    pub const Left: LineDisplayScrollDirection = LineDisplayScrollDirection(2i32);
    pub const Right: LineDisplayScrollDirection = LineDisplayScrollDirection(3i32);
}
impl ::core::convert::From<i32> for LineDisplayScrollDirection {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for LineDisplayScrollDirection {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for LineDisplayScrollDirection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.LineDisplayScrollDirection;i4)");
}
impl ::windows::core::DefaultType for LineDisplayScrollDirection {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LineDisplayStatisticsCategorySelector(pub ::windows::core::IInspectable);
impl LineDisplayStatisticsCategorySelector {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn AllStatistics(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn UnifiedPosStatistics(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ManufacturerStatistics(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for LineDisplayStatisticsCategorySelector {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplayStatisticsCategorySelector;{b521c46b-9274-4d24-94f3-b6017b832444})");
}
unsafe impl ::windows::core::Interface for LineDisplayStatisticsCategorySelector {
    type Vtable = ILineDisplayStatisticsCategorySelector_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb521c46b_9274_4d24_94f3_b6017b832444);
}
impl ::windows::core::RuntimeName for LineDisplayStatisticsCategorySelector {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayStatisticsCategorySelector";
}
impl ::core::convert::From<LineDisplayStatisticsCategorySelector> for ::windows::core::IUnknown {
    fn from(value: LineDisplayStatisticsCategorySelector) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LineDisplayStatisticsCategorySelector> for ::windows::core::IUnknown {
    fn from(value: &LineDisplayStatisticsCategorySelector) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LineDisplayStatisticsCategorySelector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LineDisplayStatisticsCategorySelector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LineDisplayStatisticsCategorySelector> for ::windows::core::IInspectable {
    fn from(value: LineDisplayStatisticsCategorySelector) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LineDisplayStatisticsCategorySelector> for ::windows::core::IInspectable {
    fn from(value: &LineDisplayStatisticsCategorySelector) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LineDisplayStatisticsCategorySelector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LineDisplayStatisticsCategorySelector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for LineDisplayStatisticsCategorySelector {}
unsafe impl ::core::marker::Sync for LineDisplayStatisticsCategorySelector {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LineDisplayStatusUpdatedEventArgs(pub ::windows::core::IInspectable);
impl LineDisplayStatusUpdatedEventArgs {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Status(&self) -> ::windows::core::Result<LineDisplayPowerStatus> {
        let this = self;
        unsafe {
            let mut result__: LineDisplayPowerStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LineDisplayPowerStatus>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for LineDisplayStatusUpdatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplayStatusUpdatedEventArgs;{ddd57c1a-86fb-4eba-93d1-6f5eda52b752})");
}
unsafe impl ::windows::core::Interface for LineDisplayStatusUpdatedEventArgs {
    type Vtable = ILineDisplayStatusUpdatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xddd57c1a_86fb_4eba_93d1_6f5eda52b752);
}
impl ::windows::core::RuntimeName for LineDisplayStatusUpdatedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayStatusUpdatedEventArgs";
}
impl ::core::convert::From<LineDisplayStatusUpdatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: LineDisplayStatusUpdatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LineDisplayStatusUpdatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &LineDisplayStatusUpdatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LineDisplayStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LineDisplayStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LineDisplayStatusUpdatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: LineDisplayStatusUpdatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LineDisplayStatusUpdatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &LineDisplayStatusUpdatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LineDisplayStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LineDisplayStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for LineDisplayStatusUpdatedEventArgs {}
unsafe impl ::core::marker::Sync for LineDisplayStatusUpdatedEventArgs {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LineDisplayStoredBitmap(pub ::windows::core::IInspectable);
impl LineDisplayStoredBitmap {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn EscapeSequence(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn TryDeleteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for LineDisplayStoredBitmap {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplayStoredBitmap;{f621515b-d81e-43ba-bf1b-bcfa3c785ba0})");
}
unsafe impl ::windows::core::Interface for LineDisplayStoredBitmap {
    type Vtable = ILineDisplayStoredBitmap_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf621515b_d81e_43ba_bf1b_bcfa3c785ba0);
}
impl ::windows::core::RuntimeName for LineDisplayStoredBitmap {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayStoredBitmap";
}
impl ::core::convert::From<LineDisplayStoredBitmap> for ::windows::core::IUnknown {
    fn from(value: LineDisplayStoredBitmap) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LineDisplayStoredBitmap> for ::windows::core::IUnknown {
    fn from(value: &LineDisplayStoredBitmap) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LineDisplayStoredBitmap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LineDisplayStoredBitmap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LineDisplayStoredBitmap> for ::windows::core::IInspectable {
    fn from(value: LineDisplayStoredBitmap) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LineDisplayStoredBitmap> for ::windows::core::IInspectable {
    fn from(value: &LineDisplayStoredBitmap) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LineDisplayStoredBitmap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LineDisplayStoredBitmap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for LineDisplayStoredBitmap {}
unsafe impl ::core::marker::Sync for LineDisplayStoredBitmap {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct LineDisplayTextAttribute(pub i32);
impl LineDisplayTextAttribute {
    pub const Normal: LineDisplayTextAttribute = LineDisplayTextAttribute(0i32);
    pub const Blink: LineDisplayTextAttribute = LineDisplayTextAttribute(1i32);
    pub const Reverse: LineDisplayTextAttribute = LineDisplayTextAttribute(2i32);
    pub const ReverseBlink: LineDisplayTextAttribute = LineDisplayTextAttribute(3i32);
}
impl ::core::convert::From<i32> for LineDisplayTextAttribute {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for LineDisplayTextAttribute {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for LineDisplayTextAttribute {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.LineDisplayTextAttribute;i4)");
}
impl ::windows::core::DefaultType for LineDisplayTextAttribute {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct LineDisplayTextAttributeGranularity(pub i32);
impl LineDisplayTextAttributeGranularity {
    pub const NotSupported: LineDisplayTextAttributeGranularity = LineDisplayTextAttributeGranularity(0i32);
    pub const EntireDisplay: LineDisplayTextAttributeGranularity = LineDisplayTextAttributeGranularity(1i32);
    pub const PerCharacter: LineDisplayTextAttributeGranularity = LineDisplayTextAttributeGranularity(2i32);
}
impl ::core::convert::From<i32> for LineDisplayTextAttributeGranularity {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for LineDisplayTextAttributeGranularity {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for LineDisplayTextAttributeGranularity {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.LineDisplayTextAttributeGranularity;i4)");
}
impl ::windows::core::DefaultType for LineDisplayTextAttributeGranularity {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct LineDisplayVerticalAlignment(pub i32);
impl LineDisplayVerticalAlignment {
    pub const Top: LineDisplayVerticalAlignment = LineDisplayVerticalAlignment(0i32);
    pub const Center: LineDisplayVerticalAlignment = LineDisplayVerticalAlignment(1i32);
    pub const Bottom: LineDisplayVerticalAlignment = LineDisplayVerticalAlignment(2i32);
}
impl ::core::convert::From<i32> for LineDisplayVerticalAlignment {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for LineDisplayVerticalAlignment {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for LineDisplayVerticalAlignment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.LineDisplayVerticalAlignment;i4)");
}
impl ::windows::core::DefaultType for LineDisplayVerticalAlignment {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LineDisplayWindow(pub ::windows::core::IInspectable);
impl LineDisplayWindow {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn SizeInCharacters(&self) -> ::windows::core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Size = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn InterCharacterWaitInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn SetInterCharacterWaitInterval<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn TryRefreshAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn TryDisplayTextAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, text: Param0, displayattribute: LineDisplayTextAttribute) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), text.into_param().abi(), displayattribute, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn TryDisplayTextAtPositionAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::Point>>(&self, text: Param0, displayattribute: LineDisplayTextAttribute, startposition: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), text.into_param().abi(), displayattribute, startposition.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn TryDisplayTextNormalAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, text: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn TryScrollTextAsync(&self, direction: LineDisplayScrollDirection, numberofcolumnsorrows: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), direction, numberofcolumnsorrows, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn TryClearTextAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Cursor(&self) -> ::windows::core::Result<LineDisplayCursor> {
        let this = &::windows::core::Interface::cast::<ILineDisplayWindow2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LineDisplayCursor>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Marquee(&self) -> ::windows::core::Result<LineDisplayMarquee> {
        let this = &::windows::core::Interface::cast::<ILineDisplayWindow2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LineDisplayMarquee>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn ReadCharacterAtCursorAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>> {
        let this = &::windows::core::Interface::cast::<ILineDisplayWindow2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn TryDisplayStoredBitmapAtCursorAsync<'a, Param0: ::windows::core::IntoParam<'a, LineDisplayStoredBitmap>>(&self, bitmap: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<ILineDisplayWindow2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), bitmap.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`, `Storage`*"]
    pub fn TryDisplayStorageFileBitmapAtCursorAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::StorageFile>>(&self, bitmap: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<ILineDisplayWindow2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), bitmap.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`, `Storage`*"]
    pub fn TryDisplayStorageFileBitmapAtCursorWithAlignmentAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::StorageFile>>(&self, bitmap: Param0, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<ILineDisplayWindow2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), bitmap.into_param().abi(), horizontalalignment, verticalalignment, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`, `Storage`*"]
    pub fn TryDisplayStorageFileBitmapAtCursorWithAlignmentAndWidthAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::StorageFile>>(&self, bitmap: Param0, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment, widthinpixels: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<ILineDisplayWindow2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), bitmap.into_param().abi(), horizontalalignment, verticalalignment, widthinpixels, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`, `Storage`*"]
    pub fn TryDisplayStorageFileBitmapAtPointAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::StorageFile>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Point>>(&self, bitmap: Param0, offsetinpixels: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<ILineDisplayWindow2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), bitmap.into_param().abi(), offsetinpixels.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`, `Storage`*"]
    pub fn TryDisplayStorageFileBitmapAtPointWithWidthAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::StorageFile>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Point>>(&self, bitmap: Param0, offsetinpixels: Param1, widthinpixels: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<ILineDisplayWindow2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), bitmap.into_param().abi(), offsetinpixels.into_param().abi(), widthinpixels, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for LineDisplayWindow {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplayWindow;{d21feef4-2364-4be5-bee1-851680af4964})");
}
unsafe impl ::windows::core::Interface for LineDisplayWindow {
    type Vtable = ILineDisplayWindow_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd21feef4_2364_4be5_bee1_851680af4964);
}
impl ::windows::core::RuntimeName for LineDisplayWindow {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayWindow";
}
impl ::core::convert::From<LineDisplayWindow> for ::windows::core::IUnknown {
    fn from(value: LineDisplayWindow) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LineDisplayWindow> for ::windows::core::IUnknown {
    fn from(value: &LineDisplayWindow) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LineDisplayWindow {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LineDisplayWindow {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LineDisplayWindow> for ::windows::core::IInspectable {
    fn from(value: LineDisplayWindow) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LineDisplayWindow> for ::windows::core::IInspectable {
    fn from(value: &LineDisplayWindow) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LineDisplayWindow {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LineDisplayWindow {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<LineDisplayWindow> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: LineDisplayWindow) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&LineDisplayWindow> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &LineDisplayWindow) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for LineDisplayWindow {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &LineDisplayWindow {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for LineDisplayWindow {}
unsafe impl ::core::marker::Sync for LineDisplayWindow {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MagneticStripeReader(pub ::windows::core::IInspectable);
impl MagneticStripeReader {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Capabilities(&self) -> ::windows::core::Result<MagneticStripeReaderCapabilities> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MagneticStripeReaderCapabilities>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SupportedCardTypes(&self) -> ::windows::core::Result<::windows::core::Array<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::Array<u32> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), ::windows::core::Array::<u32>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DeviceAuthenticationProtocol(&self) -> ::windows::core::Result<MagneticStripeReaderAuthenticationProtocol> {
        let this = self;
        unsafe {
            let mut result__: MagneticStripeReaderAuthenticationProtocol = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MagneticStripeReaderAuthenticationProtocol>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn CheckHealthAsync(&self, level: UnifiedPosHealthCheckLevel) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), level, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn ClaimReaderAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ClaimedMagneticStripeReader>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ClaimedMagneticStripeReader>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`, `Foundation_Collections`, `Storage_Streams`*"]
    pub fn RetrieveStatisticsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, statisticscategories: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), statisticscategories.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn GetErrorReportingType(&self) -> ::windows::core::Result<MagneticStripeReaderErrorReportingType> {
        let this = self;
        unsafe {
            let mut result__: MagneticStripeReaderErrorReportingType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MagneticStripeReaderErrorReportingType>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn StatusUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MagneticStripeReader, MagneticStripeReaderStatusUpdatedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveStatusUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn GetDefaultAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MagneticStripeReader>> {
        Self::IMagneticStripeReaderStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MagneticStripeReader>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MagneticStripeReader>> {
        Self::IMagneticStripeReaderStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MagneticStripeReader>>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMagneticStripeReaderStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn GetDeviceSelectorWithConnectionTypes(connectiontypes: PosConnectionTypes) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMagneticStripeReaderStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), connectiontypes, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    pub fn IMagneticStripeReaderStatics<R, F: FnOnce(&IMagneticStripeReaderStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MagneticStripeReader, IMagneticStripeReaderStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IMagneticStripeReaderStatics2<R, F: FnOnce(&IMagneticStripeReaderStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MagneticStripeReader, IMagneticStripeReaderStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for MagneticStripeReader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.MagneticStripeReader;{1a92b015-47c3-468a-9333-0c6517574883})");
}
unsafe impl ::windows::core::Interface for MagneticStripeReader {
    type Vtable = IMagneticStripeReader_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a92b015_47c3_468a_9333_0c6517574883);
}
impl ::windows::core::RuntimeName for MagneticStripeReader {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReader";
}
impl ::core::convert::From<MagneticStripeReader> for ::windows::core::IUnknown {
    fn from(value: MagneticStripeReader) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MagneticStripeReader> for ::windows::core::IUnknown {
    fn from(value: &MagneticStripeReader) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MagneticStripeReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MagneticStripeReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MagneticStripeReader> for ::windows::core::IInspectable {
    fn from(value: MagneticStripeReader) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MagneticStripeReader> for ::windows::core::IInspectable {
    fn from(value: &MagneticStripeReader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MagneticStripeReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MagneticStripeReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<MagneticStripeReader> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: MagneticStripeReader) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&MagneticStripeReader> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &MagneticStripeReader) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for MagneticStripeReader {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &MagneticStripeReader {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MagneticStripeReader {}
unsafe impl ::core::marker::Sync for MagneticStripeReader {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MagneticStripeReaderAamvaCardDataReceivedEventArgs(pub ::windows::core::IInspectable);
impl MagneticStripeReaderAamvaCardDataReceivedEventArgs {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Report(&self) -> ::windows::core::Result<MagneticStripeReaderReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MagneticStripeReaderReport>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn LicenseNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ExpirationDate(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Restrictions(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Class(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Endorsements(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn BirthDate(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn FirstName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Surname(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Suffix(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Gender(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn HairColor(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn EyeColor(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Height(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Weight(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Address(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn City(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn State(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PostalCode(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MagneticStripeReaderAamvaCardDataReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.MagneticStripeReaderAamvaCardDataReceivedEventArgs;{0a4bbd51-c316-4910-87f3-7a62ba862d31})");
}
unsafe impl ::windows::core::Interface for MagneticStripeReaderAamvaCardDataReceivedEventArgs {
    type Vtable = IMagneticStripeReaderAamvaCardDataReceivedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a4bbd51_c316_4910_87f3_7a62ba862d31);
}
impl ::windows::core::RuntimeName for MagneticStripeReaderAamvaCardDataReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderAamvaCardDataReceivedEventArgs";
}
impl ::core::convert::From<MagneticStripeReaderAamvaCardDataReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MagneticStripeReaderAamvaCardDataReceivedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MagneticStripeReaderAamvaCardDataReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MagneticStripeReaderAamvaCardDataReceivedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MagneticStripeReaderAamvaCardDataReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MagneticStripeReaderAamvaCardDataReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MagneticStripeReaderAamvaCardDataReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MagneticStripeReaderAamvaCardDataReceivedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MagneticStripeReaderAamvaCardDataReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MagneticStripeReaderAamvaCardDataReceivedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MagneticStripeReaderAamvaCardDataReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MagneticStripeReaderAamvaCardDataReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MagneticStripeReaderAamvaCardDataReceivedEventArgs {}
unsafe impl ::core::marker::Sync for MagneticStripeReaderAamvaCardDataReceivedEventArgs {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MagneticStripeReaderAuthenticationLevel(pub i32);
impl MagneticStripeReaderAuthenticationLevel {
    pub const NotSupported: MagneticStripeReaderAuthenticationLevel = MagneticStripeReaderAuthenticationLevel(0i32);
    pub const Optional: MagneticStripeReaderAuthenticationLevel = MagneticStripeReaderAuthenticationLevel(1i32);
    pub const Required: MagneticStripeReaderAuthenticationLevel = MagneticStripeReaderAuthenticationLevel(2i32);
}
impl ::core::convert::From<i32> for MagneticStripeReaderAuthenticationLevel {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MagneticStripeReaderAuthenticationLevel {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for MagneticStripeReaderAuthenticationLevel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.MagneticStripeReaderAuthenticationLevel;i4)");
}
impl ::windows::core::DefaultType for MagneticStripeReaderAuthenticationLevel {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MagneticStripeReaderAuthenticationProtocol(pub i32);
impl MagneticStripeReaderAuthenticationProtocol {
    pub const None: MagneticStripeReaderAuthenticationProtocol = MagneticStripeReaderAuthenticationProtocol(0i32);
    pub const ChallengeResponse: MagneticStripeReaderAuthenticationProtocol = MagneticStripeReaderAuthenticationProtocol(1i32);
}
impl ::core::convert::From<i32> for MagneticStripeReaderAuthenticationProtocol {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MagneticStripeReaderAuthenticationProtocol {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for MagneticStripeReaderAuthenticationProtocol {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.MagneticStripeReaderAuthenticationProtocol;i4)");
}
impl ::windows::core::DefaultType for MagneticStripeReaderAuthenticationProtocol {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MagneticStripeReaderBankCardDataReceivedEventArgs(pub ::windows::core::IInspectable);
impl MagneticStripeReaderBankCardDataReceivedEventArgs {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Report(&self) -> ::windows::core::Result<MagneticStripeReaderReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MagneticStripeReaderReport>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn AccountNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ExpirationDate(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ServiceCode(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn FirstName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn MiddleInitial(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Surname(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Suffix(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MagneticStripeReaderBankCardDataReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.MagneticStripeReaderBankCardDataReceivedEventArgs;{2e958823-a31a-4763-882c-23725e39b08e})");
}
unsafe impl ::windows::core::Interface for MagneticStripeReaderBankCardDataReceivedEventArgs {
    type Vtable = IMagneticStripeReaderBankCardDataReceivedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e958823_a31a_4763_882c_23725e39b08e);
}
impl ::windows::core::RuntimeName for MagneticStripeReaderBankCardDataReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderBankCardDataReceivedEventArgs";
}
impl ::core::convert::From<MagneticStripeReaderBankCardDataReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MagneticStripeReaderBankCardDataReceivedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MagneticStripeReaderBankCardDataReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MagneticStripeReaderBankCardDataReceivedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MagneticStripeReaderBankCardDataReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MagneticStripeReaderBankCardDataReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MagneticStripeReaderBankCardDataReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MagneticStripeReaderBankCardDataReceivedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MagneticStripeReaderBankCardDataReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MagneticStripeReaderBankCardDataReceivedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MagneticStripeReaderBankCardDataReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MagneticStripeReaderBankCardDataReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MagneticStripeReaderBankCardDataReceivedEventArgs {}
unsafe impl ::core::marker::Sync for MagneticStripeReaderBankCardDataReceivedEventArgs {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MagneticStripeReaderCapabilities(pub ::windows::core::IInspectable);
impl MagneticStripeReaderCapabilities {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CardAuthentication(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SupportedEncryptionAlgorithms(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn AuthenticationLevel(&self) -> ::windows::core::Result<MagneticStripeReaderAuthenticationLevel> {
        let this = self;
        unsafe {
            let mut result__: MagneticStripeReaderAuthenticationLevel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MagneticStripeReaderAuthenticationLevel>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsIsoSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsJisOneSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsJisTwoSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PowerReportingType(&self) -> ::windows::core::Result<UnifiedPosPowerReportingType> {
        let this = self;
        unsafe {
            let mut result__: UnifiedPosPowerReportingType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UnifiedPosPowerReportingType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsStatisticsReportingSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsStatisticsUpdatingSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsTrackDataMaskingSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsTransmitSentinelsSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MagneticStripeReaderCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.MagneticStripeReaderCapabilities;{7128809c-c440-44a2-a467-469175d02896})");
}
unsafe impl ::windows::core::Interface for MagneticStripeReaderCapabilities {
    type Vtable = IMagneticStripeReaderCapabilities_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7128809c_c440_44a2_a467_469175d02896);
}
impl ::windows::core::RuntimeName for MagneticStripeReaderCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderCapabilities";
}
impl ::core::convert::From<MagneticStripeReaderCapabilities> for ::windows::core::IUnknown {
    fn from(value: MagneticStripeReaderCapabilities) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MagneticStripeReaderCapabilities> for ::windows::core::IUnknown {
    fn from(value: &MagneticStripeReaderCapabilities) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MagneticStripeReaderCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MagneticStripeReaderCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MagneticStripeReaderCapabilities> for ::windows::core::IInspectable {
    fn from(value: MagneticStripeReaderCapabilities) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MagneticStripeReaderCapabilities> for ::windows::core::IInspectable {
    fn from(value: &MagneticStripeReaderCapabilities) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MagneticStripeReaderCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MagneticStripeReaderCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MagneticStripeReaderCapabilities {}
unsafe impl ::core::marker::Sync for MagneticStripeReaderCapabilities {}
#[doc = "*Required features: `Devices_PointOfService`*"]
pub struct MagneticStripeReaderCardTypes {}
impl MagneticStripeReaderCardTypes {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Unknown() -> ::windows::core::Result<u32> {
        Self::IMagneticStripeReaderCardTypesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Bank() -> ::windows::core::Result<u32> {
        Self::IMagneticStripeReaderCardTypesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Aamva() -> ::windows::core::Result<u32> {
        Self::IMagneticStripeReaderCardTypesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ExtendedBase() -> ::windows::core::Result<u32> {
        Self::IMagneticStripeReaderCardTypesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    pub fn IMagneticStripeReaderCardTypesStatics<R, F: FnOnce(&IMagneticStripeReaderCardTypesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MagneticStripeReaderCardTypes, IMagneticStripeReaderCardTypesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for MagneticStripeReaderCardTypes {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderCardTypes";
}
#[doc = "*Required features: `Devices_PointOfService`*"]
pub struct MagneticStripeReaderEncryptionAlgorithms {}
impl MagneticStripeReaderEncryptionAlgorithms {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn None() -> ::windows::core::Result<u32> {
        Self::IMagneticStripeReaderEncryptionAlgorithmsStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn TripleDesDukpt() -> ::windows::core::Result<u32> {
        Self::IMagneticStripeReaderEncryptionAlgorithmsStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ExtendedBase() -> ::windows::core::Result<u32> {
        Self::IMagneticStripeReaderEncryptionAlgorithmsStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    pub fn IMagneticStripeReaderEncryptionAlgorithmsStatics<R, F: FnOnce(&IMagneticStripeReaderEncryptionAlgorithmsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MagneticStripeReaderEncryptionAlgorithms, IMagneticStripeReaderEncryptionAlgorithmsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for MagneticStripeReaderEncryptionAlgorithms {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderEncryptionAlgorithms";
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MagneticStripeReaderErrorOccurredEventArgs(pub ::windows::core::IInspectable);
impl MagneticStripeReaderErrorOccurredEventArgs {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Track1Status(&self) -> ::windows::core::Result<MagneticStripeReaderTrackErrorType> {
        let this = self;
        unsafe {
            let mut result__: MagneticStripeReaderTrackErrorType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MagneticStripeReaderTrackErrorType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Track2Status(&self) -> ::windows::core::Result<MagneticStripeReaderTrackErrorType> {
        let this = self;
        unsafe {
            let mut result__: MagneticStripeReaderTrackErrorType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MagneticStripeReaderTrackErrorType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Track3Status(&self) -> ::windows::core::Result<MagneticStripeReaderTrackErrorType> {
        let this = self;
        unsafe {
            let mut result__: MagneticStripeReaderTrackErrorType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MagneticStripeReaderTrackErrorType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Track4Status(&self) -> ::windows::core::Result<MagneticStripeReaderTrackErrorType> {
        let this = self;
        unsafe {
            let mut result__: MagneticStripeReaderTrackErrorType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MagneticStripeReaderTrackErrorType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ErrorData(&self) -> ::windows::core::Result<UnifiedPosErrorData> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UnifiedPosErrorData>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PartialInputData(&self) -> ::windows::core::Result<MagneticStripeReaderReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MagneticStripeReaderReport>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MagneticStripeReaderErrorOccurredEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.MagneticStripeReaderErrorOccurredEventArgs;{1fedf95d-2c84-41ad-b778-f2356a789ab1})");
}
unsafe impl ::windows::core::Interface for MagneticStripeReaderErrorOccurredEventArgs {
    type Vtable = IMagneticStripeReaderErrorOccurredEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1fedf95d_2c84_41ad_b778_f2356a789ab1);
}
impl ::windows::core::RuntimeName for MagneticStripeReaderErrorOccurredEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderErrorOccurredEventArgs";
}
impl ::core::convert::From<MagneticStripeReaderErrorOccurredEventArgs> for ::windows::core::IUnknown {
    fn from(value: MagneticStripeReaderErrorOccurredEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MagneticStripeReaderErrorOccurredEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MagneticStripeReaderErrorOccurredEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MagneticStripeReaderErrorOccurredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MagneticStripeReaderErrorOccurredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MagneticStripeReaderErrorOccurredEventArgs> for ::windows::core::IInspectable {
    fn from(value: MagneticStripeReaderErrorOccurredEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MagneticStripeReaderErrorOccurredEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MagneticStripeReaderErrorOccurredEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MagneticStripeReaderErrorOccurredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MagneticStripeReaderErrorOccurredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MagneticStripeReaderErrorOccurredEventArgs {}
unsafe impl ::core::marker::Sync for MagneticStripeReaderErrorOccurredEventArgs {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MagneticStripeReaderErrorReportingType(pub i32);
impl MagneticStripeReaderErrorReportingType {
    pub const CardLevel: MagneticStripeReaderErrorReportingType = MagneticStripeReaderErrorReportingType(0i32);
    pub const TrackLevel: MagneticStripeReaderErrorReportingType = MagneticStripeReaderErrorReportingType(1i32);
}
impl ::core::convert::From<i32> for MagneticStripeReaderErrorReportingType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MagneticStripeReaderErrorReportingType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for MagneticStripeReaderErrorReportingType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.MagneticStripeReaderErrorReportingType;i4)");
}
impl ::windows::core::DefaultType for MagneticStripeReaderErrorReportingType {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MagneticStripeReaderReport(pub ::windows::core::IInspectable);
impl MagneticStripeReaderReport {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CardType(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Track1(&self) -> ::windows::core::Result<MagneticStripeReaderTrackData> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MagneticStripeReaderTrackData>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Track2(&self) -> ::windows::core::Result<MagneticStripeReaderTrackData> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MagneticStripeReaderTrackData>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Track3(&self) -> ::windows::core::Result<MagneticStripeReaderTrackData> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MagneticStripeReaderTrackData>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Track4(&self) -> ::windows::core::Result<MagneticStripeReaderTrackData> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MagneticStripeReaderTrackData>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation_Collections`*"]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_PointOfService`, `Storage_Streams`*"]
    pub fn CardAuthenticationData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CardAuthenticationDataLength(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_PointOfService`, `Storage_Streams`*"]
    pub fn AdditionalSecurityInformation(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MagneticStripeReaderReport {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.MagneticStripeReaderReport;{6a5b6047-99b0-4188-bef1-eddf79f78fe6})");
}
unsafe impl ::windows::core::Interface for MagneticStripeReaderReport {
    type Vtable = IMagneticStripeReaderReport_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a5b6047_99b0_4188_bef1_eddf79f78fe6);
}
impl ::windows::core::RuntimeName for MagneticStripeReaderReport {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderReport";
}
impl ::core::convert::From<MagneticStripeReaderReport> for ::windows::core::IUnknown {
    fn from(value: MagneticStripeReaderReport) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MagneticStripeReaderReport> for ::windows::core::IUnknown {
    fn from(value: &MagneticStripeReaderReport) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MagneticStripeReaderReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MagneticStripeReaderReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MagneticStripeReaderReport> for ::windows::core::IInspectable {
    fn from(value: MagneticStripeReaderReport) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MagneticStripeReaderReport> for ::windows::core::IInspectable {
    fn from(value: &MagneticStripeReaderReport) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MagneticStripeReaderReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MagneticStripeReaderReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MagneticStripeReaderReport {}
unsafe impl ::core::marker::Sync for MagneticStripeReaderReport {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MagneticStripeReaderStatus(pub i32);
impl MagneticStripeReaderStatus {
    pub const Unauthenticated: MagneticStripeReaderStatus = MagneticStripeReaderStatus(0i32);
    pub const Authenticated: MagneticStripeReaderStatus = MagneticStripeReaderStatus(1i32);
    pub const Extended: MagneticStripeReaderStatus = MagneticStripeReaderStatus(2i32);
}
impl ::core::convert::From<i32> for MagneticStripeReaderStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MagneticStripeReaderStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for MagneticStripeReaderStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.MagneticStripeReaderStatus;i4)");
}
impl ::windows::core::DefaultType for MagneticStripeReaderStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MagneticStripeReaderStatusUpdatedEventArgs(pub ::windows::core::IInspectable);
impl MagneticStripeReaderStatusUpdatedEventArgs {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Status(&self) -> ::windows::core::Result<MagneticStripeReaderStatus> {
        let this = self;
        unsafe {
            let mut result__: MagneticStripeReaderStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MagneticStripeReaderStatus>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ExtendedStatus(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MagneticStripeReaderStatusUpdatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.MagneticStripeReaderStatusUpdatedEventArgs;{09cc6bb0-3262-401d-9e8a-e80d6358906b})");
}
unsafe impl ::windows::core::Interface for MagneticStripeReaderStatusUpdatedEventArgs {
    type Vtable = IMagneticStripeReaderStatusUpdatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09cc6bb0_3262_401d_9e8a_e80d6358906b);
}
impl ::windows::core::RuntimeName for MagneticStripeReaderStatusUpdatedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderStatusUpdatedEventArgs";
}
impl ::core::convert::From<MagneticStripeReaderStatusUpdatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MagneticStripeReaderStatusUpdatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MagneticStripeReaderStatusUpdatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MagneticStripeReaderStatusUpdatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MagneticStripeReaderStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MagneticStripeReaderStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MagneticStripeReaderStatusUpdatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MagneticStripeReaderStatusUpdatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MagneticStripeReaderStatusUpdatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MagneticStripeReaderStatusUpdatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MagneticStripeReaderStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MagneticStripeReaderStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MagneticStripeReaderStatusUpdatedEventArgs {}
unsafe impl ::core::marker::Sync for MagneticStripeReaderStatusUpdatedEventArgs {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MagneticStripeReaderTrackData(pub ::windows::core::IInspectable);
impl MagneticStripeReaderTrackData {
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_PointOfService`, `Storage_Streams`*"]
    pub fn Data(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_PointOfService`, `Storage_Streams`*"]
    pub fn DiscretionaryData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_PointOfService`, `Storage_Streams`*"]
    pub fn EncryptedData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MagneticStripeReaderTrackData {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.MagneticStripeReaderTrackData;{104cf671-4a9d-446e-abc5-20402307ba36})");
}
unsafe impl ::windows::core::Interface for MagneticStripeReaderTrackData {
    type Vtable = IMagneticStripeReaderTrackData_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x104cf671_4a9d_446e_abc5_20402307ba36);
}
impl ::windows::core::RuntimeName for MagneticStripeReaderTrackData {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderTrackData";
}
impl ::core::convert::From<MagneticStripeReaderTrackData> for ::windows::core::IUnknown {
    fn from(value: MagneticStripeReaderTrackData) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MagneticStripeReaderTrackData> for ::windows::core::IUnknown {
    fn from(value: &MagneticStripeReaderTrackData) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MagneticStripeReaderTrackData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MagneticStripeReaderTrackData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MagneticStripeReaderTrackData> for ::windows::core::IInspectable {
    fn from(value: MagneticStripeReaderTrackData) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MagneticStripeReaderTrackData> for ::windows::core::IInspectable {
    fn from(value: &MagneticStripeReaderTrackData) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MagneticStripeReaderTrackData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MagneticStripeReaderTrackData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MagneticStripeReaderTrackData {}
unsafe impl ::core::marker::Sync for MagneticStripeReaderTrackData {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MagneticStripeReaderTrackErrorType(pub i32);
impl MagneticStripeReaderTrackErrorType {
    pub const None: MagneticStripeReaderTrackErrorType = MagneticStripeReaderTrackErrorType(0i32);
    pub const StartSentinelError: MagneticStripeReaderTrackErrorType = MagneticStripeReaderTrackErrorType(1i32);
    pub const EndSentinelError: MagneticStripeReaderTrackErrorType = MagneticStripeReaderTrackErrorType(2i32);
    pub const ParityError: MagneticStripeReaderTrackErrorType = MagneticStripeReaderTrackErrorType(3i32);
    pub const LrcError: MagneticStripeReaderTrackErrorType = MagneticStripeReaderTrackErrorType(4i32);
    pub const Unknown: MagneticStripeReaderTrackErrorType = MagneticStripeReaderTrackErrorType(-1i32);
}
impl ::core::convert::From<i32> for MagneticStripeReaderTrackErrorType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MagneticStripeReaderTrackErrorType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for MagneticStripeReaderTrackErrorType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.MagneticStripeReaderTrackErrorType;i4)");
}
impl ::windows::core::DefaultType for MagneticStripeReaderTrackErrorType {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MagneticStripeReaderTrackIds(pub i32);
impl MagneticStripeReaderTrackIds {
    pub const None: MagneticStripeReaderTrackIds = MagneticStripeReaderTrackIds(0i32);
    pub const Track1: MagneticStripeReaderTrackIds = MagneticStripeReaderTrackIds(1i32);
    pub const Track2: MagneticStripeReaderTrackIds = MagneticStripeReaderTrackIds(2i32);
    pub const Track3: MagneticStripeReaderTrackIds = MagneticStripeReaderTrackIds(4i32);
    pub const Track4: MagneticStripeReaderTrackIds = MagneticStripeReaderTrackIds(8i32);
}
impl ::core::convert::From<i32> for MagneticStripeReaderTrackIds {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MagneticStripeReaderTrackIds {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for MagneticStripeReaderTrackIds {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.MagneticStripeReaderTrackIds;i4)");
}
impl ::windows::core::DefaultType for MagneticStripeReaderTrackIds {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs(pub ::windows::core::IInspectable);
impl MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Report(&self) -> ::windows::core::Result<MagneticStripeReaderReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MagneticStripeReaderReport>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs;{af0a5514-59cc-4a60-99e8-99a53dace5aa})");
}
unsafe impl ::windows::core::Interface for MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    type Vtable = IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf0a5514_59cc_4a60_99e8_99a53dace5aa);
}
impl ::windows::core::RuntimeName for MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs";
}
impl ::core::convert::From<MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {}
unsafe impl ::core::marker::Sync for MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PosConnectionTypes(pub u32);
impl PosConnectionTypes {
    pub const Local: PosConnectionTypes = PosConnectionTypes(1u32);
    pub const IP: PosConnectionTypes = PosConnectionTypes(2u32);
    pub const Bluetooth: PosConnectionTypes = PosConnectionTypes(4u32);
    pub const All: PosConnectionTypes = PosConnectionTypes(4294967295u32);
}
impl ::core::convert::From<u32> for PosConnectionTypes {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PosConnectionTypes {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PosConnectionTypes {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosConnectionTypes;u4)");
}
impl ::windows::core::DefaultType for PosConnectionTypes {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for PosConnectionTypes {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for PosConnectionTypes {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for PosConnectionTypes {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for PosConnectionTypes {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for PosConnectionTypes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PosPrinter(pub ::windows::core::IInspectable);
impl PosPrinter {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Capabilities(&self) -> ::windows::core::Result<PosPrinterCapabilities> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterCapabilities>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation_Collections`*"]
    pub fn SupportedCharacterSets(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation_Collections`*"]
    pub fn SupportedTypeFaces(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Status(&self) -> ::windows::core::Result<PosPrinterStatus> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn ClaimPrinterAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ClaimedPosPrinter>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ClaimedPosPrinter>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn CheckHealthAsync(&self, level: UnifiedPosHealthCheckLevel) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), level, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetStatisticsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, statisticscategories: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), statisticscategories.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn StatusUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PosPrinter, PosPrinterStatusUpdatedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveStatusUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn GetDefaultAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PosPrinter>> {
        Self::IPosPrinterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PosPrinter>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PosPrinter>> {
        Self::IPosPrinterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PosPrinter>>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IPosPrinterStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn GetDeviceSelectorWithConnectionTypes(connectiontypes: PosConnectionTypes) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IPosPrinterStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), connectiontypes, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation_Collections`*"]
    pub fn SupportedBarcodeSymbologies(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &::windows::core::Interface::cast::<IPosPrinter2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn GetFontProperty<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, typeface: Param0) -> ::windows::core::Result<PosPrinterFontProperty> {
        let this = &::windows::core::Interface::cast::<IPosPrinter2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), typeface.into_param().abi(), &mut result__).from_abi::<PosPrinterFontProperty>(result__)
        }
    }
    pub fn IPosPrinterStatics<R, F: FnOnce(&IPosPrinterStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PosPrinter, IPosPrinterStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IPosPrinterStatics2<R, F: FnOnce(&IPosPrinterStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PosPrinter, IPosPrinterStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for PosPrinter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.PosPrinter;{2a03c10e-9a19-4a01-994f-12dfad6adcbf})");
}
unsafe impl ::windows::core::Interface for PosPrinter {
    type Vtable = IPosPrinter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a03c10e_9a19_4a01_994f_12dfad6adcbf);
}
impl ::windows::core::RuntimeName for PosPrinter {
    const NAME: &'static str = "Windows.Devices.PointOfService.PosPrinter";
}
impl ::core::convert::From<PosPrinter> for ::windows::core::IUnknown {
    fn from(value: PosPrinter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PosPrinter> for ::windows::core::IUnknown {
    fn from(value: &PosPrinter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PosPrinter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PosPrinter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PosPrinter> for ::windows::core::IInspectable {
    fn from(value: PosPrinter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PosPrinter> for ::windows::core::IInspectable {
    fn from(value: &PosPrinter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PosPrinter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PosPrinter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<PosPrinter> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: PosPrinter) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&PosPrinter> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &PosPrinter) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for PosPrinter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &PosPrinter {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for PosPrinter {}
unsafe impl ::core::marker::Sync for PosPrinter {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PosPrinterAlignment(pub i32);
impl PosPrinterAlignment {
    pub const Left: PosPrinterAlignment = PosPrinterAlignment(0i32);
    pub const Center: PosPrinterAlignment = PosPrinterAlignment(1i32);
    pub const Right: PosPrinterAlignment = PosPrinterAlignment(2i32);
}
impl ::core::convert::From<i32> for PosPrinterAlignment {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PosPrinterAlignment {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PosPrinterAlignment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterAlignment;i4)");
}
impl ::windows::core::DefaultType for PosPrinterAlignment {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PosPrinterBarcodeTextPosition(pub i32);
impl PosPrinterBarcodeTextPosition {
    pub const None: PosPrinterBarcodeTextPosition = PosPrinterBarcodeTextPosition(0i32);
    pub const Above: PosPrinterBarcodeTextPosition = PosPrinterBarcodeTextPosition(1i32);
    pub const Below: PosPrinterBarcodeTextPosition = PosPrinterBarcodeTextPosition(2i32);
}
impl ::core::convert::From<i32> for PosPrinterBarcodeTextPosition {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PosPrinterBarcodeTextPosition {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PosPrinterBarcodeTextPosition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterBarcodeTextPosition;i4)");
}
impl ::windows::core::DefaultType for PosPrinterBarcodeTextPosition {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PosPrinterCapabilities(pub ::windows::core::IInspectable);
impl PosPrinterCapabilities {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PowerReportingType(&self) -> ::windows::core::Result<UnifiedPosPowerReportingType> {
        let this = self;
        unsafe {
            let mut result__: UnifiedPosPowerReportingType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UnifiedPosPowerReportingType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsStatisticsReportingSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsStatisticsUpdatingSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DefaultCharacterSet(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn HasCoverSensor(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CanMapCharacterSet(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsTransactionSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Receipt(&self) -> ::windows::core::Result<ReceiptPrinterCapabilities> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ReceiptPrinterCapabilities>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Slip(&self) -> ::windows::core::Result<SlipPrinterCapabilities> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SlipPrinterCapabilities>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Journal(&self) -> ::windows::core::Result<JournalPrinterCapabilities> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<JournalPrinterCapabilities>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PosPrinterCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.PosPrinterCapabilities;{cde95721-4380-4985-adc5-39db30cd93bc})");
}
unsafe impl ::windows::core::Interface for PosPrinterCapabilities {
    type Vtable = IPosPrinterCapabilities_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcde95721_4380_4985_adc5_39db30cd93bc);
}
impl ::windows::core::RuntimeName for PosPrinterCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.PosPrinterCapabilities";
}
impl ::core::convert::From<PosPrinterCapabilities> for ::windows::core::IUnknown {
    fn from(value: PosPrinterCapabilities) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PosPrinterCapabilities> for ::windows::core::IUnknown {
    fn from(value: &PosPrinterCapabilities) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PosPrinterCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PosPrinterCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PosPrinterCapabilities> for ::windows::core::IInspectable {
    fn from(value: PosPrinterCapabilities) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PosPrinterCapabilities> for ::windows::core::IInspectable {
    fn from(value: &PosPrinterCapabilities) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PosPrinterCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PosPrinterCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PosPrinterCapabilities {}
unsafe impl ::core::marker::Sync for PosPrinterCapabilities {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PosPrinterCartridgeSensors(pub u32);
impl PosPrinterCartridgeSensors {
    pub const None: PosPrinterCartridgeSensors = PosPrinterCartridgeSensors(0u32);
    pub const Removed: PosPrinterCartridgeSensors = PosPrinterCartridgeSensors(1u32);
    pub const Empty: PosPrinterCartridgeSensors = PosPrinterCartridgeSensors(2u32);
    pub const HeadCleaning: PosPrinterCartridgeSensors = PosPrinterCartridgeSensors(4u32);
    pub const NearEnd: PosPrinterCartridgeSensors = PosPrinterCartridgeSensors(8u32);
}
impl ::core::convert::From<u32> for PosPrinterCartridgeSensors {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PosPrinterCartridgeSensors {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PosPrinterCartridgeSensors {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterCartridgeSensors;u4)");
}
impl ::windows::core::DefaultType for PosPrinterCartridgeSensors {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for PosPrinterCartridgeSensors {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for PosPrinterCartridgeSensors {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for PosPrinterCartridgeSensors {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for PosPrinterCartridgeSensors {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for PosPrinterCartridgeSensors {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Devices_PointOfService`*"]
pub struct PosPrinterCharacterSetIds {}
impl PosPrinterCharacterSetIds {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Utf16LE() -> ::windows::core::Result<u32> {
        Self::IPosPrinterCharacterSetIdsStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Ascii() -> ::windows::core::Result<u32> {
        Self::IPosPrinterCharacterSetIdsStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Ansi() -> ::windows::core::Result<u32> {
        Self::IPosPrinterCharacterSetIdsStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    pub fn IPosPrinterCharacterSetIdsStatics<R, F: FnOnce(&IPosPrinterCharacterSetIdsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PosPrinterCharacterSetIds, IPosPrinterCharacterSetIdsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for PosPrinterCharacterSetIds {
    const NAME: &'static str = "Windows.Devices.PointOfService.PosPrinterCharacterSetIds";
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PosPrinterColorCapabilities(pub u32);
impl PosPrinterColorCapabilities {
    pub const None: PosPrinterColorCapabilities = PosPrinterColorCapabilities(0u32);
    pub const Primary: PosPrinterColorCapabilities = PosPrinterColorCapabilities(1u32);
    pub const Custom1: PosPrinterColorCapabilities = PosPrinterColorCapabilities(2u32);
    pub const Custom2: PosPrinterColorCapabilities = PosPrinterColorCapabilities(4u32);
    pub const Custom3: PosPrinterColorCapabilities = PosPrinterColorCapabilities(8u32);
    pub const Custom4: PosPrinterColorCapabilities = PosPrinterColorCapabilities(16u32);
    pub const Custom5: PosPrinterColorCapabilities = PosPrinterColorCapabilities(32u32);
    pub const Custom6: PosPrinterColorCapabilities = PosPrinterColorCapabilities(64u32);
    pub const Cyan: PosPrinterColorCapabilities = PosPrinterColorCapabilities(128u32);
    pub const Magenta: PosPrinterColorCapabilities = PosPrinterColorCapabilities(256u32);
    pub const Yellow: PosPrinterColorCapabilities = PosPrinterColorCapabilities(512u32);
    pub const Full: PosPrinterColorCapabilities = PosPrinterColorCapabilities(1024u32);
}
impl ::core::convert::From<u32> for PosPrinterColorCapabilities {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PosPrinterColorCapabilities {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PosPrinterColorCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterColorCapabilities;u4)");
}
impl ::windows::core::DefaultType for PosPrinterColorCapabilities {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for PosPrinterColorCapabilities {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for PosPrinterColorCapabilities {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for PosPrinterColorCapabilities {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for PosPrinterColorCapabilities {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for PosPrinterColorCapabilities {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PosPrinterColorCartridge(pub i32);
impl PosPrinterColorCartridge {
    pub const Unknown: PosPrinterColorCartridge = PosPrinterColorCartridge(0i32);
    pub const Primary: PosPrinterColorCartridge = PosPrinterColorCartridge(1i32);
    pub const Custom1: PosPrinterColorCartridge = PosPrinterColorCartridge(2i32);
    pub const Custom2: PosPrinterColorCartridge = PosPrinterColorCartridge(3i32);
    pub const Custom3: PosPrinterColorCartridge = PosPrinterColorCartridge(4i32);
    pub const Custom4: PosPrinterColorCartridge = PosPrinterColorCartridge(5i32);
    pub const Custom5: PosPrinterColorCartridge = PosPrinterColorCartridge(6i32);
    pub const Custom6: PosPrinterColorCartridge = PosPrinterColorCartridge(7i32);
    pub const Cyan: PosPrinterColorCartridge = PosPrinterColorCartridge(8i32);
    pub const Magenta: PosPrinterColorCartridge = PosPrinterColorCartridge(9i32);
    pub const Yellow: PosPrinterColorCartridge = PosPrinterColorCartridge(10i32);
}
impl ::core::convert::From<i32> for PosPrinterColorCartridge {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PosPrinterColorCartridge {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PosPrinterColorCartridge {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterColorCartridge;i4)");
}
impl ::windows::core::DefaultType for PosPrinterColorCartridge {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PosPrinterFontProperty(pub ::windows::core::IInspectable);
impl PosPrinterFontProperty {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn TypeFace(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsScalableToAnySize(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation_Collections`*"]
    pub fn CharacterSizes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SizeUInt32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<SizeUInt32>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PosPrinterFontProperty {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.PosPrinterFontProperty;{a7f4e93a-f8ac-5f04-84d2-29b16d8a633c})");
}
unsafe impl ::windows::core::Interface for PosPrinterFontProperty {
    type Vtable = IPosPrinterFontProperty_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7f4e93a_f8ac_5f04_84d2_29b16d8a633c);
}
impl ::windows::core::RuntimeName for PosPrinterFontProperty {
    const NAME: &'static str = "Windows.Devices.PointOfService.PosPrinterFontProperty";
}
impl ::core::convert::From<PosPrinterFontProperty> for ::windows::core::IUnknown {
    fn from(value: PosPrinterFontProperty) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PosPrinterFontProperty> for ::windows::core::IUnknown {
    fn from(value: &PosPrinterFontProperty) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PosPrinterFontProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PosPrinterFontProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PosPrinterFontProperty> for ::windows::core::IInspectable {
    fn from(value: PosPrinterFontProperty) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PosPrinterFontProperty> for ::windows::core::IInspectable {
    fn from(value: &PosPrinterFontProperty) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PosPrinterFontProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PosPrinterFontProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PosPrinterFontProperty {}
unsafe impl ::core::marker::Sync for PosPrinterFontProperty {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PosPrinterLineDirection(pub i32);
impl PosPrinterLineDirection {
    pub const Horizontal: PosPrinterLineDirection = PosPrinterLineDirection(0i32);
    pub const Vertical: PosPrinterLineDirection = PosPrinterLineDirection(1i32);
}
impl ::core::convert::From<i32> for PosPrinterLineDirection {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PosPrinterLineDirection {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PosPrinterLineDirection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterLineDirection;i4)");
}
impl ::windows::core::DefaultType for PosPrinterLineDirection {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PosPrinterLineStyle(pub i32);
impl PosPrinterLineStyle {
    pub const SingleSolid: PosPrinterLineStyle = PosPrinterLineStyle(0i32);
    pub const DoubleSolid: PosPrinterLineStyle = PosPrinterLineStyle(1i32);
    pub const Broken: PosPrinterLineStyle = PosPrinterLineStyle(2i32);
    pub const Chain: PosPrinterLineStyle = PosPrinterLineStyle(3i32);
}
impl ::core::convert::From<i32> for PosPrinterLineStyle {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PosPrinterLineStyle {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PosPrinterLineStyle {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterLineStyle;i4)");
}
impl ::windows::core::DefaultType for PosPrinterLineStyle {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PosPrinterMapMode(pub i32);
impl PosPrinterMapMode {
    pub const Dots: PosPrinterMapMode = PosPrinterMapMode(0i32);
    pub const Twips: PosPrinterMapMode = PosPrinterMapMode(1i32);
    pub const English: PosPrinterMapMode = PosPrinterMapMode(2i32);
    pub const Metric: PosPrinterMapMode = PosPrinterMapMode(3i32);
}
impl ::core::convert::From<i32> for PosPrinterMapMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PosPrinterMapMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PosPrinterMapMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterMapMode;i4)");
}
impl ::windows::core::DefaultType for PosPrinterMapMode {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PosPrinterMarkFeedCapabilities(pub u32);
impl PosPrinterMarkFeedCapabilities {
    pub const None: PosPrinterMarkFeedCapabilities = PosPrinterMarkFeedCapabilities(0u32);
    pub const ToTakeUp: PosPrinterMarkFeedCapabilities = PosPrinterMarkFeedCapabilities(1u32);
    pub const ToCutter: PosPrinterMarkFeedCapabilities = PosPrinterMarkFeedCapabilities(2u32);
    pub const ToCurrentTopOfForm: PosPrinterMarkFeedCapabilities = PosPrinterMarkFeedCapabilities(4u32);
    pub const ToNextTopOfForm: PosPrinterMarkFeedCapabilities = PosPrinterMarkFeedCapabilities(8u32);
}
impl ::core::convert::From<u32> for PosPrinterMarkFeedCapabilities {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PosPrinterMarkFeedCapabilities {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PosPrinterMarkFeedCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterMarkFeedCapabilities;u4)");
}
impl ::windows::core::DefaultType for PosPrinterMarkFeedCapabilities {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for PosPrinterMarkFeedCapabilities {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for PosPrinterMarkFeedCapabilities {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for PosPrinterMarkFeedCapabilities {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for PosPrinterMarkFeedCapabilities {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for PosPrinterMarkFeedCapabilities {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PosPrinterMarkFeedKind(pub i32);
impl PosPrinterMarkFeedKind {
    pub const ToTakeUp: PosPrinterMarkFeedKind = PosPrinterMarkFeedKind(0i32);
    pub const ToCutter: PosPrinterMarkFeedKind = PosPrinterMarkFeedKind(1i32);
    pub const ToCurrentTopOfForm: PosPrinterMarkFeedKind = PosPrinterMarkFeedKind(2i32);
    pub const ToNextTopOfForm: PosPrinterMarkFeedKind = PosPrinterMarkFeedKind(3i32);
}
impl ::core::convert::From<i32> for PosPrinterMarkFeedKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PosPrinterMarkFeedKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PosPrinterMarkFeedKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterMarkFeedKind;i4)");
}
impl ::windows::core::DefaultType for PosPrinterMarkFeedKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PosPrinterPrintOptions(pub ::windows::core::IInspectable);
impl PosPrinterPrintOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PosPrinterPrintOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn TypeFace(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetTypeFace<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CharacterHeight(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetCharacterHeight(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Bold(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetBold(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Italic(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetItalic(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Underline(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetUnderline(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ReverseVideo(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetReverseVideo(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Strikethrough(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetStrikethrough(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Superscript(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetSuperscript(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Subscript(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetSubscript(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DoubleWide(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetDoubleWide(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DoubleHigh(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetDoubleHigh(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Alignment(&self) -> ::windows::core::Result<PosPrinterAlignment> {
        let this = self;
        unsafe {
            let mut result__: PosPrinterAlignment = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterAlignment>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetAlignment(&self, value: PosPrinterAlignment) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CharacterSet(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetCharacterSet(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for PosPrinterPrintOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.PosPrinterPrintOptions;{0a2e16fd-1d02-5a58-9d59-bfcde76fde86})");
}
unsafe impl ::windows::core::Interface for PosPrinterPrintOptions {
    type Vtable = IPosPrinterPrintOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a2e16fd_1d02_5a58_9d59_bfcde76fde86);
}
impl ::windows::core::RuntimeName for PosPrinterPrintOptions {
    const NAME: &'static str = "Windows.Devices.PointOfService.PosPrinterPrintOptions";
}
impl ::core::convert::From<PosPrinterPrintOptions> for ::windows::core::IUnknown {
    fn from(value: PosPrinterPrintOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PosPrinterPrintOptions> for ::windows::core::IUnknown {
    fn from(value: &PosPrinterPrintOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PosPrinterPrintOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PosPrinterPrintOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PosPrinterPrintOptions> for ::windows::core::IInspectable {
    fn from(value: PosPrinterPrintOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PosPrinterPrintOptions> for ::windows::core::IInspectable {
    fn from(value: &PosPrinterPrintOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PosPrinterPrintOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PosPrinterPrintOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PosPrinterPrintOptions {}
unsafe impl ::core::marker::Sync for PosPrinterPrintOptions {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PosPrinterPrintSide(pub i32);
impl PosPrinterPrintSide {
    pub const Unknown: PosPrinterPrintSide = PosPrinterPrintSide(0i32);
    pub const Side1: PosPrinterPrintSide = PosPrinterPrintSide(1i32);
    pub const Side2: PosPrinterPrintSide = PosPrinterPrintSide(2i32);
}
impl ::core::convert::From<i32> for PosPrinterPrintSide {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PosPrinterPrintSide {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PosPrinterPrintSide {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterPrintSide;i4)");
}
impl ::windows::core::DefaultType for PosPrinterPrintSide {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PosPrinterReleaseDeviceRequestedEventArgs(pub ::windows::core::IInspectable);
impl PosPrinterReleaseDeviceRequestedEventArgs {}
unsafe impl ::windows::core::RuntimeType for PosPrinterReleaseDeviceRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.PosPrinterReleaseDeviceRequestedEventArgs;{2bcba359-1cef-40b2-9ecb-f927f856ae3c})");
}
unsafe impl ::windows::core::Interface for PosPrinterReleaseDeviceRequestedEventArgs {
    type Vtable = IPosPrinterReleaseDeviceRequestedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2bcba359_1cef_40b2_9ecb_f927f856ae3c);
}
impl ::windows::core::RuntimeName for PosPrinterReleaseDeviceRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.PosPrinterReleaseDeviceRequestedEventArgs";
}
impl ::core::convert::From<PosPrinterReleaseDeviceRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PosPrinterReleaseDeviceRequestedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PosPrinterReleaseDeviceRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PosPrinterReleaseDeviceRequestedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PosPrinterReleaseDeviceRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PosPrinterReleaseDeviceRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PosPrinterReleaseDeviceRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PosPrinterReleaseDeviceRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PosPrinterReleaseDeviceRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PosPrinterReleaseDeviceRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PosPrinterReleaseDeviceRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PosPrinterReleaseDeviceRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PosPrinterReleaseDeviceRequestedEventArgs {}
unsafe impl ::core::marker::Sync for PosPrinterReleaseDeviceRequestedEventArgs {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PosPrinterRotation(pub i32);
impl PosPrinterRotation {
    pub const Normal: PosPrinterRotation = PosPrinterRotation(0i32);
    pub const Right90: PosPrinterRotation = PosPrinterRotation(1i32);
    pub const Left90: PosPrinterRotation = PosPrinterRotation(2i32);
    pub const Rotate180: PosPrinterRotation = PosPrinterRotation(3i32);
}
impl ::core::convert::From<i32> for PosPrinterRotation {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PosPrinterRotation {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PosPrinterRotation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterRotation;i4)");
}
impl ::windows::core::DefaultType for PosPrinterRotation {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PosPrinterRuledLineCapabilities(pub u32);
impl PosPrinterRuledLineCapabilities {
    pub const None: PosPrinterRuledLineCapabilities = PosPrinterRuledLineCapabilities(0u32);
    pub const Horizontal: PosPrinterRuledLineCapabilities = PosPrinterRuledLineCapabilities(1u32);
    pub const Vertical: PosPrinterRuledLineCapabilities = PosPrinterRuledLineCapabilities(2u32);
}
impl ::core::convert::From<u32> for PosPrinterRuledLineCapabilities {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PosPrinterRuledLineCapabilities {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PosPrinterRuledLineCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterRuledLineCapabilities;u4)");
}
impl ::windows::core::DefaultType for PosPrinterRuledLineCapabilities {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for PosPrinterRuledLineCapabilities {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for PosPrinterRuledLineCapabilities {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for PosPrinterRuledLineCapabilities {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for PosPrinterRuledLineCapabilities {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for PosPrinterRuledLineCapabilities {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PosPrinterStatus(pub ::windows::core::IInspectable);
impl PosPrinterStatus {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn StatusKind(&self) -> ::windows::core::Result<PosPrinterStatusKind> {
        let this = self;
        unsafe {
            let mut result__: PosPrinterStatusKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterStatusKind>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ExtendedStatus(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PosPrinterStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.PosPrinterStatus;{d1f0c730-da40-4328-bf76-5156fa33b747})");
}
unsafe impl ::windows::core::Interface for PosPrinterStatus {
    type Vtable = IPosPrinterStatus_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd1f0c730_da40_4328_bf76_5156fa33b747);
}
impl ::windows::core::RuntimeName for PosPrinterStatus {
    const NAME: &'static str = "Windows.Devices.PointOfService.PosPrinterStatus";
}
impl ::core::convert::From<PosPrinterStatus> for ::windows::core::IUnknown {
    fn from(value: PosPrinterStatus) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PosPrinterStatus> for ::windows::core::IUnknown {
    fn from(value: &PosPrinterStatus) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PosPrinterStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PosPrinterStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PosPrinterStatus> for ::windows::core::IInspectable {
    fn from(value: PosPrinterStatus) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PosPrinterStatus> for ::windows::core::IInspectable {
    fn from(value: &PosPrinterStatus) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PosPrinterStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PosPrinterStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PosPrinterStatus {}
unsafe impl ::core::marker::Sync for PosPrinterStatus {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PosPrinterStatusKind(pub i32);
impl PosPrinterStatusKind {
    pub const Online: PosPrinterStatusKind = PosPrinterStatusKind(0i32);
    pub const Off: PosPrinterStatusKind = PosPrinterStatusKind(1i32);
    pub const Offline: PosPrinterStatusKind = PosPrinterStatusKind(2i32);
    pub const OffOrOffline: PosPrinterStatusKind = PosPrinterStatusKind(3i32);
    pub const Extended: PosPrinterStatusKind = PosPrinterStatusKind(4i32);
}
impl ::core::convert::From<i32> for PosPrinterStatusKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PosPrinterStatusKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PosPrinterStatusKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterStatusKind;i4)");
}
impl ::windows::core::DefaultType for PosPrinterStatusKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PosPrinterStatusUpdatedEventArgs(pub ::windows::core::IInspectable);
impl PosPrinterStatusUpdatedEventArgs {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Status(&self) -> ::windows::core::Result<PosPrinterStatus> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterStatus>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PosPrinterStatusUpdatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.PosPrinterStatusUpdatedEventArgs;{2edb87df-13a6-428d-ba81-b0e7c3e5a3cd})");
}
unsafe impl ::windows::core::Interface for PosPrinterStatusUpdatedEventArgs {
    type Vtable = IPosPrinterStatusUpdatedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2edb87df_13a6_428d_ba81_b0e7c3e5a3cd);
}
impl ::windows::core::RuntimeName for PosPrinterStatusUpdatedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.PosPrinterStatusUpdatedEventArgs";
}
impl ::core::convert::From<PosPrinterStatusUpdatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PosPrinterStatusUpdatedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PosPrinterStatusUpdatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PosPrinterStatusUpdatedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PosPrinterStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PosPrinterStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PosPrinterStatusUpdatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PosPrinterStatusUpdatedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PosPrinterStatusUpdatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PosPrinterStatusUpdatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PosPrinterStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PosPrinterStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PosPrinterStatusUpdatedEventArgs {}
unsafe impl ::core::marker::Sync for PosPrinterStatusUpdatedEventArgs {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ReceiptPrintJob(pub ::windows::core::IInspectable);
impl ReceiptPrintJob {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn MarkFeed(&self, kind: PosPrinterMarkFeedKind) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), kind).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CutPaper(&self, percentage: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), percentage).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CutPaperDefault(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Print<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, data: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PrintLine<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, data: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PrintNewline(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn ExecuteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetBarcodeRotation(&self, value: PosPrinterRotation) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetPrintRotation(&self, value: PosPrinterRotation, includebitmaps: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value, includebitmaps).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn SetPrintArea<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Devices_PointOfService`, `Graphics_Imaging`*"]
    pub fn SetBitmap<'a, Param1: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmapnumber: u32, bitmap: Param1, alignment: PosPrinterAlignment) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), bitmapnumber, bitmap.into_param().abi(), alignment).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Devices_PointOfService`, `Graphics_Imaging`*"]
    pub fn SetBitmapCustomWidthStandardAlign<'a, Param1: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmapnumber: u32, bitmap: Param1, alignment: PosPrinterAlignment, width: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), bitmapnumber, bitmap.into_param().abi(), alignment, width).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Devices_PointOfService`, `Graphics_Imaging`*"]
    pub fn SetCustomAlignedBitmap<'a, Param1: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmapnumber: u32, bitmap: Param1, alignmentdistance: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), bitmapnumber, bitmap.into_param().abi(), alignmentdistance).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Devices_PointOfService`, `Graphics_Imaging`*"]
    pub fn SetBitmapCustomWidthCustomAlign<'a, Param1: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmapnumber: u32, bitmap: Param1, alignmentdistance: u32, width: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), bitmapnumber, bitmap.into_param().abi(), alignmentdistance, width).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PrintSavedBitmap(&self, bitmapnumber: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), bitmapnumber).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DrawRuledLine<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, positionlist: Param0, linedirection: PosPrinterLineDirection, linewidth: u32, linestyle: PosPrinterLineStyle, linecolor: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), positionlist.into_param().abi(), linedirection, linewidth, linestyle, linecolor).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PrintBarcode<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, data: Param0, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignment: PosPrinterAlignment) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), data.into_param().abi(), symbology, height, width, textposition, alignment).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PrintBarcodeCustomAlign<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, data: Param0, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignmentdistance: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), data.into_param().abi(), symbology, height, width, textposition, alignmentdistance).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Devices_PointOfService`, `Graphics_Imaging`*"]
    pub fn PrintBitmap<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmap: Param0, alignment: PosPrinterAlignment) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), bitmap.into_param().abi(), alignment).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Devices_PointOfService`, `Graphics_Imaging`*"]
    pub fn PrintBitmapCustomWidthStandardAlign<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmap: Param0, alignment: PosPrinterAlignment, width: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), bitmap.into_param().abi(), alignment, width).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Devices_PointOfService`, `Graphics_Imaging`*"]
    pub fn PrintCustomAlignedBitmap<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmap: Param0, alignmentdistance: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), bitmap.into_param().abi(), alignmentdistance).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Devices_PointOfService`, `Graphics_Imaging`*"]
    pub fn PrintBitmapCustomWidthCustomAlign<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmap: Param0, alignmentdistance: u32, width: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), bitmap.into_param().abi(), alignmentdistance, width).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn StampPaper(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptPrintJob2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Print2<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, PosPrinterPrintOptions>>(&self, data: Param0, printoptions: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptPrintJob2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), data.into_param().abi(), printoptions.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn FeedPaperByLine(&self, linecount: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptPrintJob2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), linecount).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn FeedPaperByMapModeUnit(&self, distance: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptPrintJob2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), distance).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for ReceiptPrintJob {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ReceiptPrintJob;{aa96066e-acad-4b79-9d0f-c0cfc08dc77b})");
}
unsafe impl ::windows::core::Interface for ReceiptPrintJob {
    type Vtable = IReceiptPrintJob_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa96066e_acad_4b79_9d0f_c0cfc08dc77b);
}
impl ::windows::core::RuntimeName for ReceiptPrintJob {
    const NAME: &'static str = "Windows.Devices.PointOfService.ReceiptPrintJob";
}
impl ::core::convert::From<ReceiptPrintJob> for ::windows::core::IUnknown {
    fn from(value: ReceiptPrintJob) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ReceiptPrintJob> for ::windows::core::IUnknown {
    fn from(value: &ReceiptPrintJob) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ReceiptPrintJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ReceiptPrintJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ReceiptPrintJob> for ::windows::core::IInspectable {
    fn from(value: ReceiptPrintJob) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ReceiptPrintJob> for ::windows::core::IInspectable {
    fn from(value: &ReceiptPrintJob) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ReceiptPrintJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ReceiptPrintJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<ReceiptPrintJob> for IPosPrinterJob {
    type Error = ::windows::core::Error;
    fn try_from(value: ReceiptPrintJob) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ReceiptPrintJob> for IPosPrinterJob {
    type Error = ::windows::core::Error;
    fn try_from(value: &ReceiptPrintJob) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPosPrinterJob> for ReceiptPrintJob {
    fn into_param(self) -> ::windows::core::Param<'a, IPosPrinterJob> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPosPrinterJob> for &ReceiptPrintJob {
    fn into_param(self) -> ::windows::core::Param<'a, IPosPrinterJob> {
        ::core::convert::TryInto::<IPosPrinterJob>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ReceiptPrintJob> for IReceiptOrSlipJob {
    type Error = ::windows::core::Error;
    fn try_from(value: ReceiptPrintJob) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ReceiptPrintJob> for IReceiptOrSlipJob {
    type Error = ::windows::core::Error;
    fn try_from(value: &ReceiptPrintJob) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IReceiptOrSlipJob> for ReceiptPrintJob {
    fn into_param(self) -> ::windows::core::Param<'a, IReceiptOrSlipJob> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IReceiptOrSlipJob> for &ReceiptPrintJob {
    fn into_param(self) -> ::windows::core::Param<'a, IReceiptOrSlipJob> {
        ::core::convert::TryInto::<IReceiptOrSlipJob>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ReceiptPrintJob {}
unsafe impl ::core::marker::Sync for ReceiptPrintJob {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ReceiptPrinterCapabilities(pub ::windows::core::IInspectable);
impl ReceiptPrinterCapabilities {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CanCutPaper(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsStampSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn MarkFeedCapabilities(&self) -> ::windows::core::Result<PosPrinterMarkFeedCapabilities> {
        let this = self;
        unsafe {
            let mut result__: PosPrinterMarkFeedCapabilities = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterMarkFeedCapabilities>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsPrinterPresent(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDualColorSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ColorCartridgeCapabilities(&self) -> ::windows::core::Result<PosPrinterColorCapabilities> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: PosPrinterColorCapabilities = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterColorCapabilities>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CartridgeSensors(&self) -> ::windows::core::Result<PosPrinterCartridgeSensors> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: PosPrinterCartridgeSensors = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterCartridgeSensors>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsBoldSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsItalicSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsUnderlineSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDoubleHighPrintSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDoubleWidePrintSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDoubleHighDoubleWidePrintSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsPaperEmptySensorSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsPaperNearEndSensorSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation_Collections`*"]
    pub fn SupportedCharactersPerLine(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsBarcodeSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsBitmapSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsLeft90RotationSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsRight90RotationSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Is180RotationSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsPrintAreaSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn RuledLineCapabilities(&self) -> ::windows::core::Result<PosPrinterRuledLineCapabilities> {
        let this = &::windows::core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__: PosPrinterRuledLineCapabilities = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterRuledLineCapabilities>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation_Collections`*"]
    pub fn SupportedBarcodeRotations(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>> {
        let this = &::windows::core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation_Collections`*"]
    pub fn SupportedBitmapRotations(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>> {
        let this = &::windows::core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsReverseVideoSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IReceiptPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsStrikethroughSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IReceiptPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsSuperscriptSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IReceiptPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsSubscriptSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IReceiptPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsReversePaperFeedByLineSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IReceiptPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsReversePaperFeedByMapModeUnitSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IReceiptPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for ReceiptPrinterCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ReceiptPrinterCapabilities;{b8f0b58f-51a8-43fc-9bd5-8de272a6415b})");
}
unsafe impl ::windows::core::Interface for ReceiptPrinterCapabilities {
    type Vtable = IReceiptPrinterCapabilities_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb8f0b58f_51a8_43fc_9bd5_8de272a6415b);
}
impl ::windows::core::RuntimeName for ReceiptPrinterCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.ReceiptPrinterCapabilities";
}
impl ::core::convert::From<ReceiptPrinterCapabilities> for ::windows::core::IUnknown {
    fn from(value: ReceiptPrinterCapabilities) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ReceiptPrinterCapabilities> for ::windows::core::IUnknown {
    fn from(value: &ReceiptPrinterCapabilities) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ReceiptPrinterCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ReceiptPrinterCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ReceiptPrinterCapabilities> for ::windows::core::IInspectable {
    fn from(value: ReceiptPrinterCapabilities) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ReceiptPrinterCapabilities> for ::windows::core::IInspectable {
    fn from(value: &ReceiptPrinterCapabilities) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ReceiptPrinterCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ReceiptPrinterCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<ReceiptPrinterCapabilities> for ICommonPosPrintStationCapabilities {
    type Error = ::windows::core::Error;
    fn try_from(value: ReceiptPrinterCapabilities) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ReceiptPrinterCapabilities> for ICommonPosPrintStationCapabilities {
    type Error = ::windows::core::Error;
    fn try_from(value: &ReceiptPrinterCapabilities) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICommonPosPrintStationCapabilities> for ReceiptPrinterCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ICommonPosPrintStationCapabilities> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICommonPosPrintStationCapabilities> for &ReceiptPrinterCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ICommonPosPrintStationCapabilities> {
        ::core::convert::TryInto::<ICommonPosPrintStationCapabilities>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<ReceiptPrinterCapabilities> for ICommonReceiptSlipCapabilities {
    type Error = ::windows::core::Error;
    fn try_from(value: ReceiptPrinterCapabilities) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ReceiptPrinterCapabilities> for ICommonReceiptSlipCapabilities {
    type Error = ::windows::core::Error;
    fn try_from(value: &ReceiptPrinterCapabilities) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICommonReceiptSlipCapabilities> for ReceiptPrinterCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ICommonReceiptSlipCapabilities> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICommonReceiptSlipCapabilities> for &ReceiptPrinterCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ICommonReceiptSlipCapabilities> {
        ::core::convert::TryInto::<ICommonReceiptSlipCapabilities>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for ReceiptPrinterCapabilities {}
unsafe impl ::core::marker::Sync for ReceiptPrinterCapabilities {}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Devices_PointOfService`*"]
pub struct SizeUInt32 {
    pub Width: u32,
    pub Height: u32,
}
impl SizeUInt32 {}
impl ::core::default::Default for SizeUInt32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for SizeUInt32 {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SizeUInt32").field("Width", &self.Width).field("Height", &self.Height).finish()
    }
}
impl ::core::cmp::PartialEq for SizeUInt32 {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width && self.Height == other.Height
    }
}
impl ::core::cmp::Eq for SizeUInt32 {}
unsafe impl ::windows::core::Abi for SizeUInt32 {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for SizeUInt32 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Devices.PointOfService.SizeUInt32;u4;u4)");
}
impl ::windows::core::DefaultType for SizeUInt32 {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SlipPrintJob(pub ::windows::core::IInspectable);
impl SlipPrintJob {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetBarcodeRotation(&self, value: PosPrinterRotation) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetPrintRotation(&self, value: PosPrinterRotation, includebitmaps: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value, includebitmaps).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn SetPrintArea<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Devices_PointOfService`, `Graphics_Imaging`*"]
    pub fn SetBitmap<'a, Param1: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmapnumber: u32, bitmap: Param1, alignment: PosPrinterAlignment) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), bitmapnumber, bitmap.into_param().abi(), alignment).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Devices_PointOfService`, `Graphics_Imaging`*"]
    pub fn SetBitmapCustomWidthStandardAlign<'a, Param1: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmapnumber: u32, bitmap: Param1, alignment: PosPrinterAlignment, width: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), bitmapnumber, bitmap.into_param().abi(), alignment, width).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Devices_PointOfService`, `Graphics_Imaging`*"]
    pub fn SetCustomAlignedBitmap<'a, Param1: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmapnumber: u32, bitmap: Param1, alignmentdistance: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), bitmapnumber, bitmap.into_param().abi(), alignmentdistance).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Devices_PointOfService`, `Graphics_Imaging`*"]
    pub fn SetBitmapCustomWidthCustomAlign<'a, Param1: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmapnumber: u32, bitmap: Param1, alignmentdistance: u32, width: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), bitmapnumber, bitmap.into_param().abi(), alignmentdistance, width).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PrintSavedBitmap(&self, bitmapnumber: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), bitmapnumber).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DrawRuledLine<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, positionlist: Param0, linedirection: PosPrinterLineDirection, linewidth: u32, linestyle: PosPrinterLineStyle, linecolor: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), positionlist.into_param().abi(), linedirection, linewidth, linestyle, linecolor).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PrintBarcode<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, data: Param0, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignment: PosPrinterAlignment) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), data.into_param().abi(), symbology, height, width, textposition, alignment).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PrintBarcodeCustomAlign<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, data: Param0, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignmentdistance: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), data.into_param().abi(), symbology, height, width, textposition, alignmentdistance).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Devices_PointOfService`, `Graphics_Imaging`*"]
    pub fn PrintBitmap<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmap: Param0, alignment: PosPrinterAlignment) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), bitmap.into_param().abi(), alignment).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Devices_PointOfService`, `Graphics_Imaging`*"]
    pub fn PrintBitmapCustomWidthStandardAlign<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmap: Param0, alignment: PosPrinterAlignment, width: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), bitmap.into_param().abi(), alignment, width).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Devices_PointOfService`, `Graphics_Imaging`*"]
    pub fn PrintCustomAlignedBitmap<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmap: Param0, alignmentdistance: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), bitmap.into_param().abi(), alignmentdistance).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Devices_PointOfService`, `Graphics_Imaging`*"]
    pub fn PrintBitmapCustomWidthCustomAlign<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmap: Param0, alignmentdistance: u32, width: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), bitmap.into_param().abi(), alignmentdistance, width).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Print<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, data: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PrintLine<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, data: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PrintNewline(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn ExecuteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Print2<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, PosPrinterPrintOptions>>(&self, data: Param0, printoptions: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISlipPrintJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), data.into_param().abi(), printoptions.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn FeedPaperByLine(&self, linecount: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISlipPrintJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), linecount).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn FeedPaperByMapModeUnit(&self, distance: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISlipPrintJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), distance).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for SlipPrintJob {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.SlipPrintJob;{532199be-c8c3-4dc2-89e9-5c4a37b34ddc})");
}
unsafe impl ::windows::core::Interface for SlipPrintJob {
    type Vtable = IReceiptOrSlipJob_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x532199be_c8c3_4dc2_89e9_5c4a37b34ddc);
}
impl ::windows::core::RuntimeName for SlipPrintJob {
    const NAME: &'static str = "Windows.Devices.PointOfService.SlipPrintJob";
}
impl ::core::convert::From<SlipPrintJob> for ::windows::core::IUnknown {
    fn from(value: SlipPrintJob) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SlipPrintJob> for ::windows::core::IUnknown {
    fn from(value: &SlipPrintJob) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SlipPrintJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SlipPrintJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SlipPrintJob> for ::windows::core::IInspectable {
    fn from(value: SlipPrintJob) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SlipPrintJob> for ::windows::core::IInspectable {
    fn from(value: &SlipPrintJob) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SlipPrintJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SlipPrintJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<SlipPrintJob> for IReceiptOrSlipJob {
    fn from(value: SlipPrintJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SlipPrintJob> for IReceiptOrSlipJob {
    fn from(value: &SlipPrintJob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IReceiptOrSlipJob> for SlipPrintJob {
    fn into_param(self) -> ::windows::core::Param<'a, IReceiptOrSlipJob> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IReceiptOrSlipJob> for &SlipPrintJob {
    fn into_param(self) -> ::windows::core::Param<'a, IReceiptOrSlipJob> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<SlipPrintJob> for IPosPrinterJob {
    type Error = ::windows::core::Error;
    fn try_from(value: SlipPrintJob) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SlipPrintJob> for IPosPrinterJob {
    type Error = ::windows::core::Error;
    fn try_from(value: &SlipPrintJob) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPosPrinterJob> for SlipPrintJob {
    fn into_param(self) -> ::windows::core::Param<'a, IPosPrinterJob> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPosPrinterJob> for &SlipPrintJob {
    fn into_param(self) -> ::windows::core::Param<'a, IPosPrinterJob> {
        ::core::convert::TryInto::<IPosPrinterJob>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SlipPrintJob {}
unsafe impl ::core::marker::Sync for SlipPrintJob {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SlipPrinterCapabilities(pub ::windows::core::IInspectable);
impl SlipPrinterCapabilities {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsFullLengthSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsBothSidesPrintingSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsPrinterPresent(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDualColorSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ColorCartridgeCapabilities(&self) -> ::windows::core::Result<PosPrinterColorCapabilities> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: PosPrinterColorCapabilities = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterColorCapabilities>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CartridgeSensors(&self) -> ::windows::core::Result<PosPrinterCartridgeSensors> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: PosPrinterCartridgeSensors = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterCartridgeSensors>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsBoldSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsItalicSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsUnderlineSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDoubleHighPrintSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDoubleWidePrintSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDoubleHighDoubleWidePrintSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsPaperEmptySensorSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsPaperNearEndSensorSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation_Collections`*"]
    pub fn SupportedCharactersPerLine(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsBarcodeSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsBitmapSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsLeft90RotationSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsRight90RotationSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Is180RotationSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsPrintAreaSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn RuledLineCapabilities(&self) -> ::windows::core::Result<PosPrinterRuledLineCapabilities> {
        let this = &::windows::core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__: PosPrinterRuledLineCapabilities = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterRuledLineCapabilities>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation_Collections`*"]
    pub fn SupportedBarcodeRotations(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>> {
        let this = &::windows::core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation_Collections`*"]
    pub fn SupportedBitmapRotations(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>> {
        let this = &::windows::core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsReverseVideoSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ISlipPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsStrikethroughSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ISlipPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsSuperscriptSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ISlipPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsSubscriptSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ISlipPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsReversePaperFeedByLineSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ISlipPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsReversePaperFeedByMapModeUnitSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ISlipPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for SlipPrinterCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.SlipPrinterCapabilities;{99b16399-488c-4157-8ac2-9f57f708d3db})");
}
unsafe impl ::windows::core::Interface for SlipPrinterCapabilities {
    type Vtable = ISlipPrinterCapabilities_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x99b16399_488c_4157_8ac2_9f57f708d3db);
}
impl ::windows::core::RuntimeName for SlipPrinterCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.SlipPrinterCapabilities";
}
impl ::core::convert::From<SlipPrinterCapabilities> for ::windows::core::IUnknown {
    fn from(value: SlipPrinterCapabilities) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SlipPrinterCapabilities> for ::windows::core::IUnknown {
    fn from(value: &SlipPrinterCapabilities) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SlipPrinterCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SlipPrinterCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SlipPrinterCapabilities> for ::windows::core::IInspectable {
    fn from(value: SlipPrinterCapabilities) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SlipPrinterCapabilities> for ::windows::core::IInspectable {
    fn from(value: &SlipPrinterCapabilities) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SlipPrinterCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SlipPrinterCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<SlipPrinterCapabilities> for ICommonPosPrintStationCapabilities {
    type Error = ::windows::core::Error;
    fn try_from(value: SlipPrinterCapabilities) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SlipPrinterCapabilities> for ICommonPosPrintStationCapabilities {
    type Error = ::windows::core::Error;
    fn try_from(value: &SlipPrinterCapabilities) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICommonPosPrintStationCapabilities> for SlipPrinterCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ICommonPosPrintStationCapabilities> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICommonPosPrintStationCapabilities> for &SlipPrinterCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ICommonPosPrintStationCapabilities> {
        ::core::convert::TryInto::<ICommonPosPrintStationCapabilities>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<SlipPrinterCapabilities> for ICommonReceiptSlipCapabilities {
    type Error = ::windows::core::Error;
    fn try_from(value: SlipPrinterCapabilities) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SlipPrinterCapabilities> for ICommonReceiptSlipCapabilities {
    type Error = ::windows::core::Error;
    fn try_from(value: &SlipPrinterCapabilities) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICommonReceiptSlipCapabilities> for SlipPrinterCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ICommonReceiptSlipCapabilities> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICommonReceiptSlipCapabilities> for &SlipPrinterCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ICommonReceiptSlipCapabilities> {
        ::core::convert::TryInto::<ICommonReceiptSlipCapabilities>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SlipPrinterCapabilities {}
unsafe impl ::core::marker::Sync for SlipPrinterCapabilities {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct UnifiedPosErrorData(pub ::windows::core::IInspectable);
impl UnifiedPosErrorData {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Message(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Severity(&self) -> ::windows::core::Result<UnifiedPosErrorSeverity> {
        let this = self;
        unsafe {
            let mut result__: UnifiedPosErrorSeverity = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UnifiedPosErrorSeverity>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Reason(&self) -> ::windows::core::Result<UnifiedPosErrorReason> {
        let this = self;
        unsafe {
            let mut result__: UnifiedPosErrorReason = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UnifiedPosErrorReason>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ExtendedReason(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CreateInstance<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(message: Param0, severity: UnifiedPosErrorSeverity, reason: UnifiedPosErrorReason, extendedreason: u32) -> ::windows::core::Result<UnifiedPosErrorData> {
        Self::IUnifiedPosErrorDataFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), message.into_param().abi(), severity, reason, extendedreason, &mut result__).from_abi::<UnifiedPosErrorData>(result__)
        })
    }
    pub fn IUnifiedPosErrorDataFactory<R, F: FnOnce(&IUnifiedPosErrorDataFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UnifiedPosErrorData, IUnifiedPosErrorDataFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for UnifiedPosErrorData {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.UnifiedPosErrorData;{2b998c3a-555c-4889-8ed8-c599bb3a712a})");
}
unsafe impl ::windows::core::Interface for UnifiedPosErrorData {
    type Vtable = IUnifiedPosErrorData_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2b998c3a_555c_4889_8ed8_c599bb3a712a);
}
impl ::windows::core::RuntimeName for UnifiedPosErrorData {
    const NAME: &'static str = "Windows.Devices.PointOfService.UnifiedPosErrorData";
}
impl ::core::convert::From<UnifiedPosErrorData> for ::windows::core::IUnknown {
    fn from(value: UnifiedPosErrorData) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&UnifiedPosErrorData> for ::windows::core::IUnknown {
    fn from(value: &UnifiedPosErrorData) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UnifiedPosErrorData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UnifiedPosErrorData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<UnifiedPosErrorData> for ::windows::core::IInspectable {
    fn from(value: UnifiedPosErrorData) -> Self {
        value.0
    }
}
impl ::core::convert::From<&UnifiedPosErrorData> for ::windows::core::IInspectable {
    fn from(value: &UnifiedPosErrorData) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UnifiedPosErrorData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UnifiedPosErrorData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for UnifiedPosErrorData {}
unsafe impl ::core::marker::Sync for UnifiedPosErrorData {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct UnifiedPosErrorReason(pub i32);
impl UnifiedPosErrorReason {
    pub const UnknownErrorReason: UnifiedPosErrorReason = UnifiedPosErrorReason(0i32);
    pub const NoService: UnifiedPosErrorReason = UnifiedPosErrorReason(1i32);
    pub const Disabled: UnifiedPosErrorReason = UnifiedPosErrorReason(2i32);
    pub const Illegal: UnifiedPosErrorReason = UnifiedPosErrorReason(3i32);
    pub const NoHardware: UnifiedPosErrorReason = UnifiedPosErrorReason(4i32);
    pub const Closed: UnifiedPosErrorReason = UnifiedPosErrorReason(5i32);
    pub const Offline: UnifiedPosErrorReason = UnifiedPosErrorReason(6i32);
    pub const Failure: UnifiedPosErrorReason = UnifiedPosErrorReason(7i32);
    pub const Timeout: UnifiedPosErrorReason = UnifiedPosErrorReason(8i32);
    pub const Busy: UnifiedPosErrorReason = UnifiedPosErrorReason(9i32);
    pub const Extended: UnifiedPosErrorReason = UnifiedPosErrorReason(10i32);
}
impl ::core::convert::From<i32> for UnifiedPosErrorReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for UnifiedPosErrorReason {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for UnifiedPosErrorReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.UnifiedPosErrorReason;i4)");
}
impl ::windows::core::DefaultType for UnifiedPosErrorReason {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct UnifiedPosErrorSeverity(pub i32);
impl UnifiedPosErrorSeverity {
    pub const UnknownErrorSeverity: UnifiedPosErrorSeverity = UnifiedPosErrorSeverity(0i32);
    pub const Warning: UnifiedPosErrorSeverity = UnifiedPosErrorSeverity(1i32);
    pub const Recoverable: UnifiedPosErrorSeverity = UnifiedPosErrorSeverity(2i32);
    pub const Unrecoverable: UnifiedPosErrorSeverity = UnifiedPosErrorSeverity(3i32);
    pub const AssistanceRequired: UnifiedPosErrorSeverity = UnifiedPosErrorSeverity(4i32);
    pub const Fatal: UnifiedPosErrorSeverity = UnifiedPosErrorSeverity(5i32);
}
impl ::core::convert::From<i32> for UnifiedPosErrorSeverity {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for UnifiedPosErrorSeverity {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for UnifiedPosErrorSeverity {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.UnifiedPosErrorSeverity;i4)");
}
impl ::windows::core::DefaultType for UnifiedPosErrorSeverity {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct UnifiedPosHealthCheckLevel(pub i32);
impl UnifiedPosHealthCheckLevel {
    pub const UnknownHealthCheckLevel: UnifiedPosHealthCheckLevel = UnifiedPosHealthCheckLevel(0i32);
    pub const POSInternal: UnifiedPosHealthCheckLevel = UnifiedPosHealthCheckLevel(1i32);
    pub const External: UnifiedPosHealthCheckLevel = UnifiedPosHealthCheckLevel(2i32);
    pub const Interactive: UnifiedPosHealthCheckLevel = UnifiedPosHealthCheckLevel(3i32);
}
impl ::core::convert::From<i32> for UnifiedPosHealthCheckLevel {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for UnifiedPosHealthCheckLevel {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for UnifiedPosHealthCheckLevel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.UnifiedPosHealthCheckLevel;i4)");
}
impl ::windows::core::DefaultType for UnifiedPosHealthCheckLevel {
    type DefaultType = Self;
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct UnifiedPosPowerReportingType(pub i32);
impl UnifiedPosPowerReportingType {
    pub const UnknownPowerReportingType: UnifiedPosPowerReportingType = UnifiedPosPowerReportingType(0i32);
    pub const Standard: UnifiedPosPowerReportingType = UnifiedPosPowerReportingType(1i32);
    pub const Advanced: UnifiedPosPowerReportingType = UnifiedPosPowerReportingType(2i32);
}
impl ::core::convert::From<i32> for UnifiedPosPowerReportingType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for UnifiedPosPowerReportingType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for UnifiedPosPowerReportingType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.UnifiedPosPowerReportingType;i4)");
}
impl ::windows::core::DefaultType for UnifiedPosPowerReportingType {
    type DefaultType = Self;
}

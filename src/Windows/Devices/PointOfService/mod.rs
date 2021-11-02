#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Devices_PointOfService_Provider")]
pub mod Provider;
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct BarcodeScanner(::windows::runtime::IInspectable);
impl BarcodeScanner {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Capabilities(&self) -> ::windows::runtime::Result<BarcodeScannerCapabilities> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BarcodeScannerCapabilities>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn ClaimScannerAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ClaimedBarcodeScanner>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ClaimedBarcodeScanner>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn CheckHealthAsync(&self, level: UnifiedPosHealthCheckLevel) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), level, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetSupportedSymbologiesAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<u32>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<u32>>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn IsSymbologySupportedAsync(&self, barcodesymbology: u32) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), barcodesymbology, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`, `Foundation_Collections`, `Storage_Streams`*"]
    pub fn RetrieveStatisticsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(&self, statisticscategories: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), statisticscategories.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation_Collections`*"]
    pub fn GetSupportedProfiles(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsProfileSupported<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, profile: Param0) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), profile.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn StatusUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<BarcodeScanner, BarcodeScannerStatusUpdatedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveStatusUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn GetDefaultAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<BarcodeScanner>> {
        Self::IBarcodeScannerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BarcodeScanner>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn FromIdAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(deviceid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<BarcodeScanner>> {
        Self::IBarcodeScannerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BarcodeScanner>>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn GetDeviceSelector() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IBarcodeScannerStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn VideoDeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IBarcodeScanner2>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn GetDeviceSelectorWithConnectionTypes(connectiontypes: PosConnectionTypes) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IBarcodeScannerStatics2(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), connectiontypes, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn IBarcodeScannerStatics<R, F: FnOnce(&IBarcodeScannerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BarcodeScanner, IBarcodeScannerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IBarcodeScannerStatics2<R, F: FnOnce(&IBarcodeScannerStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BarcodeScanner, IBarcodeScannerStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BarcodeScanner {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.BarcodeScanner;{bea33e06-b264-4f03-a9c1-45b20f01134f})");
}
unsafe impl ::windows::runtime::Interface for BarcodeScanner {
    type Vtable = IBarcodeScanner_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3198369286, 45668, 20227, [169, 193, 69, 178, 15, 1, 19, 79]);
}
impl ::windows::runtime::RuntimeName for BarcodeScanner {
    const NAME: &'static str = "Windows.Devices.PointOfService.BarcodeScanner";
}
impl ::std::convert::From<BarcodeScanner> for ::windows::runtime::IUnknown {
    fn from(value: BarcodeScanner) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&BarcodeScanner> for ::windows::runtime::IUnknown {
    fn from(value: &BarcodeScanner) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for BarcodeScanner {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &BarcodeScanner {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<BarcodeScanner> for ::windows::runtime::IInspectable {
    fn from(value: BarcodeScanner) -> Self {
        value.0
    }
}
impl ::std::convert::From<&BarcodeScanner> for ::windows::runtime::IInspectable {
    fn from(value: &BarcodeScanner) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for BarcodeScanner {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a BarcodeScanner {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<BarcodeScanner> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: BarcodeScanner) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&BarcodeScanner> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &BarcodeScanner) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for BarcodeScanner {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &BarcodeScanner {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for BarcodeScanner {}
unsafe impl ::std::marker::Sync for BarcodeScanner {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct BarcodeScannerCapabilities(::windows::runtime::IInspectable);
impl BarcodeScannerCapabilities {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PowerReportingType(&self) -> ::windows::runtime::Result<UnifiedPosPowerReportingType> {
        let this = self;
        unsafe {
            let mut result__: UnifiedPosPowerReportingType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UnifiedPosPowerReportingType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsStatisticsReportingSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsStatisticsUpdatingSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsImagePreviewSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsSoftwareTriggerSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IBarcodeScannerCapabilities1>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsVideoPreviewSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IBarcodeScannerCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BarcodeScannerCapabilities {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.BarcodeScannerCapabilities;{c60691e4-f2c8-4420-a307-b12ef6622857})");
}
unsafe impl ::windows::runtime::Interface for BarcodeScannerCapabilities {
    type Vtable = IBarcodeScannerCapabilities_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3322319332, 62152, 17440, [163, 7, 177, 46, 246, 98, 40, 87]);
}
impl ::windows::runtime::RuntimeName for BarcodeScannerCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.BarcodeScannerCapabilities";
}
impl ::std::convert::From<BarcodeScannerCapabilities> for ::windows::runtime::IUnknown {
    fn from(value: BarcodeScannerCapabilities) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&BarcodeScannerCapabilities> for ::windows::runtime::IUnknown {
    fn from(value: &BarcodeScannerCapabilities) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for BarcodeScannerCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &BarcodeScannerCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<BarcodeScannerCapabilities> for ::windows::runtime::IInspectable {
    fn from(value: BarcodeScannerCapabilities) -> Self {
        value.0
    }
}
impl ::std::convert::From<&BarcodeScannerCapabilities> for ::windows::runtime::IInspectable {
    fn from(value: &BarcodeScannerCapabilities) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for BarcodeScannerCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a BarcodeScannerCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for BarcodeScannerCapabilities {}
unsafe impl ::std::marker::Sync for BarcodeScannerCapabilities {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct BarcodeScannerDataReceivedEventArgs(::windows::runtime::IInspectable);
impl BarcodeScannerDataReceivedEventArgs {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Report(&self) -> ::windows::runtime::Result<BarcodeScannerReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BarcodeScannerReport>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BarcodeScannerDataReceivedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.BarcodeScannerDataReceivedEventArgs;{4234a7e2-ed97-467d-ad2b-01e44313a929})");
}
unsafe impl ::windows::runtime::Interface for BarcodeScannerDataReceivedEventArgs {
    type Vtable = IBarcodeScannerDataReceivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1110747106, 60823, 18045, [173, 43, 1, 228, 67, 19, 169, 41]);
}
impl ::windows::runtime::RuntimeName for BarcodeScannerDataReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.BarcodeScannerDataReceivedEventArgs";
}
impl ::std::convert::From<BarcodeScannerDataReceivedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: BarcodeScannerDataReceivedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&BarcodeScannerDataReceivedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &BarcodeScannerDataReceivedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for BarcodeScannerDataReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &BarcodeScannerDataReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<BarcodeScannerDataReceivedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: BarcodeScannerDataReceivedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&BarcodeScannerDataReceivedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &BarcodeScannerDataReceivedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for BarcodeScannerDataReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a BarcodeScannerDataReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for BarcodeScannerDataReceivedEventArgs {}
unsafe impl ::std::marker::Sync for BarcodeScannerDataReceivedEventArgs {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct BarcodeScannerErrorOccurredEventArgs(::windows::runtime::IInspectable);
impl BarcodeScannerErrorOccurredEventArgs {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PartialInputData(&self) -> ::windows::runtime::Result<BarcodeScannerReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BarcodeScannerReport>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsRetriable(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ErrorData(&self) -> ::windows::runtime::Result<UnifiedPosErrorData> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UnifiedPosErrorData>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BarcodeScannerErrorOccurredEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.BarcodeScannerErrorOccurredEventArgs;{2cd2602f-cf3a-4002-a75a-c5ec468f0a20})");
}
unsafe impl ::windows::runtime::Interface for BarcodeScannerErrorOccurredEventArgs {
    type Vtable = IBarcodeScannerErrorOccurredEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(751984687, 53050, 16386, [167, 90, 197, 236, 70, 143, 10, 32]);
}
impl ::windows::runtime::RuntimeName for BarcodeScannerErrorOccurredEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.BarcodeScannerErrorOccurredEventArgs";
}
impl ::std::convert::From<BarcodeScannerErrorOccurredEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: BarcodeScannerErrorOccurredEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&BarcodeScannerErrorOccurredEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &BarcodeScannerErrorOccurredEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for BarcodeScannerErrorOccurredEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &BarcodeScannerErrorOccurredEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<BarcodeScannerErrorOccurredEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: BarcodeScannerErrorOccurredEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&BarcodeScannerErrorOccurredEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &BarcodeScannerErrorOccurredEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for BarcodeScannerErrorOccurredEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a BarcodeScannerErrorOccurredEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for BarcodeScannerErrorOccurredEventArgs {}
unsafe impl ::std::marker::Sync for BarcodeScannerErrorOccurredEventArgs {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct BarcodeScannerImagePreviewReceivedEventArgs(::windows::runtime::IInspectable);
impl BarcodeScannerImagePreviewReceivedEventArgs {
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_PointOfService`, `Storage_Streams`*"]
    pub fn Preview(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IRandomAccessStreamWithContentType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStreamWithContentType>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BarcodeScannerImagePreviewReceivedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.BarcodeScannerImagePreviewReceivedEventArgs;{f3b7de85-6e8b-434e-9f58-06ef26bc4baf})");
}
unsafe impl ::windows::runtime::Interface for BarcodeScannerImagePreviewReceivedEventArgs {
    type Vtable = IBarcodeScannerImagePreviewReceivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4088913541, 28299, 17230, [159, 88, 6, 239, 38, 188, 75, 175]);
}
impl ::windows::runtime::RuntimeName for BarcodeScannerImagePreviewReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.BarcodeScannerImagePreviewReceivedEventArgs";
}
impl ::std::convert::From<BarcodeScannerImagePreviewReceivedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: BarcodeScannerImagePreviewReceivedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&BarcodeScannerImagePreviewReceivedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &BarcodeScannerImagePreviewReceivedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for BarcodeScannerImagePreviewReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &BarcodeScannerImagePreviewReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<BarcodeScannerImagePreviewReceivedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: BarcodeScannerImagePreviewReceivedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&BarcodeScannerImagePreviewReceivedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &BarcodeScannerImagePreviewReceivedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for BarcodeScannerImagePreviewReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a BarcodeScannerImagePreviewReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for BarcodeScannerImagePreviewReceivedEventArgs {}
unsafe impl ::std::marker::Sync for BarcodeScannerImagePreviewReceivedEventArgs {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct BarcodeScannerReport(::windows::runtime::IInspectable);
impl BarcodeScannerReport {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ScanDataType(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_PointOfService`, `Storage_Streams`*"]
    pub fn ScanData(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_PointOfService`, `Storage_Streams`*"]
    pub fn ScanDataLabel(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_PointOfService`, `Storage_Streams`*"]
    pub fn CreateInstance<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>, Param2: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(scandatatype: u32, scandata: Param1, scandatalabel: Param2) -> ::windows::runtime::Result<BarcodeScannerReport> {
        Self::IBarcodeScannerReportFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), scandatatype, scandata.into_param().abi(), scandatalabel.into_param().abi(), &mut result__).from_abi::<BarcodeScannerReport>(result__)
        })
    }
    pub fn IBarcodeScannerReportFactory<R, F: FnOnce(&IBarcodeScannerReportFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BarcodeScannerReport, IBarcodeScannerReportFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BarcodeScannerReport {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.BarcodeScannerReport;{5ce4d8b0-a489-4b96-86c4-f0bf8a37753d})");
}
unsafe impl ::windows::runtime::Interface for BarcodeScannerReport {
    type Vtable = IBarcodeScannerReport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1558501552, 42121, 19350, [134, 196, 240, 191, 138, 55, 117, 61]);
}
impl ::windows::runtime::RuntimeName for BarcodeScannerReport {
    const NAME: &'static str = "Windows.Devices.PointOfService.BarcodeScannerReport";
}
impl ::std::convert::From<BarcodeScannerReport> for ::windows::runtime::IUnknown {
    fn from(value: BarcodeScannerReport) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&BarcodeScannerReport> for ::windows::runtime::IUnknown {
    fn from(value: &BarcodeScannerReport) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for BarcodeScannerReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &BarcodeScannerReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<BarcodeScannerReport> for ::windows::runtime::IInspectable {
    fn from(value: BarcodeScannerReport) -> Self {
        value.0
    }
}
impl ::std::convert::From<&BarcodeScannerReport> for ::windows::runtime::IInspectable {
    fn from(value: &BarcodeScannerReport) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for BarcodeScannerReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a BarcodeScannerReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for BarcodeScannerReport {}
unsafe impl ::std::marker::Sync for BarcodeScannerReport {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct BarcodeScannerStatus(pub i32);
impl BarcodeScannerStatus {
    pub const Online: BarcodeScannerStatus = BarcodeScannerStatus(0i32);
    pub const Off: BarcodeScannerStatus = BarcodeScannerStatus(1i32);
    pub const Offline: BarcodeScannerStatus = BarcodeScannerStatus(2i32);
    pub const OffOrOffline: BarcodeScannerStatus = BarcodeScannerStatus(3i32);
    pub const Extended: BarcodeScannerStatus = BarcodeScannerStatus(4i32);
}
impl ::std::convert::From<i32> for BarcodeScannerStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for BarcodeScannerStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for BarcodeScannerStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.BarcodeScannerStatus;i4)");
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct BarcodeScannerStatusUpdatedEventArgs(::windows::runtime::IInspectable);
impl BarcodeScannerStatusUpdatedEventArgs {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<BarcodeScannerStatus> {
        let this = self;
        unsafe {
            let mut result__: BarcodeScannerStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BarcodeScannerStatus>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ExtendedStatus(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BarcodeScannerStatusUpdatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.BarcodeScannerStatusUpdatedEventArgs;{355d8586-9c43-462b-a91a-816dc97f452c})");
}
unsafe impl ::windows::runtime::Interface for BarcodeScannerStatusUpdatedEventArgs {
    type Vtable = IBarcodeScannerStatusUpdatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(895321478, 40003, 17963, [169, 26, 129, 109, 201, 127, 69, 44]);
}
impl ::windows::runtime::RuntimeName for BarcodeScannerStatusUpdatedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.BarcodeScannerStatusUpdatedEventArgs";
}
impl ::std::convert::From<BarcodeScannerStatusUpdatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: BarcodeScannerStatusUpdatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&BarcodeScannerStatusUpdatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &BarcodeScannerStatusUpdatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for BarcodeScannerStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &BarcodeScannerStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<BarcodeScannerStatusUpdatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: BarcodeScannerStatusUpdatedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&BarcodeScannerStatusUpdatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &BarcodeScannerStatusUpdatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for BarcodeScannerStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a BarcodeScannerStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for BarcodeScannerStatusUpdatedEventArgs {}
unsafe impl ::std::marker::Sync for BarcodeScannerStatusUpdatedEventArgs {}
#[doc = "*Required features: `Devices_PointOfService`*"]
pub struct BarcodeSymbologies {}
impl BarcodeSymbologies {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Unknown() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Ean8() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Ean8Add2() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Ean8Add5() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Eanv() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn EanvAdd2() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn EanvAdd5() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Ean13() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Ean13Add2() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Ean13Add5() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Isbn() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsbnAdd5() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Ismn() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsmnAdd2() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsmnAdd5() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Issn() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IssnAdd2() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IssnAdd5() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Ean99() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Ean99Add2() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Ean99Add5() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Upca() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn UpcaAdd2() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn UpcaAdd5() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Upce() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn UpceAdd2() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn UpceAdd5() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).32)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn UpcCoupon() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn TfStd() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).34)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn TfDis() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn TfInt() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).36)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn TfInd() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn TfMat() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).38)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn TfIata() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).39)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Gs1DatabarType1() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).40)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Gs1DatabarType2() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).41)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Gs1DatabarType3() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).42)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Code39() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).43)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Code39Ex() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).44)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Trioptic39() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).45)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Code32() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).46)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Pzn() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).47)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Code93() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).48)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Code93Ex() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).49)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Code128() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).50)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Gs1128() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).51)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Gs1128Coupon() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).52)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn UccEan128() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).53)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Sisac() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).54)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Isbt() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).55)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Codabar() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).56)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Code11() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).57)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Msi() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).58)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Plessey() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).59)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Telepen() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).60)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Code16k() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).61)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CodablockA() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).62)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CodablockF() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).63)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Codablock128() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).64)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Code49() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).65)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Aztec() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).66)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DataCode() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).67)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DataMatrix() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).68)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn HanXin() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).69)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Maxicode() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).70)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn MicroPdf417() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).71)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn MicroQr() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).72)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Pdf417() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).73)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Qr() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).74)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn MsTag() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).75)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Ccab() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).76)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Ccc() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).77)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Tlc39() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).78)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn AusPost() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).79)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CanPost() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).80)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ChinaPost() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).81)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DutchKix() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).82)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn InfoMail() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).83)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ItalianPost25() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).84)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ItalianPost39() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).85)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn JapanPost() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).86)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn KoreanPost() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).87)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SwedenPost() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).88)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn UkPost() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).89)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn UsIntelligent() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).90)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn UsIntelligentPkg() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).91)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn UsPlanet() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).92)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn UsPostNet() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).93)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Us4StateFics() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).94)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn OcrA() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).95)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn OcrB() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).96)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Micr() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).97)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ExtendedBase() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).98)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn GetName(scandatatype: u32) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).99)(::std::mem::transmute_copy(this), scandatatype, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Gs1DWCode() -> ::windows::runtime::Result<u32> {
        Self::IBarcodeSymbologiesStatics2(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    pub fn IBarcodeSymbologiesStatics<R, F: FnOnce(&IBarcodeSymbologiesStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BarcodeSymbologies, IBarcodeSymbologiesStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IBarcodeSymbologiesStatics2<R, F: FnOnce(&IBarcodeSymbologiesStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<BarcodeSymbologies, IBarcodeSymbologiesStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for BarcodeSymbologies {
    const NAME: &'static str = "Windows.Devices.PointOfService.BarcodeSymbologies";
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct BarcodeSymbologyAttributes(::windows::runtime::IInspectable);
impl BarcodeSymbologyAttributes {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsCheckDigitValidationEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetIsCheckDigitValidationEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsCheckDigitValidationSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsCheckDigitTransmissionEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetIsCheckDigitTransmissionEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsCheckDigitTransmissionSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DecodeLength1(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetDecodeLength1(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DecodeLength2(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetDecodeLength2(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DecodeLengthKind(&self) -> ::windows::runtime::Result<BarcodeSymbologyDecodeLengthKind> {
        let this = self;
        unsafe {
            let mut result__: BarcodeSymbologyDecodeLengthKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<BarcodeSymbologyDecodeLengthKind>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetDecodeLengthKind(&self, value: BarcodeSymbologyDecodeLengthKind) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDecodeLengthSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for BarcodeSymbologyAttributes {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.BarcodeSymbologyAttributes;{66413a78-ab7a-4ada-8ece-936014b2ead7})");
}
unsafe impl ::windows::runtime::Interface for BarcodeSymbologyAttributes {
    type Vtable = IBarcodeSymbologyAttributes_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1715550840, 43898, 19162, [142, 206, 147, 96, 20, 178, 234, 215]);
}
impl ::windows::runtime::RuntimeName for BarcodeSymbologyAttributes {
    const NAME: &'static str = "Windows.Devices.PointOfService.BarcodeSymbologyAttributes";
}
impl ::std::convert::From<BarcodeSymbologyAttributes> for ::windows::runtime::IUnknown {
    fn from(value: BarcodeSymbologyAttributes) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&BarcodeSymbologyAttributes> for ::windows::runtime::IUnknown {
    fn from(value: &BarcodeSymbologyAttributes) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for BarcodeSymbologyAttributes {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &BarcodeSymbologyAttributes {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<BarcodeSymbologyAttributes> for ::windows::runtime::IInspectable {
    fn from(value: BarcodeSymbologyAttributes) -> Self {
        value.0
    }
}
impl ::std::convert::From<&BarcodeSymbologyAttributes> for ::windows::runtime::IInspectable {
    fn from(value: &BarcodeSymbologyAttributes) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for BarcodeSymbologyAttributes {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a BarcodeSymbologyAttributes {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for BarcodeSymbologyAttributes {}
unsafe impl ::std::marker::Sync for BarcodeSymbologyAttributes {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct BarcodeSymbologyDecodeLengthKind(pub i32);
impl BarcodeSymbologyDecodeLengthKind {
    pub const AnyLength: BarcodeSymbologyDecodeLengthKind = BarcodeSymbologyDecodeLengthKind(0i32);
    pub const Discrete: BarcodeSymbologyDecodeLengthKind = BarcodeSymbologyDecodeLengthKind(1i32);
    pub const Range: BarcodeSymbologyDecodeLengthKind = BarcodeSymbologyDecodeLengthKind(2i32);
}
impl ::std::convert::From<i32> for BarcodeSymbologyDecodeLengthKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for BarcodeSymbologyDecodeLengthKind {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for BarcodeSymbologyDecodeLengthKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.BarcodeSymbologyDecodeLengthKind;i4)");
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CashDrawer(::windows::runtime::IInspectable);
impl CashDrawer {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Capabilities(&self) -> ::windows::runtime::Result<CashDrawerCapabilities> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CashDrawerCapabilities>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<CashDrawerStatus> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CashDrawerStatus>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDrawerOpen(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DrawerEventSource(&self) -> ::windows::runtime::Result<CashDrawerEventSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CashDrawerEventSource>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn ClaimDrawerAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ClaimedCashDrawer>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ClaimedCashDrawer>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn CheckHealthAsync(&self, level: UnifiedPosHealthCheckLevel) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), level, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetStatisticsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(&self, statisticscategories: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), statisticscategories.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn StatusUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<CashDrawer, CashDrawerStatusUpdatedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveStatusUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn GetDefaultAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<CashDrawer>> {
        Self::ICashDrawerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<CashDrawer>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn FromIdAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(deviceid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<CashDrawer>> {
        Self::ICashDrawerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<CashDrawer>>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn GetDeviceSelector() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::ICashDrawerStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn GetDeviceSelectorWithConnectionTypes(connectiontypes: PosConnectionTypes) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::ICashDrawerStatics2(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), connectiontypes, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn ICashDrawerStatics<R, F: FnOnce(&ICashDrawerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<CashDrawer, ICashDrawerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ICashDrawerStatics2<R, F: FnOnce(&ICashDrawerStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<CashDrawer, ICashDrawerStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CashDrawer {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.CashDrawer;{9f88f5c8-de54-4aee-a890-920bcbfe30fc})");
}
unsafe impl ::windows::runtime::Interface for CashDrawer {
    type Vtable = ICashDrawer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2676553160, 56916, 19182, [168, 144, 146, 11, 203, 254, 48, 252]);
}
impl ::windows::runtime::RuntimeName for CashDrawer {
    const NAME: &'static str = "Windows.Devices.PointOfService.CashDrawer";
}
impl ::std::convert::From<CashDrawer> for ::windows::runtime::IUnknown {
    fn from(value: CashDrawer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CashDrawer> for ::windows::runtime::IUnknown {
    fn from(value: &CashDrawer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CashDrawer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &CashDrawer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<CashDrawer> for ::windows::runtime::IInspectable {
    fn from(value: CashDrawer) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CashDrawer> for ::windows::runtime::IInspectable {
    fn from(value: &CashDrawer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CashDrawer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CashDrawer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<CashDrawer> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: CashDrawer) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&CashDrawer> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &CashDrawer) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for CashDrawer {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &CashDrawer {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for CashDrawer {}
unsafe impl ::std::marker::Sync for CashDrawer {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CashDrawerCapabilities(::windows::runtime::IInspectable);
impl CashDrawerCapabilities {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PowerReportingType(&self) -> ::windows::runtime::Result<UnifiedPosPowerReportingType> {
        let this = self;
        unsafe {
            let mut result__: UnifiedPosPowerReportingType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UnifiedPosPowerReportingType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsStatisticsReportingSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsStatisticsUpdatingSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsStatusReportingSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsStatusMultiDrawerDetectSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDrawerOpenSensorAvailable(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CashDrawerCapabilities {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.CashDrawerCapabilities;{0bc6de0b-e8e7-4b1f-b1d1-3e501ad08247})");
}
unsafe impl ::windows::runtime::Interface for CashDrawerCapabilities {
    type Vtable = ICashDrawerCapabilities_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(197582347, 59623, 19231, [177, 209, 62, 80, 26, 208, 130, 71]);
}
impl ::windows::runtime::RuntimeName for CashDrawerCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.CashDrawerCapabilities";
}
impl ::std::convert::From<CashDrawerCapabilities> for ::windows::runtime::IUnknown {
    fn from(value: CashDrawerCapabilities) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CashDrawerCapabilities> for ::windows::runtime::IUnknown {
    fn from(value: &CashDrawerCapabilities) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CashDrawerCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &CashDrawerCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<CashDrawerCapabilities> for ::windows::runtime::IInspectable {
    fn from(value: CashDrawerCapabilities) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CashDrawerCapabilities> for ::windows::runtime::IInspectable {
    fn from(value: &CashDrawerCapabilities) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CashDrawerCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CashDrawerCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CashDrawerCapabilities {}
unsafe impl ::std::marker::Sync for CashDrawerCapabilities {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CashDrawerCloseAlarm(::windows::runtime::IInspectable);
impl CashDrawerCloseAlarm {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn SetAlarmTimeout<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn AlarmTimeout(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetBeepFrequency(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn BeepFrequency(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn SetBeepDuration<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn BeepDuration(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn SetBeepDelay<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn BeepDelay(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn AlarmTimeoutExpired<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<CashDrawerCloseAlarm, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveAlarmTimeoutExpired<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn StartAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CashDrawerCloseAlarm {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.CashDrawerCloseAlarm;{6bf88cc7-6f63-430e-ab3b-95d75ffbe87f})");
}
unsafe impl ::windows::runtime::Interface for CashDrawerCloseAlarm {
    type Vtable = ICashDrawerCloseAlarm_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1811451079, 28515, 17166, [171, 59, 149, 215, 95, 251, 232, 127]);
}
impl ::windows::runtime::RuntimeName for CashDrawerCloseAlarm {
    const NAME: &'static str = "Windows.Devices.PointOfService.CashDrawerCloseAlarm";
}
impl ::std::convert::From<CashDrawerCloseAlarm> for ::windows::runtime::IUnknown {
    fn from(value: CashDrawerCloseAlarm) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CashDrawerCloseAlarm> for ::windows::runtime::IUnknown {
    fn from(value: &CashDrawerCloseAlarm) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CashDrawerCloseAlarm {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &CashDrawerCloseAlarm {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<CashDrawerCloseAlarm> for ::windows::runtime::IInspectable {
    fn from(value: CashDrawerCloseAlarm) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CashDrawerCloseAlarm> for ::windows::runtime::IInspectable {
    fn from(value: &CashDrawerCloseAlarm) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CashDrawerCloseAlarm {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CashDrawerCloseAlarm {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CashDrawerCloseAlarm {}
unsafe impl ::std::marker::Sync for CashDrawerCloseAlarm {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CashDrawerClosedEventArgs(::windows::runtime::IInspectable);
impl CashDrawerClosedEventArgs {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CashDrawer(&self) -> ::windows::runtime::Result<CashDrawer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CashDrawer>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CashDrawerClosedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.CashDrawerClosedEventArgs;{69cb3bc1-147f-421c-9c23-090123bb786c})");
}
unsafe impl ::windows::runtime::Interface for CashDrawerClosedEventArgs {
    type Vtable = ICashDrawerEventSourceEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1774926785, 5247, 16924, [156, 35, 9, 1, 35, 187, 120, 108]);
}
impl ::windows::runtime::RuntimeName for CashDrawerClosedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.CashDrawerClosedEventArgs";
}
impl ::std::convert::From<CashDrawerClosedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: CashDrawerClosedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CashDrawerClosedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &CashDrawerClosedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CashDrawerClosedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &CashDrawerClosedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<CashDrawerClosedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: CashDrawerClosedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CashDrawerClosedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &CashDrawerClosedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CashDrawerClosedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CashDrawerClosedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<CashDrawerClosedEventArgs> for ICashDrawerEventSourceEventArgs {
    fn from(value: CashDrawerClosedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CashDrawerClosedEventArgs> for ICashDrawerEventSourceEventArgs {
    fn from(value: &CashDrawerClosedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ICashDrawerEventSourceEventArgs> for CashDrawerClosedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ICashDrawerEventSourceEventArgs> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ICashDrawerEventSourceEventArgs>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ICashDrawerEventSourceEventArgs> for &CashDrawerClosedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ICashDrawerEventSourceEventArgs> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ICashDrawerEventSourceEventArgs>::into(::std::clone::Clone::clone(self)))
    }
}
unsafe impl ::std::marker::Send for CashDrawerClosedEventArgs {}
unsafe impl ::std::marker::Sync for CashDrawerClosedEventArgs {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CashDrawerEventSource(::windows::runtime::IInspectable);
impl CashDrawerEventSource {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn DrawerClosed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<CashDrawerEventSource, CashDrawerClosedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveDrawerClosed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn DrawerOpened<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<CashDrawerEventSource, CashDrawerOpenedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveDrawerOpened<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CashDrawerEventSource {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.CashDrawerEventSource;{e006e46c-f2f9-442f-8dd6-06c10a4227ba})");
}
unsafe impl ::windows::runtime::Interface for CashDrawerEventSource {
    type Vtable = ICashDrawerEventSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3758548076, 62201, 17455, [141, 214, 6, 193, 10, 66, 39, 186]);
}
impl ::windows::runtime::RuntimeName for CashDrawerEventSource {
    const NAME: &'static str = "Windows.Devices.PointOfService.CashDrawerEventSource";
}
impl ::std::convert::From<CashDrawerEventSource> for ::windows::runtime::IUnknown {
    fn from(value: CashDrawerEventSource) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CashDrawerEventSource> for ::windows::runtime::IUnknown {
    fn from(value: &CashDrawerEventSource) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CashDrawerEventSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &CashDrawerEventSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<CashDrawerEventSource> for ::windows::runtime::IInspectable {
    fn from(value: CashDrawerEventSource) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CashDrawerEventSource> for ::windows::runtime::IInspectable {
    fn from(value: &CashDrawerEventSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CashDrawerEventSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CashDrawerEventSource {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CashDrawerEventSource {}
unsafe impl ::std::marker::Sync for CashDrawerEventSource {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CashDrawerOpenedEventArgs(::windows::runtime::IInspectable);
impl CashDrawerOpenedEventArgs {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CashDrawer(&self) -> ::windows::runtime::Result<CashDrawer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CashDrawer>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CashDrawerOpenedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.CashDrawerOpenedEventArgs;{69cb3bc1-147f-421c-9c23-090123bb786c})");
}
unsafe impl ::windows::runtime::Interface for CashDrawerOpenedEventArgs {
    type Vtable = ICashDrawerEventSourceEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1774926785, 5247, 16924, [156, 35, 9, 1, 35, 187, 120, 108]);
}
impl ::windows::runtime::RuntimeName for CashDrawerOpenedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.CashDrawerOpenedEventArgs";
}
impl ::std::convert::From<CashDrawerOpenedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: CashDrawerOpenedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CashDrawerOpenedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &CashDrawerOpenedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CashDrawerOpenedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &CashDrawerOpenedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<CashDrawerOpenedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: CashDrawerOpenedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CashDrawerOpenedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &CashDrawerOpenedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CashDrawerOpenedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CashDrawerOpenedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<CashDrawerOpenedEventArgs> for ICashDrawerEventSourceEventArgs {
    fn from(value: CashDrawerOpenedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CashDrawerOpenedEventArgs> for ICashDrawerEventSourceEventArgs {
    fn from(value: &CashDrawerOpenedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ICashDrawerEventSourceEventArgs> for CashDrawerOpenedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ICashDrawerEventSourceEventArgs> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ICashDrawerEventSourceEventArgs>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ICashDrawerEventSourceEventArgs> for &CashDrawerOpenedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ICashDrawerEventSourceEventArgs> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ICashDrawerEventSourceEventArgs>::into(::std::clone::Clone::clone(self)))
    }
}
unsafe impl ::std::marker::Send for CashDrawerOpenedEventArgs {}
unsafe impl ::std::marker::Sync for CashDrawerOpenedEventArgs {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CashDrawerStatus(::windows::runtime::IInspectable);
impl CashDrawerStatus {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn StatusKind(&self) -> ::windows::runtime::Result<CashDrawerStatusKind> {
        let this = self;
        unsafe {
            let mut result__: CashDrawerStatusKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CashDrawerStatusKind>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ExtendedStatus(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CashDrawerStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.CashDrawerStatus;{6bbd78bf-dca1-4e06-99eb-5af6a5aec108})");
}
unsafe impl ::windows::runtime::Interface for CashDrawerStatus {
    type Vtable = ICashDrawerStatus_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1807579327, 56481, 19974, [153, 235, 90, 246, 165, 174, 193, 8]);
}
impl ::windows::runtime::RuntimeName for CashDrawerStatus {
    const NAME: &'static str = "Windows.Devices.PointOfService.CashDrawerStatus";
}
impl ::std::convert::From<CashDrawerStatus> for ::windows::runtime::IUnknown {
    fn from(value: CashDrawerStatus) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CashDrawerStatus> for ::windows::runtime::IUnknown {
    fn from(value: &CashDrawerStatus) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CashDrawerStatus {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &CashDrawerStatus {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<CashDrawerStatus> for ::windows::runtime::IInspectable {
    fn from(value: CashDrawerStatus) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CashDrawerStatus> for ::windows::runtime::IInspectable {
    fn from(value: &CashDrawerStatus) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CashDrawerStatus {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CashDrawerStatus {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CashDrawerStatus {}
unsafe impl ::std::marker::Sync for CashDrawerStatus {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CashDrawerStatusKind(pub i32);
impl CashDrawerStatusKind {
    pub const Online: CashDrawerStatusKind = CashDrawerStatusKind(0i32);
    pub const Off: CashDrawerStatusKind = CashDrawerStatusKind(1i32);
    pub const Offline: CashDrawerStatusKind = CashDrawerStatusKind(2i32);
    pub const OffOrOffline: CashDrawerStatusKind = CashDrawerStatusKind(3i32);
    pub const Extended: CashDrawerStatusKind = CashDrawerStatusKind(4i32);
}
impl ::std::convert::From<i32> for CashDrawerStatusKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CashDrawerStatusKind {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CashDrawerStatusKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.CashDrawerStatusKind;i4)");
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct CashDrawerStatusUpdatedEventArgs(::windows::runtime::IInspectable);
impl CashDrawerStatusUpdatedEventArgs {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<CashDrawerStatus> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CashDrawerStatus>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for CashDrawerStatusUpdatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.CashDrawerStatusUpdatedEventArgs;{30aae98a-0d70-459c-9553-87e124c52488})");
}
unsafe impl ::windows::runtime::Interface for CashDrawerStatusUpdatedEventArgs {
    type Vtable = ICashDrawerStatusUpdatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(816507274, 3440, 17820, [149, 83, 135, 225, 36, 197, 36, 136]);
}
impl ::windows::runtime::RuntimeName for CashDrawerStatusUpdatedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.CashDrawerStatusUpdatedEventArgs";
}
impl ::std::convert::From<CashDrawerStatusUpdatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: CashDrawerStatusUpdatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&CashDrawerStatusUpdatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &CashDrawerStatusUpdatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for CashDrawerStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &CashDrawerStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<CashDrawerStatusUpdatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: CashDrawerStatusUpdatedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CashDrawerStatusUpdatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &CashDrawerStatusUpdatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for CashDrawerStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a CashDrawerStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for CashDrawerStatusUpdatedEventArgs {}
unsafe impl ::std::marker::Sync for CashDrawerStatusUpdatedEventArgs {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ClaimedBarcodeScanner(::windows::runtime::IInspectable);
impl ClaimedBarcodeScanner {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetIsDisabledOnDataReceived(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDisabledOnDataReceived(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetIsDecodeDataEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDecodeDataEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn EnableAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn DisableAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn RetainDevice(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`, `Foundation_Collections`*"]
    pub fn SetActiveSymbologiesAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<u32>>>(&self, symbologies: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), symbologies.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`, `Foundation_Collections`*"]
    pub fn ResetStatisticsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(&self, statisticscategories: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), statisticscategories.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`, `Foundation_Collections`*"]
    pub fn UpdateStatisticsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>>>(&self, statistics: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), statistics.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn SetActiveProfileAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, profile: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), profile.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn DataReceived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<ClaimedBarcodeScanner, BarcodeScannerDataReceivedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveDataReceived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn TriggerPressed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventHandler<ClaimedBarcodeScanner>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveTriggerPressed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn TriggerReleased<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventHandler<ClaimedBarcodeScanner>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveTriggerReleased<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn ReleaseDeviceRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventHandler<ClaimedBarcodeScanner>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveReleaseDeviceRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn ImagePreviewReceived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<ClaimedBarcodeScanner, BarcodeScannerImagePreviewReceivedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveImagePreviewReceived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn ErrorOccurred<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<ClaimedBarcodeScanner, BarcodeScannerErrorOccurredEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveErrorOccurred<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn StartSoftwareTriggerAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IClaimedBarcodeScanner1>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn StopSoftwareTriggerAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::runtime::Interface::cast::<IClaimedBarcodeScanner1>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn GetSymbologyAttributesAsync(&self, barcodesymbology: u32) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<BarcodeSymbologyAttributes>> {
        let this = &::windows::runtime::Interface::cast::<IClaimedBarcodeScanner2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), barcodesymbology, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BarcodeSymbologyAttributes>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn SetSymbologyAttributesAsync<'a, Param1: ::windows::runtime::IntoParam<'a, BarcodeSymbologyAttributes>>(&self, barcodesymbology: u32, attributes: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::runtime::Interface::cast::<IClaimedBarcodeScanner2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), barcodesymbology, attributes.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn ShowVideoPreviewAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::runtime::Interface::cast::<IClaimedBarcodeScanner3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn HideVideoPreview(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IClaimedBarcodeScanner3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetIsVideoPreviewShownOnEnable(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IClaimedBarcodeScanner3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsVideoPreviewShownOnEnable(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IClaimedBarcodeScanner3>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn Closed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<ClaimedBarcodeScanner, ClaimedBarcodeScannerClosedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IClaimedBarcodeScanner4>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveClosed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IClaimedBarcodeScanner4>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ClaimedBarcodeScanner {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedBarcodeScanner;{4a63b49c-8fa4-4332-bb26-945d11d81e0f})");
}
unsafe impl ::windows::runtime::Interface for ClaimedBarcodeScanner {
    type Vtable = IClaimedBarcodeScanner_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1248048284, 36772, 17202, [187, 38, 148, 93, 17, 216, 30, 15]);
}
impl ::windows::runtime::RuntimeName for ClaimedBarcodeScanner {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedBarcodeScanner";
}
impl ::std::convert::From<ClaimedBarcodeScanner> for ::windows::runtime::IUnknown {
    fn from(value: ClaimedBarcodeScanner) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ClaimedBarcodeScanner> for ::windows::runtime::IUnknown {
    fn from(value: &ClaimedBarcodeScanner) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ClaimedBarcodeScanner {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ClaimedBarcodeScanner {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ClaimedBarcodeScanner> for ::windows::runtime::IInspectable {
    fn from(value: ClaimedBarcodeScanner) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ClaimedBarcodeScanner> for ::windows::runtime::IInspectable {
    fn from(value: &ClaimedBarcodeScanner) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ClaimedBarcodeScanner {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ClaimedBarcodeScanner {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<ClaimedBarcodeScanner> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ClaimedBarcodeScanner) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&ClaimedBarcodeScanner> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ClaimedBarcodeScanner) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for ClaimedBarcodeScanner {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &ClaimedBarcodeScanner {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for ClaimedBarcodeScanner {}
unsafe impl ::std::marker::Sync for ClaimedBarcodeScanner {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ClaimedBarcodeScannerClosedEventArgs(::windows::runtime::IInspectable);
impl ClaimedBarcodeScannerClosedEventArgs {}
unsafe impl ::windows::runtime::RuntimeType for ClaimedBarcodeScannerClosedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedBarcodeScannerClosedEventArgs;{cf7d5489-a22c-4c65-a901-88d77d833954})");
}
unsafe impl ::windows::runtime::Interface for ClaimedBarcodeScannerClosedEventArgs {
    type Vtable = IClaimedBarcodeScannerClosedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3481097353, 41516, 19557, [169, 1, 136, 215, 125, 131, 57, 84]);
}
impl ::windows::runtime::RuntimeName for ClaimedBarcodeScannerClosedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedBarcodeScannerClosedEventArgs";
}
impl ::std::convert::From<ClaimedBarcodeScannerClosedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ClaimedBarcodeScannerClosedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ClaimedBarcodeScannerClosedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ClaimedBarcodeScannerClosedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ClaimedBarcodeScannerClosedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ClaimedBarcodeScannerClosedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ClaimedBarcodeScannerClosedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ClaimedBarcodeScannerClosedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ClaimedBarcodeScannerClosedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ClaimedBarcodeScannerClosedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ClaimedBarcodeScannerClosedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ClaimedBarcodeScannerClosedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ClaimedBarcodeScannerClosedEventArgs {}
unsafe impl ::std::marker::Sync for ClaimedBarcodeScannerClosedEventArgs {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ClaimedCashDrawer(::windows::runtime::IInspectable);
impl ClaimedCashDrawer {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDrawerOpen(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CloseAlarm(&self) -> ::windows::runtime::Result<CashDrawerCloseAlarm> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CashDrawerCloseAlarm>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn OpenDrawerAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn EnableAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn DisableAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RetainDeviceAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`, `Foundation_Collections`*"]
    pub fn ResetStatisticsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(&self, statisticscategories: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), statisticscategories.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`, `Foundation_Collections`*"]
    pub fn UpdateStatisticsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>>>(&self, statistics: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), statistics.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn ReleaseDeviceRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<ClaimedCashDrawer, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveReleaseDeviceRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn Closed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<ClaimedCashDrawer, ClaimedCashDrawerClosedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IClaimedCashDrawer2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveClosed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IClaimedCashDrawer2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ClaimedCashDrawer {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedCashDrawer;{ca3f99af-abb8-42c1-8a84-5c66512f5a75})");
}
unsafe impl ::windows::runtime::Interface for ClaimedCashDrawer {
    type Vtable = IClaimedCashDrawer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3393165743, 43960, 17089, [138, 132, 92, 102, 81, 47, 90, 117]);
}
impl ::windows::runtime::RuntimeName for ClaimedCashDrawer {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedCashDrawer";
}
impl ::std::convert::From<ClaimedCashDrawer> for ::windows::runtime::IUnknown {
    fn from(value: ClaimedCashDrawer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ClaimedCashDrawer> for ::windows::runtime::IUnknown {
    fn from(value: &ClaimedCashDrawer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ClaimedCashDrawer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ClaimedCashDrawer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ClaimedCashDrawer> for ::windows::runtime::IInspectable {
    fn from(value: ClaimedCashDrawer) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ClaimedCashDrawer> for ::windows::runtime::IInspectable {
    fn from(value: &ClaimedCashDrawer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ClaimedCashDrawer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ClaimedCashDrawer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<ClaimedCashDrawer> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ClaimedCashDrawer) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&ClaimedCashDrawer> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ClaimedCashDrawer) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for ClaimedCashDrawer {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &ClaimedCashDrawer {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for ClaimedCashDrawer {}
unsafe impl ::std::marker::Sync for ClaimedCashDrawer {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ClaimedCashDrawerClosedEventArgs(::windows::runtime::IInspectable);
impl ClaimedCashDrawerClosedEventArgs {}
unsafe impl ::windows::runtime::RuntimeType for ClaimedCashDrawerClosedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedCashDrawerClosedEventArgs;{cc573f33-3f34-4c5c-baae-deadf16cd7fa})");
}
unsafe impl ::windows::runtime::Interface for ClaimedCashDrawerClosedEventArgs {
    type Vtable = IClaimedCashDrawerClosedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3428269875, 16180, 19548, [186, 174, 222, 173, 241, 108, 215, 250]);
}
impl ::windows::runtime::RuntimeName for ClaimedCashDrawerClosedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedCashDrawerClosedEventArgs";
}
impl ::std::convert::From<ClaimedCashDrawerClosedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ClaimedCashDrawerClosedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ClaimedCashDrawerClosedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ClaimedCashDrawerClosedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ClaimedCashDrawerClosedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ClaimedCashDrawerClosedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ClaimedCashDrawerClosedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ClaimedCashDrawerClosedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ClaimedCashDrawerClosedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ClaimedCashDrawerClosedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ClaimedCashDrawerClosedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ClaimedCashDrawerClosedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ClaimedCashDrawerClosedEventArgs {}
unsafe impl ::std::marker::Sync for ClaimedCashDrawerClosedEventArgs {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ClaimedJournalPrinter(::windows::runtime::IInspectable);
impl ClaimedJournalPrinter {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CreateJob(&self) -> ::windows::runtime::Result<JournalPrintJob> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<JournalPrintJob>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetCharactersPerLine(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CharactersPerLine(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetLineHeight(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn LineHeight(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetLineSpacing(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn LineSpacing(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn LineWidth(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetIsLetterQuality(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsLetterQuality(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsPaperNearEnd(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetColorCartridge(&self, value: PosPrinterColorCartridge) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ColorCartridge(&self) -> ::windows::runtime::Result<PosPrinterColorCartridge> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: PosPrinterColorCartridge = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterColorCartridge>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsCoverOpen(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsCartridgeRemoved(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsCartridgeEmpty(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsHeadCleaning(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsPaperEmpty(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsReadyToPrint(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ValidateData<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, data: Param0) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), data.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ClaimedJournalPrinter {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedJournalPrinter;{67ea0630-517d-487f-9fdf-d2e0a0a264a5})");
}
unsafe impl ::windows::runtime::Interface for ClaimedJournalPrinter {
    type Vtable = IClaimedJournalPrinter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1743390256, 20861, 18559, [159, 223, 210, 224, 160, 162, 100, 165]);
}
impl ::windows::runtime::RuntimeName for ClaimedJournalPrinter {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedJournalPrinter";
}
impl ::std::convert::From<ClaimedJournalPrinter> for ::windows::runtime::IUnknown {
    fn from(value: ClaimedJournalPrinter) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ClaimedJournalPrinter> for ::windows::runtime::IUnknown {
    fn from(value: &ClaimedJournalPrinter) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ClaimedJournalPrinter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ClaimedJournalPrinter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ClaimedJournalPrinter> for ::windows::runtime::IInspectable {
    fn from(value: ClaimedJournalPrinter) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ClaimedJournalPrinter> for ::windows::runtime::IInspectable {
    fn from(value: &ClaimedJournalPrinter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ClaimedJournalPrinter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ClaimedJournalPrinter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<ClaimedJournalPrinter> for ICommonClaimedPosPrinterStation {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ClaimedJournalPrinter) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&ClaimedJournalPrinter> for ICommonClaimedPosPrinterStation {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ClaimedJournalPrinter) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ICommonClaimedPosPrinterStation> for ClaimedJournalPrinter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ICommonClaimedPosPrinterStation> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ICommonClaimedPosPrinterStation> for &ClaimedJournalPrinter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ICommonClaimedPosPrinterStation> {
        ::std::convert::TryInto::<ICommonClaimedPosPrinterStation>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for ClaimedJournalPrinter {}
unsafe impl ::std::marker::Sync for ClaimedJournalPrinter {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ClaimedLineDisplay(::windows::runtime::IInspectable);
impl ClaimedLineDisplay {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Capabilities(&self) -> ::windows::runtime::Result<LineDisplayCapabilities> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LineDisplayCapabilities>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PhysicalDeviceName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PhysicalDeviceDescription(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DeviceControlDescription(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DeviceControlVersion(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DeviceServiceVersion(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DefaultWindow(&self) -> ::windows::runtime::Result<LineDisplayWindow> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LineDisplayWindow>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn RetainDevice(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn ReleaseDeviceRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<ClaimedLineDisplay, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveReleaseDeviceRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn FromIdAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(deviceid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ClaimedLineDisplay>> {
        Self::IClaimedLineDisplayStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ClaimedLineDisplay>>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn GetDeviceSelector() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IClaimedLineDisplayStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn GetDeviceSelectorWithConnectionTypes(connectiontypes: PosConnectionTypes) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IClaimedLineDisplayStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), connectiontypes, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetStatisticsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(&self, statisticscategories: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>> {
        let this = &::windows::runtime::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), statisticscategories.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn CheckHealthAsync(&self, level: UnifiedPosHealthCheckLevel) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>> {
        let this = &::windows::runtime::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), level, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn CheckPowerStatusAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<LineDisplayPowerStatus>> {
        let this = &::windows::runtime::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LineDisplayPowerStatus>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn StatusUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<ClaimedLineDisplay, LineDisplayStatusUpdatedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveStatusUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`, `Foundation_Collections`*"]
    pub fn SupportedScreenSizesInCharacters(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<super::super::Foundation::Size>> {
        let this = &::windows::runtime::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Foundation::Size>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn MaxBitmapSizeInPixels(&self) -> ::windows::runtime::Result<super::super::Foundation::Size> {
        let this = &::windows::runtime::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::Size = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation_Collections`*"]
    pub fn SupportedCharacterSets(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<i32>> {
        let this = &::windows::runtime::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<i32>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CustomGlyphs(&self) -> ::windows::runtime::Result<LineDisplayCustomGlyphs> {
        let this = &::windows::runtime::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LineDisplayCustomGlyphs>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn GetAttributes(&self) -> ::windows::runtime::Result<LineDisplayAttributes> {
        let this = &::windows::runtime::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LineDisplayAttributes>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn TryUpdateAttributesAsync<'a, Param0: ::windows::runtime::IntoParam<'a, LineDisplayAttributes>>(&self, attributes: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::runtime::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), attributes.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn TrySetDescriptorAsync(&self, descriptor: u32, descriptorstate: LineDisplayDescriptorState) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::runtime::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), descriptor, descriptorstate, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn TryClearDescriptorsAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::runtime::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn TryCreateWindowAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Rect>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Size>>(&self, viewport: Param0, windowsize: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<LineDisplayWindow>> {
        let this = &::windows::runtime::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), viewport.into_param().abi(), windowsize.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LineDisplayWindow>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`, `Storage`*"]
    pub fn TryStoreStorageFileBitmapAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::StorageFile>>(&self, bitmap: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<LineDisplayStoredBitmap>> {
        let this = &::windows::runtime::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), bitmap.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LineDisplayStoredBitmap>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`, `Storage`*"]
    pub fn TryStoreStorageFileBitmapWithAlignmentAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::StorageFile>>(&self, bitmap: Param0, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<LineDisplayStoredBitmap>> {
        let this = &::windows::runtime::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), bitmap.into_param().abi(), horizontalalignment, verticalalignment, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LineDisplayStoredBitmap>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`, `Storage`*"]
    pub fn TryStoreStorageFileBitmapWithAlignmentAndWidthAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::StorageFile>>(&self, bitmap: Param0, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment, widthinpixels: i32) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<LineDisplayStoredBitmap>> {
        let this = &::windows::runtime::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), bitmap.into_param().abi(), horizontalalignment, verticalalignment, widthinpixels, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LineDisplayStoredBitmap>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn Closed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<ClaimedLineDisplay, ClaimedLineDisplayClosedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IClaimedLineDisplay3>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveClosed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IClaimedLineDisplay3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    pub fn IClaimedLineDisplayStatics<R, F: FnOnce(&IClaimedLineDisplayStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ClaimedLineDisplay, IClaimedLineDisplayStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ClaimedLineDisplay {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedLineDisplay;{120ac970-9a75-4acf-aae7-09972bcf8794})");
}
unsafe impl ::windows::runtime::Interface for ClaimedLineDisplay {
    type Vtable = IClaimedLineDisplay_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(302696816, 39541, 19151, [170, 231, 9, 151, 43, 207, 135, 148]);
}
impl ::windows::runtime::RuntimeName for ClaimedLineDisplay {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedLineDisplay";
}
impl ::std::convert::From<ClaimedLineDisplay> for ::windows::runtime::IUnknown {
    fn from(value: ClaimedLineDisplay) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ClaimedLineDisplay> for ::windows::runtime::IUnknown {
    fn from(value: &ClaimedLineDisplay) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ClaimedLineDisplay {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ClaimedLineDisplay {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ClaimedLineDisplay> for ::windows::runtime::IInspectable {
    fn from(value: ClaimedLineDisplay) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ClaimedLineDisplay> for ::windows::runtime::IInspectable {
    fn from(value: &ClaimedLineDisplay) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ClaimedLineDisplay {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ClaimedLineDisplay {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<ClaimedLineDisplay> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ClaimedLineDisplay) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&ClaimedLineDisplay> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ClaimedLineDisplay) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for ClaimedLineDisplay {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &ClaimedLineDisplay {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for ClaimedLineDisplay {}
unsafe impl ::std::marker::Sync for ClaimedLineDisplay {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ClaimedLineDisplayClosedEventArgs(::windows::runtime::IInspectable);
impl ClaimedLineDisplayClosedEventArgs {}
unsafe impl ::windows::runtime::RuntimeType for ClaimedLineDisplayClosedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedLineDisplayClosedEventArgs;{f915f364-d3d5-4f10-b511-90939edfacd8})");
}
unsafe impl ::windows::runtime::Interface for ClaimedLineDisplayClosedEventArgs {
    type Vtable = IClaimedLineDisplayClosedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4178965348, 54229, 20240, [181, 17, 144, 147, 158, 223, 172, 216]);
}
impl ::windows::runtime::RuntimeName for ClaimedLineDisplayClosedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedLineDisplayClosedEventArgs";
}
impl ::std::convert::From<ClaimedLineDisplayClosedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ClaimedLineDisplayClosedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ClaimedLineDisplayClosedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ClaimedLineDisplayClosedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ClaimedLineDisplayClosedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ClaimedLineDisplayClosedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ClaimedLineDisplayClosedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ClaimedLineDisplayClosedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ClaimedLineDisplayClosedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ClaimedLineDisplayClosedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ClaimedLineDisplayClosedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ClaimedLineDisplayClosedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ClaimedLineDisplayClosedEventArgs {}
unsafe impl ::std::marker::Sync for ClaimedLineDisplayClosedEventArgs {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ClaimedMagneticStripeReader(::windows::runtime::IInspectable);
impl ClaimedMagneticStripeReader {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetIsDisabledOnDataReceived(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDisabledOnDataReceived(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetIsDecodeDataEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDecodeDataEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDeviceAuthenticated(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetDataEncryptionAlgorithm(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DataEncryptionAlgorithm(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetTracksToRead(&self, value: MagneticStripeReaderTrackIds) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn TracksToRead(&self) -> ::windows::runtime::Result<MagneticStripeReaderTrackIds> {
        let this = self;
        unsafe {
            let mut result__: MagneticStripeReaderTrackIds = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MagneticStripeReaderTrackIds>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetIsTransmitSentinelsEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsTransmitSentinelsEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn EnableAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn DisableAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn RetainDevice(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetErrorReportingType(&self, value: MagneticStripeReaderErrorReportingType) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`, `Storage_Streams`*"]
    pub fn RetrieveDeviceAuthenticationDataAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn AuthenticateDeviceAsync(&self, responsetoken: &[<u8 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), responsetoken.len() as u32, ::std::mem::transmute(responsetoken.as_ptr()), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn DeAuthenticateDeviceAsync(&self, responsetoken: &[<u8 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), responsetoken.len() as u32, ::std::mem::transmute(responsetoken.as_ptr()), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn UpdateKeyAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, key: Param0, keyname: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), key.into_param().abi(), keyname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`, `Foundation_Collections`*"]
    pub fn ResetStatisticsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(&self, statisticscategories: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), statisticscategories.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`, `Foundation_Collections`*"]
    pub fn UpdateStatisticsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>>>(&self, statistics: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), statistics.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn BankCardDataReceived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, MagneticStripeReaderBankCardDataReceivedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveBankCardDataReceived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn AamvaCardDataReceived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, MagneticStripeReaderAamvaCardDataReceivedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveAamvaCardDataReceived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).32)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn VendorSpecificDataReceived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveVendorSpecificDataReceived<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).34)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn ReleaseDeviceRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventHandler<ClaimedMagneticStripeReader>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveReleaseDeviceRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).36)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn ErrorOccurred<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, MagneticStripeReaderErrorOccurredEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveErrorOccurred<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).38)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn Closed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, ClaimedMagneticStripeReaderClosedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IClaimedMagneticStripeReader2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveClosed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IClaimedMagneticStripeReader2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ClaimedMagneticStripeReader {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedMagneticStripeReader;{475ca8f3-9417-48bc-b9d7-4163a7844c02})");
}
unsafe impl ::windows::runtime::Interface for ClaimedMagneticStripeReader {
    type Vtable = IClaimedMagneticStripeReader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1197254899, 37911, 18620, [185, 215, 65, 99, 167, 132, 76, 2]);
}
impl ::windows::runtime::RuntimeName for ClaimedMagneticStripeReader {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedMagneticStripeReader";
}
impl ::std::convert::From<ClaimedMagneticStripeReader> for ::windows::runtime::IUnknown {
    fn from(value: ClaimedMagneticStripeReader) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ClaimedMagneticStripeReader> for ::windows::runtime::IUnknown {
    fn from(value: &ClaimedMagneticStripeReader) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ClaimedMagneticStripeReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ClaimedMagneticStripeReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ClaimedMagneticStripeReader> for ::windows::runtime::IInspectable {
    fn from(value: ClaimedMagneticStripeReader) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ClaimedMagneticStripeReader> for ::windows::runtime::IInspectable {
    fn from(value: &ClaimedMagneticStripeReader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ClaimedMagneticStripeReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ClaimedMagneticStripeReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<ClaimedMagneticStripeReader> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ClaimedMagneticStripeReader) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&ClaimedMagneticStripeReader> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ClaimedMagneticStripeReader) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for ClaimedMagneticStripeReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &ClaimedMagneticStripeReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for ClaimedMagneticStripeReader {}
unsafe impl ::std::marker::Sync for ClaimedMagneticStripeReader {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ClaimedMagneticStripeReaderClosedEventArgs(::windows::runtime::IInspectable);
impl ClaimedMagneticStripeReaderClosedEventArgs {}
unsafe impl ::windows::runtime::RuntimeType for ClaimedMagneticStripeReaderClosedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedMagneticStripeReaderClosedEventArgs;{14ada93a-adcd-4c80-acda-c3eaed2647e1})");
}
unsafe impl ::windows::runtime::Interface for ClaimedMagneticStripeReaderClosedEventArgs {
    type Vtable = IClaimedMagneticStripeReaderClosedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(346925370, 44493, 19584, [172, 218, 195, 234, 237, 38, 71, 225]);
}
impl ::windows::runtime::RuntimeName for ClaimedMagneticStripeReaderClosedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedMagneticStripeReaderClosedEventArgs";
}
impl ::std::convert::From<ClaimedMagneticStripeReaderClosedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ClaimedMagneticStripeReaderClosedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ClaimedMagneticStripeReaderClosedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ClaimedMagneticStripeReaderClosedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ClaimedMagneticStripeReaderClosedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ClaimedMagneticStripeReaderClosedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ClaimedMagneticStripeReaderClosedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ClaimedMagneticStripeReaderClosedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ClaimedMagneticStripeReaderClosedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ClaimedMagneticStripeReaderClosedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ClaimedMagneticStripeReaderClosedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ClaimedMagneticStripeReaderClosedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ClaimedMagneticStripeReaderClosedEventArgs {}
unsafe impl ::std::marker::Sync for ClaimedMagneticStripeReaderClosedEventArgs {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ClaimedPosPrinter(::windows::runtime::IInspectable);
impl ClaimedPosPrinter {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetCharacterSet(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CharacterSet(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsCoverOpen(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetIsCharacterSetMappingEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsCharacterSetMappingEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetMapMode(&self, value: PosPrinterMapMode) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn MapMode(&self) -> ::windows::runtime::Result<PosPrinterMapMode> {
        let this = self;
        unsafe {
            let mut result__: PosPrinterMapMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterMapMode>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Receipt(&self) -> ::windows::runtime::Result<ClaimedReceiptPrinter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ClaimedReceiptPrinter>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Slip(&self) -> ::windows::runtime::Result<ClaimedSlipPrinter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ClaimedSlipPrinter>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Journal(&self) -> ::windows::runtime::Result<ClaimedJournalPrinter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ClaimedJournalPrinter>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn EnableAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn DisableAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RetainDeviceAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`, `Foundation_Collections`*"]
    pub fn ResetStatisticsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(&self, statisticscategories: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), statisticscategories.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`, `Foundation_Collections`*"]
    pub fn UpdateStatisticsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>>>(&self, statistics: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), statistics.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn ReleaseDeviceRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<ClaimedPosPrinter, PosPrinterReleaseDeviceRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveReleaseDeviceRequested<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn Closed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<ClaimedPosPrinter, ClaimedPosPrinterClosedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IClaimedPosPrinter2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveClosed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IClaimedPosPrinter2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ClaimedPosPrinter {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedPosPrinter;{6d64ce0c-e03e-4b14-a38e-c28c34b86353})");
}
unsafe impl ::windows::runtime::Interface for ClaimedPosPrinter {
    type Vtable = IClaimedPosPrinter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1835322892, 57406, 19220, [163, 142, 194, 140, 52, 184, 99, 83]);
}
impl ::windows::runtime::RuntimeName for ClaimedPosPrinter {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedPosPrinter";
}
impl ::std::convert::From<ClaimedPosPrinter> for ::windows::runtime::IUnknown {
    fn from(value: ClaimedPosPrinter) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ClaimedPosPrinter> for ::windows::runtime::IUnknown {
    fn from(value: &ClaimedPosPrinter) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ClaimedPosPrinter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ClaimedPosPrinter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ClaimedPosPrinter> for ::windows::runtime::IInspectable {
    fn from(value: ClaimedPosPrinter) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ClaimedPosPrinter> for ::windows::runtime::IInspectable {
    fn from(value: &ClaimedPosPrinter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ClaimedPosPrinter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ClaimedPosPrinter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<ClaimedPosPrinter> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ClaimedPosPrinter) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&ClaimedPosPrinter> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ClaimedPosPrinter) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for ClaimedPosPrinter {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &ClaimedPosPrinter {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for ClaimedPosPrinter {}
unsafe impl ::std::marker::Sync for ClaimedPosPrinter {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ClaimedPosPrinterClosedEventArgs(::windows::runtime::IInspectable);
impl ClaimedPosPrinterClosedEventArgs {}
unsafe impl ::windows::runtime::RuntimeType for ClaimedPosPrinterClosedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedPosPrinterClosedEventArgs;{e2b7a27b-4d40-471d-92ed-63375b18c788})");
}
unsafe impl ::windows::runtime::Interface for ClaimedPosPrinterClosedEventArgs {
    type Vtable = IClaimedPosPrinterClosedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3803685499, 19776, 18205, [146, 237, 99, 55, 91, 24, 199, 136]);
}
impl ::windows::runtime::RuntimeName for ClaimedPosPrinterClosedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedPosPrinterClosedEventArgs";
}
impl ::std::convert::From<ClaimedPosPrinterClosedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: ClaimedPosPrinterClosedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ClaimedPosPrinterClosedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &ClaimedPosPrinterClosedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ClaimedPosPrinterClosedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ClaimedPosPrinterClosedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ClaimedPosPrinterClosedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: ClaimedPosPrinterClosedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ClaimedPosPrinterClosedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &ClaimedPosPrinterClosedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ClaimedPosPrinterClosedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ClaimedPosPrinterClosedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ClaimedPosPrinterClosedEventArgs {}
unsafe impl ::std::marker::Sync for ClaimedPosPrinterClosedEventArgs {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ClaimedReceiptPrinter(::windows::runtime::IInspectable);
impl ClaimedReceiptPrinter {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SidewaysMaxLines(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SidewaysMaxChars(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn LinesToPaperCut(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn PageSize(&self) -> ::windows::runtime::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Size = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn PrintArea(&self) -> ::windows::runtime::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Rect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CreateJob(&self) -> ::windows::runtime::Result<ReceiptPrintJob> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ReceiptPrintJob>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetCharactersPerLine(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CharactersPerLine(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetLineHeight(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn LineHeight(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetLineSpacing(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn LineSpacing(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn LineWidth(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetIsLetterQuality(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsLetterQuality(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsPaperNearEnd(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetColorCartridge(&self, value: PosPrinterColorCartridge) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ColorCartridge(&self) -> ::windows::runtime::Result<PosPrinterColorCartridge> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: PosPrinterColorCartridge = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterColorCartridge>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsCoverOpen(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsCartridgeRemoved(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsCartridgeEmpty(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsHeadCleaning(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsPaperEmpty(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsReadyToPrint(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ValidateData<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, data: Param0) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), data.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ClaimedReceiptPrinter {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedReceiptPrinter;{9ad27a74-dd61-4ee2-9837-5b5d72d538b9})");
}
unsafe impl ::windows::runtime::Interface for ClaimedReceiptPrinter {
    type Vtable = IClaimedReceiptPrinter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2597485172, 56673, 20194, [152, 55, 91, 93, 114, 213, 56, 185]);
}
impl ::windows::runtime::RuntimeName for ClaimedReceiptPrinter {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedReceiptPrinter";
}
impl ::std::convert::From<ClaimedReceiptPrinter> for ::windows::runtime::IUnknown {
    fn from(value: ClaimedReceiptPrinter) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ClaimedReceiptPrinter> for ::windows::runtime::IUnknown {
    fn from(value: &ClaimedReceiptPrinter) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ClaimedReceiptPrinter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ClaimedReceiptPrinter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ClaimedReceiptPrinter> for ::windows::runtime::IInspectable {
    fn from(value: ClaimedReceiptPrinter) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ClaimedReceiptPrinter> for ::windows::runtime::IInspectable {
    fn from(value: &ClaimedReceiptPrinter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ClaimedReceiptPrinter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ClaimedReceiptPrinter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<ClaimedReceiptPrinter> for ICommonClaimedPosPrinterStation {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ClaimedReceiptPrinter) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&ClaimedReceiptPrinter> for ICommonClaimedPosPrinterStation {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ClaimedReceiptPrinter) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ICommonClaimedPosPrinterStation> for ClaimedReceiptPrinter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ICommonClaimedPosPrinterStation> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ICommonClaimedPosPrinterStation> for &ClaimedReceiptPrinter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ICommonClaimedPosPrinterStation> {
        ::std::convert::TryInto::<ICommonClaimedPosPrinterStation>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for ClaimedReceiptPrinter {}
unsafe impl ::std::marker::Sync for ClaimedReceiptPrinter {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ClaimedSlipPrinter(::windows::runtime::IInspectable);
impl ClaimedSlipPrinter {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SidewaysMaxLines(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SidewaysMaxChars(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn MaxLines(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn LinesNearEndToEnd(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PrintSide(&self) -> ::windows::runtime::Result<PosPrinterPrintSide> {
        let this = self;
        unsafe {
            let mut result__: PosPrinterPrintSide = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterPrintSide>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn PageSize(&self) -> ::windows::runtime::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Size = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn PrintArea(&self) -> ::windows::runtime::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Rect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn OpenJaws(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CloseJaws(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn InsertSlipAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, timeout: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), timeout.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveSlipAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, timeout: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), timeout.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ChangePrintSide(&self, printside: PosPrinterPrintSide) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), printside).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CreateJob(&self) -> ::windows::runtime::Result<SlipPrintJob> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SlipPrintJob>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetCharactersPerLine(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CharactersPerLine(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetLineHeight(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn LineHeight(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetLineSpacing(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn LineSpacing(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn LineWidth(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetIsLetterQuality(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsLetterQuality(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsPaperNearEnd(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetColorCartridge(&self, value: PosPrinterColorCartridge) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ColorCartridge(&self) -> ::windows::runtime::Result<PosPrinterColorCartridge> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: PosPrinterColorCartridge = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterColorCartridge>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsCoverOpen(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsCartridgeRemoved(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsCartridgeEmpty(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsHeadCleaning(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsPaperEmpty(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsReadyToPrint(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ValidateData<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, data: Param0) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), data.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ClaimedSlipPrinter {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedSlipPrinter;{bd5deff2-af90-4e8a-b77b-e3ae9ca63a7f})");
}
unsafe impl ::windows::runtime::Interface for ClaimedSlipPrinter {
    type Vtable = IClaimedSlipPrinter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3177050098, 44944, 20106, [183, 123, 227, 174, 156, 166, 58, 127]);
}
impl ::windows::runtime::RuntimeName for ClaimedSlipPrinter {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedSlipPrinter";
}
impl ::std::convert::From<ClaimedSlipPrinter> for ::windows::runtime::IUnknown {
    fn from(value: ClaimedSlipPrinter) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ClaimedSlipPrinter> for ::windows::runtime::IUnknown {
    fn from(value: &ClaimedSlipPrinter) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ClaimedSlipPrinter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ClaimedSlipPrinter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ClaimedSlipPrinter> for ::windows::runtime::IInspectable {
    fn from(value: ClaimedSlipPrinter) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ClaimedSlipPrinter> for ::windows::runtime::IInspectable {
    fn from(value: &ClaimedSlipPrinter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ClaimedSlipPrinter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ClaimedSlipPrinter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<ClaimedSlipPrinter> for ICommonClaimedPosPrinterStation {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ClaimedSlipPrinter) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&ClaimedSlipPrinter> for ICommonClaimedPosPrinterStation {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ClaimedSlipPrinter) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ICommonClaimedPosPrinterStation> for ClaimedSlipPrinter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ICommonClaimedPosPrinterStation> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ICommonClaimedPosPrinterStation> for &ClaimedSlipPrinter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ICommonClaimedPosPrinterStation> {
        ::std::convert::TryInto::<ICommonClaimedPosPrinterStation>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for ClaimedSlipPrinter {}
unsafe impl ::std::marker::Sync for ClaimedSlipPrinter {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IBarcodeScanner(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBarcodeScanner {
    type Vtable = IBarcodeScanner_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3198369286, 45668, 20227, [169, 193, 69, 178, 15, 1, 19, 79]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScanner_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, level: UnifiedPosHealthCheckLevel, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, barcodesymbology: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, statisticscategories: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, profile: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IBarcodeScanner2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBarcodeScanner2 {
    type Vtable = IBarcodeScanner2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2300662119, 36078, 17261, [137, 171, 141, 251, 67, 187, 66, 134]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScanner2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IBarcodeScannerCapabilities(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBarcodeScannerCapabilities {
    type Vtable = IBarcodeScannerCapabilities_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3322319332, 62152, 17440, [163, 7, 177, 46, 246, 98, 40, 87]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerCapabilities_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut UnifiedPosPowerReportingType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IBarcodeScannerCapabilities1(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBarcodeScannerCapabilities1 {
    type Vtable = IBarcodeScannerCapabilities1_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2388308969, 3628, 18223, [161, 204, 238, 128, 84, 182, 166, 132]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerCapabilities1_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IBarcodeScannerCapabilities2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBarcodeScannerCapabilities2 {
    type Vtable = IBarcodeScannerCapabilities2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4061253612, 57761, 20136, [154, 188, 146, 177, 89, 98, 112, 171]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerCapabilities2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IBarcodeScannerDataReceivedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBarcodeScannerDataReceivedEventArgs {
    type Vtable = IBarcodeScannerDataReceivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1110747106, 60823, 18045, [173, 43, 1, 228, 67, 19, 169, 41]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerDataReceivedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IBarcodeScannerErrorOccurredEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBarcodeScannerErrorOccurredEventArgs {
    type Vtable = IBarcodeScannerErrorOccurredEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(751984687, 53050, 16386, [167, 90, 197, 236, 70, 143, 10, 32]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerErrorOccurredEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IBarcodeScannerImagePreviewReceivedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBarcodeScannerImagePreviewReceivedEventArgs {
    type Vtable = IBarcodeScannerImagePreviewReceivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4088913541, 28299, 17230, [159, 88, 6, 239, 38, 188, 75, 175]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerImagePreviewReceivedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IBarcodeScannerReport(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBarcodeScannerReport {
    type Vtable = IBarcodeScannerReport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1558501552, 42121, 19350, [134, 196, 240, 191, 138, 55, 117, 61]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerReport_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IBarcodeScannerReportFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBarcodeScannerReportFactory {
    type Vtable = IBarcodeScannerReportFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2723443494, 8211, 17788, [137, 99, 73, 193, 93, 202, 120, 206]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerReportFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, scandatatype: u32, scandata: ::windows::runtime::RawPtr, scandatalabel: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IBarcodeScannerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBarcodeScannerStatics {
    type Vtable = IBarcodeScannerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1561419631, 55881, 16872, [140, 140, 240, 203, 98, 169, 196, 252]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IBarcodeScannerStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBarcodeScannerStatics2 {
    type Vtable = IBarcodeScannerStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3093636211, 41839, 16391, [177, 208, 39, 158, 190, 146, 166, 86]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, connectiontypes: PosConnectionTypes, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IBarcodeScannerStatusUpdatedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBarcodeScannerStatusUpdatedEventArgs {
    type Vtable = IBarcodeScannerStatusUpdatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(895321478, 40003, 17963, [169, 26, 129, 109, 201, 127, 69, 44]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerStatusUpdatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut BarcodeScannerStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IBarcodeSymbologiesStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBarcodeSymbologiesStatics {
    type Vtable = IBarcodeSymbologiesStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3397732795, 1746, 17396, [164, 75, 198, 32, 103, 159, 216, 208]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeSymbologiesStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, scandatatype: u32, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IBarcodeSymbologiesStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBarcodeSymbologiesStatics2 {
    type Vtable = IBarcodeSymbologiesStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2339707124, 39376, 16575, [148, 36, 185, 29, 109, 212, 198, 224]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeSymbologiesStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IBarcodeSymbologyAttributes(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IBarcodeSymbologyAttributes {
    type Vtable = IBarcodeSymbologyAttributes_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1715550840, 43898, 19162, [142, 206, 147, 96, 20, 178, 234, 215]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeSymbologyAttributes_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut BarcodeSymbologyDecodeLengthKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: BarcodeSymbologyDecodeLengthKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ICashDrawer(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICashDrawer {
    type Vtable = ICashDrawer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2676553160, 56916, 19182, [168, 144, 146, 11, 203, 254, 48, 252]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICashDrawer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, level: UnifiedPosHealthCheckLevel, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, statisticscategories: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ICashDrawerCapabilities(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICashDrawerCapabilities {
    type Vtable = ICashDrawerCapabilities_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(197582347, 59623, 19231, [177, 209, 62, 80, 26, 208, 130, 71]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICashDrawerCapabilities_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut UnifiedPosPowerReportingType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ICashDrawerCloseAlarm(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICashDrawerCloseAlarm {
    type Vtable = ICashDrawerCloseAlarm_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1811451079, 28515, 17166, [171, 59, 149, 215, 95, 251, 232, 127]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICashDrawerCloseAlarm_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ICashDrawerEventSource(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICashDrawerEventSource {
    type Vtable = ICashDrawerEventSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3758548076, 62201, 17455, [141, 214, 6, 193, 10, 66, 39, 186]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICashDrawerEventSource_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Devices_PointOfService`*"]
pub struct ICashDrawerEventSourceEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICashDrawerEventSourceEventArgs {
    type Vtable = ICashDrawerEventSourceEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1774926785, 5247, 16924, [156, 35, 9, 1, 35, 187, 120, 108]);
}
impl ICashDrawerEventSourceEventArgs {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CashDrawer(&self) -> ::windows::runtime::Result<CashDrawer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CashDrawer>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ICashDrawerEventSourceEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{69cb3bc1-147f-421c-9c23-090123bb786c}");
}
#[repr(C)]
#[doc(hidden)]
pub struct ICashDrawerEventSourceEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ICashDrawerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICashDrawerStatics {
    type Vtable = ICashDrawerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3751843162, 54327, 20479, [181, 71, 221, 169, 105, 164, 248, 131]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICashDrawerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ICashDrawerStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICashDrawerStatics2 {
    type Vtable = ICashDrawerStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1048674593, 35906, 16616, [156, 14, 64, 41, 112, 72, 16, 76]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICashDrawerStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, connectiontypes: PosConnectionTypes, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ICashDrawerStatus(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICashDrawerStatus {
    type Vtable = ICashDrawerStatus_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1807579327, 56481, 19974, [153, 235, 90, 246, 165, 174, 193, 8]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICashDrawerStatus_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut CashDrawerStatusKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ICashDrawerStatusUpdatedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICashDrawerStatusUpdatedEventArgs {
    type Vtable = ICashDrawerStatusUpdatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(816507274, 3440, 17820, [149, 83, 135, 225, 36, 197, 36, 136]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICashDrawerStatusUpdatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IClaimedBarcodeScanner(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IClaimedBarcodeScanner {
    type Vtable = IClaimedBarcodeScanner_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1248048284, 36772, 17202, [187, 38, 148, 93, 17, 216, 30, 15]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedBarcodeScanner_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, symbologies: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, statisticscategories: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, statistics: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, profile: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IClaimedBarcodeScanner1(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IClaimedBarcodeScanner1 {
    type Vtable = IClaimedBarcodeScanner1_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4128943372, 34129, 17076, [153, 140, 151, 12, 32, 33, 10, 34]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedBarcodeScanner1_abi(
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IClaimedBarcodeScanner2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IClaimedBarcodeScanner2 {
    type Vtable = IClaimedBarcodeScanner2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3820330636, 11659, 20336, [138, 243, 52, 72, 190, 221, 95, 226]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedBarcodeScanner2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, barcodesymbology: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, barcodesymbology: u32, attributes: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IClaimedBarcodeScanner3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IClaimedBarcodeScanner3 {
    type Vtable = IClaimedBarcodeScanner3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3872306224, 28974, 17916, [139, 134, 205, 85, 245, 174, 247, 157]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedBarcodeScanner3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IClaimedBarcodeScanner4(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IClaimedBarcodeScanner4 {
    type Vtable = IClaimedBarcodeScanner4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1565532055, 14186, 16808, [162, 48, 47, 55, 193, 148, 157, 222]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedBarcodeScanner4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IClaimedBarcodeScannerClosedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IClaimedBarcodeScannerClosedEventArgs {
    type Vtable = IClaimedBarcodeScannerClosedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3481097353, 41516, 19557, [169, 1, 136, 215, 125, 131, 57, 84]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedBarcodeScannerClosedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IClaimedCashDrawer(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IClaimedCashDrawer {
    type Vtable = IClaimedCashDrawer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3393165743, 43960, 17089, [138, 132, 92, 102, 81, 47, 90, 117]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedCashDrawer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, statisticscategories: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, statistics: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IClaimedCashDrawer2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IClaimedCashDrawer2 {
    type Vtable = IClaimedCashDrawer2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2629481890, 56898, 19803, [176, 193, 155, 87, 162, 186, 137, 195]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedCashDrawer2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IClaimedCashDrawerClosedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IClaimedCashDrawerClosedEventArgs {
    type Vtable = IClaimedCashDrawerClosedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3428269875, 16180, 19548, [186, 174, 222, 173, 241, 108, 215, 250]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedCashDrawerClosedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IClaimedJournalPrinter(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IClaimedJournalPrinter {
    type Vtable = IClaimedJournalPrinter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1743390256, 20861, 18559, [159, 223, 210, 224, 160, 162, 100, 165]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedJournalPrinter_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IClaimedLineDisplay(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IClaimedLineDisplay {
    type Vtable = IClaimedLineDisplay_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(302696816, 39541, 19151, [170, 231, 9, 151, 43, 207, 135, 148]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedLineDisplay_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IClaimedLineDisplay2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IClaimedLineDisplay2 {
    type Vtable = IClaimedLineDisplay2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2736551405, 16885, 20086, [160, 116, 121, 94, 71, 164, 110, 151]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedLineDisplay2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, statisticscategories: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, level: UnifiedPosHealthCheckLevel, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Size) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, attributes: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, descriptor: u32, descriptorstate: LineDisplayDescriptorState, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, viewport: super::super::Foundation::Rect, windowsize: super::super::Foundation::Size, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bitmap: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bitmap: ::windows::runtime::RawPtr, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bitmap: ::windows::runtime::RawPtr, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment, widthinpixels: i32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IClaimedLineDisplay3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IClaimedLineDisplay3 {
    type Vtable = IClaimedLineDisplay3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1680788882, 59860, 20172, [175, 117, 50, 156, 39, 76, 209, 143]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedLineDisplay3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IClaimedLineDisplayClosedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IClaimedLineDisplayClosedEventArgs {
    type Vtable = IClaimedLineDisplayClosedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4178965348, 54229, 20240, [181, 17, 144, 147, 158, 223, 172, 216]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedLineDisplayClosedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IClaimedLineDisplayStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IClaimedLineDisplayStatics {
    type Vtable = IClaimedLineDisplayStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2026543355, 35691, 18803, [134, 240, 62, 87, 12, 53, 24, 37]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedLineDisplayStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, connectiontypes: PosConnectionTypes, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IClaimedMagneticStripeReader(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IClaimedMagneticStripeReader {
    type Vtable = IClaimedMagneticStripeReader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1197254899, 37911, 18620, [185, 215, 65, 99, 167, 132, 76, 2]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedMagneticStripeReader_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: MagneticStripeReaderTrackIds) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MagneticStripeReaderTrackIds) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: MagneticStripeReaderErrorReportingType) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, responseToken_array_size: u32, responsetoken: *const u8, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, responseToken_array_size: u32, responsetoken: *const u8, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, keyname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, statisticscategories: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, statistics: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IClaimedMagneticStripeReader2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IClaimedMagneticStripeReader2 {
    type Vtable = IClaimedMagneticStripeReader2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(594522079, 58076, 19837, [156, 120, 6, 13, 242, 191, 41, 40]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedMagneticStripeReader2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IClaimedMagneticStripeReaderClosedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IClaimedMagneticStripeReaderClosedEventArgs {
    type Vtable = IClaimedMagneticStripeReaderClosedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(346925370, 44493, 19584, [172, 218, 195, 234, 237, 38, 71, 225]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedMagneticStripeReaderClosedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IClaimedPosPrinter(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IClaimedPosPrinter {
    type Vtable = IClaimedPosPrinter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1835322892, 57406, 19220, [163, 142, 194, 140, 52, 184, 99, 83]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedPosPrinter_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: PosPrinterMapMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PosPrinterMapMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, statisticscategories: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, statistics: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IClaimedPosPrinter2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IClaimedPosPrinter2 {
    type Vtable = IClaimedPosPrinter2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1542955989, 20888, 17274, [130, 223, 88, 153, 147, 250, 119, 225]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedPosPrinter2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IClaimedPosPrinterClosedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IClaimedPosPrinterClosedEventArgs {
    type Vtable = IClaimedPosPrinterClosedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3803685499, 19776, 18205, [146, 237, 99, 55, 91, 24, 199, 136]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedPosPrinterClosedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IClaimedReceiptPrinter(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IClaimedReceiptPrinter {
    type Vtable = IClaimedReceiptPrinter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2597485172, 56673, 20194, [152, 55, 91, 93, 114, 213, 56, 185]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedReceiptPrinter_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Size) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Rect) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IClaimedSlipPrinter(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IClaimedSlipPrinter {
    type Vtable = IClaimedSlipPrinter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3177050098, 44944, 20106, [183, 123, 227, 174, 156, 166, 58, 127]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedSlipPrinter_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PosPrinterPrintSide) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Size) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Rect) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, timeout: super::super::Foundation::TimeSpan, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, timeout: super::super::Foundation::TimeSpan, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, printside: PosPrinterPrintSide) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Devices_PointOfService`*"]
pub struct ICommonClaimedPosPrinterStation(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICommonClaimedPosPrinterStation {
    type Vtable = ICommonClaimedPosPrinterStation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3085657768, 65162, 19707, [139, 66, 227, 91, 40, 12, 178, 124]);
}
impl ICommonClaimedPosPrinterStation {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetCharactersPerLine(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CharactersPerLine(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetLineHeight(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn LineHeight(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetLineSpacing(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn LineSpacing(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn LineWidth(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetIsLetterQuality(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsLetterQuality(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsPaperNearEnd(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetColorCartridge(&self, value: PosPrinterColorCartridge) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ColorCartridge(&self) -> ::windows::runtime::Result<PosPrinterColorCartridge> {
        let this = self;
        unsafe {
            let mut result__: PosPrinterColorCartridge = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterColorCartridge>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsCoverOpen(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsCartridgeRemoved(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsCartridgeEmpty(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsHeadCleaning(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsPaperEmpty(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsReadyToPrint(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ValidateData<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, data: Param0) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), data.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ICommonClaimedPosPrinterStation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{b7eb66a8-fe8a-4cfb-8b42-e35b280cb27c}");
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommonClaimedPosPrinterStation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: PosPrinterColorCartridge) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PosPrinterColorCartridge) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, data: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Devices_PointOfService`*"]
pub struct ICommonPosPrintStationCapabilities(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICommonPosPrintStationCapabilities {
    type Vtable = ICommonPosPrintStationCapabilities_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3730526922, 57390, 16617, [158, 94, 27, 72, 142, 106, 172, 252]);
}
impl ICommonPosPrintStationCapabilities {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsPrinterPresent(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDualColorSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ColorCartridgeCapabilities(&self) -> ::windows::runtime::Result<PosPrinterColorCapabilities> {
        let this = self;
        unsafe {
            let mut result__: PosPrinterColorCapabilities = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterColorCapabilities>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CartridgeSensors(&self) -> ::windows::runtime::Result<PosPrinterCartridgeSensors> {
        let this = self;
        unsafe {
            let mut result__: PosPrinterCartridgeSensors = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterCartridgeSensors>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsBoldSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsItalicSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsUnderlineSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDoubleHighPrintSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDoubleWidePrintSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDoubleHighDoubleWidePrintSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsPaperEmptySensorSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsPaperNearEndSensorSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation_Collections`*"]
    pub fn SupportedCharactersPerLine(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ICommonPosPrintStationCapabilities {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{de5b52ca-e02e-40e9-9e5e-1b488e6aacfc}");
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommonPosPrintStationCapabilities_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PosPrinterColorCapabilities) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PosPrinterCartridgeSensors) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Devices_PointOfService`*"]
pub struct ICommonReceiptSlipCapabilities(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ICommonReceiptSlipCapabilities {
    type Vtable = ICommonReceiptSlipCapabilities_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(153643915, 39027, 19717, [191, 190, 71, 39, 166, 3, 143, 105]);
}
impl ICommonReceiptSlipCapabilities {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsBarcodeSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsBitmapSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsLeft90RotationSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsRight90RotationSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Is180RotationSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsPrintAreaSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn RuledLineCapabilities(&self) -> ::windows::runtime::Result<PosPrinterRuledLineCapabilities> {
        let this = self;
        unsafe {
            let mut result__: PosPrinterRuledLineCapabilities = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterRuledLineCapabilities>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation_Collections`*"]
    pub fn SupportedBarcodeRotations(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation_Collections`*"]
    pub fn SupportedBitmapRotations(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsPrinterPresent(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDualColorSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ColorCartridgeCapabilities(&self) -> ::windows::runtime::Result<PosPrinterColorCapabilities> {
        let this = &::windows::runtime::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: PosPrinterColorCapabilities = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterColorCapabilities>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CartridgeSensors(&self) -> ::windows::runtime::Result<PosPrinterCartridgeSensors> {
        let this = &::windows::runtime::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: PosPrinterCartridgeSensors = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterCartridgeSensors>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsBoldSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsItalicSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsUnderlineSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDoubleHighPrintSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDoubleWidePrintSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDoubleHighDoubleWidePrintSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsPaperEmptySensorSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsPaperNearEndSensorSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation_Collections`*"]
    pub fn SupportedCharactersPerLine(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &::windows::runtime::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ICommonReceiptSlipCapabilities {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{09286b8b-9873-4d05-bfbe-4727a6038f69}");
}
impl ::std::convert::TryFrom<ICommonReceiptSlipCapabilities> for ICommonPosPrintStationCapabilities {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ICommonReceiptSlipCapabilities) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&ICommonReceiptSlipCapabilities> for ICommonPosPrintStationCapabilities {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ICommonReceiptSlipCapabilities) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ICommonPosPrintStationCapabilities> for ICommonReceiptSlipCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ICommonPosPrintStationCapabilities> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ICommonPosPrintStationCapabilities> for &ICommonReceiptSlipCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ICommonPosPrintStationCapabilities> {
        ::std::convert::TryInto::<ICommonPosPrintStationCapabilities>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommonReceiptSlipCapabilities_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PosPrinterRuledLineCapabilities) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IJournalPrintJob(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IJournalPrintJob {
    type Vtable = IJournalPrintJob_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2672765028, 62448, 21968, [140, 57, 116, 204, 145, 120, 62, 237]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJournalPrintJob_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, data: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, printoptions: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, linecount: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, distance: i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IJournalPrinterCapabilities(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IJournalPrinterCapabilities {
    type Vtable = IJournalPrinterCapabilities_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(995937347, 57415, 17507, [187, 88, 23, 181, 186, 29, 128, 86]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJournalPrinterCapabilities_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IJournalPrinterCapabilities2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IJournalPrinterCapabilities2 {
    type Vtable = IJournalPrinterCapabilities2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(61912645, 13240, 21307, [186, 170, 164, 56, 146, 131, 171, 10]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJournalPrinterCapabilities2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ILineDisplay(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILineDisplay {
    type Vtable = ILineDisplay_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(620093262, 15513, 17634, [183, 63, 229, 27, 227, 99, 122, 140]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplay_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ILineDisplay2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILineDisplay2 {
    type Vtable = ILineDisplay2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3264652840, 61252, 16627, [189, 28, 176, 76, 106, 92, 220, 125]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplay2_abi(
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ILineDisplayAttributes(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILineDisplayAttributes {
    type Vtable = ILineDisplayAttributes_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3246254492, 8858, 19476, [166, 241, 180, 228, 177, 254, 173, 146]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayAttributes_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Size) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::Size) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ILineDisplayCapabilities(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILineDisplayCapabilities {
    type Vtable = ILineDisplayCapabilities_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1511372241, 36293, 19356, [145, 114, 48, 62, 71, 183, 12, 85]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayCapabilities_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut UnifiedPosPowerReportingType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut LineDisplayTextAttributeGranularity) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut LineDisplayTextAttributeGranularity) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ILineDisplayCursor(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILineDisplayCursor {
    type Vtable = ILineDisplayCursor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3974102085, 30026, 20027, [171, 43, 21, 17, 129, 8, 86, 5]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayCursor_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, attributes: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ILineDisplayCursorAttributes(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILineDisplayCursorAttributes {
    type Vtable = ILineDisplayCursorAttributes_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1311593726, 20477, 16784, [170, 225, 206, 40, 95, 32, 200, 150]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayCursorAttributes_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut LineDisplayCursorType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: LineDisplayCursorType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Point) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::Point) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ILineDisplayCustomGlyphs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILineDisplayCustomGlyphs {
    type Vtable = ILineDisplayCustomGlyphs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(576190012, 62051, 17649, [161, 160, 231, 80, 166, 160, 236, 84]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayCustomGlyphs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Size) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, glyphcode: u32, glyphdata: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ILineDisplayMarquee(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILineDisplayMarquee {
    type Vtable = ILineDisplayMarquee_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2748530238, 62570, 19322, [188, 33, 83, 235, 59, 87, 248, 180]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayMarquee_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut LineDisplayMarqueeFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: LineDisplayMarqueeFormat) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, direction: LineDisplayScrollDirection, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ILineDisplayStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILineDisplayStatics {
    type Vtable = ILineDisplayStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(36552886, 4528, 18064, [149, 71, 11, 57, 197, 175, 33, 20]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, connectiontypes: PosConnectionTypes, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ILineDisplayStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILineDisplayStatics2 {
    type Vtable = ILineDisplayStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1611415324, 30635, 18792, [167, 222, 192, 47, 241, 105, 242, 204]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ILineDisplayStatisticsCategorySelector(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILineDisplayStatisticsCategorySelector {
    type Vtable = ILineDisplayStatisticsCategorySelector_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3038889067, 37492, 19748, [148, 243, 182, 1, 123, 131, 36, 68]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayStatisticsCategorySelector_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ILineDisplayStatusUpdatedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILineDisplayStatusUpdatedEventArgs {
    type Vtable = ILineDisplayStatusUpdatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3721755674, 34555, 20154, [147, 209, 111, 94, 218, 82, 183, 82]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayStatusUpdatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut LineDisplayPowerStatus) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ILineDisplayStoredBitmap(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILineDisplayStoredBitmap {
    type Vtable = ILineDisplayStoredBitmap_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4129378651, 55326, 17338, [191, 27, 188, 250, 60, 120, 91, 160]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayStoredBitmap_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ILineDisplayWindow(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILineDisplayWindow {
    type Vtable = ILineDisplayWindow_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3525308148, 9060, 19429, [190, 225, 133, 22, 128, 175, 73, 100]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayWindow_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::Size) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, text: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, displayattribute: LineDisplayTextAttribute, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, text: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, displayattribute: LineDisplayTextAttribute, startposition: super::super::Foundation::Point, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, text: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, direction: LineDisplayScrollDirection, numberofcolumnsorrows: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ILineDisplayWindow2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILineDisplayWindow2 {
    type Vtable = ILineDisplayWindow2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2841436902, 48600, 17253, [142, 17, 222, 148, 222, 141, 255, 2]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayWindow2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bitmap: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bitmap: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bitmap: ::windows::runtime::RawPtr, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bitmap: ::windows::runtime::RawPtr, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment, widthinpixels: i32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bitmap: ::windows::runtime::RawPtr, offsetinpixels: super::super::Foundation::Point, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bitmap: ::windows::runtime::RawPtr, offsetinpixels: super::super::Foundation::Point, widthinpixels: i32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IMagneticStripeReader(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMagneticStripeReader {
    type Vtable = IMagneticStripeReader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(445820949, 18371, 18058, [147, 51, 12, 101, 23, 87, 72, 131]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReader_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result_size__: *mut u32, result__: *mut *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MagneticStripeReaderAuthenticationProtocol) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, level: UnifiedPosHealthCheckLevel, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, statisticscategories: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MagneticStripeReaderErrorReportingType) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IMagneticStripeReaderAamvaCardDataReceivedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMagneticStripeReaderAamvaCardDataReceivedEventArgs {
    type Vtable = IMagneticStripeReaderAamvaCardDataReceivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(172735825, 49942, 18704, [135, 243, 122, 98, 186, 134, 45, 49]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderAamvaCardDataReceivedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IMagneticStripeReaderBankCardDataReceivedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMagneticStripeReaderBankCardDataReceivedEventArgs {
    type Vtable = IMagneticStripeReaderBankCardDataReceivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(781551651, 41754, 18275, [136, 44, 35, 114, 94, 57, 176, 142]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderBankCardDataReceivedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IMagneticStripeReaderCapabilities(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMagneticStripeReaderCapabilities {
    type Vtable = IMagneticStripeReaderCapabilities_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1898479772, 50240, 17570, [164, 103, 70, 145, 117, 208, 40, 150]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderCapabilities_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MagneticStripeReaderAuthenticationLevel) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut UnifiedPosPowerReportingType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IMagneticStripeReaderCardTypesStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMagneticStripeReaderCardTypesStatics {
    type Vtable = IMagneticStripeReaderCardTypesStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1385114717, 10630, 18255, [132, 84, 124, 205, 5, 146, 141, 95]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderCardTypesStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IMagneticStripeReaderEncryptionAlgorithmsStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMagneticStripeReaderEncryptionAlgorithmsStatics {
    type Vtable = IMagneticStripeReaderEncryptionAlgorithmsStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1404400464, 50139, 18260, [156, 0, 65, 57, 35, 116, 161, 9]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderEncryptionAlgorithmsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IMagneticStripeReaderErrorOccurredEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMagneticStripeReaderErrorOccurredEventArgs {
    type Vtable = IMagneticStripeReaderErrorOccurredEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(535689565, 11396, 16813, [183, 120, 242, 53, 106, 120, 154, 177]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderErrorOccurredEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MagneticStripeReaderTrackErrorType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MagneticStripeReaderTrackErrorType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MagneticStripeReaderTrackErrorType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MagneticStripeReaderTrackErrorType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IMagneticStripeReaderReport(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMagneticStripeReaderReport {
    type Vtable = IMagneticStripeReaderReport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1784373319, 39344, 16776, [190, 241, 237, 223, 121, 247, 143, 230]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderReport_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IMagneticStripeReaderStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMagneticStripeReaderStatics {
    type Vtable = IMagneticStripeReaderStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3294604106, 61399, 18272, [165, 206, 21, 176, 228, 126, 148, 235]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IMagneticStripeReaderStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMagneticStripeReaderStatics2 {
    type Vtable = IMagneticStripeReaderStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2360197986, 54887, 18682, [134, 188, 245, 174, 17, 137, 38, 43]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, connectiontypes: PosConnectionTypes, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IMagneticStripeReaderStatusUpdatedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMagneticStripeReaderStatusUpdatedEventArgs {
    type Vtable = IMagneticStripeReaderStatusUpdatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(164391856, 12898, 16413, [158, 138, 232, 13, 99, 88, 144, 107]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderStatusUpdatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MagneticStripeReaderStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IMagneticStripeReaderTrackData(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMagneticStripeReaderTrackData {
    type Vtable = IMagneticStripeReaderTrackData_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(273479281, 19101, 17518, [171, 197, 32, 64, 35, 7, 186, 54]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderTrackData_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    type Vtable = IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2936689940, 22988, 19040, [153, 232, 153, 165, 61, 172, 229, 170]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPosPrinter(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPosPrinter {
    type Vtable = IPosPrinter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(704889102, 39449, 18945, [153, 79, 18, 223, 173, 106, 220, 191]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinter_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, level: UnifiedPosHealthCheckLevel, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, statisticscategories: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPosPrinter2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPosPrinter2 {
    type Vtable = IPosPrinter2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(612660712, 35736, 21783, [142, 72, 118, 14, 134, 246, 137, 135]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinter2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, typeface: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPosPrinterCapabilities(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPosPrinterCapabilities {
    type Vtable = IPosPrinterCapabilities_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3454621473, 17280, 18821, [173, 197, 57, 219, 48, 205, 147, 188]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinterCapabilities_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut UnifiedPosPowerReportingType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPosPrinterCharacterSetIdsStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPosPrinterCharacterSetIdsStatics {
    type Vtable = IPosPrinterCharacterSetIdsStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1550884607, 28826, 20455, [178, 21, 6, 167, 72, 163, 139, 57]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinterCharacterSetIdsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPosPrinterFontProperty(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPosPrinterFontProperty {
    type Vtable = IPosPrinterFontProperty_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2817845562, 63660, 24324, [132, 210, 41, 177, 109, 138, 99, 60]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinterFontProperty_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Devices_PointOfService`*"]
pub struct IPosPrinterJob(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPosPrinterJob {
    type Vtable = IPosPrinterJob_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2593390684, 1557, 17809, [165, 143, 48, 248, 126, 223, 226, 228]);
}
impl IPosPrinterJob {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Print<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, data: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PrintLine<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, data: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PrintNewline(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn ExecuteAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IPosPrinterJob {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{9a94005c-0615-4591-a58f-30f87edfe2e4}");
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinterJob_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, data: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, data: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPosPrinterPrintOptions(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPosPrinterPrintOptions {
    type Vtable = IPosPrinterPrintOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(170792701, 7426, 23128, [157, 89, 191, 205, 231, 111, 222, 134]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinterPrintOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PosPrinterAlignment) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: PosPrinterAlignment) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPosPrinterReleaseDeviceRequestedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPosPrinterReleaseDeviceRequestedEventArgs {
    type Vtable = IPosPrinterReleaseDeviceRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(734765913, 7407, 16562, [158, 203, 249, 39, 248, 86, 174, 60]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinterReleaseDeviceRequestedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPosPrinterStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPosPrinterStatics {
    type Vtable = IPosPrinterStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2363544810, 4911, 19679, [166, 74, 45, 13, 124, 150, 168, 91]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinterStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deviceid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPosPrinterStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPosPrinterStatics2 {
    type Vtable = IPosPrinterStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4006423580, 45264, 17127, [177, 55, 184, 155, 22, 36, 77, 65]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinterStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, connectiontypes: PosConnectionTypes, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPosPrinterStatus(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPosPrinterStatus {
    type Vtable = IPosPrinterStatus_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3522217776, 55872, 17192, [191, 118, 81, 86, 250, 51, 183, 71]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinterStatus_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PosPrinterStatusKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPosPrinterStatusUpdatedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPosPrinterStatusUpdatedEventArgs {
    type Vtable = IPosPrinterStatusUpdatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(786139103, 5030, 17037, [186, 129, 176, 231, 195, 229, 163, 205]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinterStatusUpdatedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Devices_PointOfService`*"]
pub struct IReceiptOrSlipJob(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IReceiptOrSlipJob {
    type Vtable = IReceiptOrSlipJob_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1394710974, 51395, 19906, [137, 233, 92, 74, 55, 179, 77, 220]);
}
impl IReceiptOrSlipJob {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetBarcodeRotation(&self, value: PosPrinterRotation) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetPrintRotation(&self, value: PosPrinterRotation, includebitmaps: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value, includebitmaps).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn SetPrintArea<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Rect>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Devices_PointOfService`, `Graphics_Imaging`*"]
    pub fn SetBitmap<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmapnumber: u32, bitmap: Param1, alignment: PosPrinterAlignment) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), bitmapnumber, bitmap.into_param().abi(), alignment).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Devices_PointOfService`, `Graphics_Imaging`*"]
    pub fn SetBitmapCustomWidthStandardAlign<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmapnumber: u32, bitmap: Param1, alignment: PosPrinterAlignment, width: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), bitmapnumber, bitmap.into_param().abi(), alignment, width).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Devices_PointOfService`, `Graphics_Imaging`*"]
    pub fn SetCustomAlignedBitmap<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmapnumber: u32, bitmap: Param1, alignmentdistance: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), bitmapnumber, bitmap.into_param().abi(), alignmentdistance).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Devices_PointOfService`, `Graphics_Imaging`*"]
    pub fn SetBitmapCustomWidthCustomAlign<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmapnumber: u32, bitmap: Param1, alignmentdistance: u32, width: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), bitmapnumber, bitmap.into_param().abi(), alignmentdistance, width).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PrintSavedBitmap(&self, bitmapnumber: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), bitmapnumber).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DrawRuledLine<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, positionlist: Param0, linedirection: PosPrinterLineDirection, linewidth: u32, linestyle: PosPrinterLineStyle, linecolor: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), positionlist.into_param().abi(), linedirection, linewidth, linestyle, linecolor).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PrintBarcode<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, data: Param0, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignment: PosPrinterAlignment) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), data.into_param().abi(), symbology, height, width, textposition, alignment).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PrintBarcodeCustomAlign<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, data: Param0, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignmentdistance: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), data.into_param().abi(), symbology, height, width, textposition, alignmentdistance).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Devices_PointOfService`, `Graphics_Imaging`*"]
    pub fn PrintBitmap<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmap: Param0, alignment: PosPrinterAlignment) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), bitmap.into_param().abi(), alignment).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Devices_PointOfService`, `Graphics_Imaging`*"]
    pub fn PrintBitmapCustomWidthStandardAlign<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmap: Param0, alignment: PosPrinterAlignment, width: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), bitmap.into_param().abi(), alignment, width).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Devices_PointOfService`, `Graphics_Imaging`*"]
    pub fn PrintCustomAlignedBitmap<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmap: Param0, alignmentdistance: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), bitmap.into_param().abi(), alignmentdistance).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Devices_PointOfService`, `Graphics_Imaging`*"]
    pub fn PrintBitmapCustomWidthCustomAlign<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmap: Param0, alignmentdistance: u32, width: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), bitmap.into_param().abi(), alignmentdistance, width).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Print<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, data: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PrintLine<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, data: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PrintNewline(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn ExecuteAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::runtime::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IReceiptOrSlipJob {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{532199be-c8c3-4dc2-89e9-5c4a37b34ddc}");
}
impl ::std::convert::TryFrom<IReceiptOrSlipJob> for IPosPrinterJob {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IReceiptOrSlipJob) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&IReceiptOrSlipJob> for IPosPrinterJob {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IReceiptOrSlipJob) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPosPrinterJob> for IReceiptOrSlipJob {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPosPrinterJob> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPosPrinterJob> for &IReceiptOrSlipJob {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPosPrinterJob> {
        ::std::convert::TryInto::<IPosPrinterJob>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IReceiptOrSlipJob_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: PosPrinterRotation) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: PosPrinterRotation, includebitmaps: bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::Rect) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bitmapnumber: u32, bitmap: ::windows::runtime::RawPtr, alignment: PosPrinterAlignment) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))] usize,
    #[cfg(feature = "Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bitmapnumber: u32, bitmap: ::windows::runtime::RawPtr, alignment: PosPrinterAlignment, width: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))] usize,
    #[cfg(feature = "Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bitmapnumber: u32, bitmap: ::windows::runtime::RawPtr, alignmentdistance: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))] usize,
    #[cfg(feature = "Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bitmapnumber: u32, bitmap: ::windows::runtime::RawPtr, alignmentdistance: u32, width: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bitmapnumber: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, positionlist: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, linedirection: PosPrinterLineDirection, linewidth: u32, linestyle: PosPrinterLineStyle, linecolor: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, data: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignment: PosPrinterAlignment) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, data: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignmentdistance: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bitmap: ::windows::runtime::RawPtr, alignment: PosPrinterAlignment) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))] usize,
    #[cfg(feature = "Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bitmap: ::windows::runtime::RawPtr, alignment: PosPrinterAlignment, width: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))] usize,
    #[cfg(feature = "Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bitmap: ::windows::runtime::RawPtr, alignmentdistance: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))] usize,
    #[cfg(feature = "Graphics_Imaging")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bitmap: ::windows::runtime::RawPtr, alignmentdistance: u32, width: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IReceiptPrintJob(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IReceiptPrintJob {
    type Vtable = IReceiptPrintJob_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2861958766, 44205, 19321, [157, 15, 192, 207, 192, 141, 199, 123]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReceiptPrintJob_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, kind: PosPrinterMarkFeedKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, percentage: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IReceiptPrintJob2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IReceiptPrintJob2 {
    type Vtable = IReceiptPrintJob2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(213652195, 40489, 20857, [188, 216, 24, 17, 211, 185, 161, 14]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReceiptPrintJob2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, data: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, printoptions: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, linecount: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, distance: i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IReceiptPrinterCapabilities(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IReceiptPrinterCapabilities {
    type Vtable = IReceiptPrinterCapabilities_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3102782863, 20904, 17404, [155, 213, 141, 226, 114, 166, 65, 91]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReceiptPrinterCapabilities_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PosPrinterMarkFeedCapabilities) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IReceiptPrinterCapabilities2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IReceiptPrinterCapabilities2 {
    type Vtable = IReceiptPrinterCapabilities2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(537069112, 35372, 21932, [154, 123, 117, 118, 216, 134, 158, 153]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReceiptPrinterCapabilities2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ISlipPrintJob(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISlipPrintJob {
    type Vtable = ISlipPrintJob_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1569257821, 24881, 23115, [183, 213, 142, 242, 218, 123, 65, 101]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISlipPrintJob_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, data: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, printoptions: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, linecount: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, distance: i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ISlipPrinterCapabilities(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISlipPrinterCapabilities {
    type Vtable = ISlipPrinterCapabilities_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2578539417, 18572, 16727, [138, 194, 159, 87, 247, 8, 211, 219]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISlipPrinterCapabilities_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct ISlipPrinterCapabilities2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISlipPrinterCapabilities2 {
    type Vtable = ISlipPrinterCapabilities2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1878562417, 11546, 20480, [135, 194, 176, 133, 27, 253, 240, 126]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISlipPrinterCapabilities2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IUnifiedPosErrorData(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUnifiedPosErrorData {
    type Vtable = IUnifiedPosErrorData_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(731483194, 21852, 18569, [142, 216, 197, 153, 187, 58, 113, 42]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnifiedPosErrorData_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut UnifiedPosErrorSeverity) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut UnifiedPosErrorReason) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IUnifiedPosErrorDataFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUnifiedPosErrorDataFactory {
    type Vtable = IUnifiedPosErrorDataFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1268262225, 8190, 17691, [163, 104, 99, 224, 206, 70, 95, 90]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnifiedPosErrorDataFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, message: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, severity: UnifiedPosErrorSeverity, reason: UnifiedPosErrorReason, extendedreason: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct JournalPrintJob(::windows::runtime::IInspectable);
impl JournalPrintJob {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Print<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, data: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PrintLine<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, data: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PrintNewline(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn ExecuteAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Print2<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, PosPrinterPrintOptions>>(&self, data: Param0, printoptions: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IJournalPrintJob>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), data.into_param().abi(), printoptions.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn FeedPaperByLine(&self, linecount: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IJournalPrintJob>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), linecount).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn FeedPaperByMapModeUnit(&self, distance: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IJournalPrintJob>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), distance).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for JournalPrintJob {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.JournalPrintJob;{9a94005c-0615-4591-a58f-30f87edfe2e4})");
}
unsafe impl ::windows::runtime::Interface for JournalPrintJob {
    type Vtable = IPosPrinterJob_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2593390684, 1557, 17809, [165, 143, 48, 248, 126, 223, 226, 228]);
}
impl ::windows::runtime::RuntimeName for JournalPrintJob {
    const NAME: &'static str = "Windows.Devices.PointOfService.JournalPrintJob";
}
impl ::std::convert::From<JournalPrintJob> for ::windows::runtime::IUnknown {
    fn from(value: JournalPrintJob) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&JournalPrintJob> for ::windows::runtime::IUnknown {
    fn from(value: &JournalPrintJob) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for JournalPrintJob {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &JournalPrintJob {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<JournalPrintJob> for ::windows::runtime::IInspectable {
    fn from(value: JournalPrintJob) -> Self {
        value.0
    }
}
impl ::std::convert::From<&JournalPrintJob> for ::windows::runtime::IInspectable {
    fn from(value: &JournalPrintJob) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for JournalPrintJob {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a JournalPrintJob {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<JournalPrintJob> for IPosPrinterJob {
    fn from(value: JournalPrintJob) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&JournalPrintJob> for IPosPrinterJob {
    fn from(value: &JournalPrintJob) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPosPrinterJob> for JournalPrintJob {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPosPrinterJob> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IPosPrinterJob>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPosPrinterJob> for &JournalPrintJob {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPosPrinterJob> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IPosPrinterJob>::into(::std::clone::Clone::clone(self)))
    }
}
unsafe impl ::std::marker::Send for JournalPrintJob {}
unsafe impl ::std::marker::Sync for JournalPrintJob {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct JournalPrinterCapabilities(::windows::runtime::IInspectable);
impl JournalPrinterCapabilities {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsPrinterPresent(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDualColorSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ColorCartridgeCapabilities(&self) -> ::windows::runtime::Result<PosPrinterColorCapabilities> {
        let this = &::windows::runtime::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: PosPrinterColorCapabilities = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterColorCapabilities>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CartridgeSensors(&self) -> ::windows::runtime::Result<PosPrinterCartridgeSensors> {
        let this = &::windows::runtime::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: PosPrinterCartridgeSensors = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterCartridgeSensors>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsBoldSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsItalicSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsUnderlineSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDoubleHighPrintSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDoubleWidePrintSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDoubleHighDoubleWidePrintSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsPaperEmptySensorSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsPaperNearEndSensorSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation_Collections`*"]
    pub fn SupportedCharactersPerLine(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &::windows::runtime::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsReverseVideoSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IJournalPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsStrikethroughSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IJournalPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsSuperscriptSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IJournalPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsSubscriptSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IJournalPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsReversePaperFeedByLineSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IJournalPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsReversePaperFeedByMapModeUnitSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IJournalPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for JournalPrinterCapabilities {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.JournalPrinterCapabilities;{3b5ccc43-e047-4463-bb58-17b5ba1d8056})");
}
unsafe impl ::windows::runtime::Interface for JournalPrinterCapabilities {
    type Vtable = IJournalPrinterCapabilities_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(995937347, 57415, 17507, [187, 88, 23, 181, 186, 29, 128, 86]);
}
impl ::windows::runtime::RuntimeName for JournalPrinterCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.JournalPrinterCapabilities";
}
impl ::std::convert::From<JournalPrinterCapabilities> for ::windows::runtime::IUnknown {
    fn from(value: JournalPrinterCapabilities) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&JournalPrinterCapabilities> for ::windows::runtime::IUnknown {
    fn from(value: &JournalPrinterCapabilities) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for JournalPrinterCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &JournalPrinterCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<JournalPrinterCapabilities> for ::windows::runtime::IInspectable {
    fn from(value: JournalPrinterCapabilities) -> Self {
        value.0
    }
}
impl ::std::convert::From<&JournalPrinterCapabilities> for ::windows::runtime::IInspectable {
    fn from(value: &JournalPrinterCapabilities) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for JournalPrinterCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a JournalPrinterCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<JournalPrinterCapabilities> for ICommonPosPrintStationCapabilities {
    type Error = ::windows::runtime::Error;
    fn try_from(value: JournalPrinterCapabilities) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&JournalPrinterCapabilities> for ICommonPosPrintStationCapabilities {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &JournalPrinterCapabilities) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ICommonPosPrintStationCapabilities> for JournalPrinterCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ICommonPosPrintStationCapabilities> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ICommonPosPrintStationCapabilities> for &JournalPrinterCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ICommonPosPrintStationCapabilities> {
        ::std::convert::TryInto::<ICommonPosPrintStationCapabilities>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for JournalPrinterCapabilities {}
unsafe impl ::std::marker::Sync for JournalPrinterCapabilities {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct LineDisplay(::windows::runtime::IInspectable);
impl LineDisplay {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Capabilities(&self) -> ::windows::runtime::Result<LineDisplayCapabilities> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LineDisplayCapabilities>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PhysicalDeviceName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PhysicalDeviceDescription(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DeviceControlDescription(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DeviceControlVersion(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DeviceServiceVersion(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn ClaimAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ClaimedLineDisplay>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ClaimedLineDisplay>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn FromIdAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(deviceid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<LineDisplay>> {
        Self::ILineDisplayStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LineDisplay>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn GetDefaultAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<LineDisplay>> {
        Self::ILineDisplayStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LineDisplay>>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn GetDeviceSelector() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::ILineDisplayStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn GetDeviceSelectorWithConnectionTypes(connectiontypes: PosConnectionTypes) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::ILineDisplayStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), connectiontypes, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn CheckPowerStatusAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<LineDisplayPowerStatus>> {
        let this = &::windows::runtime::Interface::cast::<ILineDisplay2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LineDisplayPowerStatus>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn StatisticsCategorySelector() -> ::windows::runtime::Result<LineDisplayStatisticsCategorySelector> {
        Self::ILineDisplayStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LineDisplayStatisticsCategorySelector>(result__)
        })
    }
    pub fn ILineDisplayStatics<R, F: FnOnce(&ILineDisplayStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<LineDisplay, ILineDisplayStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ILineDisplayStatics2<R, F: FnOnce(&ILineDisplayStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<LineDisplay, ILineDisplayStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LineDisplay {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplay;{24f5df4e-3c99-44e2-b73f-e51be3637a8c})");
}
unsafe impl ::windows::runtime::Interface for LineDisplay {
    type Vtable = ILineDisplay_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(620093262, 15513, 17634, [183, 63, 229, 27, 227, 99, 122, 140]);
}
impl ::windows::runtime::RuntimeName for LineDisplay {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplay";
}
impl ::std::convert::From<LineDisplay> for ::windows::runtime::IUnknown {
    fn from(value: LineDisplay) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&LineDisplay> for ::windows::runtime::IUnknown {
    fn from(value: &LineDisplay) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LineDisplay {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &LineDisplay {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<LineDisplay> for ::windows::runtime::IInspectable {
    fn from(value: LineDisplay) -> Self {
        value.0
    }
}
impl ::std::convert::From<&LineDisplay> for ::windows::runtime::IInspectable {
    fn from(value: &LineDisplay) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LineDisplay {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LineDisplay {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<LineDisplay> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: LineDisplay) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&LineDisplay> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &LineDisplay) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for LineDisplay {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &LineDisplay {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for LineDisplay {}
unsafe impl ::std::marker::Sync for LineDisplay {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct LineDisplayAttributes(::windows::runtime::IInspectable);
impl LineDisplayAttributes {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsPowerNotifyEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetIsPowerNotifyEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Brightness(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetBrightness(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn BlinkRate(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn SetBlinkRate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn ScreenSizeInCharacters(&self) -> ::windows::runtime::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Size = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn SetScreenSizeInCharacters<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Size>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CharacterSet(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetCharacterSet(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsCharacterSetMappingEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetIsCharacterSetMappingEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CurrentWindow(&self) -> ::windows::runtime::Result<LineDisplayWindow> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LineDisplayWindow>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetCurrentWindow<'a, Param0: ::windows::runtime::IntoParam<'a, LineDisplayWindow>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LineDisplayAttributes {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplayAttributes;{c17de99c-229a-4c14-a6f1-b4e4b1fead92})");
}
unsafe impl ::windows::runtime::Interface for LineDisplayAttributes {
    type Vtable = ILineDisplayAttributes_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3246254492, 8858, 19476, [166, 241, 180, 228, 177, 254, 173, 146]);
}
impl ::windows::runtime::RuntimeName for LineDisplayAttributes {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayAttributes";
}
impl ::std::convert::From<LineDisplayAttributes> for ::windows::runtime::IUnknown {
    fn from(value: LineDisplayAttributes) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&LineDisplayAttributes> for ::windows::runtime::IUnknown {
    fn from(value: &LineDisplayAttributes) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LineDisplayAttributes {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &LineDisplayAttributes {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<LineDisplayAttributes> for ::windows::runtime::IInspectable {
    fn from(value: LineDisplayAttributes) -> Self {
        value.0
    }
}
impl ::std::convert::From<&LineDisplayAttributes> for ::windows::runtime::IInspectable {
    fn from(value: &LineDisplayAttributes) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LineDisplayAttributes {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LineDisplayAttributes {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for LineDisplayAttributes {}
unsafe impl ::std::marker::Sync for LineDisplayAttributes {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct LineDisplayCapabilities(::windows::runtime::IInspectable);
impl LineDisplayCapabilities {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsStatisticsReportingSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsStatisticsUpdatingSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PowerReportingType(&self) -> ::windows::runtime::Result<UnifiedPosPowerReportingType> {
        let this = self;
        unsafe {
            let mut result__: UnifiedPosPowerReportingType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UnifiedPosPowerReportingType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CanChangeScreenSize(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CanDisplayBitmaps(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CanReadCharacterAtCursor(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CanMapCharacterSets(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CanDisplayCustomGlyphs(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CanReverse(&self) -> ::windows::runtime::Result<LineDisplayTextAttributeGranularity> {
        let this = self;
        unsafe {
            let mut result__: LineDisplayTextAttributeGranularity = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LineDisplayTextAttributeGranularity>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CanBlink(&self) -> ::windows::runtime::Result<LineDisplayTextAttributeGranularity> {
        let this = self;
        unsafe {
            let mut result__: LineDisplayTextAttributeGranularity = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LineDisplayTextAttributeGranularity>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CanChangeBlinkRate(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsBrightnessSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsCursorSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsHorizontalMarqueeSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsVerticalMarqueeSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsInterCharacterWaitSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SupportedDescriptors(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SupportedWindows(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LineDisplayCapabilities {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplayCapabilities;{5a15b5d1-8dc5-4b9c-9172-303e47b70c55})");
}
unsafe impl ::windows::runtime::Interface for LineDisplayCapabilities {
    type Vtable = ILineDisplayCapabilities_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1511372241, 36293, 19356, [145, 114, 48, 62, 71, 183, 12, 85]);
}
impl ::windows::runtime::RuntimeName for LineDisplayCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayCapabilities";
}
impl ::std::convert::From<LineDisplayCapabilities> for ::windows::runtime::IUnknown {
    fn from(value: LineDisplayCapabilities) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&LineDisplayCapabilities> for ::windows::runtime::IUnknown {
    fn from(value: &LineDisplayCapabilities) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LineDisplayCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &LineDisplayCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<LineDisplayCapabilities> for ::windows::runtime::IInspectable {
    fn from(value: LineDisplayCapabilities) -> Self {
        value.0
    }
}
impl ::std::convert::From<&LineDisplayCapabilities> for ::windows::runtime::IInspectable {
    fn from(value: &LineDisplayCapabilities) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LineDisplayCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LineDisplayCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for LineDisplayCapabilities {}
unsafe impl ::std::marker::Sync for LineDisplayCapabilities {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct LineDisplayCursor(::windows::runtime::IInspectable);
impl LineDisplayCursor {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CanCustomize(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsBlinkSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsBlockSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsHalfBlockSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsUnderlineSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsReverseSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsOtherSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn GetAttributes(&self) -> ::windows::runtime::Result<LineDisplayCursorAttributes> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LineDisplayCursorAttributes>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn TryUpdateAttributesAsync<'a, Param0: ::windows::runtime::IntoParam<'a, LineDisplayCursorAttributes>>(&self, attributes: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), attributes.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LineDisplayCursor {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplayCursor;{ecdffc45-754a-4e3b-ab2b-151181085605})");
}
unsafe impl ::windows::runtime::Interface for LineDisplayCursor {
    type Vtable = ILineDisplayCursor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3974102085, 30026, 20027, [171, 43, 21, 17, 129, 8, 86, 5]);
}
impl ::windows::runtime::RuntimeName for LineDisplayCursor {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayCursor";
}
impl ::std::convert::From<LineDisplayCursor> for ::windows::runtime::IUnknown {
    fn from(value: LineDisplayCursor) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&LineDisplayCursor> for ::windows::runtime::IUnknown {
    fn from(value: &LineDisplayCursor) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LineDisplayCursor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &LineDisplayCursor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<LineDisplayCursor> for ::windows::runtime::IInspectable {
    fn from(value: LineDisplayCursor) -> Self {
        value.0
    }
}
impl ::std::convert::From<&LineDisplayCursor> for ::windows::runtime::IInspectable {
    fn from(value: &LineDisplayCursor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LineDisplayCursor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LineDisplayCursor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for LineDisplayCursor {}
unsafe impl ::std::marker::Sync for LineDisplayCursor {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct LineDisplayCursorAttributes(::windows::runtime::IInspectable);
impl LineDisplayCursorAttributes {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsBlinkEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetIsBlinkEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CursorType(&self) -> ::windows::runtime::Result<LineDisplayCursorType> {
        let this = self;
        unsafe {
            let mut result__: LineDisplayCursorType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LineDisplayCursorType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetCursorType(&self, value: LineDisplayCursorType) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsAutoAdvanceEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetIsAutoAdvanceEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn Position(&self) -> ::windows::runtime::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn SetPosition<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Point>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LineDisplayCursorAttributes {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplayCursorAttributes;{4e2d54fe-4ffd-4190-aae1-ce285f20c896})");
}
unsafe impl ::windows::runtime::Interface for LineDisplayCursorAttributes {
    type Vtable = ILineDisplayCursorAttributes_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1311593726, 20477, 16784, [170, 225, 206, 40, 95, 32, 200, 150]);
}
impl ::windows::runtime::RuntimeName for LineDisplayCursorAttributes {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayCursorAttributes";
}
impl ::std::convert::From<LineDisplayCursorAttributes> for ::windows::runtime::IUnknown {
    fn from(value: LineDisplayCursorAttributes) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&LineDisplayCursorAttributes> for ::windows::runtime::IUnknown {
    fn from(value: &LineDisplayCursorAttributes) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LineDisplayCursorAttributes {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &LineDisplayCursorAttributes {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<LineDisplayCursorAttributes> for ::windows::runtime::IInspectable {
    fn from(value: LineDisplayCursorAttributes) -> Self {
        value.0
    }
}
impl ::std::convert::From<&LineDisplayCursorAttributes> for ::windows::runtime::IInspectable {
    fn from(value: &LineDisplayCursorAttributes) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LineDisplayCursorAttributes {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LineDisplayCursorAttributes {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for LineDisplayCursorAttributes {}
unsafe impl ::std::marker::Sync for LineDisplayCursorAttributes {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for LineDisplayCursorType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LineDisplayCursorType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for LineDisplayCursorType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.LineDisplayCursorType;i4)");
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct LineDisplayCustomGlyphs(::windows::runtime::IInspectable);
impl LineDisplayCustomGlyphs {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn SizeInPixels(&self) -> ::windows::runtime::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Size = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation_Collections`*"]
    pub fn SupportedGlyphCodes(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`, `Storage_Streams`*"]
    pub fn TryRedefineAsync<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, glyphcode: u32, glyphdata: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), glyphcode, glyphdata.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LineDisplayCustomGlyphs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplayCustomGlyphs;{2257f63c-f263-44f1-a1a0-e750a6a0ec54})");
}
unsafe impl ::windows::runtime::Interface for LineDisplayCustomGlyphs {
    type Vtable = ILineDisplayCustomGlyphs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(576190012, 62051, 17649, [161, 160, 231, 80, 166, 160, 236, 84]);
}
impl ::windows::runtime::RuntimeName for LineDisplayCustomGlyphs {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayCustomGlyphs";
}
impl ::std::convert::From<LineDisplayCustomGlyphs> for ::windows::runtime::IUnknown {
    fn from(value: LineDisplayCustomGlyphs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&LineDisplayCustomGlyphs> for ::windows::runtime::IUnknown {
    fn from(value: &LineDisplayCustomGlyphs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LineDisplayCustomGlyphs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &LineDisplayCustomGlyphs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<LineDisplayCustomGlyphs> for ::windows::runtime::IInspectable {
    fn from(value: LineDisplayCustomGlyphs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&LineDisplayCustomGlyphs> for ::windows::runtime::IInspectable {
    fn from(value: &LineDisplayCustomGlyphs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LineDisplayCustomGlyphs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LineDisplayCustomGlyphs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for LineDisplayCustomGlyphs {}
unsafe impl ::std::marker::Sync for LineDisplayCustomGlyphs {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct LineDisplayDescriptorState(pub i32);
impl LineDisplayDescriptorState {
    pub const Off: LineDisplayDescriptorState = LineDisplayDescriptorState(0i32);
    pub const On: LineDisplayDescriptorState = LineDisplayDescriptorState(1i32);
    pub const Blink: LineDisplayDescriptorState = LineDisplayDescriptorState(2i32);
}
impl ::std::convert::From<i32> for LineDisplayDescriptorState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LineDisplayDescriptorState {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for LineDisplayDescriptorState {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.LineDisplayDescriptorState;i4)");
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct LineDisplayHorizontalAlignment(pub i32);
impl LineDisplayHorizontalAlignment {
    pub const Left: LineDisplayHorizontalAlignment = LineDisplayHorizontalAlignment(0i32);
    pub const Center: LineDisplayHorizontalAlignment = LineDisplayHorizontalAlignment(1i32);
    pub const Right: LineDisplayHorizontalAlignment = LineDisplayHorizontalAlignment(2i32);
}
impl ::std::convert::From<i32> for LineDisplayHorizontalAlignment {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LineDisplayHorizontalAlignment {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for LineDisplayHorizontalAlignment {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.LineDisplayHorizontalAlignment;i4)");
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct LineDisplayMarquee(::windows::runtime::IInspectable);
impl LineDisplayMarquee {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Format(&self) -> ::windows::runtime::Result<LineDisplayMarqueeFormat> {
        let this = self;
        unsafe {
            let mut result__: LineDisplayMarqueeFormat = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LineDisplayMarqueeFormat>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetFormat(&self, value: LineDisplayMarqueeFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RepeatWaitInterval(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn SetRepeatWaitInterval<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn ScrollWaitInterval(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn SetScrollWaitInterval<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn TryStartScrollingAsync(&self, direction: LineDisplayScrollDirection) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), direction, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn TryStopScrollingAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LineDisplayMarquee {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplayMarquee;{a3d33e3e-f46a-4b7a-bc21-53eb3b57f8b4})");
}
unsafe impl ::windows::runtime::Interface for LineDisplayMarquee {
    type Vtable = ILineDisplayMarquee_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2748530238, 62570, 19322, [188, 33, 83, 235, 59, 87, 248, 180]);
}
impl ::windows::runtime::RuntimeName for LineDisplayMarquee {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayMarquee";
}
impl ::std::convert::From<LineDisplayMarquee> for ::windows::runtime::IUnknown {
    fn from(value: LineDisplayMarquee) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&LineDisplayMarquee> for ::windows::runtime::IUnknown {
    fn from(value: &LineDisplayMarquee) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LineDisplayMarquee {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &LineDisplayMarquee {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<LineDisplayMarquee> for ::windows::runtime::IInspectable {
    fn from(value: LineDisplayMarquee) -> Self {
        value.0
    }
}
impl ::std::convert::From<&LineDisplayMarquee> for ::windows::runtime::IInspectable {
    fn from(value: &LineDisplayMarquee) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LineDisplayMarquee {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LineDisplayMarquee {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for LineDisplayMarquee {}
unsafe impl ::std::marker::Sync for LineDisplayMarquee {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct LineDisplayMarqueeFormat(pub i32);
impl LineDisplayMarqueeFormat {
    pub const None: LineDisplayMarqueeFormat = LineDisplayMarqueeFormat(0i32);
    pub const Walk: LineDisplayMarqueeFormat = LineDisplayMarqueeFormat(1i32);
    pub const Place: LineDisplayMarqueeFormat = LineDisplayMarqueeFormat(2i32);
}
impl ::std::convert::From<i32> for LineDisplayMarqueeFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LineDisplayMarqueeFormat {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for LineDisplayMarqueeFormat {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.LineDisplayMarqueeFormat;i4)");
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct LineDisplayPowerStatus(pub i32);
impl LineDisplayPowerStatus {
    pub const Unknown: LineDisplayPowerStatus = LineDisplayPowerStatus(0i32);
    pub const Online: LineDisplayPowerStatus = LineDisplayPowerStatus(1i32);
    pub const Off: LineDisplayPowerStatus = LineDisplayPowerStatus(2i32);
    pub const Offline: LineDisplayPowerStatus = LineDisplayPowerStatus(3i32);
    pub const OffOrOffline: LineDisplayPowerStatus = LineDisplayPowerStatus(4i32);
}
impl ::std::convert::From<i32> for LineDisplayPowerStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LineDisplayPowerStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for LineDisplayPowerStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.LineDisplayPowerStatus;i4)");
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct LineDisplayScrollDirection(pub i32);
impl LineDisplayScrollDirection {
    pub const Up: LineDisplayScrollDirection = LineDisplayScrollDirection(0i32);
    pub const Down: LineDisplayScrollDirection = LineDisplayScrollDirection(1i32);
    pub const Left: LineDisplayScrollDirection = LineDisplayScrollDirection(2i32);
    pub const Right: LineDisplayScrollDirection = LineDisplayScrollDirection(3i32);
}
impl ::std::convert::From<i32> for LineDisplayScrollDirection {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LineDisplayScrollDirection {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for LineDisplayScrollDirection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.LineDisplayScrollDirection;i4)");
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct LineDisplayStatisticsCategorySelector(::windows::runtime::IInspectable);
impl LineDisplayStatisticsCategorySelector {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn AllStatistics(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn UnifiedPosStatistics(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ManufacturerStatistics(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LineDisplayStatisticsCategorySelector {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplayStatisticsCategorySelector;{b521c46b-9274-4d24-94f3-b6017b832444})");
}
unsafe impl ::windows::runtime::Interface for LineDisplayStatisticsCategorySelector {
    type Vtable = ILineDisplayStatisticsCategorySelector_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3038889067, 37492, 19748, [148, 243, 182, 1, 123, 131, 36, 68]);
}
impl ::windows::runtime::RuntimeName for LineDisplayStatisticsCategorySelector {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayStatisticsCategorySelector";
}
impl ::std::convert::From<LineDisplayStatisticsCategorySelector> for ::windows::runtime::IUnknown {
    fn from(value: LineDisplayStatisticsCategorySelector) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&LineDisplayStatisticsCategorySelector> for ::windows::runtime::IUnknown {
    fn from(value: &LineDisplayStatisticsCategorySelector) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LineDisplayStatisticsCategorySelector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &LineDisplayStatisticsCategorySelector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<LineDisplayStatisticsCategorySelector> for ::windows::runtime::IInspectable {
    fn from(value: LineDisplayStatisticsCategorySelector) -> Self {
        value.0
    }
}
impl ::std::convert::From<&LineDisplayStatisticsCategorySelector> for ::windows::runtime::IInspectable {
    fn from(value: &LineDisplayStatisticsCategorySelector) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LineDisplayStatisticsCategorySelector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LineDisplayStatisticsCategorySelector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for LineDisplayStatisticsCategorySelector {}
unsafe impl ::std::marker::Sync for LineDisplayStatisticsCategorySelector {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct LineDisplayStatusUpdatedEventArgs(::windows::runtime::IInspectable);
impl LineDisplayStatusUpdatedEventArgs {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<LineDisplayPowerStatus> {
        let this = self;
        unsafe {
            let mut result__: LineDisplayPowerStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LineDisplayPowerStatus>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LineDisplayStatusUpdatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplayStatusUpdatedEventArgs;{ddd57c1a-86fb-4eba-93d1-6f5eda52b752})");
}
unsafe impl ::windows::runtime::Interface for LineDisplayStatusUpdatedEventArgs {
    type Vtable = ILineDisplayStatusUpdatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3721755674, 34555, 20154, [147, 209, 111, 94, 218, 82, 183, 82]);
}
impl ::windows::runtime::RuntimeName for LineDisplayStatusUpdatedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayStatusUpdatedEventArgs";
}
impl ::std::convert::From<LineDisplayStatusUpdatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: LineDisplayStatusUpdatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&LineDisplayStatusUpdatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &LineDisplayStatusUpdatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LineDisplayStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &LineDisplayStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<LineDisplayStatusUpdatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: LineDisplayStatusUpdatedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&LineDisplayStatusUpdatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &LineDisplayStatusUpdatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LineDisplayStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LineDisplayStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for LineDisplayStatusUpdatedEventArgs {}
unsafe impl ::std::marker::Sync for LineDisplayStatusUpdatedEventArgs {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct LineDisplayStoredBitmap(::windows::runtime::IInspectable);
impl LineDisplayStoredBitmap {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn EscapeSequence(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn TryDeleteAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LineDisplayStoredBitmap {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplayStoredBitmap;{f621515b-d81e-43ba-bf1b-bcfa3c785ba0})");
}
unsafe impl ::windows::runtime::Interface for LineDisplayStoredBitmap {
    type Vtable = ILineDisplayStoredBitmap_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4129378651, 55326, 17338, [191, 27, 188, 250, 60, 120, 91, 160]);
}
impl ::windows::runtime::RuntimeName for LineDisplayStoredBitmap {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayStoredBitmap";
}
impl ::std::convert::From<LineDisplayStoredBitmap> for ::windows::runtime::IUnknown {
    fn from(value: LineDisplayStoredBitmap) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&LineDisplayStoredBitmap> for ::windows::runtime::IUnknown {
    fn from(value: &LineDisplayStoredBitmap) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LineDisplayStoredBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &LineDisplayStoredBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<LineDisplayStoredBitmap> for ::windows::runtime::IInspectable {
    fn from(value: LineDisplayStoredBitmap) -> Self {
        value.0
    }
}
impl ::std::convert::From<&LineDisplayStoredBitmap> for ::windows::runtime::IInspectable {
    fn from(value: &LineDisplayStoredBitmap) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LineDisplayStoredBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LineDisplayStoredBitmap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for LineDisplayStoredBitmap {}
unsafe impl ::std::marker::Sync for LineDisplayStoredBitmap {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct LineDisplayTextAttribute(pub i32);
impl LineDisplayTextAttribute {
    pub const Normal: LineDisplayTextAttribute = LineDisplayTextAttribute(0i32);
    pub const Blink: LineDisplayTextAttribute = LineDisplayTextAttribute(1i32);
    pub const Reverse: LineDisplayTextAttribute = LineDisplayTextAttribute(2i32);
    pub const ReverseBlink: LineDisplayTextAttribute = LineDisplayTextAttribute(3i32);
}
impl ::std::convert::From<i32> for LineDisplayTextAttribute {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LineDisplayTextAttribute {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for LineDisplayTextAttribute {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.LineDisplayTextAttribute;i4)");
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct LineDisplayTextAttributeGranularity(pub i32);
impl LineDisplayTextAttributeGranularity {
    pub const NotSupported: LineDisplayTextAttributeGranularity = LineDisplayTextAttributeGranularity(0i32);
    pub const EntireDisplay: LineDisplayTextAttributeGranularity = LineDisplayTextAttributeGranularity(1i32);
    pub const PerCharacter: LineDisplayTextAttributeGranularity = LineDisplayTextAttributeGranularity(2i32);
}
impl ::std::convert::From<i32> for LineDisplayTextAttributeGranularity {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LineDisplayTextAttributeGranularity {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for LineDisplayTextAttributeGranularity {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.LineDisplayTextAttributeGranularity;i4)");
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct LineDisplayVerticalAlignment(pub i32);
impl LineDisplayVerticalAlignment {
    pub const Top: LineDisplayVerticalAlignment = LineDisplayVerticalAlignment(0i32);
    pub const Center: LineDisplayVerticalAlignment = LineDisplayVerticalAlignment(1i32);
    pub const Bottom: LineDisplayVerticalAlignment = LineDisplayVerticalAlignment(2i32);
}
impl ::std::convert::From<i32> for LineDisplayVerticalAlignment {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LineDisplayVerticalAlignment {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for LineDisplayVerticalAlignment {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.LineDisplayVerticalAlignment;i4)");
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct LineDisplayWindow(::windows::runtime::IInspectable);
impl LineDisplayWindow {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn SizeInCharacters(&self) -> ::windows::runtime::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Size = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn InterCharacterWaitInterval(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn SetInterCharacterWaitInterval<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn TryRefreshAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn TryDisplayTextAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, text: Param0, displayattribute: LineDisplayTextAttribute) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), text.into_param().abi(), displayattribute, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn TryDisplayTextAtPositionAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::Point>>(&self, text: Param0, displayattribute: LineDisplayTextAttribute, startposition: Param2) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), text.into_param().abi(), displayattribute, startposition.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn TryDisplayTextNormalAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, text: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn TryScrollTextAsync(&self, direction: LineDisplayScrollDirection, numberofcolumnsorrows: u32) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), direction, numberofcolumnsorrows, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn TryClearTextAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Cursor(&self) -> ::windows::runtime::Result<LineDisplayCursor> {
        let this = &::windows::runtime::Interface::cast::<ILineDisplayWindow2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LineDisplayCursor>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Marquee(&self) -> ::windows::runtime::Result<LineDisplayMarquee> {
        let this = &::windows::runtime::Interface::cast::<ILineDisplayWindow2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LineDisplayMarquee>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn ReadCharacterAtCursorAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<u32>> {
        let this = &::windows::runtime::Interface::cast::<ILineDisplayWindow2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn TryDisplayStoredBitmapAtCursorAsync<'a, Param0: ::windows::runtime::IntoParam<'a, LineDisplayStoredBitmap>>(&self, bitmap: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::runtime::Interface::cast::<ILineDisplayWindow2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), bitmap.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`, `Storage`*"]
    pub fn TryDisplayStorageFileBitmapAtCursorAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::StorageFile>>(&self, bitmap: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::runtime::Interface::cast::<ILineDisplayWindow2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), bitmap.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`, `Storage`*"]
    pub fn TryDisplayStorageFileBitmapAtCursorWithAlignmentAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::StorageFile>>(&self, bitmap: Param0, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::runtime::Interface::cast::<ILineDisplayWindow2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), bitmap.into_param().abi(), horizontalalignment, verticalalignment, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`, `Storage`*"]
    pub fn TryDisplayStorageFileBitmapAtCursorWithAlignmentAndWidthAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::StorageFile>>(&self, bitmap: Param0, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment, widthinpixels: i32) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::runtime::Interface::cast::<ILineDisplayWindow2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), bitmap.into_param().abi(), horizontalalignment, verticalalignment, widthinpixels, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`, `Storage`*"]
    pub fn TryDisplayStorageFileBitmapAtPointAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::StorageFile>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Point>>(&self, bitmap: Param0, offsetinpixels: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::runtime::Interface::cast::<ILineDisplayWindow2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), bitmap.into_param().abi(), offsetinpixels.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`, `Storage`*"]
    pub fn TryDisplayStorageFileBitmapAtPointWithWidthAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::StorageFile>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::Point>>(&self, bitmap: Param0, offsetinpixels: Param1, widthinpixels: i32) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::runtime::Interface::cast::<ILineDisplayWindow2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), bitmap.into_param().abi(), offsetinpixels.into_param().abi(), widthinpixels, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LineDisplayWindow {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplayWindow;{d21feef4-2364-4be5-bee1-851680af4964})");
}
unsafe impl ::windows::runtime::Interface for LineDisplayWindow {
    type Vtable = ILineDisplayWindow_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3525308148, 9060, 19429, [190, 225, 133, 22, 128, 175, 73, 100]);
}
impl ::windows::runtime::RuntimeName for LineDisplayWindow {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayWindow";
}
impl ::std::convert::From<LineDisplayWindow> for ::windows::runtime::IUnknown {
    fn from(value: LineDisplayWindow) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&LineDisplayWindow> for ::windows::runtime::IUnknown {
    fn from(value: &LineDisplayWindow) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LineDisplayWindow {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &LineDisplayWindow {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<LineDisplayWindow> for ::windows::runtime::IInspectable {
    fn from(value: LineDisplayWindow) -> Self {
        value.0
    }
}
impl ::std::convert::From<&LineDisplayWindow> for ::windows::runtime::IInspectable {
    fn from(value: &LineDisplayWindow) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LineDisplayWindow {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LineDisplayWindow {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<LineDisplayWindow> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: LineDisplayWindow) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&LineDisplayWindow> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &LineDisplayWindow) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for LineDisplayWindow {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &LineDisplayWindow {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for LineDisplayWindow {}
unsafe impl ::std::marker::Sync for LineDisplayWindow {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MagneticStripeReader(::windows::runtime::IInspectable);
impl MagneticStripeReader {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Capabilities(&self) -> ::windows::runtime::Result<MagneticStripeReaderCapabilities> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MagneticStripeReaderCapabilities>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SupportedCardTypes(&self) -> ::windows::runtime::Result<::windows::runtime::Array<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::Array<u32> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), ::windows::runtime::Array::<u32>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DeviceAuthenticationProtocol(&self) -> ::windows::runtime::Result<MagneticStripeReaderAuthenticationProtocol> {
        let this = self;
        unsafe {
            let mut result__: MagneticStripeReaderAuthenticationProtocol = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MagneticStripeReaderAuthenticationProtocol>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn CheckHealthAsync(&self, level: UnifiedPosHealthCheckLevel) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), level, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn ClaimReaderAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ClaimedMagneticStripeReader>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ClaimedMagneticStripeReader>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`, `Foundation_Collections`, `Storage_Streams`*"]
    pub fn RetrieveStatisticsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(&self, statisticscategories: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), statisticscategories.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn GetErrorReportingType(&self) -> ::windows::runtime::Result<MagneticStripeReaderErrorReportingType> {
        let this = self;
        unsafe {
            let mut result__: MagneticStripeReaderErrorReportingType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MagneticStripeReaderErrorReportingType>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn StatusUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<MagneticStripeReader, MagneticStripeReaderStatusUpdatedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveStatusUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn GetDefaultAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<MagneticStripeReader>> {
        Self::IMagneticStripeReaderStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MagneticStripeReader>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn FromIdAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(deviceid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<MagneticStripeReader>> {
        Self::IMagneticStripeReaderStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MagneticStripeReader>>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn GetDeviceSelector() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMagneticStripeReaderStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn GetDeviceSelectorWithConnectionTypes(connectiontypes: PosConnectionTypes) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IMagneticStripeReaderStatics2(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), connectiontypes, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn IMagneticStripeReaderStatics<R, F: FnOnce(&IMagneticStripeReaderStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MagneticStripeReader, IMagneticStripeReaderStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IMagneticStripeReaderStatics2<R, F: FnOnce(&IMagneticStripeReaderStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MagneticStripeReader, IMagneticStripeReaderStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MagneticStripeReader {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.MagneticStripeReader;{1a92b015-47c3-468a-9333-0c6517574883})");
}
unsafe impl ::windows::runtime::Interface for MagneticStripeReader {
    type Vtable = IMagneticStripeReader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(445820949, 18371, 18058, [147, 51, 12, 101, 23, 87, 72, 131]);
}
impl ::windows::runtime::RuntimeName for MagneticStripeReader {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReader";
}
impl ::std::convert::From<MagneticStripeReader> for ::windows::runtime::IUnknown {
    fn from(value: MagneticStripeReader) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MagneticStripeReader> for ::windows::runtime::IUnknown {
    fn from(value: &MagneticStripeReader) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MagneticStripeReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MagneticStripeReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MagneticStripeReader> for ::windows::runtime::IInspectable {
    fn from(value: MagneticStripeReader) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MagneticStripeReader> for ::windows::runtime::IInspectable {
    fn from(value: &MagneticStripeReader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MagneticStripeReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MagneticStripeReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<MagneticStripeReader> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: MagneticStripeReader) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&MagneticStripeReader> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &MagneticStripeReader) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for MagneticStripeReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &MagneticStripeReader {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for MagneticStripeReader {}
unsafe impl ::std::marker::Sync for MagneticStripeReader {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MagneticStripeReaderAamvaCardDataReceivedEventArgs(::windows::runtime::IInspectable);
impl MagneticStripeReaderAamvaCardDataReceivedEventArgs {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Report(&self) -> ::windows::runtime::Result<MagneticStripeReaderReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MagneticStripeReaderReport>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn LicenseNumber(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ExpirationDate(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Restrictions(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Class(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Endorsements(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn BirthDate(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn FirstName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Surname(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Suffix(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Gender(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn HairColor(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn EyeColor(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Height(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Weight(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Address(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn City(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn State(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PostalCode(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MagneticStripeReaderAamvaCardDataReceivedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.MagneticStripeReaderAamvaCardDataReceivedEventArgs;{0a4bbd51-c316-4910-87f3-7a62ba862d31})");
}
unsafe impl ::windows::runtime::Interface for MagneticStripeReaderAamvaCardDataReceivedEventArgs {
    type Vtable = IMagneticStripeReaderAamvaCardDataReceivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(172735825, 49942, 18704, [135, 243, 122, 98, 186, 134, 45, 49]);
}
impl ::windows::runtime::RuntimeName for MagneticStripeReaderAamvaCardDataReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderAamvaCardDataReceivedEventArgs";
}
impl ::std::convert::From<MagneticStripeReaderAamvaCardDataReceivedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: MagneticStripeReaderAamvaCardDataReceivedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MagneticStripeReaderAamvaCardDataReceivedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &MagneticStripeReaderAamvaCardDataReceivedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MagneticStripeReaderAamvaCardDataReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MagneticStripeReaderAamvaCardDataReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MagneticStripeReaderAamvaCardDataReceivedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: MagneticStripeReaderAamvaCardDataReceivedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MagneticStripeReaderAamvaCardDataReceivedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &MagneticStripeReaderAamvaCardDataReceivedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MagneticStripeReaderAamvaCardDataReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MagneticStripeReaderAamvaCardDataReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MagneticStripeReaderAamvaCardDataReceivedEventArgs {}
unsafe impl ::std::marker::Sync for MagneticStripeReaderAamvaCardDataReceivedEventArgs {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MagneticStripeReaderAuthenticationLevel(pub i32);
impl MagneticStripeReaderAuthenticationLevel {
    pub const NotSupported: MagneticStripeReaderAuthenticationLevel = MagneticStripeReaderAuthenticationLevel(0i32);
    pub const Optional: MagneticStripeReaderAuthenticationLevel = MagneticStripeReaderAuthenticationLevel(1i32);
    pub const Required: MagneticStripeReaderAuthenticationLevel = MagneticStripeReaderAuthenticationLevel(2i32);
}
impl ::std::convert::From<i32> for MagneticStripeReaderAuthenticationLevel {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MagneticStripeReaderAuthenticationLevel {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MagneticStripeReaderAuthenticationLevel {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.MagneticStripeReaderAuthenticationLevel;i4)");
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MagneticStripeReaderAuthenticationProtocol(pub i32);
impl MagneticStripeReaderAuthenticationProtocol {
    pub const None: MagneticStripeReaderAuthenticationProtocol = MagneticStripeReaderAuthenticationProtocol(0i32);
    pub const ChallengeResponse: MagneticStripeReaderAuthenticationProtocol = MagneticStripeReaderAuthenticationProtocol(1i32);
}
impl ::std::convert::From<i32> for MagneticStripeReaderAuthenticationProtocol {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MagneticStripeReaderAuthenticationProtocol {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MagneticStripeReaderAuthenticationProtocol {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.MagneticStripeReaderAuthenticationProtocol;i4)");
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MagneticStripeReaderBankCardDataReceivedEventArgs(::windows::runtime::IInspectable);
impl MagneticStripeReaderBankCardDataReceivedEventArgs {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Report(&self) -> ::windows::runtime::Result<MagneticStripeReaderReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MagneticStripeReaderReport>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn AccountNumber(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ExpirationDate(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ServiceCode(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Title(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn FirstName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn MiddleInitial(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Surname(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Suffix(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MagneticStripeReaderBankCardDataReceivedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.MagneticStripeReaderBankCardDataReceivedEventArgs;{2e958823-a31a-4763-882c-23725e39b08e})");
}
unsafe impl ::windows::runtime::Interface for MagneticStripeReaderBankCardDataReceivedEventArgs {
    type Vtable = IMagneticStripeReaderBankCardDataReceivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(781551651, 41754, 18275, [136, 44, 35, 114, 94, 57, 176, 142]);
}
impl ::windows::runtime::RuntimeName for MagneticStripeReaderBankCardDataReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderBankCardDataReceivedEventArgs";
}
impl ::std::convert::From<MagneticStripeReaderBankCardDataReceivedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: MagneticStripeReaderBankCardDataReceivedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MagneticStripeReaderBankCardDataReceivedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &MagneticStripeReaderBankCardDataReceivedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MagneticStripeReaderBankCardDataReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MagneticStripeReaderBankCardDataReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MagneticStripeReaderBankCardDataReceivedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: MagneticStripeReaderBankCardDataReceivedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MagneticStripeReaderBankCardDataReceivedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &MagneticStripeReaderBankCardDataReceivedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MagneticStripeReaderBankCardDataReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MagneticStripeReaderBankCardDataReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MagneticStripeReaderBankCardDataReceivedEventArgs {}
unsafe impl ::std::marker::Sync for MagneticStripeReaderBankCardDataReceivedEventArgs {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MagneticStripeReaderCapabilities(::windows::runtime::IInspectable);
impl MagneticStripeReaderCapabilities {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CardAuthentication(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SupportedEncryptionAlgorithms(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn AuthenticationLevel(&self) -> ::windows::runtime::Result<MagneticStripeReaderAuthenticationLevel> {
        let this = self;
        unsafe {
            let mut result__: MagneticStripeReaderAuthenticationLevel = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MagneticStripeReaderAuthenticationLevel>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsIsoSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsJisOneSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsJisTwoSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PowerReportingType(&self) -> ::windows::runtime::Result<UnifiedPosPowerReportingType> {
        let this = self;
        unsafe {
            let mut result__: UnifiedPosPowerReportingType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UnifiedPosPowerReportingType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsStatisticsReportingSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsStatisticsUpdatingSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsTrackDataMaskingSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsTransmitSentinelsSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MagneticStripeReaderCapabilities {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.MagneticStripeReaderCapabilities;{7128809c-c440-44a2-a467-469175d02896})");
}
unsafe impl ::windows::runtime::Interface for MagneticStripeReaderCapabilities {
    type Vtable = IMagneticStripeReaderCapabilities_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1898479772, 50240, 17570, [164, 103, 70, 145, 117, 208, 40, 150]);
}
impl ::windows::runtime::RuntimeName for MagneticStripeReaderCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderCapabilities";
}
impl ::std::convert::From<MagneticStripeReaderCapabilities> for ::windows::runtime::IUnknown {
    fn from(value: MagneticStripeReaderCapabilities) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MagneticStripeReaderCapabilities> for ::windows::runtime::IUnknown {
    fn from(value: &MagneticStripeReaderCapabilities) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MagneticStripeReaderCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MagneticStripeReaderCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MagneticStripeReaderCapabilities> for ::windows::runtime::IInspectable {
    fn from(value: MagneticStripeReaderCapabilities) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MagneticStripeReaderCapabilities> for ::windows::runtime::IInspectable {
    fn from(value: &MagneticStripeReaderCapabilities) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MagneticStripeReaderCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MagneticStripeReaderCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MagneticStripeReaderCapabilities {}
unsafe impl ::std::marker::Sync for MagneticStripeReaderCapabilities {}
#[doc = "*Required features: `Devices_PointOfService`*"]
pub struct MagneticStripeReaderCardTypes {}
impl MagneticStripeReaderCardTypes {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Unknown() -> ::windows::runtime::Result<u32> {
        Self::IMagneticStripeReaderCardTypesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Bank() -> ::windows::runtime::Result<u32> {
        Self::IMagneticStripeReaderCardTypesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Aamva() -> ::windows::runtime::Result<u32> {
        Self::IMagneticStripeReaderCardTypesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ExtendedBase() -> ::windows::runtime::Result<u32> {
        Self::IMagneticStripeReaderCardTypesStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    pub fn IMagneticStripeReaderCardTypesStatics<R, F: FnOnce(&IMagneticStripeReaderCardTypesStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MagneticStripeReaderCardTypes, IMagneticStripeReaderCardTypesStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for MagneticStripeReaderCardTypes {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderCardTypes";
}
#[doc = "*Required features: `Devices_PointOfService`*"]
pub struct MagneticStripeReaderEncryptionAlgorithms {}
impl MagneticStripeReaderEncryptionAlgorithms {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn None() -> ::windows::runtime::Result<u32> {
        Self::IMagneticStripeReaderEncryptionAlgorithmsStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn TripleDesDukpt() -> ::windows::runtime::Result<u32> {
        Self::IMagneticStripeReaderEncryptionAlgorithmsStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ExtendedBase() -> ::windows::runtime::Result<u32> {
        Self::IMagneticStripeReaderEncryptionAlgorithmsStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    pub fn IMagneticStripeReaderEncryptionAlgorithmsStatics<R, F: FnOnce(&IMagneticStripeReaderEncryptionAlgorithmsStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MagneticStripeReaderEncryptionAlgorithms, IMagneticStripeReaderEncryptionAlgorithmsStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for MagneticStripeReaderEncryptionAlgorithms {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderEncryptionAlgorithms";
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MagneticStripeReaderErrorOccurredEventArgs(::windows::runtime::IInspectable);
impl MagneticStripeReaderErrorOccurredEventArgs {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Track1Status(&self) -> ::windows::runtime::Result<MagneticStripeReaderTrackErrorType> {
        let this = self;
        unsafe {
            let mut result__: MagneticStripeReaderTrackErrorType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MagneticStripeReaderTrackErrorType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Track2Status(&self) -> ::windows::runtime::Result<MagneticStripeReaderTrackErrorType> {
        let this = self;
        unsafe {
            let mut result__: MagneticStripeReaderTrackErrorType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MagneticStripeReaderTrackErrorType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Track3Status(&self) -> ::windows::runtime::Result<MagneticStripeReaderTrackErrorType> {
        let this = self;
        unsafe {
            let mut result__: MagneticStripeReaderTrackErrorType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MagneticStripeReaderTrackErrorType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Track4Status(&self) -> ::windows::runtime::Result<MagneticStripeReaderTrackErrorType> {
        let this = self;
        unsafe {
            let mut result__: MagneticStripeReaderTrackErrorType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MagneticStripeReaderTrackErrorType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ErrorData(&self) -> ::windows::runtime::Result<UnifiedPosErrorData> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UnifiedPosErrorData>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PartialInputData(&self) -> ::windows::runtime::Result<MagneticStripeReaderReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MagneticStripeReaderReport>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MagneticStripeReaderErrorOccurredEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.MagneticStripeReaderErrorOccurredEventArgs;{1fedf95d-2c84-41ad-b778-f2356a789ab1})");
}
unsafe impl ::windows::runtime::Interface for MagneticStripeReaderErrorOccurredEventArgs {
    type Vtable = IMagneticStripeReaderErrorOccurredEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(535689565, 11396, 16813, [183, 120, 242, 53, 106, 120, 154, 177]);
}
impl ::windows::runtime::RuntimeName for MagneticStripeReaderErrorOccurredEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderErrorOccurredEventArgs";
}
impl ::std::convert::From<MagneticStripeReaderErrorOccurredEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: MagneticStripeReaderErrorOccurredEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MagneticStripeReaderErrorOccurredEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &MagneticStripeReaderErrorOccurredEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MagneticStripeReaderErrorOccurredEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MagneticStripeReaderErrorOccurredEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MagneticStripeReaderErrorOccurredEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: MagneticStripeReaderErrorOccurredEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MagneticStripeReaderErrorOccurredEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &MagneticStripeReaderErrorOccurredEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MagneticStripeReaderErrorOccurredEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MagneticStripeReaderErrorOccurredEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MagneticStripeReaderErrorOccurredEventArgs {}
unsafe impl ::std::marker::Sync for MagneticStripeReaderErrorOccurredEventArgs {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MagneticStripeReaderErrorReportingType(pub i32);
impl MagneticStripeReaderErrorReportingType {
    pub const CardLevel: MagneticStripeReaderErrorReportingType = MagneticStripeReaderErrorReportingType(0i32);
    pub const TrackLevel: MagneticStripeReaderErrorReportingType = MagneticStripeReaderErrorReportingType(1i32);
}
impl ::std::convert::From<i32> for MagneticStripeReaderErrorReportingType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MagneticStripeReaderErrorReportingType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MagneticStripeReaderErrorReportingType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.MagneticStripeReaderErrorReportingType;i4)");
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MagneticStripeReaderReport(::windows::runtime::IInspectable);
impl MagneticStripeReaderReport {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CardType(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Track1(&self) -> ::windows::runtime::Result<MagneticStripeReaderTrackData> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MagneticStripeReaderTrackData>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Track2(&self) -> ::windows::runtime::Result<MagneticStripeReaderTrackData> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MagneticStripeReaderTrackData>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Track3(&self) -> ::windows::runtime::Result<MagneticStripeReaderTrackData> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MagneticStripeReaderTrackData>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Track4(&self) -> ::windows::runtime::Result<MagneticStripeReaderTrackData> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MagneticStripeReaderTrackData>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation_Collections`*"]
    pub fn Properties(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::runtime::HSTRING, ::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_PointOfService`, `Storage_Streams`*"]
    pub fn CardAuthenticationData(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CardAuthenticationDataLength(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_PointOfService`, `Storage_Streams`*"]
    pub fn AdditionalSecurityInformation(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MagneticStripeReaderReport {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.MagneticStripeReaderReport;{6a5b6047-99b0-4188-bef1-eddf79f78fe6})");
}
unsafe impl ::windows::runtime::Interface for MagneticStripeReaderReport {
    type Vtable = IMagneticStripeReaderReport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1784373319, 39344, 16776, [190, 241, 237, 223, 121, 247, 143, 230]);
}
impl ::windows::runtime::RuntimeName for MagneticStripeReaderReport {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderReport";
}
impl ::std::convert::From<MagneticStripeReaderReport> for ::windows::runtime::IUnknown {
    fn from(value: MagneticStripeReaderReport) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MagneticStripeReaderReport> for ::windows::runtime::IUnknown {
    fn from(value: &MagneticStripeReaderReport) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MagneticStripeReaderReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MagneticStripeReaderReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MagneticStripeReaderReport> for ::windows::runtime::IInspectable {
    fn from(value: MagneticStripeReaderReport) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MagneticStripeReaderReport> for ::windows::runtime::IInspectable {
    fn from(value: &MagneticStripeReaderReport) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MagneticStripeReaderReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MagneticStripeReaderReport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MagneticStripeReaderReport {}
unsafe impl ::std::marker::Sync for MagneticStripeReaderReport {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MagneticStripeReaderStatus(pub i32);
impl MagneticStripeReaderStatus {
    pub const Unauthenticated: MagneticStripeReaderStatus = MagneticStripeReaderStatus(0i32);
    pub const Authenticated: MagneticStripeReaderStatus = MagneticStripeReaderStatus(1i32);
    pub const Extended: MagneticStripeReaderStatus = MagneticStripeReaderStatus(2i32);
}
impl ::std::convert::From<i32> for MagneticStripeReaderStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MagneticStripeReaderStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MagneticStripeReaderStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.MagneticStripeReaderStatus;i4)");
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MagneticStripeReaderStatusUpdatedEventArgs(::windows::runtime::IInspectable);
impl MagneticStripeReaderStatusUpdatedEventArgs {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<MagneticStripeReaderStatus> {
        let this = self;
        unsafe {
            let mut result__: MagneticStripeReaderStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MagneticStripeReaderStatus>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ExtendedStatus(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MagneticStripeReaderStatusUpdatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.MagneticStripeReaderStatusUpdatedEventArgs;{09cc6bb0-3262-401d-9e8a-e80d6358906b})");
}
unsafe impl ::windows::runtime::Interface for MagneticStripeReaderStatusUpdatedEventArgs {
    type Vtable = IMagneticStripeReaderStatusUpdatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(164391856, 12898, 16413, [158, 138, 232, 13, 99, 88, 144, 107]);
}
impl ::windows::runtime::RuntimeName for MagneticStripeReaderStatusUpdatedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderStatusUpdatedEventArgs";
}
impl ::std::convert::From<MagneticStripeReaderStatusUpdatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: MagneticStripeReaderStatusUpdatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MagneticStripeReaderStatusUpdatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &MagneticStripeReaderStatusUpdatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MagneticStripeReaderStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MagneticStripeReaderStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MagneticStripeReaderStatusUpdatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: MagneticStripeReaderStatusUpdatedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MagneticStripeReaderStatusUpdatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &MagneticStripeReaderStatusUpdatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MagneticStripeReaderStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MagneticStripeReaderStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MagneticStripeReaderStatusUpdatedEventArgs {}
unsafe impl ::std::marker::Sync for MagneticStripeReaderStatusUpdatedEventArgs {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MagneticStripeReaderTrackData(::windows::runtime::IInspectable);
impl MagneticStripeReaderTrackData {
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_PointOfService`, `Storage_Streams`*"]
    pub fn Data(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_PointOfService`, `Storage_Streams`*"]
    pub fn DiscretionaryData(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    #[doc = "*Required features: `Devices_PointOfService`, `Storage_Streams`*"]
    pub fn EncryptedData(&self) -> ::windows::runtime::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MagneticStripeReaderTrackData {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.MagneticStripeReaderTrackData;{104cf671-4a9d-446e-abc5-20402307ba36})");
}
unsafe impl ::windows::runtime::Interface for MagneticStripeReaderTrackData {
    type Vtable = IMagneticStripeReaderTrackData_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(273479281, 19101, 17518, [171, 197, 32, 64, 35, 7, 186, 54]);
}
impl ::windows::runtime::RuntimeName for MagneticStripeReaderTrackData {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderTrackData";
}
impl ::std::convert::From<MagneticStripeReaderTrackData> for ::windows::runtime::IUnknown {
    fn from(value: MagneticStripeReaderTrackData) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MagneticStripeReaderTrackData> for ::windows::runtime::IUnknown {
    fn from(value: &MagneticStripeReaderTrackData) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MagneticStripeReaderTrackData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MagneticStripeReaderTrackData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MagneticStripeReaderTrackData> for ::windows::runtime::IInspectable {
    fn from(value: MagneticStripeReaderTrackData) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MagneticStripeReaderTrackData> for ::windows::runtime::IInspectable {
    fn from(value: &MagneticStripeReaderTrackData) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MagneticStripeReaderTrackData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MagneticStripeReaderTrackData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MagneticStripeReaderTrackData {}
unsafe impl ::std::marker::Sync for MagneticStripeReaderTrackData {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for MagneticStripeReaderTrackErrorType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MagneticStripeReaderTrackErrorType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MagneticStripeReaderTrackErrorType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.MagneticStripeReaderTrackErrorType;i4)");
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MagneticStripeReaderTrackIds(pub i32);
impl MagneticStripeReaderTrackIds {
    pub const None: MagneticStripeReaderTrackIds = MagneticStripeReaderTrackIds(0i32);
    pub const Track1: MagneticStripeReaderTrackIds = MagneticStripeReaderTrackIds(1i32);
    pub const Track2: MagneticStripeReaderTrackIds = MagneticStripeReaderTrackIds(2i32);
    pub const Track3: MagneticStripeReaderTrackIds = MagneticStripeReaderTrackIds(4i32);
    pub const Track4: MagneticStripeReaderTrackIds = MagneticStripeReaderTrackIds(8i32);
}
impl ::std::convert::From<i32> for MagneticStripeReaderTrackIds {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MagneticStripeReaderTrackIds {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MagneticStripeReaderTrackIds {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.MagneticStripeReaderTrackIds;i4)");
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs(::windows::runtime::IInspectable);
impl MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Report(&self) -> ::windows::runtime::Result<MagneticStripeReaderReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MagneticStripeReaderReport>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs;{af0a5514-59cc-4a60-99e8-99a53dace5aa})");
}
unsafe impl ::windows::runtime::Interface for MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    type Vtable = IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2936689940, 22988, 19040, [153, 232, 153, 165, 61, 172, 229, 170]);
}
impl ::windows::runtime::RuntimeName for MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs";
}
impl ::std::convert::From<MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {}
unsafe impl ::std::marker::Sync for MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PosConnectionTypes(pub u32);
impl PosConnectionTypes {
    pub const Local: PosConnectionTypes = PosConnectionTypes(1u32);
    pub const IP: PosConnectionTypes = PosConnectionTypes(2u32);
    pub const Bluetooth: PosConnectionTypes = PosConnectionTypes(4u32);
    pub const All: PosConnectionTypes = PosConnectionTypes(4294967295u32);
}
impl ::std::convert::From<u32> for PosConnectionTypes {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PosConnectionTypes {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PosConnectionTypes {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosConnectionTypes;u4)");
}
impl ::std::ops::BitOr for PosConnectionTypes {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for PosConnectionTypes {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for PosConnectionTypes {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for PosConnectionTypes {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for PosConnectionTypes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PosPrinter(::windows::runtime::IInspectable);
impl PosPrinter {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DeviceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Capabilities(&self) -> ::windows::runtime::Result<PosPrinterCapabilities> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterCapabilities>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation_Collections`*"]
    pub fn SupportedCharacterSets(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation_Collections`*"]
    pub fn SupportedTypeFaces(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<PosPrinterStatus> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn ClaimPrinterAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<ClaimedPosPrinter>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ClaimedPosPrinter>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn CheckHealthAsync(&self, level: UnifiedPosHealthCheckLevel) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), level, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`, `Foundation_Collections`*"]
    pub fn GetStatisticsAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>>>(&self, statisticscategories: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), statisticscategories.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn StatusUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<PosPrinter, PosPrinterStatusUpdatedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn RemoveStatusUpdated<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn GetDefaultAsync() -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<PosPrinter>> {
        Self::IPosPrinterStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PosPrinter>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn FromIdAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(deviceid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<PosPrinter>> {
        Self::IPosPrinterStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PosPrinter>>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn GetDeviceSelector() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IPosPrinterStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn GetDeviceSelectorWithConnectionTypes(connectiontypes: PosConnectionTypes) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IPosPrinterStatics2(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), connectiontypes, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation_Collections`*"]
    pub fn SupportedBarcodeSymbologies(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &::windows::runtime::Interface::cast::<IPosPrinter2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn GetFontProperty<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, typeface: Param0) -> ::windows::runtime::Result<PosPrinterFontProperty> {
        let this = &::windows::runtime::Interface::cast::<IPosPrinter2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), typeface.into_param().abi(), &mut result__).from_abi::<PosPrinterFontProperty>(result__)
        }
    }
    pub fn IPosPrinterStatics<R, F: FnOnce(&IPosPrinterStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PosPrinter, IPosPrinterStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IPosPrinterStatics2<R, F: FnOnce(&IPosPrinterStatics2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PosPrinter, IPosPrinterStatics2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PosPrinter {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.PosPrinter;{2a03c10e-9a19-4a01-994f-12dfad6adcbf})");
}
unsafe impl ::windows::runtime::Interface for PosPrinter {
    type Vtable = IPosPrinter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(704889102, 39449, 18945, [153, 79, 18, 223, 173, 106, 220, 191]);
}
impl ::windows::runtime::RuntimeName for PosPrinter {
    const NAME: &'static str = "Windows.Devices.PointOfService.PosPrinter";
}
impl ::std::convert::From<PosPrinter> for ::windows::runtime::IUnknown {
    fn from(value: PosPrinter) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PosPrinter> for ::windows::runtime::IUnknown {
    fn from(value: &PosPrinter) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PosPrinter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PosPrinter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<PosPrinter> for ::windows::runtime::IInspectable {
    fn from(value: PosPrinter) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PosPrinter> for ::windows::runtime::IInspectable {
    fn from(value: &PosPrinter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PosPrinter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PosPrinter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<PosPrinter> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: PosPrinter) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&PosPrinter> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &PosPrinter) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for PosPrinter {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable> for &PosPrinter {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for PosPrinter {}
unsafe impl ::std::marker::Sync for PosPrinter {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PosPrinterAlignment(pub i32);
impl PosPrinterAlignment {
    pub const Left: PosPrinterAlignment = PosPrinterAlignment(0i32);
    pub const Center: PosPrinterAlignment = PosPrinterAlignment(1i32);
    pub const Right: PosPrinterAlignment = PosPrinterAlignment(2i32);
}
impl ::std::convert::From<i32> for PosPrinterAlignment {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PosPrinterAlignment {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PosPrinterAlignment {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterAlignment;i4)");
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PosPrinterBarcodeTextPosition(pub i32);
impl PosPrinterBarcodeTextPosition {
    pub const None: PosPrinterBarcodeTextPosition = PosPrinterBarcodeTextPosition(0i32);
    pub const Above: PosPrinterBarcodeTextPosition = PosPrinterBarcodeTextPosition(1i32);
    pub const Below: PosPrinterBarcodeTextPosition = PosPrinterBarcodeTextPosition(2i32);
}
impl ::std::convert::From<i32> for PosPrinterBarcodeTextPosition {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PosPrinterBarcodeTextPosition {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PosPrinterBarcodeTextPosition {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterBarcodeTextPosition;i4)");
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PosPrinterCapabilities(::windows::runtime::IInspectable);
impl PosPrinterCapabilities {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PowerReportingType(&self) -> ::windows::runtime::Result<UnifiedPosPowerReportingType> {
        let this = self;
        unsafe {
            let mut result__: UnifiedPosPowerReportingType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UnifiedPosPowerReportingType>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsStatisticsReportingSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsStatisticsUpdatingSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DefaultCharacterSet(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn HasCoverSensor(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CanMapCharacterSet(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsTransactionSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Receipt(&self) -> ::windows::runtime::Result<ReceiptPrinterCapabilities> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ReceiptPrinterCapabilities>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Slip(&self) -> ::windows::runtime::Result<SlipPrinterCapabilities> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<SlipPrinterCapabilities>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Journal(&self) -> ::windows::runtime::Result<JournalPrinterCapabilities> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<JournalPrinterCapabilities>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PosPrinterCapabilities {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.PosPrinterCapabilities;{cde95721-4380-4985-adc5-39db30cd93bc})");
}
unsafe impl ::windows::runtime::Interface for PosPrinterCapabilities {
    type Vtable = IPosPrinterCapabilities_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3454621473, 17280, 18821, [173, 197, 57, 219, 48, 205, 147, 188]);
}
impl ::windows::runtime::RuntimeName for PosPrinterCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.PosPrinterCapabilities";
}
impl ::std::convert::From<PosPrinterCapabilities> for ::windows::runtime::IUnknown {
    fn from(value: PosPrinterCapabilities) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PosPrinterCapabilities> for ::windows::runtime::IUnknown {
    fn from(value: &PosPrinterCapabilities) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PosPrinterCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PosPrinterCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<PosPrinterCapabilities> for ::windows::runtime::IInspectable {
    fn from(value: PosPrinterCapabilities) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PosPrinterCapabilities> for ::windows::runtime::IInspectable {
    fn from(value: &PosPrinterCapabilities) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PosPrinterCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PosPrinterCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PosPrinterCapabilities {}
unsafe impl ::std::marker::Sync for PosPrinterCapabilities {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PosPrinterCartridgeSensors(pub u32);
impl PosPrinterCartridgeSensors {
    pub const None: PosPrinterCartridgeSensors = PosPrinterCartridgeSensors(0u32);
    pub const Removed: PosPrinterCartridgeSensors = PosPrinterCartridgeSensors(1u32);
    pub const Empty: PosPrinterCartridgeSensors = PosPrinterCartridgeSensors(2u32);
    pub const HeadCleaning: PosPrinterCartridgeSensors = PosPrinterCartridgeSensors(4u32);
    pub const NearEnd: PosPrinterCartridgeSensors = PosPrinterCartridgeSensors(8u32);
}
impl ::std::convert::From<u32> for PosPrinterCartridgeSensors {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PosPrinterCartridgeSensors {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PosPrinterCartridgeSensors {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterCartridgeSensors;u4)");
}
impl ::std::ops::BitOr for PosPrinterCartridgeSensors {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for PosPrinterCartridgeSensors {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for PosPrinterCartridgeSensors {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for PosPrinterCartridgeSensors {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for PosPrinterCartridgeSensors {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Devices_PointOfService`*"]
pub struct PosPrinterCharacterSetIds {}
impl PosPrinterCharacterSetIds {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Utf16LE() -> ::windows::runtime::Result<u32> {
        Self::IPosPrinterCharacterSetIdsStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Ascii() -> ::windows::runtime::Result<u32> {
        Self::IPosPrinterCharacterSetIdsStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Ansi() -> ::windows::runtime::Result<u32> {
        Self::IPosPrinterCharacterSetIdsStatics(|this| unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    pub fn IPosPrinterCharacterSetIdsStatics<R, F: FnOnce(&IPosPrinterCharacterSetIdsStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PosPrinterCharacterSetIds, IPosPrinterCharacterSetIdsStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for PosPrinterCharacterSetIds {
    const NAME: &'static str = "Windows.Devices.PointOfService.PosPrinterCharacterSetIds";
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<u32> for PosPrinterColorCapabilities {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PosPrinterColorCapabilities {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PosPrinterColorCapabilities {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterColorCapabilities;u4)");
}
impl ::std::ops::BitOr for PosPrinterColorCapabilities {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for PosPrinterColorCapabilities {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for PosPrinterColorCapabilities {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for PosPrinterColorCapabilities {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for PosPrinterColorCapabilities {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for PosPrinterColorCartridge {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PosPrinterColorCartridge {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PosPrinterColorCartridge {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterColorCartridge;i4)");
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PosPrinterFontProperty(::windows::runtime::IInspectable);
impl PosPrinterFontProperty {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn TypeFace(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsScalableToAnySize(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation_Collections`*"]
    pub fn CharacterSizes(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<SizeUInt32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<SizeUInt32>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PosPrinterFontProperty {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.PosPrinterFontProperty;{a7f4e93a-f8ac-5f04-84d2-29b16d8a633c})");
}
unsafe impl ::windows::runtime::Interface for PosPrinterFontProperty {
    type Vtable = IPosPrinterFontProperty_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2817845562, 63660, 24324, [132, 210, 41, 177, 109, 138, 99, 60]);
}
impl ::windows::runtime::RuntimeName for PosPrinterFontProperty {
    const NAME: &'static str = "Windows.Devices.PointOfService.PosPrinterFontProperty";
}
impl ::std::convert::From<PosPrinterFontProperty> for ::windows::runtime::IUnknown {
    fn from(value: PosPrinterFontProperty) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PosPrinterFontProperty> for ::windows::runtime::IUnknown {
    fn from(value: &PosPrinterFontProperty) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PosPrinterFontProperty {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PosPrinterFontProperty {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<PosPrinterFontProperty> for ::windows::runtime::IInspectable {
    fn from(value: PosPrinterFontProperty) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PosPrinterFontProperty> for ::windows::runtime::IInspectable {
    fn from(value: &PosPrinterFontProperty) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PosPrinterFontProperty {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PosPrinterFontProperty {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PosPrinterFontProperty {}
unsafe impl ::std::marker::Sync for PosPrinterFontProperty {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PosPrinterLineDirection(pub i32);
impl PosPrinterLineDirection {
    pub const Horizontal: PosPrinterLineDirection = PosPrinterLineDirection(0i32);
    pub const Vertical: PosPrinterLineDirection = PosPrinterLineDirection(1i32);
}
impl ::std::convert::From<i32> for PosPrinterLineDirection {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PosPrinterLineDirection {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PosPrinterLineDirection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterLineDirection;i4)");
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PosPrinterLineStyle(pub i32);
impl PosPrinterLineStyle {
    pub const SingleSolid: PosPrinterLineStyle = PosPrinterLineStyle(0i32);
    pub const DoubleSolid: PosPrinterLineStyle = PosPrinterLineStyle(1i32);
    pub const Broken: PosPrinterLineStyle = PosPrinterLineStyle(2i32);
    pub const Chain: PosPrinterLineStyle = PosPrinterLineStyle(3i32);
}
impl ::std::convert::From<i32> for PosPrinterLineStyle {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PosPrinterLineStyle {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PosPrinterLineStyle {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterLineStyle;i4)");
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PosPrinterMapMode(pub i32);
impl PosPrinterMapMode {
    pub const Dots: PosPrinterMapMode = PosPrinterMapMode(0i32);
    pub const Twips: PosPrinterMapMode = PosPrinterMapMode(1i32);
    pub const English: PosPrinterMapMode = PosPrinterMapMode(2i32);
    pub const Metric: PosPrinterMapMode = PosPrinterMapMode(3i32);
}
impl ::std::convert::From<i32> for PosPrinterMapMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PosPrinterMapMode {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PosPrinterMapMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterMapMode;i4)");
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PosPrinterMarkFeedCapabilities(pub u32);
impl PosPrinterMarkFeedCapabilities {
    pub const None: PosPrinterMarkFeedCapabilities = PosPrinterMarkFeedCapabilities(0u32);
    pub const ToTakeUp: PosPrinterMarkFeedCapabilities = PosPrinterMarkFeedCapabilities(1u32);
    pub const ToCutter: PosPrinterMarkFeedCapabilities = PosPrinterMarkFeedCapabilities(2u32);
    pub const ToCurrentTopOfForm: PosPrinterMarkFeedCapabilities = PosPrinterMarkFeedCapabilities(4u32);
    pub const ToNextTopOfForm: PosPrinterMarkFeedCapabilities = PosPrinterMarkFeedCapabilities(8u32);
}
impl ::std::convert::From<u32> for PosPrinterMarkFeedCapabilities {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PosPrinterMarkFeedCapabilities {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PosPrinterMarkFeedCapabilities {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterMarkFeedCapabilities;u4)");
}
impl ::std::ops::BitOr for PosPrinterMarkFeedCapabilities {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for PosPrinterMarkFeedCapabilities {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for PosPrinterMarkFeedCapabilities {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for PosPrinterMarkFeedCapabilities {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for PosPrinterMarkFeedCapabilities {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PosPrinterMarkFeedKind(pub i32);
impl PosPrinterMarkFeedKind {
    pub const ToTakeUp: PosPrinterMarkFeedKind = PosPrinterMarkFeedKind(0i32);
    pub const ToCutter: PosPrinterMarkFeedKind = PosPrinterMarkFeedKind(1i32);
    pub const ToCurrentTopOfForm: PosPrinterMarkFeedKind = PosPrinterMarkFeedKind(2i32);
    pub const ToNextTopOfForm: PosPrinterMarkFeedKind = PosPrinterMarkFeedKind(3i32);
}
impl ::std::convert::From<i32> for PosPrinterMarkFeedKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PosPrinterMarkFeedKind {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PosPrinterMarkFeedKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterMarkFeedKind;i4)");
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PosPrinterPrintOptions(::windows::runtime::IInspectable);
impl PosPrinterPrintOptions {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PosPrinterPrintOptions, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn TypeFace(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetTypeFace<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CharacterHeight(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetCharacterHeight(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Bold(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetBold(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Italic(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetItalic(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Underline(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetUnderline(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ReverseVideo(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetReverseVideo(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Strikethrough(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetStrikethrough(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Superscript(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetSuperscript(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Subscript(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetSubscript(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DoubleWide(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetDoubleWide(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DoubleHigh(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetDoubleHigh(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Alignment(&self) -> ::windows::runtime::Result<PosPrinterAlignment> {
        let this = self;
        unsafe {
            let mut result__: PosPrinterAlignment = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterAlignment>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetAlignment(&self, value: PosPrinterAlignment) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CharacterSet(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetCharacterSet(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).31)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PosPrinterPrintOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.PosPrinterPrintOptions;{0a2e16fd-1d02-5a58-9d59-bfcde76fde86})");
}
unsafe impl ::windows::runtime::Interface for PosPrinterPrintOptions {
    type Vtable = IPosPrinterPrintOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(170792701, 7426, 23128, [157, 89, 191, 205, 231, 111, 222, 134]);
}
impl ::windows::runtime::RuntimeName for PosPrinterPrintOptions {
    const NAME: &'static str = "Windows.Devices.PointOfService.PosPrinterPrintOptions";
}
impl ::std::convert::From<PosPrinterPrintOptions> for ::windows::runtime::IUnknown {
    fn from(value: PosPrinterPrintOptions) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PosPrinterPrintOptions> for ::windows::runtime::IUnknown {
    fn from(value: &PosPrinterPrintOptions) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PosPrinterPrintOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PosPrinterPrintOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<PosPrinterPrintOptions> for ::windows::runtime::IInspectable {
    fn from(value: PosPrinterPrintOptions) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PosPrinterPrintOptions> for ::windows::runtime::IInspectable {
    fn from(value: &PosPrinterPrintOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PosPrinterPrintOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PosPrinterPrintOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PosPrinterPrintOptions {}
unsafe impl ::std::marker::Sync for PosPrinterPrintOptions {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PosPrinterPrintSide(pub i32);
impl PosPrinterPrintSide {
    pub const Unknown: PosPrinterPrintSide = PosPrinterPrintSide(0i32);
    pub const Side1: PosPrinterPrintSide = PosPrinterPrintSide(1i32);
    pub const Side2: PosPrinterPrintSide = PosPrinterPrintSide(2i32);
}
impl ::std::convert::From<i32> for PosPrinterPrintSide {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PosPrinterPrintSide {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PosPrinterPrintSide {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterPrintSide;i4)");
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PosPrinterReleaseDeviceRequestedEventArgs(::windows::runtime::IInspectable);
impl PosPrinterReleaseDeviceRequestedEventArgs {}
unsafe impl ::windows::runtime::RuntimeType for PosPrinterReleaseDeviceRequestedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.PosPrinterReleaseDeviceRequestedEventArgs;{2bcba359-1cef-40b2-9ecb-f927f856ae3c})");
}
unsafe impl ::windows::runtime::Interface for PosPrinterReleaseDeviceRequestedEventArgs {
    type Vtable = IPosPrinterReleaseDeviceRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(734765913, 7407, 16562, [158, 203, 249, 39, 248, 86, 174, 60]);
}
impl ::windows::runtime::RuntimeName for PosPrinterReleaseDeviceRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.PosPrinterReleaseDeviceRequestedEventArgs";
}
impl ::std::convert::From<PosPrinterReleaseDeviceRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: PosPrinterReleaseDeviceRequestedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PosPrinterReleaseDeviceRequestedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &PosPrinterReleaseDeviceRequestedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PosPrinterReleaseDeviceRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PosPrinterReleaseDeviceRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<PosPrinterReleaseDeviceRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: PosPrinterReleaseDeviceRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PosPrinterReleaseDeviceRequestedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &PosPrinterReleaseDeviceRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PosPrinterReleaseDeviceRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PosPrinterReleaseDeviceRequestedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PosPrinterReleaseDeviceRequestedEventArgs {}
unsafe impl ::std::marker::Sync for PosPrinterReleaseDeviceRequestedEventArgs {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PosPrinterRotation(pub i32);
impl PosPrinterRotation {
    pub const Normal: PosPrinterRotation = PosPrinterRotation(0i32);
    pub const Right90: PosPrinterRotation = PosPrinterRotation(1i32);
    pub const Left90: PosPrinterRotation = PosPrinterRotation(2i32);
    pub const Rotate180: PosPrinterRotation = PosPrinterRotation(3i32);
}
impl ::std::convert::From<i32> for PosPrinterRotation {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PosPrinterRotation {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PosPrinterRotation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterRotation;i4)");
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PosPrinterRuledLineCapabilities(pub u32);
impl PosPrinterRuledLineCapabilities {
    pub const None: PosPrinterRuledLineCapabilities = PosPrinterRuledLineCapabilities(0u32);
    pub const Horizontal: PosPrinterRuledLineCapabilities = PosPrinterRuledLineCapabilities(1u32);
    pub const Vertical: PosPrinterRuledLineCapabilities = PosPrinterRuledLineCapabilities(2u32);
}
impl ::std::convert::From<u32> for PosPrinterRuledLineCapabilities {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PosPrinterRuledLineCapabilities {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PosPrinterRuledLineCapabilities {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterRuledLineCapabilities;u4)");
}
impl ::std::ops::BitOr for PosPrinterRuledLineCapabilities {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for PosPrinterRuledLineCapabilities {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for PosPrinterRuledLineCapabilities {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for PosPrinterRuledLineCapabilities {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for PosPrinterRuledLineCapabilities {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PosPrinterStatus(::windows::runtime::IInspectable);
impl PosPrinterStatus {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn StatusKind(&self) -> ::windows::runtime::Result<PosPrinterStatusKind> {
        let this = self;
        unsafe {
            let mut result__: PosPrinterStatusKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterStatusKind>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ExtendedStatus(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PosPrinterStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.PosPrinterStatus;{d1f0c730-da40-4328-bf76-5156fa33b747})");
}
unsafe impl ::windows::runtime::Interface for PosPrinterStatus {
    type Vtable = IPosPrinterStatus_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3522217776, 55872, 17192, [191, 118, 81, 86, 250, 51, 183, 71]);
}
impl ::windows::runtime::RuntimeName for PosPrinterStatus {
    const NAME: &'static str = "Windows.Devices.PointOfService.PosPrinterStatus";
}
impl ::std::convert::From<PosPrinterStatus> for ::windows::runtime::IUnknown {
    fn from(value: PosPrinterStatus) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PosPrinterStatus> for ::windows::runtime::IUnknown {
    fn from(value: &PosPrinterStatus) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PosPrinterStatus {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PosPrinterStatus {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<PosPrinterStatus> for ::windows::runtime::IInspectable {
    fn from(value: PosPrinterStatus) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PosPrinterStatus> for ::windows::runtime::IInspectable {
    fn from(value: &PosPrinterStatus) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PosPrinterStatus {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PosPrinterStatus {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PosPrinterStatus {}
unsafe impl ::std::marker::Sync for PosPrinterStatus {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PosPrinterStatusKind(pub i32);
impl PosPrinterStatusKind {
    pub const Online: PosPrinterStatusKind = PosPrinterStatusKind(0i32);
    pub const Off: PosPrinterStatusKind = PosPrinterStatusKind(1i32);
    pub const Offline: PosPrinterStatusKind = PosPrinterStatusKind(2i32);
    pub const OffOrOffline: PosPrinterStatusKind = PosPrinterStatusKind(3i32);
    pub const Extended: PosPrinterStatusKind = PosPrinterStatusKind(4i32);
}
impl ::std::convert::From<i32> for PosPrinterStatusKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PosPrinterStatusKind {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PosPrinterStatusKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterStatusKind;i4)");
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PosPrinterStatusUpdatedEventArgs(::windows::runtime::IInspectable);
impl PosPrinterStatusUpdatedEventArgs {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<PosPrinterStatus> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterStatus>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PosPrinterStatusUpdatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.PosPrinterStatusUpdatedEventArgs;{2edb87df-13a6-428d-ba81-b0e7c3e5a3cd})");
}
unsafe impl ::windows::runtime::Interface for PosPrinterStatusUpdatedEventArgs {
    type Vtable = IPosPrinterStatusUpdatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(786139103, 5030, 17037, [186, 129, 176, 231, 195, 229, 163, 205]);
}
impl ::windows::runtime::RuntimeName for PosPrinterStatusUpdatedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.PosPrinterStatusUpdatedEventArgs";
}
impl ::std::convert::From<PosPrinterStatusUpdatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: PosPrinterStatusUpdatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PosPrinterStatusUpdatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &PosPrinterStatusUpdatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PosPrinterStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PosPrinterStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<PosPrinterStatusUpdatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: PosPrinterStatusUpdatedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PosPrinterStatusUpdatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &PosPrinterStatusUpdatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PosPrinterStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PosPrinterStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PosPrinterStatusUpdatedEventArgs {}
unsafe impl ::std::marker::Sync for PosPrinterStatusUpdatedEventArgs {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ReceiptPrintJob(::windows::runtime::IInspectable);
impl ReceiptPrintJob {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn MarkFeed(&self, kind: PosPrinterMarkFeedKind) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), kind).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CutPaper(&self, percentage: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), percentage).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CutPaperDefault(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Print<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, data: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PrintLine<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, data: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PrintNewline(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn ExecuteAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::runtime::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetBarcodeRotation(&self, value: PosPrinterRotation) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetPrintRotation(&self, value: PosPrinterRotation, includebitmaps: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value, includebitmaps).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn SetPrintArea<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Rect>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Devices_PointOfService`, `Graphics_Imaging`*"]
    pub fn SetBitmap<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmapnumber: u32, bitmap: Param1, alignment: PosPrinterAlignment) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), bitmapnumber, bitmap.into_param().abi(), alignment).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Devices_PointOfService`, `Graphics_Imaging`*"]
    pub fn SetBitmapCustomWidthStandardAlign<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmapnumber: u32, bitmap: Param1, alignment: PosPrinterAlignment, width: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), bitmapnumber, bitmap.into_param().abi(), alignment, width).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Devices_PointOfService`, `Graphics_Imaging`*"]
    pub fn SetCustomAlignedBitmap<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmapnumber: u32, bitmap: Param1, alignmentdistance: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), bitmapnumber, bitmap.into_param().abi(), alignmentdistance).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Devices_PointOfService`, `Graphics_Imaging`*"]
    pub fn SetBitmapCustomWidthCustomAlign<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmapnumber: u32, bitmap: Param1, alignmentdistance: u32, width: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), bitmapnumber, bitmap.into_param().abi(), alignmentdistance, width).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PrintSavedBitmap(&self, bitmapnumber: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), bitmapnumber).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DrawRuledLine<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, positionlist: Param0, linedirection: PosPrinterLineDirection, linewidth: u32, linestyle: PosPrinterLineStyle, linecolor: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), positionlist.into_param().abi(), linedirection, linewidth, linestyle, linecolor).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PrintBarcode<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, data: Param0, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignment: PosPrinterAlignment) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), data.into_param().abi(), symbology, height, width, textposition, alignment).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PrintBarcodeCustomAlign<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, data: Param0, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignmentdistance: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), data.into_param().abi(), symbology, height, width, textposition, alignmentdistance).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Devices_PointOfService`, `Graphics_Imaging`*"]
    pub fn PrintBitmap<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmap: Param0, alignment: PosPrinterAlignment) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), bitmap.into_param().abi(), alignment).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Devices_PointOfService`, `Graphics_Imaging`*"]
    pub fn PrintBitmapCustomWidthStandardAlign<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmap: Param0, alignment: PosPrinterAlignment, width: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), bitmap.into_param().abi(), alignment, width).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Devices_PointOfService`, `Graphics_Imaging`*"]
    pub fn PrintCustomAlignedBitmap<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmap: Param0, alignmentdistance: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), bitmap.into_param().abi(), alignmentdistance).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Devices_PointOfService`, `Graphics_Imaging`*"]
    pub fn PrintBitmapCustomWidthCustomAlign<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmap: Param0, alignmentdistance: u32, width: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), bitmap.into_param().abi(), alignmentdistance, width).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn StampPaper(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IReceiptPrintJob2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Print2<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, PosPrinterPrintOptions>>(&self, data: Param0, printoptions: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IReceiptPrintJob2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), data.into_param().abi(), printoptions.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn FeedPaperByLine(&self, linecount: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IReceiptPrintJob2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), linecount).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn FeedPaperByMapModeUnit(&self, distance: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IReceiptPrintJob2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), distance).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ReceiptPrintJob {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ReceiptPrintJob;{aa96066e-acad-4b79-9d0f-c0cfc08dc77b})");
}
unsafe impl ::windows::runtime::Interface for ReceiptPrintJob {
    type Vtable = IReceiptPrintJob_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2861958766, 44205, 19321, [157, 15, 192, 207, 192, 141, 199, 123]);
}
impl ::windows::runtime::RuntimeName for ReceiptPrintJob {
    const NAME: &'static str = "Windows.Devices.PointOfService.ReceiptPrintJob";
}
impl ::std::convert::From<ReceiptPrintJob> for ::windows::runtime::IUnknown {
    fn from(value: ReceiptPrintJob) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ReceiptPrintJob> for ::windows::runtime::IUnknown {
    fn from(value: &ReceiptPrintJob) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ReceiptPrintJob {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ReceiptPrintJob {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ReceiptPrintJob> for ::windows::runtime::IInspectable {
    fn from(value: ReceiptPrintJob) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ReceiptPrintJob> for ::windows::runtime::IInspectable {
    fn from(value: &ReceiptPrintJob) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ReceiptPrintJob {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ReceiptPrintJob {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<ReceiptPrintJob> for IPosPrinterJob {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ReceiptPrintJob) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&ReceiptPrintJob> for IPosPrinterJob {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ReceiptPrintJob) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPosPrinterJob> for ReceiptPrintJob {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPosPrinterJob> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPosPrinterJob> for &ReceiptPrintJob {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPosPrinterJob> {
        ::std::convert::TryInto::<IPosPrinterJob>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<ReceiptPrintJob> for IReceiptOrSlipJob {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ReceiptPrintJob) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&ReceiptPrintJob> for IReceiptOrSlipJob {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ReceiptPrintJob) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IReceiptOrSlipJob> for ReceiptPrintJob {
    fn into_param(self) -> ::windows::runtime::Param<'a, IReceiptOrSlipJob> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IReceiptOrSlipJob> for &ReceiptPrintJob {
    fn into_param(self) -> ::windows::runtime::Param<'a, IReceiptOrSlipJob> {
        ::std::convert::TryInto::<IReceiptOrSlipJob>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for ReceiptPrintJob {}
unsafe impl ::std::marker::Sync for ReceiptPrintJob {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ReceiptPrinterCapabilities(::windows::runtime::IInspectable);
impl ReceiptPrinterCapabilities {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CanCutPaper(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsStampSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn MarkFeedCapabilities(&self) -> ::windows::runtime::Result<PosPrinterMarkFeedCapabilities> {
        let this = self;
        unsafe {
            let mut result__: PosPrinterMarkFeedCapabilities = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterMarkFeedCapabilities>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsPrinterPresent(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDualColorSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ColorCartridgeCapabilities(&self) -> ::windows::runtime::Result<PosPrinterColorCapabilities> {
        let this = &::windows::runtime::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: PosPrinterColorCapabilities = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterColorCapabilities>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CartridgeSensors(&self) -> ::windows::runtime::Result<PosPrinterCartridgeSensors> {
        let this = &::windows::runtime::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: PosPrinterCartridgeSensors = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterCartridgeSensors>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsBoldSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsItalicSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsUnderlineSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDoubleHighPrintSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDoubleWidePrintSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDoubleHighDoubleWidePrintSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsPaperEmptySensorSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsPaperNearEndSensorSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation_Collections`*"]
    pub fn SupportedCharactersPerLine(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &::windows::runtime::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsBarcodeSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsBitmapSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsLeft90RotationSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsRight90RotationSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Is180RotationSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsPrintAreaSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn RuledLineCapabilities(&self) -> ::windows::runtime::Result<PosPrinterRuledLineCapabilities> {
        let this = &::windows::runtime::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__: PosPrinterRuledLineCapabilities = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterRuledLineCapabilities>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation_Collections`*"]
    pub fn SupportedBarcodeRotations(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>> {
        let this = &::windows::runtime::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation_Collections`*"]
    pub fn SupportedBitmapRotations(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>> {
        let this = &::windows::runtime::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsReverseVideoSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IReceiptPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsStrikethroughSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IReceiptPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsSuperscriptSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IReceiptPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsSubscriptSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IReceiptPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsReversePaperFeedByLineSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IReceiptPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsReversePaperFeedByMapModeUnitSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IReceiptPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ReceiptPrinterCapabilities {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ReceiptPrinterCapabilities;{b8f0b58f-51a8-43fc-9bd5-8de272a6415b})");
}
unsafe impl ::windows::runtime::Interface for ReceiptPrinterCapabilities {
    type Vtable = IReceiptPrinterCapabilities_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3102782863, 20904, 17404, [155, 213, 141, 226, 114, 166, 65, 91]);
}
impl ::windows::runtime::RuntimeName for ReceiptPrinterCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.ReceiptPrinterCapabilities";
}
impl ::std::convert::From<ReceiptPrinterCapabilities> for ::windows::runtime::IUnknown {
    fn from(value: ReceiptPrinterCapabilities) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ReceiptPrinterCapabilities> for ::windows::runtime::IUnknown {
    fn from(value: &ReceiptPrinterCapabilities) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ReceiptPrinterCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ReceiptPrinterCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ReceiptPrinterCapabilities> for ::windows::runtime::IInspectable {
    fn from(value: ReceiptPrinterCapabilities) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ReceiptPrinterCapabilities> for ::windows::runtime::IInspectable {
    fn from(value: &ReceiptPrinterCapabilities) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ReceiptPrinterCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ReceiptPrinterCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<ReceiptPrinterCapabilities> for ICommonPosPrintStationCapabilities {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ReceiptPrinterCapabilities) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&ReceiptPrinterCapabilities> for ICommonPosPrintStationCapabilities {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ReceiptPrinterCapabilities) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ICommonPosPrintStationCapabilities> for ReceiptPrinterCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ICommonPosPrintStationCapabilities> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ICommonPosPrintStationCapabilities> for &ReceiptPrinterCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ICommonPosPrintStationCapabilities> {
        ::std::convert::TryInto::<ICommonPosPrintStationCapabilities>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<ReceiptPrinterCapabilities> for ICommonReceiptSlipCapabilities {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ReceiptPrinterCapabilities) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&ReceiptPrinterCapabilities> for ICommonReceiptSlipCapabilities {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ReceiptPrinterCapabilities) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ICommonReceiptSlipCapabilities> for ReceiptPrinterCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ICommonReceiptSlipCapabilities> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ICommonReceiptSlipCapabilities> for &ReceiptPrinterCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ICommonReceiptSlipCapabilities> {
        ::std::convert::TryInto::<ICommonReceiptSlipCapabilities>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for ReceiptPrinterCapabilities {}
unsafe impl ::std::marker::Sync for ReceiptPrinterCapabilities {}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Devices_PointOfService`*"]
pub struct SizeUInt32 {
    pub Width: u32,
    pub Height: u32,
}
impl SizeUInt32 {}
impl ::std::default::Default for SizeUInt32 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SizeUInt32 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SizeUInt32").field("Width", &self.Width).field("Height", &self.Height).finish()
    }
}
impl ::std::cmp::PartialEq for SizeUInt32 {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width && self.Height == other.Height
    }
}
impl ::std::cmp::Eq for SizeUInt32 {}
unsafe impl ::windows::runtime::Abi for SizeUInt32 {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SizeUInt32 {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Devices.PointOfService.SizeUInt32;u4;u4)");
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SlipPrintJob(::windows::runtime::IInspectable);
impl SlipPrintJob {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetBarcodeRotation(&self, value: PosPrinterRotation) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn SetPrintRotation(&self, value: PosPrinterRotation, includebitmaps: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value, includebitmaps).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn SetPrintArea<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Rect>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Devices_PointOfService`, `Graphics_Imaging`*"]
    pub fn SetBitmap<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmapnumber: u32, bitmap: Param1, alignment: PosPrinterAlignment) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), bitmapnumber, bitmap.into_param().abi(), alignment).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Devices_PointOfService`, `Graphics_Imaging`*"]
    pub fn SetBitmapCustomWidthStandardAlign<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmapnumber: u32, bitmap: Param1, alignment: PosPrinterAlignment, width: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), bitmapnumber, bitmap.into_param().abi(), alignment, width).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Devices_PointOfService`, `Graphics_Imaging`*"]
    pub fn SetCustomAlignedBitmap<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmapnumber: u32, bitmap: Param1, alignmentdistance: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), bitmapnumber, bitmap.into_param().abi(), alignmentdistance).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Devices_PointOfService`, `Graphics_Imaging`*"]
    pub fn SetBitmapCustomWidthCustomAlign<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmapnumber: u32, bitmap: Param1, alignmentdistance: u32, width: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), bitmapnumber, bitmap.into_param().abi(), alignmentdistance, width).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PrintSavedBitmap(&self, bitmapnumber: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), bitmapnumber).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn DrawRuledLine<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, positionlist: Param0, linedirection: PosPrinterLineDirection, linewidth: u32, linestyle: PosPrinterLineStyle, linecolor: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), positionlist.into_param().abi(), linedirection, linewidth, linestyle, linecolor).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PrintBarcode<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, data: Param0, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignment: PosPrinterAlignment) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), data.into_param().abi(), symbology, height, width, textposition, alignment).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PrintBarcodeCustomAlign<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, data: Param0, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignmentdistance: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), data.into_param().abi(), symbology, height, width, textposition, alignmentdistance).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Devices_PointOfService`, `Graphics_Imaging`*"]
    pub fn PrintBitmap<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmap: Param0, alignment: PosPrinterAlignment) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), bitmap.into_param().abi(), alignment).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Devices_PointOfService`, `Graphics_Imaging`*"]
    pub fn PrintBitmapCustomWidthStandardAlign<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmap: Param0, alignment: PosPrinterAlignment, width: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), bitmap.into_param().abi(), alignment, width).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Devices_PointOfService`, `Graphics_Imaging`*"]
    pub fn PrintCustomAlignedBitmap<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmap: Param0, alignmentdistance: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), bitmap.into_param().abi(), alignmentdistance).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    #[doc = "*Required features: `Devices_PointOfService`, `Graphics_Imaging`*"]
    pub fn PrintBitmapCustomWidthCustomAlign<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmap: Param0, alignmentdistance: u32, width: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), bitmap.into_param().abi(), alignmentdistance, width).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Print<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, data: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PrintLine<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, data: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn PrintNewline(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation`*"]
    pub fn ExecuteAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::runtime::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Print2<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, PosPrinterPrintOptions>>(&self, data: Param0, printoptions: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISlipPrintJob>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), data.into_param().abi(), printoptions.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn FeedPaperByLine(&self, linecount: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISlipPrintJob>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), linecount).ok() }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn FeedPaperByMapModeUnit(&self, distance: i32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ISlipPrintJob>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), distance).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SlipPrintJob {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.SlipPrintJob;{532199be-c8c3-4dc2-89e9-5c4a37b34ddc})");
}
unsafe impl ::windows::runtime::Interface for SlipPrintJob {
    type Vtable = IReceiptOrSlipJob_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1394710974, 51395, 19906, [137, 233, 92, 74, 55, 179, 77, 220]);
}
impl ::windows::runtime::RuntimeName for SlipPrintJob {
    const NAME: &'static str = "Windows.Devices.PointOfService.SlipPrintJob";
}
impl ::std::convert::From<SlipPrintJob> for ::windows::runtime::IUnknown {
    fn from(value: SlipPrintJob) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SlipPrintJob> for ::windows::runtime::IUnknown {
    fn from(value: &SlipPrintJob) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SlipPrintJob {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SlipPrintJob {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SlipPrintJob> for ::windows::runtime::IInspectable {
    fn from(value: SlipPrintJob) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SlipPrintJob> for ::windows::runtime::IInspectable {
    fn from(value: &SlipPrintJob) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SlipPrintJob {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SlipPrintJob {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<SlipPrintJob> for IReceiptOrSlipJob {
    fn from(value: SlipPrintJob) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SlipPrintJob> for IReceiptOrSlipJob {
    fn from(value: &SlipPrintJob) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IReceiptOrSlipJob> for SlipPrintJob {
    fn into_param(self) -> ::windows::runtime::Param<'a, IReceiptOrSlipJob> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IReceiptOrSlipJob>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IReceiptOrSlipJob> for &SlipPrintJob {
    fn into_param(self) -> ::windows::runtime::Param<'a, IReceiptOrSlipJob> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IReceiptOrSlipJob>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::TryFrom<SlipPrintJob> for IPosPrinterJob {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SlipPrintJob) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SlipPrintJob> for IPosPrinterJob {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SlipPrintJob) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPosPrinterJob> for SlipPrintJob {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPosPrinterJob> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPosPrinterJob> for &SlipPrintJob {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPosPrinterJob> {
        ::std::convert::TryInto::<IPosPrinterJob>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for SlipPrintJob {}
unsafe impl ::std::marker::Sync for SlipPrintJob {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct SlipPrinterCapabilities(::windows::runtime::IInspectable);
impl SlipPrinterCapabilities {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsFullLengthSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsBothSidesPrintingSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsPrinterPresent(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDualColorSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ColorCartridgeCapabilities(&self) -> ::windows::runtime::Result<PosPrinterColorCapabilities> {
        let this = &::windows::runtime::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: PosPrinterColorCapabilities = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterColorCapabilities>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CartridgeSensors(&self) -> ::windows::runtime::Result<PosPrinterCartridgeSensors> {
        let this = &::windows::runtime::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: PosPrinterCartridgeSensors = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterCartridgeSensors>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsBoldSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsItalicSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsUnderlineSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDoubleHighPrintSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDoubleWidePrintSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsDoubleHighDoubleWidePrintSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsPaperEmptySensorSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsPaperNearEndSensorSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation_Collections`*"]
    pub fn SupportedCharactersPerLine(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &::windows::runtime::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsBarcodeSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsBitmapSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsLeft90RotationSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsRight90RotationSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Is180RotationSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsPrintAreaSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn RuledLineCapabilities(&self) -> ::windows::runtime::Result<PosPrinterRuledLineCapabilities> {
        let this = &::windows::runtime::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__: PosPrinterRuledLineCapabilities = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterRuledLineCapabilities>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation_Collections`*"]
    pub fn SupportedBarcodeRotations(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>> {
        let this = &::windows::runtime::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Devices_PointOfService`, `Foundation_Collections`*"]
    pub fn SupportedBitmapRotations(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>> {
        let this = &::windows::runtime::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsReverseVideoSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ISlipPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsStrikethroughSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ISlipPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsSuperscriptSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ISlipPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsSubscriptSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ISlipPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsReversePaperFeedByLineSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ISlipPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn IsReversePaperFeedByMapModeUnitSupported(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ISlipPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for SlipPrinterCapabilities {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.SlipPrinterCapabilities;{99b16399-488c-4157-8ac2-9f57f708d3db})");
}
unsafe impl ::windows::runtime::Interface for SlipPrinterCapabilities {
    type Vtable = ISlipPrinterCapabilities_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2578539417, 18572, 16727, [138, 194, 159, 87, 247, 8, 211, 219]);
}
impl ::windows::runtime::RuntimeName for SlipPrinterCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.SlipPrinterCapabilities";
}
impl ::std::convert::From<SlipPrinterCapabilities> for ::windows::runtime::IUnknown {
    fn from(value: SlipPrinterCapabilities) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&SlipPrinterCapabilities> for ::windows::runtime::IUnknown {
    fn from(value: &SlipPrinterCapabilities) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SlipPrinterCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SlipPrinterCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<SlipPrinterCapabilities> for ::windows::runtime::IInspectable {
    fn from(value: SlipPrinterCapabilities) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SlipPrinterCapabilities> for ::windows::runtime::IInspectable {
    fn from(value: &SlipPrinterCapabilities) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for SlipPrinterCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a SlipPrinterCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<SlipPrinterCapabilities> for ICommonPosPrintStationCapabilities {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SlipPrinterCapabilities) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SlipPrinterCapabilities> for ICommonPosPrintStationCapabilities {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SlipPrinterCapabilities) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ICommonPosPrintStationCapabilities> for SlipPrinterCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ICommonPosPrintStationCapabilities> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ICommonPosPrintStationCapabilities> for &SlipPrinterCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ICommonPosPrintStationCapabilities> {
        ::std::convert::TryInto::<ICommonPosPrintStationCapabilities>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<SlipPrinterCapabilities> for ICommonReceiptSlipCapabilities {
    type Error = ::windows::runtime::Error;
    fn try_from(value: SlipPrinterCapabilities) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&SlipPrinterCapabilities> for ICommonReceiptSlipCapabilities {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &SlipPrinterCapabilities) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ICommonReceiptSlipCapabilities> for SlipPrinterCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ICommonReceiptSlipCapabilities> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ICommonReceiptSlipCapabilities> for &SlipPrinterCapabilities {
    fn into_param(self) -> ::windows::runtime::Param<'a, ICommonReceiptSlipCapabilities> {
        ::std::convert::TryInto::<ICommonReceiptSlipCapabilities>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for SlipPrinterCapabilities {}
unsafe impl ::std::marker::Sync for SlipPrinterCapabilities {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct UnifiedPosErrorData(::windows::runtime::IInspectable);
impl UnifiedPosErrorData {
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Message(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Severity(&self) -> ::windows::runtime::Result<UnifiedPosErrorSeverity> {
        let this = self;
        unsafe {
            let mut result__: UnifiedPosErrorSeverity = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UnifiedPosErrorSeverity>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn Reason(&self) -> ::windows::runtime::Result<UnifiedPosErrorReason> {
        let this = self;
        unsafe {
            let mut result__: UnifiedPosErrorReason = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UnifiedPosErrorReason>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn ExtendedReason(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Devices_PointOfService`*"]
    pub fn CreateInstance<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(message: Param0, severity: UnifiedPosErrorSeverity, reason: UnifiedPosErrorReason, extendedreason: u32) -> ::windows::runtime::Result<UnifiedPosErrorData> {
        Self::IUnifiedPosErrorDataFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), message.into_param().abi(), severity, reason, extendedreason, &mut result__).from_abi::<UnifiedPosErrorData>(result__)
        })
    }
    pub fn IUnifiedPosErrorDataFactory<R, F: FnOnce(&IUnifiedPosErrorDataFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<UnifiedPosErrorData, IUnifiedPosErrorDataFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for UnifiedPosErrorData {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.UnifiedPosErrorData;{2b998c3a-555c-4889-8ed8-c599bb3a712a})");
}
unsafe impl ::windows::runtime::Interface for UnifiedPosErrorData {
    type Vtable = IUnifiedPosErrorData_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(731483194, 21852, 18569, [142, 216, 197, 153, 187, 58, 113, 42]);
}
impl ::windows::runtime::RuntimeName for UnifiedPosErrorData {
    const NAME: &'static str = "Windows.Devices.PointOfService.UnifiedPosErrorData";
}
impl ::std::convert::From<UnifiedPosErrorData> for ::windows::runtime::IUnknown {
    fn from(value: UnifiedPosErrorData) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&UnifiedPosErrorData> for ::windows::runtime::IUnknown {
    fn from(value: &UnifiedPosErrorData) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for UnifiedPosErrorData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &UnifiedPosErrorData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<UnifiedPosErrorData> for ::windows::runtime::IInspectable {
    fn from(value: UnifiedPosErrorData) -> Self {
        value.0
    }
}
impl ::std::convert::From<&UnifiedPosErrorData> for ::windows::runtime::IInspectable {
    fn from(value: &UnifiedPosErrorData) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for UnifiedPosErrorData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a UnifiedPosErrorData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for UnifiedPosErrorData {}
unsafe impl ::std::marker::Sync for UnifiedPosErrorData {}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for UnifiedPosErrorReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UnifiedPosErrorReason {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for UnifiedPosErrorReason {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.UnifiedPosErrorReason;i4)");
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for UnifiedPosErrorSeverity {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UnifiedPosErrorSeverity {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for UnifiedPosErrorSeverity {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.UnifiedPosErrorSeverity;i4)");
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct UnifiedPosHealthCheckLevel(pub i32);
impl UnifiedPosHealthCheckLevel {
    pub const UnknownHealthCheckLevel: UnifiedPosHealthCheckLevel = UnifiedPosHealthCheckLevel(0i32);
    pub const POSInternal: UnifiedPosHealthCheckLevel = UnifiedPosHealthCheckLevel(1i32);
    pub const External: UnifiedPosHealthCheckLevel = UnifiedPosHealthCheckLevel(2i32);
    pub const Interactive: UnifiedPosHealthCheckLevel = UnifiedPosHealthCheckLevel(3i32);
}
impl ::std::convert::From<i32> for UnifiedPosHealthCheckLevel {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UnifiedPosHealthCheckLevel {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for UnifiedPosHealthCheckLevel {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.UnifiedPosHealthCheckLevel;i4)");
}
#[doc = "*Required features: `Devices_PointOfService`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct UnifiedPosPowerReportingType(pub i32);
impl UnifiedPosPowerReportingType {
    pub const UnknownPowerReportingType: UnifiedPosPowerReportingType = UnifiedPosPowerReportingType(0i32);
    pub const Standard: UnifiedPosPowerReportingType = UnifiedPosPowerReportingType(1i32);
    pub const Advanced: UnifiedPosPowerReportingType = UnifiedPosPowerReportingType(2i32);
}
impl ::std::convert::From<i32> for UnifiedPosPowerReportingType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UnifiedPosPowerReportingType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for UnifiedPosPowerReportingType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.UnifiedPosPowerReportingType;i4)");
}

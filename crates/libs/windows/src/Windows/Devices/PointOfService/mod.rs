#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Devices_PointOfService_Provider")]
pub mod Provider;
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct BarcodeScanner(::windows::core::IUnknown);
impl BarcodeScanner {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Capabilities(&self) -> ::windows::core::Result<BarcodeScannerCapabilities> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Capabilities)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BarcodeScannerCapabilities>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ClaimScannerAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ClaimedBarcodeScanner>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ClaimScannerAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ClaimedBarcodeScanner>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CheckHealthAsync(&self, level: UnifiedPosHealthCheckLevel) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CheckHealthAsync)(::core::mem::transmute_copy(this), level, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSupportedSymbologiesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<u32>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetSupportedSymbologiesAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<u32>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsSymbologySupportedAsync(&self, barcodesymbology: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsSymbologySupportedAsync)(::core::mem::transmute_copy(this), barcodesymbology, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation_Collections\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn RetrieveStatisticsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, statisticscategories: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RetrieveStatisticsAsync)(::core::mem::transmute_copy(this), statisticscategories.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSupportedProfiles(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetSupportedProfiles)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsProfileSupported<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, profile: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsProfileSupported)(::core::mem::transmute_copy(this), profile.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StatusUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<BarcodeScanner, BarcodeScannerStatusUpdatedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StatusUpdated)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStatusUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveStatusUpdated)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn VideoDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IBarcodeScanner2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VideoDeviceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDefaultAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BarcodeScanner>> {
        Self::IBarcodeScannerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDefaultAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BarcodeScanner>>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BarcodeScanner>> {
        Self::IBarcodeScannerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FromIdAsync)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BarcodeScanner>>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBarcodeScannerStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeviceSelector)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn GetDeviceSelectorWithConnectionTypes(connectiontypes: PosConnectionTypes) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBarcodeScannerStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeviceSelectorWithConnectionTypes)(::core::mem::transmute_copy(this), connectiontypes, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc(hidden)]
    pub fn IBarcodeScannerStatics<R, F: FnOnce(&IBarcodeScannerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BarcodeScanner, IBarcodeScannerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IBarcodeScannerStatics2<R, F: FnOnce(&IBarcodeScannerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BarcodeScanner, IBarcodeScannerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for BarcodeScanner {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScanner {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScanner {}
impl ::core::fmt::Debug for BarcodeScanner {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeScanner").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BarcodeScanner {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.BarcodeScanner;{bea33e06-b264-4f03-a9c1-45b20f01134f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for BarcodeScanner {
    type Vtable = IBarcodeScanner_Vtbl;
    const IID: ::windows::core::GUID = <IBarcodeScanner as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BarcodeScanner {
    const NAME: &'static str = "Windows.Devices.PointOfService.BarcodeScanner";
}
impl ::core::convert::From<BarcodeScanner> for ::windows::core::IUnknown {
    fn from(value: BarcodeScanner) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScanner> for ::windows::core::IUnknown {
    fn from(value: &BarcodeScanner) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BarcodeScanner {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BarcodeScanner {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScanner> for ::windows::core::IInspectable {
    fn from(value: BarcodeScanner) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScanner> for ::windows::core::IInspectable {
    fn from(value: &BarcodeScanner) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BarcodeScanner {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BarcodeScanner {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct BarcodeScannerCapabilities(::windows::core::IUnknown);
impl BarcodeScannerCapabilities {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn PowerReportingType(&self) -> ::windows::core::Result<UnifiedPosPowerReportingType> {
        let this = self;
        unsafe {
            let mut result__: UnifiedPosPowerReportingType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PowerReportingType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UnifiedPosPowerReportingType>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsStatisticsReportingSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsStatisticsReportingSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsStatisticsUpdatingSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsStatisticsUpdatingSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsImagePreviewSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsImagePreviewSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsSoftwareTriggerSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IBarcodeScannerCapabilities1>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsSoftwareTriggerSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsVideoPreviewSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IBarcodeScannerCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsVideoPreviewSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeScannerCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerCapabilities {}
impl ::core::fmt::Debug for BarcodeScannerCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeScannerCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BarcodeScannerCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.BarcodeScannerCapabilities;{c60691e4-f2c8-4420-a307-b12ef6622857})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for BarcodeScannerCapabilities {
    type Vtable = IBarcodeScannerCapabilities_Vtbl;
    const IID: ::windows::core::GUID = <IBarcodeScannerCapabilities as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BarcodeScannerCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.BarcodeScannerCapabilities";
}
impl ::core::convert::From<BarcodeScannerCapabilities> for ::windows::core::IUnknown {
    fn from(value: BarcodeScannerCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerCapabilities> for ::windows::core::IUnknown {
    fn from(value: &BarcodeScannerCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BarcodeScannerCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BarcodeScannerCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerCapabilities> for ::windows::core::IInspectable {
    fn from(value: BarcodeScannerCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerCapabilities> for ::windows::core::IInspectable {
    fn from(value: &BarcodeScannerCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BarcodeScannerCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BarcodeScannerCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerCapabilities {}
unsafe impl ::core::marker::Sync for BarcodeScannerCapabilities {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct BarcodeScannerDataReceivedEventArgs(::windows::core::IUnknown);
impl BarcodeScannerDataReceivedEventArgs {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Report(&self) -> ::windows::core::Result<BarcodeScannerReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Report)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BarcodeScannerReport>(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeScannerDataReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerDataReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerDataReceivedEventArgs {}
impl ::core::fmt::Debug for BarcodeScannerDataReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeScannerDataReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BarcodeScannerDataReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.BarcodeScannerDataReceivedEventArgs;{4234a7e2-ed97-467d-ad2b-01e44313a929})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for BarcodeScannerDataReceivedEventArgs {
    type Vtable = IBarcodeScannerDataReceivedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IBarcodeScannerDataReceivedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BarcodeScannerDataReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.BarcodeScannerDataReceivedEventArgs";
}
impl ::core::convert::From<BarcodeScannerDataReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: BarcodeScannerDataReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerDataReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &BarcodeScannerDataReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BarcodeScannerDataReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BarcodeScannerDataReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerDataReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: BarcodeScannerDataReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerDataReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &BarcodeScannerDataReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BarcodeScannerDataReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BarcodeScannerDataReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerDataReceivedEventArgs {}
unsafe impl ::core::marker::Sync for BarcodeScannerDataReceivedEventArgs {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct BarcodeScannerErrorOccurredEventArgs(::windows::core::IUnknown);
impl BarcodeScannerErrorOccurredEventArgs {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn PartialInputData(&self) -> ::windows::core::Result<BarcodeScannerReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PartialInputData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BarcodeScannerReport>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsRetriable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsRetriable)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn ErrorData(&self) -> ::windows::core::Result<UnifiedPosErrorData> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ErrorData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UnifiedPosErrorData>(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeScannerErrorOccurredEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerErrorOccurredEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerErrorOccurredEventArgs {}
impl ::core::fmt::Debug for BarcodeScannerErrorOccurredEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeScannerErrorOccurredEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BarcodeScannerErrorOccurredEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.BarcodeScannerErrorOccurredEventArgs;{2cd2602f-cf3a-4002-a75a-c5ec468f0a20})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for BarcodeScannerErrorOccurredEventArgs {
    type Vtable = IBarcodeScannerErrorOccurredEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IBarcodeScannerErrorOccurredEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BarcodeScannerErrorOccurredEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.BarcodeScannerErrorOccurredEventArgs";
}
impl ::core::convert::From<BarcodeScannerErrorOccurredEventArgs> for ::windows::core::IUnknown {
    fn from(value: BarcodeScannerErrorOccurredEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerErrorOccurredEventArgs> for ::windows::core::IUnknown {
    fn from(value: &BarcodeScannerErrorOccurredEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BarcodeScannerErrorOccurredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BarcodeScannerErrorOccurredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerErrorOccurredEventArgs> for ::windows::core::IInspectable {
    fn from(value: BarcodeScannerErrorOccurredEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerErrorOccurredEventArgs> for ::windows::core::IInspectable {
    fn from(value: &BarcodeScannerErrorOccurredEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BarcodeScannerErrorOccurredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BarcodeScannerErrorOccurredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerErrorOccurredEventArgs {}
unsafe impl ::core::marker::Sync for BarcodeScannerErrorOccurredEventArgs {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct BarcodeScannerImagePreviewReceivedEventArgs(::windows::core::IUnknown);
impl BarcodeScannerImagePreviewReceivedEventArgs {
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Preview(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamWithContentType> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Preview)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStreamWithContentType>(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeScannerImagePreviewReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerImagePreviewReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerImagePreviewReceivedEventArgs {}
impl ::core::fmt::Debug for BarcodeScannerImagePreviewReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeScannerImagePreviewReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BarcodeScannerImagePreviewReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.BarcodeScannerImagePreviewReceivedEventArgs;{f3b7de85-6e8b-434e-9f58-06ef26bc4baf})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for BarcodeScannerImagePreviewReceivedEventArgs {
    type Vtable = IBarcodeScannerImagePreviewReceivedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IBarcodeScannerImagePreviewReceivedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BarcodeScannerImagePreviewReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.BarcodeScannerImagePreviewReceivedEventArgs";
}
impl ::core::convert::From<BarcodeScannerImagePreviewReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: BarcodeScannerImagePreviewReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerImagePreviewReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &BarcodeScannerImagePreviewReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BarcodeScannerImagePreviewReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BarcodeScannerImagePreviewReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerImagePreviewReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: BarcodeScannerImagePreviewReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerImagePreviewReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &BarcodeScannerImagePreviewReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BarcodeScannerImagePreviewReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BarcodeScannerImagePreviewReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerImagePreviewReceivedEventArgs {}
unsafe impl ::core::marker::Sync for BarcodeScannerImagePreviewReceivedEventArgs {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct BarcodeScannerReport(::windows::core::IUnknown);
impl BarcodeScannerReport {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn ScanDataType(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ScanDataType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ScanData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ScanData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ScanDataLabel(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ScanDataLabel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateInstance<'a, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>, Param2: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(scandatatype: u32, scandata: Param1, scandatalabel: Param2) -> ::windows::core::Result<BarcodeScannerReport> {
        Self::IBarcodeScannerReportFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), scandatatype, scandata.into_param().abi(), scandatalabel.into_param().abi(), &mut result__).from_abi::<BarcodeScannerReport>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBarcodeScannerReportFactory<R, F: FnOnce(&IBarcodeScannerReportFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BarcodeScannerReport, IBarcodeScannerReportFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for BarcodeScannerReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerReport {}
impl ::core::fmt::Debug for BarcodeScannerReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeScannerReport").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BarcodeScannerReport {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.BarcodeScannerReport;{5ce4d8b0-a489-4b96-86c4-f0bf8a37753d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for BarcodeScannerReport {
    type Vtable = IBarcodeScannerReport_Vtbl;
    const IID: ::windows::core::GUID = <IBarcodeScannerReport as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BarcodeScannerReport {
    const NAME: &'static str = "Windows.Devices.PointOfService.BarcodeScannerReport";
}
impl ::core::convert::From<BarcodeScannerReport> for ::windows::core::IUnknown {
    fn from(value: BarcodeScannerReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerReport> for ::windows::core::IUnknown {
    fn from(value: &BarcodeScannerReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BarcodeScannerReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BarcodeScannerReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerReport> for ::windows::core::IInspectable {
    fn from(value: BarcodeScannerReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerReport> for ::windows::core::IInspectable {
    fn from(value: &BarcodeScannerReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BarcodeScannerReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BarcodeScannerReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerReport {}
unsafe impl ::core::marker::Sync for BarcodeScannerReport {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BarcodeScannerStatus(pub i32);
impl BarcodeScannerStatus {
    pub const Online: Self = Self(0i32);
    pub const Off: Self = Self(1i32);
    pub const Offline: Self = Self(2i32);
    pub const OffOrOffline: Self = Self(3i32);
    pub const Extended: Self = Self(4i32);
}
impl ::core::marker::Copy for BarcodeScannerStatus {}
impl ::core::clone::Clone for BarcodeScannerStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BarcodeScannerStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BarcodeScannerStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for BarcodeScannerStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeScannerStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BarcodeScannerStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.BarcodeScannerStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct BarcodeScannerStatusUpdatedEventArgs(::windows::core::IUnknown);
impl BarcodeScannerStatusUpdatedEventArgs {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<BarcodeScannerStatus> {
        let this = self;
        unsafe {
            let mut result__: BarcodeScannerStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BarcodeScannerStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn ExtendedStatus(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedStatus)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeScannerStatusUpdatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeScannerStatusUpdatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeScannerStatusUpdatedEventArgs {}
impl ::core::fmt::Debug for BarcodeScannerStatusUpdatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeScannerStatusUpdatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BarcodeScannerStatusUpdatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.BarcodeScannerStatusUpdatedEventArgs;{355d8586-9c43-462b-a91a-816dc97f452c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for BarcodeScannerStatusUpdatedEventArgs {
    type Vtable = IBarcodeScannerStatusUpdatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IBarcodeScannerStatusUpdatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BarcodeScannerStatusUpdatedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.BarcodeScannerStatusUpdatedEventArgs";
}
impl ::core::convert::From<BarcodeScannerStatusUpdatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: BarcodeScannerStatusUpdatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerStatusUpdatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &BarcodeScannerStatusUpdatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BarcodeScannerStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BarcodeScannerStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeScannerStatusUpdatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: BarcodeScannerStatusUpdatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeScannerStatusUpdatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &BarcodeScannerStatusUpdatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BarcodeScannerStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BarcodeScannerStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarcodeScannerStatusUpdatedEventArgs {}
unsafe impl ::core::marker::Sync for BarcodeScannerStatusUpdatedEventArgs {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
pub struct BarcodeSymbologies {}
impl BarcodeSymbologies {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Unknown() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Unknown)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Ean8() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Ean8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Ean8Add2() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Ean8Add2)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Ean8Add5() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Ean8Add5)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Eanv() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Eanv)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn EanvAdd2() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EanvAdd2)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn EanvAdd5() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EanvAdd5)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Ean13() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Ean13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Ean13Add2() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Ean13Add2)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Ean13Add5() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Ean13Add5)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Isbn() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Isbn)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsbnAdd5() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsbnAdd5)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Ismn() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Ismn)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsmnAdd2() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsmnAdd2)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsmnAdd5() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsmnAdd5)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Issn() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Issn)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IssnAdd2() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IssnAdd2)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IssnAdd5() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IssnAdd5)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Ean99() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Ean99)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Ean99Add2() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Ean99Add2)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Ean99Add5() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Ean99Add5)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Upca() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Upca)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn UpcaAdd2() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UpcaAdd2)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn UpcaAdd5() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UpcaAdd5)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Upce() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Upce)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn UpceAdd2() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UpceAdd2)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn UpceAdd5() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UpceAdd5)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn UpcCoupon() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UpcCoupon)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn TfStd() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TfStd)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn TfDis() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TfDis)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn TfInt() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TfInt)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn TfInd() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TfInd)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn TfMat() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TfMat)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn TfIata() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TfIata)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Gs1DatabarType1() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Gs1DatabarType1)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Gs1DatabarType2() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Gs1DatabarType2)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Gs1DatabarType3() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Gs1DatabarType3)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Code39() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Code39)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Code39Ex() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Code39Ex)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Trioptic39() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Trioptic39)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Code32() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Code32)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Pzn() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Pzn)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Code93() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Code93)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Code93Ex() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Code93Ex)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Code128() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Code128)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Gs1128() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Gs1128)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Gs1128Coupon() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Gs1128Coupon)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn UccEan128() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UccEan128)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Sisac() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Sisac)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Isbt() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Isbt)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Codabar() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Codabar)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Code11() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Code11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Msi() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Msi)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Plessey() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Plessey)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Telepen() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Telepen)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Code16k() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Code16k)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn CodablockA() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CodablockA)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn CodablockF() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CodablockF)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Codablock128() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Codablock128)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Code49() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Code49)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Aztec() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Aztec)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn DataCode() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DataCode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn DataMatrix() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DataMatrix)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn HanXin() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HanXin)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Maxicode() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Maxicode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn MicroPdf417() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MicroPdf417)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn MicroQr() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MicroQr)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Pdf417() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Pdf417)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Qr() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Qr)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn MsTag() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MsTag)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Ccab() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Ccab)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Ccc() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Ccc)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Tlc39() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Tlc39)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn AusPost() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AusPost)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn CanPost() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanPost)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn ChinaPost() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ChinaPost)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn DutchKix() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DutchKix)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn InfoMail() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InfoMail)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn ItalianPost25() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ItalianPost25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn ItalianPost39() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ItalianPost39)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn JapanPost() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).JapanPost)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn KoreanPost() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).KoreanPost)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SwedenPost() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SwedenPost)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn UkPost() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UkPost)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn UsIntelligent() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UsIntelligent)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn UsIntelligentPkg() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UsIntelligentPkg)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn UsPlanet() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UsPlanet)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn UsPostNet() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UsPostNet)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Us4StateFics() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Us4StateFics)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn OcrA() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OcrA)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn OcrB() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OcrB)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Micr() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Micr)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn ExtendedBase() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedBase)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn GetName(scandatatype: u32) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IBarcodeSymbologiesStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetName)(::core::mem::transmute_copy(this), scandatatype, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Gs1DWCode() -> ::windows::core::Result<u32> {
        Self::IBarcodeSymbologiesStatics2(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Gs1DWCode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBarcodeSymbologiesStatics<R, F: FnOnce(&IBarcodeSymbologiesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BarcodeSymbologies, IBarcodeSymbologiesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IBarcodeSymbologiesStatics2<R, F: FnOnce(&IBarcodeSymbologiesStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BarcodeSymbologies, IBarcodeSymbologiesStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for BarcodeSymbologies {
    const NAME: &'static str = "Windows.Devices.PointOfService.BarcodeSymbologies";
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct BarcodeSymbologyAttributes(::windows::core::IUnknown);
impl BarcodeSymbologyAttributes {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsCheckDigitValidationEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCheckDigitValidationEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetIsCheckDigitValidationEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsCheckDigitValidationEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsCheckDigitValidationSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCheckDigitValidationSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsCheckDigitTransmissionEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCheckDigitTransmissionEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetIsCheckDigitTransmissionEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsCheckDigitTransmissionEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsCheckDigitTransmissionSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCheckDigitTransmissionSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn DecodeLength1(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DecodeLength1)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetDecodeLength1(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDecodeLength1)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn DecodeLength2(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DecodeLength2)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetDecodeLength2(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDecodeLength2)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn DecodeLengthKind(&self) -> ::windows::core::Result<BarcodeSymbologyDecodeLengthKind> {
        let this = self;
        unsafe {
            let mut result__: BarcodeSymbologyDecodeLengthKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DecodeLengthKind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BarcodeSymbologyDecodeLengthKind>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetDecodeLengthKind(&self, value: BarcodeSymbologyDecodeLengthKind) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDecodeLengthKind)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsDecodeLengthSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDecodeLengthSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for BarcodeSymbologyAttributes {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BarcodeSymbologyAttributes {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BarcodeSymbologyAttributes {}
impl ::core::fmt::Debug for BarcodeSymbologyAttributes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeSymbologyAttributes").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BarcodeSymbologyAttributes {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.BarcodeSymbologyAttributes;{66413a78-ab7a-4ada-8ece-936014b2ead7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for BarcodeSymbologyAttributes {
    type Vtable = IBarcodeSymbologyAttributes_Vtbl;
    const IID: ::windows::core::GUID = <IBarcodeSymbologyAttributes as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BarcodeSymbologyAttributes {
    const NAME: &'static str = "Windows.Devices.PointOfService.BarcodeSymbologyAttributes";
}
impl ::core::convert::From<BarcodeSymbologyAttributes> for ::windows::core::IUnknown {
    fn from(value: BarcodeSymbologyAttributes) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeSymbologyAttributes> for ::windows::core::IUnknown {
    fn from(value: &BarcodeSymbologyAttributes) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BarcodeSymbologyAttributes {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BarcodeSymbologyAttributes {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BarcodeSymbologyAttributes> for ::windows::core::IInspectable {
    fn from(value: BarcodeSymbologyAttributes) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BarcodeSymbologyAttributes> for ::windows::core::IInspectable {
    fn from(value: &BarcodeSymbologyAttributes) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BarcodeSymbologyAttributes {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BarcodeSymbologyAttributes {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BarcodeSymbologyAttributes {}
unsafe impl ::core::marker::Sync for BarcodeSymbologyAttributes {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BarcodeSymbologyDecodeLengthKind(pub i32);
impl BarcodeSymbologyDecodeLengthKind {
    pub const AnyLength: Self = Self(0i32);
    pub const Discrete: Self = Self(1i32);
    pub const Range: Self = Self(2i32);
}
impl ::core::marker::Copy for BarcodeSymbologyDecodeLengthKind {}
impl ::core::clone::Clone for BarcodeSymbologyDecodeLengthKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BarcodeSymbologyDecodeLengthKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BarcodeSymbologyDecodeLengthKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for BarcodeSymbologyDecodeLengthKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BarcodeSymbologyDecodeLengthKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BarcodeSymbologyDecodeLengthKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.BarcodeSymbologyDecodeLengthKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct CashDrawer(::windows::core::IUnknown);
impl CashDrawer {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Capabilities(&self) -> ::windows::core::Result<CashDrawerCapabilities> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Capabilities)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CashDrawerCapabilities>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<CashDrawerStatus> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CashDrawerStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsDrawerOpen(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDrawerOpen)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn DrawerEventSource(&self) -> ::windows::core::Result<CashDrawerEventSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DrawerEventSource)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CashDrawerEventSource>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ClaimDrawerAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ClaimedCashDrawer>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ClaimDrawerAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ClaimedCashDrawer>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CheckHealthAsync(&self, level: UnifiedPosHealthCheckLevel) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CheckHealthAsync)(::core::mem::transmute_copy(this), level, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStatisticsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, statisticscategories: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStatisticsAsync)(::core::mem::transmute_copy(this), statisticscategories.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StatusUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CashDrawer, CashDrawerStatusUpdatedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StatusUpdated)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStatusUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveStatusUpdated)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDefaultAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CashDrawer>> {
        Self::ICashDrawerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDefaultAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<CashDrawer>>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CashDrawer>> {
        Self::ICashDrawerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FromIdAsync)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<CashDrawer>>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICashDrawerStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeviceSelector)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn GetDeviceSelectorWithConnectionTypes(connectiontypes: PosConnectionTypes) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ICashDrawerStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeviceSelectorWithConnectionTypes)(::core::mem::transmute_copy(this), connectiontypes, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc(hidden)]
    pub fn ICashDrawerStatics<R, F: FnOnce(&ICashDrawerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CashDrawer, ICashDrawerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn ICashDrawerStatics2<R, F: FnOnce(&ICashDrawerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CashDrawer, ICashDrawerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for CashDrawer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CashDrawer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CashDrawer {}
impl ::core::fmt::Debug for CashDrawer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CashDrawer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CashDrawer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.CashDrawer;{9f88f5c8-de54-4aee-a890-920bcbfe30fc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CashDrawer {
    type Vtable = ICashDrawer_Vtbl;
    const IID: ::windows::core::GUID = <ICashDrawer as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CashDrawer {
    const NAME: &'static str = "Windows.Devices.PointOfService.CashDrawer";
}
impl ::core::convert::From<CashDrawer> for ::windows::core::IUnknown {
    fn from(value: CashDrawer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CashDrawer> for ::windows::core::IUnknown {
    fn from(value: &CashDrawer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CashDrawer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CashDrawer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CashDrawer> for ::windows::core::IInspectable {
    fn from(value: CashDrawer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CashDrawer> for ::windows::core::IInspectable {
    fn from(value: &CashDrawer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CashDrawer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CashDrawer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct CashDrawerCapabilities(::windows::core::IUnknown);
impl CashDrawerCapabilities {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn PowerReportingType(&self) -> ::windows::core::Result<UnifiedPosPowerReportingType> {
        let this = self;
        unsafe {
            let mut result__: UnifiedPosPowerReportingType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PowerReportingType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UnifiedPosPowerReportingType>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsStatisticsReportingSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsStatisticsReportingSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsStatisticsUpdatingSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsStatisticsUpdatingSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsStatusReportingSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsStatusReportingSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsStatusMultiDrawerDetectSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsStatusMultiDrawerDetectSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsDrawerOpenSensorAvailable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDrawerOpenSensorAvailable)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for CashDrawerCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CashDrawerCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CashDrawerCapabilities {}
impl ::core::fmt::Debug for CashDrawerCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CashDrawerCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CashDrawerCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.CashDrawerCapabilities;{0bc6de0b-e8e7-4b1f-b1d1-3e501ad08247})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CashDrawerCapabilities {
    type Vtable = ICashDrawerCapabilities_Vtbl;
    const IID: ::windows::core::GUID = <ICashDrawerCapabilities as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CashDrawerCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.CashDrawerCapabilities";
}
impl ::core::convert::From<CashDrawerCapabilities> for ::windows::core::IUnknown {
    fn from(value: CashDrawerCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CashDrawerCapabilities> for ::windows::core::IUnknown {
    fn from(value: &CashDrawerCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CashDrawerCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CashDrawerCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CashDrawerCapabilities> for ::windows::core::IInspectable {
    fn from(value: CashDrawerCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CashDrawerCapabilities> for ::windows::core::IInspectable {
    fn from(value: &CashDrawerCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CashDrawerCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CashDrawerCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CashDrawerCapabilities {}
unsafe impl ::core::marker::Sync for CashDrawerCapabilities {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct CashDrawerCloseAlarm(::windows::core::IUnknown);
impl CashDrawerCloseAlarm {
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetAlarmTimeout<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAlarmTimeout)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AlarmTimeout(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AlarmTimeout)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetBeepFrequency(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBeepFrequency)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn BeepFrequency(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BeepFrequency)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetBeepDuration<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBeepDuration)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BeepDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BeepDuration)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetBeepDelay<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBeepDelay)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BeepDelay(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BeepDelay)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AlarmTimeoutExpired<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CashDrawerCloseAlarm, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AlarmTimeoutExpired)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAlarmTimeoutExpired<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAlarmTimeoutExpired)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
impl ::core::clone::Clone for CashDrawerCloseAlarm {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CashDrawerCloseAlarm {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CashDrawerCloseAlarm {}
impl ::core::fmt::Debug for CashDrawerCloseAlarm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CashDrawerCloseAlarm").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CashDrawerCloseAlarm {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.CashDrawerCloseAlarm;{6bf88cc7-6f63-430e-ab3b-95d75ffbe87f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CashDrawerCloseAlarm {
    type Vtable = ICashDrawerCloseAlarm_Vtbl;
    const IID: ::windows::core::GUID = <ICashDrawerCloseAlarm as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CashDrawerCloseAlarm {
    const NAME: &'static str = "Windows.Devices.PointOfService.CashDrawerCloseAlarm";
}
impl ::core::convert::From<CashDrawerCloseAlarm> for ::windows::core::IUnknown {
    fn from(value: CashDrawerCloseAlarm) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CashDrawerCloseAlarm> for ::windows::core::IUnknown {
    fn from(value: &CashDrawerCloseAlarm) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CashDrawerCloseAlarm {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CashDrawerCloseAlarm {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CashDrawerCloseAlarm> for ::windows::core::IInspectable {
    fn from(value: CashDrawerCloseAlarm) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CashDrawerCloseAlarm> for ::windows::core::IInspectable {
    fn from(value: &CashDrawerCloseAlarm) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CashDrawerCloseAlarm {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CashDrawerCloseAlarm {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CashDrawerCloseAlarm {}
unsafe impl ::core::marker::Sync for CashDrawerCloseAlarm {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct CashDrawerClosedEventArgs(::windows::core::IUnknown);
impl CashDrawerClosedEventArgs {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn CashDrawer(&self) -> ::windows::core::Result<CashDrawer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CashDrawer)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CashDrawer>(result__)
        }
    }
}
impl ::core::clone::Clone for CashDrawerClosedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CashDrawerClosedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CashDrawerClosedEventArgs {}
impl ::core::fmt::Debug for CashDrawerClosedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CashDrawerClosedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CashDrawerClosedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.CashDrawerClosedEventArgs;{69cb3bc1-147f-421c-9c23-090123bb786c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CashDrawerClosedEventArgs {
    type Vtable = ICashDrawerEventSourceEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ICashDrawerEventSourceEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CashDrawerClosedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.CashDrawerClosedEventArgs";
}
impl ::core::convert::From<CashDrawerClosedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CashDrawerClosedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CashDrawerClosedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CashDrawerClosedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CashDrawerClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CashDrawerClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CashDrawerClosedEventArgs> for ::windows::core::IInspectable {
    fn from(value: CashDrawerClosedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CashDrawerClosedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CashDrawerClosedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CashDrawerClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CashDrawerClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<CashDrawerClosedEventArgs> for ICashDrawerEventSourceEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: CashDrawerClosedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CashDrawerClosedEventArgs> for ICashDrawerEventSourceEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &CashDrawerClosedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICashDrawerEventSourceEventArgs> for CashDrawerClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ICashDrawerEventSourceEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICashDrawerEventSourceEventArgs> for &CashDrawerClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ICashDrawerEventSourceEventArgs> {
        ::core::convert::TryInto::<ICashDrawerEventSourceEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for CashDrawerClosedEventArgs {}
unsafe impl ::core::marker::Sync for CashDrawerClosedEventArgs {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct CashDrawerEventSource(::windows::core::IUnknown);
impl CashDrawerEventSource {
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DrawerClosed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CashDrawerEventSource, CashDrawerClosedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DrawerClosed)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDrawerClosed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveDrawerClosed)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DrawerOpened<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CashDrawerEventSource, CashDrawerOpenedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DrawerOpened)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDrawerOpened<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveDrawerOpened)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for CashDrawerEventSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CashDrawerEventSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CashDrawerEventSource {}
impl ::core::fmt::Debug for CashDrawerEventSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CashDrawerEventSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CashDrawerEventSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.CashDrawerEventSource;{e006e46c-f2f9-442f-8dd6-06c10a4227ba})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CashDrawerEventSource {
    type Vtable = ICashDrawerEventSource_Vtbl;
    const IID: ::windows::core::GUID = <ICashDrawerEventSource as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CashDrawerEventSource {
    const NAME: &'static str = "Windows.Devices.PointOfService.CashDrawerEventSource";
}
impl ::core::convert::From<CashDrawerEventSource> for ::windows::core::IUnknown {
    fn from(value: CashDrawerEventSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CashDrawerEventSource> for ::windows::core::IUnknown {
    fn from(value: &CashDrawerEventSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CashDrawerEventSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CashDrawerEventSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CashDrawerEventSource> for ::windows::core::IInspectable {
    fn from(value: CashDrawerEventSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CashDrawerEventSource> for ::windows::core::IInspectable {
    fn from(value: &CashDrawerEventSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CashDrawerEventSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CashDrawerEventSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CashDrawerEventSource {}
unsafe impl ::core::marker::Sync for CashDrawerEventSource {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct CashDrawerOpenedEventArgs(::windows::core::IUnknown);
impl CashDrawerOpenedEventArgs {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn CashDrawer(&self) -> ::windows::core::Result<CashDrawer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CashDrawer)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CashDrawer>(result__)
        }
    }
}
impl ::core::clone::Clone for CashDrawerOpenedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CashDrawerOpenedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CashDrawerOpenedEventArgs {}
impl ::core::fmt::Debug for CashDrawerOpenedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CashDrawerOpenedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CashDrawerOpenedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.CashDrawerOpenedEventArgs;{69cb3bc1-147f-421c-9c23-090123bb786c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CashDrawerOpenedEventArgs {
    type Vtable = ICashDrawerEventSourceEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ICashDrawerEventSourceEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CashDrawerOpenedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.CashDrawerOpenedEventArgs";
}
impl ::core::convert::From<CashDrawerOpenedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CashDrawerOpenedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CashDrawerOpenedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CashDrawerOpenedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CashDrawerOpenedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CashDrawerOpenedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CashDrawerOpenedEventArgs> for ::windows::core::IInspectable {
    fn from(value: CashDrawerOpenedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CashDrawerOpenedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CashDrawerOpenedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CashDrawerOpenedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CashDrawerOpenedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<CashDrawerOpenedEventArgs> for ICashDrawerEventSourceEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: CashDrawerOpenedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&CashDrawerOpenedEventArgs> for ICashDrawerEventSourceEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &CashDrawerOpenedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICashDrawerEventSourceEventArgs> for CashDrawerOpenedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ICashDrawerEventSourceEventArgs> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ICashDrawerEventSourceEventArgs> for &CashDrawerOpenedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ICashDrawerEventSourceEventArgs> {
        ::core::convert::TryInto::<ICashDrawerEventSourceEventArgs>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for CashDrawerOpenedEventArgs {}
unsafe impl ::core::marker::Sync for CashDrawerOpenedEventArgs {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct CashDrawerStatus(::windows::core::IUnknown);
impl CashDrawerStatus {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn StatusKind(&self) -> ::windows::core::Result<CashDrawerStatusKind> {
        let this = self;
        unsafe {
            let mut result__: CashDrawerStatusKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StatusKind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CashDrawerStatusKind>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn ExtendedStatus(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedStatus)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for CashDrawerStatus {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CashDrawerStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CashDrawerStatus {}
impl ::core::fmt::Debug for CashDrawerStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CashDrawerStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CashDrawerStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.CashDrawerStatus;{6bbd78bf-dca1-4e06-99eb-5af6a5aec108})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CashDrawerStatus {
    type Vtable = ICashDrawerStatus_Vtbl;
    const IID: ::windows::core::GUID = <ICashDrawerStatus as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CashDrawerStatus {
    const NAME: &'static str = "Windows.Devices.PointOfService.CashDrawerStatus";
}
impl ::core::convert::From<CashDrawerStatus> for ::windows::core::IUnknown {
    fn from(value: CashDrawerStatus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CashDrawerStatus> for ::windows::core::IUnknown {
    fn from(value: &CashDrawerStatus) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CashDrawerStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CashDrawerStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CashDrawerStatus> for ::windows::core::IInspectable {
    fn from(value: CashDrawerStatus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CashDrawerStatus> for ::windows::core::IInspectable {
    fn from(value: &CashDrawerStatus) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CashDrawerStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CashDrawerStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CashDrawerStatus {}
unsafe impl ::core::marker::Sync for CashDrawerStatus {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CashDrawerStatusKind(pub i32);
impl CashDrawerStatusKind {
    pub const Online: Self = Self(0i32);
    pub const Off: Self = Self(1i32);
    pub const Offline: Self = Self(2i32);
    pub const OffOrOffline: Self = Self(3i32);
    pub const Extended: Self = Self(4i32);
}
impl ::core::marker::Copy for CashDrawerStatusKind {}
impl ::core::clone::Clone for CashDrawerStatusKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CashDrawerStatusKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CashDrawerStatusKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for CashDrawerStatusKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CashDrawerStatusKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CashDrawerStatusKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.CashDrawerStatusKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct CashDrawerStatusUpdatedEventArgs(::windows::core::IUnknown);
impl CashDrawerStatusUpdatedEventArgs {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<CashDrawerStatus> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CashDrawerStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for CashDrawerStatusUpdatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CashDrawerStatusUpdatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CashDrawerStatusUpdatedEventArgs {}
impl ::core::fmt::Debug for CashDrawerStatusUpdatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CashDrawerStatusUpdatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CashDrawerStatusUpdatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.CashDrawerStatusUpdatedEventArgs;{30aae98a-0d70-459c-9553-87e124c52488})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CashDrawerStatusUpdatedEventArgs {
    type Vtable = ICashDrawerStatusUpdatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ICashDrawerStatusUpdatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CashDrawerStatusUpdatedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.CashDrawerStatusUpdatedEventArgs";
}
impl ::core::convert::From<CashDrawerStatusUpdatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: CashDrawerStatusUpdatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CashDrawerStatusUpdatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CashDrawerStatusUpdatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CashDrawerStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CashDrawerStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CashDrawerStatusUpdatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: CashDrawerStatusUpdatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CashDrawerStatusUpdatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CashDrawerStatusUpdatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CashDrawerStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CashDrawerStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CashDrawerStatusUpdatedEventArgs {}
unsafe impl ::core::marker::Sync for CashDrawerStatusUpdatedEventArgs {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct ClaimedBarcodeScanner(::windows::core::IUnknown);
impl ClaimedBarcodeScanner {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetIsDisabledOnDataReceived(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsDisabledOnDataReceived)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsDisabledOnDataReceived(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDisabledOnDataReceived)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetIsDecodeDataEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsDecodeDataEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsDecodeDataEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDecodeDataEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EnableAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EnableAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DisableAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisableAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn RetainDevice(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RetainDevice)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetActiveSymbologiesAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<u32>>>(&self, symbologies: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SetActiveSymbologiesAsync)(::core::mem::transmute_copy(this), symbologies.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ResetStatisticsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, statisticscategories: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResetStatisticsAsync)(::core::mem::transmute_copy(this), statisticscategories.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn UpdateStatisticsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>>(&self, statistics: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UpdateStatisticsAsync)(::core::mem::transmute_copy(this), statistics.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetActiveProfileAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, profile: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SetActiveProfileAsync)(::core::mem::transmute_copy(this), profile.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DataReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ClaimedBarcodeScanner, BarcodeScannerDataReceivedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DataReceived)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDataReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveDataReceived)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TriggerPressed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<ClaimedBarcodeScanner>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TriggerPressed)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveTriggerPressed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveTriggerPressed)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TriggerReleased<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<ClaimedBarcodeScanner>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TriggerReleased)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveTriggerReleased<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveTriggerReleased)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReleaseDeviceRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<ClaimedBarcodeScanner>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReleaseDeviceRequested)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveReleaseDeviceRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveReleaseDeviceRequested)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ImagePreviewReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ClaimedBarcodeScanner, BarcodeScannerImagePreviewReceivedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ImagePreviewReceived)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveImagePreviewReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveImagePreviewReceived)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ErrorOccurred<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ClaimedBarcodeScanner, BarcodeScannerErrorOccurredEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ErrorOccurred)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveErrorOccurred<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveErrorOccurred)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartSoftwareTriggerAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IClaimedBarcodeScanner1>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StartSoftwareTriggerAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StopSoftwareTriggerAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IClaimedBarcodeScanner1>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StopSoftwareTriggerAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetSymbologyAttributesAsync(&self, barcodesymbology: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BarcodeSymbologyAttributes>> {
        let this = &::windows::core::Interface::cast::<IClaimedBarcodeScanner2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetSymbologyAttributesAsync)(::core::mem::transmute_copy(this), barcodesymbology, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BarcodeSymbologyAttributes>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSymbologyAttributesAsync<'a, Param1: ::windows::core::IntoParam<'a, BarcodeSymbologyAttributes>>(&self, barcodesymbology: u32, attributes: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IClaimedBarcodeScanner2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SetSymbologyAttributesAsync)(::core::mem::transmute_copy(this), barcodesymbology, attributes.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShowVideoPreviewAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IClaimedBarcodeScanner3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowVideoPreviewAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn HideVideoPreview(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IClaimedBarcodeScanner3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).HideVideoPreview)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetIsVideoPreviewShownOnEnable(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IClaimedBarcodeScanner3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetIsVideoPreviewShownOnEnable)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsVideoPreviewShownOnEnable(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IClaimedBarcodeScanner3>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsVideoPreviewShownOnEnable)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Closed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ClaimedBarcodeScanner, ClaimedBarcodeScannerClosedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IClaimedBarcodeScanner4>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Closed)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IClaimedBarcodeScanner4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveClosed)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for ClaimedBarcodeScanner {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ClaimedBarcodeScanner {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ClaimedBarcodeScanner {}
impl ::core::fmt::Debug for ClaimedBarcodeScanner {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClaimedBarcodeScanner").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ClaimedBarcodeScanner {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedBarcodeScanner;{4a63b49c-8fa4-4332-bb26-945d11d81e0f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ClaimedBarcodeScanner {
    type Vtable = IClaimedBarcodeScanner_Vtbl;
    const IID: ::windows::core::GUID = <IClaimedBarcodeScanner as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ClaimedBarcodeScanner {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedBarcodeScanner";
}
impl ::core::convert::From<ClaimedBarcodeScanner> for ::windows::core::IUnknown {
    fn from(value: ClaimedBarcodeScanner) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ClaimedBarcodeScanner> for ::windows::core::IUnknown {
    fn from(value: &ClaimedBarcodeScanner) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ClaimedBarcodeScanner {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ClaimedBarcodeScanner {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ClaimedBarcodeScanner> for ::windows::core::IInspectable {
    fn from(value: ClaimedBarcodeScanner) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ClaimedBarcodeScanner> for ::windows::core::IInspectable {
    fn from(value: &ClaimedBarcodeScanner) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ClaimedBarcodeScanner {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ClaimedBarcodeScanner {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct ClaimedBarcodeScannerClosedEventArgs(::windows::core::IUnknown);
impl ClaimedBarcodeScannerClosedEventArgs {}
impl ::core::clone::Clone for ClaimedBarcodeScannerClosedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ClaimedBarcodeScannerClosedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ClaimedBarcodeScannerClosedEventArgs {}
impl ::core::fmt::Debug for ClaimedBarcodeScannerClosedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClaimedBarcodeScannerClosedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ClaimedBarcodeScannerClosedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedBarcodeScannerClosedEventArgs;{cf7d5489-a22c-4c65-a901-88d77d833954})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ClaimedBarcodeScannerClosedEventArgs {
    type Vtable = IClaimedBarcodeScannerClosedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IClaimedBarcodeScannerClosedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ClaimedBarcodeScannerClosedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedBarcodeScannerClosedEventArgs";
}
impl ::core::convert::From<ClaimedBarcodeScannerClosedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ClaimedBarcodeScannerClosedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ClaimedBarcodeScannerClosedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ClaimedBarcodeScannerClosedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ClaimedBarcodeScannerClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ClaimedBarcodeScannerClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ClaimedBarcodeScannerClosedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ClaimedBarcodeScannerClosedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ClaimedBarcodeScannerClosedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ClaimedBarcodeScannerClosedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ClaimedBarcodeScannerClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ClaimedBarcodeScannerClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ClaimedBarcodeScannerClosedEventArgs {}
unsafe impl ::core::marker::Sync for ClaimedBarcodeScannerClosedEventArgs {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct ClaimedCashDrawer(::windows::core::IUnknown);
impl ClaimedCashDrawer {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsDrawerOpen(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDrawerOpen)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn CloseAlarm(&self) -> ::windows::core::Result<CashDrawerCloseAlarm> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CloseAlarm)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CashDrawerCloseAlarm>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OpenDrawerAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OpenDrawerAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EnableAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EnableAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DisableAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisableAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RetainDeviceAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RetainDeviceAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ResetStatisticsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, statisticscategories: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResetStatisticsAsync)(::core::mem::transmute_copy(this), statisticscategories.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn UpdateStatisticsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>>(&self, statistics: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UpdateStatisticsAsync)(::core::mem::transmute_copy(this), statistics.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReleaseDeviceRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ClaimedCashDrawer, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReleaseDeviceRequested)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveReleaseDeviceRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveReleaseDeviceRequested)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Closed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ClaimedCashDrawer, ClaimedCashDrawerClosedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IClaimedCashDrawer2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Closed)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IClaimedCashDrawer2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveClosed)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for ClaimedCashDrawer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ClaimedCashDrawer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ClaimedCashDrawer {}
impl ::core::fmt::Debug for ClaimedCashDrawer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClaimedCashDrawer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ClaimedCashDrawer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedCashDrawer;{ca3f99af-abb8-42c1-8a84-5c66512f5a75})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ClaimedCashDrawer {
    type Vtable = IClaimedCashDrawer_Vtbl;
    const IID: ::windows::core::GUID = <IClaimedCashDrawer as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ClaimedCashDrawer {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedCashDrawer";
}
impl ::core::convert::From<ClaimedCashDrawer> for ::windows::core::IUnknown {
    fn from(value: ClaimedCashDrawer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ClaimedCashDrawer> for ::windows::core::IUnknown {
    fn from(value: &ClaimedCashDrawer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ClaimedCashDrawer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ClaimedCashDrawer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ClaimedCashDrawer> for ::windows::core::IInspectable {
    fn from(value: ClaimedCashDrawer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ClaimedCashDrawer> for ::windows::core::IInspectable {
    fn from(value: &ClaimedCashDrawer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ClaimedCashDrawer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ClaimedCashDrawer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct ClaimedCashDrawerClosedEventArgs(::windows::core::IUnknown);
impl ClaimedCashDrawerClosedEventArgs {}
impl ::core::clone::Clone for ClaimedCashDrawerClosedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ClaimedCashDrawerClosedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ClaimedCashDrawerClosedEventArgs {}
impl ::core::fmt::Debug for ClaimedCashDrawerClosedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClaimedCashDrawerClosedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ClaimedCashDrawerClosedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedCashDrawerClosedEventArgs;{cc573f33-3f34-4c5c-baae-deadf16cd7fa})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ClaimedCashDrawerClosedEventArgs {
    type Vtable = IClaimedCashDrawerClosedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IClaimedCashDrawerClosedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ClaimedCashDrawerClosedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedCashDrawerClosedEventArgs";
}
impl ::core::convert::From<ClaimedCashDrawerClosedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ClaimedCashDrawerClosedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ClaimedCashDrawerClosedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ClaimedCashDrawerClosedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ClaimedCashDrawerClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ClaimedCashDrawerClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ClaimedCashDrawerClosedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ClaimedCashDrawerClosedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ClaimedCashDrawerClosedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ClaimedCashDrawerClosedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ClaimedCashDrawerClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ClaimedCashDrawerClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ClaimedCashDrawerClosedEventArgs {}
unsafe impl ::core::marker::Sync for ClaimedCashDrawerClosedEventArgs {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct ClaimedJournalPrinter(::windows::core::IUnknown);
impl ClaimedJournalPrinter {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn CreateJob(&self) -> ::windows::core::Result<JournalPrintJob> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateJob)(::core::mem::transmute_copy(this), &mut result__).from_abi::<JournalPrintJob>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetCharactersPerLine(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetCharactersPerLine)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn CharactersPerLine(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CharactersPerLine)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetLineHeight(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetLineHeight)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn LineHeight(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LineHeight)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetLineSpacing(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetLineSpacing)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn LineSpacing(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LineSpacing)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn LineWidth(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LineWidth)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetIsLetterQuality(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetIsLetterQuality)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsLetterQuality(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsLetterQuality)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsPaperNearEnd(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsPaperNearEnd)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetColorCartridge(&self, value: PosPrinterColorCartridge) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetColorCartridge)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn ColorCartridge(&self) -> ::windows::core::Result<PosPrinterColorCartridge> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: PosPrinterColorCartridge = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ColorCartridge)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterColorCartridge>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsCoverOpen(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCoverOpen)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsCartridgeRemoved(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCartridgeRemoved)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsCartridgeEmpty(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCartridgeEmpty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsHeadCleaning(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsHeadCleaning)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsPaperEmpty(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsPaperEmpty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsReadyToPrint(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsReadyToPrint)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn ValidateData<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, data: Param0) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ValidateData)(::core::mem::transmute_copy(this), data.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for ClaimedJournalPrinter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ClaimedJournalPrinter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ClaimedJournalPrinter {}
impl ::core::fmt::Debug for ClaimedJournalPrinter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClaimedJournalPrinter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ClaimedJournalPrinter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedJournalPrinter;{67ea0630-517d-487f-9fdf-d2e0a0a264a5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ClaimedJournalPrinter {
    type Vtable = IClaimedJournalPrinter_Vtbl;
    const IID: ::windows::core::GUID = <IClaimedJournalPrinter as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ClaimedJournalPrinter {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedJournalPrinter";
}
impl ::core::convert::From<ClaimedJournalPrinter> for ::windows::core::IUnknown {
    fn from(value: ClaimedJournalPrinter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ClaimedJournalPrinter> for ::windows::core::IUnknown {
    fn from(value: &ClaimedJournalPrinter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ClaimedJournalPrinter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ClaimedJournalPrinter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ClaimedJournalPrinter> for ::windows::core::IInspectable {
    fn from(value: ClaimedJournalPrinter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ClaimedJournalPrinter> for ::windows::core::IInspectable {
    fn from(value: &ClaimedJournalPrinter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ClaimedJournalPrinter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ClaimedJournalPrinter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct ClaimedLineDisplay(::windows::core::IUnknown);
impl ClaimedLineDisplay {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Capabilities(&self) -> ::windows::core::Result<LineDisplayCapabilities> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Capabilities)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LineDisplayCapabilities>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn PhysicalDeviceName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PhysicalDeviceName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn PhysicalDeviceDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PhysicalDeviceDescription)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn DeviceControlDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceControlDescription)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn DeviceControlVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceControlVersion)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn DeviceServiceVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceServiceVersion)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn DefaultWindow(&self) -> ::windows::core::Result<LineDisplayWindow> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DefaultWindow)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LineDisplayWindow>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn RetainDevice(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RetainDevice)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReleaseDeviceRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ClaimedLineDisplay, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReleaseDeviceRequested)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveReleaseDeviceRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveReleaseDeviceRequested)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStatisticsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, statisticscategories: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStatisticsAsync)(::core::mem::transmute_copy(this), statisticscategories.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CheckHealthAsync(&self, level: UnifiedPosHealthCheckLevel) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CheckHealthAsync)(::core::mem::transmute_copy(this), level, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CheckPowerStatusAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LineDisplayPowerStatus>> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CheckPowerStatusAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LineDisplayPowerStatus>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StatusUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ClaimedLineDisplay, LineDisplayStatusUpdatedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StatusUpdated)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStatusUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveStatusUpdated)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedScreenSizesInCharacters(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Foundation::Size>> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SupportedScreenSizesInCharacters)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Foundation::Size>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MaxBitmapSizeInPixels(&self) -> ::windows::core::Result<super::super::Foundation::Size> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::Size = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MaxBitmapSizeInPixels)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedCharacterSets(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i32>> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SupportedCharacterSets)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<i32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn CustomGlyphs(&self) -> ::windows::core::Result<LineDisplayCustomGlyphs> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CustomGlyphs)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LineDisplayCustomGlyphs>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn GetAttributes(&self) -> ::windows::core::Result<LineDisplayAttributes> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAttributes)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LineDisplayAttributes>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryUpdateAttributesAsync<'a, Param0: ::windows::core::IntoParam<'a, LineDisplayAttributes>>(&self, attributes: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryUpdateAttributesAsync)(::core::mem::transmute_copy(this), attributes.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TrySetDescriptorAsync(&self, descriptor: u32, descriptorstate: LineDisplayDescriptorState) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TrySetDescriptorAsync)(::core::mem::transmute_copy(this), descriptor, descriptorstate, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryClearDescriptorsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryClearDescriptorsAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryCreateWindowAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Size>>(&self, viewport: Param0, windowsize: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LineDisplayWindow>> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryCreateWindowAsync)(::core::mem::transmute_copy(this), viewport.into_param().abi(), windowsize.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LineDisplayWindow>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn TryStoreStorageFileBitmapAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::StorageFile>>(&self, bitmap: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LineDisplayStoredBitmap>> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryStoreStorageFileBitmapAsync)(::core::mem::transmute_copy(this), bitmap.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LineDisplayStoredBitmap>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn TryStoreStorageFileBitmapWithAlignmentAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::StorageFile>>(&self, bitmap: Param0, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LineDisplayStoredBitmap>> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryStoreStorageFileBitmapWithAlignmentAsync)(::core::mem::transmute_copy(this), bitmap.into_param().abi(), horizontalalignment, verticalalignment, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LineDisplayStoredBitmap>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn TryStoreStorageFileBitmapWithAlignmentAndWidthAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::StorageFile>>(&self, bitmap: Param0, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment, widthinpixels: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LineDisplayStoredBitmap>> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryStoreStorageFileBitmapWithAlignmentAndWidthAsync)(::core::mem::transmute_copy(this), bitmap.into_param().abi(), horizontalalignment, verticalalignment, widthinpixels, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LineDisplayStoredBitmap>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Closed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ClaimedLineDisplay, ClaimedLineDisplayClosedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay3>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Closed)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IClaimedLineDisplay3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveClosed)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ClaimedLineDisplay>> {
        Self::IClaimedLineDisplayStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FromIdAsync)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ClaimedLineDisplay>>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IClaimedLineDisplayStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeviceSelector)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn GetDeviceSelectorWithConnectionTypes(connectiontypes: PosConnectionTypes) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IClaimedLineDisplayStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeviceSelectorWithConnectionTypes)(::core::mem::transmute_copy(this), connectiontypes, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc(hidden)]
    pub fn IClaimedLineDisplayStatics<R, F: FnOnce(&IClaimedLineDisplayStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ClaimedLineDisplay, IClaimedLineDisplayStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ClaimedLineDisplay {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ClaimedLineDisplay {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ClaimedLineDisplay {}
impl ::core::fmt::Debug for ClaimedLineDisplay {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClaimedLineDisplay").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ClaimedLineDisplay {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedLineDisplay;{120ac970-9a75-4acf-aae7-09972bcf8794})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ClaimedLineDisplay {
    type Vtable = IClaimedLineDisplay_Vtbl;
    const IID: ::windows::core::GUID = <IClaimedLineDisplay as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ClaimedLineDisplay {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedLineDisplay";
}
impl ::core::convert::From<ClaimedLineDisplay> for ::windows::core::IUnknown {
    fn from(value: ClaimedLineDisplay) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ClaimedLineDisplay> for ::windows::core::IUnknown {
    fn from(value: &ClaimedLineDisplay) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ClaimedLineDisplay {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ClaimedLineDisplay {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ClaimedLineDisplay> for ::windows::core::IInspectable {
    fn from(value: ClaimedLineDisplay) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ClaimedLineDisplay> for ::windows::core::IInspectable {
    fn from(value: &ClaimedLineDisplay) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ClaimedLineDisplay {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ClaimedLineDisplay {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct ClaimedLineDisplayClosedEventArgs(::windows::core::IUnknown);
impl ClaimedLineDisplayClosedEventArgs {}
impl ::core::clone::Clone for ClaimedLineDisplayClosedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ClaimedLineDisplayClosedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ClaimedLineDisplayClosedEventArgs {}
impl ::core::fmt::Debug for ClaimedLineDisplayClosedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClaimedLineDisplayClosedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ClaimedLineDisplayClosedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedLineDisplayClosedEventArgs;{f915f364-d3d5-4f10-b511-90939edfacd8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ClaimedLineDisplayClosedEventArgs {
    type Vtable = IClaimedLineDisplayClosedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IClaimedLineDisplayClosedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ClaimedLineDisplayClosedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedLineDisplayClosedEventArgs";
}
impl ::core::convert::From<ClaimedLineDisplayClosedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ClaimedLineDisplayClosedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ClaimedLineDisplayClosedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ClaimedLineDisplayClosedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ClaimedLineDisplayClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ClaimedLineDisplayClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ClaimedLineDisplayClosedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ClaimedLineDisplayClosedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ClaimedLineDisplayClosedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ClaimedLineDisplayClosedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ClaimedLineDisplayClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ClaimedLineDisplayClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ClaimedLineDisplayClosedEventArgs {}
unsafe impl ::core::marker::Sync for ClaimedLineDisplayClosedEventArgs {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct ClaimedMagneticStripeReader(::windows::core::IUnknown);
impl ClaimedMagneticStripeReader {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetIsDisabledOnDataReceived(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsDisabledOnDataReceived)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsDisabledOnDataReceived(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDisabledOnDataReceived)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetIsDecodeDataEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsDecodeDataEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsDecodeDataEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDecodeDataEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsDeviceAuthenticated(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDeviceAuthenticated)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetDataEncryptionAlgorithm(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDataEncryptionAlgorithm)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn DataEncryptionAlgorithm(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DataEncryptionAlgorithm)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetTracksToRead(&self, value: MagneticStripeReaderTrackIds) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTracksToRead)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn TracksToRead(&self) -> ::windows::core::Result<MagneticStripeReaderTrackIds> {
        let this = self;
        unsafe {
            let mut result__: MagneticStripeReaderTrackIds = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TracksToRead)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MagneticStripeReaderTrackIds>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetIsTransmitSentinelsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsTransmitSentinelsEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsTransmitSentinelsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsTransmitSentinelsEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EnableAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EnableAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DisableAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisableAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn RetainDevice(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RetainDevice)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetErrorReportingType(&self, value: MagneticStripeReaderErrorReportingType) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetErrorReportingType)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn RetrieveDeviceAuthenticationDataAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RetrieveDeviceAuthenticationDataAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AuthenticateDeviceAsync(&self, responsetoken: &[u8]) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AuthenticateDeviceAsync)(::core::mem::transmute_copy(this), responsetoken.len() as u32, ::core::mem::transmute(responsetoken.as_ptr()), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeAuthenticateDeviceAsync(&self, responsetoken: &[u8]) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeAuthenticateDeviceAsync)(::core::mem::transmute_copy(this), responsetoken.len() as u32, ::core::mem::transmute(responsetoken.as_ptr()), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateKeyAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, key: Param0, keyname: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UpdateKeyAsync)(::core::mem::transmute_copy(this), key.into_param().abi(), keyname.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ResetStatisticsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, statisticscategories: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResetStatisticsAsync)(::core::mem::transmute_copy(this), statisticscategories.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn UpdateStatisticsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>>(&self, statistics: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UpdateStatisticsAsync)(::core::mem::transmute_copy(this), statistics.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BankCardDataReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, MagneticStripeReaderBankCardDataReceivedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BankCardDataReceived)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveBankCardDataReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveBankCardDataReceived)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AamvaCardDataReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, MagneticStripeReaderAamvaCardDataReceivedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AamvaCardDataReceived)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAamvaCardDataReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAamvaCardDataReceived)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn VendorSpecificDataReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VendorSpecificDataReceived)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveVendorSpecificDataReceived<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveVendorSpecificDataReceived)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReleaseDeviceRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventHandler<ClaimedMagneticStripeReader>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReleaseDeviceRequested)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveReleaseDeviceRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveReleaseDeviceRequested)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ErrorOccurred<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, MagneticStripeReaderErrorOccurredEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ErrorOccurred)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveErrorOccurred<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveErrorOccurred)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Closed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, ClaimedMagneticStripeReaderClosedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IClaimedMagneticStripeReader2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Closed)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IClaimedMagneticStripeReader2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveClosed)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for ClaimedMagneticStripeReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ClaimedMagneticStripeReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ClaimedMagneticStripeReader {}
impl ::core::fmt::Debug for ClaimedMagneticStripeReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClaimedMagneticStripeReader").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ClaimedMagneticStripeReader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedMagneticStripeReader;{475ca8f3-9417-48bc-b9d7-4163a7844c02})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ClaimedMagneticStripeReader {
    type Vtable = IClaimedMagneticStripeReader_Vtbl;
    const IID: ::windows::core::GUID = <IClaimedMagneticStripeReader as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ClaimedMagneticStripeReader {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedMagneticStripeReader";
}
impl ::core::convert::From<ClaimedMagneticStripeReader> for ::windows::core::IUnknown {
    fn from(value: ClaimedMagneticStripeReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ClaimedMagneticStripeReader> for ::windows::core::IUnknown {
    fn from(value: &ClaimedMagneticStripeReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ClaimedMagneticStripeReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ClaimedMagneticStripeReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ClaimedMagneticStripeReader> for ::windows::core::IInspectable {
    fn from(value: ClaimedMagneticStripeReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ClaimedMagneticStripeReader> for ::windows::core::IInspectable {
    fn from(value: &ClaimedMagneticStripeReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ClaimedMagneticStripeReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ClaimedMagneticStripeReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct ClaimedMagneticStripeReaderClosedEventArgs(::windows::core::IUnknown);
impl ClaimedMagneticStripeReaderClosedEventArgs {}
impl ::core::clone::Clone for ClaimedMagneticStripeReaderClosedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ClaimedMagneticStripeReaderClosedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ClaimedMagneticStripeReaderClosedEventArgs {}
impl ::core::fmt::Debug for ClaimedMagneticStripeReaderClosedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClaimedMagneticStripeReaderClosedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ClaimedMagneticStripeReaderClosedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedMagneticStripeReaderClosedEventArgs;{14ada93a-adcd-4c80-acda-c3eaed2647e1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ClaimedMagneticStripeReaderClosedEventArgs {
    type Vtable = IClaimedMagneticStripeReaderClosedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IClaimedMagneticStripeReaderClosedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ClaimedMagneticStripeReaderClosedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedMagneticStripeReaderClosedEventArgs";
}
impl ::core::convert::From<ClaimedMagneticStripeReaderClosedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ClaimedMagneticStripeReaderClosedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ClaimedMagneticStripeReaderClosedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ClaimedMagneticStripeReaderClosedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ClaimedMagneticStripeReaderClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ClaimedMagneticStripeReaderClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ClaimedMagneticStripeReaderClosedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ClaimedMagneticStripeReaderClosedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ClaimedMagneticStripeReaderClosedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ClaimedMagneticStripeReaderClosedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ClaimedMagneticStripeReaderClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ClaimedMagneticStripeReaderClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ClaimedMagneticStripeReaderClosedEventArgs {}
unsafe impl ::core::marker::Sync for ClaimedMagneticStripeReaderClosedEventArgs {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct ClaimedPosPrinter(::windows::core::IUnknown);
impl ClaimedPosPrinter {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetCharacterSet(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCharacterSet)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn CharacterSet(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CharacterSet)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsCoverOpen(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCoverOpen)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetIsCharacterSetMappingEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsCharacterSetMappingEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsCharacterSetMappingEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCharacterSetMappingEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetMapMode(&self, value: PosPrinterMapMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMapMode)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn MapMode(&self) -> ::windows::core::Result<PosPrinterMapMode> {
        let this = self;
        unsafe {
            let mut result__: PosPrinterMapMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MapMode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterMapMode>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Receipt(&self) -> ::windows::core::Result<ClaimedReceiptPrinter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Receipt)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ClaimedReceiptPrinter>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Slip(&self) -> ::windows::core::Result<ClaimedSlipPrinter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Slip)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ClaimedSlipPrinter>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Journal(&self) -> ::windows::core::Result<ClaimedJournalPrinter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Journal)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ClaimedJournalPrinter>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn EnableAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EnableAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DisableAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisableAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RetainDeviceAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RetainDeviceAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ResetStatisticsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, statisticscategories: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ResetStatisticsAsync)(::core::mem::transmute_copy(this), statisticscategories.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn UpdateStatisticsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>>(&self, statistics: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UpdateStatisticsAsync)(::core::mem::transmute_copy(this), statistics.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReleaseDeviceRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ClaimedPosPrinter, PosPrinterReleaseDeviceRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReleaseDeviceRequested)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveReleaseDeviceRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveReleaseDeviceRequested)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Closed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<ClaimedPosPrinter, ClaimedPosPrinterClosedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IClaimedPosPrinter2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Closed)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IClaimedPosPrinter2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveClosed)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for ClaimedPosPrinter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ClaimedPosPrinter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ClaimedPosPrinter {}
impl ::core::fmt::Debug for ClaimedPosPrinter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClaimedPosPrinter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ClaimedPosPrinter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedPosPrinter;{6d64ce0c-e03e-4b14-a38e-c28c34b86353})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ClaimedPosPrinter {
    type Vtable = IClaimedPosPrinter_Vtbl;
    const IID: ::windows::core::GUID = <IClaimedPosPrinter as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ClaimedPosPrinter {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedPosPrinter";
}
impl ::core::convert::From<ClaimedPosPrinter> for ::windows::core::IUnknown {
    fn from(value: ClaimedPosPrinter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ClaimedPosPrinter> for ::windows::core::IUnknown {
    fn from(value: &ClaimedPosPrinter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ClaimedPosPrinter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ClaimedPosPrinter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ClaimedPosPrinter> for ::windows::core::IInspectable {
    fn from(value: ClaimedPosPrinter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ClaimedPosPrinter> for ::windows::core::IInspectable {
    fn from(value: &ClaimedPosPrinter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ClaimedPosPrinter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ClaimedPosPrinter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct ClaimedPosPrinterClosedEventArgs(::windows::core::IUnknown);
impl ClaimedPosPrinterClosedEventArgs {}
impl ::core::clone::Clone for ClaimedPosPrinterClosedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ClaimedPosPrinterClosedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ClaimedPosPrinterClosedEventArgs {}
impl ::core::fmt::Debug for ClaimedPosPrinterClosedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClaimedPosPrinterClosedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ClaimedPosPrinterClosedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedPosPrinterClosedEventArgs;{e2b7a27b-4d40-471d-92ed-63375b18c788})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ClaimedPosPrinterClosedEventArgs {
    type Vtable = IClaimedPosPrinterClosedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IClaimedPosPrinterClosedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ClaimedPosPrinterClosedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedPosPrinterClosedEventArgs";
}
impl ::core::convert::From<ClaimedPosPrinterClosedEventArgs> for ::windows::core::IUnknown {
    fn from(value: ClaimedPosPrinterClosedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ClaimedPosPrinterClosedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ClaimedPosPrinterClosedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ClaimedPosPrinterClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ClaimedPosPrinterClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ClaimedPosPrinterClosedEventArgs> for ::windows::core::IInspectable {
    fn from(value: ClaimedPosPrinterClosedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ClaimedPosPrinterClosedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ClaimedPosPrinterClosedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ClaimedPosPrinterClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ClaimedPosPrinterClosedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ClaimedPosPrinterClosedEventArgs {}
unsafe impl ::core::marker::Sync for ClaimedPosPrinterClosedEventArgs {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct ClaimedReceiptPrinter(::windows::core::IUnknown);
impl ClaimedReceiptPrinter {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SidewaysMaxLines(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SidewaysMaxLines)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SidewaysMaxChars(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SidewaysMaxChars)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn LinesToPaperCut(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LinesToPaperCut)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PageSize(&self) -> ::windows::core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Size = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PageSize)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PrintArea(&self) -> ::windows::core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PrintArea)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn CreateJob(&self) -> ::windows::core::Result<ReceiptPrintJob> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateJob)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ReceiptPrintJob>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetCharactersPerLine(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetCharactersPerLine)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn CharactersPerLine(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CharactersPerLine)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetLineHeight(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetLineHeight)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn LineHeight(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LineHeight)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetLineSpacing(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetLineSpacing)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn LineSpacing(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LineSpacing)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn LineWidth(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LineWidth)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetIsLetterQuality(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetIsLetterQuality)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsLetterQuality(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsLetterQuality)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsPaperNearEnd(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsPaperNearEnd)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetColorCartridge(&self, value: PosPrinterColorCartridge) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetColorCartridge)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn ColorCartridge(&self) -> ::windows::core::Result<PosPrinterColorCartridge> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: PosPrinterColorCartridge = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ColorCartridge)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterColorCartridge>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsCoverOpen(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCoverOpen)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsCartridgeRemoved(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCartridgeRemoved)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsCartridgeEmpty(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCartridgeEmpty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsHeadCleaning(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsHeadCleaning)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsPaperEmpty(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsPaperEmpty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsReadyToPrint(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsReadyToPrint)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn ValidateData<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, data: Param0) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ValidateData)(::core::mem::transmute_copy(this), data.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for ClaimedReceiptPrinter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ClaimedReceiptPrinter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ClaimedReceiptPrinter {}
impl ::core::fmt::Debug for ClaimedReceiptPrinter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClaimedReceiptPrinter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ClaimedReceiptPrinter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedReceiptPrinter;{9ad27a74-dd61-4ee2-9837-5b5d72d538b9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ClaimedReceiptPrinter {
    type Vtable = IClaimedReceiptPrinter_Vtbl;
    const IID: ::windows::core::GUID = <IClaimedReceiptPrinter as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ClaimedReceiptPrinter {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedReceiptPrinter";
}
impl ::core::convert::From<ClaimedReceiptPrinter> for ::windows::core::IUnknown {
    fn from(value: ClaimedReceiptPrinter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ClaimedReceiptPrinter> for ::windows::core::IUnknown {
    fn from(value: &ClaimedReceiptPrinter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ClaimedReceiptPrinter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ClaimedReceiptPrinter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ClaimedReceiptPrinter> for ::windows::core::IInspectable {
    fn from(value: ClaimedReceiptPrinter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ClaimedReceiptPrinter> for ::windows::core::IInspectable {
    fn from(value: &ClaimedReceiptPrinter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ClaimedReceiptPrinter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ClaimedReceiptPrinter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct ClaimedSlipPrinter(::windows::core::IUnknown);
impl ClaimedSlipPrinter {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SidewaysMaxLines(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SidewaysMaxLines)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SidewaysMaxChars(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SidewaysMaxChars)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn MaxLines(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MaxLines)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn LinesNearEndToEnd(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LinesNearEndToEnd)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn PrintSide(&self) -> ::windows::core::Result<PosPrinterPrintSide> {
        let this = self;
        unsafe {
            let mut result__: PosPrinterPrintSide = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PrintSide)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterPrintSide>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PageSize(&self) -> ::windows::core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Size = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PageSize)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PrintArea(&self) -> ::windows::core::Result<super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PrintArea)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn OpenJaws(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).OpenJaws)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn CloseJaws(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).CloseJaws)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InsertSlipAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, timeout: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InsertSlipAsync)(::core::mem::transmute_copy(this), timeout.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSlipAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, timeout: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RemoveSlipAsync)(::core::mem::transmute_copy(this), timeout.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn ChangePrintSide(&self, printside: PosPrinterPrintSide) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ChangePrintSide)(::core::mem::transmute_copy(this), printside).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn CreateJob(&self) -> ::windows::core::Result<SlipPrintJob> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateJob)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SlipPrintJob>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetCharactersPerLine(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetCharactersPerLine)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn CharactersPerLine(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CharactersPerLine)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetLineHeight(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetLineHeight)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn LineHeight(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LineHeight)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetLineSpacing(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetLineSpacing)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn LineSpacing(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LineSpacing)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn LineWidth(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LineWidth)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetIsLetterQuality(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetIsLetterQuality)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsLetterQuality(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsLetterQuality)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsPaperNearEnd(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsPaperNearEnd)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetColorCartridge(&self, value: PosPrinterColorCartridge) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetColorCartridge)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn ColorCartridge(&self) -> ::windows::core::Result<PosPrinterColorCartridge> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: PosPrinterColorCartridge = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ColorCartridge)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterColorCartridge>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsCoverOpen(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCoverOpen)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsCartridgeRemoved(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCartridgeRemoved)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsCartridgeEmpty(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCartridgeEmpty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsHeadCleaning(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsHeadCleaning)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsPaperEmpty(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsPaperEmpty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsReadyToPrint(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsReadyToPrint)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn ValidateData<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, data: Param0) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonClaimedPosPrinterStation>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ValidateData)(::core::mem::transmute_copy(this), data.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for ClaimedSlipPrinter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ClaimedSlipPrinter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ClaimedSlipPrinter {}
impl ::core::fmt::Debug for ClaimedSlipPrinter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ClaimedSlipPrinter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ClaimedSlipPrinter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ClaimedSlipPrinter;{bd5deff2-af90-4e8a-b77b-e3ae9ca63a7f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ClaimedSlipPrinter {
    type Vtable = IClaimedSlipPrinter_Vtbl;
    const IID: ::windows::core::GUID = <IClaimedSlipPrinter as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ClaimedSlipPrinter {
    const NAME: &'static str = "Windows.Devices.PointOfService.ClaimedSlipPrinter";
}
impl ::core::convert::From<ClaimedSlipPrinter> for ::windows::core::IUnknown {
    fn from(value: ClaimedSlipPrinter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ClaimedSlipPrinter> for ::windows::core::IUnknown {
    fn from(value: &ClaimedSlipPrinter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ClaimedSlipPrinter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ClaimedSlipPrinter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ClaimedSlipPrinter> for ::windows::core::IInspectable {
    fn from(value: ClaimedSlipPrinter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ClaimedSlipPrinter> for ::windows::core::IInspectable {
    fn from(value: &ClaimedSlipPrinter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ClaimedSlipPrinter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ClaimedSlipPrinter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScanner(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarcodeScanner {
    type Vtable = IBarcodeScanner_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbea33e06_b264_4f03_a9c1_45b20f01134f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScanner_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Capabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ClaimScannerAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ClaimScannerAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CheckHealthAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, level: UnifiedPosHealthCheckLevel, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CheckHealthAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSupportedSymbologiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSupportedSymbologiesAsync: usize,
    #[cfg(feature = "Foundation")]
    pub IsSymbologySupportedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, barcodesymbology: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsSymbologySupportedAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub RetrieveStatisticsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statisticscategories: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    RetrieveStatisticsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSupportedProfiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSupportedProfiles: usize,
    pub IsProfileSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profile: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StatusUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StatusUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStatusUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStatusUpdated: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScanner2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarcodeScanner2 {
    type Vtable = IBarcodeScanner2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x89215167_8cee_436d_89ab_8dfb43bb4286);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScanner2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub VideoDeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerCapabilities(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarcodeScannerCapabilities {
    type Vtable = IBarcodeScannerCapabilities_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc60691e4_f2c8_4420_a307_b12ef6622857);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerCapabilities_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub PowerReportingType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UnifiedPosPowerReportingType) -> ::windows::core::HRESULT,
    pub IsStatisticsReportingSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsStatisticsUpdatingSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsImagePreviewSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerCapabilities1(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarcodeScannerCapabilities1 {
    type Vtable = IBarcodeScannerCapabilities1_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8e5ab3e9_0e2c_472f_a1cc_ee8054b6a684);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerCapabilities1_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsSoftwareTriggerSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerCapabilities2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarcodeScannerCapabilities2 {
    type Vtable = IBarcodeScannerCapabilities2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf211cfec_e1a1_4ea8_9abc_92b1596270ab);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerCapabilities2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsVideoPreviewSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerDataReceivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarcodeScannerDataReceivedEventArgs {
    type Vtable = IBarcodeScannerDataReceivedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4234a7e2_ed97_467d_ad2b_01e44313a929);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerDataReceivedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Report: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerErrorOccurredEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarcodeScannerErrorOccurredEventArgs {
    type Vtable = IBarcodeScannerErrorOccurredEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2cd2602f_cf3a_4002_a75a_c5ec468f0a20);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerErrorOccurredEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub PartialInputData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub IsRetriable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ErrorData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerImagePreviewReceivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarcodeScannerImagePreviewReceivedEventArgs {
    type Vtable = IBarcodeScannerImagePreviewReceivedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf3b7de85_6e8b_434e_9f58_06ef26bc4baf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerImagePreviewReceivedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Preview: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Preview: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerReport(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarcodeScannerReport {
    type Vtable = IBarcodeScannerReport_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5ce4d8b0_a489_4b96_86c4_f0bf8a37753d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerReport_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ScanDataType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub ScanData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ScanData: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ScanDataLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ScanDataLabel: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerReportFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarcodeScannerReportFactory {
    type Vtable = IBarcodeScannerReportFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa2547326_2013_457c_8963_49c15dca78ce);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerReportFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scandatatype: u32, scandata: ::windows::core::RawPtr, scandatalabel: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateInstance: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarcodeScannerStatics {
    type Vtable = IBarcodeScannerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d115f6f_da49_41e8_8c8c_f0cb62a9c4fc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDefaultAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarcodeScannerStatics2 {
    type Vtable = IBarcodeScannerStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb8652473_a36f_4007_b1d0_279ebe92a656);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetDeviceSelectorWithConnectionTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectiontypes: PosConnectionTypes, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeScannerStatusUpdatedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarcodeScannerStatusUpdatedEventArgs {
    type Vtable = IBarcodeScannerStatusUpdatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x355d8586_9c43_462b_a91a_816dc97f452c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeScannerStatusUpdatedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BarcodeScannerStatus) -> ::windows::core::HRESULT,
    pub ExtendedStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeSymbologiesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarcodeSymbologiesStatics {
    type Vtable = IBarcodeSymbologiesStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca8549bb_06d2_43f4_a44b_c620679fd8d0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeSymbologiesStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Unknown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Ean8: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Ean8Add2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Ean8Add5: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Eanv: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub EanvAdd2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub EanvAdd5: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Ean13: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Ean13Add2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Ean13Add5: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Isbn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub IsbnAdd5: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Ismn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub IsmnAdd2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub IsmnAdd5: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Issn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub IssnAdd2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub IssnAdd5: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Ean99: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Ean99Add2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Ean99Add5: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Upca: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub UpcaAdd2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub UpcaAdd5: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Upce: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub UpceAdd2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub UpceAdd5: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub UpcCoupon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub TfStd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub TfDis: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub TfInt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub TfInd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub TfMat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub TfIata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Gs1DatabarType1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Gs1DatabarType2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Gs1DatabarType3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Code39: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Code39Ex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Trioptic39: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Code32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Pzn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Code93: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Code93Ex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Code128: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Gs1128: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Gs1128Coupon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub UccEan128: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Sisac: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Isbt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Codabar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Code11: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Msi: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Plessey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Telepen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Code16k: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub CodablockA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub CodablockF: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Codablock128: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Code49: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Aztec: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub DataCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub DataMatrix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub HanXin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Maxicode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub MicroPdf417: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub MicroQr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Pdf417: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Qr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub MsTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Ccab: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Ccc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Tlc39: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub AusPost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub CanPost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub ChinaPost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub DutchKix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub InfoMail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub ItalianPost25: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub ItalianPost39: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub JapanPost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub KoreanPost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SwedenPost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub UkPost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub UsIntelligent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub UsIntelligentPkg: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub UsPlanet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub UsPostNet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Us4StateFics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub OcrA: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub OcrB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Micr: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub ExtendedBase: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scandatatype: u32, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeSymbologiesStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarcodeSymbologiesStatics2 {
    type Vtable = IBarcodeSymbologiesStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8b7518f4_99d0_40bf_9424_b91d6dd4c6e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeSymbologiesStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Gs1DWCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBarcodeSymbologyAttributes(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBarcodeSymbologyAttributes {
    type Vtable = IBarcodeSymbologyAttributes_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x66413a78_ab7a_4ada_8ece_936014b2ead7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBarcodeSymbologyAttributes_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsCheckDigitValidationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsCheckDigitValidationEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsCheckDigitValidationSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsCheckDigitTransmissionEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsCheckDigitTransmissionEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsCheckDigitTransmissionSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub DecodeLength1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetDecodeLength1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub DecodeLength2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetDecodeLength2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub DecodeLengthKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BarcodeSymbologyDecodeLengthKind) -> ::windows::core::HRESULT,
    pub SetDecodeLengthKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: BarcodeSymbologyDecodeLengthKind) -> ::windows::core::HRESULT,
    pub IsDecodeLengthSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICashDrawer(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICashDrawer {
    type Vtable = ICashDrawer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f88f5c8_de54_4aee_a890_920bcbfe30fc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICashDrawer_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Capabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub IsDrawerOpen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub DrawerEventSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ClaimDrawerAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ClaimDrawerAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CheckHealthAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, level: UnifiedPosHealthCheckLevel, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CheckHealthAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetStatisticsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statisticscategories: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetStatisticsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StatusUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StatusUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStatusUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStatusUpdated: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICashDrawerCapabilities(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICashDrawerCapabilities {
    type Vtable = ICashDrawerCapabilities_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0bc6de0b_e8e7_4b1f_b1d1_3e501ad08247);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICashDrawerCapabilities_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub PowerReportingType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UnifiedPosPowerReportingType) -> ::windows::core::HRESULT,
    pub IsStatisticsReportingSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsStatisticsUpdatingSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsStatusReportingSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsStatusMultiDrawerDetectSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsDrawerOpenSensorAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICashDrawerCloseAlarm(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICashDrawerCloseAlarm {
    type Vtable = ICashDrawerCloseAlarm_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6bf88cc7_6f63_430e_ab3b_95d75ffbe87f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICashDrawerCloseAlarm_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub SetAlarmTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetAlarmTimeout: usize,
    #[cfg(feature = "Foundation")]
    pub AlarmTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AlarmTimeout: usize,
    pub SetBeepFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub BeepFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetBeepDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetBeepDuration: usize,
    #[cfg(feature = "Foundation")]
    pub BeepDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BeepDuration: usize,
    #[cfg(feature = "Foundation")]
    pub SetBeepDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetBeepDelay: usize,
    #[cfg(feature = "Foundation")]
    pub BeepDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BeepDelay: usize,
    #[cfg(feature = "Foundation")]
    pub AlarmTimeoutExpired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AlarmTimeoutExpired: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAlarmTimeoutExpired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAlarmTimeoutExpired: usize,
    #[cfg(feature = "Foundation")]
    pub StartAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICashDrawerEventSource(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICashDrawerEventSource {
    type Vtable = ICashDrawerEventSource_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe006e46c_f2f9_442f_8dd6_06c10a4227ba);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICashDrawerEventSource_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub DrawerClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DrawerClosed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDrawerClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDrawerClosed: usize,
    #[cfg(feature = "Foundation")]
    pub DrawerOpened: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DrawerOpened: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDrawerOpened: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDrawerOpened: usize,
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct ICashDrawerEventSourceEventArgs(::windows::core::IUnknown);
impl ICashDrawerEventSourceEventArgs {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn CashDrawer(&self) -> ::windows::core::Result<CashDrawer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CashDrawer)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CashDrawer>(result__)
        }
    }
}
impl ::core::convert::From<ICashDrawerEventSourceEventArgs> for ::windows::core::IUnknown {
    fn from(value: ICashDrawerEventSourceEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICashDrawerEventSourceEventArgs> for ::windows::core::IUnknown {
    fn from(value: &ICashDrawerEventSourceEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICashDrawerEventSourceEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICashDrawerEventSourceEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ICashDrawerEventSourceEventArgs> for ::windows::core::IInspectable {
    fn from(value: ICashDrawerEventSourceEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICashDrawerEventSourceEventArgs> for ::windows::core::IInspectable {
    fn from(value: &ICashDrawerEventSourceEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ICashDrawerEventSourceEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ICashDrawerEventSourceEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICashDrawerEventSourceEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICashDrawerEventSourceEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICashDrawerEventSourceEventArgs {}
impl ::core::fmt::Debug for ICashDrawerEventSourceEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICashDrawerEventSourceEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ICashDrawerEventSourceEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{69cb3bc1-147f-421c-9c23-090123bb786c}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ICashDrawerEventSourceEventArgs {
    type Vtable = ICashDrawerEventSourceEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69cb3bc1_147f_421c_9c23_090123bb786c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICashDrawerEventSourceEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CashDrawer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICashDrawerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICashDrawerStatics {
    type Vtable = ICashDrawerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdfa0955a_d437_4fff_b547_dda969a4f883);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICashDrawerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDefaultAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICashDrawerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICashDrawerStatics2 {
    type Vtable = ICashDrawerStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3e818121_8c42_40e8_9c0e_40297048104c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICashDrawerStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetDeviceSelectorWithConnectionTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectiontypes: PosConnectionTypes, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICashDrawerStatus(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICashDrawerStatus {
    type Vtable = ICashDrawerStatus_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6bbd78bf_dca1_4e06_99eb_5af6a5aec108);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICashDrawerStatus_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub StatusKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CashDrawerStatusKind) -> ::windows::core::HRESULT,
    pub ExtendedStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICashDrawerStatusUpdatedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICashDrawerStatusUpdatedEventArgs {
    type Vtable = ICashDrawerStatusUpdatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30aae98a_0d70_459c_9553_87e124c52488);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICashDrawerStatusUpdatedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedBarcodeScanner(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IClaimedBarcodeScanner {
    type Vtable = IClaimedBarcodeScanner_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4a63b49c_8fa4_4332_bb26_945d11d81e0f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedBarcodeScanner_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsDisabledOnDataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsDisabledOnDataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsDecodeDataEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsDecodeDataEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EnableAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnableAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DisableAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DisableAsync: usize,
    pub RetainDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SetActiveSymbologiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, symbologies: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetActiveSymbologiesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ResetStatisticsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statisticscategories: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ResetStatisticsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub UpdateStatisticsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statistics: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UpdateStatisticsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SetActiveProfileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, profile: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetActiveProfileAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DataReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDataReceived: usize,
    #[cfg(feature = "Foundation")]
    pub TriggerPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TriggerPressed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTriggerPressed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTriggerPressed: usize,
    #[cfg(feature = "Foundation")]
    pub TriggerReleased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TriggerReleased: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTriggerReleased: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTriggerReleased: usize,
    #[cfg(feature = "Foundation")]
    pub ReleaseDeviceRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReleaseDeviceRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReleaseDeviceRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReleaseDeviceRequested: usize,
    #[cfg(feature = "Foundation")]
    pub ImagePreviewReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ImagePreviewReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveImagePreviewReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveImagePreviewReceived: usize,
    #[cfg(feature = "Foundation")]
    pub ErrorOccurred: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ErrorOccurred: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveErrorOccurred: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveErrorOccurred: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedBarcodeScanner1(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IClaimedBarcodeScanner1 {
    type Vtable = IClaimedBarcodeScanner1_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf61aad0c_8551_42b4_998c_970c20210a22);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedBarcodeScanner1_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub StartSoftwareTriggerAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartSoftwareTriggerAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StopSoftwareTriggerAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StopSoftwareTriggerAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedBarcodeScanner2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IClaimedBarcodeScanner2 {
    type Vtable = IClaimedBarcodeScanner2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3b59e8c_2d8b_4f70_8af3_3448bedd5fe2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedBarcodeScanner2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub GetSymbologyAttributesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, barcodesymbology: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetSymbologyAttributesAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SetSymbologyAttributesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, barcodesymbology: u32, attributes: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSymbologyAttributesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedBarcodeScanner3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IClaimedBarcodeScanner3 {
    type Vtable = IClaimedBarcodeScanner3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe6ceb430_712e_45fc_8b86_cd55f5aef79d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedBarcodeScanner3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub ShowVideoPreviewAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowVideoPreviewAsync: usize,
    pub HideVideoPreview: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetIsVideoPreviewShownOnEnable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsVideoPreviewShownOnEnable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedBarcodeScanner4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IClaimedBarcodeScanner4 {
    type Vtable = IClaimedBarcodeScanner4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d501f97_376a_41a8_a230_2f37c1949dde);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedBarcodeScanner4_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Closed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosed: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedBarcodeScannerClosedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IClaimedBarcodeScannerClosedEventArgs {
    type Vtable = IClaimedBarcodeScannerClosedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf7d5489_a22c_4c65_a901_88d77d833954);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedBarcodeScannerClosedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedCashDrawer(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IClaimedCashDrawer {
    type Vtable = IClaimedCashDrawer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca3f99af_abb8_42c1_8a84_5c66512f5a75);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedCashDrawer_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsDrawerOpen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CloseAlarm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub OpenDrawerAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OpenDrawerAsync: usize,
    #[cfg(feature = "Foundation")]
    pub EnableAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnableAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DisableAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DisableAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RetainDeviceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RetainDeviceAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ResetStatisticsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statisticscategories: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ResetStatisticsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub UpdateStatisticsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statistics: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UpdateStatisticsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReleaseDeviceRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReleaseDeviceRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReleaseDeviceRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReleaseDeviceRequested: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedCashDrawer2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IClaimedCashDrawer2 {
    type Vtable = IClaimedCashDrawer2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9cbab5a2_de42_4d5b_b0c1_9b57a2ba89c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedCashDrawer2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Closed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosed: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedCashDrawerClosedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IClaimedCashDrawerClosedEventArgs {
    type Vtable = IClaimedCashDrawerClosedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcc573f33_3f34_4c5c_baae_deadf16cd7fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedCashDrawerClosedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedJournalPrinter(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IClaimedJournalPrinter {
    type Vtable = IClaimedJournalPrinter_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x67ea0630_517d_487f_9fdf_d2e0a0a264a5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedJournalPrinter_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateJob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedLineDisplay(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IClaimedLineDisplay {
    type Vtable = IClaimedLineDisplay_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x120ac970_9a75_4acf_aae7_09972bcf8794);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedLineDisplay_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Capabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub PhysicalDeviceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PhysicalDeviceDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DeviceControlDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DeviceControlVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DeviceServiceVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DefaultWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RetainDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReleaseDeviceRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReleaseDeviceRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReleaseDeviceRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReleaseDeviceRequested: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedLineDisplay2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IClaimedLineDisplay2 {
    type Vtable = IClaimedLineDisplay2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa31c75ed_41f5_4e76_a074_795e47a46e97);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedLineDisplay2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetStatisticsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statisticscategories: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetStatisticsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CheckHealthAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, level: UnifiedPosHealthCheckLevel, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CheckHealthAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CheckPowerStatusAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CheckPowerStatusAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StatusUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StatusUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStatusUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStatusUpdated: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedScreenSizesInCharacters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedScreenSizesInCharacters: usize,
    #[cfg(feature = "Foundation")]
    pub MaxBitmapSizeInPixels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MaxBitmapSizeInPixels: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedCharacterSets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedCharacterSets: usize,
    pub CustomGlyphs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TryUpdateAttributesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributes: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryUpdateAttributesAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TrySetDescriptorAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, descriptor: u32, descriptorstate: LineDisplayDescriptorState, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TrySetDescriptorAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryClearDescriptorsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryClearDescriptorsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryCreateWindowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewport: super::super::Foundation::Rect, windowsize: super::super::Foundation::Size, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryCreateWindowAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub TryStoreStorageFileBitmapAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    TryStoreStorageFileBitmapAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub TryStoreStorageFileBitmapWithAlignmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    TryStoreStorageFileBitmapWithAlignmentAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub TryStoreStorageFileBitmapWithAlignmentAndWidthAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment, widthinpixels: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    TryStoreStorageFileBitmapWithAlignmentAndWidthAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedLineDisplay3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IClaimedLineDisplay3 {
    type Vtable = IClaimedLineDisplay3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x642ecd92_e9d4_4ecc_af75_329c274cd18f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedLineDisplay3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Closed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosed: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedLineDisplayClosedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IClaimedLineDisplayClosedEventArgs {
    type Vtable = IClaimedLineDisplayClosedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf915f364_d3d5_4f10_b511_90939edfacd8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedLineDisplayClosedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedLineDisplayStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IClaimedLineDisplayStatics {
    type Vtable = IClaimedLineDisplayStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x78ca98fb_8b6b_4973_86f0_3e570c351825);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedLineDisplayStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetDeviceSelectorWithConnectionTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectiontypes: PosConnectionTypes, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedMagneticStripeReader(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IClaimedMagneticStripeReader {
    type Vtable = IClaimedMagneticStripeReader_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x475ca8f3_9417_48bc_b9d7_4163a7844c02);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedMagneticStripeReader_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsDisabledOnDataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsDisabledOnDataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsDecodeDataEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsDecodeDataEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsDeviceAuthenticated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetDataEncryptionAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub DataEncryptionAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetTracksToRead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MagneticStripeReaderTrackIds) -> ::windows::core::HRESULT,
    pub TracksToRead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MagneticStripeReaderTrackIds) -> ::windows::core::HRESULT,
    pub SetIsTransmitSentinelsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsTransmitSentinelsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EnableAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnableAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DisableAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DisableAsync: usize,
    pub RetainDevice: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetErrorReportingType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MagneticStripeReaderErrorReportingType) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub RetrieveDeviceAuthenticationDataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    RetrieveDeviceAuthenticationDataAsync: usize,
    #[cfg(feature = "Foundation")]
    pub AuthenticateDeviceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, responseToken_array_size: u32, responsetoken: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AuthenticateDeviceAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeAuthenticateDeviceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, responseToken_array_size: u32, responsetoken: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeAuthenticateDeviceAsync: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateKeyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, keyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateKeyAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ResetStatisticsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statisticscategories: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ResetStatisticsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub UpdateStatisticsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statistics: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UpdateStatisticsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub BankCardDataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BankCardDataReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBankCardDataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBankCardDataReceived: usize,
    #[cfg(feature = "Foundation")]
    pub AamvaCardDataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AamvaCardDataReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAamvaCardDataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAamvaCardDataReceived: usize,
    #[cfg(feature = "Foundation")]
    pub VendorSpecificDataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    VendorSpecificDataReceived: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveVendorSpecificDataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveVendorSpecificDataReceived: usize,
    #[cfg(feature = "Foundation")]
    pub ReleaseDeviceRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReleaseDeviceRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReleaseDeviceRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReleaseDeviceRequested: usize,
    #[cfg(feature = "Foundation")]
    pub ErrorOccurred: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ErrorOccurred: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveErrorOccurred: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveErrorOccurred: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedMagneticStripeReader2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IClaimedMagneticStripeReader2 {
    type Vtable = IClaimedMagneticStripeReader2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x236fafdf_e2dc_4d7d_9c78_060df2bf2928);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedMagneticStripeReader2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Closed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosed: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedMagneticStripeReaderClosedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IClaimedMagneticStripeReaderClosedEventArgs {
    type Vtable = IClaimedMagneticStripeReaderClosedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x14ada93a_adcd_4c80_acda_c3eaed2647e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedMagneticStripeReaderClosedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedPosPrinter(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IClaimedPosPrinter {
    type Vtable = IClaimedPosPrinter_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d64ce0c_e03e_4b14_a38e_c28c34b86353);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedPosPrinter_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetCharacterSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub CharacterSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub IsCoverOpen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsCharacterSetMappingEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsCharacterSetMappingEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetMapMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PosPrinterMapMode) -> ::windows::core::HRESULT,
    pub MapMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterMapMode) -> ::windows::core::HRESULT,
    pub Receipt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Slip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Journal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub EnableAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    EnableAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DisableAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DisableAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RetainDeviceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RetainDeviceAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ResetStatisticsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statisticscategories: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ResetStatisticsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub UpdateStatisticsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statistics: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    UpdateStatisticsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReleaseDeviceRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReleaseDeviceRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveReleaseDeviceRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveReleaseDeviceRequested: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedPosPrinter2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IClaimedPosPrinter2 {
    type Vtable = IClaimedPosPrinter2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5bf7a3d5_5198_437a_82df_589993fa77e1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedPosPrinter2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Closed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosed: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedPosPrinterClosedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IClaimedPosPrinterClosedEventArgs {
    type Vtable = IClaimedPosPrinterClosedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2b7a27b_4d40_471d_92ed_63375b18c788);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedPosPrinterClosedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedReceiptPrinter(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IClaimedReceiptPrinter {
    type Vtable = IClaimedReceiptPrinter_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ad27a74_dd61_4ee2_9837_5b5d72d538b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedReceiptPrinter_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SidewaysMaxLines: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SidewaysMaxChars: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub LinesToPaperCut: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PageSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PageSize: usize,
    #[cfg(feature = "Foundation")]
    pub PrintArea: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PrintArea: usize,
    pub CreateJob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IClaimedSlipPrinter(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IClaimedSlipPrinter {
    type Vtable = IClaimedSlipPrinter_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbd5deff2_af90_4e8a_b77b_e3ae9ca63a7f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClaimedSlipPrinter_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SidewaysMaxLines: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SidewaysMaxChars: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub MaxLines: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub LinesNearEndToEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub PrintSide: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterPrintSide) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub PageSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PageSize: usize,
    #[cfg(feature = "Foundation")]
    pub PrintArea: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PrintArea: usize,
    pub OpenJaws: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CloseJaws: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub InsertSlipAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timeout: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InsertSlipAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSlipAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timeout: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSlipAsync: usize,
    pub ChangePrintSide: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printside: PosPrinterPrintSide) -> ::windows::core::HRESULT,
    pub CreateJob: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct ICommonClaimedPosPrinterStation(::windows::core::IUnknown);
impl ICommonClaimedPosPrinterStation {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetCharactersPerLine(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCharactersPerLine)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn CharactersPerLine(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CharactersPerLine)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetLineHeight(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLineHeight)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn LineHeight(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LineHeight)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetLineSpacing(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLineSpacing)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn LineSpacing(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LineSpacing)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn LineWidth(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LineWidth)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetIsLetterQuality(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsLetterQuality)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsLetterQuality(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsLetterQuality)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsPaperNearEnd(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsPaperNearEnd)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetColorCartridge(&self, value: PosPrinterColorCartridge) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetColorCartridge)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn ColorCartridge(&self) -> ::windows::core::Result<PosPrinterColorCartridge> {
        let this = self;
        unsafe {
            let mut result__: PosPrinterColorCartridge = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ColorCartridge)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterColorCartridge>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsCoverOpen(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCoverOpen)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsCartridgeRemoved(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCartridgeRemoved)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsCartridgeEmpty(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCartridgeEmpty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsHeadCleaning(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsHeadCleaning)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsPaperEmpty(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsPaperEmpty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsReadyToPrint(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsReadyToPrint)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn ValidateData<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, data: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ValidateData)(::core::mem::transmute_copy(this), data.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::convert::From<ICommonClaimedPosPrinterStation> for ::windows::core::IUnknown {
    fn from(value: ICommonClaimedPosPrinterStation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICommonClaimedPosPrinterStation> for ::windows::core::IUnknown {
    fn from(value: &ICommonClaimedPosPrinterStation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICommonClaimedPosPrinterStation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICommonClaimedPosPrinterStation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ICommonClaimedPosPrinterStation> for ::windows::core::IInspectable {
    fn from(value: ICommonClaimedPosPrinterStation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICommonClaimedPosPrinterStation> for ::windows::core::IInspectable {
    fn from(value: &ICommonClaimedPosPrinterStation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ICommonClaimedPosPrinterStation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ICommonClaimedPosPrinterStation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICommonClaimedPosPrinterStation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICommonClaimedPosPrinterStation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICommonClaimedPosPrinterStation {}
impl ::core::fmt::Debug for ICommonClaimedPosPrinterStation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICommonClaimedPosPrinterStation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ICommonClaimedPosPrinterStation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{b7eb66a8-fe8a-4cfb-8b42-e35b280cb27c}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ICommonClaimedPosPrinterStation {
    type Vtable = ICommonClaimedPosPrinterStation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb7eb66a8_fe8a_4cfb_8b42_e35b280cb27c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommonClaimedPosPrinterStation_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SetCharactersPerLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub CharactersPerLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetLineHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub LineHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetLineSpacing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub LineSpacing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub LineWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetIsLetterQuality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsLetterQuality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsPaperNearEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetColorCartridge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PosPrinterColorCartridge) -> ::windows::core::HRESULT,
    pub ColorCartridge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterColorCartridge) -> ::windows::core::HRESULT,
    pub IsCoverOpen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsCartridgeRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsCartridgeEmpty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsHeadCleaning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsPaperEmpty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsReadyToPrint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ValidateData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct ICommonPosPrintStationCapabilities(::windows::core::IUnknown);
impl ICommonPosPrintStationCapabilities {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsPrinterPresent(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsPrinterPresent)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsDualColorSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDualColorSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn ColorCartridgeCapabilities(&self) -> ::windows::core::Result<PosPrinterColorCapabilities> {
        let this = self;
        unsafe {
            let mut result__: PosPrinterColorCapabilities = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ColorCartridgeCapabilities)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterColorCapabilities>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn CartridgeSensors(&self) -> ::windows::core::Result<PosPrinterCartridgeSensors> {
        let this = self;
        unsafe {
            let mut result__: PosPrinterCartridgeSensors = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CartridgeSensors)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterCartridgeSensors>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsBoldSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsBoldSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsItalicSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsItalicSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsUnderlineSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsUnderlineSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsDoubleHighPrintSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDoubleHighPrintSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsDoubleWidePrintSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDoubleWidePrintSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsDoubleHighDoubleWidePrintSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDoubleHighDoubleWidePrintSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsPaperEmptySensorSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsPaperEmptySensorSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsPaperNearEndSensorSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsPaperNearEndSensorSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedCharactersPerLine(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SupportedCharactersPerLine)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
}
impl ::core::convert::From<ICommonPosPrintStationCapabilities> for ::windows::core::IUnknown {
    fn from(value: ICommonPosPrintStationCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICommonPosPrintStationCapabilities> for ::windows::core::IUnknown {
    fn from(value: &ICommonPosPrintStationCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICommonPosPrintStationCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICommonPosPrintStationCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ICommonPosPrintStationCapabilities> for ::windows::core::IInspectable {
    fn from(value: ICommonPosPrintStationCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICommonPosPrintStationCapabilities> for ::windows::core::IInspectable {
    fn from(value: &ICommonPosPrintStationCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ICommonPosPrintStationCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ICommonPosPrintStationCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICommonPosPrintStationCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICommonPosPrintStationCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICommonPosPrintStationCapabilities {}
impl ::core::fmt::Debug for ICommonPosPrintStationCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICommonPosPrintStationCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ICommonPosPrintStationCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{de5b52ca-e02e-40e9-9e5e-1b488e6aacfc}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ICommonPosPrintStationCapabilities {
    type Vtable = ICommonPosPrintStationCapabilities_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde5b52ca_e02e_40e9_9e5e_1b488e6aacfc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommonPosPrintStationCapabilities_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsPrinterPresent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsDualColorSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ColorCartridgeCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterColorCapabilities) -> ::windows::core::HRESULT,
    pub CartridgeSensors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterCartridgeSensors) -> ::windows::core::HRESULT,
    pub IsBoldSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsItalicSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsUnderlineSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsDoubleHighPrintSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsDoubleWidePrintSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsDoubleHighDoubleWidePrintSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsPaperEmptySensorSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsPaperNearEndSensorSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedCharactersPerLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedCharactersPerLine: usize,
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct ICommonReceiptSlipCapabilities(::windows::core::IUnknown);
impl ICommonReceiptSlipCapabilities {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsBarcodeSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsBarcodeSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsBitmapSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsBitmapSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsLeft90RotationSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsLeft90RotationSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsRight90RotationSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsRight90RotationSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Is180RotationSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Is180RotationSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsPrintAreaSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsPrintAreaSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn RuledLineCapabilities(&self) -> ::windows::core::Result<PosPrinterRuledLineCapabilities> {
        let this = self;
        unsafe {
            let mut result__: PosPrinterRuledLineCapabilities = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RuledLineCapabilities)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterRuledLineCapabilities>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedBarcodeRotations(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SupportedBarcodeRotations)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedBitmapRotations(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SupportedBitmapRotations)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsPrinterPresent(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsPrinterPresent)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsDualColorSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDualColorSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn ColorCartridgeCapabilities(&self) -> ::windows::core::Result<PosPrinterColorCapabilities> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: PosPrinterColorCapabilities = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ColorCartridgeCapabilities)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterColorCapabilities>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn CartridgeSensors(&self) -> ::windows::core::Result<PosPrinterCartridgeSensors> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: PosPrinterCartridgeSensors = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CartridgeSensors)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterCartridgeSensors>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsBoldSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsBoldSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsItalicSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsItalicSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsUnderlineSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsUnderlineSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsDoubleHighPrintSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDoubleHighPrintSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsDoubleWidePrintSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDoubleWidePrintSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsDoubleHighDoubleWidePrintSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDoubleHighDoubleWidePrintSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsPaperEmptySensorSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsPaperEmptySensorSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsPaperNearEndSensorSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsPaperNearEndSensorSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedCharactersPerLine(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SupportedCharactersPerLine)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
}
impl ::core::convert::From<ICommonReceiptSlipCapabilities> for ::windows::core::IUnknown {
    fn from(value: ICommonReceiptSlipCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICommonReceiptSlipCapabilities> for ::windows::core::IUnknown {
    fn from(value: &ICommonReceiptSlipCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICommonReceiptSlipCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICommonReceiptSlipCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ICommonReceiptSlipCapabilities> for ::windows::core::IInspectable {
    fn from(value: ICommonReceiptSlipCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICommonReceiptSlipCapabilities> for ::windows::core::IInspectable {
    fn from(value: &ICommonReceiptSlipCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ICommonReceiptSlipCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ICommonReceiptSlipCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl ::core::clone::Clone for ICommonReceiptSlipCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICommonReceiptSlipCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICommonReceiptSlipCapabilities {}
impl ::core::fmt::Debug for ICommonReceiptSlipCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICommonReceiptSlipCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ICommonReceiptSlipCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{09286b8b-9873-4d05-bfbe-4727a6038f69}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ICommonReceiptSlipCapabilities {
    type Vtable = ICommonReceiptSlipCapabilities_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09286b8b_9873_4d05_bfbe_4727a6038f69);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommonReceiptSlipCapabilities_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsBarcodeSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsBitmapSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsLeft90RotationSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsRight90RotationSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Is180RotationSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsPrintAreaSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub RuledLineCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterRuledLineCapabilities) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedBarcodeRotations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedBarcodeRotations: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedBitmapRotations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedBitmapRotations: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IJournalPrintJob(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IJournalPrintJob {
    type Vtable = IJournalPrintJob_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f4f2864_f3f0_55d0_8c39_74cc91783eed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJournalPrintJob_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Print: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, printoptions: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub FeedPaperByLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, linecount: i32) -> ::windows::core::HRESULT,
    pub FeedPaperByMapModeUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, distance: i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IJournalPrinterCapabilities(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IJournalPrinterCapabilities {
    type Vtable = IJournalPrinterCapabilities_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b5ccc43_e047_4463_bb58_17b5ba1d8056);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJournalPrinterCapabilities_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IJournalPrinterCapabilities2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IJournalPrinterCapabilities2 {
    type Vtable = IJournalPrinterCapabilities2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03b0b645_33b8_533b_baaa_a4389283ab0a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IJournalPrinterCapabilities2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsReverseVideoSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsStrikethroughSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsSuperscriptSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsSubscriptSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsReversePaperFeedByLineSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsReversePaperFeedByMapModeUnitSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineDisplay(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILineDisplay {
    type Vtable = ILineDisplay_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x24f5df4e_3c99_44e2_b73f_e51be3637a8c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplay_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Capabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub PhysicalDeviceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PhysicalDeviceDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DeviceControlDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DeviceControlVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DeviceServiceVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ClaimAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ClaimAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineDisplay2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILineDisplay2 {
    type Vtable = ILineDisplay2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc296a628_ef44_40f3_bd1c_b04c6a5cdc7d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplay2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub CheckPowerStatusAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CheckPowerStatusAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineDisplayAttributes(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILineDisplayAttributes {
    type Vtable = ILineDisplayAttributes_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc17de99c_229a_4c14_a6f1_b4e4b1fead92);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayAttributes_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsPowerNotifyEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsPowerNotifyEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Brightness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetBrightness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub BlinkRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BlinkRate: usize,
    #[cfg(feature = "Foundation")]
    pub SetBlinkRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetBlinkRate: usize,
    #[cfg(feature = "Foundation")]
    pub ScreenSizeInCharacters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ScreenSizeInCharacters: usize,
    #[cfg(feature = "Foundation")]
    pub SetScreenSizeInCharacters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetScreenSizeInCharacters: usize,
    pub CharacterSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetCharacterSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub IsCharacterSetMappingEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsCharacterSetMappingEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub CurrentWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetCurrentWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineDisplayCapabilities(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILineDisplayCapabilities {
    type Vtable = ILineDisplayCapabilities_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5a15b5d1_8dc5_4b9c_9172_303e47b70c55);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayCapabilities_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsStatisticsReportingSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsStatisticsUpdatingSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub PowerReportingType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UnifiedPosPowerReportingType) -> ::windows::core::HRESULT,
    pub CanChangeScreenSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CanDisplayBitmaps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CanReadCharacterAtCursor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CanMapCharacterSets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CanDisplayCustomGlyphs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CanReverse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LineDisplayTextAttributeGranularity) -> ::windows::core::HRESULT,
    pub CanBlink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LineDisplayTextAttributeGranularity) -> ::windows::core::HRESULT,
    pub CanChangeBlinkRate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsBrightnessSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsCursorSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsHorizontalMarqueeSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsVerticalMarqueeSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsInterCharacterWaitSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SupportedDescriptors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SupportedWindows: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineDisplayCursor(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILineDisplayCursor {
    type Vtable = ILineDisplayCursor_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecdffc45_754a_4e3b_ab2b_151181085605);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayCursor_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CanCustomize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsBlinkSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsBlockSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsHalfBlockSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsUnderlineSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsReverseSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsOtherSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub GetAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TryUpdateAttributesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributes: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryUpdateAttributesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineDisplayCursorAttributes(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILineDisplayCursorAttributes {
    type Vtable = ILineDisplayCursorAttributes_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e2d54fe_4ffd_4190_aae1_ce285f20c896);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayCursorAttributes_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsBlinkEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsBlinkEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub CursorType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LineDisplayCursorType) -> ::windows::core::HRESULT,
    pub SetCursorType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: LineDisplayCursorType) -> ::windows::core::HRESULT,
    pub IsAutoAdvanceEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsAutoAdvanceEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Position: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Position: usize,
    #[cfg(feature = "Foundation")]
    pub SetPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPosition: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineDisplayCustomGlyphs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILineDisplayCustomGlyphs {
    type Vtable = ILineDisplayCustomGlyphs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2257f63c_f263_44f1_a1a0_e750a6a0ec54);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayCustomGlyphs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub SizeInPixels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SizeInPixels: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedGlyphCodes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedGlyphCodes: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub TryRedefineAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, glyphcode: u32, glyphdata: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    TryRedefineAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineDisplayMarquee(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILineDisplayMarquee {
    type Vtable = ILineDisplayMarquee_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa3d33e3e_f46a_4b7a_bc21_53eb3b57f8b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayMarquee_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Format: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LineDisplayMarqueeFormat) -> ::windows::core::HRESULT,
    pub SetFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: LineDisplayMarqueeFormat) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RepeatWaitInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RepeatWaitInterval: usize,
    #[cfg(feature = "Foundation")]
    pub SetRepeatWaitInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetRepeatWaitInterval: usize,
    #[cfg(feature = "Foundation")]
    pub ScrollWaitInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ScrollWaitInterval: usize,
    #[cfg(feature = "Foundation")]
    pub SetScrollWaitInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetScrollWaitInterval: usize,
    #[cfg(feature = "Foundation")]
    pub TryStartScrollingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, direction: LineDisplayScrollDirection, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryStartScrollingAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryStopScrollingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryStopScrollingAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineDisplayStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILineDisplayStatics {
    type Vtable = ILineDisplayStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x022dc0b6_11b0_4690_9547_0b39c5af2114);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDefaultAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetDeviceSelectorWithConnectionTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectiontypes: PosConnectionTypes, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineDisplayStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILineDisplayStatics2 {
    type Vtable = ILineDisplayStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x600c3f1c_77ab_4968_a7de_c02ff169f2cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub StatisticsCategorySelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineDisplayStatisticsCategorySelector(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILineDisplayStatisticsCategorySelector {
    type Vtable = ILineDisplayStatisticsCategorySelector_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb521c46b_9274_4d24_94f3_b6017b832444);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayStatisticsCategorySelector_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AllStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub UnifiedPosStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ManufacturerStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineDisplayStatusUpdatedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILineDisplayStatusUpdatedEventArgs {
    type Vtable = ILineDisplayStatusUpdatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xddd57c1a_86fb_4eba_93d1_6f5eda52b752);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayStatusUpdatedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LineDisplayPowerStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineDisplayStoredBitmap(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILineDisplayStoredBitmap {
    type Vtable = ILineDisplayStoredBitmap_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf621515b_d81e_43ba_bf1b_bcfa3c785ba0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayStoredBitmap_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub EscapeSequence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub TryDeleteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryDeleteAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineDisplayWindow(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILineDisplayWindow {
    type Vtable = ILineDisplayWindow_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd21feef4_2364_4be5_bee1_851680af4964);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayWindow_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub SizeInCharacters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SizeInCharacters: usize,
    #[cfg(feature = "Foundation")]
    pub InterCharacterWaitInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    InterCharacterWaitInterval: usize,
    #[cfg(feature = "Foundation")]
    pub SetInterCharacterWaitInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetInterCharacterWaitInterval: usize,
    #[cfg(feature = "Foundation")]
    pub TryRefreshAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryRefreshAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryDisplayTextAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayattribute: LineDisplayTextAttribute, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryDisplayTextAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryDisplayTextAtPositionAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, displayattribute: LineDisplayTextAttribute, startposition: super::super::Foundation::Point, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryDisplayTextAtPositionAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryDisplayTextNormalAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryDisplayTextNormalAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryScrollTextAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, direction: LineDisplayScrollDirection, numberofcolumnsorrows: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryScrollTextAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryClearTextAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryClearTextAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILineDisplayWindow2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ILineDisplayWindow2 {
    type Vtable = ILineDisplayWindow2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa95ce2e6_bdd8_4365_8e11_de94de8dff02);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineDisplayWindow2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Cursor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Marquee: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReadCharacterAtCursorAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReadCharacterAtCursorAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryDisplayStoredBitmapAtCursorAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryDisplayStoredBitmapAtCursorAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub TryDisplayStorageFileBitmapAtCursorAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    TryDisplayStorageFileBitmapAtCursorAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub TryDisplayStorageFileBitmapAtCursorWithAlignmentAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    TryDisplayStorageFileBitmapAtCursorWithAlignmentAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub TryDisplayStorageFileBitmapAtCursorWithAlignmentAndWidthAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment, widthinpixels: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    TryDisplayStorageFileBitmapAtCursorWithAlignmentAndWidthAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub TryDisplayStorageFileBitmapAtPointAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, offsetinpixels: super::super::Foundation::Point, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    TryDisplayStorageFileBitmapAtPointAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub TryDisplayStorageFileBitmapAtPointWithWidthAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, offsetinpixels: super::super::Foundation::Point, widthinpixels: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    TryDisplayStorageFileBitmapAtPointWithWidthAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagneticStripeReader(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMagneticStripeReader {
    type Vtable = IMagneticStripeReader_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a92b015_47c3_468a_9333_0c6517574883);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReader_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Capabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SupportedCardTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u32) -> ::windows::core::HRESULT,
    pub DeviceAuthenticationProtocol: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MagneticStripeReaderAuthenticationProtocol) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CheckHealthAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, level: UnifiedPosHealthCheckLevel, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CheckHealthAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ClaimReaderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ClaimReaderAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub RetrieveStatisticsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statisticscategories: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    RetrieveStatisticsAsync: usize,
    pub GetErrorReportingType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MagneticStripeReaderErrorReportingType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub StatusUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StatusUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStatusUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStatusUpdated: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagneticStripeReaderAamvaCardDataReceivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMagneticStripeReaderAamvaCardDataReceivedEventArgs {
    type Vtable = IMagneticStripeReaderAamvaCardDataReceivedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a4bbd51_c316_4910_87f3_7a62ba862d31);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderAamvaCardDataReceivedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Report: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub LicenseNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ExpirationDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Restrictions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Class: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Endorsements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub BirthDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub FirstName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Surname: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Suffix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Gender: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub HairColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub EyeColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Height: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Weight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Address: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub City: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PostalCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagneticStripeReaderBankCardDataReceivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMagneticStripeReaderBankCardDataReceivedEventArgs {
    type Vtable = IMagneticStripeReaderBankCardDataReceivedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e958823_a31a_4763_882c_23725e39b08e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderBankCardDataReceivedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Report: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub AccountNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ExpirationDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ServiceCode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub FirstName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub MiddleInitial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Surname: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Suffix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagneticStripeReaderCapabilities(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMagneticStripeReaderCapabilities {
    type Vtable = IMagneticStripeReaderCapabilities_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7128809c_c440_44a2_a467_469175d02896);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderCapabilities_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CardAuthentication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SupportedEncryptionAlgorithms: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub AuthenticationLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MagneticStripeReaderAuthenticationLevel) -> ::windows::core::HRESULT,
    pub IsIsoSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsJisOneSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsJisTwoSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub PowerReportingType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UnifiedPosPowerReportingType) -> ::windows::core::HRESULT,
    pub IsStatisticsReportingSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsStatisticsUpdatingSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsTrackDataMaskingSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsTransmitSentinelsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagneticStripeReaderCardTypesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMagneticStripeReaderCardTypesStatics {
    type Vtable = IMagneticStripeReaderCardTypesStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x528f2c5d_2986_474f_8454_7ccd05928d5f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderCardTypesStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Unknown: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Bank: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Aamva: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub ExtendedBase: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagneticStripeReaderEncryptionAlgorithmsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMagneticStripeReaderEncryptionAlgorithmsStatics {
    type Vtable = IMagneticStripeReaderEncryptionAlgorithmsStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x53b57350_c3db_4754_9c00_41392374a109);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderEncryptionAlgorithmsStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub None: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub TripleDesDukpt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub ExtendedBase: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagneticStripeReaderErrorOccurredEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMagneticStripeReaderErrorOccurredEventArgs {
    type Vtable = IMagneticStripeReaderErrorOccurredEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1fedf95d_2c84_41ad_b778_f2356a789ab1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderErrorOccurredEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Track1Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MagneticStripeReaderTrackErrorType) -> ::windows::core::HRESULT,
    pub Track2Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MagneticStripeReaderTrackErrorType) -> ::windows::core::HRESULT,
    pub Track3Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MagneticStripeReaderTrackErrorType) -> ::windows::core::HRESULT,
    pub Track4Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MagneticStripeReaderTrackErrorType) -> ::windows::core::HRESULT,
    pub ErrorData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub PartialInputData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagneticStripeReaderReport(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMagneticStripeReaderReport {
    type Vtable = IMagneticStripeReaderReport_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a5b6047_99b0_4188_bef1_eddf79f78fe6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderReport_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CardType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Track1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Track2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Track3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Track4: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CardAuthenticationData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CardAuthenticationData: usize,
    pub CardAuthenticationDataLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub AdditionalSecurityInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    AdditionalSecurityInformation: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagneticStripeReaderStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMagneticStripeReaderStatics {
    type Vtable = IMagneticStripeReaderStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc45fab4a_efd7_4760_a5ce_15b0e47e94eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDefaultAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagneticStripeReaderStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMagneticStripeReaderStatics2 {
    type Vtable = IMagneticStripeReaderStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8cadc362_d667_48fa_86bc_f5ae1189262b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetDeviceSelectorWithConnectionTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectiontypes: PosConnectionTypes, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagneticStripeReaderStatusUpdatedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMagneticStripeReaderStatusUpdatedEventArgs {
    type Vtable = IMagneticStripeReaderStatusUpdatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09cc6bb0_3262_401d_9e8a_e80d6358906b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderStatusUpdatedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MagneticStripeReaderStatus) -> ::windows::core::HRESULT,
    pub ExtendedStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagneticStripeReaderTrackData(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMagneticStripeReaderTrackData {
    type Vtable = IMagneticStripeReaderTrackData_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x104cf671_4a9d_446e_abc5_20402307ba36);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderTrackData_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Data: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Data: usize,
    #[cfg(feature = "Storage_Streams")]
    pub DiscretionaryData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    DiscretionaryData: usize,
    #[cfg(feature = "Storage_Streams")]
    pub EncryptedData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    EncryptedData: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    type Vtable = IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf0a5514_59cc_4a60_99e8_99a53dace5aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Report: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPosPrinter(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPosPrinter {
    type Vtable = IPosPrinter_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a03c10e_9a19_4a01_994f_12dfad6adcbf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinter_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DeviceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Capabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedCharacterSets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedCharacterSets: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedTypeFaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedTypeFaces: usize,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ClaimPrinterAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ClaimPrinterAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CheckHealthAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, level: UnifiedPosHealthCheckLevel, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CheckHealthAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetStatisticsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statisticscategories: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetStatisticsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StatusUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StatusUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStatusUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStatusUpdated: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPosPrinter2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPosPrinter2 {
    type Vtable = IPosPrinter2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x248475e8_8b98_5517_8e48_760e86f68987);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinter2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SupportedBarcodeSymbologies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SupportedBarcodeSymbologies: usize,
    pub GetFontProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, typeface: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPosPrinterCapabilities(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPosPrinterCapabilities {
    type Vtable = IPosPrinterCapabilities_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcde95721_4380_4985_adc5_39db30cd93bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinterCapabilities_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub PowerReportingType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UnifiedPosPowerReportingType) -> ::windows::core::HRESULT,
    pub IsStatisticsReportingSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsStatisticsUpdatingSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub DefaultCharacterSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub HasCoverSensor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CanMapCharacterSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsTransactionSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Receipt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Slip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Journal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPosPrinterCharacterSetIdsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPosPrinterCharacterSetIdsStatics {
    type Vtable = IPosPrinterCharacterSetIdsStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c709eff_709a_4fe7_b215_06a748a38b39);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinterCharacterSetIdsStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Utf16LE: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Ascii: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Ansi: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPosPrinterFontProperty(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPosPrinterFontProperty {
    type Vtable = IPosPrinterFontProperty_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7f4e93a_f8ac_5f04_84d2_29b16d8a633c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinterFontProperty_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub TypeFace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsScalableToAnySize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CharacterSizes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CharacterSizes: usize,
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct IPosPrinterJob(::windows::core::IUnknown);
impl IPosPrinterJob {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Print<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, data: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Print)(::core::mem::transmute_copy(this), data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn PrintLine<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, data: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).PrintLine)(::core::mem::transmute_copy(this), data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn PrintNewline(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).PrintNewline)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ExecuteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExecuteAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
impl ::core::convert::From<IPosPrinterJob> for ::windows::core::IUnknown {
    fn from(value: IPosPrinterJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPosPrinterJob> for ::windows::core::IUnknown {
    fn from(value: &IPosPrinterJob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPosPrinterJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPosPrinterJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IPosPrinterJob> for ::windows::core::IInspectable {
    fn from(value: IPosPrinterJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPosPrinterJob> for ::windows::core::IInspectable {
    fn from(value: &IPosPrinterJob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IPosPrinterJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IPosPrinterJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPosPrinterJob {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPosPrinterJob {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPosPrinterJob {}
impl ::core::fmt::Debug for IPosPrinterJob {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPosPrinterJob").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IPosPrinterJob {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{9a94005c-0615-4591-a58f-30f87edfe2e4}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IPosPrinterJob {
    type Vtable = IPosPrinterJob_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a94005c_0615_4591_a58f_30f87edfe2e4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinterJob_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Print: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PrintLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PrintNewline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ExecuteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ExecuteAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPosPrinterPrintOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPosPrinterPrintOptions {
    type Vtable = IPosPrinterPrintOptions_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a2e16fd_1d02_5a58_9d59_bfcde76fde86);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinterPrintOptions_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub TypeFace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetTypeFace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub CharacterHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetCharacterHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub Bold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetBold: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Italic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetItalic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Underline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetUnderline: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub ReverseVideo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetReverseVideo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Strikethrough: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetStrikethrough: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Superscript: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetSuperscript: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Subscript: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetSubscript: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub DoubleWide: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetDoubleWide: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub DoubleHigh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetDoubleHigh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Alignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterAlignment) -> ::windows::core::HRESULT,
    pub SetAlignment: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PosPrinterAlignment) -> ::windows::core::HRESULT,
    pub CharacterSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetCharacterSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPosPrinterReleaseDeviceRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPosPrinterReleaseDeviceRequestedEventArgs {
    type Vtable = IPosPrinterReleaseDeviceRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2bcba359_1cef_40b2_9ecb_f927f856ae3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinterReleaseDeviceRequestedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPosPrinterStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPosPrinterStatics {
    type Vtable = IPosPrinterStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8ce0d4ea_132f_4cdf_a64a_2d0d7c96a85b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinterStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub GetDefaultAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDefaultAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, deviceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    pub GetDeviceSelector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPosPrinterStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPosPrinterStatics2 {
    type Vtable = IPosPrinterStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeecd2c1c_b0d0_42e7_b137_b89b16244d41);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinterStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetDeviceSelectorWithConnectionTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectiontypes: PosConnectionTypes, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPosPrinterStatus(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPosPrinterStatus {
    type Vtable = IPosPrinterStatus_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd1f0c730_da40_4328_bf76_5156fa33b747);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinterStatus_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub StatusKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterStatusKind) -> ::windows::core::HRESULT,
    pub ExtendedStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPosPrinterStatusUpdatedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPosPrinterStatusUpdatedEventArgs {
    type Vtable = IPosPrinterStatusUpdatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2edb87df_13a6_428d_ba81_b0e7c3e5a3cd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPosPrinterStatusUpdatedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct IReceiptOrSlipJob(::windows::core::IUnknown);
impl IReceiptOrSlipJob {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetBarcodeRotation(&self, value: PosPrinterRotation) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBarcodeRotation)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetPrintRotation(&self, value: PosPrinterRotation, includebitmaps: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPrintRotation)(::core::mem::transmute_copy(this), value, includebitmaps).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPrintArea<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPrintArea)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetBitmap<'a, Param1: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmapnumber: u32, bitmap: Param1, alignment: PosPrinterAlignment) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBitmap)(::core::mem::transmute_copy(this), bitmapnumber, bitmap.into_param().abi(), alignment).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetBitmapCustomWidthStandardAlign<'a, Param1: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmapnumber: u32, bitmap: Param1, alignment: PosPrinterAlignment, width: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBitmapCustomWidthStandardAlign)(::core::mem::transmute_copy(this), bitmapnumber, bitmap.into_param().abi(), alignment, width).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetCustomAlignedBitmap<'a, Param1: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmapnumber: u32, bitmap: Param1, alignmentdistance: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCustomAlignedBitmap)(::core::mem::transmute_copy(this), bitmapnumber, bitmap.into_param().abi(), alignmentdistance).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetBitmapCustomWidthCustomAlign<'a, Param1: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmapnumber: u32, bitmap: Param1, alignmentdistance: u32, width: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBitmapCustomWidthCustomAlign)(::core::mem::transmute_copy(this), bitmapnumber, bitmap.into_param().abi(), alignmentdistance, width).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn PrintSavedBitmap(&self, bitmapnumber: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).PrintSavedBitmap)(::core::mem::transmute_copy(this), bitmapnumber).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn DrawRuledLine<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, positionlist: Param0, linedirection: PosPrinterLineDirection, linewidth: u32, linestyle: PosPrinterLineStyle, linecolor: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).DrawRuledLine)(::core::mem::transmute_copy(this), positionlist.into_param().abi(), linedirection, linewidth, linestyle, linecolor).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn PrintBarcode<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, data: Param0, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignment: PosPrinterAlignment) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).PrintBarcode)(::core::mem::transmute_copy(this), data.into_param().abi(), symbology, height, width, textposition, alignment).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn PrintBarcodeCustomAlign<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, data: Param0, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignmentdistance: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).PrintBarcodeCustomAlign)(::core::mem::transmute_copy(this), data.into_param().abi(), symbology, height, width, textposition, alignmentdistance).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn PrintBitmap<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmap: Param0, alignment: PosPrinterAlignment) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).PrintBitmap)(::core::mem::transmute_copy(this), bitmap.into_param().abi(), alignment).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn PrintBitmapCustomWidthStandardAlign<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmap: Param0, alignment: PosPrinterAlignment, width: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).PrintBitmapCustomWidthStandardAlign)(::core::mem::transmute_copy(this), bitmap.into_param().abi(), alignment, width).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn PrintCustomAlignedBitmap<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmap: Param0, alignmentdistance: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).PrintCustomAlignedBitmap)(::core::mem::transmute_copy(this), bitmap.into_param().abi(), alignmentdistance).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn PrintBitmapCustomWidthCustomAlign<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmap: Param0, alignmentdistance: u32, width: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).PrintBitmapCustomWidthCustomAlign)(::core::mem::transmute_copy(this), bitmap.into_param().abi(), alignmentdistance, width).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Print<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, data: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Print)(::core::mem::transmute_copy(this), data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn PrintLine<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, data: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).PrintLine)(::core::mem::transmute_copy(this), data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn PrintNewline(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).PrintNewline)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ExecuteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExecuteAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
impl ::core::convert::From<IReceiptOrSlipJob> for ::windows::core::IUnknown {
    fn from(value: IReceiptOrSlipJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IReceiptOrSlipJob> for ::windows::core::IUnknown {
    fn from(value: &IReceiptOrSlipJob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IReceiptOrSlipJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IReceiptOrSlipJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IReceiptOrSlipJob> for ::windows::core::IInspectable {
    fn from(value: IReceiptOrSlipJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IReceiptOrSlipJob> for ::windows::core::IInspectable {
    fn from(value: &IReceiptOrSlipJob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IReceiptOrSlipJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IReceiptOrSlipJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl ::core::clone::Clone for IReceiptOrSlipJob {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IReceiptOrSlipJob {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IReceiptOrSlipJob {}
impl ::core::fmt::Debug for IReceiptOrSlipJob {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IReceiptOrSlipJob").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IReceiptOrSlipJob {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{532199be-c8c3-4dc2-89e9-5c4a37b34ddc}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IReceiptOrSlipJob {
    type Vtable = IReceiptOrSlipJob_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x532199be_c8c3_4dc2_89e9_5c4a37b34ddc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReceiptOrSlipJob_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SetBarcodeRotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PosPrinterRotation) -> ::windows::core::HRESULT,
    pub SetPrintRotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PosPrinterRotation, includebitmaps: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SetPrintArea: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPrintArea: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmapnumber: u32, bitmap: ::windows::core::RawPtr, alignment: PosPrinterAlignment) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetBitmap: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetBitmapCustomWidthStandardAlign: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmapnumber: u32, bitmap: ::windows::core::RawPtr, alignment: PosPrinterAlignment, width: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetBitmapCustomWidthStandardAlign: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetCustomAlignedBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmapnumber: u32, bitmap: ::windows::core::RawPtr, alignmentdistance: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetCustomAlignedBitmap: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetBitmapCustomWidthCustomAlign: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmapnumber: u32, bitmap: ::windows::core::RawPtr, alignmentdistance: u32, width: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetBitmapCustomWidthCustomAlign: usize,
    pub PrintSavedBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmapnumber: u32) -> ::windows::core::HRESULT,
    pub DrawRuledLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, positionlist: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, linedirection: PosPrinterLineDirection, linewidth: u32, linestyle: PosPrinterLineStyle, linecolor: u32) -> ::windows::core::HRESULT,
    pub PrintBarcode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignment: PosPrinterAlignment) -> ::windows::core::HRESULT,
    pub PrintBarcodeCustomAlign: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignmentdistance: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Graphics_Imaging")]
    pub PrintBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, alignment: PosPrinterAlignment) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    PrintBitmap: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub PrintBitmapCustomWidthStandardAlign: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, alignment: PosPrinterAlignment, width: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    PrintBitmapCustomWidthStandardAlign: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub PrintCustomAlignedBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, alignmentdistance: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    PrintCustomAlignedBitmap: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub PrintBitmapCustomWidthCustomAlign: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: ::windows::core::RawPtr, alignmentdistance: u32, width: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    PrintBitmapCustomWidthCustomAlign: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IReceiptPrintJob(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IReceiptPrintJob {
    type Vtable = IReceiptPrintJob_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa96066e_acad_4b79_9d0f_c0cfc08dc77b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReceiptPrintJob_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub MarkFeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, kind: PosPrinterMarkFeedKind) -> ::windows::core::HRESULT,
    pub CutPaper: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, percentage: f64) -> ::windows::core::HRESULT,
    pub CutPaperDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IReceiptPrintJob2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IReceiptPrintJob2 {
    type Vtable = IReceiptPrintJob2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cbc12e3_9e29_5179_bcd8_1811d3b9a10e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReceiptPrintJob2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub StampPaper: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Print: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, printoptions: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub FeedPaperByLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, linecount: i32) -> ::windows::core::HRESULT,
    pub FeedPaperByMapModeUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, distance: i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IReceiptPrinterCapabilities(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IReceiptPrinterCapabilities {
    type Vtable = IReceiptPrinterCapabilities_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb8f0b58f_51a8_43fc_9bd5_8de272a6415b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReceiptPrinterCapabilities_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CanCutPaper: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsStampSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub MarkFeedCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PosPrinterMarkFeedCapabilities) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IReceiptPrinterCapabilities2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IReceiptPrinterCapabilities2 {
    type Vtable = IReceiptPrinterCapabilities2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x20030638_8a2c_55ac_9a7b_7576d8869e99);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReceiptPrinterCapabilities2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsReverseVideoSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsStrikethroughSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsSuperscriptSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsSubscriptSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsReversePaperFeedByLineSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsReversePaperFeedByMapModeUnitSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISlipPrintJob(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISlipPrintJob {
    type Vtable = ISlipPrintJob_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5d88f95d_6131_5a4b_b7d5_8ef2da7b4165);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISlipPrintJob_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Print: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, printoptions: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub FeedPaperByLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, linecount: i32) -> ::windows::core::HRESULT,
    pub FeedPaperByMapModeUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, distance: i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISlipPrinterCapabilities(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISlipPrinterCapabilities {
    type Vtable = ISlipPrinterCapabilities_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x99b16399_488c_4157_8ac2_9f57f708d3db);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISlipPrinterCapabilities_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsFullLengthSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsBothSidesPrintingSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISlipPrinterCapabilities2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISlipPrinterCapabilities2 {
    type Vtable = ISlipPrinterCapabilities2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6ff89671_2d1a_5000_87c2_b0851bfdf07e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISlipPrinterCapabilities2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsReverseVideoSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsStrikethroughSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsSuperscriptSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsSubscriptSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsReversePaperFeedByLineSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsReversePaperFeedByMapModeUnitSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUnifiedPosErrorData(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUnifiedPosErrorData {
    type Vtable = IUnifiedPosErrorData_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2b998c3a_555c_4889_8ed8_c599bb3a712a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnifiedPosErrorData_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Severity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UnifiedPosErrorSeverity) -> ::windows::core::HRESULT,
    pub Reason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UnifiedPosErrorReason) -> ::windows::core::HRESULT,
    pub ExtendedReason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUnifiedPosErrorDataFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUnifiedPosErrorDataFactory {
    type Vtable = IUnifiedPosErrorDataFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b982551_1ffe_451b_a368_63e0ce465f5a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnifiedPosErrorDataFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, severity: UnifiedPosErrorSeverity, reason: UnifiedPosErrorReason, extendedreason: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct JournalPrintJob(::windows::core::IUnknown);
impl JournalPrintJob {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Print<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, PosPrinterPrintOptions>>(&self, data: Param0, printoptions: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IJournalPrintJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Print)(::core::mem::transmute_copy(this), data.into_param().abi(), printoptions.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn FeedPaperByLine(&self, linecount: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IJournalPrintJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).FeedPaperByLine)(::core::mem::transmute_copy(this), linecount).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn FeedPaperByMapModeUnit(&self, distance: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IJournalPrintJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).FeedPaperByMapModeUnit)(::core::mem::transmute_copy(this), distance).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Print2<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, data: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Print)(::core::mem::transmute_copy(this), data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn PrintLine<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, data: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).PrintLine)(::core::mem::transmute_copy(this), data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn PrintNewline(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).PrintNewline)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ExecuteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExecuteAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
impl ::core::clone::Clone for JournalPrintJob {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for JournalPrintJob {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for JournalPrintJob {}
impl ::core::fmt::Debug for JournalPrintJob {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JournalPrintJob").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for JournalPrintJob {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.JournalPrintJob;{9a94005c-0615-4591-a58f-30f87edfe2e4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for JournalPrintJob {
    type Vtable = IPosPrinterJob_Vtbl;
    const IID: ::windows::core::GUID = <IPosPrinterJob as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for JournalPrintJob {
    const NAME: &'static str = "Windows.Devices.PointOfService.JournalPrintJob";
}
impl ::core::convert::From<JournalPrintJob> for ::windows::core::IUnknown {
    fn from(value: JournalPrintJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&JournalPrintJob> for ::windows::core::IUnknown {
    fn from(value: &JournalPrintJob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for JournalPrintJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a JournalPrintJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<JournalPrintJob> for ::windows::core::IInspectable {
    fn from(value: JournalPrintJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&JournalPrintJob> for ::windows::core::IInspectable {
    fn from(value: &JournalPrintJob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for JournalPrintJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a JournalPrintJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<JournalPrintJob> for IPosPrinterJob {
    type Error = ::windows::core::Error;
    fn try_from(value: JournalPrintJob) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&JournalPrintJob> for IPosPrinterJob {
    type Error = ::windows::core::Error;
    fn try_from(value: &JournalPrintJob) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPosPrinterJob> for JournalPrintJob {
    fn into_param(self) -> ::windows::core::Param<'a, IPosPrinterJob> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPosPrinterJob> for &JournalPrintJob {
    fn into_param(self) -> ::windows::core::Param<'a, IPosPrinterJob> {
        ::core::convert::TryInto::<IPosPrinterJob>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for JournalPrintJob {}
unsafe impl ::core::marker::Sync for JournalPrintJob {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct JournalPrinterCapabilities(::windows::core::IUnknown);
impl JournalPrinterCapabilities {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsPrinterPresent(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsPrinterPresent)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsDualColorSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDualColorSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn ColorCartridgeCapabilities(&self) -> ::windows::core::Result<PosPrinterColorCapabilities> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: PosPrinterColorCapabilities = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ColorCartridgeCapabilities)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterColorCapabilities>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn CartridgeSensors(&self) -> ::windows::core::Result<PosPrinterCartridgeSensors> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: PosPrinterCartridgeSensors = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CartridgeSensors)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterCartridgeSensors>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsBoldSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsBoldSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsItalicSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsItalicSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsUnderlineSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsUnderlineSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsDoubleHighPrintSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDoubleHighPrintSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsDoubleWidePrintSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDoubleWidePrintSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsDoubleHighDoubleWidePrintSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDoubleHighDoubleWidePrintSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsPaperEmptySensorSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsPaperEmptySensorSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsPaperNearEndSensorSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsPaperNearEndSensorSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedCharactersPerLine(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SupportedCharactersPerLine)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsReverseVideoSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IJournalPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsReverseVideoSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsStrikethroughSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IJournalPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsStrikethroughSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsSuperscriptSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IJournalPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsSuperscriptSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsSubscriptSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IJournalPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsSubscriptSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsReversePaperFeedByLineSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IJournalPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsReversePaperFeedByLineSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsReversePaperFeedByMapModeUnitSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IJournalPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsReversePaperFeedByMapModeUnitSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for JournalPrinterCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for JournalPrinterCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for JournalPrinterCapabilities {}
impl ::core::fmt::Debug for JournalPrinterCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JournalPrinterCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for JournalPrinterCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.JournalPrinterCapabilities;{3b5ccc43-e047-4463-bb58-17b5ba1d8056})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for JournalPrinterCapabilities {
    type Vtable = IJournalPrinterCapabilities_Vtbl;
    const IID: ::windows::core::GUID = <IJournalPrinterCapabilities as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for JournalPrinterCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.JournalPrinterCapabilities";
}
impl ::core::convert::From<JournalPrinterCapabilities> for ::windows::core::IUnknown {
    fn from(value: JournalPrinterCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&JournalPrinterCapabilities> for ::windows::core::IUnknown {
    fn from(value: &JournalPrinterCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for JournalPrinterCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a JournalPrinterCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<JournalPrinterCapabilities> for ::windows::core::IInspectable {
    fn from(value: JournalPrinterCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&JournalPrinterCapabilities> for ::windows::core::IInspectable {
    fn from(value: &JournalPrinterCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for JournalPrinterCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a JournalPrinterCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct LineDisplay(::windows::core::IUnknown);
impl LineDisplay {
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Capabilities(&self) -> ::windows::core::Result<LineDisplayCapabilities> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Capabilities)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LineDisplayCapabilities>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn PhysicalDeviceName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PhysicalDeviceName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn PhysicalDeviceDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PhysicalDeviceDescription)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn DeviceControlDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceControlDescription)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn DeviceControlVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceControlVersion)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn DeviceServiceVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceServiceVersion)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ClaimAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ClaimedLineDisplay>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ClaimAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ClaimedLineDisplay>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CheckPowerStatusAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LineDisplayPowerStatus>> {
        let this = &::windows::core::Interface::cast::<ILineDisplay2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CheckPowerStatusAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LineDisplayPowerStatus>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LineDisplay>> {
        Self::ILineDisplayStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FromIdAsync)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LineDisplay>>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDefaultAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LineDisplay>> {
        Self::ILineDisplayStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDefaultAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<LineDisplay>>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ILineDisplayStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeviceSelector)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn GetDeviceSelectorWithConnectionTypes(connectiontypes: PosConnectionTypes) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::ILineDisplayStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeviceSelectorWithConnectionTypes)(::core::mem::transmute_copy(this), connectiontypes, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn StatisticsCategorySelector() -> ::windows::core::Result<LineDisplayStatisticsCategorySelector> {
        Self::ILineDisplayStatics2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StatisticsCategorySelector)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LineDisplayStatisticsCategorySelector>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILineDisplayStatics<R, F: FnOnce(&ILineDisplayStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LineDisplay, ILineDisplayStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn ILineDisplayStatics2<R, F: FnOnce(&ILineDisplayStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<LineDisplay, ILineDisplayStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for LineDisplay {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LineDisplay {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LineDisplay {}
impl ::core::fmt::Debug for LineDisplay {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplay").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LineDisplay {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplay;{24f5df4e-3c99-44e2-b73f-e51be3637a8c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LineDisplay {
    type Vtable = ILineDisplay_Vtbl;
    const IID: ::windows::core::GUID = <ILineDisplay as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LineDisplay {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplay";
}
impl ::core::convert::From<LineDisplay> for ::windows::core::IUnknown {
    fn from(value: LineDisplay) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LineDisplay> for ::windows::core::IUnknown {
    fn from(value: &LineDisplay) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LineDisplay {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LineDisplay {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LineDisplay> for ::windows::core::IInspectable {
    fn from(value: LineDisplay) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LineDisplay> for ::windows::core::IInspectable {
    fn from(value: &LineDisplay) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LineDisplay {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LineDisplay {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct LineDisplayAttributes(::windows::core::IUnknown);
impl LineDisplayAttributes {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsPowerNotifyEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsPowerNotifyEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetIsPowerNotifyEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsPowerNotifyEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Brightness(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Brightness)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetBrightness(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBrightness)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BlinkRate(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BlinkRate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetBlinkRate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBlinkRate)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ScreenSizeInCharacters(&self) -> ::windows::core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Size = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ScreenSizeInCharacters)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetScreenSizeInCharacters<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Size>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetScreenSizeInCharacters)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn CharacterSet(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CharacterSet)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetCharacterSet(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCharacterSet)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsCharacterSetMappingEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCharacterSetMappingEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetIsCharacterSetMappingEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsCharacterSetMappingEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn CurrentWindow(&self) -> ::windows::core::Result<LineDisplayWindow> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CurrentWindow)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LineDisplayWindow>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetCurrentWindow<'a, Param0: ::windows::core::IntoParam<'a, LineDisplayWindow>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCurrentWindow)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for LineDisplayAttributes {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LineDisplayAttributes {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LineDisplayAttributes {}
impl ::core::fmt::Debug for LineDisplayAttributes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayAttributes").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LineDisplayAttributes {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplayAttributes;{c17de99c-229a-4c14-a6f1-b4e4b1fead92})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LineDisplayAttributes {
    type Vtable = ILineDisplayAttributes_Vtbl;
    const IID: ::windows::core::GUID = <ILineDisplayAttributes as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LineDisplayAttributes {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayAttributes";
}
impl ::core::convert::From<LineDisplayAttributes> for ::windows::core::IUnknown {
    fn from(value: LineDisplayAttributes) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LineDisplayAttributes> for ::windows::core::IUnknown {
    fn from(value: &LineDisplayAttributes) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LineDisplayAttributes {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LineDisplayAttributes {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LineDisplayAttributes> for ::windows::core::IInspectable {
    fn from(value: LineDisplayAttributes) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LineDisplayAttributes> for ::windows::core::IInspectable {
    fn from(value: &LineDisplayAttributes) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LineDisplayAttributes {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LineDisplayAttributes {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LineDisplayAttributes {}
unsafe impl ::core::marker::Sync for LineDisplayAttributes {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct LineDisplayCapabilities(::windows::core::IUnknown);
impl LineDisplayCapabilities {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsStatisticsReportingSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsStatisticsReportingSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsStatisticsUpdatingSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsStatisticsUpdatingSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn PowerReportingType(&self) -> ::windows::core::Result<UnifiedPosPowerReportingType> {
        let this = self;
        unsafe {
            let mut result__: UnifiedPosPowerReportingType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PowerReportingType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UnifiedPosPowerReportingType>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn CanChangeScreenSize(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanChangeScreenSize)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn CanDisplayBitmaps(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanDisplayBitmaps)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn CanReadCharacterAtCursor(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanReadCharacterAtCursor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn CanMapCharacterSets(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanMapCharacterSets)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn CanDisplayCustomGlyphs(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanDisplayCustomGlyphs)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn CanReverse(&self) -> ::windows::core::Result<LineDisplayTextAttributeGranularity> {
        let this = self;
        unsafe {
            let mut result__: LineDisplayTextAttributeGranularity = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanReverse)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LineDisplayTextAttributeGranularity>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn CanBlink(&self) -> ::windows::core::Result<LineDisplayTextAttributeGranularity> {
        let this = self;
        unsafe {
            let mut result__: LineDisplayTextAttributeGranularity = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanBlink)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LineDisplayTextAttributeGranularity>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn CanChangeBlinkRate(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanChangeBlinkRate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsBrightnessSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsBrightnessSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsCursorSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCursorSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsHorizontalMarqueeSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsHorizontalMarqueeSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsVerticalMarqueeSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsVerticalMarqueeSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsInterCharacterWaitSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsInterCharacterWaitSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SupportedDescriptors(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SupportedDescriptors)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SupportedWindows(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SupportedWindows)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for LineDisplayCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LineDisplayCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LineDisplayCapabilities {}
impl ::core::fmt::Debug for LineDisplayCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LineDisplayCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplayCapabilities;{5a15b5d1-8dc5-4b9c-9172-303e47b70c55})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LineDisplayCapabilities {
    type Vtable = ILineDisplayCapabilities_Vtbl;
    const IID: ::windows::core::GUID = <ILineDisplayCapabilities as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LineDisplayCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayCapabilities";
}
impl ::core::convert::From<LineDisplayCapabilities> for ::windows::core::IUnknown {
    fn from(value: LineDisplayCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LineDisplayCapabilities> for ::windows::core::IUnknown {
    fn from(value: &LineDisplayCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LineDisplayCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LineDisplayCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LineDisplayCapabilities> for ::windows::core::IInspectable {
    fn from(value: LineDisplayCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LineDisplayCapabilities> for ::windows::core::IInspectable {
    fn from(value: &LineDisplayCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LineDisplayCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LineDisplayCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LineDisplayCapabilities {}
unsafe impl ::core::marker::Sync for LineDisplayCapabilities {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct LineDisplayCursor(::windows::core::IUnknown);
impl LineDisplayCursor {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn CanCustomize(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanCustomize)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsBlinkSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsBlinkSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsBlockSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsBlockSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsHalfBlockSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsHalfBlockSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsUnderlineSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsUnderlineSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsReverseSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsReverseSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsOtherSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsOtherSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn GetAttributes(&self) -> ::windows::core::Result<LineDisplayCursorAttributes> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAttributes)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LineDisplayCursorAttributes>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryUpdateAttributesAsync<'a, Param0: ::windows::core::IntoParam<'a, LineDisplayCursorAttributes>>(&self, attributes: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryUpdateAttributesAsync)(::core::mem::transmute_copy(this), attributes.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
impl ::core::clone::Clone for LineDisplayCursor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LineDisplayCursor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LineDisplayCursor {}
impl ::core::fmt::Debug for LineDisplayCursor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayCursor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LineDisplayCursor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplayCursor;{ecdffc45-754a-4e3b-ab2b-151181085605})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LineDisplayCursor {
    type Vtable = ILineDisplayCursor_Vtbl;
    const IID: ::windows::core::GUID = <ILineDisplayCursor as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LineDisplayCursor {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayCursor";
}
impl ::core::convert::From<LineDisplayCursor> for ::windows::core::IUnknown {
    fn from(value: LineDisplayCursor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LineDisplayCursor> for ::windows::core::IUnknown {
    fn from(value: &LineDisplayCursor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LineDisplayCursor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LineDisplayCursor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LineDisplayCursor> for ::windows::core::IInspectable {
    fn from(value: LineDisplayCursor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LineDisplayCursor> for ::windows::core::IInspectable {
    fn from(value: &LineDisplayCursor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LineDisplayCursor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LineDisplayCursor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LineDisplayCursor {}
unsafe impl ::core::marker::Sync for LineDisplayCursor {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct LineDisplayCursorAttributes(::windows::core::IUnknown);
impl LineDisplayCursorAttributes {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsBlinkEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsBlinkEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetIsBlinkEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsBlinkEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn CursorType(&self) -> ::windows::core::Result<LineDisplayCursorType> {
        let this = self;
        unsafe {
            let mut result__: LineDisplayCursorType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CursorType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LineDisplayCursorType>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetCursorType(&self, value: LineDisplayCursorType) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCursorType)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsAutoAdvanceEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsAutoAdvanceEnabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetIsAutoAdvanceEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsAutoAdvanceEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Position)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPosition<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Point>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPosition)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for LineDisplayCursorAttributes {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LineDisplayCursorAttributes {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LineDisplayCursorAttributes {}
impl ::core::fmt::Debug for LineDisplayCursorAttributes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayCursorAttributes").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LineDisplayCursorAttributes {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplayCursorAttributes;{4e2d54fe-4ffd-4190-aae1-ce285f20c896})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LineDisplayCursorAttributes {
    type Vtable = ILineDisplayCursorAttributes_Vtbl;
    const IID: ::windows::core::GUID = <ILineDisplayCursorAttributes as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LineDisplayCursorAttributes {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayCursorAttributes";
}
impl ::core::convert::From<LineDisplayCursorAttributes> for ::windows::core::IUnknown {
    fn from(value: LineDisplayCursorAttributes) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LineDisplayCursorAttributes> for ::windows::core::IUnknown {
    fn from(value: &LineDisplayCursorAttributes) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LineDisplayCursorAttributes {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LineDisplayCursorAttributes {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LineDisplayCursorAttributes> for ::windows::core::IInspectable {
    fn from(value: LineDisplayCursorAttributes) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LineDisplayCursorAttributes> for ::windows::core::IInspectable {
    fn from(value: &LineDisplayCursorAttributes) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LineDisplayCursorAttributes {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LineDisplayCursorAttributes {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LineDisplayCursorAttributes {}
unsafe impl ::core::marker::Sync for LineDisplayCursorAttributes {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LineDisplayCursorType(pub i32);
impl LineDisplayCursorType {
    pub const None: Self = Self(0i32);
    pub const Block: Self = Self(1i32);
    pub const HalfBlock: Self = Self(2i32);
    pub const Underline: Self = Self(3i32);
    pub const Reverse: Self = Self(4i32);
    pub const Other: Self = Self(5i32);
}
impl ::core::marker::Copy for LineDisplayCursorType {}
impl ::core::clone::Clone for LineDisplayCursorType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LineDisplayCursorType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for LineDisplayCursorType {
    type Abi = Self;
}
impl ::core::fmt::Debug for LineDisplayCursorType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayCursorType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LineDisplayCursorType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.LineDisplayCursorType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct LineDisplayCustomGlyphs(::windows::core::IUnknown);
impl LineDisplayCustomGlyphs {
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SizeInPixels(&self) -> ::windows::core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Size = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SizeInPixels)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedGlyphCodes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SupportedGlyphCodes)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn TryRedefineAsync<'a, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, glyphcode: u32, glyphdata: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryRedefineAsync)(::core::mem::transmute_copy(this), glyphcode, glyphdata.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
impl ::core::clone::Clone for LineDisplayCustomGlyphs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LineDisplayCustomGlyphs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LineDisplayCustomGlyphs {}
impl ::core::fmt::Debug for LineDisplayCustomGlyphs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayCustomGlyphs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LineDisplayCustomGlyphs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplayCustomGlyphs;{2257f63c-f263-44f1-a1a0-e750a6a0ec54})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LineDisplayCustomGlyphs {
    type Vtable = ILineDisplayCustomGlyphs_Vtbl;
    const IID: ::windows::core::GUID = <ILineDisplayCustomGlyphs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LineDisplayCustomGlyphs {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayCustomGlyphs";
}
impl ::core::convert::From<LineDisplayCustomGlyphs> for ::windows::core::IUnknown {
    fn from(value: LineDisplayCustomGlyphs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LineDisplayCustomGlyphs> for ::windows::core::IUnknown {
    fn from(value: &LineDisplayCustomGlyphs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LineDisplayCustomGlyphs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LineDisplayCustomGlyphs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LineDisplayCustomGlyphs> for ::windows::core::IInspectable {
    fn from(value: LineDisplayCustomGlyphs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LineDisplayCustomGlyphs> for ::windows::core::IInspectable {
    fn from(value: &LineDisplayCustomGlyphs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LineDisplayCustomGlyphs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LineDisplayCustomGlyphs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LineDisplayCustomGlyphs {}
unsafe impl ::core::marker::Sync for LineDisplayCustomGlyphs {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LineDisplayDescriptorState(pub i32);
impl LineDisplayDescriptorState {
    pub const Off: Self = Self(0i32);
    pub const On: Self = Self(1i32);
    pub const Blink: Self = Self(2i32);
}
impl ::core::marker::Copy for LineDisplayDescriptorState {}
impl ::core::clone::Clone for LineDisplayDescriptorState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LineDisplayDescriptorState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for LineDisplayDescriptorState {
    type Abi = Self;
}
impl ::core::fmt::Debug for LineDisplayDescriptorState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayDescriptorState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LineDisplayDescriptorState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.LineDisplayDescriptorState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LineDisplayHorizontalAlignment(pub i32);
impl LineDisplayHorizontalAlignment {
    pub const Left: Self = Self(0i32);
    pub const Center: Self = Self(1i32);
    pub const Right: Self = Self(2i32);
}
impl ::core::marker::Copy for LineDisplayHorizontalAlignment {}
impl ::core::clone::Clone for LineDisplayHorizontalAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LineDisplayHorizontalAlignment {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for LineDisplayHorizontalAlignment {
    type Abi = Self;
}
impl ::core::fmt::Debug for LineDisplayHorizontalAlignment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayHorizontalAlignment").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LineDisplayHorizontalAlignment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.LineDisplayHorizontalAlignment;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct LineDisplayMarquee(::windows::core::IUnknown);
impl LineDisplayMarquee {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Format(&self) -> ::windows::core::Result<LineDisplayMarqueeFormat> {
        let this = self;
        unsafe {
            let mut result__: LineDisplayMarqueeFormat = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Format)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LineDisplayMarqueeFormat>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetFormat(&self, value: LineDisplayMarqueeFormat) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetFormat)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RepeatWaitInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RepeatWaitInterval)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetRepeatWaitInterval<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRepeatWaitInterval)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ScrollWaitInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ScrollWaitInterval)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetScrollWaitInterval<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetScrollWaitInterval)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryStartScrollingAsync(&self, direction: LineDisplayScrollDirection) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryStartScrollingAsync)(::core::mem::transmute_copy(this), direction, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryStopScrollingAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryStopScrollingAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
impl ::core::clone::Clone for LineDisplayMarquee {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LineDisplayMarquee {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LineDisplayMarquee {}
impl ::core::fmt::Debug for LineDisplayMarquee {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayMarquee").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LineDisplayMarquee {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplayMarquee;{a3d33e3e-f46a-4b7a-bc21-53eb3b57f8b4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LineDisplayMarquee {
    type Vtable = ILineDisplayMarquee_Vtbl;
    const IID: ::windows::core::GUID = <ILineDisplayMarquee as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LineDisplayMarquee {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayMarquee";
}
impl ::core::convert::From<LineDisplayMarquee> for ::windows::core::IUnknown {
    fn from(value: LineDisplayMarquee) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LineDisplayMarquee> for ::windows::core::IUnknown {
    fn from(value: &LineDisplayMarquee) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LineDisplayMarquee {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LineDisplayMarquee {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LineDisplayMarquee> for ::windows::core::IInspectable {
    fn from(value: LineDisplayMarquee) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LineDisplayMarquee> for ::windows::core::IInspectable {
    fn from(value: &LineDisplayMarquee) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LineDisplayMarquee {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LineDisplayMarquee {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LineDisplayMarquee {}
unsafe impl ::core::marker::Sync for LineDisplayMarquee {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LineDisplayMarqueeFormat(pub i32);
impl LineDisplayMarqueeFormat {
    pub const None: Self = Self(0i32);
    pub const Walk: Self = Self(1i32);
    pub const Place: Self = Self(2i32);
}
impl ::core::marker::Copy for LineDisplayMarqueeFormat {}
impl ::core::clone::Clone for LineDisplayMarqueeFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LineDisplayMarqueeFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for LineDisplayMarqueeFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for LineDisplayMarqueeFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayMarqueeFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LineDisplayMarqueeFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.LineDisplayMarqueeFormat;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LineDisplayPowerStatus(pub i32);
impl LineDisplayPowerStatus {
    pub const Unknown: Self = Self(0i32);
    pub const Online: Self = Self(1i32);
    pub const Off: Self = Self(2i32);
    pub const Offline: Self = Self(3i32);
    pub const OffOrOffline: Self = Self(4i32);
}
impl ::core::marker::Copy for LineDisplayPowerStatus {}
impl ::core::clone::Clone for LineDisplayPowerStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LineDisplayPowerStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for LineDisplayPowerStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for LineDisplayPowerStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayPowerStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LineDisplayPowerStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.LineDisplayPowerStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LineDisplayScrollDirection(pub i32);
impl LineDisplayScrollDirection {
    pub const Up: Self = Self(0i32);
    pub const Down: Self = Self(1i32);
    pub const Left: Self = Self(2i32);
    pub const Right: Self = Self(3i32);
}
impl ::core::marker::Copy for LineDisplayScrollDirection {}
impl ::core::clone::Clone for LineDisplayScrollDirection {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LineDisplayScrollDirection {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for LineDisplayScrollDirection {
    type Abi = Self;
}
impl ::core::fmt::Debug for LineDisplayScrollDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayScrollDirection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LineDisplayScrollDirection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.LineDisplayScrollDirection;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct LineDisplayStatisticsCategorySelector(::windows::core::IUnknown);
impl LineDisplayStatisticsCategorySelector {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn AllStatistics(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AllStatistics)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn UnifiedPosStatistics(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UnifiedPosStatistics)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn ManufacturerStatistics(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ManufacturerStatistics)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for LineDisplayStatisticsCategorySelector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LineDisplayStatisticsCategorySelector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LineDisplayStatisticsCategorySelector {}
impl ::core::fmt::Debug for LineDisplayStatisticsCategorySelector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayStatisticsCategorySelector").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LineDisplayStatisticsCategorySelector {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplayStatisticsCategorySelector;{b521c46b-9274-4d24-94f3-b6017b832444})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LineDisplayStatisticsCategorySelector {
    type Vtable = ILineDisplayStatisticsCategorySelector_Vtbl;
    const IID: ::windows::core::GUID = <ILineDisplayStatisticsCategorySelector as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LineDisplayStatisticsCategorySelector {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayStatisticsCategorySelector";
}
impl ::core::convert::From<LineDisplayStatisticsCategorySelector> for ::windows::core::IUnknown {
    fn from(value: LineDisplayStatisticsCategorySelector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LineDisplayStatisticsCategorySelector> for ::windows::core::IUnknown {
    fn from(value: &LineDisplayStatisticsCategorySelector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LineDisplayStatisticsCategorySelector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LineDisplayStatisticsCategorySelector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LineDisplayStatisticsCategorySelector> for ::windows::core::IInspectable {
    fn from(value: LineDisplayStatisticsCategorySelector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LineDisplayStatisticsCategorySelector> for ::windows::core::IInspectable {
    fn from(value: &LineDisplayStatisticsCategorySelector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LineDisplayStatisticsCategorySelector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LineDisplayStatisticsCategorySelector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LineDisplayStatisticsCategorySelector {}
unsafe impl ::core::marker::Sync for LineDisplayStatisticsCategorySelector {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct LineDisplayStatusUpdatedEventArgs(::windows::core::IUnknown);
impl LineDisplayStatusUpdatedEventArgs {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<LineDisplayPowerStatus> {
        let this = self;
        unsafe {
            let mut result__: LineDisplayPowerStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LineDisplayPowerStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for LineDisplayStatusUpdatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LineDisplayStatusUpdatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LineDisplayStatusUpdatedEventArgs {}
impl ::core::fmt::Debug for LineDisplayStatusUpdatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayStatusUpdatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LineDisplayStatusUpdatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplayStatusUpdatedEventArgs;{ddd57c1a-86fb-4eba-93d1-6f5eda52b752})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LineDisplayStatusUpdatedEventArgs {
    type Vtable = ILineDisplayStatusUpdatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ILineDisplayStatusUpdatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LineDisplayStatusUpdatedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayStatusUpdatedEventArgs";
}
impl ::core::convert::From<LineDisplayStatusUpdatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: LineDisplayStatusUpdatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LineDisplayStatusUpdatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &LineDisplayStatusUpdatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LineDisplayStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LineDisplayStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LineDisplayStatusUpdatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: LineDisplayStatusUpdatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LineDisplayStatusUpdatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &LineDisplayStatusUpdatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LineDisplayStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LineDisplayStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LineDisplayStatusUpdatedEventArgs {}
unsafe impl ::core::marker::Sync for LineDisplayStatusUpdatedEventArgs {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct LineDisplayStoredBitmap(::windows::core::IUnknown);
impl LineDisplayStoredBitmap {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn EscapeSequence(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EscapeSequence)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryDeleteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryDeleteAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
impl ::core::clone::Clone for LineDisplayStoredBitmap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LineDisplayStoredBitmap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LineDisplayStoredBitmap {}
impl ::core::fmt::Debug for LineDisplayStoredBitmap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayStoredBitmap").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LineDisplayStoredBitmap {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplayStoredBitmap;{f621515b-d81e-43ba-bf1b-bcfa3c785ba0})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LineDisplayStoredBitmap {
    type Vtable = ILineDisplayStoredBitmap_Vtbl;
    const IID: ::windows::core::GUID = <ILineDisplayStoredBitmap as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LineDisplayStoredBitmap {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayStoredBitmap";
}
impl ::core::convert::From<LineDisplayStoredBitmap> for ::windows::core::IUnknown {
    fn from(value: LineDisplayStoredBitmap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LineDisplayStoredBitmap> for ::windows::core::IUnknown {
    fn from(value: &LineDisplayStoredBitmap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LineDisplayStoredBitmap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LineDisplayStoredBitmap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LineDisplayStoredBitmap> for ::windows::core::IInspectable {
    fn from(value: LineDisplayStoredBitmap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LineDisplayStoredBitmap> for ::windows::core::IInspectable {
    fn from(value: &LineDisplayStoredBitmap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LineDisplayStoredBitmap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LineDisplayStoredBitmap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for LineDisplayStoredBitmap {}
unsafe impl ::core::marker::Sync for LineDisplayStoredBitmap {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LineDisplayTextAttribute(pub i32);
impl LineDisplayTextAttribute {
    pub const Normal: Self = Self(0i32);
    pub const Blink: Self = Self(1i32);
    pub const Reverse: Self = Self(2i32);
    pub const ReverseBlink: Self = Self(3i32);
}
impl ::core::marker::Copy for LineDisplayTextAttribute {}
impl ::core::clone::Clone for LineDisplayTextAttribute {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LineDisplayTextAttribute {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for LineDisplayTextAttribute {
    type Abi = Self;
}
impl ::core::fmt::Debug for LineDisplayTextAttribute {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayTextAttribute").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LineDisplayTextAttribute {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.LineDisplayTextAttribute;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LineDisplayTextAttributeGranularity(pub i32);
impl LineDisplayTextAttributeGranularity {
    pub const NotSupported: Self = Self(0i32);
    pub const EntireDisplay: Self = Self(1i32);
    pub const PerCharacter: Self = Self(2i32);
}
impl ::core::marker::Copy for LineDisplayTextAttributeGranularity {}
impl ::core::clone::Clone for LineDisplayTextAttributeGranularity {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LineDisplayTextAttributeGranularity {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for LineDisplayTextAttributeGranularity {
    type Abi = Self;
}
impl ::core::fmt::Debug for LineDisplayTextAttributeGranularity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayTextAttributeGranularity").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LineDisplayTextAttributeGranularity {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.LineDisplayTextAttributeGranularity;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LineDisplayVerticalAlignment(pub i32);
impl LineDisplayVerticalAlignment {
    pub const Top: Self = Self(0i32);
    pub const Center: Self = Self(1i32);
    pub const Bottom: Self = Self(2i32);
}
impl ::core::marker::Copy for LineDisplayVerticalAlignment {}
impl ::core::clone::Clone for LineDisplayVerticalAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LineDisplayVerticalAlignment {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for LineDisplayVerticalAlignment {
    type Abi = Self;
}
impl ::core::fmt::Debug for LineDisplayVerticalAlignment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayVerticalAlignment").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LineDisplayVerticalAlignment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.LineDisplayVerticalAlignment;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct LineDisplayWindow(::windows::core::IUnknown);
impl LineDisplayWindow {
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SizeInCharacters(&self) -> ::windows::core::Result<super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::Size = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SizeInCharacters)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Size>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn InterCharacterWaitInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InterCharacterWaitInterval)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetInterCharacterWaitInterval<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetInterCharacterWaitInterval)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryRefreshAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryRefreshAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryDisplayTextAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, text: Param0, displayattribute: LineDisplayTextAttribute) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryDisplayTextAsync)(::core::mem::transmute_copy(this), text.into_param().abi(), displayattribute, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryDisplayTextAtPositionAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::Point>>(&self, text: Param0, displayattribute: LineDisplayTextAttribute, startposition: Param2) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryDisplayTextAtPositionAsync)(::core::mem::transmute_copy(this), text.into_param().abi(), displayattribute, startposition.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryDisplayTextNormalAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, text: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryDisplayTextNormalAsync)(::core::mem::transmute_copy(this), text.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryScrollTextAsync(&self, direction: LineDisplayScrollDirection, numberofcolumnsorrows: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryScrollTextAsync)(::core::mem::transmute_copy(this), direction, numberofcolumnsorrows, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryClearTextAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryClearTextAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Cursor(&self) -> ::windows::core::Result<LineDisplayCursor> {
        let this = &::windows::core::Interface::cast::<ILineDisplayWindow2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Cursor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LineDisplayCursor>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Marquee(&self) -> ::windows::core::Result<LineDisplayMarquee> {
        let this = &::windows::core::Interface::cast::<ILineDisplayWindow2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Marquee)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LineDisplayMarquee>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReadCharacterAtCursorAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>> {
        let this = &::windows::core::Interface::cast::<ILineDisplayWindow2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReadCharacterAtCursorAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryDisplayStoredBitmapAtCursorAsync<'a, Param0: ::windows::core::IntoParam<'a, LineDisplayStoredBitmap>>(&self, bitmap: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<ILineDisplayWindow2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryDisplayStoredBitmapAtCursorAsync)(::core::mem::transmute_copy(this), bitmap.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn TryDisplayStorageFileBitmapAtCursorAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::StorageFile>>(&self, bitmap: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<ILineDisplayWindow2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryDisplayStorageFileBitmapAtCursorAsync)(::core::mem::transmute_copy(this), bitmap.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn TryDisplayStorageFileBitmapAtCursorWithAlignmentAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::StorageFile>>(&self, bitmap: Param0, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<ILineDisplayWindow2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryDisplayStorageFileBitmapAtCursorWithAlignmentAsync)(::core::mem::transmute_copy(this), bitmap.into_param().abi(), horizontalalignment, verticalalignment, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn TryDisplayStorageFileBitmapAtCursorWithAlignmentAndWidthAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::StorageFile>>(&self, bitmap: Param0, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment, widthinpixels: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<ILineDisplayWindow2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryDisplayStorageFileBitmapAtCursorWithAlignmentAndWidthAsync)(::core::mem::transmute_copy(this), bitmap.into_param().abi(), horizontalalignment, verticalalignment, widthinpixels, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn TryDisplayStorageFileBitmapAtPointAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::StorageFile>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Point>>(&self, bitmap: Param0, offsetinpixels: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<ILineDisplayWindow2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryDisplayStorageFileBitmapAtPointAsync)(::core::mem::transmute_copy(this), bitmap.into_param().abi(), offsetinpixels.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn TryDisplayStorageFileBitmapAtPointWithWidthAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::StorageFile>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Point>>(&self, bitmap: Param0, offsetinpixels: Param1, widthinpixels: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<ILineDisplayWindow2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TryDisplayStorageFileBitmapAtPointWithWidthAsync)(::core::mem::transmute_copy(this), bitmap.into_param().abi(), offsetinpixels.into_param().abi(), widthinpixels, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
impl ::core::clone::Clone for LineDisplayWindow {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for LineDisplayWindow {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LineDisplayWindow {}
impl ::core::fmt::Debug for LineDisplayWindow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LineDisplayWindow").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for LineDisplayWindow {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.LineDisplayWindow;{d21feef4-2364-4be5-bee1-851680af4964})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for LineDisplayWindow {
    type Vtable = ILineDisplayWindow_Vtbl;
    const IID: ::windows::core::GUID = <ILineDisplayWindow as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for LineDisplayWindow {
    const NAME: &'static str = "Windows.Devices.PointOfService.LineDisplayWindow";
}
impl ::core::convert::From<LineDisplayWindow> for ::windows::core::IUnknown {
    fn from(value: LineDisplayWindow) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LineDisplayWindow> for ::windows::core::IUnknown {
    fn from(value: &LineDisplayWindow) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for LineDisplayWindow {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a LineDisplayWindow {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<LineDisplayWindow> for ::windows::core::IInspectable {
    fn from(value: LineDisplayWindow) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&LineDisplayWindow> for ::windows::core::IInspectable {
    fn from(value: &LineDisplayWindow) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for LineDisplayWindow {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a LineDisplayWindow {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct MagneticStripeReader(::windows::core::IUnknown);
impl MagneticStripeReader {
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Capabilities(&self) -> ::windows::core::Result<MagneticStripeReaderCapabilities> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Capabilities)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MagneticStripeReaderCapabilities>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SupportedCardTypes(&self) -> ::windows::core::Result<::windows::core::Array<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::Array<u32> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SupportedCardTypes)(::core::mem::transmute_copy(this), ::windows::core::Array::<u32>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn DeviceAuthenticationProtocol(&self) -> ::windows::core::Result<MagneticStripeReaderAuthenticationProtocol> {
        let this = self;
        unsafe {
            let mut result__: MagneticStripeReaderAuthenticationProtocol = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceAuthenticationProtocol)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MagneticStripeReaderAuthenticationProtocol>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CheckHealthAsync(&self, level: UnifiedPosHealthCheckLevel) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CheckHealthAsync)(::core::mem::transmute_copy(this), level, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ClaimReaderAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ClaimedMagneticStripeReader>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ClaimReaderAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ClaimedMagneticStripeReader>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation_Collections\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn RetrieveStatisticsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, statisticscategories: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RetrieveStatisticsAsync)(::core::mem::transmute_copy(this), statisticscategories.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn GetErrorReportingType(&self) -> ::windows::core::Result<MagneticStripeReaderErrorReportingType> {
        let this = self;
        unsafe {
            let mut result__: MagneticStripeReaderErrorReportingType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetErrorReportingType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MagneticStripeReaderErrorReportingType>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StatusUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<MagneticStripeReader, MagneticStripeReaderStatusUpdatedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StatusUpdated)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStatusUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveStatusUpdated)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDefaultAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MagneticStripeReader>> {
        Self::IMagneticStripeReaderStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDefaultAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MagneticStripeReader>>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MagneticStripeReader>> {
        Self::IMagneticStripeReaderStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FromIdAsync)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<MagneticStripeReader>>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMagneticStripeReaderStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeviceSelector)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn GetDeviceSelectorWithConnectionTypes(connectiontypes: PosConnectionTypes) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IMagneticStripeReaderStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeviceSelectorWithConnectionTypes)(::core::mem::transmute_copy(this), connectiontypes, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMagneticStripeReaderStatics<R, F: FnOnce(&IMagneticStripeReaderStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MagneticStripeReader, IMagneticStripeReaderStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IMagneticStripeReaderStatics2<R, F: FnOnce(&IMagneticStripeReaderStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MagneticStripeReader, IMagneticStripeReaderStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MagneticStripeReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MagneticStripeReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MagneticStripeReader {}
impl ::core::fmt::Debug for MagneticStripeReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagneticStripeReader").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MagneticStripeReader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.MagneticStripeReader;{1a92b015-47c3-468a-9333-0c6517574883})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MagneticStripeReader {
    type Vtable = IMagneticStripeReader_Vtbl;
    const IID: ::windows::core::GUID = <IMagneticStripeReader as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MagneticStripeReader {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReader";
}
impl ::core::convert::From<MagneticStripeReader> for ::windows::core::IUnknown {
    fn from(value: MagneticStripeReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MagneticStripeReader> for ::windows::core::IUnknown {
    fn from(value: &MagneticStripeReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MagneticStripeReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MagneticStripeReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MagneticStripeReader> for ::windows::core::IInspectable {
    fn from(value: MagneticStripeReader) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MagneticStripeReader> for ::windows::core::IInspectable {
    fn from(value: &MagneticStripeReader) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MagneticStripeReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MagneticStripeReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct MagneticStripeReaderAamvaCardDataReceivedEventArgs(::windows::core::IUnknown);
impl MagneticStripeReaderAamvaCardDataReceivedEventArgs {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Report(&self) -> ::windows::core::Result<MagneticStripeReaderReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Report)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MagneticStripeReaderReport>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn LicenseNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LicenseNumber)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn ExpirationDate(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExpirationDate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Restrictions(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Restrictions)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Class(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Class)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Endorsements(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Endorsements)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn BirthDate(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BirthDate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn FirstName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FirstName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Surname(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Surname)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Suffix(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Suffix)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Gender(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Gender)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn HairColor(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HairColor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn EyeColor(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EyeColor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Height(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Height)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Weight(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Weight)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Address(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Address)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn City(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).City)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn State(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).State)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn PostalCode(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PostalCode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for MagneticStripeReaderAamvaCardDataReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MagneticStripeReaderAamvaCardDataReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MagneticStripeReaderAamvaCardDataReceivedEventArgs {}
impl ::core::fmt::Debug for MagneticStripeReaderAamvaCardDataReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagneticStripeReaderAamvaCardDataReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MagneticStripeReaderAamvaCardDataReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.MagneticStripeReaderAamvaCardDataReceivedEventArgs;{0a4bbd51-c316-4910-87f3-7a62ba862d31})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MagneticStripeReaderAamvaCardDataReceivedEventArgs {
    type Vtable = IMagneticStripeReaderAamvaCardDataReceivedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IMagneticStripeReaderAamvaCardDataReceivedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MagneticStripeReaderAamvaCardDataReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderAamvaCardDataReceivedEventArgs";
}
impl ::core::convert::From<MagneticStripeReaderAamvaCardDataReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MagneticStripeReaderAamvaCardDataReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MagneticStripeReaderAamvaCardDataReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MagneticStripeReaderAamvaCardDataReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MagneticStripeReaderAamvaCardDataReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MagneticStripeReaderAamvaCardDataReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MagneticStripeReaderAamvaCardDataReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MagneticStripeReaderAamvaCardDataReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MagneticStripeReaderAamvaCardDataReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MagneticStripeReaderAamvaCardDataReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MagneticStripeReaderAamvaCardDataReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MagneticStripeReaderAamvaCardDataReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MagneticStripeReaderAamvaCardDataReceivedEventArgs {}
unsafe impl ::core::marker::Sync for MagneticStripeReaderAamvaCardDataReceivedEventArgs {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MagneticStripeReaderAuthenticationLevel(pub i32);
impl MagneticStripeReaderAuthenticationLevel {
    pub const NotSupported: Self = Self(0i32);
    pub const Optional: Self = Self(1i32);
    pub const Required: Self = Self(2i32);
}
impl ::core::marker::Copy for MagneticStripeReaderAuthenticationLevel {}
impl ::core::clone::Clone for MagneticStripeReaderAuthenticationLevel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MagneticStripeReaderAuthenticationLevel {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MagneticStripeReaderAuthenticationLevel {
    type Abi = Self;
}
impl ::core::fmt::Debug for MagneticStripeReaderAuthenticationLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagneticStripeReaderAuthenticationLevel").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MagneticStripeReaderAuthenticationLevel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.MagneticStripeReaderAuthenticationLevel;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MagneticStripeReaderAuthenticationProtocol(pub i32);
impl MagneticStripeReaderAuthenticationProtocol {
    pub const None: Self = Self(0i32);
    pub const ChallengeResponse: Self = Self(1i32);
}
impl ::core::marker::Copy for MagneticStripeReaderAuthenticationProtocol {}
impl ::core::clone::Clone for MagneticStripeReaderAuthenticationProtocol {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MagneticStripeReaderAuthenticationProtocol {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MagneticStripeReaderAuthenticationProtocol {
    type Abi = Self;
}
impl ::core::fmt::Debug for MagneticStripeReaderAuthenticationProtocol {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagneticStripeReaderAuthenticationProtocol").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MagneticStripeReaderAuthenticationProtocol {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.MagneticStripeReaderAuthenticationProtocol;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct MagneticStripeReaderBankCardDataReceivedEventArgs(::windows::core::IUnknown);
impl MagneticStripeReaderBankCardDataReceivedEventArgs {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Report(&self) -> ::windows::core::Result<MagneticStripeReaderReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Report)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MagneticStripeReaderReport>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn AccountNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AccountNumber)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn ExpirationDate(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExpirationDate)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn ServiceCode(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ServiceCode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Title)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn FirstName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FirstName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn MiddleInitial(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MiddleInitial)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Surname(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Surname)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Suffix(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Suffix)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for MagneticStripeReaderBankCardDataReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MagneticStripeReaderBankCardDataReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MagneticStripeReaderBankCardDataReceivedEventArgs {}
impl ::core::fmt::Debug for MagneticStripeReaderBankCardDataReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagneticStripeReaderBankCardDataReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MagneticStripeReaderBankCardDataReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.MagneticStripeReaderBankCardDataReceivedEventArgs;{2e958823-a31a-4763-882c-23725e39b08e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MagneticStripeReaderBankCardDataReceivedEventArgs {
    type Vtable = IMagneticStripeReaderBankCardDataReceivedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IMagneticStripeReaderBankCardDataReceivedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MagneticStripeReaderBankCardDataReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderBankCardDataReceivedEventArgs";
}
impl ::core::convert::From<MagneticStripeReaderBankCardDataReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MagneticStripeReaderBankCardDataReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MagneticStripeReaderBankCardDataReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MagneticStripeReaderBankCardDataReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MagneticStripeReaderBankCardDataReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MagneticStripeReaderBankCardDataReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MagneticStripeReaderBankCardDataReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MagneticStripeReaderBankCardDataReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MagneticStripeReaderBankCardDataReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MagneticStripeReaderBankCardDataReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MagneticStripeReaderBankCardDataReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MagneticStripeReaderBankCardDataReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MagneticStripeReaderBankCardDataReceivedEventArgs {}
unsafe impl ::core::marker::Sync for MagneticStripeReaderBankCardDataReceivedEventArgs {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct MagneticStripeReaderCapabilities(::windows::core::IUnknown);
impl MagneticStripeReaderCapabilities {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn CardAuthentication(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CardAuthentication)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SupportedEncryptionAlgorithms(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SupportedEncryptionAlgorithms)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn AuthenticationLevel(&self) -> ::windows::core::Result<MagneticStripeReaderAuthenticationLevel> {
        let this = self;
        unsafe {
            let mut result__: MagneticStripeReaderAuthenticationLevel = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AuthenticationLevel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MagneticStripeReaderAuthenticationLevel>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsIsoSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsIsoSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsJisOneSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsJisOneSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsJisTwoSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsJisTwoSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn PowerReportingType(&self) -> ::windows::core::Result<UnifiedPosPowerReportingType> {
        let this = self;
        unsafe {
            let mut result__: UnifiedPosPowerReportingType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PowerReportingType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UnifiedPosPowerReportingType>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsStatisticsReportingSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsStatisticsReportingSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsStatisticsUpdatingSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsStatisticsUpdatingSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsTrackDataMaskingSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsTrackDataMaskingSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsTransmitSentinelsSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsTransmitSentinelsSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for MagneticStripeReaderCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MagneticStripeReaderCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MagneticStripeReaderCapabilities {}
impl ::core::fmt::Debug for MagneticStripeReaderCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagneticStripeReaderCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MagneticStripeReaderCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.MagneticStripeReaderCapabilities;{7128809c-c440-44a2-a467-469175d02896})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MagneticStripeReaderCapabilities {
    type Vtable = IMagneticStripeReaderCapabilities_Vtbl;
    const IID: ::windows::core::GUID = <IMagneticStripeReaderCapabilities as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MagneticStripeReaderCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderCapabilities";
}
impl ::core::convert::From<MagneticStripeReaderCapabilities> for ::windows::core::IUnknown {
    fn from(value: MagneticStripeReaderCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MagneticStripeReaderCapabilities> for ::windows::core::IUnknown {
    fn from(value: &MagneticStripeReaderCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MagneticStripeReaderCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MagneticStripeReaderCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MagneticStripeReaderCapabilities> for ::windows::core::IInspectable {
    fn from(value: MagneticStripeReaderCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MagneticStripeReaderCapabilities> for ::windows::core::IInspectable {
    fn from(value: &MagneticStripeReaderCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MagneticStripeReaderCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MagneticStripeReaderCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MagneticStripeReaderCapabilities {}
unsafe impl ::core::marker::Sync for MagneticStripeReaderCapabilities {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
pub struct MagneticStripeReaderCardTypes {}
impl MagneticStripeReaderCardTypes {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Unknown() -> ::windows::core::Result<u32> {
        Self::IMagneticStripeReaderCardTypesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Unknown)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Bank() -> ::windows::core::Result<u32> {
        Self::IMagneticStripeReaderCardTypesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Bank)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Aamva() -> ::windows::core::Result<u32> {
        Self::IMagneticStripeReaderCardTypesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Aamva)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn ExtendedBase() -> ::windows::core::Result<u32> {
        Self::IMagneticStripeReaderCardTypesStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedBase)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMagneticStripeReaderCardTypesStatics<R, F: FnOnce(&IMagneticStripeReaderCardTypesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MagneticStripeReaderCardTypes, IMagneticStripeReaderCardTypesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for MagneticStripeReaderCardTypes {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderCardTypes";
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
pub struct MagneticStripeReaderEncryptionAlgorithms {}
impl MagneticStripeReaderEncryptionAlgorithms {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn None() -> ::windows::core::Result<u32> {
        Self::IMagneticStripeReaderEncryptionAlgorithmsStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).None)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn TripleDesDukpt() -> ::windows::core::Result<u32> {
        Self::IMagneticStripeReaderEncryptionAlgorithmsStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TripleDesDukpt)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn ExtendedBase() -> ::windows::core::Result<u32> {
        Self::IMagneticStripeReaderEncryptionAlgorithmsStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedBase)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMagneticStripeReaderEncryptionAlgorithmsStatics<R, F: FnOnce(&IMagneticStripeReaderEncryptionAlgorithmsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MagneticStripeReaderEncryptionAlgorithms, IMagneticStripeReaderEncryptionAlgorithmsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for MagneticStripeReaderEncryptionAlgorithms {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderEncryptionAlgorithms";
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct MagneticStripeReaderErrorOccurredEventArgs(::windows::core::IUnknown);
impl MagneticStripeReaderErrorOccurredEventArgs {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Track1Status(&self) -> ::windows::core::Result<MagneticStripeReaderTrackErrorType> {
        let this = self;
        unsafe {
            let mut result__: MagneticStripeReaderTrackErrorType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Track1Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MagneticStripeReaderTrackErrorType>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Track2Status(&self) -> ::windows::core::Result<MagneticStripeReaderTrackErrorType> {
        let this = self;
        unsafe {
            let mut result__: MagneticStripeReaderTrackErrorType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Track2Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MagneticStripeReaderTrackErrorType>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Track3Status(&self) -> ::windows::core::Result<MagneticStripeReaderTrackErrorType> {
        let this = self;
        unsafe {
            let mut result__: MagneticStripeReaderTrackErrorType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Track3Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MagneticStripeReaderTrackErrorType>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Track4Status(&self) -> ::windows::core::Result<MagneticStripeReaderTrackErrorType> {
        let this = self;
        unsafe {
            let mut result__: MagneticStripeReaderTrackErrorType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Track4Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MagneticStripeReaderTrackErrorType>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn ErrorData(&self) -> ::windows::core::Result<UnifiedPosErrorData> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ErrorData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UnifiedPosErrorData>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn PartialInputData(&self) -> ::windows::core::Result<MagneticStripeReaderReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PartialInputData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MagneticStripeReaderReport>(result__)
        }
    }
}
impl ::core::clone::Clone for MagneticStripeReaderErrorOccurredEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MagneticStripeReaderErrorOccurredEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MagneticStripeReaderErrorOccurredEventArgs {}
impl ::core::fmt::Debug for MagneticStripeReaderErrorOccurredEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagneticStripeReaderErrorOccurredEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MagneticStripeReaderErrorOccurredEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.MagneticStripeReaderErrorOccurredEventArgs;{1fedf95d-2c84-41ad-b778-f2356a789ab1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MagneticStripeReaderErrorOccurredEventArgs {
    type Vtable = IMagneticStripeReaderErrorOccurredEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IMagneticStripeReaderErrorOccurredEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MagneticStripeReaderErrorOccurredEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderErrorOccurredEventArgs";
}
impl ::core::convert::From<MagneticStripeReaderErrorOccurredEventArgs> for ::windows::core::IUnknown {
    fn from(value: MagneticStripeReaderErrorOccurredEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MagneticStripeReaderErrorOccurredEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MagneticStripeReaderErrorOccurredEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MagneticStripeReaderErrorOccurredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MagneticStripeReaderErrorOccurredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MagneticStripeReaderErrorOccurredEventArgs> for ::windows::core::IInspectable {
    fn from(value: MagneticStripeReaderErrorOccurredEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MagneticStripeReaderErrorOccurredEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MagneticStripeReaderErrorOccurredEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MagneticStripeReaderErrorOccurredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MagneticStripeReaderErrorOccurredEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MagneticStripeReaderErrorOccurredEventArgs {}
unsafe impl ::core::marker::Sync for MagneticStripeReaderErrorOccurredEventArgs {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MagneticStripeReaderErrorReportingType(pub i32);
impl MagneticStripeReaderErrorReportingType {
    pub const CardLevel: Self = Self(0i32);
    pub const TrackLevel: Self = Self(1i32);
}
impl ::core::marker::Copy for MagneticStripeReaderErrorReportingType {}
impl ::core::clone::Clone for MagneticStripeReaderErrorReportingType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MagneticStripeReaderErrorReportingType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MagneticStripeReaderErrorReportingType {
    type Abi = Self;
}
impl ::core::fmt::Debug for MagneticStripeReaderErrorReportingType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagneticStripeReaderErrorReportingType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MagneticStripeReaderErrorReportingType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.MagneticStripeReaderErrorReportingType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct MagneticStripeReaderReport(::windows::core::IUnknown);
impl MagneticStripeReaderReport {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn CardType(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CardType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Track1(&self) -> ::windows::core::Result<MagneticStripeReaderTrackData> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Track1)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MagneticStripeReaderTrackData>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Track2(&self) -> ::windows::core::Result<MagneticStripeReaderTrackData> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Track2)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MagneticStripeReaderTrackData>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Track3(&self) -> ::windows::core::Result<MagneticStripeReaderTrackData> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Track3)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MagneticStripeReaderTrackData>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Track4(&self) -> ::windows::core::Result<MagneticStripeReaderTrackData> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Track4)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MagneticStripeReaderTrackData>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CardAuthenticationData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CardAuthenticationData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn CardAuthenticationDataLength(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CardAuthenticationDataLength)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn AdditionalSecurityInformation(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AdditionalSecurityInformation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
}
impl ::core::clone::Clone for MagneticStripeReaderReport {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MagneticStripeReaderReport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MagneticStripeReaderReport {}
impl ::core::fmt::Debug for MagneticStripeReaderReport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagneticStripeReaderReport").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MagneticStripeReaderReport {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.MagneticStripeReaderReport;{6a5b6047-99b0-4188-bef1-eddf79f78fe6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MagneticStripeReaderReport {
    type Vtable = IMagneticStripeReaderReport_Vtbl;
    const IID: ::windows::core::GUID = <IMagneticStripeReaderReport as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MagneticStripeReaderReport {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderReport";
}
impl ::core::convert::From<MagneticStripeReaderReport> for ::windows::core::IUnknown {
    fn from(value: MagneticStripeReaderReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MagneticStripeReaderReport> for ::windows::core::IUnknown {
    fn from(value: &MagneticStripeReaderReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MagneticStripeReaderReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MagneticStripeReaderReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MagneticStripeReaderReport> for ::windows::core::IInspectable {
    fn from(value: MagneticStripeReaderReport) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MagneticStripeReaderReport> for ::windows::core::IInspectable {
    fn from(value: &MagneticStripeReaderReport) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MagneticStripeReaderReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MagneticStripeReaderReport {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MagneticStripeReaderReport {}
unsafe impl ::core::marker::Sync for MagneticStripeReaderReport {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MagneticStripeReaderStatus(pub i32);
impl MagneticStripeReaderStatus {
    pub const Unauthenticated: Self = Self(0i32);
    pub const Authenticated: Self = Self(1i32);
    pub const Extended: Self = Self(2i32);
}
impl ::core::marker::Copy for MagneticStripeReaderStatus {}
impl ::core::clone::Clone for MagneticStripeReaderStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MagneticStripeReaderStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MagneticStripeReaderStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for MagneticStripeReaderStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagneticStripeReaderStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MagneticStripeReaderStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.MagneticStripeReaderStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct MagneticStripeReaderStatusUpdatedEventArgs(::windows::core::IUnknown);
impl MagneticStripeReaderStatusUpdatedEventArgs {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<MagneticStripeReaderStatus> {
        let this = self;
        unsafe {
            let mut result__: MagneticStripeReaderStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MagneticStripeReaderStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn ExtendedStatus(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedStatus)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for MagneticStripeReaderStatusUpdatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MagneticStripeReaderStatusUpdatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MagneticStripeReaderStatusUpdatedEventArgs {}
impl ::core::fmt::Debug for MagneticStripeReaderStatusUpdatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagneticStripeReaderStatusUpdatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MagneticStripeReaderStatusUpdatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.MagneticStripeReaderStatusUpdatedEventArgs;{09cc6bb0-3262-401d-9e8a-e80d6358906b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MagneticStripeReaderStatusUpdatedEventArgs {
    type Vtable = IMagneticStripeReaderStatusUpdatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IMagneticStripeReaderStatusUpdatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MagneticStripeReaderStatusUpdatedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderStatusUpdatedEventArgs";
}
impl ::core::convert::From<MagneticStripeReaderStatusUpdatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MagneticStripeReaderStatusUpdatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MagneticStripeReaderStatusUpdatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MagneticStripeReaderStatusUpdatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MagneticStripeReaderStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MagneticStripeReaderStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MagneticStripeReaderStatusUpdatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MagneticStripeReaderStatusUpdatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MagneticStripeReaderStatusUpdatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MagneticStripeReaderStatusUpdatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MagneticStripeReaderStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MagneticStripeReaderStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MagneticStripeReaderStatusUpdatedEventArgs {}
unsafe impl ::core::marker::Sync for MagneticStripeReaderStatusUpdatedEventArgs {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct MagneticStripeReaderTrackData(::windows::core::IUnknown);
impl MagneticStripeReaderTrackData {
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Data(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Data)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn DiscretionaryData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DiscretionaryData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn EncryptedData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).EncryptedData)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IBuffer>(result__)
        }
    }
}
impl ::core::clone::Clone for MagneticStripeReaderTrackData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MagneticStripeReaderTrackData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MagneticStripeReaderTrackData {}
impl ::core::fmt::Debug for MagneticStripeReaderTrackData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagneticStripeReaderTrackData").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MagneticStripeReaderTrackData {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.MagneticStripeReaderTrackData;{104cf671-4a9d-446e-abc5-20402307ba36})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MagneticStripeReaderTrackData {
    type Vtable = IMagneticStripeReaderTrackData_Vtbl;
    const IID: ::windows::core::GUID = <IMagneticStripeReaderTrackData as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MagneticStripeReaderTrackData {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderTrackData";
}
impl ::core::convert::From<MagneticStripeReaderTrackData> for ::windows::core::IUnknown {
    fn from(value: MagneticStripeReaderTrackData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MagneticStripeReaderTrackData> for ::windows::core::IUnknown {
    fn from(value: &MagneticStripeReaderTrackData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MagneticStripeReaderTrackData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MagneticStripeReaderTrackData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MagneticStripeReaderTrackData> for ::windows::core::IInspectable {
    fn from(value: MagneticStripeReaderTrackData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MagneticStripeReaderTrackData> for ::windows::core::IInspectable {
    fn from(value: &MagneticStripeReaderTrackData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MagneticStripeReaderTrackData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MagneticStripeReaderTrackData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MagneticStripeReaderTrackData {}
unsafe impl ::core::marker::Sync for MagneticStripeReaderTrackData {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MagneticStripeReaderTrackErrorType(pub i32);
impl MagneticStripeReaderTrackErrorType {
    pub const None: Self = Self(0i32);
    pub const StartSentinelError: Self = Self(1i32);
    pub const EndSentinelError: Self = Self(2i32);
    pub const ParityError: Self = Self(3i32);
    pub const LrcError: Self = Self(4i32);
    pub const Unknown: Self = Self(-1i32);
}
impl ::core::marker::Copy for MagneticStripeReaderTrackErrorType {}
impl ::core::clone::Clone for MagneticStripeReaderTrackErrorType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MagneticStripeReaderTrackErrorType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MagneticStripeReaderTrackErrorType {
    type Abi = Self;
}
impl ::core::fmt::Debug for MagneticStripeReaderTrackErrorType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagneticStripeReaderTrackErrorType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MagneticStripeReaderTrackErrorType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.MagneticStripeReaderTrackErrorType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MagneticStripeReaderTrackIds(pub i32);
impl MagneticStripeReaderTrackIds {
    pub const None: Self = Self(0i32);
    pub const Track1: Self = Self(1i32);
    pub const Track2: Self = Self(2i32);
    pub const Track3: Self = Self(4i32);
    pub const Track4: Self = Self(8i32);
}
impl ::core::marker::Copy for MagneticStripeReaderTrackIds {}
impl ::core::clone::Clone for MagneticStripeReaderTrackIds {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MagneticStripeReaderTrackIds {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MagneticStripeReaderTrackIds {
    type Abi = Self;
}
impl ::core::fmt::Debug for MagneticStripeReaderTrackIds {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagneticStripeReaderTrackIds").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MagneticStripeReaderTrackIds {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.MagneticStripeReaderTrackIds;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs(::windows::core::IUnknown);
impl MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Report(&self) -> ::windows::core::Result<MagneticStripeReaderReport> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Report)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MagneticStripeReaderReport>(result__)
        }
    }
}
impl ::core::clone::Clone for MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {}
impl ::core::fmt::Debug for MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs;{af0a5514-59cc-4a60-99e8-99a53dace5aa})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    type Vtable = IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs";
}
impl ::core::convert::From<MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {}
unsafe impl ::core::marker::Sync for MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PosConnectionTypes(pub u32);
impl PosConnectionTypes {
    pub const Local: Self = Self(1u32);
    pub const IP: Self = Self(2u32);
    pub const Bluetooth: Self = Self(4u32);
    pub const All: Self = Self(4294967295u32);
}
impl ::core::marker::Copy for PosConnectionTypes {}
impl ::core::clone::Clone for PosConnectionTypes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PosConnectionTypes {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PosConnectionTypes {
    type Abi = Self;
}
impl ::core::fmt::Debug for PosConnectionTypes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosConnectionTypes").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PosConnectionTypes {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PosConnectionTypes {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PosConnectionTypes {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PosConnectionTypes {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PosConnectionTypes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for PosConnectionTypes {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosConnectionTypes;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct PosPrinter(::windows::core::IUnknown);
impl PosPrinter {
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeviceId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Capabilities(&self) -> ::windows::core::Result<PosPrinterCapabilities> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Capabilities)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterCapabilities>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedCharacterSets(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SupportedCharacterSets)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedTypeFaces(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SupportedTypeFaces)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<PosPrinterStatus> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ClaimPrinterAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ClaimedPosPrinter>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ClaimPrinterAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ClaimedPosPrinter>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CheckHealthAsync(&self, level: UnifiedPosHealthCheckLevel) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CheckHealthAsync)(::core::mem::transmute_copy(this), level, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStatisticsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, statisticscategories: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStatisticsAsync)(::core::mem::transmute_copy(this), statisticscategories.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StatusUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<PosPrinter, PosPrinterStatusUpdatedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StatusUpdated)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStatusUpdated<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveStatusUpdated)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedBarcodeSymbologies(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &::windows::core::Interface::cast::<IPosPrinter2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SupportedBarcodeSymbologies)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn GetFontProperty<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, typeface: Param0) -> ::windows::core::Result<PosPrinterFontProperty> {
        let this = &::windows::core::Interface::cast::<IPosPrinter2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFontProperty)(::core::mem::transmute_copy(this), typeface.into_param().abi(), &mut result__).from_abi::<PosPrinterFontProperty>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDefaultAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PosPrinter>> {
        Self::IPosPrinterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDefaultAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PosPrinter>>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(deviceid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PosPrinter>> {
        Self::IPosPrinterStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FromIdAsync)(::core::mem::transmute_copy(this), deviceid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PosPrinter>>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn GetDeviceSelector() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IPosPrinterStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeviceSelector)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn GetDeviceSelectorWithConnectionTypes(connectiontypes: PosConnectionTypes) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IPosPrinterStatics2(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeviceSelectorWithConnectionTypes)(::core::mem::transmute_copy(this), connectiontypes, &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPosPrinterStatics<R, F: FnOnce(&IPosPrinterStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PosPrinter, IPosPrinterStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IPosPrinterStatics2<R, F: FnOnce(&IPosPrinterStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PosPrinter, IPosPrinterStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PosPrinter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PosPrinter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PosPrinter {}
impl ::core::fmt::Debug for PosPrinter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PosPrinter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.PosPrinter;{2a03c10e-9a19-4a01-994f-12dfad6adcbf})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PosPrinter {
    type Vtable = IPosPrinter_Vtbl;
    const IID: ::windows::core::GUID = <IPosPrinter as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PosPrinter {
    const NAME: &'static str = "Windows.Devices.PointOfService.PosPrinter";
}
impl ::core::convert::From<PosPrinter> for ::windows::core::IUnknown {
    fn from(value: PosPrinter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PosPrinter> for ::windows::core::IUnknown {
    fn from(value: &PosPrinter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PosPrinter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PosPrinter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PosPrinter> for ::windows::core::IInspectable {
    fn from(value: PosPrinter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PosPrinter> for ::windows::core::IInspectable {
    fn from(value: &PosPrinter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PosPrinter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PosPrinter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PosPrinterAlignment(pub i32);
impl PosPrinterAlignment {
    pub const Left: Self = Self(0i32);
    pub const Center: Self = Self(1i32);
    pub const Right: Self = Self(2i32);
}
impl ::core::marker::Copy for PosPrinterAlignment {}
impl ::core::clone::Clone for PosPrinterAlignment {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PosPrinterAlignment {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PosPrinterAlignment {
    type Abi = Self;
}
impl ::core::fmt::Debug for PosPrinterAlignment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterAlignment").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PosPrinterAlignment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterAlignment;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PosPrinterBarcodeTextPosition(pub i32);
impl PosPrinterBarcodeTextPosition {
    pub const None: Self = Self(0i32);
    pub const Above: Self = Self(1i32);
    pub const Below: Self = Self(2i32);
}
impl ::core::marker::Copy for PosPrinterBarcodeTextPosition {}
impl ::core::clone::Clone for PosPrinterBarcodeTextPosition {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PosPrinterBarcodeTextPosition {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PosPrinterBarcodeTextPosition {
    type Abi = Self;
}
impl ::core::fmt::Debug for PosPrinterBarcodeTextPosition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterBarcodeTextPosition").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PosPrinterBarcodeTextPosition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterBarcodeTextPosition;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct PosPrinterCapabilities(::windows::core::IUnknown);
impl PosPrinterCapabilities {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn PowerReportingType(&self) -> ::windows::core::Result<UnifiedPosPowerReportingType> {
        let this = self;
        unsafe {
            let mut result__: UnifiedPosPowerReportingType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PowerReportingType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UnifiedPosPowerReportingType>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsStatisticsReportingSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsStatisticsReportingSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsStatisticsUpdatingSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsStatisticsUpdatingSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn DefaultCharacterSet(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DefaultCharacterSet)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn HasCoverSensor(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HasCoverSensor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn CanMapCharacterSet(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanMapCharacterSet)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsTransactionSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsTransactionSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Receipt(&self) -> ::windows::core::Result<ReceiptPrinterCapabilities> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Receipt)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ReceiptPrinterCapabilities>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Slip(&self) -> ::windows::core::Result<SlipPrinterCapabilities> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Slip)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SlipPrinterCapabilities>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Journal(&self) -> ::windows::core::Result<JournalPrinterCapabilities> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Journal)(::core::mem::transmute_copy(this), &mut result__).from_abi::<JournalPrinterCapabilities>(result__)
        }
    }
}
impl ::core::clone::Clone for PosPrinterCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PosPrinterCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PosPrinterCapabilities {}
impl ::core::fmt::Debug for PosPrinterCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PosPrinterCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.PosPrinterCapabilities;{cde95721-4380-4985-adc5-39db30cd93bc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PosPrinterCapabilities {
    type Vtable = IPosPrinterCapabilities_Vtbl;
    const IID: ::windows::core::GUID = <IPosPrinterCapabilities as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PosPrinterCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.PosPrinterCapabilities";
}
impl ::core::convert::From<PosPrinterCapabilities> for ::windows::core::IUnknown {
    fn from(value: PosPrinterCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PosPrinterCapabilities> for ::windows::core::IUnknown {
    fn from(value: &PosPrinterCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PosPrinterCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PosPrinterCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PosPrinterCapabilities> for ::windows::core::IInspectable {
    fn from(value: PosPrinterCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PosPrinterCapabilities> for ::windows::core::IInspectable {
    fn from(value: &PosPrinterCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PosPrinterCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PosPrinterCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PosPrinterCapabilities {}
unsafe impl ::core::marker::Sync for PosPrinterCapabilities {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PosPrinterCartridgeSensors(pub u32);
impl PosPrinterCartridgeSensors {
    pub const None: Self = Self(0u32);
    pub const Removed: Self = Self(1u32);
    pub const Empty: Self = Self(2u32);
    pub const HeadCleaning: Self = Self(4u32);
    pub const NearEnd: Self = Self(8u32);
}
impl ::core::marker::Copy for PosPrinterCartridgeSensors {}
impl ::core::clone::Clone for PosPrinterCartridgeSensors {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PosPrinterCartridgeSensors {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PosPrinterCartridgeSensors {
    type Abi = Self;
}
impl ::core::fmt::Debug for PosPrinterCartridgeSensors {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterCartridgeSensors").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PosPrinterCartridgeSensors {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PosPrinterCartridgeSensors {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PosPrinterCartridgeSensors {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PosPrinterCartridgeSensors {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PosPrinterCartridgeSensors {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for PosPrinterCartridgeSensors {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterCartridgeSensors;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
pub struct PosPrinterCharacterSetIds {}
impl PosPrinterCharacterSetIds {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Utf16LE() -> ::windows::core::Result<u32> {
        Self::IPosPrinterCharacterSetIdsStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Utf16LE)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Ascii() -> ::windows::core::Result<u32> {
        Self::IPosPrinterCharacterSetIdsStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Ascii)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Ansi() -> ::windows::core::Result<u32> {
        Self::IPosPrinterCharacterSetIdsStatics(|this| unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Ansi)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPosPrinterCharacterSetIdsStatics<R, F: FnOnce(&IPosPrinterCharacterSetIdsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PosPrinterCharacterSetIds, IPosPrinterCharacterSetIdsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for PosPrinterCharacterSetIds {
    const NAME: &'static str = "Windows.Devices.PointOfService.PosPrinterCharacterSetIds";
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PosPrinterColorCapabilities(pub u32);
impl PosPrinterColorCapabilities {
    pub const None: Self = Self(0u32);
    pub const Primary: Self = Self(1u32);
    pub const Custom1: Self = Self(2u32);
    pub const Custom2: Self = Self(4u32);
    pub const Custom3: Self = Self(8u32);
    pub const Custom4: Self = Self(16u32);
    pub const Custom5: Self = Self(32u32);
    pub const Custom6: Self = Self(64u32);
    pub const Cyan: Self = Self(128u32);
    pub const Magenta: Self = Self(256u32);
    pub const Yellow: Self = Self(512u32);
    pub const Full: Self = Self(1024u32);
}
impl ::core::marker::Copy for PosPrinterColorCapabilities {}
impl ::core::clone::Clone for PosPrinterColorCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PosPrinterColorCapabilities {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PosPrinterColorCapabilities {
    type Abi = Self;
}
impl ::core::fmt::Debug for PosPrinterColorCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterColorCapabilities").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PosPrinterColorCapabilities {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PosPrinterColorCapabilities {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PosPrinterColorCapabilities {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PosPrinterColorCapabilities {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PosPrinterColorCapabilities {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for PosPrinterColorCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterColorCapabilities;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PosPrinterColorCartridge(pub i32);
impl PosPrinterColorCartridge {
    pub const Unknown: Self = Self(0i32);
    pub const Primary: Self = Self(1i32);
    pub const Custom1: Self = Self(2i32);
    pub const Custom2: Self = Self(3i32);
    pub const Custom3: Self = Self(4i32);
    pub const Custom4: Self = Self(5i32);
    pub const Custom5: Self = Self(6i32);
    pub const Custom6: Self = Self(7i32);
    pub const Cyan: Self = Self(8i32);
    pub const Magenta: Self = Self(9i32);
    pub const Yellow: Self = Self(10i32);
}
impl ::core::marker::Copy for PosPrinterColorCartridge {}
impl ::core::clone::Clone for PosPrinterColorCartridge {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PosPrinterColorCartridge {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PosPrinterColorCartridge {
    type Abi = Self;
}
impl ::core::fmt::Debug for PosPrinterColorCartridge {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterColorCartridge").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PosPrinterColorCartridge {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterColorCartridge;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct PosPrinterFontProperty(::windows::core::IUnknown);
impl PosPrinterFontProperty {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn TypeFace(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TypeFace)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsScalableToAnySize(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsScalableToAnySize)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CharacterSizes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SizeUInt32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CharacterSizes)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<SizeUInt32>>(result__)
        }
    }
}
impl ::core::clone::Clone for PosPrinterFontProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PosPrinterFontProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PosPrinterFontProperty {}
impl ::core::fmt::Debug for PosPrinterFontProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterFontProperty").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PosPrinterFontProperty {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.PosPrinterFontProperty;{a7f4e93a-f8ac-5f04-84d2-29b16d8a633c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PosPrinterFontProperty {
    type Vtable = IPosPrinterFontProperty_Vtbl;
    const IID: ::windows::core::GUID = <IPosPrinterFontProperty as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PosPrinterFontProperty {
    const NAME: &'static str = "Windows.Devices.PointOfService.PosPrinterFontProperty";
}
impl ::core::convert::From<PosPrinterFontProperty> for ::windows::core::IUnknown {
    fn from(value: PosPrinterFontProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PosPrinterFontProperty> for ::windows::core::IUnknown {
    fn from(value: &PosPrinterFontProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PosPrinterFontProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PosPrinterFontProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PosPrinterFontProperty> for ::windows::core::IInspectable {
    fn from(value: PosPrinterFontProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PosPrinterFontProperty> for ::windows::core::IInspectable {
    fn from(value: &PosPrinterFontProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PosPrinterFontProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PosPrinterFontProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PosPrinterFontProperty {}
unsafe impl ::core::marker::Sync for PosPrinterFontProperty {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PosPrinterLineDirection(pub i32);
impl PosPrinterLineDirection {
    pub const Horizontal: Self = Self(0i32);
    pub const Vertical: Self = Self(1i32);
}
impl ::core::marker::Copy for PosPrinterLineDirection {}
impl ::core::clone::Clone for PosPrinterLineDirection {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PosPrinterLineDirection {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PosPrinterLineDirection {
    type Abi = Self;
}
impl ::core::fmt::Debug for PosPrinterLineDirection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterLineDirection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PosPrinterLineDirection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterLineDirection;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PosPrinterLineStyle(pub i32);
impl PosPrinterLineStyle {
    pub const SingleSolid: Self = Self(0i32);
    pub const DoubleSolid: Self = Self(1i32);
    pub const Broken: Self = Self(2i32);
    pub const Chain: Self = Self(3i32);
}
impl ::core::marker::Copy for PosPrinterLineStyle {}
impl ::core::clone::Clone for PosPrinterLineStyle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PosPrinterLineStyle {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PosPrinterLineStyle {
    type Abi = Self;
}
impl ::core::fmt::Debug for PosPrinterLineStyle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterLineStyle").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PosPrinterLineStyle {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterLineStyle;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PosPrinterMapMode(pub i32);
impl PosPrinterMapMode {
    pub const Dots: Self = Self(0i32);
    pub const Twips: Self = Self(1i32);
    pub const English: Self = Self(2i32);
    pub const Metric: Self = Self(3i32);
}
impl ::core::marker::Copy for PosPrinterMapMode {}
impl ::core::clone::Clone for PosPrinterMapMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PosPrinterMapMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PosPrinterMapMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for PosPrinterMapMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterMapMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PosPrinterMapMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterMapMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PosPrinterMarkFeedCapabilities(pub u32);
impl PosPrinterMarkFeedCapabilities {
    pub const None: Self = Self(0u32);
    pub const ToTakeUp: Self = Self(1u32);
    pub const ToCutter: Self = Self(2u32);
    pub const ToCurrentTopOfForm: Self = Self(4u32);
    pub const ToNextTopOfForm: Self = Self(8u32);
}
impl ::core::marker::Copy for PosPrinterMarkFeedCapabilities {}
impl ::core::clone::Clone for PosPrinterMarkFeedCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PosPrinterMarkFeedCapabilities {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PosPrinterMarkFeedCapabilities {
    type Abi = Self;
}
impl ::core::fmt::Debug for PosPrinterMarkFeedCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterMarkFeedCapabilities").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PosPrinterMarkFeedCapabilities {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PosPrinterMarkFeedCapabilities {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PosPrinterMarkFeedCapabilities {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PosPrinterMarkFeedCapabilities {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PosPrinterMarkFeedCapabilities {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for PosPrinterMarkFeedCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterMarkFeedCapabilities;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PosPrinterMarkFeedKind(pub i32);
impl PosPrinterMarkFeedKind {
    pub const ToTakeUp: Self = Self(0i32);
    pub const ToCutter: Self = Self(1i32);
    pub const ToCurrentTopOfForm: Self = Self(2i32);
    pub const ToNextTopOfForm: Self = Self(3i32);
}
impl ::core::marker::Copy for PosPrinterMarkFeedKind {}
impl ::core::clone::Clone for PosPrinterMarkFeedKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PosPrinterMarkFeedKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PosPrinterMarkFeedKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for PosPrinterMarkFeedKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterMarkFeedKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PosPrinterMarkFeedKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterMarkFeedKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct PosPrinterPrintOptions(::windows::core::IUnknown);
impl PosPrinterPrintOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PosPrinterPrintOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn TypeFace(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TypeFace)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetTypeFace<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTypeFace)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn CharacterHeight(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CharacterHeight)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetCharacterHeight(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCharacterHeight)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Bold(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Bold)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetBold(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBold)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Italic(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Italic)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetItalic(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetItalic)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Underline(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Underline)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetUnderline(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetUnderline)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn ReverseVideo(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ReverseVideo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetReverseVideo(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetReverseVideo)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Strikethrough(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Strikethrough)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetStrikethrough(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStrikethrough)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Superscript(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Superscript)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetSuperscript(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSuperscript)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Subscript(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Subscript)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetSubscript(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSubscript)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn DoubleWide(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DoubleWide)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetDoubleWide(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDoubleWide)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn DoubleHigh(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DoubleHigh)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetDoubleHigh(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDoubleHigh)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Alignment(&self) -> ::windows::core::Result<PosPrinterAlignment> {
        let this = self;
        unsafe {
            let mut result__: PosPrinterAlignment = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Alignment)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterAlignment>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetAlignment(&self, value: PosPrinterAlignment) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAlignment)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn CharacterSet(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CharacterSet)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetCharacterSet(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCharacterSet)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for PosPrinterPrintOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PosPrinterPrintOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PosPrinterPrintOptions {}
impl ::core::fmt::Debug for PosPrinterPrintOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterPrintOptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PosPrinterPrintOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.PosPrinterPrintOptions;{0a2e16fd-1d02-5a58-9d59-bfcde76fde86})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PosPrinterPrintOptions {
    type Vtable = IPosPrinterPrintOptions_Vtbl;
    const IID: ::windows::core::GUID = <IPosPrinterPrintOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PosPrinterPrintOptions {
    const NAME: &'static str = "Windows.Devices.PointOfService.PosPrinterPrintOptions";
}
impl ::core::convert::From<PosPrinterPrintOptions> for ::windows::core::IUnknown {
    fn from(value: PosPrinterPrintOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PosPrinterPrintOptions> for ::windows::core::IUnknown {
    fn from(value: &PosPrinterPrintOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PosPrinterPrintOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PosPrinterPrintOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PosPrinterPrintOptions> for ::windows::core::IInspectable {
    fn from(value: PosPrinterPrintOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PosPrinterPrintOptions> for ::windows::core::IInspectable {
    fn from(value: &PosPrinterPrintOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PosPrinterPrintOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PosPrinterPrintOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PosPrinterPrintOptions {}
unsafe impl ::core::marker::Sync for PosPrinterPrintOptions {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PosPrinterPrintSide(pub i32);
impl PosPrinterPrintSide {
    pub const Unknown: Self = Self(0i32);
    pub const Side1: Self = Self(1i32);
    pub const Side2: Self = Self(2i32);
}
impl ::core::marker::Copy for PosPrinterPrintSide {}
impl ::core::clone::Clone for PosPrinterPrintSide {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PosPrinterPrintSide {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PosPrinterPrintSide {
    type Abi = Self;
}
impl ::core::fmt::Debug for PosPrinterPrintSide {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterPrintSide").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PosPrinterPrintSide {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterPrintSide;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct PosPrinterReleaseDeviceRequestedEventArgs(::windows::core::IUnknown);
impl PosPrinterReleaseDeviceRequestedEventArgs {}
impl ::core::clone::Clone for PosPrinterReleaseDeviceRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PosPrinterReleaseDeviceRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PosPrinterReleaseDeviceRequestedEventArgs {}
impl ::core::fmt::Debug for PosPrinterReleaseDeviceRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterReleaseDeviceRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PosPrinterReleaseDeviceRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.PosPrinterReleaseDeviceRequestedEventArgs;{2bcba359-1cef-40b2-9ecb-f927f856ae3c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PosPrinterReleaseDeviceRequestedEventArgs {
    type Vtable = IPosPrinterReleaseDeviceRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IPosPrinterReleaseDeviceRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PosPrinterReleaseDeviceRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.PosPrinterReleaseDeviceRequestedEventArgs";
}
impl ::core::convert::From<PosPrinterReleaseDeviceRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PosPrinterReleaseDeviceRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PosPrinterReleaseDeviceRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PosPrinterReleaseDeviceRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PosPrinterReleaseDeviceRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PosPrinterReleaseDeviceRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PosPrinterReleaseDeviceRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PosPrinterReleaseDeviceRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PosPrinterReleaseDeviceRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PosPrinterReleaseDeviceRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PosPrinterReleaseDeviceRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PosPrinterReleaseDeviceRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PosPrinterReleaseDeviceRequestedEventArgs {}
unsafe impl ::core::marker::Sync for PosPrinterReleaseDeviceRequestedEventArgs {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PosPrinterRotation(pub i32);
impl PosPrinterRotation {
    pub const Normal: Self = Self(0i32);
    pub const Right90: Self = Self(1i32);
    pub const Left90: Self = Self(2i32);
    pub const Rotate180: Self = Self(3i32);
}
impl ::core::marker::Copy for PosPrinterRotation {}
impl ::core::clone::Clone for PosPrinterRotation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PosPrinterRotation {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PosPrinterRotation {
    type Abi = Self;
}
impl ::core::fmt::Debug for PosPrinterRotation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterRotation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PosPrinterRotation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterRotation;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PosPrinterRuledLineCapabilities(pub u32);
impl PosPrinterRuledLineCapabilities {
    pub const None: Self = Self(0u32);
    pub const Horizontal: Self = Self(1u32);
    pub const Vertical: Self = Self(2u32);
}
impl ::core::marker::Copy for PosPrinterRuledLineCapabilities {}
impl ::core::clone::Clone for PosPrinterRuledLineCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PosPrinterRuledLineCapabilities {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PosPrinterRuledLineCapabilities {
    type Abi = Self;
}
impl ::core::fmt::Debug for PosPrinterRuledLineCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterRuledLineCapabilities").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PosPrinterRuledLineCapabilities {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PosPrinterRuledLineCapabilities {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PosPrinterRuledLineCapabilities {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PosPrinterRuledLineCapabilities {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PosPrinterRuledLineCapabilities {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for PosPrinterRuledLineCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterRuledLineCapabilities;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct PosPrinterStatus(::windows::core::IUnknown);
impl PosPrinterStatus {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn StatusKind(&self) -> ::windows::core::Result<PosPrinterStatusKind> {
        let this = self;
        unsafe {
            let mut result__: PosPrinterStatusKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StatusKind)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterStatusKind>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn ExtendedStatus(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedStatus)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for PosPrinterStatus {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PosPrinterStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PosPrinterStatus {}
impl ::core::fmt::Debug for PosPrinterStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PosPrinterStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.PosPrinterStatus;{d1f0c730-da40-4328-bf76-5156fa33b747})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PosPrinterStatus {
    type Vtable = IPosPrinterStatus_Vtbl;
    const IID: ::windows::core::GUID = <IPosPrinterStatus as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PosPrinterStatus {
    const NAME: &'static str = "Windows.Devices.PointOfService.PosPrinterStatus";
}
impl ::core::convert::From<PosPrinterStatus> for ::windows::core::IUnknown {
    fn from(value: PosPrinterStatus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PosPrinterStatus> for ::windows::core::IUnknown {
    fn from(value: &PosPrinterStatus) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PosPrinterStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PosPrinterStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PosPrinterStatus> for ::windows::core::IInspectable {
    fn from(value: PosPrinterStatus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PosPrinterStatus> for ::windows::core::IInspectable {
    fn from(value: &PosPrinterStatus) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PosPrinterStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PosPrinterStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PosPrinterStatus {}
unsafe impl ::core::marker::Sync for PosPrinterStatus {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PosPrinterStatusKind(pub i32);
impl PosPrinterStatusKind {
    pub const Online: Self = Self(0i32);
    pub const Off: Self = Self(1i32);
    pub const Offline: Self = Self(2i32);
    pub const OffOrOffline: Self = Self(3i32);
    pub const Extended: Self = Self(4i32);
}
impl ::core::marker::Copy for PosPrinterStatusKind {}
impl ::core::clone::Clone for PosPrinterStatusKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PosPrinterStatusKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PosPrinterStatusKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for PosPrinterStatusKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterStatusKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PosPrinterStatusKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.PosPrinterStatusKind;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct PosPrinterStatusUpdatedEventArgs(::windows::core::IUnknown);
impl PosPrinterStatusUpdatedEventArgs {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<PosPrinterStatus> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterStatus>(result__)
        }
    }
}
impl ::core::clone::Clone for PosPrinterStatusUpdatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PosPrinterStatusUpdatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PosPrinterStatusUpdatedEventArgs {}
impl ::core::fmt::Debug for PosPrinterStatusUpdatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PosPrinterStatusUpdatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PosPrinterStatusUpdatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.PosPrinterStatusUpdatedEventArgs;{2edb87df-13a6-428d-ba81-b0e7c3e5a3cd})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PosPrinterStatusUpdatedEventArgs {
    type Vtable = IPosPrinterStatusUpdatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IPosPrinterStatusUpdatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PosPrinterStatusUpdatedEventArgs {
    const NAME: &'static str = "Windows.Devices.PointOfService.PosPrinterStatusUpdatedEventArgs";
}
impl ::core::convert::From<PosPrinterStatusUpdatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PosPrinterStatusUpdatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PosPrinterStatusUpdatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PosPrinterStatusUpdatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PosPrinterStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PosPrinterStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PosPrinterStatusUpdatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PosPrinterStatusUpdatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PosPrinterStatusUpdatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PosPrinterStatusUpdatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PosPrinterStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PosPrinterStatusUpdatedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PosPrinterStatusUpdatedEventArgs {}
unsafe impl ::core::marker::Sync for PosPrinterStatusUpdatedEventArgs {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct ReceiptPrintJob(::windows::core::IUnknown);
impl ReceiptPrintJob {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Print<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, data: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Print)(::core::mem::transmute_copy(this), data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn PrintLine<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, data: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).PrintLine)(::core::mem::transmute_copy(this), data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn PrintNewline(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).PrintNewline)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ExecuteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExecuteAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetBarcodeRotation(&self, value: PosPrinterRotation) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetBarcodeRotation)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetPrintRotation(&self, value: PosPrinterRotation, includebitmaps: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPrintRotation)(::core::mem::transmute_copy(this), value, includebitmaps).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPrintArea<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetPrintArea)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetBitmap<'a, Param1: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmapnumber: u32, bitmap: Param1, alignment: PosPrinterAlignment) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetBitmap)(::core::mem::transmute_copy(this), bitmapnumber, bitmap.into_param().abi(), alignment).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetBitmapCustomWidthStandardAlign<'a, Param1: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmapnumber: u32, bitmap: Param1, alignment: PosPrinterAlignment, width: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetBitmapCustomWidthStandardAlign)(::core::mem::transmute_copy(this), bitmapnumber, bitmap.into_param().abi(), alignment, width).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetCustomAlignedBitmap<'a, Param1: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmapnumber: u32, bitmap: Param1, alignmentdistance: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetCustomAlignedBitmap)(::core::mem::transmute_copy(this), bitmapnumber, bitmap.into_param().abi(), alignmentdistance).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetBitmapCustomWidthCustomAlign<'a, Param1: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmapnumber: u32, bitmap: Param1, alignmentdistance: u32, width: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetBitmapCustomWidthCustomAlign)(::core::mem::transmute_copy(this), bitmapnumber, bitmap.into_param().abi(), alignmentdistance, width).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn PrintSavedBitmap(&self, bitmapnumber: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).PrintSavedBitmap)(::core::mem::transmute_copy(this), bitmapnumber).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn DrawRuledLine<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, positionlist: Param0, linedirection: PosPrinterLineDirection, linewidth: u32, linestyle: PosPrinterLineStyle, linecolor: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).DrawRuledLine)(::core::mem::transmute_copy(this), positionlist.into_param().abi(), linedirection, linewidth, linestyle, linecolor).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn PrintBarcode<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, data: Param0, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignment: PosPrinterAlignment) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).PrintBarcode)(::core::mem::transmute_copy(this), data.into_param().abi(), symbology, height, width, textposition, alignment).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn PrintBarcodeCustomAlign<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, data: Param0, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignmentdistance: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).PrintBarcodeCustomAlign)(::core::mem::transmute_copy(this), data.into_param().abi(), symbology, height, width, textposition, alignmentdistance).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn PrintBitmap<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmap: Param0, alignment: PosPrinterAlignment) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).PrintBitmap)(::core::mem::transmute_copy(this), bitmap.into_param().abi(), alignment).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn PrintBitmapCustomWidthStandardAlign<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmap: Param0, alignment: PosPrinterAlignment, width: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).PrintBitmapCustomWidthStandardAlign)(::core::mem::transmute_copy(this), bitmap.into_param().abi(), alignment, width).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn PrintCustomAlignedBitmap<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmap: Param0, alignmentdistance: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).PrintCustomAlignedBitmap)(::core::mem::transmute_copy(this), bitmap.into_param().abi(), alignmentdistance).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn PrintBitmapCustomWidthCustomAlign<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmap: Param0, alignmentdistance: u32, width: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptOrSlipJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).PrintBitmapCustomWidthCustomAlign)(::core::mem::transmute_copy(this), bitmap.into_param().abi(), alignmentdistance, width).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn MarkFeed(&self, kind: PosPrinterMarkFeedKind) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).MarkFeed)(::core::mem::transmute_copy(this), kind).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn CutPaper(&self, percentage: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).CutPaper)(::core::mem::transmute_copy(this), percentage).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn CutPaperDefault(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).CutPaperDefault)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn StampPaper(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptPrintJob2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).StampPaper)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Print2<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, PosPrinterPrintOptions>>(&self, data: Param0, printoptions: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptPrintJob2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Print)(::core::mem::transmute_copy(this), data.into_param().abi(), printoptions.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn FeedPaperByLine(&self, linecount: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptPrintJob2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).FeedPaperByLine)(::core::mem::transmute_copy(this), linecount).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn FeedPaperByMapModeUnit(&self, distance: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IReceiptPrintJob2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).FeedPaperByMapModeUnit)(::core::mem::transmute_copy(this), distance).ok() }
    }
}
impl ::core::clone::Clone for ReceiptPrintJob {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ReceiptPrintJob {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ReceiptPrintJob {}
impl ::core::fmt::Debug for ReceiptPrintJob {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ReceiptPrintJob").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ReceiptPrintJob {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ReceiptPrintJob;{aa96066e-acad-4b79-9d0f-c0cfc08dc77b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ReceiptPrintJob {
    type Vtable = IReceiptPrintJob_Vtbl;
    const IID: ::windows::core::GUID = <IReceiptPrintJob as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ReceiptPrintJob {
    const NAME: &'static str = "Windows.Devices.PointOfService.ReceiptPrintJob";
}
impl ::core::convert::From<ReceiptPrintJob> for ::windows::core::IUnknown {
    fn from(value: ReceiptPrintJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ReceiptPrintJob> for ::windows::core::IUnknown {
    fn from(value: &ReceiptPrintJob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ReceiptPrintJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ReceiptPrintJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ReceiptPrintJob> for ::windows::core::IInspectable {
    fn from(value: ReceiptPrintJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ReceiptPrintJob> for ::windows::core::IInspectable {
    fn from(value: &ReceiptPrintJob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ReceiptPrintJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ReceiptPrintJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct ReceiptPrinterCapabilities(::windows::core::IUnknown);
impl ReceiptPrinterCapabilities {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsPrinterPresent(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsPrinterPresent)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsDualColorSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDualColorSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn ColorCartridgeCapabilities(&self) -> ::windows::core::Result<PosPrinterColorCapabilities> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: PosPrinterColorCapabilities = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ColorCartridgeCapabilities)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterColorCapabilities>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn CartridgeSensors(&self) -> ::windows::core::Result<PosPrinterCartridgeSensors> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: PosPrinterCartridgeSensors = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CartridgeSensors)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterCartridgeSensors>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsBoldSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsBoldSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsItalicSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsItalicSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsUnderlineSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsUnderlineSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsDoubleHighPrintSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDoubleHighPrintSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsDoubleWidePrintSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDoubleWidePrintSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsDoubleHighDoubleWidePrintSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDoubleHighDoubleWidePrintSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsPaperEmptySensorSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsPaperEmptySensorSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsPaperNearEndSensorSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsPaperNearEndSensorSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedCharactersPerLine(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SupportedCharactersPerLine)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsBarcodeSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsBarcodeSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsBitmapSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsBitmapSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsLeft90RotationSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsLeft90RotationSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsRight90RotationSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsRight90RotationSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Is180RotationSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Is180RotationSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsPrintAreaSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsPrintAreaSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn RuledLineCapabilities(&self) -> ::windows::core::Result<PosPrinterRuledLineCapabilities> {
        let this = &::windows::core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__: PosPrinterRuledLineCapabilities = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RuledLineCapabilities)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterRuledLineCapabilities>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedBarcodeRotations(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>> {
        let this = &::windows::core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SupportedBarcodeRotations)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedBitmapRotations(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>> {
        let this = &::windows::core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SupportedBitmapRotations)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn CanCutPaper(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanCutPaper)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsStampSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsStampSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn MarkFeedCapabilities(&self) -> ::windows::core::Result<PosPrinterMarkFeedCapabilities> {
        let this = self;
        unsafe {
            let mut result__: PosPrinterMarkFeedCapabilities = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MarkFeedCapabilities)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterMarkFeedCapabilities>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsReverseVideoSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IReceiptPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsReverseVideoSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsStrikethroughSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IReceiptPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsStrikethroughSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsSuperscriptSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IReceiptPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsSuperscriptSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsSubscriptSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IReceiptPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsSubscriptSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsReversePaperFeedByLineSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IReceiptPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsReversePaperFeedByLineSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsReversePaperFeedByMapModeUnitSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IReceiptPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsReversePaperFeedByMapModeUnitSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for ReceiptPrinterCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ReceiptPrinterCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ReceiptPrinterCapabilities {}
impl ::core::fmt::Debug for ReceiptPrinterCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ReceiptPrinterCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ReceiptPrinterCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.ReceiptPrinterCapabilities;{b8f0b58f-51a8-43fc-9bd5-8de272a6415b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ReceiptPrinterCapabilities {
    type Vtable = IReceiptPrinterCapabilities_Vtbl;
    const IID: ::windows::core::GUID = <IReceiptPrinterCapabilities as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ReceiptPrinterCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.ReceiptPrinterCapabilities";
}
impl ::core::convert::From<ReceiptPrinterCapabilities> for ::windows::core::IUnknown {
    fn from(value: ReceiptPrinterCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ReceiptPrinterCapabilities> for ::windows::core::IUnknown {
    fn from(value: &ReceiptPrinterCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ReceiptPrinterCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ReceiptPrinterCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ReceiptPrinterCapabilities> for ::windows::core::IInspectable {
    fn from(value: ReceiptPrinterCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ReceiptPrinterCapabilities> for ::windows::core::IInspectable {
    fn from(value: &ReceiptPrinterCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ReceiptPrinterCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ReceiptPrinterCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
#[repr(C)]
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
pub struct SizeUInt32 {
    pub Width: u32,
    pub Height: u32,
}
impl ::core::marker::Copy for SizeUInt32 {}
impl ::core::clone::Clone for SizeUInt32 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SizeUInt32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SizeUInt32").field("Width", &self.Width).field("Height", &self.Height).finish()
    }
}
unsafe impl ::windows::core::Abi for SizeUInt32 {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for SizeUInt32 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Devices.PointOfService.SizeUInt32;u4;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for SizeUInt32 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SizeUInt32>()) == 0 }
    }
}
impl ::core::cmp::Eq for SizeUInt32 {}
impl ::core::default::Default for SizeUInt32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct SlipPrintJob(::windows::core::IUnknown);
impl SlipPrintJob {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Print<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, data: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Print)(::core::mem::transmute_copy(this), data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn PrintLine<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, data: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).PrintLine)(::core::mem::transmute_copy(this), data.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn PrintNewline(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).PrintNewline)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ExecuteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<IPosPrinterJob>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExecuteAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetBarcodeRotation(&self, value: PosPrinterRotation) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBarcodeRotation)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn SetPrintRotation(&self, value: PosPrinterRotation, includebitmaps: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPrintRotation)(::core::mem::transmute_copy(this), value, includebitmaps).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPrintArea<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPrintArea)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetBitmap<'a, Param1: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmapnumber: u32, bitmap: Param1, alignment: PosPrinterAlignment) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBitmap)(::core::mem::transmute_copy(this), bitmapnumber, bitmap.into_param().abi(), alignment).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetBitmapCustomWidthStandardAlign<'a, Param1: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmapnumber: u32, bitmap: Param1, alignment: PosPrinterAlignment, width: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBitmapCustomWidthStandardAlign)(::core::mem::transmute_copy(this), bitmapnumber, bitmap.into_param().abi(), alignment, width).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetCustomAlignedBitmap<'a, Param1: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmapnumber: u32, bitmap: Param1, alignmentdistance: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCustomAlignedBitmap)(::core::mem::transmute_copy(this), bitmapnumber, bitmap.into_param().abi(), alignmentdistance).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetBitmapCustomWidthCustomAlign<'a, Param1: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmapnumber: u32, bitmap: Param1, alignmentdistance: u32, width: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBitmapCustomWidthCustomAlign)(::core::mem::transmute_copy(this), bitmapnumber, bitmap.into_param().abi(), alignmentdistance, width).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn PrintSavedBitmap(&self, bitmapnumber: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).PrintSavedBitmap)(::core::mem::transmute_copy(this), bitmapnumber).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn DrawRuledLine<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, positionlist: Param0, linedirection: PosPrinterLineDirection, linewidth: u32, linestyle: PosPrinterLineStyle, linecolor: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).DrawRuledLine)(::core::mem::transmute_copy(this), positionlist.into_param().abi(), linedirection, linewidth, linestyle, linecolor).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn PrintBarcode<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, data: Param0, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignment: PosPrinterAlignment) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).PrintBarcode)(::core::mem::transmute_copy(this), data.into_param().abi(), symbology, height, width, textposition, alignment).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn PrintBarcodeCustomAlign<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, data: Param0, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignmentdistance: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).PrintBarcodeCustomAlign)(::core::mem::transmute_copy(this), data.into_param().abi(), symbology, height, width, textposition, alignmentdistance).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn PrintBitmap<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmap: Param0, alignment: PosPrinterAlignment) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).PrintBitmap)(::core::mem::transmute_copy(this), bitmap.into_param().abi(), alignment).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn PrintBitmapCustomWidthStandardAlign<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmap: Param0, alignment: PosPrinterAlignment, width: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).PrintBitmapCustomWidthStandardAlign)(::core::mem::transmute_copy(this), bitmap.into_param().abi(), alignment, width).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn PrintCustomAlignedBitmap<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmap: Param0, alignmentdistance: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).PrintCustomAlignedBitmap)(::core::mem::transmute_copy(this), bitmap.into_param().abi(), alignmentdistance).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn PrintBitmapCustomWidthCustomAlign<'a, Param0: ::windows::core::IntoParam<'a, super::super::Graphics::Imaging::BitmapFrame>>(&self, bitmap: Param0, alignmentdistance: u32, width: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).PrintBitmapCustomWidthCustomAlign)(::core::mem::transmute_copy(this), bitmap.into_param().abi(), alignmentdistance, width).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Print2<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, PosPrinterPrintOptions>>(&self, data: Param0, printoptions: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISlipPrintJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Print)(::core::mem::transmute_copy(this), data.into_param().abi(), printoptions.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn FeedPaperByLine(&self, linecount: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISlipPrintJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).FeedPaperByLine)(::core::mem::transmute_copy(this), linecount).ok() }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn FeedPaperByMapModeUnit(&self, distance: i32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ISlipPrintJob>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).FeedPaperByMapModeUnit)(::core::mem::transmute_copy(this), distance).ok() }
    }
}
impl ::core::clone::Clone for SlipPrintJob {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SlipPrintJob {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SlipPrintJob {}
impl ::core::fmt::Debug for SlipPrintJob {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SlipPrintJob").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SlipPrintJob {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.SlipPrintJob;{532199be-c8c3-4dc2-89e9-5c4a37b34ddc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SlipPrintJob {
    type Vtable = IReceiptOrSlipJob_Vtbl;
    const IID: ::windows::core::GUID = <IReceiptOrSlipJob as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SlipPrintJob {
    const NAME: &'static str = "Windows.Devices.PointOfService.SlipPrintJob";
}
impl ::core::convert::From<SlipPrintJob> for ::windows::core::IUnknown {
    fn from(value: SlipPrintJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SlipPrintJob> for ::windows::core::IUnknown {
    fn from(value: &SlipPrintJob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SlipPrintJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SlipPrintJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SlipPrintJob> for ::windows::core::IInspectable {
    fn from(value: SlipPrintJob) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SlipPrintJob> for ::windows::core::IInspectable {
    fn from(value: &SlipPrintJob) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SlipPrintJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SlipPrintJob {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
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
impl ::core::convert::TryFrom<SlipPrintJob> for IReceiptOrSlipJob {
    type Error = ::windows::core::Error;
    fn try_from(value: SlipPrintJob) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&SlipPrintJob> for IReceiptOrSlipJob {
    type Error = ::windows::core::Error;
    fn try_from(value: &SlipPrintJob) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IReceiptOrSlipJob> for SlipPrintJob {
    fn into_param(self) -> ::windows::core::Param<'a, IReceiptOrSlipJob> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IReceiptOrSlipJob> for &SlipPrintJob {
    fn into_param(self) -> ::windows::core::Param<'a, IReceiptOrSlipJob> {
        ::core::convert::TryInto::<IReceiptOrSlipJob>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SlipPrintJob {}
unsafe impl ::core::marker::Sync for SlipPrintJob {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct SlipPrinterCapabilities(::windows::core::IUnknown);
impl SlipPrinterCapabilities {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsPrinterPresent(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsPrinterPresent)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsDualColorSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDualColorSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn ColorCartridgeCapabilities(&self) -> ::windows::core::Result<PosPrinterColorCapabilities> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: PosPrinterColorCapabilities = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ColorCartridgeCapabilities)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterColorCapabilities>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn CartridgeSensors(&self) -> ::windows::core::Result<PosPrinterCartridgeSensors> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: PosPrinterCartridgeSensors = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CartridgeSensors)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterCartridgeSensors>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsBoldSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsBoldSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsItalicSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsItalicSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsUnderlineSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsUnderlineSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsDoubleHighPrintSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDoubleHighPrintSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsDoubleWidePrintSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDoubleWidePrintSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsDoubleHighDoubleWidePrintSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsDoubleHighDoubleWidePrintSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsPaperEmptySensorSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsPaperEmptySensorSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsPaperNearEndSensorSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsPaperNearEndSensorSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedCharactersPerLine(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &::windows::core::Interface::cast::<ICommonPosPrintStationCapabilities>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SupportedCharactersPerLine)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsBarcodeSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsBarcodeSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsBitmapSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsBitmapSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsLeft90RotationSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsLeft90RotationSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsRight90RotationSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsRight90RotationSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Is180RotationSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Is180RotationSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsPrintAreaSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsPrintAreaSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn RuledLineCapabilities(&self) -> ::windows::core::Result<PosPrinterRuledLineCapabilities> {
        let this = &::windows::core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__: PosPrinterRuledLineCapabilities = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RuledLineCapabilities)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PosPrinterRuledLineCapabilities>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedBarcodeRotations(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>> {
        let this = &::windows::core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SupportedBarcodeRotations)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SupportedBitmapRotations(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>> {
        let this = &::windows::core::Interface::cast::<ICommonReceiptSlipCapabilities>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SupportedBitmapRotations)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsFullLengthSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsFullLengthSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsBothSidesPrintingSupported(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsBothSidesPrintingSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsReverseVideoSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ISlipPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsReverseVideoSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsStrikethroughSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ISlipPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsStrikethroughSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsSuperscriptSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ISlipPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsSuperscriptSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsSubscriptSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ISlipPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsSubscriptSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsReversePaperFeedByLineSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ISlipPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsReversePaperFeedByLineSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn IsReversePaperFeedByMapModeUnitSupported(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ISlipPrinterCapabilities2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsReversePaperFeedByMapModeUnitSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for SlipPrinterCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SlipPrinterCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SlipPrinterCapabilities {}
impl ::core::fmt::Debug for SlipPrinterCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SlipPrinterCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SlipPrinterCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.SlipPrinterCapabilities;{99b16399-488c-4157-8ac2-9f57f708d3db})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SlipPrinterCapabilities {
    type Vtable = ISlipPrinterCapabilities_Vtbl;
    const IID: ::windows::core::GUID = <ISlipPrinterCapabilities as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SlipPrinterCapabilities {
    const NAME: &'static str = "Windows.Devices.PointOfService.SlipPrinterCapabilities";
}
impl ::core::convert::From<SlipPrinterCapabilities> for ::windows::core::IUnknown {
    fn from(value: SlipPrinterCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SlipPrinterCapabilities> for ::windows::core::IUnknown {
    fn from(value: &SlipPrinterCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SlipPrinterCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SlipPrinterCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SlipPrinterCapabilities> for ::windows::core::IInspectable {
    fn from(value: SlipPrinterCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SlipPrinterCapabilities> for ::windows::core::IInspectable {
    fn from(value: &SlipPrinterCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SlipPrinterCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SlipPrinterCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
pub struct UnifiedPosErrorData(::windows::core::IUnknown);
impl UnifiedPosErrorData {
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Message(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Message)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Severity(&self) -> ::windows::core::Result<UnifiedPosErrorSeverity> {
        let this = self;
        unsafe {
            let mut result__: UnifiedPosErrorSeverity = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Severity)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UnifiedPosErrorSeverity>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn Reason(&self) -> ::windows::core::Result<UnifiedPosErrorReason> {
        let this = self;
        unsafe {
            let mut result__: UnifiedPosErrorReason = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Reason)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UnifiedPosErrorReason>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn ExtendedReason(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedReason)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Devices_PointOfService\"`*"]
    pub fn CreateInstance<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(message: Param0, severity: UnifiedPosErrorSeverity, reason: UnifiedPosErrorReason, extendedreason: u32) -> ::windows::core::Result<UnifiedPosErrorData> {
        Self::IUnifiedPosErrorDataFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), message.into_param().abi(), severity, reason, extendedreason, &mut result__).from_abi::<UnifiedPosErrorData>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IUnifiedPosErrorDataFactory<R, F: FnOnce(&IUnifiedPosErrorDataFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UnifiedPosErrorData, IUnifiedPosErrorDataFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for UnifiedPosErrorData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UnifiedPosErrorData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UnifiedPosErrorData {}
impl ::core::fmt::Debug for UnifiedPosErrorData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UnifiedPosErrorData").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UnifiedPosErrorData {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Devices.PointOfService.UnifiedPosErrorData;{2b998c3a-555c-4889-8ed8-c599bb3a712a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for UnifiedPosErrorData {
    type Vtable = IUnifiedPosErrorData_Vtbl;
    const IID: ::windows::core::GUID = <IUnifiedPosErrorData as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UnifiedPosErrorData {
    const NAME: &'static str = "Windows.Devices.PointOfService.UnifiedPosErrorData";
}
impl ::core::convert::From<UnifiedPosErrorData> for ::windows::core::IUnknown {
    fn from(value: UnifiedPosErrorData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UnifiedPosErrorData> for ::windows::core::IUnknown {
    fn from(value: &UnifiedPosErrorData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UnifiedPosErrorData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UnifiedPosErrorData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UnifiedPosErrorData> for ::windows::core::IInspectable {
    fn from(value: UnifiedPosErrorData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UnifiedPosErrorData> for ::windows::core::IInspectable {
    fn from(value: &UnifiedPosErrorData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UnifiedPosErrorData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UnifiedPosErrorData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UnifiedPosErrorData {}
unsafe impl ::core::marker::Sync for UnifiedPosErrorData {}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UnifiedPosErrorReason(pub i32);
impl UnifiedPosErrorReason {
    pub const UnknownErrorReason: Self = Self(0i32);
    pub const NoService: Self = Self(1i32);
    pub const Disabled: Self = Self(2i32);
    pub const Illegal: Self = Self(3i32);
    pub const NoHardware: Self = Self(4i32);
    pub const Closed: Self = Self(5i32);
    pub const Offline: Self = Self(6i32);
    pub const Failure: Self = Self(7i32);
    pub const Timeout: Self = Self(8i32);
    pub const Busy: Self = Self(9i32);
    pub const Extended: Self = Self(10i32);
}
impl ::core::marker::Copy for UnifiedPosErrorReason {}
impl ::core::clone::Clone for UnifiedPosErrorReason {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UnifiedPosErrorReason {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UnifiedPosErrorReason {
    type Abi = Self;
}
impl ::core::fmt::Debug for UnifiedPosErrorReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UnifiedPosErrorReason").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UnifiedPosErrorReason {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.UnifiedPosErrorReason;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UnifiedPosErrorSeverity(pub i32);
impl UnifiedPosErrorSeverity {
    pub const UnknownErrorSeverity: Self = Self(0i32);
    pub const Warning: Self = Self(1i32);
    pub const Recoverable: Self = Self(2i32);
    pub const Unrecoverable: Self = Self(3i32);
    pub const AssistanceRequired: Self = Self(4i32);
    pub const Fatal: Self = Self(5i32);
}
impl ::core::marker::Copy for UnifiedPosErrorSeverity {}
impl ::core::clone::Clone for UnifiedPosErrorSeverity {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UnifiedPosErrorSeverity {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UnifiedPosErrorSeverity {
    type Abi = Self;
}
impl ::core::fmt::Debug for UnifiedPosErrorSeverity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UnifiedPosErrorSeverity").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UnifiedPosErrorSeverity {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.UnifiedPosErrorSeverity;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UnifiedPosHealthCheckLevel(pub i32);
impl UnifiedPosHealthCheckLevel {
    pub const UnknownHealthCheckLevel: Self = Self(0i32);
    pub const POSInternal: Self = Self(1i32);
    pub const External: Self = Self(2i32);
    pub const Interactive: Self = Self(3i32);
}
impl ::core::marker::Copy for UnifiedPosHealthCheckLevel {}
impl ::core::clone::Clone for UnifiedPosHealthCheckLevel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UnifiedPosHealthCheckLevel {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UnifiedPosHealthCheckLevel {
    type Abi = Self;
}
impl ::core::fmt::Debug for UnifiedPosHealthCheckLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UnifiedPosHealthCheckLevel").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UnifiedPosHealthCheckLevel {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.UnifiedPosHealthCheckLevel;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Devices_PointOfService\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UnifiedPosPowerReportingType(pub i32);
impl UnifiedPosPowerReportingType {
    pub const UnknownPowerReportingType: Self = Self(0i32);
    pub const Standard: Self = Self(1i32);
    pub const Advanced: Self = Self(2i32);
}
impl ::core::marker::Copy for UnifiedPosPowerReportingType {}
impl ::core::clone::Clone for UnifiedPosPowerReportingType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UnifiedPosPowerReportingType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UnifiedPosPowerReportingType {
    type Abi = Self;
}
impl ::core::fmt::Debug for UnifiedPosPowerReportingType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UnifiedPosPowerReportingType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UnifiedPosPowerReportingType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Devices.PointOfService.UnifiedPosPowerReportingType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Capabilities(&self) -> ::windows::core::Result<BarcodeScannerCapabilities>;
    fn ClaimScannerAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ClaimedBarcodeScanner>>;
    fn CheckHealthAsync(&self, level: UnifiedPosHealthCheckLevel) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetSupportedSymbologiesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<u32>>>;
    fn IsSymbologySupportedAsync(&self, barcodesymbology: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RetrieveStatisticsAsync(&self, statisticscategories: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>>;
    fn GetSupportedProfiles(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn IsProfileSupported(&self, profile: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn StatusUpdated(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<BarcodeScanner, BarcodeScannerStatusUpdatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStatusUpdated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScanner2Impl: Sized {
    fn VideoDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerCapabilitiesImpl: Sized {
    fn PowerReportingType(&self) -> ::windows::core::Result<UnifiedPosPowerReportingType>;
    fn IsStatisticsReportingSupported(&self) -> ::windows::core::Result<bool>;
    fn IsStatisticsUpdatingSupported(&self) -> ::windows::core::Result<bool>;
    fn IsImagePreviewSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerCapabilities1Impl: Sized {
    fn IsSoftwareTriggerSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerCapabilities2Impl: Sized {
    fn IsVideoPreviewSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerDataReceivedEventArgsImpl: Sized {
    fn Report(&self) -> ::windows::core::Result<BarcodeScannerReport>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerErrorOccurredEventArgsImpl: Sized {
    fn PartialInputData(&self) -> ::windows::core::Result<BarcodeScannerReport>;
    fn IsRetriable(&self) -> ::windows::core::Result<bool>;
    fn ErrorData(&self) -> ::windows::core::Result<UnifiedPosErrorData>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerImagePreviewReceivedEventArgsImpl: Sized {
    fn Preview(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamWithContentType>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerReportImpl: Sized {
    fn ScanDataType(&self) -> ::windows::core::Result<u32>;
    fn ScanData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn ScanDataLabel(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerReportFactoryImpl: Sized {
    fn CreateInstance(&self, scandatatype: u32, scandata: &::core::option::Option<super::super::Storage::Streams::IBuffer>, scandatalabel: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<BarcodeScannerReport>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerStaticsImpl: Sized {
    fn GetDefaultAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BarcodeScanner>>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BarcodeScanner>>;
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerStatics2Impl: Sized {
    fn GetDeviceSelectorWithConnectionTypes(&self, connectiontypes: PosConnectionTypes) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerStatusUpdatedEventArgsImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<BarcodeScannerStatus>;
    fn ExtendedStatus(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeSymbologiesStaticsImpl: Sized {
    fn Unknown(&self) -> ::windows::core::Result<u32>;
    fn Ean8(&self) -> ::windows::core::Result<u32>;
    fn Ean8Add2(&self) -> ::windows::core::Result<u32>;
    fn Ean8Add5(&self) -> ::windows::core::Result<u32>;
    fn Eanv(&self) -> ::windows::core::Result<u32>;
    fn EanvAdd2(&self) -> ::windows::core::Result<u32>;
    fn EanvAdd5(&self) -> ::windows::core::Result<u32>;
    fn Ean13(&self) -> ::windows::core::Result<u32>;
    fn Ean13Add2(&self) -> ::windows::core::Result<u32>;
    fn Ean13Add5(&self) -> ::windows::core::Result<u32>;
    fn Isbn(&self) -> ::windows::core::Result<u32>;
    fn IsbnAdd5(&self) -> ::windows::core::Result<u32>;
    fn Ismn(&self) -> ::windows::core::Result<u32>;
    fn IsmnAdd2(&self) -> ::windows::core::Result<u32>;
    fn IsmnAdd5(&self) -> ::windows::core::Result<u32>;
    fn Issn(&self) -> ::windows::core::Result<u32>;
    fn IssnAdd2(&self) -> ::windows::core::Result<u32>;
    fn IssnAdd5(&self) -> ::windows::core::Result<u32>;
    fn Ean99(&self) -> ::windows::core::Result<u32>;
    fn Ean99Add2(&self) -> ::windows::core::Result<u32>;
    fn Ean99Add5(&self) -> ::windows::core::Result<u32>;
    fn Upca(&self) -> ::windows::core::Result<u32>;
    fn UpcaAdd2(&self) -> ::windows::core::Result<u32>;
    fn UpcaAdd5(&self) -> ::windows::core::Result<u32>;
    fn Upce(&self) -> ::windows::core::Result<u32>;
    fn UpceAdd2(&self) -> ::windows::core::Result<u32>;
    fn UpceAdd5(&self) -> ::windows::core::Result<u32>;
    fn UpcCoupon(&self) -> ::windows::core::Result<u32>;
    fn TfStd(&self) -> ::windows::core::Result<u32>;
    fn TfDis(&self) -> ::windows::core::Result<u32>;
    fn TfInt(&self) -> ::windows::core::Result<u32>;
    fn TfInd(&self) -> ::windows::core::Result<u32>;
    fn TfMat(&self) -> ::windows::core::Result<u32>;
    fn TfIata(&self) -> ::windows::core::Result<u32>;
    fn Gs1DatabarType1(&self) -> ::windows::core::Result<u32>;
    fn Gs1DatabarType2(&self) -> ::windows::core::Result<u32>;
    fn Gs1DatabarType3(&self) -> ::windows::core::Result<u32>;
    fn Code39(&self) -> ::windows::core::Result<u32>;
    fn Code39Ex(&self) -> ::windows::core::Result<u32>;
    fn Trioptic39(&self) -> ::windows::core::Result<u32>;
    fn Code32(&self) -> ::windows::core::Result<u32>;
    fn Pzn(&self) -> ::windows::core::Result<u32>;
    fn Code93(&self) -> ::windows::core::Result<u32>;
    fn Code93Ex(&self) -> ::windows::core::Result<u32>;
    fn Code128(&self) -> ::windows::core::Result<u32>;
    fn Gs1128(&self) -> ::windows::core::Result<u32>;
    fn Gs1128Coupon(&self) -> ::windows::core::Result<u32>;
    fn UccEan128(&self) -> ::windows::core::Result<u32>;
    fn Sisac(&self) -> ::windows::core::Result<u32>;
    fn Isbt(&self) -> ::windows::core::Result<u32>;
    fn Codabar(&self) -> ::windows::core::Result<u32>;
    fn Code11(&self) -> ::windows::core::Result<u32>;
    fn Msi(&self) -> ::windows::core::Result<u32>;
    fn Plessey(&self) -> ::windows::core::Result<u32>;
    fn Telepen(&self) -> ::windows::core::Result<u32>;
    fn Code16k(&self) -> ::windows::core::Result<u32>;
    fn CodablockA(&self) -> ::windows::core::Result<u32>;
    fn CodablockF(&self) -> ::windows::core::Result<u32>;
    fn Codablock128(&self) -> ::windows::core::Result<u32>;
    fn Code49(&self) -> ::windows::core::Result<u32>;
    fn Aztec(&self) -> ::windows::core::Result<u32>;
    fn DataCode(&self) -> ::windows::core::Result<u32>;
    fn DataMatrix(&self) -> ::windows::core::Result<u32>;
    fn HanXin(&self) -> ::windows::core::Result<u32>;
    fn Maxicode(&self) -> ::windows::core::Result<u32>;
    fn MicroPdf417(&self) -> ::windows::core::Result<u32>;
    fn MicroQr(&self) -> ::windows::core::Result<u32>;
    fn Pdf417(&self) -> ::windows::core::Result<u32>;
    fn Qr(&self) -> ::windows::core::Result<u32>;
    fn MsTag(&self) -> ::windows::core::Result<u32>;
    fn Ccab(&self) -> ::windows::core::Result<u32>;
    fn Ccc(&self) -> ::windows::core::Result<u32>;
    fn Tlc39(&self) -> ::windows::core::Result<u32>;
    fn AusPost(&self) -> ::windows::core::Result<u32>;
    fn CanPost(&self) -> ::windows::core::Result<u32>;
    fn ChinaPost(&self) -> ::windows::core::Result<u32>;
    fn DutchKix(&self) -> ::windows::core::Result<u32>;
    fn InfoMail(&self) -> ::windows::core::Result<u32>;
    fn ItalianPost25(&self) -> ::windows::core::Result<u32>;
    fn ItalianPost39(&self) -> ::windows::core::Result<u32>;
    fn JapanPost(&self) -> ::windows::core::Result<u32>;
    fn KoreanPost(&self) -> ::windows::core::Result<u32>;
    fn SwedenPost(&self) -> ::windows::core::Result<u32>;
    fn UkPost(&self) -> ::windows::core::Result<u32>;
    fn UsIntelligent(&self) -> ::windows::core::Result<u32>;
    fn UsIntelligentPkg(&self) -> ::windows::core::Result<u32>;
    fn UsPlanet(&self) -> ::windows::core::Result<u32>;
    fn UsPostNet(&self) -> ::windows::core::Result<u32>;
    fn Us4StateFics(&self) -> ::windows::core::Result<u32>;
    fn OcrA(&self) -> ::windows::core::Result<u32>;
    fn OcrB(&self) -> ::windows::core::Result<u32>;
    fn Micr(&self) -> ::windows::core::Result<u32>;
    fn ExtendedBase(&self) -> ::windows::core::Result<u32>;
    fn GetName(&self, scandatatype: u32) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeSymbologiesStatics2Impl: Sized {
    fn Gs1DWCode(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeSymbologyAttributesImpl: Sized {
    fn IsCheckDigitValidationEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsCheckDigitValidationEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsCheckDigitValidationSupported(&self) -> ::windows::core::Result<bool>;
    fn IsCheckDigitTransmissionEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsCheckDigitTransmissionEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsCheckDigitTransmissionSupported(&self) -> ::windows::core::Result<bool>;
    fn DecodeLength1(&self) -> ::windows::core::Result<u32>;
    fn SetDecodeLength1(&self, value: u32) -> ::windows::core::Result<()>;
    fn DecodeLength2(&self) -> ::windows::core::Result<u32>;
    fn SetDecodeLength2(&self, value: u32) -> ::windows::core::Result<()>;
    fn DecodeLengthKind(&self) -> ::windows::core::Result<BarcodeSymbologyDecodeLengthKind>;
    fn SetDecodeLengthKind(&self, value: BarcodeSymbologyDecodeLengthKind) -> ::windows::core::Result<()>;
    fn IsDecodeLengthSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICashDrawerImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Capabilities(&self) -> ::windows::core::Result<CashDrawerCapabilities>;
    fn Status(&self) -> ::windows::core::Result<CashDrawerStatus>;
    fn IsDrawerOpen(&self) -> ::windows::core::Result<bool>;
    fn DrawerEventSource(&self) -> ::windows::core::Result<CashDrawerEventSource>;
    fn ClaimDrawerAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ClaimedCashDrawer>>;
    fn CheckHealthAsync(&self, level: UnifiedPosHealthCheckLevel) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetStatisticsAsync(&self, statisticscategories: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn StatusUpdated(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CashDrawer, CashDrawerStatusUpdatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStatusUpdated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICashDrawerCapabilitiesImpl: Sized {
    fn PowerReportingType(&self) -> ::windows::core::Result<UnifiedPosPowerReportingType>;
    fn IsStatisticsReportingSupported(&self) -> ::windows::core::Result<bool>;
    fn IsStatisticsUpdatingSupported(&self) -> ::windows::core::Result<bool>;
    fn IsStatusReportingSupported(&self) -> ::windows::core::Result<bool>;
    fn IsStatusMultiDrawerDetectSupported(&self) -> ::windows::core::Result<bool>;
    fn IsDrawerOpenSensorAvailable(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICashDrawerCloseAlarmImpl: Sized {
    fn SetAlarmTimeout(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn AlarmTimeout(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetBeepFrequency(&self, value: u32) -> ::windows::core::Result<()>;
    fn BeepFrequency(&self) -> ::windows::core::Result<u32>;
    fn SetBeepDuration(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn BeepDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetBeepDelay(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn BeepDelay(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn AlarmTimeoutExpired(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CashDrawerCloseAlarm, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAlarmTimeoutExpired(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StartAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICashDrawerEventSourceImpl: Sized {
    fn DrawerClosed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CashDrawerEventSource, CashDrawerClosedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDrawerClosed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DrawerOpened(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<CashDrawerEventSource, CashDrawerOpenedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDrawerOpened(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
pub trait ICashDrawerEventSourceEventArgsImpl: Sized {
    fn CashDrawer(&self) -> ::windows::core::Result<CashDrawer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICashDrawerStaticsImpl: Sized {
    fn GetDefaultAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CashDrawer>>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<CashDrawer>>;
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICashDrawerStatics2Impl: Sized {
    fn GetDeviceSelectorWithConnectionTypes(&self, connectiontypes: PosConnectionTypes) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICashDrawerStatusImpl: Sized {
    fn StatusKind(&self) -> ::windows::core::Result<CashDrawerStatusKind>;
    fn ExtendedStatus(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICashDrawerStatusUpdatedEventArgsImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<CashDrawerStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IClaimedBarcodeScannerImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsDisabledOnDataReceived(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsDisabledOnDataReceived(&self) -> ::windows::core::Result<bool>;
    fn SetIsDecodeDataEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsDecodeDataEnabled(&self) -> ::windows::core::Result<bool>;
    fn EnableAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DisableAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn RetainDevice(&self) -> ::windows::core::Result<()>;
    fn SetActiveSymbologiesAsync(&self, symbologies: &::core::option::Option<super::super::Foundation::Collections::IIterable<u32>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ResetStatisticsAsync(&self, statisticscategories: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn UpdateStatisticsAsync(&self, statistics: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SetActiveProfileAsync(&self, profile: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DataReceived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ClaimedBarcodeScanner, BarcodeScannerDataReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDataReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TriggerPressed(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<ClaimedBarcodeScanner>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTriggerPressed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TriggerReleased(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<ClaimedBarcodeScanner>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTriggerReleased(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ReleaseDeviceRequested(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<ClaimedBarcodeScanner>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReleaseDeviceRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ImagePreviewReceived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ClaimedBarcodeScanner, BarcodeScannerImagePreviewReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveImagePreviewReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ErrorOccurred(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ClaimedBarcodeScanner, BarcodeScannerErrorOccurredEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveErrorOccurred(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IClaimedBarcodeScanner1Impl: Sized {
    fn StartSoftwareTriggerAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn StopSoftwareTriggerAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IClaimedBarcodeScanner2Impl: Sized {
    fn GetSymbologyAttributesAsync(&self, barcodesymbology: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BarcodeSymbologyAttributes>>;
    fn SetSymbologyAttributesAsync(&self, barcodesymbology: u32, attributes: &::core::option::Option<BarcodeSymbologyAttributes>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IClaimedBarcodeScanner3Impl: Sized {
    fn ShowVideoPreviewAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn HideVideoPreview(&self) -> ::windows::core::Result<()>;
    fn SetIsVideoPreviewShownOnEnable(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsVideoPreviewShownOnEnable(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IClaimedBarcodeScanner4Impl: Sized {
    fn Closed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ClaimedBarcodeScanner, ClaimedBarcodeScannerClosedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IClaimedBarcodeScannerClosedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IClaimedCashDrawerImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn IsDrawerOpen(&self) -> ::windows::core::Result<bool>;
    fn CloseAlarm(&self) -> ::windows::core::Result<CashDrawerCloseAlarm>;
    fn OpenDrawerAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn EnableAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn DisableAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RetainDeviceAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn ResetStatisticsAsync(&self, statisticscategories: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn UpdateStatisticsAsync(&self, statistics: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn ReleaseDeviceRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ClaimedCashDrawer, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReleaseDeviceRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IClaimedCashDrawer2Impl: Sized {
    fn Closed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ClaimedCashDrawer, ClaimedCashDrawerClosedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IClaimedCashDrawerClosedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IClaimedJournalPrinterImpl: Sized {
    fn CreateJob(&self) -> ::windows::core::Result<JournalPrintJob>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IClaimedLineDisplayImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Capabilities(&self) -> ::windows::core::Result<LineDisplayCapabilities>;
    fn PhysicalDeviceName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PhysicalDeviceDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DeviceControlDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DeviceControlVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DeviceServiceVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DefaultWindow(&self) -> ::windows::core::Result<LineDisplayWindow>;
    fn RetainDevice(&self) -> ::windows::core::Result<()>;
    fn ReleaseDeviceRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ClaimedLineDisplay, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReleaseDeviceRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IClaimedLineDisplay2Impl: Sized {
    fn GetStatisticsAsync(&self, statisticscategories: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn CheckHealthAsync(&self, level: UnifiedPosHealthCheckLevel) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn CheckPowerStatusAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LineDisplayPowerStatus>>;
    fn StatusUpdated(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ClaimedLineDisplay, LineDisplayStatusUpdatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStatusUpdated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SupportedScreenSizesInCharacters(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Foundation::Size>>;
    fn MaxBitmapSizeInPixels(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn SupportedCharacterSets(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<i32>>;
    fn CustomGlyphs(&self) -> ::windows::core::Result<LineDisplayCustomGlyphs>;
    fn GetAttributes(&self) -> ::windows::core::Result<LineDisplayAttributes>;
    fn TryUpdateAttributesAsync(&self, attributes: &::core::option::Option<LineDisplayAttributes>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TrySetDescriptorAsync(&self, descriptor: u32, descriptorstate: LineDisplayDescriptorState) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryClearDescriptorsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryCreateWindowAsync(&self, viewport: &super::super::Foundation::Rect, windowsize: &super::super::Foundation::Size) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LineDisplayWindow>>;
    fn TryStoreStorageFileBitmapAsync(&self, bitmap: &::core::option::Option<super::super::Storage::StorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LineDisplayStoredBitmap>>;
    fn TryStoreStorageFileBitmapWithAlignmentAsync(&self, bitmap: &::core::option::Option<super::super::Storage::StorageFile>, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LineDisplayStoredBitmap>>;
    fn TryStoreStorageFileBitmapWithAlignmentAndWidthAsync(&self, bitmap: &::core::option::Option<super::super::Storage::StorageFile>, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment, widthinpixels: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LineDisplayStoredBitmap>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IClaimedLineDisplay3Impl: Sized {
    fn Closed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ClaimedLineDisplay, ClaimedLineDisplayClosedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IClaimedLineDisplayClosedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IClaimedLineDisplayStaticsImpl: Sized {
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ClaimedLineDisplay>>;
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorWithConnectionTypes(&self, connectiontypes: PosConnectionTypes) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IClaimedMagneticStripeReaderImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsDisabledOnDataReceived(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsDisabledOnDataReceived(&self) -> ::windows::core::Result<bool>;
    fn SetIsDecodeDataEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsDecodeDataEnabled(&self) -> ::windows::core::Result<bool>;
    fn IsDeviceAuthenticated(&self) -> ::windows::core::Result<bool>;
    fn SetDataEncryptionAlgorithm(&self, value: u32) -> ::windows::core::Result<()>;
    fn DataEncryptionAlgorithm(&self) -> ::windows::core::Result<u32>;
    fn SetTracksToRead(&self, value: MagneticStripeReaderTrackIds) -> ::windows::core::Result<()>;
    fn TracksToRead(&self) -> ::windows::core::Result<MagneticStripeReaderTrackIds>;
    fn SetIsTransmitSentinelsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsTransmitSentinelsEnabled(&self) -> ::windows::core::Result<bool>;
    fn EnableAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DisableAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn RetainDevice(&self) -> ::windows::core::Result<()>;
    fn SetErrorReportingType(&self, value: MagneticStripeReaderErrorReportingType) -> ::windows::core::Result<()>;
    fn RetrieveDeviceAuthenticationDataAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>>;
    fn AuthenticateDeviceAsync(&self, responsetoken: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DeAuthenticateDeviceAsync(&self, responsetoken: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn UpdateKeyAsync(&self, key: &::windows::core::HSTRING, keyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ResetStatisticsAsync(&self, statisticscategories: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn UpdateStatisticsAsync(&self, statistics: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn BankCardDataReceived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, MagneticStripeReaderBankCardDataReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBankCardDataReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AamvaCardDataReceived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, MagneticStripeReaderAamvaCardDataReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAamvaCardDataReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn VendorSpecificDataReceived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveVendorSpecificDataReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ReleaseDeviceRequested(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<ClaimedMagneticStripeReader>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReleaseDeviceRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ErrorOccurred(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, MagneticStripeReaderErrorOccurredEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveErrorOccurred(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IClaimedMagneticStripeReader2Impl: Sized {
    fn Closed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ClaimedMagneticStripeReader, ClaimedMagneticStripeReaderClosedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IClaimedMagneticStripeReaderClosedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IClaimedPosPrinterImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetCharacterSet(&self, value: u32) -> ::windows::core::Result<()>;
    fn CharacterSet(&self) -> ::windows::core::Result<u32>;
    fn IsCoverOpen(&self) -> ::windows::core::Result<bool>;
    fn SetIsCharacterSetMappingEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsCharacterSetMappingEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetMapMode(&self, value: PosPrinterMapMode) -> ::windows::core::Result<()>;
    fn MapMode(&self) -> ::windows::core::Result<PosPrinterMapMode>;
    fn Receipt(&self) -> ::windows::core::Result<ClaimedReceiptPrinter>;
    fn Slip(&self) -> ::windows::core::Result<ClaimedSlipPrinter>;
    fn Journal(&self) -> ::windows::core::Result<ClaimedJournalPrinter>;
    fn EnableAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn DisableAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RetainDeviceAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn ResetStatisticsAsync(&self, statisticscategories: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn UpdateStatisticsAsync(&self, statistics: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::HSTRING>>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn ReleaseDeviceRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ClaimedPosPrinter, PosPrinterReleaseDeviceRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveReleaseDeviceRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IClaimedPosPrinter2Impl: Sized {
    fn Closed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<ClaimedPosPrinter, ClaimedPosPrinterClosedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IClaimedPosPrinterClosedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IClaimedReceiptPrinterImpl: Sized {
    fn SidewaysMaxLines(&self) -> ::windows::core::Result<u32>;
    fn SidewaysMaxChars(&self) -> ::windows::core::Result<u32>;
    fn LinesToPaperCut(&self) -> ::windows::core::Result<u32>;
    fn PageSize(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn PrintArea(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn CreateJob(&self) -> ::windows::core::Result<ReceiptPrintJob>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IClaimedSlipPrinterImpl: Sized {
    fn SidewaysMaxLines(&self) -> ::windows::core::Result<u32>;
    fn SidewaysMaxChars(&self) -> ::windows::core::Result<u32>;
    fn MaxLines(&self) -> ::windows::core::Result<u32>;
    fn LinesNearEndToEnd(&self) -> ::windows::core::Result<u32>;
    fn PrintSide(&self) -> ::windows::core::Result<PosPrinterPrintSide>;
    fn PageSize(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn PrintArea(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn OpenJaws(&self) -> ::windows::core::Result<()>;
    fn CloseJaws(&self) -> ::windows::core::Result<()>;
    fn InsertSlipAsync(&self, timeout: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn RemoveSlipAsync(&self, timeout: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn ChangePrintSide(&self, printside: PosPrinterPrintSide) -> ::windows::core::Result<()>;
    fn CreateJob(&self) -> ::windows::core::Result<SlipPrintJob>;
}
pub trait ICommonClaimedPosPrinterStationImpl: Sized {
    fn SetCharactersPerLine(&self, value: u32) -> ::windows::core::Result<()>;
    fn CharactersPerLine(&self) -> ::windows::core::Result<u32>;
    fn SetLineHeight(&self, value: u32) -> ::windows::core::Result<()>;
    fn LineHeight(&self) -> ::windows::core::Result<u32>;
    fn SetLineSpacing(&self, value: u32) -> ::windows::core::Result<()>;
    fn LineSpacing(&self) -> ::windows::core::Result<u32>;
    fn LineWidth(&self) -> ::windows::core::Result<u32>;
    fn SetIsLetterQuality(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsLetterQuality(&self) -> ::windows::core::Result<bool>;
    fn IsPaperNearEnd(&self) -> ::windows::core::Result<bool>;
    fn SetColorCartridge(&self, value: PosPrinterColorCartridge) -> ::windows::core::Result<()>;
    fn ColorCartridge(&self) -> ::windows::core::Result<PosPrinterColorCartridge>;
    fn IsCoverOpen(&self) -> ::windows::core::Result<bool>;
    fn IsCartridgeRemoved(&self) -> ::windows::core::Result<bool>;
    fn IsCartridgeEmpty(&self) -> ::windows::core::Result<bool>;
    fn IsHeadCleaning(&self) -> ::windows::core::Result<bool>;
    fn IsPaperEmpty(&self) -> ::windows::core::Result<bool>;
    fn IsReadyToPrint(&self) -> ::windows::core::Result<bool>;
    fn ValidateData(&self, data: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
}
pub trait ICommonPosPrintStationCapabilitiesImpl: Sized {
    fn IsPrinterPresent(&self) -> ::windows::core::Result<bool>;
    fn IsDualColorSupported(&self) -> ::windows::core::Result<bool>;
    fn ColorCartridgeCapabilities(&self) -> ::windows::core::Result<PosPrinterColorCapabilities>;
    fn CartridgeSensors(&self) -> ::windows::core::Result<PosPrinterCartridgeSensors>;
    fn IsBoldSupported(&self) -> ::windows::core::Result<bool>;
    fn IsItalicSupported(&self) -> ::windows::core::Result<bool>;
    fn IsUnderlineSupported(&self) -> ::windows::core::Result<bool>;
    fn IsDoubleHighPrintSupported(&self) -> ::windows::core::Result<bool>;
    fn IsDoubleWidePrintSupported(&self) -> ::windows::core::Result<bool>;
    fn IsDoubleHighDoubleWidePrintSupported(&self) -> ::windows::core::Result<bool>;
    fn IsPaperEmptySensorSupported(&self) -> ::windows::core::Result<bool>;
    fn IsPaperNearEndSensorSupported(&self) -> ::windows::core::Result<bool>;
    fn SupportedCharactersPerLine(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
}
pub trait ICommonReceiptSlipCapabilitiesImpl: Sized + ICommonPosPrintStationCapabilitiesImpl {
    fn IsBarcodeSupported(&self) -> ::windows::core::Result<bool>;
    fn IsBitmapSupported(&self) -> ::windows::core::Result<bool>;
    fn IsLeft90RotationSupported(&self) -> ::windows::core::Result<bool>;
    fn IsRight90RotationSupported(&self) -> ::windows::core::Result<bool>;
    fn Is180RotationSupported(&self) -> ::windows::core::Result<bool>;
    fn IsPrintAreaSupported(&self) -> ::windows::core::Result<bool>;
    fn RuledLineCapabilities(&self) -> ::windows::core::Result<PosPrinterRuledLineCapabilities>;
    fn SupportedBarcodeRotations(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>>;
    fn SupportedBitmapRotations(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PosPrinterRotation>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IJournalPrintJobImpl: Sized {
    fn Print(&self, data: &::windows::core::HSTRING, printoptions: &::core::option::Option<PosPrinterPrintOptions>) -> ::windows::core::Result<()>;
    fn FeedPaperByLine(&self, linecount: i32) -> ::windows::core::Result<()>;
    fn FeedPaperByMapModeUnit(&self, distance: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IJournalPrinterCapabilitiesImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IJournalPrinterCapabilities2Impl: Sized {
    fn IsReverseVideoSupported(&self) -> ::windows::core::Result<bool>;
    fn IsStrikethroughSupported(&self) -> ::windows::core::Result<bool>;
    fn IsSuperscriptSupported(&self) -> ::windows::core::Result<bool>;
    fn IsSubscriptSupported(&self) -> ::windows::core::Result<bool>;
    fn IsReversePaperFeedByLineSupported(&self) -> ::windows::core::Result<bool>;
    fn IsReversePaperFeedByMapModeUnitSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILineDisplayImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Capabilities(&self) -> ::windows::core::Result<LineDisplayCapabilities>;
    fn PhysicalDeviceName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PhysicalDeviceDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DeviceControlDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DeviceControlVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DeviceServiceVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ClaimAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ClaimedLineDisplay>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILineDisplay2Impl: Sized {
    fn CheckPowerStatusAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LineDisplayPowerStatus>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILineDisplayAttributesImpl: Sized {
    fn IsPowerNotifyEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsPowerNotifyEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn Brightness(&self) -> ::windows::core::Result<i32>;
    fn SetBrightness(&self, value: i32) -> ::windows::core::Result<()>;
    fn BlinkRate(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetBlinkRate(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn ScreenSizeInCharacters(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn SetScreenSizeInCharacters(&self, value: &super::super::Foundation::Size) -> ::windows::core::Result<()>;
    fn CharacterSet(&self) -> ::windows::core::Result<i32>;
    fn SetCharacterSet(&self, value: i32) -> ::windows::core::Result<()>;
    fn IsCharacterSetMappingEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsCharacterSetMappingEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn CurrentWindow(&self) -> ::windows::core::Result<LineDisplayWindow>;
    fn SetCurrentWindow(&self, value: &::core::option::Option<LineDisplayWindow>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILineDisplayCapabilitiesImpl: Sized {
    fn IsStatisticsReportingSupported(&self) -> ::windows::core::Result<bool>;
    fn IsStatisticsUpdatingSupported(&self) -> ::windows::core::Result<bool>;
    fn PowerReportingType(&self) -> ::windows::core::Result<UnifiedPosPowerReportingType>;
    fn CanChangeScreenSize(&self) -> ::windows::core::Result<bool>;
    fn CanDisplayBitmaps(&self) -> ::windows::core::Result<bool>;
    fn CanReadCharacterAtCursor(&self) -> ::windows::core::Result<bool>;
    fn CanMapCharacterSets(&self) -> ::windows::core::Result<bool>;
    fn CanDisplayCustomGlyphs(&self) -> ::windows::core::Result<bool>;
    fn CanReverse(&self) -> ::windows::core::Result<LineDisplayTextAttributeGranularity>;
    fn CanBlink(&self) -> ::windows::core::Result<LineDisplayTextAttributeGranularity>;
    fn CanChangeBlinkRate(&self) -> ::windows::core::Result<bool>;
    fn IsBrightnessSupported(&self) -> ::windows::core::Result<bool>;
    fn IsCursorSupported(&self) -> ::windows::core::Result<bool>;
    fn IsHorizontalMarqueeSupported(&self) -> ::windows::core::Result<bool>;
    fn IsVerticalMarqueeSupported(&self) -> ::windows::core::Result<bool>;
    fn IsInterCharacterWaitSupported(&self) -> ::windows::core::Result<bool>;
    fn SupportedDescriptors(&self) -> ::windows::core::Result<u32>;
    fn SupportedWindows(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILineDisplayCursorImpl: Sized {
    fn CanCustomize(&self) -> ::windows::core::Result<bool>;
    fn IsBlinkSupported(&self) -> ::windows::core::Result<bool>;
    fn IsBlockSupported(&self) -> ::windows::core::Result<bool>;
    fn IsHalfBlockSupported(&self) -> ::windows::core::Result<bool>;
    fn IsUnderlineSupported(&self) -> ::windows::core::Result<bool>;
    fn IsReverseSupported(&self) -> ::windows::core::Result<bool>;
    fn IsOtherSupported(&self) -> ::windows::core::Result<bool>;
    fn GetAttributes(&self) -> ::windows::core::Result<LineDisplayCursorAttributes>;
    fn TryUpdateAttributesAsync(&self, attributes: &::core::option::Option<LineDisplayCursorAttributes>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILineDisplayCursorAttributesImpl: Sized {
    fn IsBlinkEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsBlinkEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn CursorType(&self) -> ::windows::core::Result<LineDisplayCursorType>;
    fn SetCursorType(&self, value: LineDisplayCursorType) -> ::windows::core::Result<()>;
    fn IsAutoAdvanceEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsAutoAdvanceEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn Position(&self) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn SetPosition(&self, value: &super::super::Foundation::Point) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILineDisplayCustomGlyphsImpl: Sized {
    fn SizeInPixels(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn SupportedGlyphCodes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
    fn TryRedefineAsync(&self, glyphcode: u32, glyphdata: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILineDisplayMarqueeImpl: Sized {
    fn Format(&self) -> ::windows::core::Result<LineDisplayMarqueeFormat>;
    fn SetFormat(&self, value: LineDisplayMarqueeFormat) -> ::windows::core::Result<()>;
    fn RepeatWaitInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetRepeatWaitInterval(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn ScrollWaitInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetScrollWaitInterval(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn TryStartScrollingAsync(&self, direction: LineDisplayScrollDirection) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryStopScrollingAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILineDisplayStaticsImpl: Sized {
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LineDisplay>>;
    fn GetDefaultAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<LineDisplay>>;
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeviceSelectorWithConnectionTypes(&self, connectiontypes: PosConnectionTypes) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILineDisplayStatics2Impl: Sized {
    fn StatisticsCategorySelector(&self) -> ::windows::core::Result<LineDisplayStatisticsCategorySelector>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILineDisplayStatisticsCategorySelectorImpl: Sized {
    fn AllStatistics(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn UnifiedPosStatistics(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ManufacturerStatistics(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILineDisplayStatusUpdatedEventArgsImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<LineDisplayPowerStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILineDisplayStoredBitmapImpl: Sized {
    fn EscapeSequence(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TryDeleteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILineDisplayWindowImpl: Sized {
    fn SizeInCharacters(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn InterCharacterWaitInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetInterCharacterWaitInterval(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn TryRefreshAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryDisplayTextAsync(&self, text: &::windows::core::HSTRING, displayattribute: LineDisplayTextAttribute) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryDisplayTextAtPositionAsync(&self, text: &::windows::core::HSTRING, displayattribute: LineDisplayTextAttribute, startposition: &super::super::Foundation::Point) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryDisplayTextNormalAsync(&self, text: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryScrollTextAsync(&self, direction: LineDisplayScrollDirection, numberofcolumnsorrows: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryClearTextAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILineDisplayWindow2Impl: Sized {
    fn Cursor(&self) -> ::windows::core::Result<LineDisplayCursor>;
    fn Marquee(&self) -> ::windows::core::Result<LineDisplayMarquee>;
    fn ReadCharacterAtCursorAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>>;
    fn TryDisplayStoredBitmapAtCursorAsync(&self, bitmap: &::core::option::Option<LineDisplayStoredBitmap>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryDisplayStorageFileBitmapAtCursorAsync(&self, bitmap: &::core::option::Option<super::super::Storage::StorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryDisplayStorageFileBitmapAtCursorWithAlignmentAsync(&self, bitmap: &::core::option::Option<super::super::Storage::StorageFile>, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryDisplayStorageFileBitmapAtCursorWithAlignmentAndWidthAsync(&self, bitmap: &::core::option::Option<super::super::Storage::StorageFile>, horizontalalignment: LineDisplayHorizontalAlignment, verticalalignment: LineDisplayVerticalAlignment, widthinpixels: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryDisplayStorageFileBitmapAtPointAsync(&self, bitmap: &::core::option::Option<super::super::Storage::StorageFile>, offsetinpixels: &super::super::Foundation::Point) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryDisplayStorageFileBitmapAtPointWithWidthAsync(&self, bitmap: &::core::option::Option<super::super::Storage::StorageFile>, offsetinpixels: &super::super::Foundation::Point, widthinpixels: i32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagneticStripeReaderImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Capabilities(&self) -> ::windows::core::Result<MagneticStripeReaderCapabilities>;
    fn SupportedCardTypes(&self) -> ::windows::core::Result<::windows::core::Array<u32>>;
    fn DeviceAuthenticationProtocol(&self) -> ::windows::core::Result<MagneticStripeReaderAuthenticationProtocol>;
    fn CheckHealthAsync(&self, level: UnifiedPosHealthCheckLevel) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn ClaimReaderAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ClaimedMagneticStripeReader>>;
    fn RetrieveStatisticsAsync(&self, statisticscategories: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IBuffer>>;
    fn GetErrorReportingType(&self) -> ::windows::core::Result<MagneticStripeReaderErrorReportingType>;
    fn StatusUpdated(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MagneticStripeReader, MagneticStripeReaderStatusUpdatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStatusUpdated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagneticStripeReaderAamvaCardDataReceivedEventArgsImpl: Sized {
    fn Report(&self) -> ::windows::core::Result<MagneticStripeReaderReport>;
    fn LicenseNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ExpirationDate(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Restrictions(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Class(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Endorsements(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BirthDate(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FirstName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Surname(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Suffix(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Gender(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HairColor(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EyeColor(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Height(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Weight(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Address(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn City(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn State(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PostalCode(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagneticStripeReaderBankCardDataReceivedEventArgsImpl: Sized {
    fn Report(&self) -> ::windows::core::Result<MagneticStripeReaderReport>;
    fn AccountNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ExpirationDate(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ServiceCode(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FirstName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MiddleInitial(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Surname(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Suffix(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagneticStripeReaderCapabilitiesImpl: Sized {
    fn CardAuthentication(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SupportedEncryptionAlgorithms(&self) -> ::windows::core::Result<u32>;
    fn AuthenticationLevel(&self) -> ::windows::core::Result<MagneticStripeReaderAuthenticationLevel>;
    fn IsIsoSupported(&self) -> ::windows::core::Result<bool>;
    fn IsJisOneSupported(&self) -> ::windows::core::Result<bool>;
    fn IsJisTwoSupported(&self) -> ::windows::core::Result<bool>;
    fn PowerReportingType(&self) -> ::windows::core::Result<UnifiedPosPowerReportingType>;
    fn IsStatisticsReportingSupported(&self) -> ::windows::core::Result<bool>;
    fn IsStatisticsUpdatingSupported(&self) -> ::windows::core::Result<bool>;
    fn IsTrackDataMaskingSupported(&self) -> ::windows::core::Result<bool>;
    fn IsTransmitSentinelsSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagneticStripeReaderCardTypesStaticsImpl: Sized {
    fn Unknown(&self) -> ::windows::core::Result<u32>;
    fn Bank(&self) -> ::windows::core::Result<u32>;
    fn Aamva(&self) -> ::windows::core::Result<u32>;
    fn ExtendedBase(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagneticStripeReaderEncryptionAlgorithmsStaticsImpl: Sized {
    fn None(&self) -> ::windows::core::Result<u32>;
    fn TripleDesDukpt(&self) -> ::windows::core::Result<u32>;
    fn ExtendedBase(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagneticStripeReaderErrorOccurredEventArgsImpl: Sized {
    fn Track1Status(&self) -> ::windows::core::Result<MagneticStripeReaderTrackErrorType>;
    fn Track2Status(&self) -> ::windows::core::Result<MagneticStripeReaderTrackErrorType>;
    fn Track3Status(&self) -> ::windows::core::Result<MagneticStripeReaderTrackErrorType>;
    fn Track4Status(&self) -> ::windows::core::Result<MagneticStripeReaderTrackErrorType>;
    fn ErrorData(&self) -> ::windows::core::Result<UnifiedPosErrorData>;
    fn PartialInputData(&self) -> ::windows::core::Result<MagneticStripeReaderReport>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagneticStripeReaderReportImpl: Sized {
    fn CardType(&self) -> ::windows::core::Result<u32>;
    fn Track1(&self) -> ::windows::core::Result<MagneticStripeReaderTrackData>;
    fn Track2(&self) -> ::windows::core::Result<MagneticStripeReaderTrackData>;
    fn Track3(&self) -> ::windows::core::Result<MagneticStripeReaderTrackData>;
    fn Track4(&self) -> ::windows::core::Result<MagneticStripeReaderTrackData>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn CardAuthenticationData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn CardAuthenticationDataLength(&self) -> ::windows::core::Result<u32>;
    fn AdditionalSecurityInformation(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagneticStripeReaderStaticsImpl: Sized {
    fn GetDefaultAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MagneticStripeReader>>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MagneticStripeReader>>;
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagneticStripeReaderStatics2Impl: Sized {
    fn GetDeviceSelectorWithConnectionTypes(&self, connectiontypes: PosConnectionTypes) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagneticStripeReaderStatusUpdatedEventArgsImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<MagneticStripeReaderStatus>;
    fn ExtendedStatus(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagneticStripeReaderTrackDataImpl: Sized {
    fn Data(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn DiscretionaryData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn EncryptedData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgsImpl: Sized {
    fn Report(&self) -> ::windows::core::Result<MagneticStripeReaderReport>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPosPrinterImpl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Capabilities(&self) -> ::windows::core::Result<PosPrinterCapabilities>;
    fn SupportedCharacterSets(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
    fn SupportedTypeFaces(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn Status(&self) -> ::windows::core::Result<PosPrinterStatus>;
    fn ClaimPrinterAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ClaimedPosPrinter>>;
    fn CheckHealthAsync(&self, level: UnifiedPosHealthCheckLevel) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetStatisticsAsync(&self, statisticscategories: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn StatusUpdated(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PosPrinter, PosPrinterStatusUpdatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStatusUpdated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPosPrinter2Impl: Sized {
    fn SupportedBarcodeSymbologies(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>>;
    fn GetFontProperty(&self, typeface: &::windows::core::HSTRING) -> ::windows::core::Result<PosPrinterFontProperty>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPosPrinterCapabilitiesImpl: Sized {
    fn PowerReportingType(&self) -> ::windows::core::Result<UnifiedPosPowerReportingType>;
    fn IsStatisticsReportingSupported(&self) -> ::windows::core::Result<bool>;
    fn IsStatisticsUpdatingSupported(&self) -> ::windows::core::Result<bool>;
    fn DefaultCharacterSet(&self) -> ::windows::core::Result<u32>;
    fn HasCoverSensor(&self) -> ::windows::core::Result<bool>;
    fn CanMapCharacterSet(&self) -> ::windows::core::Result<bool>;
    fn IsTransactionSupported(&self) -> ::windows::core::Result<bool>;
    fn Receipt(&self) -> ::windows::core::Result<ReceiptPrinterCapabilities>;
    fn Slip(&self) -> ::windows::core::Result<SlipPrinterCapabilities>;
    fn Journal(&self) -> ::windows::core::Result<JournalPrinterCapabilities>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPosPrinterCharacterSetIdsStaticsImpl: Sized {
    fn Utf16LE(&self) -> ::windows::core::Result<u32>;
    fn Ascii(&self) -> ::windows::core::Result<u32>;
    fn Ansi(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPosPrinterFontPropertyImpl: Sized {
    fn TypeFace(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsScalableToAnySize(&self) -> ::windows::core::Result<bool>;
    fn CharacterSizes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SizeUInt32>>;
}
pub trait IPosPrinterJobImpl: Sized {
    fn Print(&self, data: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PrintLine(&self, data: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PrintNewline(&self) -> ::windows::core::Result<()>;
    fn ExecuteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPosPrinterPrintOptionsImpl: Sized {
    fn TypeFace(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTypeFace(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn CharacterHeight(&self) -> ::windows::core::Result<u32>;
    fn SetCharacterHeight(&self, value: u32) -> ::windows::core::Result<()>;
    fn Bold(&self) -> ::windows::core::Result<bool>;
    fn SetBold(&self, value: bool) -> ::windows::core::Result<()>;
    fn Italic(&self) -> ::windows::core::Result<bool>;
    fn SetItalic(&self, value: bool) -> ::windows::core::Result<()>;
    fn Underline(&self) -> ::windows::core::Result<bool>;
    fn SetUnderline(&self, value: bool) -> ::windows::core::Result<()>;
    fn ReverseVideo(&self) -> ::windows::core::Result<bool>;
    fn SetReverseVideo(&self, value: bool) -> ::windows::core::Result<()>;
    fn Strikethrough(&self) -> ::windows::core::Result<bool>;
    fn SetStrikethrough(&self, value: bool) -> ::windows::core::Result<()>;
    fn Superscript(&self) -> ::windows::core::Result<bool>;
    fn SetSuperscript(&self, value: bool) -> ::windows::core::Result<()>;
    fn Subscript(&self) -> ::windows::core::Result<bool>;
    fn SetSubscript(&self, value: bool) -> ::windows::core::Result<()>;
    fn DoubleWide(&self) -> ::windows::core::Result<bool>;
    fn SetDoubleWide(&self, value: bool) -> ::windows::core::Result<()>;
    fn DoubleHigh(&self) -> ::windows::core::Result<bool>;
    fn SetDoubleHigh(&self, value: bool) -> ::windows::core::Result<()>;
    fn Alignment(&self) -> ::windows::core::Result<PosPrinterAlignment>;
    fn SetAlignment(&self, value: PosPrinterAlignment) -> ::windows::core::Result<()>;
    fn CharacterSet(&self) -> ::windows::core::Result<u32>;
    fn SetCharacterSet(&self, value: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPosPrinterReleaseDeviceRequestedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IPosPrinterStaticsImpl: Sized {
    fn GetDefaultAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PosPrinter>>;
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PosPrinter>>;
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPosPrinterStatics2Impl: Sized {
    fn GetDeviceSelectorWithConnectionTypes(&self, connectiontypes: PosConnectionTypes) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPosPrinterStatusImpl: Sized {
    fn StatusKind(&self) -> ::windows::core::Result<PosPrinterStatusKind>;
    fn ExtendedStatus(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPosPrinterStatusUpdatedEventArgsImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<PosPrinterStatus>;
}
pub trait IReceiptOrSlipJobImpl: Sized + IPosPrinterJobImpl {
    fn SetBarcodeRotation(&self, value: PosPrinterRotation) -> ::windows::core::Result<()>;
    fn SetPrintRotation(&self, value: PosPrinterRotation, includebitmaps: bool) -> ::windows::core::Result<()>;
    fn SetPrintArea(&self, value: &super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn SetBitmap(&self, bitmapnumber: u32, bitmap: &::core::option::Option<super::super::Graphics::Imaging::BitmapFrame>, alignment: PosPrinterAlignment) -> ::windows::core::Result<()>;
    fn SetBitmapCustomWidthStandardAlign(&self, bitmapnumber: u32, bitmap: &::core::option::Option<super::super::Graphics::Imaging::BitmapFrame>, alignment: PosPrinterAlignment, width: u32) -> ::windows::core::Result<()>;
    fn SetCustomAlignedBitmap(&self, bitmapnumber: u32, bitmap: &::core::option::Option<super::super::Graphics::Imaging::BitmapFrame>, alignmentdistance: u32) -> ::windows::core::Result<()>;
    fn SetBitmapCustomWidthCustomAlign(&self, bitmapnumber: u32, bitmap: &::core::option::Option<super::super::Graphics::Imaging::BitmapFrame>, alignmentdistance: u32, width: u32) -> ::windows::core::Result<()>;
    fn PrintSavedBitmap(&self, bitmapnumber: u32) -> ::windows::core::Result<()>;
    fn DrawRuledLine(&self, positionlist: &::windows::core::HSTRING, linedirection: PosPrinterLineDirection, linewidth: u32, linestyle: PosPrinterLineStyle, linecolor: u32) -> ::windows::core::Result<()>;
    fn PrintBarcode(&self, data: &::windows::core::HSTRING, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignment: PosPrinterAlignment) -> ::windows::core::Result<()>;
    fn PrintBarcodeCustomAlign(&self, data: &::windows::core::HSTRING, symbology: u32, height: u32, width: u32, textposition: PosPrinterBarcodeTextPosition, alignmentdistance: u32) -> ::windows::core::Result<()>;
    fn PrintBitmap(&self, bitmap: &::core::option::Option<super::super::Graphics::Imaging::BitmapFrame>, alignment: PosPrinterAlignment) -> ::windows::core::Result<()>;
    fn PrintBitmapCustomWidthStandardAlign(&self, bitmap: &::core::option::Option<super::super::Graphics::Imaging::BitmapFrame>, alignment: PosPrinterAlignment, width: u32) -> ::windows::core::Result<()>;
    fn PrintCustomAlignedBitmap(&self, bitmap: &::core::option::Option<super::super::Graphics::Imaging::BitmapFrame>, alignmentdistance: u32) -> ::windows::core::Result<()>;
    fn PrintBitmapCustomWidthCustomAlign(&self, bitmap: &::core::option::Option<super::super::Graphics::Imaging::BitmapFrame>, alignmentdistance: u32, width: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IReceiptPrintJobImpl: Sized {
    fn MarkFeed(&self, kind: PosPrinterMarkFeedKind) -> ::windows::core::Result<()>;
    fn CutPaper(&self, percentage: f64) -> ::windows::core::Result<()>;
    fn CutPaperDefault(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IReceiptPrintJob2Impl: Sized {
    fn StampPaper(&self) -> ::windows::core::Result<()>;
    fn Print(&self, data: &::windows::core::HSTRING, printoptions: &::core::option::Option<PosPrinterPrintOptions>) -> ::windows::core::Result<()>;
    fn FeedPaperByLine(&self, linecount: i32) -> ::windows::core::Result<()>;
    fn FeedPaperByMapModeUnit(&self, distance: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IReceiptPrinterCapabilitiesImpl: Sized {
    fn CanCutPaper(&self) -> ::windows::core::Result<bool>;
    fn IsStampSupported(&self) -> ::windows::core::Result<bool>;
    fn MarkFeedCapabilities(&self) -> ::windows::core::Result<PosPrinterMarkFeedCapabilities>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IReceiptPrinterCapabilities2Impl: Sized {
    fn IsReverseVideoSupported(&self) -> ::windows::core::Result<bool>;
    fn IsStrikethroughSupported(&self) -> ::windows::core::Result<bool>;
    fn IsSuperscriptSupported(&self) -> ::windows::core::Result<bool>;
    fn IsSubscriptSupported(&self) -> ::windows::core::Result<bool>;
    fn IsReversePaperFeedByLineSupported(&self) -> ::windows::core::Result<bool>;
    fn IsReversePaperFeedByMapModeUnitSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISlipPrintJobImpl: Sized {
    fn Print(&self, data: &::windows::core::HSTRING, printoptions: &::core::option::Option<PosPrinterPrintOptions>) -> ::windows::core::Result<()>;
    fn FeedPaperByLine(&self, linecount: i32) -> ::windows::core::Result<()>;
    fn FeedPaperByMapModeUnit(&self, distance: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISlipPrinterCapabilitiesImpl: Sized {
    fn IsFullLengthSupported(&self) -> ::windows::core::Result<bool>;
    fn IsBothSidesPrintingSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISlipPrinterCapabilities2Impl: Sized {
    fn IsReverseVideoSupported(&self) -> ::windows::core::Result<bool>;
    fn IsStrikethroughSupported(&self) -> ::windows::core::Result<bool>;
    fn IsSuperscriptSupported(&self) -> ::windows::core::Result<bool>;
    fn IsSubscriptSupported(&self) -> ::windows::core::Result<bool>;
    fn IsReversePaperFeedByLineSupported(&self) -> ::windows::core::Result<bool>;
    fn IsReversePaperFeedByMapModeUnitSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUnifiedPosErrorDataImpl: Sized {
    fn Message(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Severity(&self) -> ::windows::core::Result<UnifiedPosErrorSeverity>;
    fn Reason(&self) -> ::windows::core::Result<UnifiedPosErrorReason>;
    fn ExtendedReason(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUnifiedPosErrorDataFactoryImpl: Sized {
    fn CreateInstance(&self, message: &::windows::core::HSTRING, severity: UnifiedPosErrorSeverity, reason: UnifiedPosErrorReason, extendedreason: u32) -> ::windows::core::Result<UnifiedPosErrorData>;
}

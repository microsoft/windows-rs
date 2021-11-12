#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Devices_PointOfService_Provider")]
pub mod Provider;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct BarcodeScanner(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BarcodeScannerCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BarcodeScannerDataReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BarcodeScannerErrorOccurredEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BarcodeScannerImagePreviewReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BarcodeScannerReport(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct BarcodeScannerStatus(i32);
#[repr(transparent)]
pub struct BarcodeScannerStatusUpdatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BarcodeSymbologyAttributes(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct BarcodeSymbologyDecodeLengthKind(i32);
#[repr(transparent)]
pub struct CashDrawer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CashDrawerCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CashDrawerCloseAlarm(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CashDrawerClosedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CashDrawerEventSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CashDrawerOpenedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CashDrawerStatus(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct CashDrawerStatusKind(i32);
#[repr(transparent)]
pub struct CashDrawerStatusUpdatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ClaimedBarcodeScanner(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ClaimedBarcodeScannerClosedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ClaimedCashDrawer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ClaimedCashDrawerClosedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ClaimedJournalPrinter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ClaimedLineDisplay(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ClaimedLineDisplayClosedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ClaimedMagneticStripeReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ClaimedMagneticStripeReaderClosedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ClaimedPosPrinter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ClaimedPosPrinterClosedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ClaimedReceiptPrinter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ClaimedSlipPrinter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarcodeScanner(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarcodeScanner2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarcodeScannerCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarcodeScannerCapabilities1(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarcodeScannerCapabilities2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarcodeScannerDataReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarcodeScannerErrorOccurredEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarcodeScannerImagePreviewReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarcodeScannerReport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarcodeScannerReportFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarcodeScannerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarcodeScannerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarcodeScannerStatusUpdatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarcodeSymbologiesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarcodeSymbologiesStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBarcodeSymbologyAttributes(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICashDrawer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICashDrawerCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICashDrawerCloseAlarm(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICashDrawerEventSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICashDrawerEventSourceEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICashDrawerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICashDrawerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICashDrawerStatus(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICashDrawerStatusUpdatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IClaimedBarcodeScanner(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IClaimedBarcodeScanner1(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IClaimedBarcodeScanner2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IClaimedBarcodeScanner3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IClaimedBarcodeScanner4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IClaimedBarcodeScannerClosedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IClaimedCashDrawer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IClaimedCashDrawer2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IClaimedCashDrawerClosedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IClaimedJournalPrinter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IClaimedLineDisplay(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IClaimedLineDisplay2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IClaimedLineDisplay3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IClaimedLineDisplayClosedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IClaimedLineDisplayStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IClaimedMagneticStripeReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IClaimedMagneticStripeReader2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IClaimedMagneticStripeReaderClosedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IClaimedPosPrinter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IClaimedPosPrinter2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IClaimedPosPrinterClosedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IClaimedReceiptPrinter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IClaimedSlipPrinter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICommonClaimedPosPrinterStation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICommonPosPrintStationCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICommonReceiptSlipCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IJournalPrintJob(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IJournalPrinterCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IJournalPrinterCapabilities2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILineDisplay(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILineDisplay2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILineDisplayAttributes(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILineDisplayCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILineDisplayCursor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILineDisplayCursorAttributes(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILineDisplayCustomGlyphs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILineDisplayMarquee(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILineDisplayStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILineDisplayStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILineDisplayStatisticsCategorySelector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILineDisplayStatusUpdatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILineDisplayStoredBitmap(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILineDisplayWindow(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILineDisplayWindow2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMagneticStripeReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMagneticStripeReaderAamvaCardDataReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMagneticStripeReaderBankCardDataReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMagneticStripeReaderCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMagneticStripeReaderCardTypesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMagneticStripeReaderEncryptionAlgorithmsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMagneticStripeReaderErrorOccurredEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMagneticStripeReaderReport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMagneticStripeReaderStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMagneticStripeReaderStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMagneticStripeReaderStatusUpdatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMagneticStripeReaderTrackData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPosPrinter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPosPrinter2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPosPrinterCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPosPrinterCharacterSetIdsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPosPrinterFontProperty(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPosPrinterJob(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPosPrinterPrintOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPosPrinterReleaseDeviceRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPosPrinterStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPosPrinterStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPosPrinterStatus(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPosPrinterStatusUpdatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IReceiptOrSlipJob(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IReceiptPrintJob(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IReceiptPrintJob2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IReceiptPrinterCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IReceiptPrinterCapabilities2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISlipPrintJob(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISlipPrinterCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISlipPrinterCapabilities2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUnifiedPosErrorData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUnifiedPosErrorDataFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct JournalPrintJob(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct JournalPrinterCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LineDisplay(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LineDisplayAttributes(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LineDisplayCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LineDisplayCursor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LineDisplayCursorAttributes(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct LineDisplayCursorType(i32);
#[repr(transparent)]
pub struct LineDisplayCustomGlyphs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct LineDisplayDescriptorState(i32);
#[repr(C)]
pub struct LineDisplayHorizontalAlignment(i32);
#[repr(transparent)]
pub struct LineDisplayMarquee(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct LineDisplayMarqueeFormat(i32);
#[repr(C)]
pub struct LineDisplayPowerStatus(i32);
#[repr(C)]
pub struct LineDisplayScrollDirection(i32);
#[repr(transparent)]
pub struct LineDisplayStatisticsCategorySelector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LineDisplayStatusUpdatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LineDisplayStoredBitmap(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct LineDisplayTextAttribute(i32);
#[repr(C)]
pub struct LineDisplayTextAttributeGranularity(i32);
#[repr(C)]
pub struct LineDisplayVerticalAlignment(i32);
#[repr(transparent)]
pub struct LineDisplayWindow(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MagneticStripeReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MagneticStripeReaderAamvaCardDataReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct MagneticStripeReaderAuthenticationLevel(i32);
#[repr(C)]
pub struct MagneticStripeReaderAuthenticationProtocol(i32);
#[repr(transparent)]
pub struct MagneticStripeReaderBankCardDataReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MagneticStripeReaderCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MagneticStripeReaderErrorOccurredEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct MagneticStripeReaderErrorReportingType(i32);
#[repr(transparent)]
pub struct MagneticStripeReaderReport(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct MagneticStripeReaderStatus(i32);
#[repr(transparent)]
pub struct MagneticStripeReaderStatusUpdatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MagneticStripeReaderTrackData(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct MagneticStripeReaderTrackErrorType(i32);
#[repr(C)]
pub struct MagneticStripeReaderTrackIds(i32);
#[repr(transparent)]
pub struct MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PosConnectionTypes(i32);
#[repr(transparent)]
pub struct PosPrinter(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PosPrinterAlignment(i32);
#[repr(C)]
pub struct PosPrinterBarcodeTextPosition(i32);
#[repr(transparent)]
pub struct PosPrinterCapabilities(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PosPrinterCartridgeSensors(i32);
#[repr(C)]
pub struct PosPrinterColorCapabilities(i32);
#[repr(C)]
pub struct PosPrinterColorCartridge(i32);
#[repr(transparent)]
pub struct PosPrinterFontProperty(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PosPrinterLineDirection(i32);
#[repr(C)]
pub struct PosPrinterLineStyle(i32);
#[repr(C)]
pub struct PosPrinterMapMode(i32);
#[repr(C)]
pub struct PosPrinterMarkFeedCapabilities(i32);
#[repr(C)]
pub struct PosPrinterMarkFeedKind(i32);
#[repr(transparent)]
pub struct PosPrinterPrintOptions(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PosPrinterPrintSide(i32);
#[repr(transparent)]
pub struct PosPrinterReleaseDeviceRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PosPrinterRotation(i32);
#[repr(C)]
pub struct PosPrinterRuledLineCapabilities(i32);
#[repr(transparent)]
pub struct PosPrinterStatus(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PosPrinterStatusKind(i32);
#[repr(transparent)]
pub struct PosPrinterStatusUpdatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ReceiptPrintJob(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ReceiptPrinterCapabilities(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct SizeUInt32(i32);
#[repr(transparent)]
pub struct SlipPrintJob(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SlipPrinterCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UnifiedPosErrorData(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct UnifiedPosErrorReason(i32);
#[repr(C)]
pub struct UnifiedPosErrorSeverity(i32);
#[repr(C)]
pub struct UnifiedPosHealthCheckLevel(i32);
#[repr(C)]
pub struct UnifiedPosPowerReportingType(i32);

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[repr(transparent)]
pub struct BarcodeScannerStatus(pub i32);
impl BarcodeScannerStatus {
    pub const Online: BarcodeScannerStatus = BarcodeScannerStatus(0i32);
    pub const Off: BarcodeScannerStatus = BarcodeScannerStatus(1i32);
    pub const Offline: BarcodeScannerStatus = BarcodeScannerStatus(2i32);
    pub const OffOrOffline: BarcodeScannerStatus = BarcodeScannerStatus(3i32);
    pub const Extended: BarcodeScannerStatus = BarcodeScannerStatus(4i32);
}
#[repr(transparent)]
pub struct BarcodeScannerStatusUpdatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BarcodeSymbologyAttributes(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BarcodeSymbologyDecodeLengthKind(pub i32);
impl BarcodeSymbologyDecodeLengthKind {
    pub const AnyLength: BarcodeSymbologyDecodeLengthKind = BarcodeSymbologyDecodeLengthKind(0i32);
    pub const Discrete: BarcodeSymbologyDecodeLengthKind = BarcodeSymbologyDecodeLengthKind(1i32);
    pub const Range: BarcodeSymbologyDecodeLengthKind = BarcodeSymbologyDecodeLengthKind(2i32);
}
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
#[repr(transparent)]
pub struct CashDrawerStatusKind(pub i32);
impl CashDrawerStatusKind {
    pub const Online: CashDrawerStatusKind = CashDrawerStatusKind(0i32);
    pub const Off: CashDrawerStatusKind = CashDrawerStatusKind(1i32);
    pub const Offline: CashDrawerStatusKind = CashDrawerStatusKind(2i32);
    pub const OffOrOffline: CashDrawerStatusKind = CashDrawerStatusKind(3i32);
    pub const Extended: CashDrawerStatusKind = CashDrawerStatusKind(4i32);
}
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
#[repr(transparent)]
pub struct LineDisplayCustomGlyphs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LineDisplayDescriptorState(pub i32);
impl LineDisplayDescriptorState {
    pub const Off: LineDisplayDescriptorState = LineDisplayDescriptorState(0i32);
    pub const On: LineDisplayDescriptorState = LineDisplayDescriptorState(1i32);
    pub const Blink: LineDisplayDescriptorState = LineDisplayDescriptorState(2i32);
}
#[repr(transparent)]
pub struct LineDisplayHorizontalAlignment(pub i32);
impl LineDisplayHorizontalAlignment {
    pub const Left: LineDisplayHorizontalAlignment = LineDisplayHorizontalAlignment(0i32);
    pub const Center: LineDisplayHorizontalAlignment = LineDisplayHorizontalAlignment(1i32);
    pub const Right: LineDisplayHorizontalAlignment = LineDisplayHorizontalAlignment(2i32);
}
#[repr(transparent)]
pub struct LineDisplayMarquee(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LineDisplayMarqueeFormat(pub i32);
impl LineDisplayMarqueeFormat {
    pub const None: LineDisplayMarqueeFormat = LineDisplayMarqueeFormat(0i32);
    pub const Walk: LineDisplayMarqueeFormat = LineDisplayMarqueeFormat(1i32);
    pub const Place: LineDisplayMarqueeFormat = LineDisplayMarqueeFormat(2i32);
}
#[repr(transparent)]
pub struct LineDisplayPowerStatus(pub i32);
impl LineDisplayPowerStatus {
    pub const Unknown: LineDisplayPowerStatus = LineDisplayPowerStatus(0i32);
    pub const Online: LineDisplayPowerStatus = LineDisplayPowerStatus(1i32);
    pub const Off: LineDisplayPowerStatus = LineDisplayPowerStatus(2i32);
    pub const Offline: LineDisplayPowerStatus = LineDisplayPowerStatus(3i32);
    pub const OffOrOffline: LineDisplayPowerStatus = LineDisplayPowerStatus(4i32);
}
#[repr(transparent)]
pub struct LineDisplayScrollDirection(pub i32);
impl LineDisplayScrollDirection {
    pub const Up: LineDisplayScrollDirection = LineDisplayScrollDirection(0i32);
    pub const Down: LineDisplayScrollDirection = LineDisplayScrollDirection(1i32);
    pub const Left: LineDisplayScrollDirection = LineDisplayScrollDirection(2i32);
    pub const Right: LineDisplayScrollDirection = LineDisplayScrollDirection(3i32);
}
#[repr(transparent)]
pub struct LineDisplayStatisticsCategorySelector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LineDisplayStatusUpdatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LineDisplayStoredBitmap(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LineDisplayTextAttribute(pub i32);
impl LineDisplayTextAttribute {
    pub const Normal: LineDisplayTextAttribute = LineDisplayTextAttribute(0i32);
    pub const Blink: LineDisplayTextAttribute = LineDisplayTextAttribute(1i32);
    pub const Reverse: LineDisplayTextAttribute = LineDisplayTextAttribute(2i32);
    pub const ReverseBlink: LineDisplayTextAttribute = LineDisplayTextAttribute(3i32);
}
#[repr(transparent)]
pub struct LineDisplayTextAttributeGranularity(pub i32);
impl LineDisplayTextAttributeGranularity {
    pub const NotSupported: LineDisplayTextAttributeGranularity = LineDisplayTextAttributeGranularity(0i32);
    pub const EntireDisplay: LineDisplayTextAttributeGranularity = LineDisplayTextAttributeGranularity(1i32);
    pub const PerCharacter: LineDisplayTextAttributeGranularity = LineDisplayTextAttributeGranularity(2i32);
}
#[repr(transparent)]
pub struct LineDisplayVerticalAlignment(pub i32);
impl LineDisplayVerticalAlignment {
    pub const Top: LineDisplayVerticalAlignment = LineDisplayVerticalAlignment(0i32);
    pub const Center: LineDisplayVerticalAlignment = LineDisplayVerticalAlignment(1i32);
    pub const Bottom: LineDisplayVerticalAlignment = LineDisplayVerticalAlignment(2i32);
}
#[repr(transparent)]
pub struct LineDisplayWindow(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MagneticStripeReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MagneticStripeReaderAamvaCardDataReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MagneticStripeReaderAuthenticationLevel(pub i32);
impl MagneticStripeReaderAuthenticationLevel {
    pub const NotSupported: MagneticStripeReaderAuthenticationLevel = MagneticStripeReaderAuthenticationLevel(0i32);
    pub const Optional: MagneticStripeReaderAuthenticationLevel = MagneticStripeReaderAuthenticationLevel(1i32);
    pub const Required: MagneticStripeReaderAuthenticationLevel = MagneticStripeReaderAuthenticationLevel(2i32);
}
#[repr(transparent)]
pub struct MagneticStripeReaderAuthenticationProtocol(pub i32);
impl MagneticStripeReaderAuthenticationProtocol {
    pub const None: MagneticStripeReaderAuthenticationProtocol = MagneticStripeReaderAuthenticationProtocol(0i32);
    pub const ChallengeResponse: MagneticStripeReaderAuthenticationProtocol = MagneticStripeReaderAuthenticationProtocol(1i32);
}
#[repr(transparent)]
pub struct MagneticStripeReaderBankCardDataReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MagneticStripeReaderCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MagneticStripeReaderErrorOccurredEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MagneticStripeReaderErrorReportingType(pub i32);
impl MagneticStripeReaderErrorReportingType {
    pub const CardLevel: MagneticStripeReaderErrorReportingType = MagneticStripeReaderErrorReportingType(0i32);
    pub const TrackLevel: MagneticStripeReaderErrorReportingType = MagneticStripeReaderErrorReportingType(1i32);
}
#[repr(transparent)]
pub struct MagneticStripeReaderReport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MagneticStripeReaderStatus(pub i32);
impl MagneticStripeReaderStatus {
    pub const Unauthenticated: MagneticStripeReaderStatus = MagneticStripeReaderStatus(0i32);
    pub const Authenticated: MagneticStripeReaderStatus = MagneticStripeReaderStatus(1i32);
    pub const Extended: MagneticStripeReaderStatus = MagneticStripeReaderStatus(2i32);
}
#[repr(transparent)]
pub struct MagneticStripeReaderStatusUpdatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MagneticStripeReaderTrackData(pub *mut ::core::ffi::c_void);
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
#[repr(transparent)]
pub struct MagneticStripeReaderTrackIds(pub i32);
impl MagneticStripeReaderTrackIds {
    pub const None: MagneticStripeReaderTrackIds = MagneticStripeReaderTrackIds(0i32);
    pub const Track1: MagneticStripeReaderTrackIds = MagneticStripeReaderTrackIds(1i32);
    pub const Track2: MagneticStripeReaderTrackIds = MagneticStripeReaderTrackIds(2i32);
    pub const Track3: MagneticStripeReaderTrackIds = MagneticStripeReaderTrackIds(4i32);
    pub const Track4: MagneticStripeReaderTrackIds = MagneticStripeReaderTrackIds(8i32);
}
#[repr(transparent)]
pub struct MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PosConnectionTypes(pub u32);
impl PosConnectionTypes {
    pub const Local: PosConnectionTypes = PosConnectionTypes(1u32);
    pub const IP: PosConnectionTypes = PosConnectionTypes(2u32);
    pub const Bluetooth: PosConnectionTypes = PosConnectionTypes(4u32);
    pub const All: PosConnectionTypes = PosConnectionTypes(4294967295u32);
}
#[repr(transparent)]
pub struct PosPrinter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PosPrinterAlignment(pub i32);
impl PosPrinterAlignment {
    pub const Left: PosPrinterAlignment = PosPrinterAlignment(0i32);
    pub const Center: PosPrinterAlignment = PosPrinterAlignment(1i32);
    pub const Right: PosPrinterAlignment = PosPrinterAlignment(2i32);
}
#[repr(transparent)]
pub struct PosPrinterBarcodeTextPosition(pub i32);
impl PosPrinterBarcodeTextPosition {
    pub const None: PosPrinterBarcodeTextPosition = PosPrinterBarcodeTextPosition(0i32);
    pub const Above: PosPrinterBarcodeTextPosition = PosPrinterBarcodeTextPosition(1i32);
    pub const Below: PosPrinterBarcodeTextPosition = PosPrinterBarcodeTextPosition(2i32);
}
#[repr(transparent)]
pub struct PosPrinterCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PosPrinterCartridgeSensors(pub u32);
impl PosPrinterCartridgeSensors {
    pub const None: PosPrinterCartridgeSensors = PosPrinterCartridgeSensors(0u32);
    pub const Removed: PosPrinterCartridgeSensors = PosPrinterCartridgeSensors(1u32);
    pub const Empty: PosPrinterCartridgeSensors = PosPrinterCartridgeSensors(2u32);
    pub const HeadCleaning: PosPrinterCartridgeSensors = PosPrinterCartridgeSensors(4u32);
    pub const NearEnd: PosPrinterCartridgeSensors = PosPrinterCartridgeSensors(8u32);
}
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
#[repr(transparent)]
pub struct PosPrinterFontProperty(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PosPrinterLineDirection(pub i32);
impl PosPrinterLineDirection {
    pub const Horizontal: PosPrinterLineDirection = PosPrinterLineDirection(0i32);
    pub const Vertical: PosPrinterLineDirection = PosPrinterLineDirection(1i32);
}
#[repr(transparent)]
pub struct PosPrinterLineStyle(pub i32);
impl PosPrinterLineStyle {
    pub const SingleSolid: PosPrinterLineStyle = PosPrinterLineStyle(0i32);
    pub const DoubleSolid: PosPrinterLineStyle = PosPrinterLineStyle(1i32);
    pub const Broken: PosPrinterLineStyle = PosPrinterLineStyle(2i32);
    pub const Chain: PosPrinterLineStyle = PosPrinterLineStyle(3i32);
}
#[repr(transparent)]
pub struct PosPrinterMapMode(pub i32);
impl PosPrinterMapMode {
    pub const Dots: PosPrinterMapMode = PosPrinterMapMode(0i32);
    pub const Twips: PosPrinterMapMode = PosPrinterMapMode(1i32);
    pub const English: PosPrinterMapMode = PosPrinterMapMode(2i32);
    pub const Metric: PosPrinterMapMode = PosPrinterMapMode(3i32);
}
#[repr(transparent)]
pub struct PosPrinterMarkFeedCapabilities(pub u32);
impl PosPrinterMarkFeedCapabilities {
    pub const None: PosPrinterMarkFeedCapabilities = PosPrinterMarkFeedCapabilities(0u32);
    pub const ToTakeUp: PosPrinterMarkFeedCapabilities = PosPrinterMarkFeedCapabilities(1u32);
    pub const ToCutter: PosPrinterMarkFeedCapabilities = PosPrinterMarkFeedCapabilities(2u32);
    pub const ToCurrentTopOfForm: PosPrinterMarkFeedCapabilities = PosPrinterMarkFeedCapabilities(4u32);
    pub const ToNextTopOfForm: PosPrinterMarkFeedCapabilities = PosPrinterMarkFeedCapabilities(8u32);
}
#[repr(transparent)]
pub struct PosPrinterMarkFeedKind(pub i32);
impl PosPrinterMarkFeedKind {
    pub const ToTakeUp: PosPrinterMarkFeedKind = PosPrinterMarkFeedKind(0i32);
    pub const ToCutter: PosPrinterMarkFeedKind = PosPrinterMarkFeedKind(1i32);
    pub const ToCurrentTopOfForm: PosPrinterMarkFeedKind = PosPrinterMarkFeedKind(2i32);
    pub const ToNextTopOfForm: PosPrinterMarkFeedKind = PosPrinterMarkFeedKind(3i32);
}
#[repr(transparent)]
pub struct PosPrinterPrintOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PosPrinterPrintSide(pub i32);
impl PosPrinterPrintSide {
    pub const Unknown: PosPrinterPrintSide = PosPrinterPrintSide(0i32);
    pub const Side1: PosPrinterPrintSide = PosPrinterPrintSide(1i32);
    pub const Side2: PosPrinterPrintSide = PosPrinterPrintSide(2i32);
}
#[repr(transparent)]
pub struct PosPrinterReleaseDeviceRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PosPrinterRotation(pub i32);
impl PosPrinterRotation {
    pub const Normal: PosPrinterRotation = PosPrinterRotation(0i32);
    pub const Right90: PosPrinterRotation = PosPrinterRotation(1i32);
    pub const Left90: PosPrinterRotation = PosPrinterRotation(2i32);
    pub const Rotate180: PosPrinterRotation = PosPrinterRotation(3i32);
}
#[repr(transparent)]
pub struct PosPrinterRuledLineCapabilities(pub u32);
impl PosPrinterRuledLineCapabilities {
    pub const None: PosPrinterRuledLineCapabilities = PosPrinterRuledLineCapabilities(0u32);
    pub const Horizontal: PosPrinterRuledLineCapabilities = PosPrinterRuledLineCapabilities(1u32);
    pub const Vertical: PosPrinterRuledLineCapabilities = PosPrinterRuledLineCapabilities(2u32);
}
#[repr(transparent)]
pub struct PosPrinterStatus(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PosPrinterStatusKind(pub i32);
impl PosPrinterStatusKind {
    pub const Online: PosPrinterStatusKind = PosPrinterStatusKind(0i32);
    pub const Off: PosPrinterStatusKind = PosPrinterStatusKind(1i32);
    pub const Offline: PosPrinterStatusKind = PosPrinterStatusKind(2i32);
    pub const OffOrOffline: PosPrinterStatusKind = PosPrinterStatusKind(3i32);
    pub const Extended: PosPrinterStatusKind = PosPrinterStatusKind(4i32);
}
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
#[repr(transparent)]
pub struct UnifiedPosHealthCheckLevel(pub i32);
impl UnifiedPosHealthCheckLevel {
    pub const UnknownHealthCheckLevel: UnifiedPosHealthCheckLevel = UnifiedPosHealthCheckLevel(0i32);
    pub const POSInternal: UnifiedPosHealthCheckLevel = UnifiedPosHealthCheckLevel(1i32);
    pub const External: UnifiedPosHealthCheckLevel = UnifiedPosHealthCheckLevel(2i32);
    pub const Interactive: UnifiedPosHealthCheckLevel = UnifiedPosHealthCheckLevel(3i32);
}
#[repr(transparent)]
pub struct UnifiedPosPowerReportingType(pub i32);
impl UnifiedPosPowerReportingType {
    pub const UnknownPowerReportingType: UnifiedPosPowerReportingType = UnifiedPosPowerReportingType(0i32);
    pub const Standard: UnifiedPosPowerReportingType = UnifiedPosPowerReportingType(1i32);
    pub const Advanced: UnifiedPosPowerReportingType = UnifiedPosPowerReportingType(2i32);
}

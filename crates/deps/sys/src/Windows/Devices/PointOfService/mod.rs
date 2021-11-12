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
#[repr(transparent)]
pub struct BarcodeScannerStatusUpdatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BarcodeSymbologyAttributes(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
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
#[repr(transparent)]
pub struct LineDisplayCustomGlyphs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
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
#[repr(transparent)]
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
#[repr(transparent)]
pub struct LineDisplayMarquee(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
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
#[repr(transparent)]
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
#[repr(transparent)]
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
#[repr(transparent)]
pub struct LineDisplayStatisticsCategorySelector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LineDisplayStatusUpdatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LineDisplayStoredBitmap(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
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
#[repr(transparent)]
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
#[repr(transparent)]
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
#[repr(transparent)]
pub struct LineDisplayWindow(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MagneticStripeReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MagneticStripeReaderAamvaCardDataReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
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
#[repr(transparent)]
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
#[repr(transparent)]
pub struct MagneticStripeReaderBankCardDataReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MagneticStripeReaderCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MagneticStripeReaderErrorOccurredEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
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
#[repr(transparent)]
pub struct MagneticStripeReaderReport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
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
#[repr(transparent)]
pub struct MagneticStripeReaderStatusUpdatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MagneticStripeReaderTrackData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
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
#[repr(transparent)]
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
#[repr(transparent)]
pub struct MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
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
#[repr(transparent)]
pub struct PosPrinter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
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
#[repr(transparent)]
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
#[repr(transparent)]
pub struct PosPrinterCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
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
#[repr(transparent)]
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
#[repr(transparent)]
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
#[repr(transparent)]
pub struct PosPrinterFontProperty(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
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
#[repr(transparent)]
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
#[repr(transparent)]
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
#[repr(transparent)]
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
#[repr(transparent)]
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
#[repr(transparent)]
pub struct PosPrinterPrintOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
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
#[repr(transparent)]
pub struct PosPrinterReleaseDeviceRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
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
#[repr(transparent)]
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
#[repr(transparent)]
pub struct PosPrinterStatus(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
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
#[repr(transparent)]
pub struct PosPrinterStatusUpdatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ReceiptPrintJob(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ReceiptPrinterCapabilities(pub *mut ::core::ffi::c_void);
#[repr(C)]
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
#[repr(transparent)]
pub struct SlipPrintJob(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SlipPrinterCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UnifiedPosErrorData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
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
#[repr(transparent)]
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
#[repr(transparent)]
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
#[repr(transparent)]
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

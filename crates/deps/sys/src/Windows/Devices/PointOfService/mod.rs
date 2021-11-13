#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Devices_PointOfService_Provider")]
pub mod Provider;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct BarcodeScanner(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BarcodeScanner {}
impl ::core::clone::Clone for BarcodeScanner {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BarcodeScannerCapabilities(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BarcodeScannerCapabilities {}
impl ::core::clone::Clone for BarcodeScannerCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BarcodeScannerDataReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BarcodeScannerDataReceivedEventArgs {}
impl ::core::clone::Clone for BarcodeScannerDataReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BarcodeScannerErrorOccurredEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BarcodeScannerErrorOccurredEventArgs {}
impl ::core::clone::Clone for BarcodeScannerErrorOccurredEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BarcodeScannerImagePreviewReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BarcodeScannerImagePreviewReceivedEventArgs {}
impl ::core::clone::Clone for BarcodeScannerImagePreviewReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BarcodeScannerReport(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BarcodeScannerReport {}
impl ::core::clone::Clone for BarcodeScannerReport {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for BarcodeScannerStatusUpdatedEventArgs {}
impl ::core::clone::Clone for BarcodeScannerStatusUpdatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BarcodeSymbologyAttributes(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BarcodeSymbologyAttributes {}
impl ::core::clone::Clone for BarcodeSymbologyAttributes {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for CashDrawer {}
impl ::core::clone::Clone for CashDrawer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CashDrawerCapabilities(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CashDrawerCapabilities {}
impl ::core::clone::Clone for CashDrawerCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CashDrawerCloseAlarm(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CashDrawerCloseAlarm {}
impl ::core::clone::Clone for CashDrawerCloseAlarm {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CashDrawerClosedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CashDrawerClosedEventArgs {}
impl ::core::clone::Clone for CashDrawerClosedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CashDrawerEventSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CashDrawerEventSource {}
impl ::core::clone::Clone for CashDrawerEventSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CashDrawerOpenedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CashDrawerOpenedEventArgs {}
impl ::core::clone::Clone for CashDrawerOpenedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CashDrawerStatus(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CashDrawerStatus {}
impl ::core::clone::Clone for CashDrawerStatus {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for CashDrawerStatusUpdatedEventArgs {}
impl ::core::clone::Clone for CashDrawerStatusUpdatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ClaimedBarcodeScanner(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ClaimedBarcodeScanner {}
impl ::core::clone::Clone for ClaimedBarcodeScanner {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ClaimedBarcodeScannerClosedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ClaimedBarcodeScannerClosedEventArgs {}
impl ::core::clone::Clone for ClaimedBarcodeScannerClosedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ClaimedCashDrawer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ClaimedCashDrawer {}
impl ::core::clone::Clone for ClaimedCashDrawer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ClaimedCashDrawerClosedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ClaimedCashDrawerClosedEventArgs {}
impl ::core::clone::Clone for ClaimedCashDrawerClosedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ClaimedJournalPrinter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ClaimedJournalPrinter {}
impl ::core::clone::Clone for ClaimedJournalPrinter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ClaimedLineDisplay(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ClaimedLineDisplay {}
impl ::core::clone::Clone for ClaimedLineDisplay {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ClaimedLineDisplayClosedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ClaimedLineDisplayClosedEventArgs {}
impl ::core::clone::Clone for ClaimedLineDisplayClosedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ClaimedMagneticStripeReader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ClaimedMagneticStripeReader {}
impl ::core::clone::Clone for ClaimedMagneticStripeReader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ClaimedMagneticStripeReaderClosedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ClaimedMagneticStripeReaderClosedEventArgs {}
impl ::core::clone::Clone for ClaimedMagneticStripeReaderClosedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ClaimedPosPrinter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ClaimedPosPrinter {}
impl ::core::clone::Clone for ClaimedPosPrinter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ClaimedPosPrinterClosedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ClaimedPosPrinterClosedEventArgs {}
impl ::core::clone::Clone for ClaimedPosPrinterClosedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ClaimedReceiptPrinter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ClaimedReceiptPrinter {}
impl ::core::clone::Clone for ClaimedReceiptPrinter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ClaimedSlipPrinter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ClaimedSlipPrinter {}
impl ::core::clone::Clone for ClaimedSlipPrinter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarcodeScanner(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarcodeScanner {}
impl ::core::clone::Clone for IBarcodeScanner {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarcodeScanner2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarcodeScanner2 {}
impl ::core::clone::Clone for IBarcodeScanner2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarcodeScannerCapabilities(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarcodeScannerCapabilities {}
impl ::core::clone::Clone for IBarcodeScannerCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarcodeScannerCapabilities1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarcodeScannerCapabilities1 {}
impl ::core::clone::Clone for IBarcodeScannerCapabilities1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarcodeScannerCapabilities2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarcodeScannerCapabilities2 {}
impl ::core::clone::Clone for IBarcodeScannerCapabilities2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarcodeScannerDataReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarcodeScannerDataReceivedEventArgs {}
impl ::core::clone::Clone for IBarcodeScannerDataReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarcodeScannerErrorOccurredEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarcodeScannerErrorOccurredEventArgs {}
impl ::core::clone::Clone for IBarcodeScannerErrorOccurredEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarcodeScannerImagePreviewReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarcodeScannerImagePreviewReceivedEventArgs {}
impl ::core::clone::Clone for IBarcodeScannerImagePreviewReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarcodeScannerReport(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarcodeScannerReport {}
impl ::core::clone::Clone for IBarcodeScannerReport {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarcodeScannerReportFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarcodeScannerReportFactory {}
impl ::core::clone::Clone for IBarcodeScannerReportFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarcodeScannerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarcodeScannerStatics {}
impl ::core::clone::Clone for IBarcodeScannerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarcodeScannerStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarcodeScannerStatics2 {}
impl ::core::clone::Clone for IBarcodeScannerStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarcodeScannerStatusUpdatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarcodeScannerStatusUpdatedEventArgs {}
impl ::core::clone::Clone for IBarcodeScannerStatusUpdatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarcodeSymbologiesStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarcodeSymbologiesStatics {}
impl ::core::clone::Clone for IBarcodeSymbologiesStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarcodeSymbologiesStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarcodeSymbologiesStatics2 {}
impl ::core::clone::Clone for IBarcodeSymbologiesStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBarcodeSymbologyAttributes(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBarcodeSymbologyAttributes {}
impl ::core::clone::Clone for IBarcodeSymbologyAttributes {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICashDrawer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICashDrawer {}
impl ::core::clone::Clone for ICashDrawer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICashDrawerCapabilities(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICashDrawerCapabilities {}
impl ::core::clone::Clone for ICashDrawerCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICashDrawerCloseAlarm(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICashDrawerCloseAlarm {}
impl ::core::clone::Clone for ICashDrawerCloseAlarm {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICashDrawerEventSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICashDrawerEventSource {}
impl ::core::clone::Clone for ICashDrawerEventSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICashDrawerEventSourceEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICashDrawerEventSourceEventArgs {}
impl ::core::clone::Clone for ICashDrawerEventSourceEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICashDrawerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICashDrawerStatics {}
impl ::core::clone::Clone for ICashDrawerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICashDrawerStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICashDrawerStatics2 {}
impl ::core::clone::Clone for ICashDrawerStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICashDrawerStatus(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICashDrawerStatus {}
impl ::core::clone::Clone for ICashDrawerStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICashDrawerStatusUpdatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICashDrawerStatusUpdatedEventArgs {}
impl ::core::clone::Clone for ICashDrawerStatusUpdatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IClaimedBarcodeScanner(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IClaimedBarcodeScanner {}
impl ::core::clone::Clone for IClaimedBarcodeScanner {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IClaimedBarcodeScanner1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IClaimedBarcodeScanner1 {}
impl ::core::clone::Clone for IClaimedBarcodeScanner1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IClaimedBarcodeScanner2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IClaimedBarcodeScanner2 {}
impl ::core::clone::Clone for IClaimedBarcodeScanner2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IClaimedBarcodeScanner3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IClaimedBarcodeScanner3 {}
impl ::core::clone::Clone for IClaimedBarcodeScanner3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IClaimedBarcodeScanner4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IClaimedBarcodeScanner4 {}
impl ::core::clone::Clone for IClaimedBarcodeScanner4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IClaimedBarcodeScannerClosedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IClaimedBarcodeScannerClosedEventArgs {}
impl ::core::clone::Clone for IClaimedBarcodeScannerClosedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IClaimedCashDrawer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IClaimedCashDrawer {}
impl ::core::clone::Clone for IClaimedCashDrawer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IClaimedCashDrawer2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IClaimedCashDrawer2 {}
impl ::core::clone::Clone for IClaimedCashDrawer2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IClaimedCashDrawerClosedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IClaimedCashDrawerClosedEventArgs {}
impl ::core::clone::Clone for IClaimedCashDrawerClosedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IClaimedJournalPrinter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IClaimedJournalPrinter {}
impl ::core::clone::Clone for IClaimedJournalPrinter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IClaimedLineDisplay(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IClaimedLineDisplay {}
impl ::core::clone::Clone for IClaimedLineDisplay {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IClaimedLineDisplay2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IClaimedLineDisplay2 {}
impl ::core::clone::Clone for IClaimedLineDisplay2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IClaimedLineDisplay3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IClaimedLineDisplay3 {}
impl ::core::clone::Clone for IClaimedLineDisplay3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IClaimedLineDisplayClosedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IClaimedLineDisplayClosedEventArgs {}
impl ::core::clone::Clone for IClaimedLineDisplayClosedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IClaimedLineDisplayStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IClaimedLineDisplayStatics {}
impl ::core::clone::Clone for IClaimedLineDisplayStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IClaimedMagneticStripeReader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IClaimedMagneticStripeReader {}
impl ::core::clone::Clone for IClaimedMagneticStripeReader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IClaimedMagneticStripeReader2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IClaimedMagneticStripeReader2 {}
impl ::core::clone::Clone for IClaimedMagneticStripeReader2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IClaimedMagneticStripeReaderClosedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IClaimedMagneticStripeReaderClosedEventArgs {}
impl ::core::clone::Clone for IClaimedMagneticStripeReaderClosedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IClaimedPosPrinter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IClaimedPosPrinter {}
impl ::core::clone::Clone for IClaimedPosPrinter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IClaimedPosPrinter2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IClaimedPosPrinter2 {}
impl ::core::clone::Clone for IClaimedPosPrinter2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IClaimedPosPrinterClosedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IClaimedPosPrinterClosedEventArgs {}
impl ::core::clone::Clone for IClaimedPosPrinterClosedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IClaimedReceiptPrinter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IClaimedReceiptPrinter {}
impl ::core::clone::Clone for IClaimedReceiptPrinter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IClaimedSlipPrinter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IClaimedSlipPrinter {}
impl ::core::clone::Clone for IClaimedSlipPrinter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICommonClaimedPosPrinterStation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICommonClaimedPosPrinterStation {}
impl ::core::clone::Clone for ICommonClaimedPosPrinterStation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICommonPosPrintStationCapabilities(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICommonPosPrintStationCapabilities {}
impl ::core::clone::Clone for ICommonPosPrintStationCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICommonReceiptSlipCapabilities(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICommonReceiptSlipCapabilities {}
impl ::core::clone::Clone for ICommonReceiptSlipCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IJournalPrintJob(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IJournalPrintJob {}
impl ::core::clone::Clone for IJournalPrintJob {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IJournalPrinterCapabilities(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IJournalPrinterCapabilities {}
impl ::core::clone::Clone for IJournalPrinterCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IJournalPrinterCapabilities2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IJournalPrinterCapabilities2 {}
impl ::core::clone::Clone for IJournalPrinterCapabilities2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILineDisplay(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILineDisplay {}
impl ::core::clone::Clone for ILineDisplay {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILineDisplay2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILineDisplay2 {}
impl ::core::clone::Clone for ILineDisplay2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILineDisplayAttributes(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILineDisplayAttributes {}
impl ::core::clone::Clone for ILineDisplayAttributes {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILineDisplayCapabilities(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILineDisplayCapabilities {}
impl ::core::clone::Clone for ILineDisplayCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILineDisplayCursor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILineDisplayCursor {}
impl ::core::clone::Clone for ILineDisplayCursor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILineDisplayCursorAttributes(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILineDisplayCursorAttributes {}
impl ::core::clone::Clone for ILineDisplayCursorAttributes {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILineDisplayCustomGlyphs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILineDisplayCustomGlyphs {}
impl ::core::clone::Clone for ILineDisplayCustomGlyphs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILineDisplayMarquee(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILineDisplayMarquee {}
impl ::core::clone::Clone for ILineDisplayMarquee {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILineDisplayStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILineDisplayStatics {}
impl ::core::clone::Clone for ILineDisplayStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILineDisplayStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILineDisplayStatics2 {}
impl ::core::clone::Clone for ILineDisplayStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILineDisplayStatisticsCategorySelector(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILineDisplayStatisticsCategorySelector {}
impl ::core::clone::Clone for ILineDisplayStatisticsCategorySelector {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILineDisplayStatusUpdatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILineDisplayStatusUpdatedEventArgs {}
impl ::core::clone::Clone for ILineDisplayStatusUpdatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILineDisplayStoredBitmap(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILineDisplayStoredBitmap {}
impl ::core::clone::Clone for ILineDisplayStoredBitmap {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILineDisplayWindow(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILineDisplayWindow {}
impl ::core::clone::Clone for ILineDisplayWindow {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILineDisplayWindow2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILineDisplayWindow2 {}
impl ::core::clone::Clone for ILineDisplayWindow2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMagneticStripeReader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMagneticStripeReader {}
impl ::core::clone::Clone for IMagneticStripeReader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMagneticStripeReaderAamvaCardDataReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMagneticStripeReaderAamvaCardDataReceivedEventArgs {}
impl ::core::clone::Clone for IMagneticStripeReaderAamvaCardDataReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMagneticStripeReaderBankCardDataReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMagneticStripeReaderBankCardDataReceivedEventArgs {}
impl ::core::clone::Clone for IMagneticStripeReaderBankCardDataReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMagneticStripeReaderCapabilities(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMagneticStripeReaderCapabilities {}
impl ::core::clone::Clone for IMagneticStripeReaderCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMagneticStripeReaderCardTypesStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMagneticStripeReaderCardTypesStatics {}
impl ::core::clone::Clone for IMagneticStripeReaderCardTypesStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMagneticStripeReaderEncryptionAlgorithmsStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMagneticStripeReaderEncryptionAlgorithmsStatics {}
impl ::core::clone::Clone for IMagneticStripeReaderEncryptionAlgorithmsStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMagneticStripeReaderErrorOccurredEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMagneticStripeReaderErrorOccurredEventArgs {}
impl ::core::clone::Clone for IMagneticStripeReaderErrorOccurredEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMagneticStripeReaderReport(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMagneticStripeReaderReport {}
impl ::core::clone::Clone for IMagneticStripeReaderReport {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMagneticStripeReaderStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMagneticStripeReaderStatics {}
impl ::core::clone::Clone for IMagneticStripeReaderStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMagneticStripeReaderStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMagneticStripeReaderStatics2 {}
impl ::core::clone::Clone for IMagneticStripeReaderStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMagneticStripeReaderStatusUpdatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMagneticStripeReaderStatusUpdatedEventArgs {}
impl ::core::clone::Clone for IMagneticStripeReaderStatusUpdatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMagneticStripeReaderTrackData(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMagneticStripeReaderTrackData {}
impl ::core::clone::Clone for IMagneticStripeReaderTrackData {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {}
impl ::core::clone::Clone for IMagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPosPrinter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPosPrinter {}
impl ::core::clone::Clone for IPosPrinter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPosPrinter2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPosPrinter2 {}
impl ::core::clone::Clone for IPosPrinter2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPosPrinterCapabilities(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPosPrinterCapabilities {}
impl ::core::clone::Clone for IPosPrinterCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPosPrinterCharacterSetIdsStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPosPrinterCharacterSetIdsStatics {}
impl ::core::clone::Clone for IPosPrinterCharacterSetIdsStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPosPrinterFontProperty(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPosPrinterFontProperty {}
impl ::core::clone::Clone for IPosPrinterFontProperty {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPosPrinterJob(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPosPrinterJob {}
impl ::core::clone::Clone for IPosPrinterJob {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPosPrinterPrintOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPosPrinterPrintOptions {}
impl ::core::clone::Clone for IPosPrinterPrintOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPosPrinterReleaseDeviceRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPosPrinterReleaseDeviceRequestedEventArgs {}
impl ::core::clone::Clone for IPosPrinterReleaseDeviceRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPosPrinterStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPosPrinterStatics {}
impl ::core::clone::Clone for IPosPrinterStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPosPrinterStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPosPrinterStatics2 {}
impl ::core::clone::Clone for IPosPrinterStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPosPrinterStatus(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPosPrinterStatus {}
impl ::core::clone::Clone for IPosPrinterStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPosPrinterStatusUpdatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPosPrinterStatusUpdatedEventArgs {}
impl ::core::clone::Clone for IPosPrinterStatusUpdatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IReceiptOrSlipJob(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IReceiptOrSlipJob {}
impl ::core::clone::Clone for IReceiptOrSlipJob {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IReceiptPrintJob(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IReceiptPrintJob {}
impl ::core::clone::Clone for IReceiptPrintJob {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IReceiptPrintJob2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IReceiptPrintJob2 {}
impl ::core::clone::Clone for IReceiptPrintJob2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IReceiptPrinterCapabilities(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IReceiptPrinterCapabilities {}
impl ::core::clone::Clone for IReceiptPrinterCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IReceiptPrinterCapabilities2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IReceiptPrinterCapabilities2 {}
impl ::core::clone::Clone for IReceiptPrinterCapabilities2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISlipPrintJob(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISlipPrintJob {}
impl ::core::clone::Clone for ISlipPrintJob {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISlipPrinterCapabilities(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISlipPrinterCapabilities {}
impl ::core::clone::Clone for ISlipPrinterCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISlipPrinterCapabilities2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISlipPrinterCapabilities2 {}
impl ::core::clone::Clone for ISlipPrinterCapabilities2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUnifiedPosErrorData(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUnifiedPosErrorData {}
impl ::core::clone::Clone for IUnifiedPosErrorData {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUnifiedPosErrorDataFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUnifiedPosErrorDataFactory {}
impl ::core::clone::Clone for IUnifiedPosErrorDataFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct JournalPrintJob(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for JournalPrintJob {}
impl ::core::clone::Clone for JournalPrintJob {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct JournalPrinterCapabilities(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for JournalPrinterCapabilities {}
impl ::core::clone::Clone for JournalPrinterCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LineDisplay(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LineDisplay {}
impl ::core::clone::Clone for LineDisplay {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LineDisplayAttributes(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LineDisplayAttributes {}
impl ::core::clone::Clone for LineDisplayAttributes {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LineDisplayCapabilities(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LineDisplayCapabilities {}
impl ::core::clone::Clone for LineDisplayCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LineDisplayCursor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LineDisplayCursor {}
impl ::core::clone::Clone for LineDisplayCursor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LineDisplayCursorAttributes(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LineDisplayCursorAttributes {}
impl ::core::clone::Clone for LineDisplayCursorAttributes {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for LineDisplayCustomGlyphs {}
impl ::core::clone::Clone for LineDisplayCustomGlyphs {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for LineDisplayMarquee {}
impl ::core::clone::Clone for LineDisplayMarquee {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for LineDisplayStatisticsCategorySelector {}
impl ::core::clone::Clone for LineDisplayStatisticsCategorySelector {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LineDisplayStatusUpdatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LineDisplayStatusUpdatedEventArgs {}
impl ::core::clone::Clone for LineDisplayStatusUpdatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LineDisplayStoredBitmap(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LineDisplayStoredBitmap {}
impl ::core::clone::Clone for LineDisplayStoredBitmap {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for LineDisplayWindow {}
impl ::core::clone::Clone for LineDisplayWindow {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MagneticStripeReader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MagneticStripeReader {}
impl ::core::clone::Clone for MagneticStripeReader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MagneticStripeReaderAamvaCardDataReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MagneticStripeReaderAamvaCardDataReceivedEventArgs {}
impl ::core::clone::Clone for MagneticStripeReaderAamvaCardDataReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for MagneticStripeReaderBankCardDataReceivedEventArgs {}
impl ::core::clone::Clone for MagneticStripeReaderBankCardDataReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MagneticStripeReaderCapabilities(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MagneticStripeReaderCapabilities {}
impl ::core::clone::Clone for MagneticStripeReaderCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MagneticStripeReaderErrorOccurredEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MagneticStripeReaderErrorOccurredEventArgs {}
impl ::core::clone::Clone for MagneticStripeReaderErrorOccurredEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for MagneticStripeReaderReport {}
impl ::core::clone::Clone for MagneticStripeReaderReport {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for MagneticStripeReaderStatusUpdatedEventArgs {}
impl ::core::clone::Clone for MagneticStripeReaderStatusUpdatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MagneticStripeReaderTrackData(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MagneticStripeReaderTrackData {}
impl ::core::clone::Clone for MagneticStripeReaderTrackData {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {}
impl ::core::clone::Clone for MagneticStripeReaderVendorSpecificCardDataReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for PosPrinter {}
impl ::core::clone::Clone for PosPrinter {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for PosPrinterCapabilities {}
impl ::core::clone::Clone for PosPrinterCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for PosPrinterFontProperty {}
impl ::core::clone::Clone for PosPrinterFontProperty {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for PosPrinterPrintOptions {}
impl ::core::clone::Clone for PosPrinterPrintOptions {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for PosPrinterReleaseDeviceRequestedEventArgs {}
impl ::core::clone::Clone for PosPrinterReleaseDeviceRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for PosPrinterStatus {}
impl ::core::clone::Clone for PosPrinterStatus {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for PosPrinterStatusUpdatedEventArgs {}
impl ::core::clone::Clone for PosPrinterStatusUpdatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ReceiptPrintJob(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ReceiptPrintJob {}
impl ::core::clone::Clone for ReceiptPrintJob {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ReceiptPrinterCapabilities(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ReceiptPrinterCapabilities {}
impl ::core::clone::Clone for ReceiptPrinterCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for SlipPrintJob {}
impl ::core::clone::Clone for SlipPrintJob {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SlipPrinterCapabilities(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SlipPrinterCapabilities {}
impl ::core::clone::Clone for SlipPrinterCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UnifiedPosErrorData(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UnifiedPosErrorData {}
impl ::core::clone::Clone for UnifiedPosErrorData {
    fn clone(&self) -> Self {
        *self
    }
}
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

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "ApplicationModel_Wallet_System")]
pub mod System;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IWalletBarcode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWalletBarcodeFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWalletItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWalletItemCustomProperty(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWalletItemCustomPropertyFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWalletItemFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWalletItemStore(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWalletItemStore2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWalletManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWalletRelevantLocation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWalletTransaction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWalletVerb(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWalletVerbFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WalletActionKind(pub i32);
impl WalletActionKind {
    pub const OpenItem: WalletActionKind = WalletActionKind(0i32);
    pub const Transaction: WalletActionKind = WalletActionKind(1i32);
    pub const MoreTransactions: WalletActionKind = WalletActionKind(2i32);
    pub const Message: WalletActionKind = WalletActionKind(3i32);
    pub const Verb: WalletActionKind = WalletActionKind(4i32);
}
#[repr(transparent)]
pub struct WalletBarcode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WalletBarcodeSymbology(pub i32);
impl WalletBarcodeSymbology {
    pub const Invalid: WalletBarcodeSymbology = WalletBarcodeSymbology(0i32);
    pub const Upca: WalletBarcodeSymbology = WalletBarcodeSymbology(1i32);
    pub const Upce: WalletBarcodeSymbology = WalletBarcodeSymbology(2i32);
    pub const Ean13: WalletBarcodeSymbology = WalletBarcodeSymbology(3i32);
    pub const Ean8: WalletBarcodeSymbology = WalletBarcodeSymbology(4i32);
    pub const Itf: WalletBarcodeSymbology = WalletBarcodeSymbology(5i32);
    pub const Code39: WalletBarcodeSymbology = WalletBarcodeSymbology(6i32);
    pub const Code128: WalletBarcodeSymbology = WalletBarcodeSymbology(7i32);
    pub const Qr: WalletBarcodeSymbology = WalletBarcodeSymbology(8i32);
    pub const Pdf417: WalletBarcodeSymbology = WalletBarcodeSymbology(9i32);
    pub const Aztec: WalletBarcodeSymbology = WalletBarcodeSymbology(10i32);
    pub const Custom: WalletBarcodeSymbology = WalletBarcodeSymbology(100000i32);
}
#[repr(C)]
pub struct WalletContract(i32);
#[repr(transparent)]
pub struct WalletDetailViewPosition(pub i32);
impl WalletDetailViewPosition {
    pub const Hidden: WalletDetailViewPosition = WalletDetailViewPosition(0i32);
    pub const HeaderField1: WalletDetailViewPosition = WalletDetailViewPosition(1i32);
    pub const HeaderField2: WalletDetailViewPosition = WalletDetailViewPosition(2i32);
    pub const PrimaryField1: WalletDetailViewPosition = WalletDetailViewPosition(3i32);
    pub const PrimaryField2: WalletDetailViewPosition = WalletDetailViewPosition(4i32);
    pub const SecondaryField1: WalletDetailViewPosition = WalletDetailViewPosition(5i32);
    pub const SecondaryField2: WalletDetailViewPosition = WalletDetailViewPosition(6i32);
    pub const SecondaryField3: WalletDetailViewPosition = WalletDetailViewPosition(7i32);
    pub const SecondaryField4: WalletDetailViewPosition = WalletDetailViewPosition(8i32);
    pub const SecondaryField5: WalletDetailViewPosition = WalletDetailViewPosition(9i32);
    pub const CenterField1: WalletDetailViewPosition = WalletDetailViewPosition(10i32);
    pub const FooterField1: WalletDetailViewPosition = WalletDetailViewPosition(11i32);
    pub const FooterField2: WalletDetailViewPosition = WalletDetailViewPosition(12i32);
    pub const FooterField3: WalletDetailViewPosition = WalletDetailViewPosition(13i32);
    pub const FooterField4: WalletDetailViewPosition = WalletDetailViewPosition(14i32);
}
#[repr(transparent)]
pub struct WalletItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WalletItemCustomProperty(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WalletItemKind(pub i32);
impl WalletItemKind {
    pub const Invalid: WalletItemKind = WalletItemKind(0i32);
    pub const Deal: WalletItemKind = WalletItemKind(1i32);
    pub const General: WalletItemKind = WalletItemKind(2i32);
    pub const PaymentInstrument: WalletItemKind = WalletItemKind(3i32);
    pub const Ticket: WalletItemKind = WalletItemKind(4i32);
    pub const BoardingPass: WalletItemKind = WalletItemKind(5i32);
    pub const MembershipCard: WalletItemKind = WalletItemKind(6i32);
}
#[repr(transparent)]
pub struct WalletItemStore(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WalletRelevantLocation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WalletSummaryViewPosition(pub i32);
impl WalletSummaryViewPosition {
    pub const Hidden: WalletSummaryViewPosition = WalletSummaryViewPosition(0i32);
    pub const Field1: WalletSummaryViewPosition = WalletSummaryViewPosition(1i32);
    pub const Field2: WalletSummaryViewPosition = WalletSummaryViewPosition(2i32);
}
#[repr(transparent)]
pub struct WalletTransaction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WalletVerb(pub *mut ::core::ffi::c_void);

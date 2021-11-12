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
    pub const OpenItem: Self = Self(0i32);
    pub const Transaction: Self = Self(1i32);
    pub const MoreTransactions: Self = Self(2i32);
    pub const Message: Self = Self(3i32);
    pub const Verb: Self = Self(4i32);
}
#[repr(transparent)]
pub struct WalletBarcode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WalletBarcodeSymbology(pub i32);
impl WalletBarcodeSymbology {
    pub const Invalid: Self = Self(0i32);
    pub const Upca: Self = Self(1i32);
    pub const Upce: Self = Self(2i32);
    pub const Ean13: Self = Self(3i32);
    pub const Ean8: Self = Self(4i32);
    pub const Itf: Self = Self(5i32);
    pub const Code39: Self = Self(6i32);
    pub const Code128: Self = Self(7i32);
    pub const Qr: Self = Self(8i32);
    pub const Pdf417: Self = Self(9i32);
    pub const Aztec: Self = Self(10i32);
    pub const Custom: Self = Self(100000i32);
}
#[repr(C)]
pub struct WalletContract(i32);
#[repr(transparent)]
pub struct WalletDetailViewPosition(pub i32);
impl WalletDetailViewPosition {
    pub const Hidden: Self = Self(0i32);
    pub const HeaderField1: Self = Self(1i32);
    pub const HeaderField2: Self = Self(2i32);
    pub const PrimaryField1: Self = Self(3i32);
    pub const PrimaryField2: Self = Self(4i32);
    pub const SecondaryField1: Self = Self(5i32);
    pub const SecondaryField2: Self = Self(6i32);
    pub const SecondaryField3: Self = Self(7i32);
    pub const SecondaryField4: Self = Self(8i32);
    pub const SecondaryField5: Self = Self(9i32);
    pub const CenterField1: Self = Self(10i32);
    pub const FooterField1: Self = Self(11i32);
    pub const FooterField2: Self = Self(12i32);
    pub const FooterField3: Self = Self(13i32);
    pub const FooterField4: Self = Self(14i32);
}
#[repr(transparent)]
pub struct WalletItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WalletItemCustomProperty(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WalletItemKind(pub i32);
impl WalletItemKind {
    pub const Invalid: Self = Self(0i32);
    pub const Deal: Self = Self(1i32);
    pub const General: Self = Self(2i32);
    pub const PaymentInstrument: Self = Self(3i32);
    pub const Ticket: Self = Self(4i32);
    pub const BoardingPass: Self = Self(5i32);
    pub const MembershipCard: Self = Self(6i32);
}
#[repr(transparent)]
pub struct WalletItemStore(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WalletRelevantLocation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WalletSummaryViewPosition(pub i32);
impl WalletSummaryViewPosition {
    pub const Hidden: Self = Self(0i32);
    pub const Field1: Self = Self(1i32);
    pub const Field2: Self = Self(2i32);
}
#[repr(transparent)]
pub struct WalletTransaction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WalletVerb(pub *mut ::core::ffi::c_void);

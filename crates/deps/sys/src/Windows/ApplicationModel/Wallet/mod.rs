#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "ApplicationModel_Wallet_System")]
pub mod System;
#[link(name = "windows")]
extern "system" {}
pub struct IWalletBarcode(pub *mut ::core::ffi::c_void);
pub struct IWalletBarcodeFactory(pub *mut ::core::ffi::c_void);
pub struct IWalletItem(pub *mut ::core::ffi::c_void);
pub struct IWalletItemCustomProperty(pub *mut ::core::ffi::c_void);
pub struct IWalletItemCustomPropertyFactory(pub *mut ::core::ffi::c_void);
pub struct IWalletItemFactory(pub *mut ::core::ffi::c_void);
pub struct IWalletItemStore(pub *mut ::core::ffi::c_void);
pub struct IWalletItemStore2(pub *mut ::core::ffi::c_void);
pub struct IWalletManagerStatics(pub *mut ::core::ffi::c_void);
pub struct IWalletRelevantLocation(pub *mut ::core::ffi::c_void);
pub struct IWalletTransaction(pub *mut ::core::ffi::c_void);
pub struct IWalletVerb(pub *mut ::core::ffi::c_void);
pub struct IWalletVerbFactory(pub *mut ::core::ffi::c_void);
pub struct WalletActionKind(i32);
pub struct WalletBarcode(i32);
pub struct WalletBarcodeSymbology(i32);
pub struct WalletContract(i32);
pub struct WalletDetailViewPosition(i32);
pub struct WalletItem(i32);
pub struct WalletItemCustomProperty(i32);
pub struct WalletItemKind(i32);
pub struct WalletItemStore(i32);
pub struct WalletManager(i32);
pub struct WalletRelevantLocation(i32);
pub struct WalletSummaryViewPosition(i32);
pub struct WalletTransaction(i32);
pub struct WalletVerb(i32);

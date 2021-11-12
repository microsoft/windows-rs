#![allow(non_snake_case, non_camel_case_types)]
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
pub struct WalletActionKind(i32);
#[repr(transparent)]
pub struct WalletBarcode(pub *mut ::core::ffi::c_void);
pub struct WalletBarcodeSymbology(i32);
pub struct WalletContract(i32);
pub struct WalletDetailViewPosition(i32);
#[repr(transparent)]
pub struct WalletItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WalletItemCustomProperty(pub *mut ::core::ffi::c_void);
pub struct WalletItemKind(i32);
#[repr(transparent)]
pub struct WalletItemStore(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WalletManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WalletRelevantLocation(pub *mut ::core::ffi::c_void);
pub struct WalletSummaryViewPosition(i32);
#[repr(transparent)]
pub struct WalletTransaction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WalletVerb(pub *mut ::core::ffi::c_void);

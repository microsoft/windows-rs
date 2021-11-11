#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "ApplicationModel_Wallet_System")]
pub mod System;
#[link(name = "windows")]
extern "system" {
    fn IWalletBarcode();
    fn IWalletBarcodeFactory();
    fn IWalletItem();
    fn IWalletItemCustomProperty();
    fn IWalletItemCustomPropertyFactory();
    fn IWalletItemFactory();
    fn IWalletItemStore();
    fn IWalletItemStore2();
    fn IWalletManagerStatics();
    fn IWalletRelevantLocation();
    fn IWalletTransaction();
    fn IWalletVerb();
    fn IWalletVerbFactory();
    fn WalletActionKind();
    fn WalletBarcode();
    fn WalletBarcodeSymbology();
    fn WalletContract();
    fn WalletDetailViewPosition();
    fn WalletItem();
    fn WalletItemCustomProperty();
    fn WalletItemKind();
    fn WalletItemStore();
    fn WalletManager();
    fn WalletRelevantLocation();
    fn WalletSummaryViewPosition();
    fn WalletTransaction();
    fn WalletVerb();
}

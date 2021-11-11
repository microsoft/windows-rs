#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IWalletItemSystemStore();
    fn IWalletItemSystemStore2();
    fn IWalletManagerSystemStatics();
    fn WalletItemAppAssociation();
    fn WalletItemSystemStore();
    fn WalletManagerSystem();
}

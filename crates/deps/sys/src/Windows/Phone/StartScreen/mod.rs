#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn DualSimTile();
    fn DualSimTileContract();
    fn IDualSimTile();
    fn IDualSimTileStatics();
    fn IToastNotificationManagerStatics3();
}

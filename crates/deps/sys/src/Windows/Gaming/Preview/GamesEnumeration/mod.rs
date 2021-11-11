#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn GameList();
    fn GameListCategory();
    fn GameListChangedEventHandler();
    fn GameListEntry();
    fn GameListEntryLaunchableState();
    fn GameListRemovedEventHandler();
    fn GameModeConfiguration();
    fn GameModeUserConfiguration();
    fn IGameListEntry();
    fn IGameListEntry2();
    fn IGameListStatics();
    fn IGameListStatics2();
    fn IGameModeConfiguration();
    fn IGameModeUserConfiguration();
    fn IGameModeUserConfigurationStatics();
}

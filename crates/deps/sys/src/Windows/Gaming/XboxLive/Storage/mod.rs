#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn GameSaveBlobGetResult();
    fn GameSaveBlobInfo();
    fn GameSaveBlobInfoGetResult();
    fn GameSaveBlobInfoQuery();
    fn GameSaveContainer();
    fn GameSaveContainerInfo();
    fn GameSaveContainerInfoGetResult();
    fn GameSaveContainerInfoQuery();
    fn GameSaveErrorStatus();
    fn GameSaveOperationResult();
    fn GameSaveProvider();
    fn GameSaveProviderGetResult();
    fn IGameSaveBlobGetResult();
    fn IGameSaveBlobInfo();
    fn IGameSaveBlobInfoGetResult();
    fn IGameSaveBlobInfoQuery();
    fn IGameSaveContainer();
    fn IGameSaveContainerInfo();
    fn IGameSaveContainerInfoGetResult();
    fn IGameSaveContainerInfoQuery();
    fn IGameSaveOperationResult();
    fn IGameSaveProvider();
    fn IGameSaveProviderGetResult();
    fn IGameSaveProviderStatics();
}

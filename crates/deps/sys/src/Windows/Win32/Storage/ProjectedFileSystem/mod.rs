#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn PrjAllocateAlignedBuffer();
    fn PrjClearNegativePathCache();
    fn PrjCompleteCommand();
    fn PrjDeleteFile();
    fn PrjDoesNameContainWildCards();
    fn PrjFileNameCompare();
    fn PrjFileNameMatch();
    fn PrjFillDirEntryBuffer();
    fn PrjFillDirEntryBuffer2();
    fn PrjFreeAlignedBuffer();
    fn PrjGetOnDiskFileState();
    fn PrjGetVirtualizationInstanceInfo();
    fn PrjMarkDirectoryAsPlaceholder();
    fn PrjStartVirtualizing();
    fn PrjStopVirtualizing();
    fn PrjUpdateFileIfNeeded();
    fn PrjWriteFileData();
    fn PrjWritePlaceholderInfo();
    fn PrjWritePlaceholderInfo2();
}

#[doc(hidden)]
#[repr(transparent)]
pub struct IPhotoImportDeleteImportedItemsFromSourceResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhotoImportDeleteImportedItemsFromSourceResult {
    type Vtable = IPhotoImportDeleteImportedItemsFromSourceResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf4e112f8_843d_428a_a1a6_81510292b0ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoImportDeleteImportedItemsFromSourceResult_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Session: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub HasSucceeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub DeletedItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DeletedItems: usize,
    pub PhotosCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub PhotosSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub VideosCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub VideosSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub SidecarsCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SidecarsSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub SiblingsCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SiblingsSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub TotalCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub TotalSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhotoImportFindItemsResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhotoImportFindItemsResult {
    type Vtable = IPhotoImportFindItemsResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3915e647_6c78_492b_844e_8fe5e8f6bfb9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoImportFindItemsResult_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Session: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub HasSucceeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FoundItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FoundItems: usize,
    pub PhotosCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub PhotosSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub VideosCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub VideosSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub SidecarsCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SidecarsSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub SiblingsCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SiblingsSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub TotalCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub TotalSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub SelectAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SelectNone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SelectNewAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SelectNewAsync: usize,
    pub SetImportMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PhotoImportImportMode) -> ::windows::core::HRESULT,
    pub ImportMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhotoImportImportMode) -> ::windows::core::HRESULT,
    pub SelectedPhotosCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SelectedPhotosSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub SelectedVideosCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SelectedVideosSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub SelectedSidecarsCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SelectedSidecarsSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub SelectedSiblingsCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SelectedSiblingsSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub SelectedTotalCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SelectedTotalSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub SelectionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SelectionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSelectionChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSelectionChanged: usize,
    #[cfg(feature = "Foundation")]
    pub ImportItemsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ImportItemsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ItemImported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ItemImported: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveItemImported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveItemImported: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhotoImportFindItemsResult2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhotoImportFindItemsResult2 {
    type Vtable = IPhotoImportFindItemsResult2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbdd6a3b_ecf9_406a_815e_5015625b0a88);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoImportFindItemsResult2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub AddItemsInDateRangeToSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rangestart: super::super::Foundation::DateTime, rangelength: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AddItemsInDateRangeToSelection: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhotoImportImportItemsResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhotoImportImportItemsResult {
    type Vtable = IPhotoImportImportItemsResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe4d4f478_d419_4443_a84e_f06a850c0b00);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoImportImportItemsResult_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Session: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub HasSucceeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ImportedItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ImportedItems: usize,
    pub PhotosCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub PhotosSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub VideosCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub VideosSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub SidecarsCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SidecarsSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub SiblingsCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SiblingsSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub TotalCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub TotalSizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub DeleteImportedItemsFromSourceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteImportedItemsFromSourceAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhotoImportItem(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhotoImportItem {
    type Vtable = IPhotoImportItem_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9d07e76_9bfc_43b8_b356_633b6a988c9e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoImportItem_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ItemKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub ContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhotoImportContentType) -> ::windows::core::HRESULT,
    pub SizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Date: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Date: usize,
    pub Sibling: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Sidecars: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Sidecars: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub VideoSegments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    VideoSegments: usize,
    pub IsSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Thumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Thumbnail: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ImportedFileNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ImportedFileNames: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub DeletedFileNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DeletedFileNames: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhotoImportItem2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhotoImportItem2 {
    type Vtable = IPhotoImportItem2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf1053505_f53b_46a3_9e30_3610791a9110);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoImportItem2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhotoImportItemImportedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhotoImportItemImportedEventArgs {
    type Vtable = IPhotoImportItemImportedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x42cb2fdd_7d68_47b5_bc7c_ceb73e0c77dc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoImportItemImportedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub ImportedItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhotoImportManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhotoImportManagerStatics {
    type Vtable = IPhotoImportManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2771903d_a046_4f06_9b9c_bfd662e83287);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoImportManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub IsSupportedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsSupportedAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub FindAllSourcesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindAllSourcesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetPendingOperations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetPendingOperations: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhotoImportOperation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhotoImportOperation {
    type Vtable = IPhotoImportOperation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd9f797e4_a09a_4ee4_a4b1_20940277a5be);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoImportOperation_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Stage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhotoImportStage) -> ::windows::core::HRESULT,
    pub Session: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ContinueFindingItemsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContinueFindingItemsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ContinueImportingItemsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContinueImportingItemsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ContinueDeletingImportedItemsFromSourceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContinueDeletingImportedItemsFromSourceAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhotoImportSelectionChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhotoImportSelectionChangedEventArgs {
    type Vtable = IPhotoImportSelectionChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x10461782_fa9d_4c30_8bc9_4d64911572d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoImportSelectionChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub IsSelectionEmpty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhotoImportSession(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhotoImportSession {
    type Vtable = IPhotoImportSession_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa63916e_ecdb_4efe_94c6_5f5cafe34cfb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoImportSession_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SessionId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage")]
    pub SetDestinationFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    SetDestinationFolder: usize,
    #[cfg(feature = "Storage")]
    pub DestinationFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    DestinationFolder: usize,
    pub SetAppendSessionDateToDestinationFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub AppendSessionDateToDestinationFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetSubfolderCreationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PhotoImportSubfolderCreationMode) -> ::windows::core::HRESULT,
    pub SubfolderCreationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhotoImportSubfolderCreationMode) -> ::windows::core::HRESULT,
    pub SetDestinationFileNamePrefix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DestinationFileNamePrefix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FindItemsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contenttypefilter: PhotoImportContentTypeFilter, itemselectionmode: PhotoImportItemSelectionMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FindItemsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhotoImportSession2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhotoImportSession2 {
    type Vtable = IPhotoImportSession2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a526710_3ec6_469d_a375_2b9f4785391e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoImportSession2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub SetSubfolderDateFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PhotoImportSubfolderDateFormat) -> ::windows::core::HRESULT,
    pub SubfolderDateFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhotoImportSubfolderDateFormat) -> ::windows::core::HRESULT,
    pub SetRememberDeselectedItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub RememberDeselectedItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhotoImportSidecar(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhotoImportSidecar {
    type Vtable = IPhotoImportSidecar_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x46d7d757_f802_44c7_9c98_7a71f4bc1486);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoImportSidecar_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Date: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Date: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhotoImportSource(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhotoImportSource {
    type Vtable = IPhotoImportSource_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1f8ea35e_145b_4cd6_87f1_54965a982fef);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoImportSource_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Manufacturer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Model: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SerialNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ConnectionProtocol: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ConnectionTransport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhotoImportConnectionTransport) -> ::windows::core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhotoImportSourceType) -> ::windows::core::HRESULT,
    pub PowerSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhotoImportPowerSource) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub BatteryLevelPercent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BatteryLevelPercent: usize,
    #[cfg(feature = "Foundation")]
    pub DateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DateTime: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub StorageMedia: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StorageMedia: usize,
    #[cfg(feature = "Foundation")]
    pub IsLocked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    IsLocked: usize,
    pub IsMassStorage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Thumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Thumbnail: usize,
    pub CreateImportSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhotoImportSourceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhotoImportSourceStatics {
    type Vtable = IPhotoImportSourceStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0528e586_32d8_467c_8cee_23a1b2f43e85);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoImportSourceStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub FromIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FromIdAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub FromFolderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcerootfolder: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    FromFolderAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhotoImportStorageMedium(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhotoImportStorageMedium {
    type Vtable = IPhotoImportStorageMedium_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2b9b093_fc85_487f_87c2_58d675d05b07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoImportStorageMedium_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SerialNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub StorageMediumType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhotoImportStorageMediumType) -> ::windows::core::HRESULT,
    pub SupportedAccessMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PhotoImportAccessMode) -> ::windows::core::HRESULT,
    pub CapacityInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub AvailableSpaceInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    pub Refresh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPhotoImportVideoSegment(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPhotoImportVideoSegment {
    type Vtable = IPhotoImportVideoSegment_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x623c0289_321a_41d8_9166_8c62a333276c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPhotoImportVideoSegment_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SizeInBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Date: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Date: usize,
    pub Sibling: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Sidecars: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Sidecars: usize,
}
#[doc = "*Required features: `\"Media_Import\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PhotoImportAccessMode(pub i32);
impl PhotoImportAccessMode {
    pub const ReadWrite: Self = Self(0i32);
    pub const ReadOnly: Self = Self(1i32);
    pub const ReadAndDelete: Self = Self(2i32);
}
impl ::core::marker::Copy for PhotoImportAccessMode {}
impl ::core::clone::Clone for PhotoImportAccessMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhotoImportAccessMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PhotoImportAccessMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhotoImportAccessMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoImportAccessMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhotoImportAccessMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Import.PhotoImportAccessMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Import\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PhotoImportConnectionTransport(pub i32);
impl PhotoImportConnectionTransport {
    pub const Unknown: Self = Self(0i32);
    pub const Usb: Self = Self(1i32);
    pub const IP: Self = Self(2i32);
    pub const Bluetooth: Self = Self(3i32);
}
impl ::core::marker::Copy for PhotoImportConnectionTransport {}
impl ::core::clone::Clone for PhotoImportConnectionTransport {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhotoImportConnectionTransport {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PhotoImportConnectionTransport {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhotoImportConnectionTransport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoImportConnectionTransport").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhotoImportConnectionTransport {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Import.PhotoImportConnectionTransport;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Import\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PhotoImportContentType(pub i32);
impl PhotoImportContentType {
    pub const Unknown: Self = Self(0i32);
    pub const Image: Self = Self(1i32);
    pub const Video: Self = Self(2i32);
}
impl ::core::marker::Copy for PhotoImportContentType {}
impl ::core::clone::Clone for PhotoImportContentType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhotoImportContentType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PhotoImportContentType {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhotoImportContentType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoImportContentType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhotoImportContentType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Import.PhotoImportContentType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Import\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PhotoImportContentTypeFilter(pub i32);
impl PhotoImportContentTypeFilter {
    pub const OnlyImages: Self = Self(0i32);
    pub const OnlyVideos: Self = Self(1i32);
    pub const ImagesAndVideos: Self = Self(2i32);
    pub const ImagesAndVideosFromCameraRoll: Self = Self(3i32);
}
impl ::core::marker::Copy for PhotoImportContentTypeFilter {}
impl ::core::clone::Clone for PhotoImportContentTypeFilter {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhotoImportContentTypeFilter {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PhotoImportContentTypeFilter {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhotoImportContentTypeFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoImportContentTypeFilter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhotoImportContentTypeFilter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Import.PhotoImportContentTypeFilter;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Import\"`*"]
#[repr(transparent)]
pub struct PhotoImportDeleteImportedItemsFromSourceResult(::windows::core::IUnknown);
impl PhotoImportDeleteImportedItemsFromSourceResult {
    pub fn Session(&self) -> ::windows::core::Result<PhotoImportSession> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Session)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhotoImportSession>(result__)
        }
    }
    pub fn HasSucceeded(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HasSucceeded)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DeletedItems(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PhotoImportItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DeletedItems)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<PhotoImportItem>>(result__)
        }
    }
    pub fn PhotosCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PhotosCount)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn PhotosSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PhotosSizeInBytes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn VideosCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).VideosCount)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn VideosSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).VideosSizeInBytes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn SidecarsCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SidecarsCount)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SidecarsSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SidecarsSizeInBytes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn SiblingsCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SiblingsCount)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SiblingsSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SiblingsSizeInBytes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn TotalCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TotalCount)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn TotalSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TotalSizeInBytes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
}
impl ::core::clone::Clone for PhotoImportDeleteImportedItemsFromSourceResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhotoImportDeleteImportedItemsFromSourceResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhotoImportDeleteImportedItemsFromSourceResult {}
impl ::core::fmt::Debug for PhotoImportDeleteImportedItemsFromSourceResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoImportDeleteImportedItemsFromSourceResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhotoImportDeleteImportedItemsFromSourceResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Import.PhotoImportDeleteImportedItemsFromSourceResult;{f4e112f8-843d-428a-a1a6-81510292b0ae})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PhotoImportDeleteImportedItemsFromSourceResult {
    type Vtable = IPhotoImportDeleteImportedItemsFromSourceResult_Vtbl;
    const IID: ::windows::core::GUID = <IPhotoImportDeleteImportedItemsFromSourceResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PhotoImportDeleteImportedItemsFromSourceResult {
    const NAME: &'static str = "Windows.Media.Import.PhotoImportDeleteImportedItemsFromSourceResult";
}
impl ::core::convert::From<PhotoImportDeleteImportedItemsFromSourceResult> for ::windows::core::IUnknown {
    fn from(value: PhotoImportDeleteImportedItemsFromSourceResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoImportDeleteImportedItemsFromSourceResult> for ::windows::core::IUnknown {
    fn from(value: &PhotoImportDeleteImportedItemsFromSourceResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PhotoImportDeleteImportedItemsFromSourceResult> for &::windows::core::IUnknown {
    fn from(value: &PhotoImportDeleteImportedItemsFromSourceResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PhotoImportDeleteImportedItemsFromSourceResult> for ::windows::core::IInspectable {
    fn from(value: PhotoImportDeleteImportedItemsFromSourceResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoImportDeleteImportedItemsFromSourceResult> for ::windows::core::IInspectable {
    fn from(value: &PhotoImportDeleteImportedItemsFromSourceResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PhotoImportDeleteImportedItemsFromSourceResult> for &::windows::core::IInspectable {
    fn from(value: &PhotoImportDeleteImportedItemsFromSourceResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for PhotoImportDeleteImportedItemsFromSourceResult {}
unsafe impl ::core::marker::Sync for PhotoImportDeleteImportedItemsFromSourceResult {}
#[doc = "*Required features: `\"Media_Import\"`*"]
#[repr(transparent)]
pub struct PhotoImportFindItemsResult(::windows::core::IUnknown);
impl PhotoImportFindItemsResult {
    pub fn Session(&self) -> ::windows::core::Result<PhotoImportSession> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Session)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhotoImportSession>(result__)
        }
    }
    pub fn HasSucceeded(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HasSucceeded)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FoundItems(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PhotoImportItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FoundItems)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<PhotoImportItem>>(result__)
        }
    }
    pub fn PhotosCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PhotosCount)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn PhotosSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PhotosSizeInBytes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn VideosCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).VideosCount)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn VideosSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).VideosSizeInBytes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn SidecarsCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SidecarsCount)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SidecarsSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SidecarsSizeInBytes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn SiblingsCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SiblingsCount)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SiblingsSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SiblingsSizeInBytes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn TotalCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TotalCount)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn TotalSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TotalSizeInBytes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn SelectAll(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SelectAll)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn SelectNone(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SelectNone)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SelectNewAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectNewAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn SetImportMode(&self, value: PhotoImportImportMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetImportMode)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ImportMode(&self) -> ::windows::core::Result<PhotoImportImportMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ImportMode)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhotoImportImportMode>(result__)
        }
    }
    pub fn SelectedPhotosCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectedPhotosCount)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SelectedPhotosSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectedPhotosSizeInBytes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn SelectedVideosCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectedVideosCount)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SelectedVideosSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectedVideosSizeInBytes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn SelectedSidecarsCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectedSidecarsCount)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SelectedSidecarsSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectedSidecarsSizeInBytes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn SelectedSiblingsCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectedSiblingsCount)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SelectedSiblingsSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectedSiblingsSizeInBytes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn SelectedTotalCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectedTotalCount)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SelectedTotalSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectedTotalSizeInBytes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SelectionChanged<'a, P0>(&self, value: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<PhotoImportFindItemsResult, PhotoImportSelectionChangedEventArgs>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SelectionChanged)(::windows::core::Interface::as_raw(this), value.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSelectionChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveSelectionChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ImportItemsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<PhotoImportImportItemsResult, PhotoImportProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ImportItemsAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<PhotoImportImportItemsResult, PhotoImportProgress>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ItemImported<'a, P0>(&self, value: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<PhotoImportFindItemsResult, PhotoImportItemImportedEventArgs>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ItemImported)(::windows::core::Interface::as_raw(this), value.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveItemImported(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveItemImported)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AddItemsInDateRangeToSelection(&self, rangestart: super::super::Foundation::DateTime, rangelength: super::super::Foundation::TimeSpan) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPhotoImportFindItemsResult2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).AddItemsInDateRangeToSelection)(::windows::core::Interface::as_raw(this), rangestart, rangelength).ok() }
    }
}
impl ::core::clone::Clone for PhotoImportFindItemsResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhotoImportFindItemsResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhotoImportFindItemsResult {}
impl ::core::fmt::Debug for PhotoImportFindItemsResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoImportFindItemsResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhotoImportFindItemsResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Import.PhotoImportFindItemsResult;{3915e647-6c78-492b-844e-8fe5e8f6bfb9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PhotoImportFindItemsResult {
    type Vtable = IPhotoImportFindItemsResult_Vtbl;
    const IID: ::windows::core::GUID = <IPhotoImportFindItemsResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PhotoImportFindItemsResult {
    const NAME: &'static str = "Windows.Media.Import.PhotoImportFindItemsResult";
}
impl ::core::convert::From<PhotoImportFindItemsResult> for ::windows::core::IUnknown {
    fn from(value: PhotoImportFindItemsResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoImportFindItemsResult> for ::windows::core::IUnknown {
    fn from(value: &PhotoImportFindItemsResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PhotoImportFindItemsResult> for &::windows::core::IUnknown {
    fn from(value: &PhotoImportFindItemsResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PhotoImportFindItemsResult> for ::windows::core::IInspectable {
    fn from(value: PhotoImportFindItemsResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoImportFindItemsResult> for ::windows::core::IInspectable {
    fn from(value: &PhotoImportFindItemsResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PhotoImportFindItemsResult> for &::windows::core::IInspectable {
    fn from(value: &PhotoImportFindItemsResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for PhotoImportFindItemsResult {}
unsafe impl ::core::marker::Sync for PhotoImportFindItemsResult {}
#[doc = "*Required features: `\"Media_Import\"`*"]
#[repr(transparent)]
pub struct PhotoImportImportItemsResult(::windows::core::IUnknown);
impl PhotoImportImportItemsResult {
    pub fn Session(&self) -> ::windows::core::Result<PhotoImportSession> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Session)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhotoImportSession>(result__)
        }
    }
    pub fn HasSucceeded(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HasSucceeded)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ImportedItems(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PhotoImportItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ImportedItems)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<PhotoImportItem>>(result__)
        }
    }
    pub fn PhotosCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PhotosCount)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn PhotosSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PhotosSizeInBytes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn VideosCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).VideosCount)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn VideosSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).VideosSizeInBytes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn SidecarsCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SidecarsCount)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SidecarsSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SidecarsSizeInBytes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn SiblingsCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SiblingsCount)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn SiblingsSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SiblingsSizeInBytes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn TotalCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TotalCount)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    pub fn TotalSizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TotalSizeInBytes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteImportedItemsFromSourceAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<PhotoImportDeleteImportedItemsFromSourceResult, f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DeleteImportedItemsFromSourceAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<PhotoImportDeleteImportedItemsFromSourceResult, f64>>(result__)
        }
    }
}
impl ::core::clone::Clone for PhotoImportImportItemsResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhotoImportImportItemsResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhotoImportImportItemsResult {}
impl ::core::fmt::Debug for PhotoImportImportItemsResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoImportImportItemsResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhotoImportImportItemsResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Import.PhotoImportImportItemsResult;{e4d4f478-d419-4443-a84e-f06a850c0b00})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PhotoImportImportItemsResult {
    type Vtable = IPhotoImportImportItemsResult_Vtbl;
    const IID: ::windows::core::GUID = <IPhotoImportImportItemsResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PhotoImportImportItemsResult {
    const NAME: &'static str = "Windows.Media.Import.PhotoImportImportItemsResult";
}
impl ::core::convert::From<PhotoImportImportItemsResult> for ::windows::core::IUnknown {
    fn from(value: PhotoImportImportItemsResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoImportImportItemsResult> for ::windows::core::IUnknown {
    fn from(value: &PhotoImportImportItemsResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PhotoImportImportItemsResult> for &::windows::core::IUnknown {
    fn from(value: &PhotoImportImportItemsResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PhotoImportImportItemsResult> for ::windows::core::IInspectable {
    fn from(value: PhotoImportImportItemsResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoImportImportItemsResult> for ::windows::core::IInspectable {
    fn from(value: &PhotoImportImportItemsResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PhotoImportImportItemsResult> for &::windows::core::IInspectable {
    fn from(value: &PhotoImportImportItemsResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for PhotoImportImportItemsResult {}
unsafe impl ::core::marker::Sync for PhotoImportImportItemsResult {}
#[doc = "*Required features: `\"Media_Import\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PhotoImportImportMode(pub i32);
impl PhotoImportImportMode {
    pub const ImportEverything: Self = Self(0i32);
    pub const IgnoreSidecars: Self = Self(1i32);
    pub const IgnoreSiblings: Self = Self(2i32);
    pub const IgnoreSidecarsAndSiblings: Self = Self(3i32);
}
impl ::core::marker::Copy for PhotoImportImportMode {}
impl ::core::clone::Clone for PhotoImportImportMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhotoImportImportMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PhotoImportImportMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhotoImportImportMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoImportImportMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhotoImportImportMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Import.PhotoImportImportMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Import\"`*"]
#[repr(transparent)]
pub struct PhotoImportItem(::windows::core::IUnknown);
impl PhotoImportItem {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ItemKey(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ItemKey)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn ContentType(&self) -> ::windows::core::Result<PhotoImportContentType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ContentType)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhotoImportContentType>(result__)
        }
    }
    pub fn SizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SizeInBytes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Date(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Date)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    pub fn Sibling(&self) -> ::windows::core::Result<PhotoImportSidecar> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Sibling)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhotoImportSidecar>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Sidecars(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PhotoImportSidecar>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Sidecars)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<PhotoImportSidecar>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn VideoSegments(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PhotoImportVideoSegment>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).VideoSegments)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<PhotoImportVideoSegment>>(result__)
        }
    }
    pub fn IsSelected(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsSelected)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsSelected(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsSelected)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Thumbnail(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Thumbnail)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ImportedFileNames(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ImportedFileNames)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DeletedFileNames(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DeletedFileNames)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    pub fn Path(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPhotoImportItem2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Path)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for PhotoImportItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhotoImportItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhotoImportItem {}
impl ::core::fmt::Debug for PhotoImportItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoImportItem").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhotoImportItem {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Import.PhotoImportItem;{a9d07e76-9bfc-43b8-b356-633b6a988c9e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PhotoImportItem {
    type Vtable = IPhotoImportItem_Vtbl;
    const IID: ::windows::core::GUID = <IPhotoImportItem as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PhotoImportItem {
    const NAME: &'static str = "Windows.Media.Import.PhotoImportItem";
}
impl ::core::convert::From<PhotoImportItem> for ::windows::core::IUnknown {
    fn from(value: PhotoImportItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoImportItem> for ::windows::core::IUnknown {
    fn from(value: &PhotoImportItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PhotoImportItem> for &::windows::core::IUnknown {
    fn from(value: &PhotoImportItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PhotoImportItem> for ::windows::core::IInspectable {
    fn from(value: PhotoImportItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoImportItem> for ::windows::core::IInspectable {
    fn from(value: &PhotoImportItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PhotoImportItem> for &::windows::core::IInspectable {
    fn from(value: &PhotoImportItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for PhotoImportItem {}
unsafe impl ::core::marker::Sync for PhotoImportItem {}
#[doc = "*Required features: `\"Media_Import\"`*"]
#[repr(transparent)]
pub struct PhotoImportItemImportedEventArgs(::windows::core::IUnknown);
impl PhotoImportItemImportedEventArgs {
    pub fn ImportedItem(&self) -> ::windows::core::Result<PhotoImportItem> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ImportedItem)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhotoImportItem>(result__)
        }
    }
}
impl ::core::clone::Clone for PhotoImportItemImportedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhotoImportItemImportedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhotoImportItemImportedEventArgs {}
impl ::core::fmt::Debug for PhotoImportItemImportedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoImportItemImportedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhotoImportItemImportedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Import.PhotoImportItemImportedEventArgs;{42cb2fdd-7d68-47b5-bc7c-ceb73e0c77dc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PhotoImportItemImportedEventArgs {
    type Vtable = IPhotoImportItemImportedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IPhotoImportItemImportedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PhotoImportItemImportedEventArgs {
    const NAME: &'static str = "Windows.Media.Import.PhotoImportItemImportedEventArgs";
}
impl ::core::convert::From<PhotoImportItemImportedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PhotoImportItemImportedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoImportItemImportedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PhotoImportItemImportedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PhotoImportItemImportedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &PhotoImportItemImportedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PhotoImportItemImportedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PhotoImportItemImportedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoImportItemImportedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PhotoImportItemImportedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PhotoImportItemImportedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &PhotoImportItemImportedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for PhotoImportItemImportedEventArgs {}
unsafe impl ::core::marker::Sync for PhotoImportItemImportedEventArgs {}
#[doc = "*Required features: `\"Media_Import\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PhotoImportItemSelectionMode(pub i32);
impl PhotoImportItemSelectionMode {
    pub const SelectAll: Self = Self(0i32);
    pub const SelectNone: Self = Self(1i32);
    pub const SelectNew: Self = Self(2i32);
}
impl ::core::marker::Copy for PhotoImportItemSelectionMode {}
impl ::core::clone::Clone for PhotoImportItemSelectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhotoImportItemSelectionMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PhotoImportItemSelectionMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhotoImportItemSelectionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoImportItemSelectionMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhotoImportItemSelectionMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Import.PhotoImportItemSelectionMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Import\"`*"]
pub struct PhotoImportManager;
impl PhotoImportManager {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsSupportedAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IPhotoImportManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsSupportedAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindAllSourcesAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PhotoImportSource>>> {
        Self::IPhotoImportManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FindAllSourcesAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PhotoImportSource>>>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetPendingOperations() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PhotoImportOperation>> {
        Self::IPhotoImportManagerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetPendingOperations)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<PhotoImportOperation>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPhotoImportManagerStatics<R, F: FnOnce(&IPhotoImportManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PhotoImportManager, IPhotoImportManagerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for PhotoImportManager {
    const NAME: &'static str = "Windows.Media.Import.PhotoImportManager";
}
#[doc = "*Required features: `\"Media_Import\"`*"]
#[repr(transparent)]
pub struct PhotoImportOperation(::windows::core::IUnknown);
impl PhotoImportOperation {
    pub fn Stage(&self) -> ::windows::core::Result<PhotoImportStage> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Stage)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhotoImportStage>(result__)
        }
    }
    pub fn Session(&self) -> ::windows::core::Result<PhotoImportSession> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Session)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhotoImportSession>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ContinueFindingItemsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<PhotoImportFindItemsResult, u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ContinueFindingItemsAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<PhotoImportFindItemsResult, u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ContinueImportingItemsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<PhotoImportImportItemsResult, PhotoImportProgress>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ContinueImportingItemsAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<PhotoImportImportItemsResult, PhotoImportProgress>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ContinueDeletingImportedItemsFromSourceAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<PhotoImportDeleteImportedItemsFromSourceResult, f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ContinueDeletingImportedItemsFromSourceAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<PhotoImportDeleteImportedItemsFromSourceResult, f64>>(result__)
        }
    }
}
impl ::core::clone::Clone for PhotoImportOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhotoImportOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhotoImportOperation {}
impl ::core::fmt::Debug for PhotoImportOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoImportOperation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhotoImportOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Import.PhotoImportOperation;{d9f797e4-a09a-4ee4-a4b1-20940277a5be})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PhotoImportOperation {
    type Vtable = IPhotoImportOperation_Vtbl;
    const IID: ::windows::core::GUID = <IPhotoImportOperation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PhotoImportOperation {
    const NAME: &'static str = "Windows.Media.Import.PhotoImportOperation";
}
impl ::core::convert::From<PhotoImportOperation> for ::windows::core::IUnknown {
    fn from(value: PhotoImportOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoImportOperation> for ::windows::core::IUnknown {
    fn from(value: &PhotoImportOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PhotoImportOperation> for &::windows::core::IUnknown {
    fn from(value: &PhotoImportOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PhotoImportOperation> for ::windows::core::IInspectable {
    fn from(value: PhotoImportOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoImportOperation> for ::windows::core::IInspectable {
    fn from(value: &PhotoImportOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PhotoImportOperation> for &::windows::core::IInspectable {
    fn from(value: &PhotoImportOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for PhotoImportOperation {}
unsafe impl ::core::marker::Sync for PhotoImportOperation {}
#[doc = "*Required features: `\"Media_Import\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PhotoImportPowerSource(pub i32);
impl PhotoImportPowerSource {
    pub const Unknown: Self = Self(0i32);
    pub const Battery: Self = Self(1i32);
    pub const External: Self = Self(2i32);
}
impl ::core::marker::Copy for PhotoImportPowerSource {}
impl ::core::clone::Clone for PhotoImportPowerSource {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhotoImportPowerSource {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PhotoImportPowerSource {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhotoImportPowerSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoImportPowerSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhotoImportPowerSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Import.PhotoImportPowerSource;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Media_Import\"`*"]
pub struct PhotoImportProgress {
    pub ItemsImported: u32,
    pub TotalItemsToImport: u32,
    pub BytesImported: u64,
    pub TotalBytesToImport: u64,
    pub ImportProgress: f64,
}
impl ::core::marker::Copy for PhotoImportProgress {}
impl ::core::clone::Clone for PhotoImportProgress {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PhotoImportProgress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PhotoImportProgress").field("ItemsImported", &self.ItemsImported).field("TotalItemsToImport", &self.TotalItemsToImport).field("BytesImported", &self.BytesImported).field("TotalBytesToImport", &self.TotalBytesToImport).field("ImportProgress", &self.ImportProgress).finish()
    }
}
unsafe impl ::windows::core::Abi for PhotoImportProgress {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PhotoImportProgress {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Media.Import.PhotoImportProgress;u4;u4;u8;u8;f8)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for PhotoImportProgress {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PhotoImportProgress>()) == 0 }
    }
}
impl ::core::cmp::Eq for PhotoImportProgress {}
impl ::core::default::Default for PhotoImportProgress {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Media_Import\"`*"]
#[repr(transparent)]
pub struct PhotoImportSelectionChangedEventArgs(::windows::core::IUnknown);
impl PhotoImportSelectionChangedEventArgs {
    pub fn IsSelectionEmpty(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsSelectionEmpty)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for PhotoImportSelectionChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhotoImportSelectionChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhotoImportSelectionChangedEventArgs {}
impl ::core::fmt::Debug for PhotoImportSelectionChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoImportSelectionChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhotoImportSelectionChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Import.PhotoImportSelectionChangedEventArgs;{10461782-fa9d-4c30-8bc9-4d64911572d5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PhotoImportSelectionChangedEventArgs {
    type Vtable = IPhotoImportSelectionChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IPhotoImportSelectionChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PhotoImportSelectionChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Import.PhotoImportSelectionChangedEventArgs";
}
impl ::core::convert::From<PhotoImportSelectionChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PhotoImportSelectionChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoImportSelectionChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PhotoImportSelectionChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PhotoImportSelectionChangedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &PhotoImportSelectionChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PhotoImportSelectionChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PhotoImportSelectionChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoImportSelectionChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PhotoImportSelectionChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PhotoImportSelectionChangedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &PhotoImportSelectionChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for PhotoImportSelectionChangedEventArgs {}
unsafe impl ::core::marker::Sync for PhotoImportSelectionChangedEventArgs {}
#[doc = "*Required features: `\"Media_Import\"`*"]
#[repr(transparent)]
pub struct PhotoImportSession(::windows::core::IUnknown);
impl PhotoImportSession {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Source(&self) -> ::windows::core::Result<PhotoImportSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Source)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhotoImportSource>(result__)
        }
    }
    pub fn SessionId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SessionId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn SetDestinationFolder<'a, P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Storage::IStorageFolder>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDestinationFolder)(::windows::core::Interface::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage\"`*"]
    #[cfg(feature = "Storage")]
    pub fn DestinationFolder(&self) -> ::windows::core::Result<super::super::Storage::IStorageFolder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DestinationFolder)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::IStorageFolder>(result__)
        }
    }
    pub fn SetAppendSessionDateToDestinationFolder(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAppendSessionDateToDestinationFolder)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn AppendSessionDateToDestinationFolder(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AppendSessionDateToDestinationFolder)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetSubfolderCreationMode(&self, value: PhotoImportSubfolderCreationMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSubfolderCreationMode)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn SubfolderCreationMode(&self) -> ::windows::core::Result<PhotoImportSubfolderCreationMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SubfolderCreationMode)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhotoImportSubfolderCreationMode>(result__)
        }
    }
    pub fn SetDestinationFileNamePrefix(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDestinationFileNamePrefix)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn DestinationFileNamePrefix(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DestinationFileNamePrefix)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FindItemsAsync(&self, contenttypefilter: PhotoImportContentTypeFilter, itemselectionmode: PhotoImportItemSelectionMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<PhotoImportFindItemsResult, u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FindItemsAsync)(::windows::core::Interface::as_raw(this), contenttypefilter, itemselectionmode, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<PhotoImportFindItemsResult, u32>>(result__)
        }
    }
    pub fn SetSubfolderDateFormat(&self, value: PhotoImportSubfolderDateFormat) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPhotoImportSession2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetSubfolderDateFormat)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn SubfolderDateFormat(&self) -> ::windows::core::Result<PhotoImportSubfolderDateFormat> {
        let this = &::windows::core::Interface::cast::<IPhotoImportSession2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SubfolderDateFormat)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhotoImportSubfolderDateFormat>(result__)
        }
    }
    pub fn SetRememberDeselectedItems(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPhotoImportSession2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetRememberDeselectedItems)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn RememberDeselectedItems(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IPhotoImportSession2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RememberDeselectedItems)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for PhotoImportSession {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhotoImportSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhotoImportSession {}
impl ::core::fmt::Debug for PhotoImportSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoImportSession").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhotoImportSession {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Import.PhotoImportSession;{aa63916e-ecdb-4efe-94c6-5f5cafe34cfb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PhotoImportSession {
    type Vtable = IPhotoImportSession_Vtbl;
    const IID: ::windows::core::GUID = <IPhotoImportSession as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PhotoImportSession {
    const NAME: &'static str = "Windows.Media.Import.PhotoImportSession";
}
impl ::core::convert::From<PhotoImportSession> for ::windows::core::IUnknown {
    fn from(value: PhotoImportSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoImportSession> for ::windows::core::IUnknown {
    fn from(value: &PhotoImportSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PhotoImportSession> for &::windows::core::IUnknown {
    fn from(value: &PhotoImportSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PhotoImportSession> for ::windows::core::IInspectable {
    fn from(value: PhotoImportSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoImportSession> for ::windows::core::IInspectable {
    fn from(value: &PhotoImportSession) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PhotoImportSession> for &::windows::core::IInspectable {
    fn from(value: &PhotoImportSession) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<PhotoImportSession> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: PhotoImportSession) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&PhotoImportSession> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &PhotoImportSession) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::core::convert::TryFrom<&PhotoImportSession> for ::windows::core::InParam<'a, super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &PhotoImportSession) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for PhotoImportSession {}
unsafe impl ::core::marker::Sync for PhotoImportSession {}
#[doc = "*Required features: `\"Media_Import\"`*"]
#[repr(transparent)]
pub struct PhotoImportSidecar(::windows::core::IUnknown);
impl PhotoImportSidecar {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SizeInBytes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Date(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Date)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
}
impl ::core::clone::Clone for PhotoImportSidecar {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhotoImportSidecar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhotoImportSidecar {}
impl ::core::fmt::Debug for PhotoImportSidecar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoImportSidecar").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhotoImportSidecar {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Import.PhotoImportSidecar;{46d7d757-f802-44c7-9c98-7a71f4bc1486})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PhotoImportSidecar {
    type Vtable = IPhotoImportSidecar_Vtbl;
    const IID: ::windows::core::GUID = <IPhotoImportSidecar as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PhotoImportSidecar {
    const NAME: &'static str = "Windows.Media.Import.PhotoImportSidecar";
}
impl ::core::convert::From<PhotoImportSidecar> for ::windows::core::IUnknown {
    fn from(value: PhotoImportSidecar) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoImportSidecar> for ::windows::core::IUnknown {
    fn from(value: &PhotoImportSidecar) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PhotoImportSidecar> for &::windows::core::IUnknown {
    fn from(value: &PhotoImportSidecar) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PhotoImportSidecar> for ::windows::core::IInspectable {
    fn from(value: PhotoImportSidecar) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoImportSidecar> for ::windows::core::IInspectable {
    fn from(value: &PhotoImportSidecar) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PhotoImportSidecar> for &::windows::core::IInspectable {
    fn from(value: &PhotoImportSidecar) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for PhotoImportSidecar {}
unsafe impl ::core::marker::Sync for PhotoImportSidecar {}
#[doc = "*Required features: `\"Media_Import\"`*"]
#[repr(transparent)]
pub struct PhotoImportSource(::windows::core::IUnknown);
impl PhotoImportSource {
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DisplayName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Description)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Manufacturer(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Manufacturer)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Model(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Model)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SerialNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SerialNumber)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ConnectionProtocol(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ConnectionProtocol)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ConnectionTransport(&self) -> ::windows::core::Result<PhotoImportConnectionTransport> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ConnectionTransport)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhotoImportConnectionTransport>(result__)
        }
    }
    pub fn Type(&self) -> ::windows::core::Result<PhotoImportSourceType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Type)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhotoImportSourceType>(result__)
        }
    }
    pub fn PowerSource(&self) -> ::windows::core::Result<PhotoImportPowerSource> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PowerSource)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhotoImportPowerSource>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BatteryLevelPercent(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).BatteryLevelPercent)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DateTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DateTime)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn StorageMedia(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PhotoImportStorageMedium>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).StorageMedia)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<PhotoImportStorageMedium>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn IsLocked(&self) -> ::windows::core::Result<super::super::Foundation::IReference<bool>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsLocked)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<bool>>(result__)
        }
    }
    pub fn IsMassStorage(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsMassStorage)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Thumbnail(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Thumbnail)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::Streams::IRandomAccessStreamReference>(result__)
        }
    }
    pub fn CreateImportSession(&self) -> ::windows::core::Result<PhotoImportSession> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateImportSession)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhotoImportSession>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FromIdAsync(sourceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhotoImportSource>> {
        Self::IPhotoImportSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FromIdAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(sourceid), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<PhotoImportSource>>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn FromFolderAsync<'a, P0, E0>(sourcerootfolder: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PhotoImportSource>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Storage::IStorageFolder>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IPhotoImportSourceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FromFolderAsync)(::windows::core::Interface::as_raw(this), sourcerootfolder.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<PhotoImportSource>>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPhotoImportSourceStatics<R, F: FnOnce(&IPhotoImportSourceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PhotoImportSource, IPhotoImportSourceStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for PhotoImportSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhotoImportSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhotoImportSource {}
impl ::core::fmt::Debug for PhotoImportSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoImportSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhotoImportSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Import.PhotoImportSource;{1f8ea35e-145b-4cd6-87f1-54965a982fef})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PhotoImportSource {
    type Vtable = IPhotoImportSource_Vtbl;
    const IID: ::windows::core::GUID = <IPhotoImportSource as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PhotoImportSource {
    const NAME: &'static str = "Windows.Media.Import.PhotoImportSource";
}
impl ::core::convert::From<PhotoImportSource> for ::windows::core::IUnknown {
    fn from(value: PhotoImportSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoImportSource> for ::windows::core::IUnknown {
    fn from(value: &PhotoImportSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PhotoImportSource> for &::windows::core::IUnknown {
    fn from(value: &PhotoImportSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PhotoImportSource> for ::windows::core::IInspectable {
    fn from(value: PhotoImportSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoImportSource> for ::windows::core::IInspectable {
    fn from(value: &PhotoImportSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PhotoImportSource> for &::windows::core::IInspectable {
    fn from(value: &PhotoImportSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for PhotoImportSource {}
unsafe impl ::core::marker::Sync for PhotoImportSource {}
#[doc = "*Required features: `\"Media_Import\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PhotoImportSourceType(pub i32);
impl PhotoImportSourceType {
    pub const Generic: Self = Self(0i32);
    pub const Camera: Self = Self(1i32);
    pub const MediaPlayer: Self = Self(2i32);
    pub const Phone: Self = Self(3i32);
    pub const Video: Self = Self(4i32);
    pub const PersonalInfoManager: Self = Self(5i32);
    pub const AudioRecorder: Self = Self(6i32);
}
impl ::core::marker::Copy for PhotoImportSourceType {}
impl ::core::clone::Clone for PhotoImportSourceType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhotoImportSourceType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PhotoImportSourceType {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhotoImportSourceType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoImportSourceType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhotoImportSourceType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Import.PhotoImportSourceType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Import\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PhotoImportStage(pub i32);
impl PhotoImportStage {
    pub const NotStarted: Self = Self(0i32);
    pub const FindingItems: Self = Self(1i32);
    pub const ImportingItems: Self = Self(2i32);
    pub const DeletingImportedItemsFromSource: Self = Self(3i32);
}
impl ::core::marker::Copy for PhotoImportStage {}
impl ::core::clone::Clone for PhotoImportStage {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhotoImportStage {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PhotoImportStage {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhotoImportStage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoImportStage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhotoImportStage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Import.PhotoImportStage;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Import\"`*"]
#[repr(transparent)]
pub struct PhotoImportStorageMedium(::windows::core::IUnknown);
impl PhotoImportStorageMedium {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Description)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SerialNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SerialNumber)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn StorageMediumType(&self) -> ::windows::core::Result<PhotoImportStorageMediumType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).StorageMediumType)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhotoImportStorageMediumType>(result__)
        }
    }
    pub fn SupportedAccessMode(&self) -> ::windows::core::Result<PhotoImportAccessMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SupportedAccessMode)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhotoImportAccessMode>(result__)
        }
    }
    pub fn CapacityInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CapacityInBytes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn AvailableSpaceInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AvailableSpaceInBytes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    pub fn Refresh(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Refresh)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for PhotoImportStorageMedium {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhotoImportStorageMedium {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhotoImportStorageMedium {}
impl ::core::fmt::Debug for PhotoImportStorageMedium {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoImportStorageMedium").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhotoImportStorageMedium {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Import.PhotoImportStorageMedium;{f2b9b093-fc85-487f-87c2-58d675d05b07})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PhotoImportStorageMedium {
    type Vtable = IPhotoImportStorageMedium_Vtbl;
    const IID: ::windows::core::GUID = <IPhotoImportStorageMedium as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PhotoImportStorageMedium {
    const NAME: &'static str = "Windows.Media.Import.PhotoImportStorageMedium";
}
impl ::core::convert::From<PhotoImportStorageMedium> for ::windows::core::IUnknown {
    fn from(value: PhotoImportStorageMedium) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoImportStorageMedium> for ::windows::core::IUnknown {
    fn from(value: &PhotoImportStorageMedium) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PhotoImportStorageMedium> for &::windows::core::IUnknown {
    fn from(value: &PhotoImportStorageMedium) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PhotoImportStorageMedium> for ::windows::core::IInspectable {
    fn from(value: PhotoImportStorageMedium) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoImportStorageMedium> for ::windows::core::IInspectable {
    fn from(value: &PhotoImportStorageMedium) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PhotoImportStorageMedium> for &::windows::core::IInspectable {
    fn from(value: &PhotoImportStorageMedium) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for PhotoImportStorageMedium {}
unsafe impl ::core::marker::Sync for PhotoImportStorageMedium {}
#[doc = "*Required features: `\"Media_Import\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PhotoImportStorageMediumType(pub i32);
impl PhotoImportStorageMediumType {
    pub const Undefined: Self = Self(0i32);
    pub const Fixed: Self = Self(1i32);
    pub const Removable: Self = Self(2i32);
}
impl ::core::marker::Copy for PhotoImportStorageMediumType {}
impl ::core::clone::Clone for PhotoImportStorageMediumType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhotoImportStorageMediumType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PhotoImportStorageMediumType {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhotoImportStorageMediumType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoImportStorageMediumType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhotoImportStorageMediumType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Import.PhotoImportStorageMediumType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Import\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PhotoImportSubfolderCreationMode(pub i32);
impl PhotoImportSubfolderCreationMode {
    pub const DoNotCreateSubfolders: Self = Self(0i32);
    pub const CreateSubfoldersFromFileDate: Self = Self(1i32);
    pub const CreateSubfoldersFromExifDate: Self = Self(2i32);
    pub const KeepOriginalFolderStructure: Self = Self(3i32);
}
impl ::core::marker::Copy for PhotoImportSubfolderCreationMode {}
impl ::core::clone::Clone for PhotoImportSubfolderCreationMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhotoImportSubfolderCreationMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PhotoImportSubfolderCreationMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhotoImportSubfolderCreationMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoImportSubfolderCreationMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhotoImportSubfolderCreationMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Import.PhotoImportSubfolderCreationMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Import\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PhotoImportSubfolderDateFormat(pub i32);
impl PhotoImportSubfolderDateFormat {
    pub const Year: Self = Self(0i32);
    pub const YearMonth: Self = Self(1i32);
    pub const YearMonthDay: Self = Self(2i32);
}
impl ::core::marker::Copy for PhotoImportSubfolderDateFormat {}
impl ::core::clone::Clone for PhotoImportSubfolderDateFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PhotoImportSubfolderDateFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PhotoImportSubfolderDateFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for PhotoImportSubfolderDateFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoImportSubfolderDateFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhotoImportSubfolderDateFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Media.Import.PhotoImportSubfolderDateFormat;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Media_Import\"`*"]
#[repr(transparent)]
pub struct PhotoImportVideoSegment(::windows::core::IUnknown);
impl PhotoImportVideoSegment {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SizeInBytes(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SizeInBytes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Date(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Date)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    pub fn Sibling(&self) -> ::windows::core::Result<PhotoImportSidecar> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Sibling)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PhotoImportSidecar>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Sidecars(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PhotoImportSidecar>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Sidecars)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<PhotoImportSidecar>>(result__)
        }
    }
}
impl ::core::clone::Clone for PhotoImportVideoSegment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PhotoImportVideoSegment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PhotoImportVideoSegment {}
impl ::core::fmt::Debug for PhotoImportVideoSegment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PhotoImportVideoSegment").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PhotoImportVideoSegment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Import.PhotoImportVideoSegment;{623c0289-321a-41d8-9166-8c62a333276c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PhotoImportVideoSegment {
    type Vtable = IPhotoImportVideoSegment_Vtbl;
    const IID: ::windows::core::GUID = <IPhotoImportVideoSegment as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PhotoImportVideoSegment {
    const NAME: &'static str = "Windows.Media.Import.PhotoImportVideoSegment";
}
impl ::core::convert::From<PhotoImportVideoSegment> for ::windows::core::IUnknown {
    fn from(value: PhotoImportVideoSegment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoImportVideoSegment> for ::windows::core::IUnknown {
    fn from(value: &PhotoImportVideoSegment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PhotoImportVideoSegment> for &::windows::core::IUnknown {
    fn from(value: &PhotoImportVideoSegment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<PhotoImportVideoSegment> for ::windows::core::IInspectable {
    fn from(value: PhotoImportVideoSegment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PhotoImportVideoSegment> for ::windows::core::IInspectable {
    fn from(value: &PhotoImportVideoSegment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&PhotoImportVideoSegment> for &::windows::core::IInspectable {
    fn from(value: &PhotoImportVideoSegment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for PhotoImportVideoSegment {}
unsafe impl ::core::marker::Sync for PhotoImportVideoSegment {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

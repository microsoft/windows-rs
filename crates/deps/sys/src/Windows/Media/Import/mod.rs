#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IPhotoImportDeleteImportedItemsFromSourceResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhotoImportFindItemsResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhotoImportFindItemsResult2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhotoImportImportItemsResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhotoImportItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhotoImportItem2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhotoImportItemImportedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhotoImportManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhotoImportOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhotoImportSelectionChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhotoImportSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhotoImportSession2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhotoImportSidecar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhotoImportSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhotoImportSourceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhotoImportStorageMedium(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhotoImportVideoSegment(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PhotoImportAccessMode(i32);
#[repr(C)]
pub struct PhotoImportConnectionTransport(i32);
#[repr(C)]
pub struct PhotoImportContentType(i32);
#[repr(C)]
pub struct PhotoImportContentTypeFilter(i32);
#[repr(transparent)]
pub struct PhotoImportDeleteImportedItemsFromSourceResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhotoImportFindItemsResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhotoImportImportItemsResult(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PhotoImportImportMode(i32);
#[repr(transparent)]
pub struct PhotoImportItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhotoImportItemImportedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PhotoImportItemSelectionMode(i32);
#[repr(transparent)]
pub struct PhotoImportOperation(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PhotoImportPowerSource(i32);
#[repr(C)]
pub struct PhotoImportProgress(i32);
#[repr(transparent)]
pub struct PhotoImportSelectionChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhotoImportSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhotoImportSidecar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhotoImportSource(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PhotoImportSourceType(i32);
#[repr(C)]
pub struct PhotoImportStage(i32);
#[repr(transparent)]
pub struct PhotoImportStorageMedium(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PhotoImportStorageMediumType(i32);
#[repr(C)]
pub struct PhotoImportSubfolderCreationMode(i32);
#[repr(C)]
pub struct PhotoImportSubfolderDateFormat(i32);
#[repr(transparent)]
pub struct PhotoImportVideoSegment(pub *mut ::core::ffi::c_void);

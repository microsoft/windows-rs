#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "ApplicationModel_DataTransfer_DragDrop")]
pub mod DragDrop;
#[cfg(feature = "ApplicationModel_DataTransfer_ShareTarget")]
pub mod ShareTarget;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ClipboardContentOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ClipboardHistoryChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ClipboardHistoryItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ClipboardHistoryItemsResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ClipboardHistoryItemsResultStatus(pub i32);
impl ClipboardHistoryItemsResultStatus {
    pub const Success: Self = Self(0i32);
    pub const AccessDenied: Self = Self(1i32);
    pub const ClipboardHistoryDisabled: Self = Self(2i32);
}
#[repr(transparent)]
pub struct DataPackage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DataPackageOperation(pub u32);
impl DataPackageOperation {
    pub const None: Self = Self(0u32);
    pub const Copy: Self = Self(1u32);
    pub const Move: Self = Self(2u32);
    pub const Link: Self = Self(4u32);
}
#[repr(transparent)]
pub struct DataPackagePropertySet(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DataPackagePropertySetView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DataPackageView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DataProviderDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DataProviderHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DataProviderRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DataRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DataRequestDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DataRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DataTransferManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IClipboardContentOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IClipboardHistoryChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IClipboardHistoryItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IClipboardHistoryItemsResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IClipboardStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IClipboardStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataPackage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataPackage2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataPackage3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataPackage4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataPackagePropertySet(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataPackagePropertySet2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataPackagePropertySet3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataPackagePropertySet4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataPackagePropertySetView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataPackagePropertySetView2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataPackagePropertySetView3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataPackagePropertySetView4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataPackagePropertySetView5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataPackageView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataPackageView2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataPackageView3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataPackageView4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataProviderDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataProviderRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataRequestDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataTransferManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataTransferManager2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataTransferManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataTransferManagerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataTransferManagerStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHtmlFormatHelperStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOperationCompletedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOperationCompletedEventArgs2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShareCompletedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShareProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShareProviderFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShareProviderOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShareProvidersRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShareTargetInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShareUIOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISharedStorageAccessManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStandardDataFormatsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStandardDataFormatsStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStandardDataFormatsStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITargetApplicationChosenEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct OperationCompletedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SetHistoryItemAsContentStatus(pub i32);
impl SetHistoryItemAsContentStatus {
    pub const Success: Self = Self(0i32);
    pub const AccessDenied: Self = Self(1i32);
    pub const ItemDeleted: Self = Self(2i32);
}
#[repr(transparent)]
pub struct ShareCompletedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ShareProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ShareProviderHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ShareProviderOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ShareProvidersRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ShareTargetInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ShareUIOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ShareUITheme(pub i32);
impl ShareUITheme {
    pub const Default: Self = Self(0i32);
    pub const Light: Self = Self(1i32);
    pub const Dark: Self = Self(2i32);
}
#[repr(transparent)]
pub struct TargetApplicationChosenEventArgs(pub *mut ::core::ffi::c_void);

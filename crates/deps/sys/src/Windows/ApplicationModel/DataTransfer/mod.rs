#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "ApplicationModel_DataTransfer_DragDrop")]
pub mod DragDrop;
#[cfg(feature = "ApplicationModel_DataTransfer_ShareTarget")]
pub mod ShareTarget;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct Clipboard(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ClipboardContentOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ClipboardHistoryChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ClipboardHistoryItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ClipboardHistoryItemsResult(pub *mut ::core::ffi::c_void);
pub struct ClipboardHistoryItemsResultStatus(i32);
#[repr(transparent)]
pub struct DataPackage(pub *mut ::core::ffi::c_void);
pub struct DataPackageOperation(i32);
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
pub struct HtmlFormatHelper(pub *mut ::core::ffi::c_void);
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
pub struct SetHistoryItemAsContentStatus(i32);
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
pub struct ShareUITheme(i32);
#[repr(transparent)]
pub struct SharedStorageAccessManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StandardDataFormats(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TargetApplicationChosenEventArgs(pub *mut ::core::ffi::c_void);

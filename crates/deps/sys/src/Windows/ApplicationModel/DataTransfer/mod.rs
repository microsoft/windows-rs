#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "ApplicationModel_DataTransfer_DragDrop")]
pub mod DragDrop;
#[cfg(feature = "ApplicationModel_DataTransfer_ShareTarget")]
pub mod ShareTarget;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ClipboardContentOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ClipboardContentOptions {}
impl ::core::clone::Clone for ClipboardContentOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ClipboardHistoryChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ClipboardHistoryChangedEventArgs {}
impl ::core::clone::Clone for ClipboardHistoryChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ClipboardHistoryItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ClipboardHistoryItem {}
impl ::core::clone::Clone for ClipboardHistoryItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ClipboardHistoryItemsResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ClipboardHistoryItemsResult {}
impl ::core::clone::Clone for ClipboardHistoryItemsResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ClipboardHistoryItemsResultStatus(pub i32);
impl ClipboardHistoryItemsResultStatus {
    pub const Success: Self = Self(0i32);
    pub const AccessDenied: Self = Self(1i32);
    pub const ClipboardHistoryDisabled: Self = Self(2i32);
}
impl ::core::marker::Copy for ClipboardHistoryItemsResultStatus {}
impl ::core::clone::Clone for ClipboardHistoryItemsResultStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DataPackage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DataPackage {}
impl ::core::clone::Clone for DataPackage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DataPackageOperation(pub u32);
impl DataPackageOperation {
    pub const None: Self = Self(0u32);
    pub const Copy: Self = Self(1u32);
    pub const Move: Self = Self(2u32);
    pub const Link: Self = Self(4u32);
}
impl ::core::marker::Copy for DataPackageOperation {}
impl ::core::clone::Clone for DataPackageOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DataPackagePropertySet(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DataPackagePropertySet {}
impl ::core::clone::Clone for DataPackagePropertySet {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DataPackagePropertySetView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DataPackagePropertySetView {}
impl ::core::clone::Clone for DataPackagePropertySetView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DataPackageView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DataPackageView {}
impl ::core::clone::Clone for DataPackageView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DataProviderDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DataProviderDeferral {}
impl ::core::clone::Clone for DataProviderDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DataProviderHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DataProviderHandler {}
impl ::core::clone::Clone for DataProviderHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DataProviderRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DataProviderRequest {}
impl ::core::clone::Clone for DataProviderRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DataRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DataRequest {}
impl ::core::clone::Clone for DataRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DataRequestDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DataRequestDeferral {}
impl ::core::clone::Clone for DataRequestDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DataRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DataRequestedEventArgs {}
impl ::core::clone::Clone for DataRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DataTransferManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DataTransferManager {}
impl ::core::clone::Clone for DataTransferManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IClipboardContentOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IClipboardContentOptions {}
impl ::core::clone::Clone for IClipboardContentOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IClipboardHistoryChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IClipboardHistoryChangedEventArgs {}
impl ::core::clone::Clone for IClipboardHistoryChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IClipboardHistoryItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IClipboardHistoryItem {}
impl ::core::clone::Clone for IClipboardHistoryItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IClipboardHistoryItemsResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IClipboardHistoryItemsResult {}
impl ::core::clone::Clone for IClipboardHistoryItemsResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IClipboardStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IClipboardStatics {}
impl ::core::clone::Clone for IClipboardStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IClipboardStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IClipboardStatics2 {}
impl ::core::clone::Clone for IClipboardStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDataPackage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDataPackage {}
impl ::core::clone::Clone for IDataPackage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDataPackage2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDataPackage2 {}
impl ::core::clone::Clone for IDataPackage2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDataPackage3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDataPackage3 {}
impl ::core::clone::Clone for IDataPackage3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDataPackage4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDataPackage4 {}
impl ::core::clone::Clone for IDataPackage4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDataPackagePropertySet(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDataPackagePropertySet {}
impl ::core::clone::Clone for IDataPackagePropertySet {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDataPackagePropertySet2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDataPackagePropertySet2 {}
impl ::core::clone::Clone for IDataPackagePropertySet2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDataPackagePropertySet3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDataPackagePropertySet3 {}
impl ::core::clone::Clone for IDataPackagePropertySet3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDataPackagePropertySet4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDataPackagePropertySet4 {}
impl ::core::clone::Clone for IDataPackagePropertySet4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDataPackagePropertySetView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDataPackagePropertySetView {}
impl ::core::clone::Clone for IDataPackagePropertySetView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDataPackagePropertySetView2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDataPackagePropertySetView2 {}
impl ::core::clone::Clone for IDataPackagePropertySetView2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDataPackagePropertySetView3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDataPackagePropertySetView3 {}
impl ::core::clone::Clone for IDataPackagePropertySetView3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDataPackagePropertySetView4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDataPackagePropertySetView4 {}
impl ::core::clone::Clone for IDataPackagePropertySetView4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDataPackagePropertySetView5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDataPackagePropertySetView5 {}
impl ::core::clone::Clone for IDataPackagePropertySetView5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDataPackageView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDataPackageView {}
impl ::core::clone::Clone for IDataPackageView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDataPackageView2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDataPackageView2 {}
impl ::core::clone::Clone for IDataPackageView2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDataPackageView3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDataPackageView3 {}
impl ::core::clone::Clone for IDataPackageView3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDataPackageView4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDataPackageView4 {}
impl ::core::clone::Clone for IDataPackageView4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDataProviderDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDataProviderDeferral {}
impl ::core::clone::Clone for IDataProviderDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDataProviderRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDataProviderRequest {}
impl ::core::clone::Clone for IDataProviderRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDataRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDataRequest {}
impl ::core::clone::Clone for IDataRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDataRequestDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDataRequestDeferral {}
impl ::core::clone::Clone for IDataRequestDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDataRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDataRequestedEventArgs {}
impl ::core::clone::Clone for IDataRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDataTransferManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDataTransferManager {}
impl ::core::clone::Clone for IDataTransferManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDataTransferManager2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDataTransferManager2 {}
impl ::core::clone::Clone for IDataTransferManager2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDataTransferManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDataTransferManagerStatics {}
impl ::core::clone::Clone for IDataTransferManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDataTransferManagerStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDataTransferManagerStatics2 {}
impl ::core::clone::Clone for IDataTransferManagerStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDataTransferManagerStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDataTransferManagerStatics3 {}
impl ::core::clone::Clone for IDataTransferManagerStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHtmlFormatHelperStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHtmlFormatHelperStatics {}
impl ::core::clone::Clone for IHtmlFormatHelperStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOperationCompletedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOperationCompletedEventArgs {}
impl ::core::clone::Clone for IOperationCompletedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOperationCompletedEventArgs2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOperationCompletedEventArgs2 {}
impl ::core::clone::Clone for IOperationCompletedEventArgs2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IShareCompletedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IShareCompletedEventArgs {}
impl ::core::clone::Clone for IShareCompletedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IShareProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IShareProvider {}
impl ::core::clone::Clone for IShareProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IShareProviderFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IShareProviderFactory {}
impl ::core::clone::Clone for IShareProviderFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IShareProviderOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IShareProviderOperation {}
impl ::core::clone::Clone for IShareProviderOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IShareProvidersRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IShareProvidersRequestedEventArgs {}
impl ::core::clone::Clone for IShareProvidersRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IShareTargetInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IShareTargetInfo {}
impl ::core::clone::Clone for IShareTargetInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IShareUIOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IShareUIOptions {}
impl ::core::clone::Clone for IShareUIOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISharedStorageAccessManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISharedStorageAccessManagerStatics {}
impl ::core::clone::Clone for ISharedStorageAccessManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStandardDataFormatsStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStandardDataFormatsStatics {}
impl ::core::clone::Clone for IStandardDataFormatsStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStandardDataFormatsStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStandardDataFormatsStatics2 {}
impl ::core::clone::Clone for IStandardDataFormatsStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStandardDataFormatsStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStandardDataFormatsStatics3 {}
impl ::core::clone::Clone for IStandardDataFormatsStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITargetApplicationChosenEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITargetApplicationChosenEventArgs {}
impl ::core::clone::Clone for ITargetApplicationChosenEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct OperationCompletedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for OperationCompletedEventArgs {}
impl ::core::clone::Clone for OperationCompletedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SetHistoryItemAsContentStatus(pub i32);
impl SetHistoryItemAsContentStatus {
    pub const Success: Self = Self(0i32);
    pub const AccessDenied: Self = Self(1i32);
    pub const ItemDeleted: Self = Self(2i32);
}
impl ::core::marker::Copy for SetHistoryItemAsContentStatus {}
impl ::core::clone::Clone for SetHistoryItemAsContentStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ShareCompletedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ShareCompletedEventArgs {}
impl ::core::clone::Clone for ShareCompletedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ShareProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ShareProvider {}
impl ::core::clone::Clone for ShareProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ShareProviderHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ShareProviderHandler {}
impl ::core::clone::Clone for ShareProviderHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ShareProviderOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ShareProviderOperation {}
impl ::core::clone::Clone for ShareProviderOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ShareProvidersRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ShareProvidersRequestedEventArgs {}
impl ::core::clone::Clone for ShareProvidersRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ShareTargetInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ShareTargetInfo {}
impl ::core::clone::Clone for ShareTargetInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ShareUIOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ShareUIOptions {}
impl ::core::clone::Clone for ShareUIOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ShareUITheme(pub i32);
impl ShareUITheme {
    pub const Default: Self = Self(0i32);
    pub const Light: Self = Self(1i32);
    pub const Dark: Self = Self(2i32);
}
impl ::core::marker::Copy for ShareUITheme {}
impl ::core::clone::Clone for ShareUITheme {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TargetApplicationChosenEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TargetApplicationChosenEventArgs {}
impl ::core::clone::Clone for TargetApplicationChosenEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}

#[cfg(feature = "implement_exclusive")]
pub trait IClipboardContentOptionsImpl: Sized {
    fn IsRoamable(&self) -> ::windows::core::Result<bool>;
    fn SetIsRoamable(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsAllowedInHistory(&self) -> ::windows::core::Result<bool>;
    fn SetIsAllowedInHistory(&self, value: bool) -> ::windows::core::Result<()>;
    fn RoamingFormats(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn HistoryFormats(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IClipboardHistoryChangedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IClipboardHistoryItemImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn Content(&self) -> ::windows::core::Result<DataPackageView>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IClipboardHistoryItemsResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<ClipboardHistoryItemsResultStatus>;
    fn Items(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ClipboardHistoryItem>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IClipboardStaticsImpl: Sized {
    fn GetContent(&self) -> ::windows::core::Result<DataPackageView>;
    fn SetContent(&self, content: &::core::option::Option<DataPackage>) -> ::windows::core::Result<()>;
    fn Flush(&self) -> ::windows::core::Result<()>;
    fn Clear(&self) -> ::windows::core::Result<()>;
    fn ContentChanged(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveContentChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IClipboardStatics2Impl: Sized {
    fn GetHistoryItemsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ClipboardHistoryItemsResult>>;
    fn ClearHistory(&self) -> ::windows::core::Result<bool>;
    fn DeleteItemFromHistory(&self, item: &::core::option::Option<ClipboardHistoryItem>) -> ::windows::core::Result<bool>;
    fn SetHistoryItemAsContent(&self, item: &::core::option::Option<ClipboardHistoryItem>) -> ::windows::core::Result<SetHistoryItemAsContentStatus>;
    fn IsHistoryEnabled(&self) -> ::windows::core::Result<bool>;
    fn IsRoamingEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetContentWithOptions(&self, content: &::core::option::Option<DataPackage>, options: &::core::option::Option<ClipboardContentOptions>) -> ::windows::core::Result<bool>;
    fn HistoryChanged(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<ClipboardHistoryChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveHistoryChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RoamingEnabledChanged(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRoamingEnabledChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn HistoryEnabledChanged(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveHistoryEnabledChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataPackageImpl: Sized {
    fn GetView(&self) -> ::windows::core::Result<DataPackageView>;
    fn Properties(&self) -> ::windows::core::Result<DataPackagePropertySet>;
    fn RequestedOperation(&self) -> ::windows::core::Result<DataPackageOperation>;
    fn SetRequestedOperation(&self, value: DataPackageOperation) -> ::windows::core::Result<()>;
    fn OperationCompleted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DataPackage, OperationCompletedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveOperationCompleted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Destroyed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DataPackage, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDestroyed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SetData(&self, formatid: &::windows::core::HSTRING, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn SetDataProvider(&self, formatid: &::windows::core::HSTRING, delayrenderer: &::core::option::Option<DataProviderHandler>) -> ::windows::core::Result<()>;
    fn SetText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn SetHtmlFormat(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ResourceMap(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, super::super::Storage::Streams::RandomAccessStreamReference>>;
    fn SetRtf(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetBitmap(&self, value: &::core::option::Option<super::super::Storage::Streams::RandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn SetStorageItemsReadOnly(&self, value: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Storage::IStorageItem>>) -> ::windows::core::Result<()>;
    fn SetStorageItems(&self, value: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Storage::IStorageItem>>, readonly: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataPackage2Impl: Sized {
    fn SetApplicationLink(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn SetWebLink(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataPackage3Impl: Sized {
    fn ShareCompleted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DataPackage, ShareCompletedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveShareCompleted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataPackage4Impl: Sized {
    fn ShareCanceled(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DataPackage, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveShareCanceled(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IDataPackagePropertySetImpl: Sized + IIterableImpl<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, ::windows::core::IInspectable>> + IMapImpl<::windows::core::HSTRING, ::windows::core::IInspectable> {
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Thumbnail(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn SetThumbnail(&self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn FileTypes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn ApplicationName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetApplicationName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ApplicationListingUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetApplicationListingUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataPackagePropertySet2Impl: Sized {
    fn ContentSourceWebLink(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetContentSourceWebLink(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn ContentSourceApplicationLink(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetContentSourceApplicationLink(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn PackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPackageFamilyName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Square30x30Logo(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn SetSquare30x30Logo(&self, value: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamReference>) -> ::windows::core::Result<()>;
    fn LogoBackgroundColor(&self) -> ::windows::core::Result<super::super::UI::Color>;
    fn SetLogoBackgroundColor(&self, value: &super::super::UI::Color) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataPackagePropertySet3Impl: Sized {
    fn EnterpriseId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetEnterpriseId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataPackagePropertySet4Impl: Sized {
    fn ContentSourceUserActivityJson(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetContentSourceUserActivityJson(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataPackagePropertySetViewImpl: Sized {
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Thumbnail(&self) -> ::windows::core::Result<super::super::Storage::Streams::RandomAccessStreamReference>;
    fn FileTypes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn ApplicationName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ApplicationListingUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataPackagePropertySetView2Impl: Sized {
    fn PackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ContentSourceWebLink(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn ContentSourceApplicationLink(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn Square30x30Logo(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn LogoBackgroundColor(&self) -> ::windows::core::Result<super::super::UI::Color>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataPackagePropertySetView3Impl: Sized {
    fn EnterpriseId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataPackagePropertySetView4Impl: Sized {
    fn ContentSourceUserActivityJson(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataPackagePropertySetView5Impl: Sized {
    fn IsFromRoamingClipboard(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataPackageViewImpl: Sized {
    fn Properties(&self) -> ::windows::core::Result<DataPackagePropertySetView>;
    fn RequestedOperation(&self) -> ::windows::core::Result<DataPackageOperation>;
    fn ReportOperationCompleted(&self, value: DataPackageOperation) -> ::windows::core::Result<()>;
    fn AvailableFormats(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn Contains(&self, formatid: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn GetDataAsync(&self, formatid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::IInspectable>>;
    fn GetTextAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetCustomTextAsync(&self, formatid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetUriAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Uri>>;
    fn GetHtmlFormatAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetResourceMapAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, super::super::Storage::Streams::RandomAccessStreamReference>>>;
    fn GetRtfAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetBitmapAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::RandomAccessStreamReference>>;
    fn GetStorageItemsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataPackageView2Impl: Sized {
    fn GetApplicationLinkAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Uri>>;
    fn GetWebLinkAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Uri>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataPackageView3Impl: Sized {
    fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Security::EnterpriseData::ProtectionPolicyEvaluationResult>>;
    fn RequestAccessWithEnterpriseIdAsync(&self, enterpriseid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Security::EnterpriseData::ProtectionPolicyEvaluationResult>>;
    fn UnlockAndAssumeEnterpriseIdentity(&self) -> ::windows::core::Result<super::super::Security::EnterpriseData::ProtectionPolicyEvaluationResult>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataPackageView4Impl: Sized {
    fn SetAcceptedFormatId(&self, formatid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataProviderDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataProviderRequestImpl: Sized {
    fn FormatId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Deadline(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn GetDeferral(&self) -> ::windows::core::Result<DataProviderDeferral>;
    fn SetData(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataRequestImpl: Sized {
    fn Data(&self) -> ::windows::core::Result<DataPackage>;
    fn SetData(&self, value: &::core::option::Option<DataPackage>) -> ::windows::core::Result<()>;
    fn Deadline(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn FailWithDisplayText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<DataRequestDeferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataRequestDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataRequestedEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<DataRequest>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataTransferManagerImpl: Sized {
    fn DataRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DataTransferManager, DataRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDataRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TargetApplicationChosen(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DataTransferManager, TargetApplicationChosenEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTargetApplicationChosen(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataTransferManager2Impl: Sized {
    fn ShareProvidersRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DataTransferManager, ShareProvidersRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveShareProvidersRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataTransferManagerStaticsImpl: Sized {
    fn ShowShareUI(&self) -> ::windows::core::Result<()>;
    fn GetForCurrentView(&self) -> ::windows::core::Result<DataTransferManager>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataTransferManagerStatics2Impl: Sized {
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataTransferManagerStatics3Impl: Sized {
    fn ShowShareUIWithOptions(&self, options: &::core::option::Option<ShareUIOptions>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHtmlFormatHelperStaticsImpl: Sized {
    fn GetStaticFragment(&self, htmlformat: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CreateHtmlFormat(&self, htmlfragment: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IOperationCompletedEventArgsImpl: Sized {
    fn Operation(&self) -> ::windows::core::Result<DataPackageOperation>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IOperationCompletedEventArgs2Impl: Sized {
    fn AcceptedFormatId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IShareCompletedEventArgsImpl: Sized {
    fn ShareTarget(&self) -> ::windows::core::Result<ShareTargetInfo>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IShareProviderImpl: Sized {
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayIcon(&self) -> ::windows::core::Result<super::super::Storage::Streams::RandomAccessStreamReference>;
    fn BackgroundColor(&self) -> ::windows::core::Result<super::super::UI::Color>;
    fn Tag(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetTag(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IShareProviderFactoryImpl: Sized {
    fn Create(&self, title: &::windows::core::HSTRING, displayicon: &::core::option::Option<super::super::Storage::Streams::RandomAccessStreamReference>, backgroundcolor: &super::super::UI::Color, handler: &::core::option::Option<ShareProviderHandler>) -> ::windows::core::Result<ShareProvider>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IShareProviderOperationImpl: Sized {
    fn Data(&self) -> ::windows::core::Result<DataPackageView>;
    fn Provider(&self) -> ::windows::core::Result<ShareProvider>;
    fn ReportCompleted(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IShareProvidersRequestedEventArgsImpl: Sized {
    fn Providers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ShareProvider>>;
    fn Data(&self) -> ::windows::core::Result<DataPackageView>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IShareTargetInfoImpl: Sized {
    fn AppUserModelId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ShareProvider(&self) -> ::windows::core::Result<ShareProvider>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IShareUIOptionsImpl: Sized {
    fn Theme(&self) -> ::windows::core::Result<ShareUITheme>;
    fn SetTheme(&self, value: ShareUITheme) -> ::windows::core::Result<()>;
    fn SelectionRect(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::Rect>>;
    fn SetSelectionRect(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::Rect>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISharedStorageAccessManagerStaticsImpl: Sized {
    fn AddFile(&self, file: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RedeemTokenForFileAsync(&self, token: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::StorageFile>>;
    fn RemoveFile(&self, token: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStandardDataFormatsStaticsImpl: Sized {
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Uri(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Html(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Rtf(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Bitmap(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn StorageItems(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStandardDataFormatsStatics2Impl: Sized {
    fn WebLink(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ApplicationLink(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStandardDataFormatsStatics3Impl: Sized {
    fn UserActivityJsonArray(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITargetApplicationChosenEventArgsImpl: Sized {
    fn ApplicationName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}

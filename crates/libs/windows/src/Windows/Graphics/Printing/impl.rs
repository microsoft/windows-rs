pub trait IPrintDocumentSourceImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintManagerImpl: Sized {
    fn PrintTaskRequested(&self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PrintManager, PrintTaskRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePrintTaskRequested(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintManagerStaticImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<PrintManager>;
    fn ShowPrintUIAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintManagerStatic2Impl: Sized {
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintPageInfoImpl: Sized {
    fn SetMediaSize(&self, value: PrintMediaSize) -> ::windows::core::Result<()>;
    fn MediaSize(&self) -> ::windows::core::Result<PrintMediaSize>;
    fn SetPageSize(&self, value: &super::super::Foundation::Size) -> ::windows::core::Result<()>;
    fn PageSize(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn SetDpiX(&self, value: u32) -> ::windows::core::Result<()>;
    fn DpiX(&self) -> ::windows::core::Result<u32>;
    fn SetDpiY(&self, value: u32) -> ::windows::core::Result<()>;
    fn DpiY(&self) -> ::windows::core::Result<u32>;
    fn SetOrientation(&self, value: PrintOrientation) -> ::windows::core::Result<()>;
    fn Orientation(&self) -> ::windows::core::Result<PrintOrientation>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintPageRangeImpl: Sized {
    fn FirstPageNumber(&self) -> ::windows::core::Result<i32>;
    fn LastPageNumber(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintPageRangeFactoryImpl: Sized {
    fn Create(&self, firstpage: i32, lastpage: i32) -> ::windows::core::Result<PrintPageRange>;
    fn CreateWithSinglePage(&self, page: i32) -> ::windows::core::Result<PrintPageRange>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintPageRangeOptionsImpl: Sized {
    fn SetAllowAllPages(&self, value: bool) -> ::windows::core::Result<()>;
    fn AllowAllPages(&self) -> ::windows::core::Result<bool>;
    fn SetAllowCurrentPage(&self, value: bool) -> ::windows::core::Result<()>;
    fn AllowCurrentPage(&self) -> ::windows::core::Result<bool>;
    fn SetAllowCustomSetOfPages(&self, value: bool) -> ::windows::core::Result<()>;
    fn AllowCustomSetOfPages(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintTaskImpl: Sized {
    fn Properties(&self) -> ::windows::core::Result<super::super::ApplicationModel::DataTransfer::DataPackagePropertySet>;
    fn Source(&self) -> ::windows::core::Result<IPrintDocumentSource>;
    fn Options(&self) -> ::windows::core::Result<PrintTaskOptions>;
    fn Previewing(&self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PrintTask, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePreviewing(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Submitting(&self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PrintTask, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSubmitting(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Progressing(&self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PrintTask, PrintTaskProgressingEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveProgressing(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Completed(&self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PrintTask, PrintTaskCompletedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCompleted(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintTask2Impl: Sized {
    fn SetIsPreviewEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsPreviewEnabled(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintTaskCompletedEventArgsImpl: Sized {
    fn Completion(&self) -> ::windows::core::Result<PrintTaskCompletion>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintTaskOptionsImpl: Sized {
    fn SetBordering(&self, value: PrintBordering) -> ::windows::core::Result<()>;
    fn Bordering(&self) -> ::windows::core::Result<PrintBordering>;
    fn GetPagePrintTicket(&self, printpageinfo: &::core::option::Option<PrintPageInfo>) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStream>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintTaskOptions2Impl: Sized {
    fn PageRangeOptions(&self) -> ::windows::core::Result<PrintPageRangeOptions>;
    fn CustomPageRanges(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<PrintPageRange>>;
}
pub trait IPrintTaskOptionsCoreImpl: Sized {
    fn GetPageDescription(&self, jobpagenumber: u32) -> ::windows::core::Result<PrintPageDescription>;
}
pub trait IPrintTaskOptionsCorePropertiesImpl: Sized {
    fn SetMediaSize(&self, value: PrintMediaSize) -> ::windows::core::Result<()>;
    fn MediaSize(&self) -> ::windows::core::Result<PrintMediaSize>;
    fn SetMediaType(&self, value: PrintMediaType) -> ::windows::core::Result<()>;
    fn MediaType(&self) -> ::windows::core::Result<PrintMediaType>;
    fn SetOrientation(&self, value: PrintOrientation) -> ::windows::core::Result<()>;
    fn Orientation(&self) -> ::windows::core::Result<PrintOrientation>;
    fn SetPrintQuality(&self, value: PrintQuality) -> ::windows::core::Result<()>;
    fn PrintQuality(&self) -> ::windows::core::Result<PrintQuality>;
    fn SetColorMode(&self, value: PrintColorMode) -> ::windows::core::Result<()>;
    fn ColorMode(&self) -> ::windows::core::Result<PrintColorMode>;
    fn SetDuplex(&self, value: PrintDuplex) -> ::windows::core::Result<()>;
    fn Duplex(&self) -> ::windows::core::Result<PrintDuplex>;
    fn SetCollation(&self, value: PrintCollation) -> ::windows::core::Result<()>;
    fn Collation(&self) -> ::windows::core::Result<PrintCollation>;
    fn SetStaple(&self, value: PrintStaple) -> ::windows::core::Result<()>;
    fn Staple(&self) -> ::windows::core::Result<PrintStaple>;
    fn SetHolePunch(&self, value: PrintHolePunch) -> ::windows::core::Result<()>;
    fn HolePunch(&self) -> ::windows::core::Result<PrintHolePunch>;
    fn SetBinding(&self, value: PrintBinding) -> ::windows::core::Result<()>;
    fn Binding(&self) -> ::windows::core::Result<PrintBinding>;
    fn MinCopies(&self) -> ::windows::core::Result<u32>;
    fn MaxCopies(&self) -> ::windows::core::Result<u32>;
    fn SetNumberOfCopies(&self, value: u32) -> ::windows::core::Result<()>;
    fn NumberOfCopies(&self) -> ::windows::core::Result<u32>;
}
pub trait IPrintTaskOptionsCoreUIConfigurationImpl: Sized {
    fn DisplayedOptions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintTaskProgressingEventArgsImpl: Sized {
    fn DocumentPageCount(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintTaskRequestImpl: Sized {
    fn Deadline(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn CreatePrintTask(&self, title: &::windows::core::HSTRING, handler: &::core::option::Option<PrintTaskSourceRequestedHandler>) -> ::windows::core::Result<PrintTask>;
    fn GetDeferral(&self) -> ::windows::core::Result<PrintTaskRequestedDeferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintTaskRequestedDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintTaskRequestedEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<PrintTaskRequest>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintTaskSourceRequestedArgsImpl: Sized {
    fn Deadline(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn SetSource(&self, source: &::core::option::Option<IPrintDocumentSource>) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<PrintTaskSourceRequestedDeferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintTaskSourceRequestedDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintTaskTargetDeviceSupportImpl: Sized {
    fn SetIsPrinterTargetEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsPrinterTargetEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIs3DManufacturingTargetEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn Is3DManufacturingTargetEnabled(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStandardPrintTaskOptionsStaticImpl: Sized {
    fn MediaSize(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MediaType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Orientation(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PrintQuality(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ColorMode(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Duplex(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Collation(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Staple(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HolePunch(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Binding(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Copies(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NUp(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn InputBin(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStandardPrintTaskOptionsStatic2Impl: Sized {
    fn Bordering(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStandardPrintTaskOptionsStatic3Impl: Sized {
    fn CustomPageRanges(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}

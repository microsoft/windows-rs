#[cfg(feature = "implement_exclusive")]
pub trait IAddPagesEventArgsImpl: Sized {
    fn PrintTaskOptions(&self) -> ::windows::core::Result<super::super::super::Graphics::Printing::PrintTaskOptions>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGetPreviewPageEventArgsImpl: Sized {
    fn PageNumber(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPaginateEventArgsImpl: Sized {
    fn PrintTaskOptions(&self) -> ::windows::core::Result<super::super::super::Graphics::Printing::PrintTaskOptions>;
    fn CurrentPreviewPageNumber(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintDocumentImpl: Sized {
    fn DocumentSource(&self) -> ::windows::core::Result<super::super::super::Graphics::Printing::IPrintDocumentSource>;
    fn Paginate(&self, handler: &::core::option::Option<PaginateEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePaginate(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetPreviewPage(&self, handler: &::core::option::Option<GetPreviewPageEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveGetPreviewPage(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AddPages(&self, handler: &::core::option::Option<AddPagesEventHandler>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveAddPages(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AddPage(&self, pagevisual: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn AddPagesComplete(&self) -> ::windows::core::Result<()>;
    fn SetPreviewPageCount(&self, count: i32, r#type: PreviewPageCountType) -> ::windows::core::Result<()>;
    fn SetPreviewPage(&self, pagenumber: i32, pagevisual: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn InvalidatePreview(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintDocumentFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<PrintDocument>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintDocumentStaticsImpl: Sized {
    fn DocumentSourceProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
}

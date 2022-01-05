#[cfg(feature = "implement_exclusive")]
pub trait IIppAttributeErrorImpl: Sized {
    fn Reason(&self) -> ::windows::core::Result<IppAttributeErrorReason>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn GetUnsupportedValues(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<IppAttributeValue>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IIppAttributeValueImpl: Sized {
    fn Kind(&self) -> ::windows::core::Result<IppAttributeValueKind>;
    fn GetIntegerArray(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<i32>>;
    fn GetBooleanArray(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<bool>>;
    fn GetEnumArray(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<i32>>;
    fn GetOctetStringArray(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Storage::Streams::IBuffer>>;
    fn GetDateTimeArray(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::DateTime>>;
    fn GetResolutionArray(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<IppResolution>>;
    fn GetRangeOfIntegerArray(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<IppIntegerRange>>;
    fn GetCollectionArray(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, IppAttributeValue>>>;
    fn GetTextWithLanguageArray(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<IppTextWithLanguage>>;
    fn GetNameWithLanguageArray(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<IppTextWithLanguage>>;
    fn GetTextWithoutLanguageArray(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn GetNameWithoutLanguageArray(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn GetKeywordArray(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn GetUriArray(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>;
    fn GetUriSchemaArray(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn GetCharsetArray(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn GetNaturalLanguageArray(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn GetMimeMediaTypeArray(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IIppAttributeValueStaticsImpl: Sized {
    fn CreateUnsupported(&self) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateUnknown(&self) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateNoValue(&self) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateInteger(&self, value: i32) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateIntegerArray(&self, values: &::core::option::Option<super::super::Foundation::Collections::IIterable<i32>>) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateBoolean(&self, value: bool) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateBooleanArray(&self, values: &::core::option::Option<super::super::Foundation::Collections::IIterable<bool>>) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateEnum(&self, value: i32) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateEnumArray(&self, values: &::core::option::Option<super::super::Foundation::Collections::IIterable<i32>>) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateOctetString(&self, value: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateOctetStringArray(&self, values: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Storage::Streams::IBuffer>>) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateDateTime(&self, value: &super::super::Foundation::DateTime) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateDateTimeArray(&self, values: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::DateTime>>) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateResolution(&self, value: &::core::option::Option<IppResolution>) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateResolutionArray(&self, values: &::core::option::Option<super::super::Foundation::Collections::IIterable<IppResolution>>) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateRangeOfInteger(&self, value: &::core::option::Option<IppIntegerRange>) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateRangeOfIntegerArray(&self, values: &::core::option::Option<super::super::Foundation::Collections::IIterable<IppIntegerRange>>) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateCollection(&self, memberattributes: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, IppAttributeValue>>>) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateCollectionArray(&self, memberattributesarray: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, IppAttributeValue>>>>) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateTextWithLanguage(&self, value: &::core::option::Option<IppTextWithLanguage>) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateTextWithLanguageArray(&self, values: &::core::option::Option<super::super::Foundation::Collections::IIterable<IppTextWithLanguage>>) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateNameWithLanguage(&self, value: &::core::option::Option<IppTextWithLanguage>) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateNameWithLanguageArray(&self, values: &::core::option::Option<super::super::Foundation::Collections::IIterable<IppTextWithLanguage>>) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateTextWithoutLanguage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateTextWithoutLanguageArray(&self, values: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateNameWithoutLanguage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateNameWithoutLanguageArray(&self, values: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateKeyword(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateKeywordArray(&self, values: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateUriArray(&self, values: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Uri>>) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateUriSchema(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateUriSchemaArray(&self, values: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateCharset(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateCharsetArray(&self, values: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateNaturalLanguage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateNaturalLanguageArray(&self, values: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateMimeMedia(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<IppAttributeValue>;
    fn CreateMimeMediaArray(&self, values: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<IppAttributeValue>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IIppIntegerRangeImpl: Sized {
    fn Start(&self) -> ::windows::core::Result<i32>;
    fn End(&self) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IIppIntegerRangeFactoryImpl: Sized {
    fn CreateInstance(&self, start: i32, end: i32) -> ::windows::core::Result<IppIntegerRange>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IIppPrintDeviceImpl: Sized {
    fn PrinterName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PrinterUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn GetPrinterAttributesAsBuffer(&self, attributenames: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
    fn GetPrinterAttributes(&self, attributenames: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, IppAttributeValue>>;
    fn SetPrinterAttributesFromBuffer(&self, printerattributesbuffer: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<IppSetAttributesResult>;
    fn SetPrinterAttributes(&self, printerattributes: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, IppAttributeValue>>>) -> ::windows::core::Result<IppSetAttributesResult>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IIppResolutionImpl: Sized {
    fn Width(&self) -> ::windows::core::Result<i32>;
    fn Height(&self) -> ::windows::core::Result<i32>;
    fn Unit(&self) -> ::windows::core::Result<IppResolutionUnit>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IIppResolutionFactoryImpl: Sized {
    fn CreateInstance(&self, width: i32, height: i32, unit: IppResolutionUnit) -> ::windows::core::Result<IppResolution>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IIppSetAttributesResultImpl: Sized {
    fn Succeeded(&self) -> ::windows::core::Result<bool>;
    fn AttributeErrors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, IppAttributeError>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IIppTextWithLanguageImpl: Sized {
    fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IIppTextWithLanguageFactoryImpl: Sized {
    fn CreateInstance(&self, language: &::windows::core::HSTRING, text: &::windows::core::HSTRING) -> ::windows::core::Result<IppTextWithLanguage>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrint3DDeviceImpl: Sized {
    fn PrintSchema(&self) -> ::windows::core::Result<PrintSchema>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrint3DDeviceStaticsImpl: Sized {
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Print3DDevice>>;
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPrintSchemaImpl: Sized {
    fn GetDefaultPrintTicketAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStreamWithContentType>>;
    fn GetCapabilitiesAsync(&self, constrainticket: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamWithContentType>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStreamWithContentType>>;
    fn MergeAndValidateWithDefaultPrintTicketAsync(&self, deltaticket: &::core::option::Option<super::super::Storage::Streams::IRandomAccessStreamWithContentType>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStreamWithContentType>>;
}

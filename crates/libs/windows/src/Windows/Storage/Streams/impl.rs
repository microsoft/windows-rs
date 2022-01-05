pub trait IBufferImpl: Sized {
    fn Capacity(&self) -> ::windows::core::Result<u32>;
    fn Length(&self) -> ::windows::core::Result<u32>;
    fn SetLength(&self, value: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBufferFactoryImpl: Sized {
    fn Create(&self, capacity: u32) -> ::windows::core::Result<Buffer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBufferStaticsImpl: Sized {
    fn CreateCopyFromMemoryBuffer(&self, input: &::core::option::Option<super::super::Foundation::IMemoryBuffer>) -> ::windows::core::Result<Buffer>;
    fn CreateMemoryBufferOverIBuffer(&self, input: &::core::option::Option<IBuffer>) -> ::windows::core::Result<super::super::Foundation::MemoryBuffer>;
}
pub trait IContentTypeProviderImpl: Sized {
    fn ContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
pub trait IDataReaderImpl: Sized {
    fn UnconsumedBufferLength(&self) -> ::windows::core::Result<u32>;
    fn UnicodeEncoding(&self) -> ::windows::core::Result<UnicodeEncoding>;
    fn SetUnicodeEncoding(&self, value: UnicodeEncoding) -> ::windows::core::Result<()>;
    fn ByteOrder(&self) -> ::windows::core::Result<ByteOrder>;
    fn SetByteOrder(&self, value: ByteOrder) -> ::windows::core::Result<()>;
    fn InputStreamOptions(&self) -> ::windows::core::Result<InputStreamOptions>;
    fn SetInputStreamOptions(&self, value: InputStreamOptions) -> ::windows::core::Result<()>;
    fn ReadByte(&self) -> ::windows::core::Result<u8>;
    fn ReadBytes(&self, value: &mut [<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn ReadBuffer(&self, length: u32) -> ::windows::core::Result<IBuffer>;
    fn ReadBoolean(&self) -> ::windows::core::Result<bool>;
    fn ReadGuid(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ReadInt16(&self) -> ::windows::core::Result<i16>;
    fn ReadInt32(&self) -> ::windows::core::Result<i32>;
    fn ReadInt64(&self) -> ::windows::core::Result<i64>;
    fn ReadUInt16(&self) -> ::windows::core::Result<u16>;
    fn ReadUInt32(&self) -> ::windows::core::Result<u32>;
    fn ReadUInt64(&self) -> ::windows::core::Result<u64>;
    fn ReadSingle(&self) -> ::windows::core::Result<f32>;
    fn ReadDouble(&self) -> ::windows::core::Result<f64>;
    fn ReadString(&self, codeunitcount: u32) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReadDateTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn ReadTimeSpan(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn LoadAsync(&self, count: u32) -> ::windows::core::Result<DataReaderLoadOperation>;
    fn DetachBuffer(&self) -> ::windows::core::Result<IBuffer>;
    fn DetachStream(&self) -> ::windows::core::Result<IInputStream>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataReaderFactoryImpl: Sized {
    fn CreateDataReader(&self, inputstream: &::core::option::Option<IInputStream>) -> ::windows::core::Result<DataReader>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataReaderStaticsImpl: Sized {
    fn FromBuffer(&self, buffer: &::core::option::Option<IBuffer>) -> ::windows::core::Result<DataReader>;
}
pub trait IDataWriterImpl: Sized {
    fn UnstoredBufferLength(&self) -> ::windows::core::Result<u32>;
    fn UnicodeEncoding(&self) -> ::windows::core::Result<UnicodeEncoding>;
    fn SetUnicodeEncoding(&self, value: UnicodeEncoding) -> ::windows::core::Result<()>;
    fn ByteOrder(&self) -> ::windows::core::Result<ByteOrder>;
    fn SetByteOrder(&self, value: ByteOrder) -> ::windows::core::Result<()>;
    fn WriteByte(&self, value: u8) -> ::windows::core::Result<()>;
    fn WriteBytes(&self, value: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn WriteBuffer(&self, buffer: &::core::option::Option<IBuffer>) -> ::windows::core::Result<()>;
    fn WriteBufferRange(&self, buffer: &::core::option::Option<IBuffer>, start: u32, count: u32) -> ::windows::core::Result<()>;
    fn WriteBoolean(&self, value: bool) -> ::windows::core::Result<()>;
    fn WriteGuid(&self, value: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn WriteInt16(&self, value: i16) -> ::windows::core::Result<()>;
    fn WriteInt32(&self, value: i32) -> ::windows::core::Result<()>;
    fn WriteInt64(&self, value: i64) -> ::windows::core::Result<()>;
    fn WriteUInt16(&self, value: u16) -> ::windows::core::Result<()>;
    fn WriteUInt32(&self, value: u32) -> ::windows::core::Result<()>;
    fn WriteUInt64(&self, value: u64) -> ::windows::core::Result<()>;
    fn WriteSingle(&self, value: f32) -> ::windows::core::Result<()>;
    fn WriteDouble(&self, value: f64) -> ::windows::core::Result<()>;
    fn WriteDateTime(&self, value: &super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn WriteTimeSpan(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn WriteString(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<u32>;
    fn MeasureString(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<u32>;
    fn StoreAsync(&self) -> ::windows::core::Result<DataWriterStoreOperation>;
    fn FlushAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn DetachBuffer(&self) -> ::windows::core::Result<IBuffer>;
    fn DetachStream(&self) -> ::windows::core::Result<IOutputStream>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDataWriterFactoryImpl: Sized {
    fn CreateDataWriter(&self, outputstream: &::core::option::Option<IOutputStream>) -> ::windows::core::Result<DataWriter>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFileRandomAccessStreamStaticsImpl: Sized {
    fn OpenAsync(&self, filepath: &::windows::core::HSTRING, accessmode: super::FileAccessMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IRandomAccessStream>>;
    fn OpenWithOptionsAsync(&self, filepath: &::windows::core::HSTRING, accessmode: super::FileAccessMode, sharingoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IRandomAccessStream>>;
    fn OpenTransactedWriteAsync(&self, filepath: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>>;
    fn OpenTransactedWriteWithOptionsAsync(&self, filepath: &::windows::core::HSTRING, openoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>>;
    fn OpenForUserAsync(&self, user: &::core::option::Option<super::super::System::User>, filepath: &::windows::core::HSTRING, accessmode: super::FileAccessMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IRandomAccessStream>>;
    fn OpenForUserWithOptionsAsync(&self, user: &::core::option::Option<super::super::System::User>, filepath: &::windows::core::HSTRING, accessmode: super::FileAccessMode, sharingoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IRandomAccessStream>>;
    fn OpenTransactedWriteForUserAsync(&self, user: &::core::option::Option<super::super::System::User>, filepath: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>>;
    fn OpenTransactedWriteForUserWithOptionsAsync(&self, user: &::core::option::Option<super::super::System::User>, filepath: &::windows::core::HSTRING, openoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>>;
}
#[cfg(feature = "Foundation")]
pub trait IInputStreamImpl: Sized + IClosableImpl {
    fn ReadAsync(&self, buffer: &::core::option::Option<IBuffer>, count: u32, options: InputStreamOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>>;
}
pub trait IInputStreamReferenceImpl: Sized {
    fn OpenSequentialReadAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IInputStream>>;
}
#[cfg(feature = "Foundation")]
pub trait IOutputStreamImpl: Sized + IClosableImpl {
    fn WriteAsync(&self, buffer: &::core::option::Option<IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>;
    fn FlushAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
pub trait IPropertySetSerializerImpl: Sized {
    fn Serialize(&self, propertyset: &::core::option::Option<super::super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<IBuffer>;
    fn Deserialize(&self, propertyset: &::core::option::Option<super::super::Foundation::Collections::IPropertySet>, buffer: &::core::option::Option<IBuffer>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation")]
pub trait IRandomAccessStreamImpl: Sized + IClosableImpl + IInputStreamImpl + IOutputStreamImpl {
    fn Size(&self) -> ::windows::core::Result<u64>;
    fn SetSize(&self, value: u64) -> ::windows::core::Result<()>;
    fn GetInputStreamAt(&self, position: u64) -> ::windows::core::Result<IInputStream>;
    fn GetOutputStreamAt(&self, position: u64) -> ::windows::core::Result<IOutputStream>;
    fn Position(&self) -> ::windows::core::Result<u64>;
    fn Seek(&self, position: u64) -> ::windows::core::Result<()>;
    fn CloneStream(&self) -> ::windows::core::Result<IRandomAccessStream>;
    fn CanRead(&self) -> ::windows::core::Result<bool>;
    fn CanWrite(&self) -> ::windows::core::Result<bool>;
}
pub trait IRandomAccessStreamReferenceImpl: Sized {
    fn OpenReadAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IRandomAccessStreamWithContentType>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRandomAccessStreamReferenceStaticsImpl: Sized {
    fn CreateFromFile(&self, file: &::core::option::Option<super::IStorageFile>) -> ::windows::core::Result<RandomAccessStreamReference>;
    fn CreateFromUri(&self, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<RandomAccessStreamReference>;
    fn CreateFromStream(&self, stream: &::core::option::Option<IRandomAccessStream>) -> ::windows::core::Result<RandomAccessStreamReference>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRandomAccessStreamStaticsImpl: Sized {
    fn CopyAsync(&self, source: &::core::option::Option<IInputStream>, destination: &::core::option::Option<IOutputStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>;
    fn CopySizeAsync(&self, source: &::core::option::Option<IInputStream>, destination: &::core::option::Option<IOutputStream>, bytestocopy: u64) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>;
    fn CopyAndCloseAsync(&self, source: &::core::option::Option<IInputStream>, destination: &::core::option::Option<IOutputStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u64, u64>>;
}
#[cfg(feature = "Foundation")]
pub trait IRandomAccessStreamWithContentTypeImpl: Sized + IClosableImpl + IContentTypeProviderImpl + IInputStreamImpl + IOutputStreamImpl + IRandomAccessStreamImpl {}

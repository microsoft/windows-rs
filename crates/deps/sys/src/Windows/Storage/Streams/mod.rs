#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct Buffer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ByteOrder(pub i32);
impl ByteOrder {
    pub const LittleEndian: ByteOrder = ByteOrder(0i32);
    pub const BigEndian: ByteOrder = ByteOrder(1i32);
}
#[repr(transparent)]
pub struct DataReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DataReaderLoadOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DataWriter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DataWriterStoreOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FileInputStream(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FileOpenDisposition(pub i32);
impl FileOpenDisposition {
    pub const OpenExisting: FileOpenDisposition = FileOpenDisposition(0i32);
    pub const OpenAlways: FileOpenDisposition = FileOpenDisposition(1i32);
    pub const CreateNew: FileOpenDisposition = FileOpenDisposition(2i32);
    pub const CreateAlways: FileOpenDisposition = FileOpenDisposition(3i32);
    pub const TruncateExisting: FileOpenDisposition = FileOpenDisposition(4i32);
}
#[repr(transparent)]
pub struct FileOutputStream(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FileRandomAccessStream(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBuffer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBufferFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBufferStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContentTypeProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataReaderFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataReaderStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataWriter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataWriterFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileRandomAccessStreamStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInputStream(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInputStreamReference(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOutputStream(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPropertySetSerializer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRandomAccessStream(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRandomAccessStreamReference(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRandomAccessStreamReferenceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRandomAccessStreamStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRandomAccessStreamWithContentType(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InMemoryRandomAccessStream(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InputStreamOptions(pub u32);
impl InputStreamOptions {
    pub const None: InputStreamOptions = InputStreamOptions(0u32);
    pub const Partial: InputStreamOptions = InputStreamOptions(1u32);
    pub const ReadAhead: InputStreamOptions = InputStreamOptions(2u32);
}
#[repr(transparent)]
pub struct InputStreamOverStream(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct OutputStreamOverStream(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RandomAccessStreamOverStream(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RandomAccessStreamReference(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UnicodeEncoding(pub i32);
impl UnicodeEncoding {
    pub const Utf8: UnicodeEncoding = UnicodeEncoding(0i32);
    pub const Utf16LE: UnicodeEncoding = UnicodeEncoding(1i32);
    pub const Utf16BE: UnicodeEncoding = UnicodeEncoding(2i32);
}

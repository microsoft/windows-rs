#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct Buffer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ByteOrder(pub i32);
impl ByteOrder {
    pub const LittleEndian: Self = Self(0i32);
    pub const BigEndian: Self = Self(1i32);
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
    pub const OpenExisting: Self = Self(0i32);
    pub const OpenAlways: Self = Self(1i32);
    pub const CreateNew: Self = Self(2i32);
    pub const CreateAlways: Self = Self(3i32);
    pub const TruncateExisting: Self = Self(4i32);
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
    pub const None: Self = Self(0u32);
    pub const Partial: Self = Self(1u32);
    pub const ReadAhead: Self = Self(2u32);
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
    pub const Utf8: Self = Self(0i32);
    pub const Utf16LE: Self = Self(1i32);
    pub const Utf16BE: Self = Self(2i32);
}

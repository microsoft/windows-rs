#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct Buffer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Buffer {}
impl ::core::clone::Clone for Buffer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ByteOrder(pub i32);
impl ByteOrder {
    pub const LittleEndian: Self = Self(0i32);
    pub const BigEndian: Self = Self(1i32);
}
impl ::core::marker::Copy for ByteOrder {}
impl ::core::clone::Clone for ByteOrder {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DataReader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DataReader {}
impl ::core::clone::Clone for DataReader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DataReaderLoadOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DataReaderLoadOperation {}
impl ::core::clone::Clone for DataReaderLoadOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DataWriter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DataWriter {}
impl ::core::clone::Clone for DataWriter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DataWriterStoreOperation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DataWriterStoreOperation {}
impl ::core::clone::Clone for DataWriterStoreOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FileInputStream(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FileInputStream {}
impl ::core::clone::Clone for FileInputStream {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FileOpenDisposition(pub i32);
impl FileOpenDisposition {
    pub const OpenExisting: Self = Self(0i32);
    pub const OpenAlways: Self = Self(1i32);
    pub const CreateNew: Self = Self(2i32);
    pub const CreateAlways: Self = Self(3i32);
    pub const TruncateExisting: Self = Self(4i32);
}
impl ::core::marker::Copy for FileOpenDisposition {}
impl ::core::clone::Clone for FileOpenDisposition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FileOutputStream(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FileOutputStream {}
impl ::core::clone::Clone for FileOutputStream {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FileRandomAccessStream(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for FileRandomAccessStream {}
impl ::core::clone::Clone for FileRandomAccessStream {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBuffer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBuffer {}
impl ::core::clone::Clone for IBuffer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBufferFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBufferFactory {}
impl ::core::clone::Clone for IBufferFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBufferStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBufferStatics {}
impl ::core::clone::Clone for IBufferStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContentTypeProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContentTypeProvider {}
impl ::core::clone::Clone for IContentTypeProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDataReader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDataReader {}
impl ::core::clone::Clone for IDataReader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDataReaderFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDataReaderFactory {}
impl ::core::clone::Clone for IDataReaderFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDataReaderStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDataReaderStatics {}
impl ::core::clone::Clone for IDataReaderStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDataWriter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDataWriter {}
impl ::core::clone::Clone for IDataWriter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDataWriterFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDataWriterFactory {}
impl ::core::clone::Clone for IDataWriterFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFileRandomAccessStreamStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFileRandomAccessStreamStatics {}
impl ::core::clone::Clone for IFileRandomAccessStreamStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInputStream(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInputStream {}
impl ::core::clone::Clone for IInputStream {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInputStreamReference(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInputStreamReference {}
impl ::core::clone::Clone for IInputStreamReference {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOutputStream(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOutputStream {}
impl ::core::clone::Clone for IOutputStream {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPropertySetSerializer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPropertySetSerializer {}
impl ::core::clone::Clone for IPropertySetSerializer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRandomAccessStream(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRandomAccessStream {}
impl ::core::clone::Clone for IRandomAccessStream {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRandomAccessStreamReference(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRandomAccessStreamReference {}
impl ::core::clone::Clone for IRandomAccessStreamReference {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRandomAccessStreamReferenceStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRandomAccessStreamReferenceStatics {}
impl ::core::clone::Clone for IRandomAccessStreamReferenceStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRandomAccessStreamStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRandomAccessStreamStatics {}
impl ::core::clone::Clone for IRandomAccessStreamStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRandomAccessStreamWithContentType(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRandomAccessStreamWithContentType {}
impl ::core::clone::Clone for IRandomAccessStreamWithContentType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InMemoryRandomAccessStream(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for InMemoryRandomAccessStream {}
impl ::core::clone::Clone for InMemoryRandomAccessStream {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InputStreamOptions(pub u32);
impl InputStreamOptions {
    pub const None: Self = Self(0u32);
    pub const Partial: Self = Self(1u32);
    pub const ReadAhead: Self = Self(2u32);
}
impl ::core::marker::Copy for InputStreamOptions {}
impl ::core::clone::Clone for InputStreamOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InputStreamOverStream(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for InputStreamOverStream {}
impl ::core::clone::Clone for InputStreamOverStream {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct OutputStreamOverStream(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for OutputStreamOverStream {}
impl ::core::clone::Clone for OutputStreamOverStream {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RandomAccessStreamOverStream(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RandomAccessStreamOverStream {}
impl ::core::clone::Clone for RandomAccessStreamOverStream {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RandomAccessStreamReference(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RandomAccessStreamReference {}
impl ::core::clone::Clone for RandomAccessStreamReference {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UnicodeEncoding(pub i32);
impl UnicodeEncoding {
    pub const Utf8: Self = Self(0i32);
    pub const Utf16LE: Self = Self(1i32);
    pub const Utf16BE: Self = Self(2i32);
}
impl ::core::marker::Copy for UnicodeEncoding {}
impl ::core::clone::Clone for UnicodeEncoding {
    fn clone(&self) -> Self {
        *self
    }
}

impl ::core::cmp::PartialEq for Buffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Buffer {}
impl ::core::fmt::Debug for Buffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Buffer").field(&self.0).finish()
    }
}
impl ::core::default::Default for ByteOrder {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ByteOrder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ByteOrder").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for DataReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DataReader {}
impl ::core::fmt::Debug for DataReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataReader").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::PartialEq for DataReaderLoadOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::Eq for DataReaderLoadOperation {}
#[cfg(feature = "Foundation")]
impl ::core::fmt::Debug for DataReaderLoadOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataReaderLoadOperation").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for DataWriter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DataWriter {}
impl ::core::fmt::Debug for DataWriter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataWriter").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::PartialEq for DataWriterStoreOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::Eq for DataWriterStoreOperation {}
#[cfg(feature = "Foundation")]
impl ::core::fmt::Debug for DataWriterStoreOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataWriterStoreOperation").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for FileInputStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FileInputStream {}
impl ::core::fmt::Debug for FileInputStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileInputStream").field(&self.0).finish()
    }
}
impl ::core::default::Default for FileOpenDisposition {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FileOpenDisposition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileOpenDisposition").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for FileOutputStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FileOutputStream {}
impl ::core::fmt::Debug for FileOutputStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileOutputStream").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for FileRandomAccessStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FileRandomAccessStream {}
impl ::core::fmt::Debug for FileRandomAccessStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileRandomAccessStream").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBuffer {}
impl ::core::fmt::Debug for IBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBuffer").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IContentTypeProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContentTypeProvider {}
impl ::core::fmt::Debug for IContentTypeProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContentTypeProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDataReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDataReader {}
impl ::core::fmt::Debug for IDataReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataReader").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDataWriter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDataWriter {}
impl ::core::fmt::Debug for IDataWriter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataWriter").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IInputStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInputStream {}
impl ::core::fmt::Debug for IInputStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInputStream").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IInputStreamReference {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInputStreamReference {}
impl ::core::fmt::Debug for IInputStreamReference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInputStreamReference").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IOutputStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOutputStream {}
impl ::core::fmt::Debug for IOutputStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOutputStream").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPropertySetSerializer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertySetSerializer {}
impl ::core::fmt::Debug for IPropertySetSerializer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertySetSerializer").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRandomAccessStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRandomAccessStream {}
impl ::core::fmt::Debug for IRandomAccessStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRandomAccessStream").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRandomAccessStreamReference {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRandomAccessStreamReference {}
impl ::core::fmt::Debug for IRandomAccessStreamReference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRandomAccessStreamReference").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRandomAccessStreamWithContentType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRandomAccessStreamWithContentType {}
impl ::core::fmt::Debug for IRandomAccessStreamWithContentType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRandomAccessStreamWithContentType").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for InMemoryRandomAccessStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InMemoryRandomAccessStream {}
impl ::core::fmt::Debug for InMemoryRandomAccessStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InMemoryRandomAccessStream").field(&self.0).finish()
    }
}
impl ::core::default::Default for InputStreamOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for InputStreamOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InputStreamOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for InputStreamOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for InputStreamOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for InputStreamOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for InputStreamOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for InputStreamOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::cmp::PartialEq for InputStreamOverStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InputStreamOverStream {}
impl ::core::fmt::Debug for InputStreamOverStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InputStreamOverStream").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for OutputStreamOverStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OutputStreamOverStream {}
impl ::core::fmt::Debug for OutputStreamOverStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OutputStreamOverStream").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for RandomAccessStreamOverStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RandomAccessStreamOverStream {}
impl ::core::fmt::Debug for RandomAccessStreamOverStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RandomAccessStreamOverStream").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for RandomAccessStreamReference {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RandomAccessStreamReference {}
impl ::core::fmt::Debug for RandomAccessStreamReference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RandomAccessStreamReference").field(&self.0).finish()
    }
}
impl ::core::default::Default for UnicodeEncoding {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UnicodeEncoding {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UnicodeEncoding").field(&self.0).finish()
    }
}

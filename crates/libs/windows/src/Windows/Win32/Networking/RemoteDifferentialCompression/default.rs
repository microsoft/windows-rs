impl ::core::default::Default for FindSimilarFileIndexResults {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FindSimilarFileIndexResults {
    fn eq(&self, other: &Self) -> bool {
        self.m_FileIndex == other.m_FileIndex && self.m_MatchCount == other.m_MatchCount
    }
}
impl ::core::cmp::Eq for FindSimilarFileIndexResults {}
impl ::core::fmt::Debug for FindSimilarFileIndexResults {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FindSimilarFileIndexResults").field("m_FileIndex", &self.m_FileIndex).field("m_MatchCount", &self.m_MatchCount).finish()
    }
}
impl ::core::default::Default for GeneratorParametersType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GeneratorParametersType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GeneratorParametersType").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IFindSimilarResults {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFindSimilarResults {}
impl ::core::fmt::Debug for IFindSimilarResults {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFindSimilarResults").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRdcComparator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRdcComparator {}
impl ::core::fmt::Debug for IRdcComparator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRdcComparator").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRdcFileReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRdcFileReader {}
impl ::core::fmt::Debug for IRdcFileReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRdcFileReader").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRdcFileWriter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRdcFileWriter {}
impl ::core::fmt::Debug for IRdcFileWriter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRdcFileWriter").field(&self.0).finish()
    }
}
impl IRdcFileWriter {
    pub unsafe fn GetFileSize(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFileSize)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Read(&self, offsetfilestart: u64, bytestoread: u32, bytesactuallyread: *mut u32, buffer: *mut u8, eof: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Read)(::windows::core::Vtable::as_raw(self), offsetfilestart, bytestoread, bytesactuallyread, buffer, eof).ok()
    }
    pub unsafe fn GetFilePosition(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFilePosition)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IRdcGenerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRdcGenerator {}
impl ::core::fmt::Debug for IRdcGenerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRdcGenerator").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRdcGeneratorFilterMaxParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRdcGeneratorFilterMaxParameters {}
impl ::core::fmt::Debug for IRdcGeneratorFilterMaxParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRdcGeneratorFilterMaxParameters").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRdcGeneratorParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRdcGeneratorParameters {}
impl ::core::fmt::Debug for IRdcGeneratorParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRdcGeneratorParameters").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRdcLibrary {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRdcLibrary {}
impl ::core::fmt::Debug for IRdcLibrary {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRdcLibrary").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRdcSignatureReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRdcSignatureReader {}
impl ::core::fmt::Debug for IRdcSignatureReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRdcSignatureReader").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRdcSimilarityGenerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRdcSimilarityGenerator {}
impl ::core::fmt::Debug for IRdcSimilarityGenerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRdcSimilarityGenerator").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISimilarity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISimilarity {}
impl ::core::fmt::Debug for ISimilarity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISimilarity").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISimilarityFileIdTable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISimilarityFileIdTable {}
impl ::core::fmt::Debug for ISimilarityFileIdTable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISimilarityFileIdTable").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISimilarityReportProgress {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISimilarityReportProgress {}
impl ::core::fmt::Debug for ISimilarityReportProgress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISimilarityReportProgress").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISimilarityTableDumpState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISimilarityTableDumpState {}
impl ::core::fmt::Debug for ISimilarityTableDumpState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISimilarityTableDumpState").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISimilarityTraitsMappedView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISimilarityTraitsMappedView {}
impl ::core::fmt::Debug for ISimilarityTraitsMappedView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISimilarityTraitsMappedView").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISimilarityTraitsMapping {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISimilarityTraitsMapping {}
impl ::core::fmt::Debug for ISimilarityTraitsMapping {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISimilarityTraitsMapping").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISimilarityTraitsTable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISimilarityTraitsTable {}
impl ::core::fmt::Debug for ISimilarityTraitsTable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISimilarityTraitsTable").field(&self.0).finish()
    }
}
impl ::core::default::Default for RDC_ErrorCode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RDC_ErrorCode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RDC_ErrorCode").field(&self.0).finish()
    }
}
impl ::core::default::Default for RdcBufferPointer {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RdcBufferPointer {
    fn eq(&self, other: &Self) -> bool {
        self.m_Size == other.m_Size && self.m_Used == other.m_Used && self.m_Data == other.m_Data
    }
}
impl ::core::cmp::Eq for RdcBufferPointer {}
impl ::core::fmt::Debug for RdcBufferPointer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RdcBufferPointer").field("m_Size", &self.m_Size).field("m_Used", &self.m_Used).field("m_Data", &self.m_Data).finish()
    }
}
impl ::core::default::Default for RdcCreatedTables {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RdcCreatedTables {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RdcCreatedTables").field(&self.0).finish()
    }
}
impl ::core::default::Default for RdcMappingAccessMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RdcMappingAccessMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RdcMappingAccessMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for RdcNeed {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RdcNeed {
    fn eq(&self, other: &Self) -> bool {
        self.m_BlockType == other.m_BlockType && self.m_FileOffset == other.m_FileOffset && self.m_BlockLength == other.m_BlockLength
    }
}
impl ::core::cmp::Eq for RdcNeed {}
impl ::core::fmt::Debug for RdcNeed {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RdcNeed").field("m_BlockType", &self.m_BlockType).field("m_FileOffset", &self.m_FileOffset).field("m_BlockLength", &self.m_BlockLength).finish()
    }
}
impl ::core::default::Default for RdcNeedPointer {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RdcNeedPointer {
    fn eq(&self, other: &Self) -> bool {
        self.m_Size == other.m_Size && self.m_Used == other.m_Used && self.m_Data == other.m_Data
    }
}
impl ::core::cmp::Eq for RdcNeedPointer {}
impl ::core::fmt::Debug for RdcNeedPointer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RdcNeedPointer").field("m_Size", &self.m_Size).field("m_Used", &self.m_Used).field("m_Data", &self.m_Data).finish()
    }
}
impl ::core::default::Default for RdcNeedType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RdcNeedType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RdcNeedType").field(&self.0).finish()
    }
}
impl ::core::default::Default for RdcSignature {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RdcSignature {
    fn eq(&self, other: &Self) -> bool {
        self.m_Signature == other.m_Signature && self.m_BlockLength == other.m_BlockLength
    }
}
impl ::core::cmp::Eq for RdcSignature {}
impl ::core::fmt::Debug for RdcSignature {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RdcSignature").field("m_Signature", &self.m_Signature).field("m_BlockLength", &self.m_BlockLength).finish()
    }
}
impl ::core::default::Default for RdcSignaturePointer {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RdcSignaturePointer {
    fn eq(&self, other: &Self) -> bool {
        self.m_Size == other.m_Size && self.m_Used == other.m_Used && self.m_Data == other.m_Data
    }
}
impl ::core::cmp::Eq for RdcSignaturePointer {}
impl ::core::fmt::Debug for RdcSignaturePointer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RdcSignaturePointer").field("m_Size", &self.m_Size).field("m_Used", &self.m_Used).field("m_Data", &self.m_Data).finish()
    }
}
impl ::core::default::Default for SimilarityData {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SimilarityData {
    fn eq(&self, other: &Self) -> bool {
        self.m_Data == other.m_Data
    }
}
impl ::core::cmp::Eq for SimilarityData {}
impl ::core::fmt::Debug for SimilarityData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SimilarityData").field("m_Data", &self.m_Data).finish()
    }
}
impl ::core::default::Default for SimilarityDumpData {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SimilarityDumpData {
    fn eq(&self, other: &Self) -> bool {
        self.m_FileIndex == other.m_FileIndex && self.m_Data == other.m_Data
    }
}
impl ::core::cmp::Eq for SimilarityDumpData {}
impl ::core::fmt::Debug for SimilarityDumpData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SimilarityDumpData").field("m_FileIndex", &self.m_FileIndex).field("m_Data", &self.m_Data).finish()
    }
}
impl ::core::default::Default for SimilarityFileId {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SimilarityFileId {
    fn eq(&self, other: &Self) -> bool {
        self.m_FileId == other.m_FileId
    }
}
impl ::core::cmp::Eq for SimilarityFileId {}
impl ::core::fmt::Debug for SimilarityFileId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SimilarityFileId").field("m_FileId", &self.m_FileId).finish()
    }
}
impl ::core::default::Default for SimilarityMappedViewInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SimilarityMappedViewInfo {
    fn eq(&self, other: &Self) -> bool {
        self.m_Data == other.m_Data && self.m_Length == other.m_Length
    }
}
impl ::core::cmp::Eq for SimilarityMappedViewInfo {}
impl ::core::fmt::Debug for SimilarityMappedViewInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SimilarityMappedViewInfo").field("m_Data", &self.m_Data).field("m_Length", &self.m_Length).finish()
    }
}

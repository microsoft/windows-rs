impl ::core::default::Default for CHUNKSTATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CHUNKSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CHUNKSTATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CHUNK_BREAKTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CHUNK_BREAKTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CHUNK_BREAKTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CI_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CI_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.cWordList == other.cWordList && self.cPersistentIndex == other.cPersistentIndex && self.cQueries == other.cQueries && self.cDocuments == other.cDocuments && self.cFreshTest == other.cFreshTest && self.dwMergeProgress == other.dwMergeProgress && self.eState == other.eState && self.cFilteredDocuments == other.cFilteredDocuments && self.cTotalDocuments == other.cTotalDocuments && self.cPendingScans == other.cPendingScans && self.dwIndexSize == other.dwIndexSize && self.cUniqueKeys == other.cUniqueKeys && self.cSecQDocuments == other.cSecQDocuments && self.dwPropCacheSize == other.dwPropCacheSize
    }
}
impl ::core::cmp::Eq for CI_STATE {}
impl ::core::fmt::Debug for CI_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CI_STATE")
            .field("cbStruct", &self.cbStruct)
            .field("cWordList", &self.cWordList)
            .field("cPersistentIndex", &self.cPersistentIndex)
            .field("cQueries", &self.cQueries)
            .field("cDocuments", &self.cDocuments)
            .field("cFreshTest", &self.cFreshTest)
            .field("dwMergeProgress", &self.dwMergeProgress)
            .field("eState", &self.eState)
            .field("cFilteredDocuments", &self.cFilteredDocuments)
            .field("cTotalDocuments", &self.cTotalDocuments)
            .field("cPendingScans", &self.cPendingScans)
            .field("dwIndexSize", &self.dwIndexSize)
            .field("cUniqueKeys", &self.cUniqueKeys)
            .field("cSecQDocuments", &self.cSecQDocuments)
            .field("dwPropCacheSize", &self.dwPropCacheSize)
            .finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for DBID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for DBID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DBKINDENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBKINDENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBKINDENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for FILTERREGION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILTERREGION {
    fn eq(&self, other: &Self) -> bool {
        self.idChunk == other.idChunk && self.cwcStart == other.cwcStart && self.cwcExtent == other.cwcExtent
    }
}
impl ::core::cmp::Eq for FILTERREGION {}
impl ::core::fmt::Debug for FILTERREGION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILTERREGION").field("idChunk", &self.idChunk).field("cwcStart", &self.cwcStart).field("cwcExtent", &self.cwcExtent).finish()
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::default::Default for FULLPROPSPEC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IFILTER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IFILTER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFILTER_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for IFILTER_INIT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IFILTER_INIT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFILTER_INIT").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFilter {}
impl ::core::fmt::Debug for IFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFilter").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IPhraseSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPhraseSink {}
impl ::core::fmt::Debug for IPhraseSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPhraseSink").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::core::default::Default for STAT_CHUNK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WORDREP_BREAK_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WORDREP_BREAK_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WORDREP_BREAK_TYPE").field(&self.0).finish()
    }
}

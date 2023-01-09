#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JET_BKINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JET_BKLOGTIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JET_COLUMNBASE_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for JET_COLUMNBASE_A {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.columnid == other.columnid && self.coltyp == other.coltyp && self.wCountry == other.wCountry && self.langid == other.langid && self.cp == other.cp && self.wFiller == other.wFiller && self.cbMax == other.cbMax && self.grbit == other.grbit && self.szBaseTableName == other.szBaseTableName && self.szBaseColumnName == other.szBaseColumnName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for JET_COLUMNBASE_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for JET_COLUMNBASE_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_COLUMNBASE_A").field("cbStruct", &self.cbStruct).field("columnid", &self.columnid).field("coltyp", &self.coltyp).field("wCountry", &self.wCountry).field("langid", &self.langid).field("cp", &self.cp).field("wFiller", &self.wFiller).field("cbMax", &self.cbMax).field("grbit", &self.grbit).field("szBaseTableName", &self.szBaseTableName).field("szBaseColumnName", &self.szBaseColumnName).finish()
    }
}
impl ::core::default::Default for JET_COLUMNBASE_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JET_COLUMNBASE_W {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.columnid == other.columnid && self.coltyp == other.coltyp && self.wCountry == other.wCountry && self.langid == other.langid && self.cp == other.cp && self.wFiller == other.wFiller && self.cbMax == other.cbMax && self.grbit == other.grbit && self.szBaseTableName == other.szBaseTableName && self.szBaseColumnName == other.szBaseColumnName
    }
}
impl ::core::cmp::Eq for JET_COLUMNBASE_W {}
impl ::core::fmt::Debug for JET_COLUMNBASE_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_COLUMNBASE_W").field("cbStruct", &self.cbStruct).field("columnid", &self.columnid).field("coltyp", &self.coltyp).field("wCountry", &self.wCountry).field("langid", &self.langid).field("cp", &self.cp).field("wFiller", &self.wFiller).field("cbMax", &self.cbMax).field("grbit", &self.grbit).field("szBaseTableName", &self.szBaseTableName).field("szBaseColumnName", &self.szBaseColumnName).finish()
    }
}
impl ::core::default::Default for JET_COLUMNCREATE_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JET_COLUMNCREATE_A {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.szColumnName == other.szColumnName && self.coltyp == other.coltyp && self.cbMax == other.cbMax && self.grbit == other.grbit && self.pvDefault == other.pvDefault && self.cbDefault == other.cbDefault && self.cp == other.cp && self.columnid == other.columnid && self.err == other.err
    }
}
impl ::core::cmp::Eq for JET_COLUMNCREATE_A {}
impl ::core::fmt::Debug for JET_COLUMNCREATE_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_COLUMNCREATE_A").field("cbStruct", &self.cbStruct).field("szColumnName", &self.szColumnName).field("coltyp", &self.coltyp).field("cbMax", &self.cbMax).field("grbit", &self.grbit).field("pvDefault", &self.pvDefault).field("cbDefault", &self.cbDefault).field("cp", &self.cp).field("columnid", &self.columnid).field("err", &self.err).finish()
    }
}
impl ::core::default::Default for JET_COLUMNCREATE_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JET_COLUMNCREATE_W {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.szColumnName == other.szColumnName && self.coltyp == other.coltyp && self.cbMax == other.cbMax && self.grbit == other.grbit && self.pvDefault == other.pvDefault && self.cbDefault == other.cbDefault && self.cp == other.cp && self.columnid == other.columnid && self.err == other.err
    }
}
impl ::core::cmp::Eq for JET_COLUMNCREATE_W {}
impl ::core::fmt::Debug for JET_COLUMNCREATE_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_COLUMNCREATE_W").field("cbStruct", &self.cbStruct).field("szColumnName", &self.szColumnName).field("coltyp", &self.coltyp).field("cbMax", &self.cbMax).field("grbit", &self.grbit).field("pvDefault", &self.pvDefault).field("cbDefault", &self.cbDefault).field("cp", &self.cp).field("columnid", &self.columnid).field("err", &self.err).finish()
    }
}
impl ::core::default::Default for JET_COLUMNDEF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JET_COLUMNDEF {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.columnid == other.columnid && self.coltyp == other.coltyp && self.wCountry == other.wCountry && self.langid == other.langid && self.cp == other.cp && self.wCollate == other.wCollate && self.cbMax == other.cbMax && self.grbit == other.grbit
    }
}
impl ::core::cmp::Eq for JET_COLUMNDEF {}
impl ::core::fmt::Debug for JET_COLUMNDEF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_COLUMNDEF").field("cbStruct", &self.cbStruct).field("columnid", &self.columnid).field("coltyp", &self.coltyp).field("wCountry", &self.wCountry).field("langid", &self.langid).field("cp", &self.cp).field("wCollate", &self.wCollate).field("cbMax", &self.cbMax).field("grbit", &self.grbit).finish()
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::default::Default for JET_COLUMNLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::PartialEq for JET_COLUMNLIST {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.tableid == other.tableid
            && self.cRecord == other.cRecord
            && self.columnidPresentationOrder == other.columnidPresentationOrder
            && self.columnidcolumnname == other.columnidcolumnname
            && self.columnidcolumnid == other.columnidcolumnid
            && self.columnidcoltyp == other.columnidcoltyp
            && self.columnidCountry == other.columnidCountry
            && self.columnidLangid == other.columnidLangid
            && self.columnidCp == other.columnidCp
            && self.columnidCollate == other.columnidCollate
            && self.columnidcbMax == other.columnidcbMax
            && self.columnidgrbit == other.columnidgrbit
            && self.columnidDefault == other.columnidDefault
            && self.columnidBaseTableName == other.columnidBaseTableName
            && self.columnidBaseColumnName == other.columnidBaseColumnName
            && self.columnidDefinitionName == other.columnidDefinitionName
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::Eq for JET_COLUMNLIST {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::fmt::Debug for JET_COLUMNLIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_COLUMNLIST")
            .field("cbStruct", &self.cbStruct)
            .field("tableid", &self.tableid)
            .field("cRecord", &self.cRecord)
            .field("columnidPresentationOrder", &self.columnidPresentationOrder)
            .field("columnidcolumnname", &self.columnidcolumnname)
            .field("columnidcolumnid", &self.columnidcolumnid)
            .field("columnidcoltyp", &self.columnidcoltyp)
            .field("columnidCountry", &self.columnidCountry)
            .field("columnidLangid", &self.columnidLangid)
            .field("columnidCp", &self.columnidCp)
            .field("columnidCollate", &self.columnidCollate)
            .field("columnidcbMax", &self.columnidcbMax)
            .field("columnidgrbit", &self.columnidgrbit)
            .field("columnidDefault", &self.columnidDefault)
            .field("columnidBaseTableName", &self.columnidBaseTableName)
            .field("columnidBaseColumnName", &self.columnidBaseColumnName)
            .field("columnidDefinitionName", &self.columnidDefinitionName)
            .finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JET_COMMIT_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JET_COMMIT_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for JET_CONDITIONALCOLUMN_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JET_CONDITIONALCOLUMN_A {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.szColumnName == other.szColumnName && self.grbit == other.grbit
    }
}
impl ::core::cmp::Eq for JET_CONDITIONALCOLUMN_A {}
impl ::core::fmt::Debug for JET_CONDITIONALCOLUMN_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_CONDITIONALCOLUMN_A").field("cbStruct", &self.cbStruct).field("szColumnName", &self.szColumnName).field("grbit", &self.grbit).finish()
    }
}
impl ::core::default::Default for JET_CONDITIONALCOLUMN_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JET_CONDITIONALCOLUMN_W {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.szColumnName == other.szColumnName && self.grbit == other.grbit
    }
}
impl ::core::cmp::Eq for JET_CONDITIONALCOLUMN_W {}
impl ::core::fmt::Debug for JET_CONDITIONALCOLUMN_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_CONDITIONALCOLUMN_W").field("cbStruct", &self.cbStruct).field("szColumnName", &self.szColumnName).field("grbit", &self.grbit).finish()
    }
}
impl ::core::default::Default for JET_CONVERT_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for JET_CONVERT_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JET_DBINFOMISC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JET_DBINFOMISC2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JET_DBINFOMISC3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JET_DBINFOMISC4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for JET_DBINFOUPGRADE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for JET_ENUMCOLUMN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for JET_ENUMCOLUMNID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JET_ENUMCOLUMNID {
    fn eq(&self, other: &Self) -> bool {
        self.columnid == other.columnid && self.ctagSequence == other.ctagSequence && self.rgtagSequence == other.rgtagSequence
    }
}
impl ::core::cmp::Eq for JET_ENUMCOLUMNID {}
impl ::core::fmt::Debug for JET_ENUMCOLUMNID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_ENUMCOLUMNID").field("columnid", &self.columnid).field("ctagSequence", &self.ctagSequence).field("rgtagSequence", &self.rgtagSequence).finish()
    }
}
impl ::core::default::Default for JET_ENUMCOLUMNVALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JET_ENUMCOLUMNVALUE {
    fn eq(&self, other: &Self) -> bool {
        self.itagSequence == other.itagSequence && self.err == other.err && self.cbData == other.cbData && self.pvData == other.pvData
    }
}
impl ::core::cmp::Eq for JET_ENUMCOLUMNVALUE {}
impl ::core::fmt::Debug for JET_ENUMCOLUMNVALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_ENUMCOLUMNVALUE").field("itagSequence", &self.itagSequence).field("err", &self.err).field("cbData", &self.cbData).field("pvData", &self.pvData).finish()
    }
}
impl ::core::default::Default for JET_ERRCAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for JET_ERRCAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JET_ERRCAT").field(&self.0).finish()
    }
}
impl ::core::default::Default for JET_ERRINFOBASIC_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JET_ERRINFOBASIC_W {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.errValue == other.errValue && self.errcatMostSpecific == other.errcatMostSpecific && self.rgCategoricalHierarchy == other.rgCategoricalHierarchy && self.lSourceLine == other.lSourceLine && self.rgszSourceFile == other.rgszSourceFile
    }
}
impl ::core::cmp::Eq for JET_ERRINFOBASIC_W {}
impl ::core::fmt::Debug for JET_ERRINFOBASIC_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_ERRINFOBASIC_W").field("cbStruct", &self.cbStruct).field("errValue", &self.errValue).field("errcatMostSpecific", &self.errcatMostSpecific).field("rgCategoricalHierarchy", &self.rgCategoricalHierarchy).field("lSourceLine", &self.lSourceLine).field("rgszSourceFile", &self.rgszSourceFile).finish()
    }
}
impl ::core::default::Default for JET_INDEXCHECKING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for JET_INDEXCHECKING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JET_INDEXCHECKING").field(&self.0).finish()
    }
}
impl ::core::default::Default for JET_INDEXCREATE2_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for JET_INDEXCREATE2_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for JET_INDEXCREATE3_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for JET_INDEXCREATE3_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for JET_INDEXCREATE_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for JET_INDEXCREATE_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for JET_INDEXID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for JET_INDEXID {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.rgbIndexId == other.rgbIndexId
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for JET_INDEXID {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for JET_INDEXID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_INDEXID").field("cbStruct", &self.cbStruct).field("rgbIndexId", &self.rgbIndexId).finish()
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for JET_INDEXID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::PartialEq for JET_INDEXID {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.rgbIndexId == other.rgbIndexId
    }
}
#[cfg(target_arch = "x86")]
impl ::core::cmp::Eq for JET_INDEXID {}
#[cfg(target_arch = "x86")]
impl ::core::fmt::Debug for JET_INDEXID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_INDEXID").field("cbStruct", &self.cbStruct).field("rgbIndexId", &self.rgbIndexId).finish()
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::default::Default for JET_INDEXLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::PartialEq for JET_INDEXLIST {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct
            && self.tableid == other.tableid
            && self.cRecord == other.cRecord
            && self.columnidindexname == other.columnidindexname
            && self.columnidgrbitIndex == other.columnidgrbitIndex
            && self.columnidcKey == other.columnidcKey
            && self.columnidcEntry == other.columnidcEntry
            && self.columnidcPage == other.columnidcPage
            && self.columnidcColumn == other.columnidcColumn
            && self.columnidiColumn == other.columnidiColumn
            && self.columnidcolumnid == other.columnidcolumnid
            && self.columnidcoltyp == other.columnidcoltyp
            && self.columnidCountry == other.columnidCountry
            && self.columnidLangid == other.columnidLangid
            && self.columnidCp == other.columnidCp
            && self.columnidCollate == other.columnidCollate
            && self.columnidgrbitColumn == other.columnidgrbitColumn
            && self.columnidcolumnname == other.columnidcolumnname
            && self.columnidLCMapFlags == other.columnidLCMapFlags
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::Eq for JET_INDEXLIST {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::fmt::Debug for JET_INDEXLIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_INDEXLIST")
            .field("cbStruct", &self.cbStruct)
            .field("tableid", &self.tableid)
            .field("cRecord", &self.cRecord)
            .field("columnidindexname", &self.columnidindexname)
            .field("columnidgrbitIndex", &self.columnidgrbitIndex)
            .field("columnidcKey", &self.columnidcKey)
            .field("columnidcEntry", &self.columnidcEntry)
            .field("columnidcPage", &self.columnidcPage)
            .field("columnidcColumn", &self.columnidcColumn)
            .field("columnidiColumn", &self.columnidiColumn)
            .field("columnidcolumnid", &self.columnidcolumnid)
            .field("columnidcoltyp", &self.columnidcoltyp)
            .field("columnidCountry", &self.columnidCountry)
            .field("columnidLangid", &self.columnidLangid)
            .field("columnidCp", &self.columnidCp)
            .field("columnidCollate", &self.columnidCollate)
            .field("columnidgrbitColumn", &self.columnidgrbitColumn)
            .field("columnidcolumnname", &self.columnidcolumnname)
            .field("columnidLCMapFlags", &self.columnidLCMapFlags)
            .finish()
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::default::Default for JET_INDEXRANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::PartialEq for JET_INDEXRANGE {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.tableid == other.tableid && self.grbit == other.grbit
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::Eq for JET_INDEXRANGE {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::fmt::Debug for JET_INDEXRANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_INDEXRANGE").field("cbStruct", &self.cbStruct).field("tableid", &self.tableid).field("grbit", &self.grbit).finish()
    }
}
impl ::core::default::Default for JET_INDEX_COLUMN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JET_INDEX_COLUMN {
    fn eq(&self, other: &Self) -> bool {
        self.columnid == other.columnid && self.relop == other.relop && self.pv == other.pv && self.cb == other.cb && self.grbit == other.grbit
    }
}
impl ::core::cmp::Eq for JET_INDEX_COLUMN {}
impl ::core::fmt::Debug for JET_INDEX_COLUMN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_INDEX_COLUMN").field("columnid", &self.columnid).field("relop", &self.relop).field("pv", &self.pv).field("cb", &self.cb).field("grbit", &self.grbit).finish()
    }
}
impl ::core::default::Default for JET_INDEX_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JET_INDEX_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.rgStartColumns == other.rgStartColumns && self.cStartColumns == other.cStartColumns && self.rgEndColumns == other.rgEndColumns && self.cEndColumns == other.cEndColumns
    }
}
impl ::core::cmp::Eq for JET_INDEX_RANGE {}
impl ::core::fmt::Debug for JET_INDEX_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_INDEX_RANGE").field("rgStartColumns", &self.rgStartColumns).field("cStartColumns", &self.cStartColumns).field("rgEndColumns", &self.rgEndColumns).field("cEndColumns", &self.cEndColumns).finish()
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::default::Default for JET_INSTANCE_INFO_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::PartialEq for JET_INSTANCE_INFO_A {
    fn eq(&self, other: &Self) -> bool {
        self.hInstanceId == other.hInstanceId && self.szInstanceName == other.szInstanceName && self.cDatabases == other.cDatabases && self.szDatabaseFileName == other.szDatabaseFileName && self.szDatabaseDisplayName == other.szDatabaseDisplayName && self.szDatabaseSLVFileName_Obsolete == other.szDatabaseSLVFileName_Obsolete
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::Eq for JET_INSTANCE_INFO_A {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::fmt::Debug for JET_INSTANCE_INFO_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_INSTANCE_INFO_A").field("hInstanceId", &self.hInstanceId).field("szInstanceName", &self.szInstanceName).field("cDatabases", &self.cDatabases).field("szDatabaseFileName", &self.szDatabaseFileName).field("szDatabaseDisplayName", &self.szDatabaseDisplayName).field("szDatabaseSLVFileName_Obsolete", &self.szDatabaseSLVFileName_Obsolete).finish()
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::default::Default for JET_INSTANCE_INFO_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::PartialEq for JET_INSTANCE_INFO_W {
    fn eq(&self, other: &Self) -> bool {
        self.hInstanceId == other.hInstanceId && self.szInstanceName == other.szInstanceName && self.cDatabases == other.cDatabases && self.szDatabaseFileName == other.szDatabaseFileName && self.szDatabaseDisplayName == other.szDatabaseDisplayName && self.szDatabaseSLVFileName_Obsolete == other.szDatabaseSLVFileName_Obsolete
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::Eq for JET_INSTANCE_INFO_W {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::fmt::Debug for JET_INSTANCE_INFO_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_INSTANCE_INFO_W").field("hInstanceId", &self.hInstanceId).field("szInstanceName", &self.szInstanceName).field("cDatabases", &self.cDatabases).field("szDatabaseFileName", &self.szDatabaseFileName).field("szDatabaseDisplayName", &self.szDatabaseDisplayName).field("szDatabaseSLVFileName_Obsolete", &self.szDatabaseSLVFileName_Obsolete).finish()
    }
}
impl ::core::default::Default for JET_LGPOS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JET_LOGINFO_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for JET_LOGINFO_A {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.ulGenLow == other.ulGenLow && self.ulGenHigh == other.ulGenHigh && self.szBaseName == other.szBaseName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for JET_LOGINFO_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for JET_LOGINFO_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_LOGINFO_A").field("cbSize", &self.cbSize).field("ulGenLow", &self.ulGenLow).field("ulGenHigh", &self.ulGenHigh).field("szBaseName", &self.szBaseName).finish()
    }
}
impl ::core::default::Default for JET_LOGINFO_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JET_LOGINFO_W {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.ulGenLow == other.ulGenLow && self.ulGenHigh == other.ulGenHigh && self.szBaseName == other.szBaseName
    }
}
impl ::core::cmp::Eq for JET_LOGINFO_W {}
impl ::core::fmt::Debug for JET_LOGINFO_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_LOGINFO_W").field("cbSize", &self.cbSize).field("ulGenLow", &self.ulGenLow).field("ulGenHigh", &self.ulGenHigh).field("szBaseName", &self.szBaseName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JET_LOGTIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for JET_OBJECTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for JET_OBJECTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::default::Default for JET_OBJECTLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::PartialEq for JET_OBJECTLIST {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.tableid == other.tableid && self.cRecord == other.cRecord && self.columnidcontainername == other.columnidcontainername && self.columnidobjectname == other.columnidobjectname && self.columnidobjtyp == other.columnidobjtyp && self.columniddtCreate == other.columniddtCreate && self.columniddtUpdate == other.columniddtUpdate && self.columnidgrbit == other.columnidgrbit && self.columnidflags == other.columnidflags && self.columnidcRecord == other.columnidcRecord && self.columnidcPage == other.columnidcPage
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::Eq for JET_OBJECTLIST {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::fmt::Debug for JET_OBJECTLIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_OBJECTLIST")
            .field("cbStruct", &self.cbStruct)
            .field("tableid", &self.tableid)
            .field("cRecord", &self.cRecord)
            .field("columnidcontainername", &self.columnidcontainername)
            .field("columnidobjectname", &self.columnidobjectname)
            .field("columnidobjtyp", &self.columnidobjtyp)
            .field("columniddtCreate", &self.columniddtCreate)
            .field("columniddtUpdate", &self.columniddtUpdate)
            .field("columnidgrbit", &self.columnidgrbit)
            .field("columnidflags", &self.columnidflags)
            .field("columnidcRecord", &self.columnidcRecord)
            .field("columnidcPage", &self.columnidcPage)
            .finish()
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::default::Default for JET_OPENTEMPORARYTABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::PartialEq for JET_OPENTEMPORARYTABLE {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.prgcolumndef == other.prgcolumndef && self.ccolumn == other.ccolumn && self.pidxunicode == other.pidxunicode && self.grbit == other.grbit && self.prgcolumnid == other.prgcolumnid && self.cbKeyMost == other.cbKeyMost && self.cbVarSegMac == other.cbVarSegMac && self.tableid == other.tableid
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::Eq for JET_OPENTEMPORARYTABLE {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::fmt::Debug for JET_OPENTEMPORARYTABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_OPENTEMPORARYTABLE").field("cbStruct", &self.cbStruct).field("prgcolumndef", &self.prgcolumndef).field("ccolumn", &self.ccolumn).field("pidxunicode", &self.pidxunicode).field("grbit", &self.grbit).field("prgcolumnid", &self.prgcolumnid).field("cbKeyMost", &self.cbKeyMost).field("cbVarSegMac", &self.cbVarSegMac).field("tableid", &self.tableid).finish()
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::default::Default for JET_OPENTEMPORARYTABLE2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::PartialEq for JET_OPENTEMPORARYTABLE2 {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.prgcolumndef == other.prgcolumndef && self.ccolumn == other.ccolumn && self.pidxunicode == other.pidxunicode && self.grbit == other.grbit && self.prgcolumnid == other.prgcolumnid && self.cbKeyMost == other.cbKeyMost && self.cbVarSegMac == other.cbVarSegMac && self.tableid == other.tableid
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::Eq for JET_OPENTEMPORARYTABLE2 {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::fmt::Debug for JET_OPENTEMPORARYTABLE2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_OPENTEMPORARYTABLE2").field("cbStruct", &self.cbStruct).field("prgcolumndef", &self.prgcolumndef).field("ccolumn", &self.ccolumn).field("pidxunicode", &self.pidxunicode).field("grbit", &self.grbit).field("prgcolumnid", &self.prgcolumnid).field("cbKeyMost", &self.cbKeyMost).field("cbVarSegMac", &self.cbVarSegMac).field("tableid", &self.tableid).finish()
    }
}
impl ::core::default::Default for JET_OPERATIONCONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JET_OPERATIONCONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.ulUserID == other.ulUserID && self.nOperationID == other.nOperationID && self.nOperationType == other.nOperationType && self.nClientType == other.nClientType && self.fFlags == other.fFlags
    }
}
impl ::core::cmp::Eq for JET_OPERATIONCONTEXT {}
impl ::core::fmt::Debug for JET_OPERATIONCONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_OPERATIONCONTEXT").field("ulUserID", &self.ulUserID).field("nOperationID", &self.nOperationID).field("nOperationType", &self.nOperationType).field("nClientType", &self.nClientType).field("fFlags", &self.fFlags).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JET_RBSINFOMISC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JET_RBSINFOMISC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JET_RBSREVERTINFOMISC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JET_RBSREVERTINFOMISC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::default::Default for JET_RECORDLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::PartialEq for JET_RECORDLIST {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.tableid == other.tableid && self.cRecord == other.cRecord && self.columnidBookmark == other.columnidBookmark
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::Eq for JET_RECORDLIST {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::fmt::Debug for JET_RECORDLIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_RECORDLIST").field("cbStruct", &self.cbStruct).field("tableid", &self.tableid).field("cRecord", &self.cRecord).field("columnidBookmark", &self.columnidBookmark).finish()
    }
}
impl ::core::default::Default for JET_RECPOS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JET_RECPOS {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.centriesLT == other.centriesLT && self.centriesInRange == other.centriesInRange && self.centriesTotal == other.centriesTotal
    }
}
impl ::core::cmp::Eq for JET_RECPOS {}
impl ::core::fmt::Debug for JET_RECPOS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_RECPOS").field("cbStruct", &self.cbStruct).field("centriesLT", &self.centriesLT).field("centriesInRange", &self.centriesInRange).field("centriesTotal", &self.centriesTotal).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for JET_RECSIZE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for JET_RECSIZE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for JET_RECSIZE2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for JET_RECSIZE2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for JET_RELOP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for JET_RELOP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JET_RELOP").field(&self.0).finish()
    }
}
impl ::core::default::Default for JET_RETINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JET_RETINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.ibLongValue == other.ibLongValue && self.itagSequence == other.itagSequence && self.columnidNextTagged == other.columnidNextTagged
    }
}
impl ::core::cmp::Eq for JET_RETINFO {}
impl ::core::fmt::Debug for JET_RETINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_RETINFO").field("cbStruct", &self.cbStruct).field("ibLongValue", &self.ibLongValue).field("itagSequence", &self.itagSequence).field("columnidNextTagged", &self.columnidNextTagged).finish()
    }
}
impl ::core::default::Default for JET_RETRIEVECOLUMN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JET_RETRIEVECOLUMN {
    fn eq(&self, other: &Self) -> bool {
        self.columnid == other.columnid && self.pvData == other.pvData && self.cbData == other.cbData && self.cbActual == other.cbActual && self.grbit == other.grbit && self.ibLongValue == other.ibLongValue && self.itagSequence == other.itagSequence && self.columnidNextTagged == other.columnidNextTagged && self.err == other.err
    }
}
impl ::core::cmp::Eq for JET_RETRIEVECOLUMN {}
impl ::core::fmt::Debug for JET_RETRIEVECOLUMN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_RETRIEVECOLUMN").field("columnid", &self.columnid).field("pvData", &self.pvData).field("cbData", &self.cbData).field("cbActual", &self.cbActual).field("grbit", &self.grbit).field("ibLongValue", &self.ibLongValue).field("itagSequence", &self.itagSequence).field("columnidNextTagged", &self.columnidNextTagged).field("err", &self.err).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
impl ::core::default::Default for JET_RSTINFO_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_StructuredStorage"))]
impl ::core::default::Default for JET_RSTINFO_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for JET_RSTMAP_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JET_RSTMAP_A {
    fn eq(&self, other: &Self) -> bool {
        self.szDatabaseName == other.szDatabaseName && self.szNewDatabaseName == other.szNewDatabaseName
    }
}
impl ::core::cmp::Eq for JET_RSTMAP_A {}
impl ::core::fmt::Debug for JET_RSTMAP_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_RSTMAP_A").field("szDatabaseName", &self.szDatabaseName).field("szNewDatabaseName", &self.szNewDatabaseName).finish()
    }
}
impl ::core::default::Default for JET_RSTMAP_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JET_RSTMAP_W {
    fn eq(&self, other: &Self) -> bool {
        self.szDatabaseName == other.szDatabaseName && self.szNewDatabaseName == other.szNewDatabaseName
    }
}
impl ::core::cmp::Eq for JET_RSTMAP_W {}
impl ::core::fmt::Debug for JET_RSTMAP_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_RSTMAP_W").field("szDatabaseName", &self.szDatabaseName).field("szNewDatabaseName", &self.szNewDatabaseName).finish()
    }
}
impl ::core::default::Default for JET_SETCOLUMN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JET_SETCOLUMN {
    fn eq(&self, other: &Self) -> bool {
        self.columnid == other.columnid && self.pvData == other.pvData && self.cbData == other.cbData && self.grbit == other.grbit && self.ibLongValue == other.ibLongValue && self.itagSequence == other.itagSequence && self.err == other.err
    }
}
impl ::core::cmp::Eq for JET_SETCOLUMN {}
impl ::core::fmt::Debug for JET_SETCOLUMN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_SETCOLUMN").field("columnid", &self.columnid).field("pvData", &self.pvData).field("cbData", &self.cbData).field("grbit", &self.grbit).field("ibLongValue", &self.ibLongValue).field("itagSequence", &self.itagSequence).field("err", &self.err).finish()
    }
}
impl ::core::default::Default for JET_SETINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JET_SETINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.ibLongValue == other.ibLongValue && self.itagSequence == other.itagSequence
    }
}
impl ::core::cmp::Eq for JET_SETINFO {}
impl ::core::fmt::Debug for JET_SETINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_SETINFO").field("cbStruct", &self.cbStruct).field("ibLongValue", &self.ibLongValue).field("itagSequence", &self.itagSequence).finish()
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::default::Default for JET_SETSYSPARAM_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::PartialEq for JET_SETSYSPARAM_A {
    fn eq(&self, other: &Self) -> bool {
        self.paramid == other.paramid && self.lParam == other.lParam && self.sz == other.sz && self.err == other.err
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::Eq for JET_SETSYSPARAM_A {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::fmt::Debug for JET_SETSYSPARAM_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_SETSYSPARAM_A").field("paramid", &self.paramid).field("lParam", &self.lParam).field("sz", &self.sz).field("err", &self.err).finish()
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::default::Default for JET_SETSYSPARAM_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::PartialEq for JET_SETSYSPARAM_W {
    fn eq(&self, other: &Self) -> bool {
        self.paramid == other.paramid && self.lParam == other.lParam && self.sz == other.sz && self.err == other.err
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::Eq for JET_SETSYSPARAM_W {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::fmt::Debug for JET_SETSYSPARAM_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_SETSYSPARAM_W").field("paramid", &self.paramid).field("lParam", &self.lParam).field("sz", &self.sz).field("err", &self.err).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JET_SIGNATURE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for JET_SNPROG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JET_SNPROG {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.cunitDone == other.cunitDone && self.cunitTotal == other.cunitTotal
    }
}
impl ::core::cmp::Eq for JET_SNPROG {}
impl ::core::fmt::Debug for JET_SNPROG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_SNPROG").field("cbStruct", &self.cbStruct).field("cunitDone", &self.cunitDone).field("cunitTotal", &self.cunitTotal).finish()
    }
}
impl ::core::default::Default for JET_SPACEHINTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JET_SPACEHINTS {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.ulInitialDensity == other.ulInitialDensity && self.cbInitial == other.cbInitial && self.grbit == other.grbit && self.ulMaintDensity == other.ulMaintDensity && self.ulGrowth == other.ulGrowth && self.cbMinExtent == other.cbMinExtent && self.cbMaxExtent == other.cbMaxExtent
    }
}
impl ::core::cmp::Eq for JET_SPACEHINTS {}
impl ::core::fmt::Debug for JET_SPACEHINTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_SPACEHINTS").field("cbStruct", &self.cbStruct).field("ulInitialDensity", &self.ulInitialDensity).field("cbInitial", &self.cbInitial).field("grbit", &self.grbit).field("ulMaintDensity", &self.ulMaintDensity).field("ulGrowth", &self.ulGrowth).field("cbMinExtent", &self.cbMinExtent).field("cbMaxExtent", &self.cbMaxExtent).finish()
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::default::Default for JET_TABLECREATE2_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::PartialEq for JET_TABLECREATE2_A {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.szTableName == other.szTableName && self.szTemplateTableName == other.szTemplateTableName && self.ulPages == other.ulPages && self.ulDensity == other.ulDensity && self.rgcolumncreate == other.rgcolumncreate && self.cColumns == other.cColumns && self.rgindexcreate == other.rgindexcreate && self.cIndexes == other.cIndexes && self.szCallback == other.szCallback && self.cbtyp == other.cbtyp && self.grbit == other.grbit && self.tableid == other.tableid && self.cCreated == other.cCreated
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::Eq for JET_TABLECREATE2_A {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::fmt::Debug for JET_TABLECREATE2_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_TABLECREATE2_A")
            .field("cbStruct", &self.cbStruct)
            .field("szTableName", &self.szTableName)
            .field("szTemplateTableName", &self.szTemplateTableName)
            .field("ulPages", &self.ulPages)
            .field("ulDensity", &self.ulDensity)
            .field("rgcolumncreate", &self.rgcolumncreate)
            .field("cColumns", &self.cColumns)
            .field("rgindexcreate", &self.rgindexcreate)
            .field("cIndexes", &self.cIndexes)
            .field("szCallback", &self.szCallback)
            .field("cbtyp", &self.cbtyp)
            .field("grbit", &self.grbit)
            .field("tableid", &self.tableid)
            .field("cCreated", &self.cCreated)
            .finish()
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::default::Default for JET_TABLECREATE2_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::PartialEq for JET_TABLECREATE2_W {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.szTableName == other.szTableName && self.szTemplateTableName == other.szTemplateTableName && self.ulPages == other.ulPages && self.ulDensity == other.ulDensity && self.rgcolumncreate == other.rgcolumncreate && self.cColumns == other.cColumns && self.rgindexcreate == other.rgindexcreate && self.cIndexes == other.cIndexes && self.szCallback == other.szCallback && self.cbtyp == other.cbtyp && self.grbit == other.grbit && self.tableid == other.tableid && self.cCreated == other.cCreated
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::Eq for JET_TABLECREATE2_W {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::fmt::Debug for JET_TABLECREATE2_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_TABLECREATE2_W")
            .field("cbStruct", &self.cbStruct)
            .field("szTableName", &self.szTableName)
            .field("szTemplateTableName", &self.szTemplateTableName)
            .field("ulPages", &self.ulPages)
            .field("ulDensity", &self.ulDensity)
            .field("rgcolumncreate", &self.rgcolumncreate)
            .field("cColumns", &self.cColumns)
            .field("rgindexcreate", &self.rgindexcreate)
            .field("cIndexes", &self.cIndexes)
            .field("szCallback", &self.szCallback)
            .field("cbtyp", &self.cbtyp)
            .field("grbit", &self.grbit)
            .field("tableid", &self.tableid)
            .field("cCreated", &self.cCreated)
            .finish()
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::default::Default for JET_TABLECREATE3_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::PartialEq for JET_TABLECREATE3_A {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.szTableName == other.szTableName && self.szTemplateTableName == other.szTemplateTableName && self.ulPages == other.ulPages && self.ulDensity == other.ulDensity && self.rgcolumncreate == other.rgcolumncreate && self.cColumns == other.cColumns && self.rgindexcreate == other.rgindexcreate && self.cIndexes == other.cIndexes && self.szCallback == other.szCallback && self.cbtyp == other.cbtyp && self.grbit == other.grbit && self.pSeqSpacehints == other.pSeqSpacehints && self.pLVSpacehints == other.pLVSpacehints && self.cbSeparateLV == other.cbSeparateLV && self.tableid == other.tableid && self.cCreated == other.cCreated
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::Eq for JET_TABLECREATE3_A {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::fmt::Debug for JET_TABLECREATE3_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_TABLECREATE3_A")
            .field("cbStruct", &self.cbStruct)
            .field("szTableName", &self.szTableName)
            .field("szTemplateTableName", &self.szTemplateTableName)
            .field("ulPages", &self.ulPages)
            .field("ulDensity", &self.ulDensity)
            .field("rgcolumncreate", &self.rgcolumncreate)
            .field("cColumns", &self.cColumns)
            .field("rgindexcreate", &self.rgindexcreate)
            .field("cIndexes", &self.cIndexes)
            .field("szCallback", &self.szCallback)
            .field("cbtyp", &self.cbtyp)
            .field("grbit", &self.grbit)
            .field("pSeqSpacehints", &self.pSeqSpacehints)
            .field("pLVSpacehints", &self.pLVSpacehints)
            .field("cbSeparateLV", &self.cbSeparateLV)
            .field("tableid", &self.tableid)
            .field("cCreated", &self.cCreated)
            .finish()
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::default::Default for JET_TABLECREATE3_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::PartialEq for JET_TABLECREATE3_W {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.szTableName == other.szTableName && self.szTemplateTableName == other.szTemplateTableName && self.ulPages == other.ulPages && self.ulDensity == other.ulDensity && self.rgcolumncreate == other.rgcolumncreate && self.cColumns == other.cColumns && self.rgindexcreate == other.rgindexcreate && self.cIndexes == other.cIndexes && self.szCallback == other.szCallback && self.cbtyp == other.cbtyp && self.grbit == other.grbit && self.pSeqSpacehints == other.pSeqSpacehints && self.pLVSpacehints == other.pLVSpacehints && self.cbSeparateLV == other.cbSeparateLV && self.tableid == other.tableid && self.cCreated == other.cCreated
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::Eq for JET_TABLECREATE3_W {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::fmt::Debug for JET_TABLECREATE3_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_TABLECREATE3_W")
            .field("cbStruct", &self.cbStruct)
            .field("szTableName", &self.szTableName)
            .field("szTemplateTableName", &self.szTemplateTableName)
            .field("ulPages", &self.ulPages)
            .field("ulDensity", &self.ulDensity)
            .field("rgcolumncreate", &self.rgcolumncreate)
            .field("cColumns", &self.cColumns)
            .field("rgindexcreate", &self.rgindexcreate)
            .field("cIndexes", &self.cIndexes)
            .field("szCallback", &self.szCallback)
            .field("cbtyp", &self.cbtyp)
            .field("grbit", &self.grbit)
            .field("pSeqSpacehints", &self.pSeqSpacehints)
            .field("pLVSpacehints", &self.pLVSpacehints)
            .field("cbSeparateLV", &self.cbSeparateLV)
            .field("tableid", &self.tableid)
            .field("cCreated", &self.cCreated)
            .finish()
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::default::Default for JET_TABLECREATE4_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::PartialEq for JET_TABLECREATE4_A {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.szTableName == other.szTableName && self.szTemplateTableName == other.szTemplateTableName && self.ulPages == other.ulPages && self.ulDensity == other.ulDensity && self.rgcolumncreate == other.rgcolumncreate && self.cColumns == other.cColumns && self.rgindexcreate == other.rgindexcreate && self.cIndexes == other.cIndexes && self.szCallback == other.szCallback && self.cbtyp == other.cbtyp && self.grbit == other.grbit && self.pSeqSpacehints == other.pSeqSpacehints && self.pLVSpacehints == other.pLVSpacehints && self.cbSeparateLV == other.cbSeparateLV && self.tableid == other.tableid && self.cCreated == other.cCreated
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::Eq for JET_TABLECREATE4_A {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::fmt::Debug for JET_TABLECREATE4_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_TABLECREATE4_A")
            .field("cbStruct", &self.cbStruct)
            .field("szTableName", &self.szTableName)
            .field("szTemplateTableName", &self.szTemplateTableName)
            .field("ulPages", &self.ulPages)
            .field("ulDensity", &self.ulDensity)
            .field("rgcolumncreate", &self.rgcolumncreate)
            .field("cColumns", &self.cColumns)
            .field("rgindexcreate", &self.rgindexcreate)
            .field("cIndexes", &self.cIndexes)
            .field("szCallback", &self.szCallback)
            .field("cbtyp", &self.cbtyp)
            .field("grbit", &self.grbit)
            .field("pSeqSpacehints", &self.pSeqSpacehints)
            .field("pLVSpacehints", &self.pLVSpacehints)
            .field("cbSeparateLV", &self.cbSeparateLV)
            .field("tableid", &self.tableid)
            .field("cCreated", &self.cCreated)
            .finish()
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::default::Default for JET_TABLECREATE4_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::PartialEq for JET_TABLECREATE4_W {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.szTableName == other.szTableName && self.szTemplateTableName == other.szTemplateTableName && self.ulPages == other.ulPages && self.ulDensity == other.ulDensity && self.rgcolumncreate == other.rgcolumncreate && self.cColumns == other.cColumns && self.rgindexcreate == other.rgindexcreate && self.cIndexes == other.cIndexes && self.szCallback == other.szCallback && self.cbtyp == other.cbtyp && self.grbit == other.grbit && self.pSeqSpacehints == other.pSeqSpacehints && self.pLVSpacehints == other.pLVSpacehints && self.cbSeparateLV == other.cbSeparateLV && self.tableid == other.tableid && self.cCreated == other.cCreated
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::Eq for JET_TABLECREATE4_W {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::fmt::Debug for JET_TABLECREATE4_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_TABLECREATE4_W")
            .field("cbStruct", &self.cbStruct)
            .field("szTableName", &self.szTableName)
            .field("szTemplateTableName", &self.szTemplateTableName)
            .field("ulPages", &self.ulPages)
            .field("ulDensity", &self.ulDensity)
            .field("rgcolumncreate", &self.rgcolumncreate)
            .field("cColumns", &self.cColumns)
            .field("rgindexcreate", &self.rgindexcreate)
            .field("cIndexes", &self.cIndexes)
            .field("szCallback", &self.szCallback)
            .field("cbtyp", &self.cbtyp)
            .field("grbit", &self.grbit)
            .field("pSeqSpacehints", &self.pSeqSpacehints)
            .field("pLVSpacehints", &self.pLVSpacehints)
            .field("cbSeparateLV", &self.cbSeparateLV)
            .field("tableid", &self.tableid)
            .field("cCreated", &self.cCreated)
            .finish()
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::default::Default for JET_TABLECREATE_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::PartialEq for JET_TABLECREATE_A {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.szTableName == other.szTableName && self.szTemplateTableName == other.szTemplateTableName && self.ulPages == other.ulPages && self.ulDensity == other.ulDensity && self.rgcolumncreate == other.rgcolumncreate && self.cColumns == other.cColumns && self.rgindexcreate == other.rgindexcreate && self.cIndexes == other.cIndexes && self.grbit == other.grbit && self.tableid == other.tableid && self.cCreated == other.cCreated
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::Eq for JET_TABLECREATE_A {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::fmt::Debug for JET_TABLECREATE_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_TABLECREATE_A")
            .field("cbStruct", &self.cbStruct)
            .field("szTableName", &self.szTableName)
            .field("szTemplateTableName", &self.szTemplateTableName)
            .field("ulPages", &self.ulPages)
            .field("ulDensity", &self.ulDensity)
            .field("rgcolumncreate", &self.rgcolumncreate)
            .field("cColumns", &self.cColumns)
            .field("rgindexcreate", &self.rgindexcreate)
            .field("cIndexes", &self.cIndexes)
            .field("grbit", &self.grbit)
            .field("tableid", &self.tableid)
            .field("cCreated", &self.cCreated)
            .finish()
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::default::Default for JET_TABLECREATE_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::PartialEq for JET_TABLECREATE_W {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.szTableName == other.szTableName && self.szTemplateTableName == other.szTemplateTableName && self.ulPages == other.ulPages && self.ulDensity == other.ulDensity && self.rgcolumncreate == other.rgcolumncreate && self.cColumns == other.cColumns && self.rgindexcreate == other.rgindexcreate && self.cIndexes == other.cIndexes && self.grbit == other.grbit && self.tableid == other.tableid && self.cCreated == other.cCreated
    }
}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::cmp::Eq for JET_TABLECREATE_W {}
#[cfg(feature = "Win32_Storage_StructuredStorage")]
impl ::core::fmt::Debug for JET_TABLECREATE_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_TABLECREATE_W")
            .field("cbStruct", &self.cbStruct)
            .field("szTableName", &self.szTableName)
            .field("szTemplateTableName", &self.szTemplateTableName)
            .field("ulPages", &self.ulPages)
            .field("ulDensity", &self.ulDensity)
            .field("rgcolumncreate", &self.rgcolumncreate)
            .field("cColumns", &self.cColumns)
            .field("rgindexcreate", &self.rgindexcreate)
            .field("cIndexes", &self.cIndexes)
            .field("grbit", &self.grbit)
            .field("tableid", &self.tableid)
            .field("cCreated", &self.cCreated)
            .finish()
    }
}
impl ::core::default::Default for JET_THREADSTATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JET_THREADSTATS {
    fn eq(&self, other: &Self) -> bool {
        self.cbStruct == other.cbStruct && self.cPageReferenced == other.cPageReferenced && self.cPageRead == other.cPageRead && self.cPagePreread == other.cPagePreread && self.cPageDirtied == other.cPageDirtied && self.cPageRedirtied == other.cPageRedirtied && self.cLogRecord == other.cLogRecord && self.cbLogRecord == other.cbLogRecord
    }
}
impl ::core::cmp::Eq for JET_THREADSTATS {}
impl ::core::fmt::Debug for JET_THREADSTATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_THREADSTATS").field("cbStruct", &self.cbStruct).field("cPageReferenced", &self.cPageReferenced).field("cPageRead", &self.cPageRead).field("cPagePreread", &self.cPagePreread).field("cPageDirtied", &self.cPageDirtied).field("cPageRedirtied", &self.cPageRedirtied).field("cLogRecord", &self.cLogRecord).field("cbLogRecord", &self.cbLogRecord).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for JET_THREADSTATS2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for JET_THREADSTATS2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for JET_TUPLELIMITS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JET_TUPLELIMITS {
    fn eq(&self, other: &Self) -> bool {
        self.chLengthMin == other.chLengthMin && self.chLengthMax == other.chLengthMax && self.chToIndexMax == other.chToIndexMax && self.cchIncrement == other.cchIncrement && self.ichStart == other.ichStart
    }
}
impl ::core::cmp::Eq for JET_TUPLELIMITS {}
impl ::core::fmt::Debug for JET_TUPLELIMITS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_TUPLELIMITS").field("chLengthMin", &self.chLengthMin).field("chLengthMax", &self.chLengthMax).field("chToIndexMax", &self.chToIndexMax).field("cchIncrement", &self.cchIncrement).field("ichStart", &self.ichStart).finish()
    }
}
impl ::core::default::Default for JET_UNICODEINDEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JET_UNICODEINDEX {
    fn eq(&self, other: &Self) -> bool {
        self.lcid == other.lcid && self.dwMapFlags == other.dwMapFlags
    }
}
impl ::core::cmp::Eq for JET_UNICODEINDEX {}
impl ::core::fmt::Debug for JET_UNICODEINDEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_UNICODEINDEX").field("lcid", &self.lcid).field("dwMapFlags", &self.dwMapFlags).finish()
    }
}
impl ::core::default::Default for JET_UNICODEINDEX2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JET_UNICODEINDEX2 {
    fn eq(&self, other: &Self) -> bool {
        self.szLocaleName == other.szLocaleName && self.dwMapFlags == other.dwMapFlags
    }
}
impl ::core::cmp::Eq for JET_UNICODEINDEX2 {}
impl ::core::fmt::Debug for JET_UNICODEINDEX2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_UNICODEINDEX2").field("szLocaleName", &self.szLocaleName).field("dwMapFlags", &self.dwMapFlags).finish()
    }
}
impl ::core::default::Default for JET_USERDEFINEDDEFAULT_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JET_USERDEFINEDDEFAULT_A {
    fn eq(&self, other: &Self) -> bool {
        self.szCallback == other.szCallback && self.pbUserData == other.pbUserData && self.cbUserData == other.cbUserData && self.szDependantColumns == other.szDependantColumns
    }
}
impl ::core::cmp::Eq for JET_USERDEFINEDDEFAULT_A {}
impl ::core::fmt::Debug for JET_USERDEFINEDDEFAULT_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_USERDEFINEDDEFAULT_A").field("szCallback", &self.szCallback).field("pbUserData", &self.pbUserData).field("cbUserData", &self.cbUserData).field("szDependantColumns", &self.szDependantColumns).finish()
    }
}
impl ::core::default::Default for JET_USERDEFINEDDEFAULT_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for JET_USERDEFINEDDEFAULT_W {
    fn eq(&self, other: &Self) -> bool {
        self.szCallback == other.szCallback && self.pbUserData == other.pbUserData && self.cbUserData == other.cbUserData && self.szDependantColumns == other.szDependantColumns
    }
}
impl ::core::cmp::Eq for JET_USERDEFINEDDEFAULT_W {}
impl ::core::fmt::Debug for JET_USERDEFINEDDEFAULT_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JET_USERDEFINEDDEFAULT_W").field("szCallback", &self.szCallback).field("pbUserData", &self.pbUserData).field("cbUserData", &self.cbUserData).field("szDependantColumns", &self.szDependantColumns).finish()
    }
}

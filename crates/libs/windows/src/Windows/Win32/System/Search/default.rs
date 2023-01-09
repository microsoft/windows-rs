impl ::core::default::Default for ACCESS_MASKENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ACCESS_MASKENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ACCESS_MASKENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for AUTHENTICATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for AUTHENTICATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.atAuthenticationType == other.atAuthenticationType && self.pcwszUser == other.pcwszUser && self.pcwszPassword == other.pcwszPassword
    }
}
impl ::core::cmp::Eq for AUTHENTICATION_INFO {}
impl ::core::fmt::Debug for AUTHENTICATION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUTHENTICATION_INFO").field("dwSize", &self.dwSize).field("atAuthenticationType", &self.atAuthenticationType).field("pcwszUser", &self.pcwszUser).field("pcwszPassword", &self.pcwszPassword).finish()
    }
}
impl ::core::default::Default for AUTH_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AUTH_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUTH_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for BUCKETCATEGORIZE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BUCKETCATEGORIZE {
    fn eq(&self, other: &Self) -> bool {
        self.cBuckets == other.cBuckets && self.Distribution == other.Distribution
    }
}
impl ::core::cmp::Eq for BUCKETCATEGORIZE {}
impl ::core::fmt::Debug for BUCKETCATEGORIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BUCKETCATEGORIZE").field("cBuckets", &self.cBuckets).field("Distribution", &self.Distribution).finish()
    }
}
impl ::core::default::Default for CASE_REQUIREMENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CASE_REQUIREMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CASE_REQUIREMENT").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for CATEGORIZATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for CATEGORIZATIONSET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::PartialEq for CATEGORIZATIONSET {
    fn eq(&self, other: &Self) -> bool {
        self.cCat == other.cCat && self.aCat == other.aCat
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::Eq for CATEGORIZATIONSET {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::fmt::Debug for CATEGORIZATIONSET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CATEGORIZATIONSET").field("cCat", &self.cCat).field("aCat", &self.aCat).finish()
    }
}
impl ::core::default::Default for CHANNEL_AGENT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CHANNEL_AGENT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CHANNEL_AGENT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for CLUSION_REASON {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CLUSION_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLUSION_REASON").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for COLUMNSET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::PartialEq for COLUMNSET {
    fn eq(&self, other: &Self) -> bool {
        self.cCol == other.cCol && self.aCol == other.aCol
    }
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::Eq for COLUMNSET {}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::fmt::Debug for COLUMNSET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COLUMNSET").field("cCol", &self.cCol).field("aCol", &self.aCol).finish()
    }
}
impl ::core::default::Default for CONDITION_CREATION_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CONDITION_CREATION_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CONDITION_CREATION_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CONDITION_CREATION_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CONDITION_CREATION_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CONDITION_CREATION_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CONDITION_CREATION_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CONDITION_CREATION_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for CONTENTRESTRICTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CREATESUBSCRIPTIONFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CREATESUBSCRIPTIONFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CREATESUBSCRIPTIONFLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for CatalogPausedReason {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CatalogPausedReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CatalogPausedReason").field(&self.0).finish()
    }
}
impl ::core::default::Default for CatalogStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CatalogStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CatalogStatus").field(&self.0).finish()
    }
}
impl ::core::default::Default for DATE_STRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DATE_STRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.year == other.year && self.month == other.month && self.day == other.day
    }
}
impl ::core::cmp::Eq for DATE_STRUCT {}
impl ::core::fmt::Debug for DATE_STRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DATE_STRUCT").field("year", &self.year).field("month", &self.month).field("day", &self.day).finish()
    }
}
impl ::core::default::Default for DBACCESSORFLAGSENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBACCESSORFLAGSENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBACCESSORFLAGSENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBASYNCHOPENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBASYNCHOPENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBASYNCHOPENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBASYNCHPHASEENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBASYNCHPHASEENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBASYNCHPHASEENUM").field(&self.0).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for DBBINDEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for DBBINDEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DBBINDFLAGENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBBINDFLAGENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBBINDFLAGENUM").field(&self.0).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for DBBINDING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for DBBINDING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DBBINDSTATUSENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBBINDSTATUSENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBBINDSTATUSENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBBINDURLFLAGENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBBINDURLFLAGENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBBINDURLFLAGENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBBINDURLSTATUSENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBBINDURLSTATUSENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBBINDURLSTATUSENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBBOOKMARK {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBBOOKMARK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBBOOKMARK").field(&self.0).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Storage_IndexServer")]
impl ::core::default::Default for DBCOLUMNACCESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Storage_IndexServer")]
impl ::core::default::Default for DBCOLUMNACCESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::default::Default for DBCOLUMNDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::default::Default for DBCOLUMNDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DBCOLUMNDESCFLAGSENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBCOLUMNDESCFLAGSENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBCOLUMNDESCFLAGSENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBCOLUMNFLAGS15ENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBCOLUMNFLAGS15ENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBCOLUMNFLAGS15ENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBCOLUMNFLAGSENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBCOLUMNFLAGSENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBCOLUMNFLAGSENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBCOLUMNFLAGSENUM20 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBCOLUMNFLAGSENUM20 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBCOLUMNFLAGSENUM20").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBCOLUMNFLAGSENUM21 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBCOLUMNFLAGSENUM21 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBCOLUMNFLAGSENUM21").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBCOLUMNFLAGSENUM26 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBCOLUMNFLAGSENUM26 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBCOLUMNFLAGSENUM26").field(&self.0).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
impl ::core::default::Default for DBCOLUMNINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
impl ::core::default::Default for DBCOLUMNINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DBCOMMANDPERSISTFLAGENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBCOMMANDPERSISTFLAGENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBCOMMANDPERSISTFLAGENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBCOMMANDPERSISTFLAGENUM21 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBCOMMANDPERSISTFLAGENUM21 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBCOMMANDPERSISTFLAGENUM21").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBCOMPAREENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBCOMPAREENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBCOMPAREENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBCOMPAREOPSENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBCOMPAREOPSENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBCOMPAREOPSENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBCOMPAREOPSENUM20 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBCOMPAREOPSENUM20 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBCOMPAREOPSENUM20").field(&self.0).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::default::Default for DBCONSTRAINTDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::default::Default for DBCONSTRAINTDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DBCONSTRAINTTYPEENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBCONSTRAINTTYPEENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBCONSTRAINTTYPEENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBCONVERTFLAGSENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBCONVERTFLAGSENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBCONVERTFLAGSENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBCONVERTFLAGSENUM20 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBCONVERTFLAGSENUM20 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBCONVERTFLAGSENUM20").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBCOPYFLAGSENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBCOPYFLAGSENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBCOPYFLAGSENUM").field(&self.0).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for DBCOST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for DBCOST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DBCOSTUNITENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBCOSTUNITENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBCOSTUNITENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBDATACONVERTENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBDATACONVERTENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBDATACONVERTENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBDATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DBDATE {
    fn eq(&self, other: &Self) -> bool {
        self.year == other.year && self.month == other.month && self.day == other.day
    }
}
impl ::core::cmp::Eq for DBDATE {}
impl ::core::fmt::Debug for DBDATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DBDATE").field("year", &self.year).field("month", &self.month).field("day", &self.day).finish()
    }
}
impl ::core::default::Default for DBDATETIM4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DBDATETIM4 {
    fn eq(&self, other: &Self) -> bool {
        self.numdays == other.numdays && self.nummins == other.nummins
    }
}
impl ::core::cmp::Eq for DBDATETIM4 {}
impl ::core::fmt::Debug for DBDATETIM4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DBDATETIM4").field("numdays", &self.numdays).field("nummins", &self.nummins).finish()
    }
}
impl ::core::default::Default for DBDATETIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DBDATETIME {
    fn eq(&self, other: &Self) -> bool {
        self.dtdays == other.dtdays && self.dttime == other.dttime
    }
}
impl ::core::cmp::Eq for DBDATETIME {}
impl ::core::fmt::Debug for DBDATETIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DBDATETIME").field("dtdays", &self.dtdays).field("dttime", &self.dttime).finish()
    }
}
impl ::core::default::Default for DBDEFERRABILITYENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBDEFERRABILITYENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBDEFERRABILITYENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBDELETEFLAGSENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBDELETEFLAGSENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBDELETEFLAGSENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBEVENTPHASEENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBEVENTPHASEENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBEVENTPHASEENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBEXECLIMITSENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBEXECLIMITSENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBEXECLIMITSENUM").field(&self.0).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for DBFAILUREINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for DBFAILUREINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for DBIMPLICITSESSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for DBIMPLICITSESSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Storage_IndexServer")]
impl ::core::default::Default for DBINDEXCOLUMNDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Storage_IndexServer")]
impl ::core::default::Default for DBINDEXCOLUMNDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DBINDEX_COL_ORDERENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBINDEX_COL_ORDERENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBINDEX_COL_ORDERENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBLITERALENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBLITERALENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBLITERALENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBLITERALENUM20 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBLITERALENUM20 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBLITERALENUM20").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBLITERALENUM21 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBLITERALENUM21 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBLITERALENUM21").field(&self.0).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DBLITERALINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DBLITERALINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DBMATCHTYPEENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBMATCHTYPEENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBMATCHTYPEENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBMEMOWNERENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBMEMOWNERENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBMEMOWNERENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBMONEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DBMONEY {
    fn eq(&self, other: &Self) -> bool {
        self.mnyhigh == other.mnyhigh && self.mnylow == other.mnylow
    }
}
impl ::core::cmp::Eq for DBMONEY {}
impl ::core::fmt::Debug for DBMONEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DBMONEY").field("mnyhigh", &self.mnyhigh).field("mnylow", &self.mnylow).finish()
    }
}
impl ::core::default::Default for DBMOVEFLAGSENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBMOVEFLAGSENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBMOVEFLAGSENUM").field(&self.0).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for DBOBJECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for DBOBJECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for DBPARAMBINDINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for DBPARAMBINDINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DBPARAMFLAGSENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBPARAMFLAGSENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBPARAMFLAGSENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBPARAMFLAGSENUM20 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBPARAMFLAGSENUM20 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBPARAMFLAGSENUM20").field(&self.0).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for DBPARAMINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for DBPARAMINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DBPARAMIOENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBPARAMIOENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBPARAMIOENUM").field(&self.0).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for DBPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for DBPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DBPARTENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBPARTENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBPARTENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBPENDINGSTATUSENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBPENDINGSTATUSENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBPENDINGSTATUSENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBPOSITIONFLAGSENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBPOSITIONFLAGSENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBPOSITIONFLAGSENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBPROMPTOPTIONSENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBPROMPTOPTIONSENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBPROMPTOPTIONSENUM").field(&self.0).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::default::Default for DBPROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::default::Default for DBPROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DBPROPENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBPROPENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBPROPENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBPROPENUM15 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBPROPENUM15 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBPROPENUM15").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBPROPENUM20 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBPROPENUM20 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBPROPENUM20").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBPROPENUM21 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBPROPENUM21 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBPROPENUM21").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBPROPENUM25 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBPROPENUM25 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBPROPENUM25").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBPROPENUM25_DEPRECATED {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBPROPENUM25_DEPRECATED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBPROPENUM25_DEPRECATED").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBPROPENUM26 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBPROPENUM26 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBPROPENUM26").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBPROPFLAGSENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBPROPFLAGSENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBPROPFLAGSENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBPROPFLAGSENUM21 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBPROPFLAGSENUM21 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBPROPFLAGSENUM21").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBPROPFLAGSENUM25 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBPROPFLAGSENUM25 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBPROPFLAGSENUM25").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBPROPFLAGSENUM26 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBPROPFLAGSENUM26 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBPROPFLAGSENUM26").field(&self.0).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for DBPROPIDSET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for DBPROPIDSET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::default::Default for DBPROPINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::default::Default for DBPROPINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::default::Default for DBPROPINFOSET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::default::Default for DBPROPINFOSET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DBPROPOPTIONSENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBPROPOPTIONSENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBPROPOPTIONSENUM").field(&self.0).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::default::Default for DBPROPSET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::default::Default for DBPROPSET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DBPROPSTATUSENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBPROPSTATUSENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBPROPSTATUSENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBPROPSTATUSENUM21 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBPROPSTATUSENUM21 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBPROPSTATUSENUM21").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBRANGEENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBRANGEENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBRANGEENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBRANGEENUM20 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBRANGEENUM20 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBRANGEENUM20").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBREASONENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBREASONENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBREASONENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBREASONENUM15 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBREASONENUM15 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBREASONENUM15").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBREASONENUM25 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBREASONENUM25 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBREASONENUM25").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBRESOURCEKINDENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBRESOURCEKINDENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBRESOURCEKINDENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBRESULTFLAGENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBRESULTFLAGENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBRESULTFLAGENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBROWCHANGEKINDENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBROWCHANGEKINDENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBROWCHANGEKINDENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBROWSTATUSENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBROWSTATUSENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBROWSTATUSENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBROWSTATUSENUM20 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBROWSTATUSENUM20 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBROWSTATUSENUM20").field(&self.0).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for DBROWWATCHCHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for DBROWWATCHCHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DBSEEKENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBSEEKENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBSEEKENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBSORTENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBSORTENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBSORTENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBSOURCETYPEENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBSOURCETYPEENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBSOURCETYPEENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBSOURCETYPEENUM20 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBSOURCETYPEENUM20 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBSOURCETYPEENUM20").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBSOURCETYPEENUM25 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBSOURCETYPEENUM25 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBSOURCETYPEENUM25").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBSTATUSENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBSTATUSENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBSTATUSENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBSTATUSENUM20 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBSTATUSENUM20 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBSTATUSENUM20").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBSTATUSENUM21 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBSTATUSENUM21 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBSTATUSENUM21").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBSTATUSENUM25 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBSTATUSENUM25 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBSTATUSENUM25").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBSTATUSENUM26 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBSTATUSENUM26 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBSTATUSENUM26").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBTABLESTATISTICSTYPE26 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBTABLESTATISTICSTYPE26 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBTABLESTATISTICSTYPE26").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBTIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DBTIME {
    fn eq(&self, other: &Self) -> bool {
        self.hour == other.hour && self.minute == other.minute && self.second == other.second
    }
}
impl ::core::cmp::Eq for DBTIME {}
impl ::core::fmt::Debug for DBTIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DBTIME").field("hour", &self.hour).field("minute", &self.minute).field("second", &self.second).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for DBTIMESTAMP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for DBTIMESTAMP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DBTYPEENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBTYPEENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBTYPEENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBTYPEENUM15 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBTYPEENUM15 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBTYPEENUM15").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBTYPEENUM20 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBTYPEENUM20 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBTYPEENUM20").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBUPDELRULEENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBUPDELRULEENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBUPDELRULEENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBVARYBIN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DBVARYBIN {
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len && self.array == other.array
    }
}
impl ::core::cmp::Eq for DBVARYBIN {}
impl ::core::fmt::Debug for DBVARYBIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DBVARYBIN").field("len", &self.len).field("array", &self.array).finish()
    }
}
impl ::core::default::Default for DBVARYCHAR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DBVARYCHAR {
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len && self.str == other.str
    }
}
impl ::core::cmp::Eq for DBVARYCHAR {}
impl ::core::fmt::Debug for DBVARYCHAR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DBVARYCHAR").field("len", &self.len).field("str", &self.str).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for DBVECTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for DBVECTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DBWATCHMODEENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBWATCHMODEENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBWATCHMODEENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for DBWATCHNOTIFYENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DBWATCHNOTIFYENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DBWATCHNOTIFYENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for DB_NUMERIC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DB_NUMERIC {
    fn eq(&self, other: &Self) -> bool {
        self.precision == other.precision && self.scale == other.scale && self.sign == other.sign && self.val == other.val
    }
}
impl ::core::cmp::Eq for DB_NUMERIC {}
impl ::core::fmt::Debug for DB_NUMERIC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DB_NUMERIC").field("precision", &self.precision).field("scale", &self.scale).field("sign", &self.sign).field("val", &self.val).finish()
    }
}
impl ::core::default::Default for DB_VARNUMERIC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DB_VARNUMERIC {
    fn eq(&self, other: &Self) -> bool {
        self.precision == other.precision && self.scale == other.scale && self.sign == other.sign && self.val == other.val
    }
}
impl ::core::cmp::Eq for DB_VARNUMERIC {}
impl ::core::fmt::Debug for DB_VARNUMERIC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DB_VARNUMERIC").field("precision", &self.precision).field("scale", &self.scale).field("sign", &self.sign).field("val", &self.val).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::default::Default for DCINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DCINFOTYPEENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DCINFOTYPEENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DCINFOTYPEENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for DELIVERY_AGENT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DELIVERY_AGENT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DELIVERY_AGENT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for DataSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DataSource {}
impl ::core::fmt::Debug for DataSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataSource").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for DataSourceListener {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DataSourceListener {}
impl ::core::fmt::Debug for DataSourceListener {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataSourceListener").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for DataSourceObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for DataSourceObject {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for DataSourceObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataSourceObject").field(&self.0).finish()
    }
}
impl ::core::default::Default for EBindInfoOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EBindInfoOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EBindInfoOptions").field(&self.0).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for ERRORINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for ERRORINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for FILTERED_DATA_SOURCES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILTERED_DATA_SOURCES {
    fn eq(&self, other: &Self) -> bool {
        self.pwcsExtension == other.pwcsExtension && self.pwcsMime == other.pwcsMime && self.pClsid == other.pClsid && self.pwcsOverride == other.pwcsOverride
    }
}
impl ::core::cmp::Eq for FILTERED_DATA_SOURCES {}
impl ::core::fmt::Debug for FILTERED_DATA_SOURCES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILTERED_DATA_SOURCES").field("pwcsExtension", &self.pwcsExtension).field("pwcsMime", &self.pwcsMime).field("pClsid", &self.pClsid).field("pwcsOverride", &self.pwcsOverride).finish()
    }
}
impl ::core::default::Default for FOLLOW_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FOLLOW_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FOLLOW_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for HITRANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HITRANGE {
    fn eq(&self, other: &Self) -> bool {
        self.iPosition == other.iPosition && self.cLength == other.cLength
    }
}
impl ::core::cmp::Eq for HITRANGE {}
impl ::core::fmt::Debug for HITRANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HITRANGE").field("iPosition", &self.iPosition).field("cLength", &self.cLength).finish()
    }
}
impl ::core::cmp::PartialEq for IAccessor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAccessor {}
impl ::core::fmt::Debug for IAccessor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAccessor").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAlterIndex {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAlterIndex {}
impl ::core::fmt::Debug for IAlterIndex {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAlterIndex").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IAlterTable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAlterTable {}
impl ::core::fmt::Debug for IAlterTable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAlterTable").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IBindResource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBindResource {}
impl ::core::fmt::Debug for IBindResource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBindResource").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IChapteredRowset {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IChapteredRowset {}
impl ::core::fmt::Debug for IChapteredRowset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IChapteredRowset").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IColumnMapper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IColumnMapper {}
impl ::core::fmt::Debug for IColumnMapper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IColumnMapper").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IColumnMapperCreator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IColumnMapperCreator {}
impl ::core::fmt::Debug for IColumnMapperCreator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IColumnMapperCreator").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IColumnsInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IColumnsInfo {}
impl ::core::fmt::Debug for IColumnsInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IColumnsInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IColumnsInfo2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IColumnsInfo2 {}
impl ::core::fmt::Debug for IColumnsInfo2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IColumnsInfo2").field(&self.0).finish()
    }
}
impl IColumnsInfo2 {
    #[doc = "*Required features: `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
    pub unsafe fn GetColumnInfo(&self, pccolumns: *mut usize, prginfo: *mut *mut DBCOLUMNINFO, ppstringsbuffer: ::core::option::Option<*mut *mut u16>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetColumnInfo)(::windows::core::Vtable::as_raw(self), pccolumns, prginfo, ::core::mem::transmute(ppstringsbuffer.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_IndexServer\"`*"]
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn MapColumnIDs(&self, ccolumnids: usize, rgcolumnids: ::core::option::Option<*const super::super::Storage::IndexServer::DBID>, rgcolumns: ::core::option::Option<*mut usize>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.MapColumnIDs)(::windows::core::Vtable::as_raw(self), ccolumnids, ::core::mem::transmute(rgcolumnids.unwrap_or(::std::ptr::null())), ::core::mem::transmute(rgcolumns.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
impl ::core::cmp::PartialEq for IColumnsRowset {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IColumnsRowset {}
impl ::core::fmt::Debug for IColumnsRowset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IColumnsRowset").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ICommand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICommand {}
impl ::core::fmt::Debug for ICommand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICommand").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ICommandCost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICommandCost {}
impl ::core::fmt::Debug for ICommandCost {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICommandCost").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ICommandPersist {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICommandPersist {}
impl ::core::fmt::Debug for ICommandPersist {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICommandPersist").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ICommandPrepare {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICommandPrepare {}
impl ::core::fmt::Debug for ICommandPrepare {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICommandPrepare").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ICommandProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICommandProperties {}
impl ::core::fmt::Debug for ICommandProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICommandProperties").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ICommandStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICommandStream {}
impl ::core::fmt::Debug for ICommandStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICommandStream").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ICommandText {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICommandText {}
impl ::core::fmt::Debug for ICommandText {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICommandText").field(&self.0).finish()
    }
}
impl ICommandText {
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Cancel)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Execute<P0>(&self, punkouter: P0, riid: *const ::windows::core::GUID, pparams: ::core::option::Option<*mut DBPARAMS>, pcrowsaffected: ::core::option::Option<*mut isize>, pprowset: ::core::option::Option<*mut ::core::option::Option<::windows::core::IUnknown>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Execute)(::windows::core::Vtable::as_raw(self), punkouter.into().abi(), riid, ::core::mem::transmute(pparams.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pcrowsaffected.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pprowset.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetDBSession(&self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDBSession)(::windows::core::Vtable::as_raw(self), riid, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ICommandValidate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICommandValidate {}
impl ::core::fmt::Debug for ICommandValidate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICommandValidate").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ICommandWithParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICommandWithParameters {}
impl ::core::fmt::Debug for ICommandWithParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICommandWithParameters").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICondition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICondition {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICondition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICondition").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ICondition {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetClassID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetClassID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IsDirty(&self) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).base__.IsDirty)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Load<P0>(&self, pstm: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IStream>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Load)(::windows::core::Vtable::as_raw(self), pstm.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Save<P0, P1>(&self, pstm: P0, fcleardirty: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IStream>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.Save)(::windows::core::Vtable::as_raw(self), pstm.into().abi(), fcleardirty.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSizeMax(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSizeMax)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICondition2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICondition2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICondition2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICondition2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ICondition2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetClassID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetClassID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IsDirty(&self) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).base__.base__.IsDirty)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Load<P0>(&self, pstm: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IStream>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Load)(::windows::core::Vtable::as_raw(self), pstm.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Save<P0, P1>(&self, pstm: P0, fcleardirty: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IStream>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Save)(::windows::core::Vtable::as_raw(self), pstm.into().abi(), fcleardirty.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSizeMax(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetSizeMax)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Search_Common\"`*"]
    #[cfg(feature = "Win32_System_Search_Common")]
    pub unsafe fn GetConditionType(&self) -> ::windows::core::Result<Common::CONDITION_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetConditionType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSubConditions<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSubConditions)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Search_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common"))]
    pub unsafe fn GetComparisonInfo(&self, ppszpropertyname: ::core::option::Option<*mut ::windows::core::PWSTR>, pcop: ::core::option::Option<*mut Common::CONDITION_OPERATION>, ppropvar: ::core::option::Option<*mut super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetComparisonInfo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppszpropertyname.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pcop.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppropvar.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetValueType(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetValueType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetValueNormalization(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetValueNormalization)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetInputTerms(&self, pppropertyterm: ::core::option::Option<*mut ::core::option::Option<IRichChunk>>, ppoperationterm: ::core::option::Option<*mut ::core::option::Option<IRichChunk>>, ppvalueterm: ::core::option::Option<*mut ::core::option::Option<IRichChunk>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetInputTerms)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pppropertyterm.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppoperationterm.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppvalueterm.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<ICondition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IConditionFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IConditionFactory {}
impl ::core::fmt::Debug for IConditionFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IConditionFactory").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IConditionFactory2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IConditionFactory2 {}
impl ::core::fmt::Debug for IConditionFactory2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IConditionFactory2").field(&self.0).finish()
    }
}
impl IConditionFactory2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn MakeNot<P0, P1>(&self, pcsub: P0, fsimplify: P1) -> ::windows::core::Result<ICondition>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ICondition>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.MakeNot)(::windows::core::Vtable::as_raw(self), pcsub.into().abi(), fsimplify.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Search_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Search_Common"))]
    pub unsafe fn MakeAndOr<P0, P1>(&self, ct: Common::CONDITION_TYPE, peusubs: P0, fsimplify: P1) -> ::windows::core::Result<ICondition>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IEnumUnknown>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.MakeAndOr)(::windows::core::Vtable::as_raw(self), ct, peusubs.into().abi(), fsimplify.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Search_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common"))]
    pub unsafe fn MakeLeaf<P0, P1, P2, P3, P4, P5>(&self, pszpropertyname: P0, cop: Common::CONDITION_OPERATION, pszvaluetype: P1, ppropvar: *const super::Com::StructuredStorage::PROPVARIANT, ppropertynameterm: P2, poperationterm: P3, pvalueterm: P4, fexpand: P5) -> ::windows::core::Result<ICondition>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<IRichChunk>>,
        P3: ::std::convert::Into<::windows::core::InParam<IRichChunk>>,
        P4: ::std::convert::Into<::windows::core::InParam<IRichChunk>>,
        P5: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.MakeLeaf)(::windows::core::Vtable::as_raw(self), pszpropertyname.into().abi(), cop, pszvaluetype.into().abi(), ppropvar, ppropertynameterm.into().abi(), poperationterm.into().abi(), pvalueterm.into().abi(), fexpand.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Resolve<P0>(&self, pc: P0, sqro: STRUCTURED_QUERY_RESOLVE_OPTION, pstreferencetime: ::core::option::Option<*const super::super::Foundation::SYSTEMTIME>) -> ::windows::core::Result<ICondition>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ICondition>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Resolve)(::windows::core::Vtable::as_raw(self), pc.into().abi(), sqro, ::core::mem::transmute(pstreferencetime.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IConditionGenerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IConditionGenerator {}
impl ::core::fmt::Debug for IConditionGenerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IConditionGenerator").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IConvertType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IConvertType {}
impl ::core::fmt::Debug for IConvertType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IConvertType").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ICreateRow {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICreateRow {}
impl ::core::fmt::Debug for ICreateRow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICreateRow").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDBAsynchNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDBAsynchNotify {}
impl ::core::fmt::Debug for IDBAsynchNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDBAsynchNotify").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDBAsynchStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDBAsynchStatus {}
impl ::core::fmt::Debug for IDBAsynchStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDBAsynchStatus").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDBBinderProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDBBinderProperties {}
impl ::core::fmt::Debug for IDBBinderProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDBBinderProperties").field(&self.0).finish()
    }
}
impl IDBBinderProperties {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProperties(&self, rgpropertyidsets: ::core::option::Option<&[DBPROPIDSET]>, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetProperties)(::windows::core::Vtable::as_raw(self), rgpropertyidsets.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(rgpropertyidsets.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pcpropertysets, prgpropertysets).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetPropertyInfo(&self, rgpropertyidsets: ::core::option::Option<&[DBPROPIDSET]>, pcpropertyinfosets: *mut u32, prgpropertyinfosets: *mut *mut DBPROPINFOSET, ppdescbuffer: ::core::option::Option<*mut *mut u16>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPropertyInfo)(::windows::core::Vtable::as_raw(self), rgpropertyidsets.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(rgpropertyidsets.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pcpropertyinfosets, prgpropertyinfosets, ::core::mem::transmute(ppdescbuffer.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetProperties(&self, rgpropertysets: ::core::option::Option<&mut [DBPROPSET]>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetProperties)(::windows::core::Vtable::as_raw(self), rgpropertysets.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(rgpropertysets.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr()))).ok()
    }
}
impl ::core::cmp::PartialEq for IDBCreateCommand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDBCreateCommand {}
impl ::core::fmt::Debug for IDBCreateCommand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDBCreateCommand").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDBCreateSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDBCreateSession {}
impl ::core::fmt::Debug for IDBCreateSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDBCreateSession").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDBDataSourceAdmin {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDBDataSourceAdmin {}
impl ::core::fmt::Debug for IDBDataSourceAdmin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDBDataSourceAdmin").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDBInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDBInfo {}
impl ::core::fmt::Debug for IDBInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDBInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDBInitialize {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDBInitialize {}
impl ::core::fmt::Debug for IDBInitialize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDBInitialize").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDBPromptInitialize {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDBPromptInitialize {}
impl ::core::fmt::Debug for IDBPromptInitialize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDBPromptInitialize").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDBProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDBProperties {}
impl ::core::fmt::Debug for IDBProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDBProperties").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDBSchemaCommand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDBSchemaCommand {}
impl ::core::fmt::Debug for IDBSchemaCommand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDBSchemaCommand").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDBSchemaRowset {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDBSchemaRowset {}
impl ::core::fmt::Debug for IDBSchemaRowset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDBSchemaRowset").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDCInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCInfo {}
impl ::core::fmt::Debug for IDCInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDataConvert {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDataConvert {}
impl ::core::fmt::Debug for IDataConvert {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataConvert").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDataInitialize {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDataInitialize {}
impl ::core::fmt::Debug for IDataInitialize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataInitialize").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IDataSourceLocator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IDataSourceLocator {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IDataSourceLocator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataSourceLocator").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEntity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEntity {}
impl ::core::fmt::Debug for IEntity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEntity").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumItemProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumItemProperties {}
impl ::core::fmt::Debug for IEnumItemProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumItemProperties").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumSearchRoots {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumSearchRoots {}
impl ::core::fmt::Debug for IEnumSearchRoots {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumSearchRoots").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumSearchScopeRules {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumSearchScopeRules {}
impl ::core::fmt::Debug for IEnumSearchScopeRules {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumSearchScopeRules").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumSubscription {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumSubscription {}
impl ::core::fmt::Debug for IEnumSubscription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumSubscription").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IErrorLookup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IErrorLookup {}
impl ::core::fmt::Debug for IErrorLookup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IErrorLookup").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IErrorRecords {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IErrorRecords {}
impl ::core::fmt::Debug for IErrorRecords {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IErrorRecords").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IGetDataSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGetDataSource {}
impl ::core::fmt::Debug for IGetDataSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGetDataSource").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IGetRow {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGetRow {}
impl ::core::fmt::Debug for IGetRow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGetRow").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IGetSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGetSession {}
impl ::core::fmt::Debug for IGetSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGetSession").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IGetSourceRow {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGetSourceRow {}
impl ::core::fmt::Debug for IGetSourceRow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGetSourceRow").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IIndexDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IIndexDefinition {}
impl ::core::fmt::Debug for IIndexDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IIndexDefinition").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IInterval {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInterval {}
impl ::core::fmt::Debug for IInterval {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInterval").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ILoadFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILoadFilter {}
impl ::core::fmt::Debug for ILoadFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILoadFilter").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ILoadFilterWithPrivateComActivation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILoadFilterWithPrivateComActivation {}
impl ::core::fmt::Debug for ILoadFilterWithPrivateComActivation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILoadFilterWithPrivateComActivation").field(&self.0).finish()
    }
}
impl ILoadFilterWithPrivateComActivation {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer"))]
    pub unsafe fn LoadIFilter<P0, P1, P2>(&self, pwcspath: P0, pfilteredsources: *const FILTERED_DATA_SOURCES, punkouter: P1, fusedefault: P2, pfilterclsid: *mut ::windows::core::GUID, searchdecsize: *mut i32, pwcssearchdesc: *mut *mut u16, ppifilt: *mut ::core::option::Option<super::super::Storage::IndexServer::IFilter>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P2: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.LoadIFilter)(::windows::core::Vtable::as_raw(self), pwcspath.into().abi(), pfilteredsources, punkouter.into().abi(), fusedefault.into(), pfilterclsid, searchdecsize, pwcssearchdesc, ::core::mem::transmute(ppifilt)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn LoadIFilterFromStorage<P0, P1, P2, P3>(&self, pstg: P0, punkouter: P1, pwcsoverride: P2, fusedefault: P3, pfilterclsid: *mut ::windows::core::GUID, searchdecsize: *mut i32, pwcssearchdesc: *mut *mut u16, ppifilt: *mut ::core::option::Option<super::super::Storage::IndexServer::IFilter>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::StructuredStorage::IStorage>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P3: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.LoadIFilterFromStorage)(::windows::core::Vtable::as_raw(self), pstg.into().abi(), punkouter.into().abi(), pwcsoverride.into().abi(), fusedefault.into(), pfilterclsid, searchdecsize, pwcssearchdesc, ::core::mem::transmute(ppifilt)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
    pub unsafe fn LoadIFilterFromStream<P0, P1, P2>(&self, pstm: P0, pfilteredsources: *const FILTERED_DATA_SOURCES, punkouter: P1, fusedefault: P2, pfilterclsid: *mut ::windows::core::GUID, searchdecsize: *mut i32, pwcssearchdesc: *mut *mut u16, ppifilt: *mut ::core::option::Option<super::super::Storage::IndexServer::IFilter>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P2: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.LoadIFilterFromStream)(::windows::core::Vtable::as_raw(self), pstm.into().abi(), pfilteredsources, punkouter.into().abi(), fusedefault.into(), pfilterclsid, searchdecsize, pwcssearchdesc, ::core::mem::transmute(ppifilt)).ok()
    }
}
impl ::core::cmp::PartialEq for IMDDataset {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMDDataset {}
impl ::core::fmt::Debug for IMDDataset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMDDataset").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMDFind {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMDFind {}
impl ::core::fmt::Debug for IMDFind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMDFind").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMDRangeRowset {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMDRangeRowset {}
impl ::core::fmt::Debug for IMDRangeRowset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMDRangeRowset").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMetaData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMetaData {}
impl ::core::fmt::Debug for IMetaData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMetaData").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMultipleResults {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMultipleResults {}
impl ::core::fmt::Debug for IMultipleResults {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMultipleResults").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for INCREMENTAL_ACCESS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for INCREMENTAL_ACCESS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.ftLastModifiedTime == other.ftLastModifiedTime
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for INCREMENTAL_ACCESS_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for INCREMENTAL_ACCESS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("INCREMENTAL_ACCESS_INFO").field("dwSize", &self.dwSize).field("ftLastModifiedTime", &self.ftLastModifiedTime).finish()
    }
}
impl ::core::default::Default for INTERVAL_LIMIT_KIND {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for INTERVAL_LIMIT_KIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INTERVAL_LIMIT_KIND").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for INamedEntity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INamedEntity {}
impl ::core::fmt::Debug for INamedEntity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INamedEntity").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for INamedEntityCollector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INamedEntityCollector {}
impl ::core::fmt::Debug for INamedEntityCollector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INamedEntityCollector").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IObjectAccessControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IObjectAccessControl {}
impl ::core::fmt::Debug for IObjectAccessControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IObjectAccessControl").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IOpLockStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOpLockStatus {}
impl ::core::fmt::Debug for IOpLockStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOpLockStatus").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IOpenRowset {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IOpenRowset {}
impl ::core::fmt::Debug for IOpenRowset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOpenRowset").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IParentRowset {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IParentRowset {}
impl ::core::fmt::Debug for IParentRowset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IParentRowset").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IProtocolHandlerSite {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProtocolHandlerSite {}
impl ::core::fmt::Debug for IProtocolHandlerSite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProtocolHandlerSite").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IProvideMoniker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProvideMoniker {}
impl ::core::fmt::Debug for IProvideMoniker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProvideMoniker").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IQueryParser {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IQueryParser {}
impl ::core::fmt::Debug for IQueryParser {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IQueryParser").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IQueryParserManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IQueryParserManager {}
impl ::core::fmt::Debug for IQueryParserManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IQueryParserManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IQuerySolution {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IQuerySolution {}
impl ::core::fmt::Debug for IQuerySolution {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IQuerySolution").field(&self.0).finish()
    }
}
impl IQuerySolution {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn MakeNot<P0, P1>(&self, pcsub: P0, fsimplify: P1) -> ::windows::core::Result<ICondition>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ICondition>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.MakeNot)(::windows::core::Vtable::as_raw(self), pcsub.into().abi(), fsimplify.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Search_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Search_Common"))]
    pub unsafe fn MakeAndOr<P0, P1>(&self, ct: Common::CONDITION_TYPE, peusubs: P0, fsimplify: P1) -> ::windows::core::Result<ICondition>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IEnumUnknown>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.MakeAndOr)(::windows::core::Vtable::as_raw(self), ct, peusubs.into().abi(), fsimplify.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Search_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common"))]
    pub unsafe fn MakeLeaf<P0, P1, P2, P3, P4, P5>(&self, pszpropertyname: P0, cop: Common::CONDITION_OPERATION, pszvaluetype: P1, ppropvar: *const super::Com::StructuredStorage::PROPVARIANT, ppropertynameterm: P2, poperationterm: P3, pvalueterm: P4, fexpand: P5) -> ::windows::core::Result<ICondition>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<IRichChunk>>,
        P3: ::std::convert::Into<::windows::core::InParam<IRichChunk>>,
        P4: ::std::convert::Into<::windows::core::InParam<IRichChunk>>,
        P5: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.MakeLeaf)(::windows::core::Vtable::as_raw(self), pszpropertyname.into().abi(), cop, pszvaluetype.into().abi(), ppropvar, ppropertynameterm.into().abi(), poperationterm.into().abi(), pvalueterm.into().abi(), fexpand.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Resolve<P0>(&self, pc: P0, sqro: STRUCTURED_QUERY_RESOLVE_OPTION, pstreferencetime: ::core::option::Option<*const super::super::Foundation::SYSTEMTIME>) -> ::windows::core::Result<ICondition>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ICondition>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Resolve)(::windows::core::Vtable::as_raw(self), pc.into().abi(), sqro, ::core::mem::transmute(pstreferencetime.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IReadData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IReadData {}
impl ::core::fmt::Debug for IReadData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IReadData").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRegisterProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRegisterProvider {}
impl ::core::fmt::Debug for IRegisterProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRegisterProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRelationship {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRelationship {}
impl ::core::fmt::Debug for IRelationship {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRelationship").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRichChunk {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRichChunk {}
impl ::core::fmt::Debug for IRichChunk {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRichChunk").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRow {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRow {}
impl ::core::fmt::Debug for IRow {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRow").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRowChange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRowChange {}
impl ::core::fmt::Debug for IRowChange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRowChange").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRowPosition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRowPosition {}
impl ::core::fmt::Debug for IRowPosition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRowPosition").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRowPositionChange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRowPositionChange {}
impl ::core::fmt::Debug for IRowPositionChange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRowPositionChange").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRowSchemaChange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRowSchemaChange {}
impl ::core::fmt::Debug for IRowSchemaChange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRowSchemaChange").field(&self.0).finish()
    }
}
impl IRowSchemaChange {
    #[doc = "*Required features: `\"Win32_Storage_IndexServer\"`*"]
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn SetColumns(&self, rgcolumns: &[DBCOLUMNACCESS]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetColumns)(::windows::core::Vtable::as_raw(self), rgcolumns.len() as _, ::core::mem::transmute(rgcolumns.as_ptr())).ok()
    }
}
impl ::core::cmp::PartialEq for IRowset {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRowset {}
impl ::core::fmt::Debug for IRowset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRowset").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRowsetAsynch {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRowsetAsynch {}
impl ::core::fmt::Debug for IRowsetAsynch {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRowsetAsynch").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRowsetBookmark {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRowsetBookmark {}
impl ::core::fmt::Debug for IRowsetBookmark {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRowsetBookmark").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRowsetChange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRowsetChange {}
impl ::core::fmt::Debug for IRowsetChange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRowsetChange").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRowsetChangeExtInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRowsetChangeExtInfo {}
impl ::core::fmt::Debug for IRowsetChangeExtInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRowsetChangeExtInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRowsetChapterMember {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRowsetChapterMember {}
impl ::core::fmt::Debug for IRowsetChapterMember {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRowsetChapterMember").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRowsetCopyRows {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRowsetCopyRows {}
impl ::core::fmt::Debug for IRowsetCopyRows {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRowsetCopyRows").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRowsetCurrentIndex {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRowsetCurrentIndex {}
impl ::core::fmt::Debug for IRowsetCurrentIndex {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRowsetCurrentIndex").field(&self.0).finish()
    }
}
impl IRowsetCurrentIndex {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetIndexInfo(&self, pckeycolumns: *mut usize, prgindexcolumndesc: *mut *mut DBINDEXCOLUMNDESC, pcindexpropertysets: *mut u32, prgindexpropertysets: *mut *mut DBPROPSET) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetIndexInfo)(::windows::core::Vtable::as_raw(self), pckeycolumns, prgindexcolumndesc, pcindexpropertysets, prgindexpropertysets).ok()
    }
    pub unsafe fn Seek<P0>(&self, haccessor: P0, ckeyvalues: usize, pdata: *mut ::core::ffi::c_void, dwseekoptions: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<HACCESSOR>,
    {
        (::windows::core::Vtable::vtable(self).base__.Seek)(::windows::core::Vtable::as_raw(self), haccessor.into(), ckeyvalues, pdata, dwseekoptions).ok()
    }
    pub unsafe fn SetRange<P0>(&self, haccessor: P0, cstartkeycolumns: usize, pstartdata: *mut ::core::ffi::c_void, cendkeycolumns: usize, penddata: *mut ::core::ffi::c_void, dwrangeoptions: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<HACCESSOR>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetRange)(::windows::core::Vtable::as_raw(self), haccessor.into(), cstartkeycolumns, pstartdata, cendkeycolumns, penddata, dwrangeoptions).ok()
    }
}
impl ::core::cmp::PartialEq for IRowsetEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRowsetEvents {}
impl ::core::fmt::Debug for IRowsetEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRowsetEvents").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRowsetFastLoad {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRowsetFastLoad {}
impl ::core::fmt::Debug for IRowsetFastLoad {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRowsetFastLoad").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRowsetFind {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRowsetFind {}
impl ::core::fmt::Debug for IRowsetFind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRowsetFind").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRowsetIdentity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRowsetIdentity {}
impl ::core::fmt::Debug for IRowsetIdentity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRowsetIdentity").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRowsetIndex {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRowsetIndex {}
impl ::core::fmt::Debug for IRowsetIndex {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRowsetIndex").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRowsetInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRowsetInfo {}
impl ::core::fmt::Debug for IRowsetInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRowsetInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRowsetKeys {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRowsetKeys {}
impl ::core::fmt::Debug for IRowsetKeys {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRowsetKeys").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRowsetLocate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRowsetLocate {}
impl ::core::fmt::Debug for IRowsetLocate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRowsetLocate").field(&self.0).finish()
    }
}
impl IRowsetLocate {
    pub unsafe fn AddRefRows(&self, crows: usize, rghrows: *const usize, rgrefcounts: *mut u32, rgrowstatus: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.AddRefRows)(::windows::core::Vtable::as_raw(self), crows, rghrows, rgrefcounts, rgrowstatus).ok()
    }
    pub unsafe fn GetData<P0>(&self, hrow: usize, haccessor: P0, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<HACCESSOR>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetData)(::windows::core::Vtable::as_raw(self), hrow, haccessor.into(), pdata).ok()
    }
    pub unsafe fn GetNextRows(&self, hreserved: usize, lrowsoffset: isize, crows: isize, pcrowsobtained: *mut usize, prghrows: *mut *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetNextRows)(::windows::core::Vtable::as_raw(self), hreserved, lrowsoffset, crows, pcrowsobtained, prghrows).ok()
    }
    pub unsafe fn ReleaseRows(&self, crows: usize, rghrows: *const usize, rgrowoptions: *mut u32, rgrefcounts: *mut u32, rgrowstatus: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ReleaseRows)(::windows::core::Vtable::as_raw(self), crows, rghrows, rgrowoptions, rgrefcounts, rgrowstatus).ok()
    }
    pub unsafe fn RestartPosition(&self, hreserved: usize) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.RestartPosition)(::windows::core::Vtable::as_raw(self), hreserved).ok()
    }
}
impl ::core::cmp::PartialEq for IRowsetNewRowAfter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRowsetNewRowAfter {}
impl ::core::fmt::Debug for IRowsetNewRowAfter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRowsetNewRowAfter").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRowsetNextRowset {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRowsetNextRowset {}
impl ::core::fmt::Debug for IRowsetNextRowset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRowsetNextRowset").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRowsetNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRowsetNotify {}
impl ::core::fmt::Debug for IRowsetNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRowsetNotify").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRowsetPrioritization {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRowsetPrioritization {}
impl ::core::fmt::Debug for IRowsetPrioritization {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRowsetPrioritization").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRowsetQueryStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRowsetQueryStatus {}
impl ::core::fmt::Debug for IRowsetQueryStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRowsetQueryStatus").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRowsetRefresh {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRowsetRefresh {}
impl ::core::fmt::Debug for IRowsetRefresh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRowsetRefresh").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRowsetResynch {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRowsetResynch {}
impl ::core::fmt::Debug for IRowsetResynch {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRowsetResynch").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRowsetScroll {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRowsetScroll {}
impl ::core::fmt::Debug for IRowsetScroll {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRowsetScroll").field(&self.0).finish()
    }
}
impl IRowsetScroll {
    pub unsafe fn AddRefRows(&self, crows: usize, rghrows: *const usize, rgrefcounts: *mut u32, rgrowstatus: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.AddRefRows)(::windows::core::Vtable::as_raw(self), crows, rghrows, rgrefcounts, rgrowstatus).ok()
    }
    pub unsafe fn GetData<P0>(&self, hrow: usize, haccessor: P0, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<HACCESSOR>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetData)(::windows::core::Vtable::as_raw(self), hrow, haccessor.into(), pdata).ok()
    }
    pub unsafe fn GetNextRows(&self, hreserved: usize, lrowsoffset: isize, crows: isize, pcrowsobtained: *mut usize, prghrows: *mut *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetNextRows)(::windows::core::Vtable::as_raw(self), hreserved, lrowsoffset, crows, pcrowsobtained, prghrows).ok()
    }
    pub unsafe fn ReleaseRows(&self, crows: usize, rghrows: *const usize, rgrowoptions: *mut u32, rgrefcounts: *mut u32, rgrowstatus: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.ReleaseRows)(::windows::core::Vtable::as_raw(self), crows, rghrows, rgrowoptions, rgrefcounts, rgrowstatus).ok()
    }
    pub unsafe fn RestartPosition(&self, hreserved: usize) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.RestartPosition)(::windows::core::Vtable::as_raw(self), hreserved).ok()
    }
    pub unsafe fn Compare(&self, hreserved: usize, cbbookmark1: usize, pbookmark1: *const u8, cbbookmark2: usize, pbookmark2: *const u8, pcomparison: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Compare)(::windows::core::Vtable::as_raw(self), hreserved, cbbookmark1, pbookmark1, cbbookmark2, pbookmark2, pcomparison).ok()
    }
    pub unsafe fn GetRowsAt(&self, hreserved1: usize, hreserved2: usize, cbbookmark: usize, pbookmark: *const u8, lrowsoffset: isize, crows: isize, pcrowsobtained: *mut usize, prghrows: *mut *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetRowsAt)(::windows::core::Vtable::as_raw(self), hreserved1, hreserved2, cbbookmark, pbookmark, lrowsoffset, crows, pcrowsobtained, prghrows).ok()
    }
    pub unsafe fn GetRowsByBookmark(&self, hreserved: usize, crows: usize, rgcbbookmarks: *const usize, rgpbookmarks: *const *const u8, rghrows: *mut usize, rgrowstatus: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetRowsByBookmark)(::windows::core::Vtable::as_raw(self), hreserved, crows, rgcbbookmarks, rgpbookmarks, rghrows, rgrowstatus).ok()
    }
    pub unsafe fn Hash(&self, hreserved: usize, cbookmarks: usize, rgcbbookmarks: *const usize, rgpbookmarks: *const *const u8, rghashedvalues: *mut usize, rgbookmarkstatus: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Hash)(::windows::core::Vtable::as_raw(self), hreserved, cbookmarks, rgcbbookmarks, rgpbookmarks, rghashedvalues, rgbookmarkstatus).ok()
    }
}
impl ::core::cmp::PartialEq for IRowsetUpdate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRowsetUpdate {}
impl ::core::fmt::Debug for IRowsetUpdate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRowsetUpdate").field(&self.0).finish()
    }
}
impl IRowsetUpdate {
    pub unsafe fn DeleteRows(&self, hreserved: usize, crows: usize, rghrows: *const usize, rgrowstatus: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DeleteRows)(::windows::core::Vtable::as_raw(self), hreserved, crows, rghrows, rgrowstatus).ok()
    }
    pub unsafe fn SetData<P0>(&self, hrow: usize, haccessor: P0, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<HACCESSOR>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetData)(::windows::core::Vtable::as_raw(self), hrow, haccessor.into(), pdata).ok()
    }
    pub unsafe fn InsertRow<P0>(&self, hreserved: usize, haccessor: P0, pdata: *mut ::core::ffi::c_void, phrow: *mut usize) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<HACCESSOR>,
    {
        (::windows::core::Vtable::vtable(self).base__.InsertRow)(::windows::core::Vtable::as_raw(self), hreserved, haccessor.into(), pdata, phrow).ok()
    }
}
impl ::core::cmp::PartialEq for IRowsetView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRowsetView {}
impl ::core::fmt::Debug for IRowsetView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRowsetView").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRowsetWatchAll {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRowsetWatchAll {}
impl ::core::fmt::Debug for IRowsetWatchAll {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRowsetWatchAll").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRowsetWatchNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRowsetWatchNotify {}
impl ::core::fmt::Debug for IRowsetWatchNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRowsetWatchNotify").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IRowsetWatchRegion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRowsetWatchRegion {}
impl ::core::fmt::Debug for IRowsetWatchRegion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRowsetWatchRegion").field(&self.0).finish()
    }
}
impl IRowsetWatchRegion {
    pub unsafe fn Acknowledge(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Acknowledge)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Start(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Start)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn StopWatching(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.StopWatching)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for IRowsetWithParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRowsetWithParameters {}
impl ::core::fmt::Debug for IRowsetWithParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRowsetWithParameters").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISQLErrorInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISQLErrorInfo {}
impl ::core::fmt::Debug for ISQLErrorInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISQLErrorInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISQLGetDiagField {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISQLGetDiagField {}
impl ::core::fmt::Debug for ISQLGetDiagField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISQLGetDiagField").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISQLRequestDiagFields {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISQLRequestDiagFields {}
impl ::core::fmt::Debug for ISQLRequestDiagFields {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISQLRequestDiagFields").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISQLServerErrorInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISQLServerErrorInfo {}
impl ::core::fmt::Debug for ISQLServerErrorInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISQLServerErrorInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISchemaLocalizerSupport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISchemaLocalizerSupport {}
impl ::core::fmt::Debug for ISchemaLocalizerSupport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISchemaLocalizerSupport").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISchemaLock {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISchemaLock {}
impl ::core::fmt::Debug for ISchemaLock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISchemaLock").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISchemaProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISchemaProvider {}
impl ::core::fmt::Debug for ISchemaProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISchemaProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IScopedOperations {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IScopedOperations {}
impl ::core::fmt::Debug for IScopedOperations {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IScopedOperations").field(&self.0).finish()
    }
}
impl IScopedOperations {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Bind<P0, P1, P2>(&self, punkouter: P0, pwszurl: P1, dwbindurlflags: u32, rguid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pauthenticate: P2, pimplsession: ::core::option::Option<*mut DBIMPLICITSESSION>, pdwbindstatus: ::core::option::Option<*mut u32>, ppunk: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<super::Com::IAuthenticate>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Bind)(::windows::core::Vtable::as_raw(self), punkouter.into().abi(), pwszurl.into().abi(), dwbindurlflags, rguid, riid, pauthenticate.into().abi(), ::core::mem::transmute(pimplsession.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pdwbindstatus.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppunk)).ok()
    }
}
impl ::core::cmp::PartialEq for ISearchCatalogManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISearchCatalogManager {}
impl ::core::fmt::Debug for ISearchCatalogManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISearchCatalogManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISearchCatalogManager2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISearchCatalogManager2 {}
impl ::core::fmt::Debug for ISearchCatalogManager2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISearchCatalogManager2").field(&self.0).finish()
    }
}
impl ISearchCatalogManager2 {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Name)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetParameter<P0>(&self, pszname: P0) -> ::windows::core::Result<*mut super::Com::StructuredStorage::PROPVARIANT>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetParameter)(::windows::core::Vtable::as_raw(self), pszname.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn SetParameter<P0>(&self, pszname: P0, pvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetParameter)(::windows::core::Vtable::as_raw(self), pszname.into().abi(), pvalue).ok()
    }
    pub unsafe fn GetCatalogStatus(&self, pstatus: *mut CatalogStatus, ppausedreason: *mut CatalogPausedReason) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetCatalogStatus)(::windows::core::Vtable::as_raw(self), pstatus, ppausedreason).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Reindex(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Reindex)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ReindexMatchingURLs<P0>(&self, pszpattern: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ReindexMatchingURLs)(::windows::core::Vtable::as_raw(self), pszpattern.into().abi()).ok()
    }
    pub unsafe fn ReindexSearchRoot<P0>(&self, pszrooturl: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ReindexSearchRoot)(::windows::core::Vtable::as_raw(self), pszrooturl.into().abi()).ok()
    }
    pub unsafe fn SetConnectTimeout(&self, dwconnecttimeout: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetConnectTimeout)(::windows::core::Vtable::as_raw(self), dwconnecttimeout).ok()
    }
    pub unsafe fn ConnectTimeout(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ConnectTimeout)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDataTimeout(&self, dwdatatimeout: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetDataTimeout)(::windows::core::Vtable::as_raw(self), dwdatatimeout).ok()
    }
    pub unsafe fn DataTimeout(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DataTimeout)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn NumberOfItems(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.NumberOfItems)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn NumberOfItemsToIndex(&self, plincrementalcount: *mut i32, plnotificationqueue: *mut i32, plhighpriorityqueue: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.NumberOfItemsToIndex)(::windows::core::Vtable::as_raw(self), plincrementalcount, plnotificationqueue, plhighpriorityqueue).ok()
    }
    pub unsafe fn URLBeingIndexed(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.URLBeingIndexed)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetURLIndexingState<P0>(&self, pszurl: P0) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetURLIndexingState)(::windows::core::Vtable::as_raw(self), pszurl.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPersistentItemsChangedSink(&self) -> ::windows::core::Result<ISearchPersistentItemsChangedSink> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPersistentItemsChangedSink)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RegisterViewForNotification<P0, P1>(&self, pszview: P0, pviewchangedsink: P1) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<ISearchViewChangedSink>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.RegisterViewForNotification)(::windows::core::Vtable::as_raw(self), pszview.into().abi(), pviewchangedsink.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetItemsChangedSink<P0>(&self, pisearchnotifyinlinesite: P0, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void, pguidcatalogresetsignature: *mut ::windows::core::GUID, pguidcheckpointsignature: *mut ::windows::core::GUID, pdwlastcheckpointnumber: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ISearchNotifyInlineSite>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetItemsChangedSink)(::windows::core::Vtable::as_raw(self), pisearchnotifyinlinesite.into().abi(), riid, ppv, pguidcatalogresetsignature, pguidcheckpointsignature, pdwlastcheckpointnumber).ok()
    }
    pub unsafe fn UnregisterViewForNotification(&self, dwcookie: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.UnregisterViewForNotification)(::windows::core::Vtable::as_raw(self), dwcookie).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetExtensionClusion<P0, P1>(&self, pszextension: P0, fexclude: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetExtensionClusion)(::windows::core::Vtable::as_raw(self), pszextension.into().abi(), fexclude.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumerateExcludedExtensions(&self) -> ::windows::core::Result<super::Com::IEnumString> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumerateExcludedExtensions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetQueryHelper(&self) -> ::windows::core::Result<ISearchQueryHelper> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetQueryHelper)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDiacriticSensitivity<P0>(&self, fdiacriticsensitive: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetDiacriticSensitivity)(::windows::core::Vtable::as_raw(self), fdiacriticsensitive.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DiacriticSensitivity(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DiacriticSensitivity)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCrawlScopeManager(&self) -> ::windows::core::Result<ISearchCrawlScopeManager> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCrawlScopeManager)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ISearchCrawlScopeManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISearchCrawlScopeManager {}
impl ::core::fmt::Debug for ISearchCrawlScopeManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISearchCrawlScopeManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISearchCrawlScopeManager2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISearchCrawlScopeManager2 {}
impl ::core::fmt::Debug for ISearchCrawlScopeManager2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISearchCrawlScopeManager2").field(&self.0).finish()
    }
}
impl ISearchCrawlScopeManager2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddDefaultScopeRule<P0, P1>(&self, pszurl: P0, finclude: P1, ffollowflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.AddDefaultScopeRule)(::windows::core::Vtable::as_raw(self), pszurl.into().abi(), finclude.into(), ffollowflags).ok()
    }
    pub unsafe fn AddRoot<P0>(&self, psearchroot: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ISearchRoot>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AddRoot)(::windows::core::Vtable::as_raw(self), psearchroot.into().abi()).ok()
    }
    pub unsafe fn RemoveRoot<P0>(&self, pszurl: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.RemoveRoot)(::windows::core::Vtable::as_raw(self), pszurl.into().abi()).ok()
    }
    pub unsafe fn EnumerateRoots(&self) -> ::windows::core::Result<IEnumSearchRoots> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumerateRoots)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddHierarchicalScope<P0, P1, P2, P3>(&self, pszurl: P0, finclude: P1, fdefault: P2, foverridechildren: P3) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
        P2: ::std::convert::Into<super::super::Foundation::BOOL>,
        P3: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.AddHierarchicalScope)(::windows::core::Vtable::as_raw(self), pszurl.into().abi(), finclude.into(), fdefault.into(), foverridechildren.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddUserScopeRule<P0, P1, P2>(&self, pszurl: P0, finclude: P1, foverridechildren: P2, ffollowflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
        P2: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.AddUserScopeRule)(::windows::core::Vtable::as_raw(self), pszurl.into().abi(), finclude.into(), foverridechildren.into(), ffollowflags).ok()
    }
    pub unsafe fn RemoveScopeRule<P0>(&self, pszrule: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.RemoveScopeRule)(::windows::core::Vtable::as_raw(self), pszrule.into().abi()).ok()
    }
    pub unsafe fn EnumerateScopeRules(&self) -> ::windows::core::Result<IEnumSearchScopeRules> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumerateScopeRules)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasParentScopeRule<P0>(&self, pszurl: P0) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.HasParentScopeRule)(::windows::core::Vtable::as_raw(self), pszurl.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasChildScopeRule<P0>(&self, pszurl: P0) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.HasChildScopeRule)(::windows::core::Vtable::as_raw(self), pszurl.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IncludedInCrawlScope<P0>(&self, pszurl: P0) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IncludedInCrawlScope)(::windows::core::Vtable::as_raw(self), pszurl.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IncludedInCrawlScopeEx<P0>(&self, pszurl: P0, pfisincluded: *mut super::super::Foundation::BOOL, preason: *mut CLUSION_REASON) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.IncludedInCrawlScopeEx)(::windows::core::Vtable::as_raw(self), pszurl.into().abi(), pfisincluded, preason).ok()
    }
    pub unsafe fn RevertToDefaultScopes(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.RevertToDefaultScopes)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SaveAll(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SaveAll)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetParentScopeVersionId<P0>(&self, pszurl: P0) -> ::windows::core::Result<i32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetParentScopeVersionId)(::windows::core::Vtable::as_raw(self), pszurl.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RemoveDefaultScopeRule<P0>(&self, pszurl: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.RemoveDefaultScopeRule)(::windows::core::Vtable::as_raw(self), pszurl.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for ISearchItemsChangedSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISearchItemsChangedSink {}
impl ::core::fmt::Debug for ISearchItemsChangedSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISearchItemsChangedSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISearchLanguageSupport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISearchLanguageSupport {}
impl ::core::fmt::Debug for ISearchLanguageSupport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISearchLanguageSupport").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISearchManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISearchManager {}
impl ::core::fmt::Debug for ISearchManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISearchManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISearchManager2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISearchManager2 {}
impl ::core::fmt::Debug for ISearchManager2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISearchManager2").field(&self.0).finish()
    }
}
impl ISearchManager2 {
    pub unsafe fn GetIndexerVersionStr(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetIndexerVersionStr)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetIndexerVersion(&self, pdwmajor: *mut u32, pdwminor: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetIndexerVersion)(::windows::core::Vtable::as_raw(self), pdwmajor, pdwminor).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetParameter<P0>(&self, pszname: P0) -> ::windows::core::Result<*mut super::Com::StructuredStorage::PROPVARIANT>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetParameter)(::windows::core::Vtable::as_raw(self), pszname.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn SetParameter<P0>(&self, pszname: P0, pvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetParameter)(::windows::core::Vtable::as_raw(self), pszname.into().abi(), pvalue).ok()
    }
    pub unsafe fn ProxyName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ProxyName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn BypassList(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.BypassList)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetProxy<P0, P1, P2>(&self, suseproxy: PROXY_ACCESS, flocalbypassproxy: P0, dwportnumber: u32, pszproxyname: P1, pszbypasslist: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetProxy)(::windows::core::Vtable::as_raw(self), suseproxy, flocalbypassproxy.into(), dwportnumber, pszproxyname.into().abi(), pszbypasslist.into().abi()).ok()
    }
    pub unsafe fn GetCatalog<P0>(&self, pszcatalog: P0) -> ::windows::core::Result<ISearchCatalogManager>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCatalog)(::windows::core::Vtable::as_raw(self), pszcatalog.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn UserAgent(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.UserAgent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetUserAgent<P0>(&self, pszuseragent: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetUserAgent)(::windows::core::Vtable::as_raw(self), pszuseragent.into().abi()).ok()
    }
    pub unsafe fn UseProxy(&self) -> ::windows::core::Result<PROXY_ACCESS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.UseProxy)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LocalBypass(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.LocalBypass)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn PortNumber(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.PortNumber)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ISearchNotifyInlineSite {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISearchNotifyInlineSite {}
impl ::core::fmt::Debug for ISearchNotifyInlineSite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISearchNotifyInlineSite").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISearchPersistentItemsChangedSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISearchPersistentItemsChangedSink {}
impl ::core::fmt::Debug for ISearchPersistentItemsChangedSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISearchPersistentItemsChangedSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISearchProtocol {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISearchProtocol {}
impl ::core::fmt::Debug for ISearchProtocol {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISearchProtocol").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISearchProtocol2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISearchProtocol2 {}
impl ::core::fmt::Debug for ISearchProtocol2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISearchProtocol2").field(&self.0).finish()
    }
}
impl ISearchProtocol2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Init<P0>(&self, ptimeoutinfo: *mut TIMEOUT_INFO, pprotocolhandlersite: P0, pproxyinfo: *mut PROXY_INFO) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IProtocolHandlerSite>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Init)(::windows::core::Vtable::as_raw(self), ptimeoutinfo, pprotocolhandlersite.into().abi(), pproxyinfo).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateAccessor<P0>(&self, pcwszurl: P0, pauthenticationinfo: *mut AUTHENTICATION_INFO, pincrementalaccessinfo: *mut INCREMENTAL_ACCESS_INFO, piteminfo: *mut ITEM_INFO, ppaccessor: *mut ::core::option::Option<IUrlAccessor>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateAccessor)(::windows::core::Vtable::as_raw(self), pcwszurl.into().abi(), pauthenticationinfo, pincrementalaccessinfo, piteminfo, ::core::mem::transmute(ppaccessor)).ok()
    }
    pub unsafe fn CloseAccessor<P0>(&self, paccessor: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IUrlAccessor>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CloseAccessor)(::windows::core::Vtable::as_raw(self), paccessor.into().abi()).ok()
    }
    pub unsafe fn ShutDown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ShutDown)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for ISearchProtocolThreadContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISearchProtocolThreadContext {}
impl ::core::fmt::Debug for ISearchProtocolThreadContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISearchProtocolThreadContext").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISearchQueryHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISearchQueryHelper {}
impl ::core::fmt::Debug for ISearchQueryHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISearchQueryHelper").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISearchQueryHits {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISearchQueryHits {}
impl ::core::fmt::Debug for ISearchQueryHits {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISearchQueryHits").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISearchRoot {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISearchRoot {}
impl ::core::fmt::Debug for ISearchRoot {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISearchRoot").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISearchScopeRule {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISearchScopeRule {}
impl ::core::fmt::Debug for ISearchScopeRule {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISearchScopeRule").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISearchViewChangedSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISearchViewChangedSink {}
impl ::core::fmt::Debug for ISearchViewChangedSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISearchViewChangedSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISecurityInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISecurityInfo {}
impl ::core::fmt::Debug for ISecurityInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISecurityInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IService {}
impl ::core::fmt::Debug for IService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IService").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISessionProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISessionProperties {}
impl ::core::fmt::Debug for ISessionProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISessionProperties").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISimpleCommandCreator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISimpleCommandCreator {}
impl ::core::fmt::Debug for ISimpleCommandCreator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISimpleCommandCreator").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISourcesRowset {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISourcesRowset {}
impl ::core::fmt::Debug for ISourcesRowset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISourcesRowset").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IStemmer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStemmer {}
impl ::core::fmt::Debug for IStemmer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStemmer").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISubscriptionItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISubscriptionItem {}
impl ::core::fmt::Debug for ISubscriptionItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISubscriptionItem").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISubscriptionMgr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISubscriptionMgr {}
impl ::core::fmt::Debug for ISubscriptionMgr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISubscriptionMgr").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ISubscriptionMgr2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISubscriptionMgr2 {}
impl ::core::fmt::Debug for ISubscriptionMgr2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISubscriptionMgr2").field(&self.0).finish()
    }
}
impl ISubscriptionMgr2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteSubscription<P0, P1>(&self, pwszurl: P0, hwnd: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.DeleteSubscription)(::windows::core::Vtable::as_raw(self), pwszurl.into().abi(), hwnd.into()).ok()
    }
    pub unsafe fn UpdateSubscription<P0>(&self, pwszurl: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.UpdateSubscription)(::windows::core::Vtable::as_raw(self), pwszurl.into().abi()).ok()
    }
    pub unsafe fn UpdateAll(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.UpdateAll)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSubscribed<P0>(&self, pwszurl: P0) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsSubscribed)(::windows::core::Vtable::as_raw(self), pwszurl.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSubscriptionInfo<P0>(&self, pwszurl: P0, pinfo: *mut SUBSCRIPTIONINFO) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetSubscriptionInfo)(::windows::core::Vtable::as_raw(self), pwszurl.into().abi(), pinfo).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDefaultInfo(&self, subtype: SUBSCRIPTIONTYPE, pinfo: *mut SUBSCRIPTIONINFO) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDefaultInfo)(::windows::core::Vtable::as_raw(self), subtype, pinfo).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShowSubscriptionProperties<P0, P1>(&self, pwszurl: P0, hwnd: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.ShowSubscriptionProperties)(::windows::core::Vtable::as_raw(self), pwszurl.into().abi(), hwnd.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSubscription<P0, P1, P2>(&self, hwnd: P0, pwszurl: P1, pwszfriendlyname: P2, dwflags: u32, substype: SUBSCRIPTIONTYPE, pinfo: *mut SUBSCRIPTIONINFO) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateSubscription)(::windows::core::Vtable::as_raw(self), hwnd.into(), pwszurl.into().abi(), pwszfriendlyname.into().abi(), dwflags, substype, pinfo).ok()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::default::Default for ITEMPROP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for ITEM_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ITEM_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.pcwszFromEMail == other.pcwszFromEMail && self.pcwszApplicationName == other.pcwszApplicationName && self.pcwszCatalogName == other.pcwszCatalogName && self.pcwszContentClass == other.pcwszContentClass
    }
}
impl ::core::cmp::Eq for ITEM_INFO {}
impl ::core::fmt::Debug for ITEM_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ITEM_INFO").field("dwSize", &self.dwSize).field("pcwszFromEMail", &self.pcwszFromEMail).field("pcwszApplicationName", &self.pcwszApplicationName).field("pcwszCatalogName", &self.pcwszCatalogName).field("pcwszContentClass", &self.pcwszContentClass).finish()
    }
}
impl ::core::cmp::PartialEq for ITableCreation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITableCreation {}
impl ::core::fmt::Debug for ITableCreation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITableCreation").field(&self.0).finish()
    }
}
impl ITableCreation {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateTable<P0>(&self, punkouter: P0, ptableid: ::core::option::Option<*const super::super::Storage::IndexServer::DBID>, rgcolumndescs: ::core::option::Option<&[DBCOLUMNDESC]>, riid: *const ::windows::core::GUID, rgpropertysets: ::core::option::Option<&mut [DBPROPSET]>, pptableid: ::core::option::Option<*mut *mut super::super::Storage::IndexServer::DBID>, pprowset: ::core::option::Option<*mut ::core::option::Option<::windows::core::IUnknown>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateTable)(
            ::windows::core::Vtable::as_raw(self),
            punkouter.into().abi(),
            ::core::mem::transmute(ptableid.unwrap_or(::std::ptr::null())),
            rgcolumndescs.as_deref().map_or(0, |slice| slice.len() as _),
            ::core::mem::transmute(rgcolumndescs.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
            riid,
            rgpropertysets.as_deref().map_or(0, |slice| slice.len() as _),
            ::core::mem::transmute(rgpropertysets.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
            ::core::mem::transmute(pptableid.unwrap_or(::std::ptr::null_mut())),
            ::core::mem::transmute(pprowset.unwrap_or(::std::ptr::null_mut())),
        )
        .ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_IndexServer\"`*"]
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn DropTable(&self, ptableid: *const super::super::Storage::IndexServer::DBID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DropTable)(::windows::core::Vtable::as_raw(self), ptableid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddColumn(&self, ptableid: *const super::super::Storage::IndexServer::DBID, pcolumndesc: *const DBCOLUMNDESC, ppcolumnid: ::core::option::Option<*mut *mut super::super::Storage::IndexServer::DBID>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.AddColumn)(::windows::core::Vtable::as_raw(self), ptableid, pcolumndesc, ::core::mem::transmute(ppcolumnid.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_IndexServer\"`*"]
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn DropColumn(&self, ptableid: *const super::super::Storage::IndexServer::DBID, pcolumnid: *const super::super::Storage::IndexServer::DBID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DropColumn)(::windows::core::Vtable::as_raw(self), ptableid, pcolumnid).ok()
    }
}
impl ::core::cmp::PartialEq for ITableDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITableDefinition {}
impl ::core::fmt::Debug for ITableDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITableDefinition").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITableDefinitionWithConstraints {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITableDefinitionWithConstraints {}
impl ::core::fmt::Debug for ITableDefinitionWithConstraints {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITableDefinitionWithConstraints").field(&self.0).finish()
    }
}
impl ITableDefinitionWithConstraints {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CreateTable<P0>(&self, punkouter: P0, ptableid: ::core::option::Option<*const super::super::Storage::IndexServer::DBID>, rgcolumndescs: ::core::option::Option<&[DBCOLUMNDESC]>, riid: *const ::windows::core::GUID, rgpropertysets: ::core::option::Option<&mut [DBPROPSET]>, pptableid: ::core::option::Option<*mut *mut super::super::Storage::IndexServer::DBID>, pprowset: ::core::option::Option<*mut ::core::option::Option<::windows::core::IUnknown>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateTable)(
            ::windows::core::Vtable::as_raw(self),
            punkouter.into().abi(),
            ::core::mem::transmute(ptableid.unwrap_or(::std::ptr::null())),
            rgcolumndescs.as_deref().map_or(0, |slice| slice.len() as _),
            ::core::mem::transmute(rgcolumndescs.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
            riid,
            rgpropertysets.as_deref().map_or(0, |slice| slice.len() as _),
            ::core::mem::transmute(rgpropertysets.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())),
            ::core::mem::transmute(pptableid.unwrap_or(::std::ptr::null_mut())),
            ::core::mem::transmute(pprowset.unwrap_or(::std::ptr::null_mut())),
        )
        .ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_IndexServer\"`*"]
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn DropTable(&self, ptableid: *const super::super::Storage::IndexServer::DBID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.DropTable)(::windows::core::Vtable::as_raw(self), ptableid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn AddColumn(&self, ptableid: *const super::super::Storage::IndexServer::DBID, pcolumndesc: *const DBCOLUMNDESC, ppcolumnid: ::core::option::Option<*mut *mut super::super::Storage::IndexServer::DBID>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.AddColumn)(::windows::core::Vtable::as_raw(self), ptableid, pcolumndesc, ::core::mem::transmute(ppcolumnid.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_IndexServer\"`*"]
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn DropColumn(&self, ptableid: *const super::super::Storage::IndexServer::DBID, pcolumnid: *const super::super::Storage::IndexServer::DBID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.DropColumn)(::windows::core::Vtable::as_raw(self), ptableid, pcolumnid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetTableDefinition(&self, ptableid: *const super::super::Storage::IndexServer::DBID, pccolumndescs: ::core::option::Option<*mut usize>, prgcolumndescs: ::core::option::Option<*mut *mut DBCOLUMNDESC>, pcpropertysets: ::core::option::Option<*mut u32>, prgpropertysets: ::core::option::Option<*mut *mut DBPROPSET>, pcconstraintdescs: ::core::option::Option<*mut u32>, prgconstraintdescs: ::core::option::Option<*mut *mut DBCONSTRAINTDESC>, ppwszstringbuffer: ::core::option::Option<*mut *mut u16>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetTableDefinition)(
            ::windows::core::Vtable::as_raw(self),
            ptableid,
            ::core::mem::transmute(pccolumndescs.unwrap_or(::std::ptr::null_mut())),
            ::core::mem::transmute(prgcolumndescs.unwrap_or(::std::ptr::null_mut())),
            ::core::mem::transmute(pcpropertysets.unwrap_or(::std::ptr::null_mut())),
            ::core::mem::transmute(prgpropertysets.unwrap_or(::std::ptr::null_mut())),
            ::core::mem::transmute(pcconstraintdescs.unwrap_or(::std::ptr::null_mut())),
            ::core::mem::transmute(prgconstraintdescs.unwrap_or(::std::ptr::null_mut())),
            ::core::mem::transmute(ppwszstringbuffer.unwrap_or(::std::ptr::null_mut())),
        )
        .ok()
    }
}
impl ::core::cmp::PartialEq for ITableRename {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITableRename {}
impl ::core::fmt::Debug for ITableRename {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITableRename").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITokenCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITokenCollection {}
impl ::core::fmt::Debug for ITokenCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITokenCollection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITransactionJoin {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionJoin {}
impl ::core::fmt::Debug for ITransactionJoin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionJoin").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl ::core::cmp::PartialEq for ITransactionLocal {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl ::core::cmp::Eq for ITransactionLocal {}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl ::core::fmt::Debug for ITransactionLocal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionLocal").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl ITransactionLocal {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_DistributedTransactionCoordinator\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_DistributedTransactionCoordinator"))]
    pub unsafe fn Commit<P0>(&self, fretaining: P0, grftc: u32, grfrm: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.Commit)(::windows::core::Vtable::as_raw(self), fretaining.into(), grftc, grfrm).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_DistributedTransactionCoordinator\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_DistributedTransactionCoordinator"))]
    pub unsafe fn Abort<P0, P1>(&self, pboidreason: *const super::DistributedTransactionCoordinator::BOID, fretaining: P0, fasync: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.Abort)(::windows::core::Vtable::as_raw(self), pboidreason, fretaining.into(), fasync.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_DistributedTransactionCoordinator\"`*"]
    #[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
    pub unsafe fn GetTransactionInfo(&self, pinfo: *mut super::DistributedTransactionCoordinator::XACTTRANSINFO) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetTransactionInfo)(::windows::core::Vtable::as_raw(self), pinfo).ok()
    }
}
impl ::core::cmp::PartialEq for ITransactionObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransactionObject {}
impl ::core::fmt::Debug for ITransactionObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransactionObject").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITrusteeAdmin {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITrusteeAdmin {}
impl ::core::fmt::Debug for ITrusteeAdmin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITrusteeAdmin").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ITrusteeGroupAdmin {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITrusteeGroupAdmin {}
impl ::core::fmt::Debug for ITrusteeGroupAdmin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITrusteeGroupAdmin").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUMS {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUMS {}
impl ::core::fmt::Debug for IUMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUMS").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUMSInitialize {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUMSInitialize {}
impl ::core::fmt::Debug for IUMSInitialize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUMSInitialize").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUrlAccessor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUrlAccessor {}
impl ::core::fmt::Debug for IUrlAccessor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUrlAccessor").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUrlAccessor2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUrlAccessor2 {}
impl ::core::fmt::Debug for IUrlAccessor2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUrlAccessor2").field(&self.0).finish()
    }
}
impl IUrlAccessor2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn AddRequestParameter(&self, pspec: *const super::Com::StructuredStorage::PROPSPEC, pvar: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.AddRequestParameter)(::windows::core::Vtable::as_raw(self), pspec, pvar).ok()
    }
    pub unsafe fn GetDocFormat(&self, wszdocformat: ::windows::core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDocFormat)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(wszdocformat), dwsize, pdwlength).ok()
    }
    pub unsafe fn GetCLSID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCLSID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetHost(&self, wszhost: ::windows::core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetHost)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(wszhost), dwsize, pdwlength).ok()
    }
    pub unsafe fn IsDirectory(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.IsDirectory)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetSize(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSize)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLastModified(&self) -> ::windows::core::Result<super::super::Foundation::FILETIME> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetLastModified)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFileName(&self, wszfilename: ::windows::core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetFileName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(wszfilename), dwsize, pdwlength).ok()
    }
    pub unsafe fn GetSecurityDescriptor(&self, psd: &mut [u8], pdwlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetSecurityDescriptor)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(psd.as_ptr()), psd.len() as _, pdwlength).ok()
    }
    pub unsafe fn GetRedirectedURL(&self, wszredirectedurl: ::windows::core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetRedirectedURL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(wszredirectedurl), dwsize, pdwlength).ok()
    }
    pub unsafe fn GetSecurityProvider(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSecurityProvider)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BindToStream(&self) -> ::windows::core::Result<super::Com::IStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.BindToStream)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_IndexServer\"`*"]
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn BindToFilter(&self) -> ::windows::core::Result<super::super::Storage::IndexServer::IFilter> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.BindToFilter)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IUrlAccessor3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUrlAccessor3 {}
impl ::core::fmt::Debug for IUrlAccessor3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUrlAccessor3").field(&self.0).finish()
    }
}
impl IUrlAccessor3 {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn AddRequestParameter(&self, pspec: *const super::Com::StructuredStorage::PROPSPEC, pvar: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.AddRequestParameter)(::windows::core::Vtable::as_raw(self), pspec, pvar).ok()
    }
    pub unsafe fn GetDocFormat(&self, wszdocformat: ::windows::core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDocFormat)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(wszdocformat), dwsize, pdwlength).ok()
    }
    pub unsafe fn GetCLSID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetCLSID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetHost(&self, wszhost: ::windows::core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetHost)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(wszhost), dwsize, pdwlength).ok()
    }
    pub unsafe fn IsDirectory(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.IsDirectory)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetSize(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetSize)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLastModified(&self) -> ::windows::core::Result<super::super::Foundation::FILETIME> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetLastModified)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFileName(&self, wszfilename: ::windows::core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetFileName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(wszfilename), dwsize, pdwlength).ok()
    }
    pub unsafe fn GetSecurityDescriptor(&self, psd: &mut [u8], pdwlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetSecurityDescriptor)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(psd.as_ptr()), psd.len() as _, pdwlength).ok()
    }
    pub unsafe fn GetRedirectedURL(&self, wszredirectedurl: ::windows::core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetRedirectedURL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(wszredirectedurl), dwsize, pdwlength).ok()
    }
    pub unsafe fn GetSecurityProvider(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetSecurityProvider)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BindToStream(&self) -> ::windows::core::Result<super::Com::IStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.BindToStream)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_IndexServer\"`*"]
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn BindToFilter(&self) -> ::windows::core::Result<super::super::Storage::IndexServer::IFilter> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.BindToFilter)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDisplayUrl(&self, wszdocurl: ::windows::core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDisplayUrl)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(wszdocurl), dwsize, pdwlength).ok()
    }
    pub unsafe fn IsDocument(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.IsDocument)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetCodePage(&self, wszcodepage: ::windows::core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetCodePage)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(wszcodepage), dwsize, pdwlength).ok()
    }
}
impl ::core::cmp::PartialEq for IUrlAccessor4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUrlAccessor4 {}
impl ::core::fmt::Debug for IUrlAccessor4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUrlAccessor4").field(&self.0).finish()
    }
}
impl IUrlAccessor4 {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn AddRequestParameter(&self, pspec: *const super::Com::StructuredStorage::PROPSPEC, pvar: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.AddRequestParameter)(::windows::core::Vtable::as_raw(self), pspec, pvar).ok()
    }
    pub unsafe fn GetDocFormat(&self, wszdocformat: ::windows::core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetDocFormat)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(wszdocformat), dwsize, pdwlength).ok()
    }
    pub unsafe fn GetCLSID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetCLSID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetHost(&self, wszhost: ::windows::core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetHost)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(wszhost), dwsize, pdwlength).ok()
    }
    pub unsafe fn IsDirectory(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.IsDirectory)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetSize(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetSize)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLastModified(&self) -> ::windows::core::Result<super::super::Foundation::FILETIME> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetLastModified)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFileName(&self, wszfilename: ::windows::core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFileName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(wszfilename), dwsize, pdwlength).ok()
    }
    pub unsafe fn GetSecurityDescriptor(&self, psd: &mut [u8], pdwlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetSecurityDescriptor)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(psd.as_ptr()), psd.len() as _, pdwlength).ok()
    }
    pub unsafe fn GetRedirectedURL(&self, wszredirectedurl: ::windows::core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetRedirectedURL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(wszredirectedurl), dwsize, pdwlength).ok()
    }
    pub unsafe fn GetSecurityProvider(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetSecurityProvider)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn BindToStream(&self) -> ::windows::core::Result<super::Com::IStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.BindToStream)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_IndexServer\"`*"]
    #[cfg(feature = "Win32_Storage_IndexServer")]
    pub unsafe fn BindToFilter(&self) -> ::windows::core::Result<super::super::Storage::IndexServer::IFilter> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.BindToFilter)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDisplayUrl(&self, wszdocurl: ::windows::core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDisplayUrl)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(wszdocurl), dwsize, pdwlength).ok()
    }
    pub unsafe fn IsDocument(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.IsDocument)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetCodePage(&self, wszcodepage: ::windows::core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetCodePage)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(wszcodepage), dwsize, pdwlength).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetImpersonationSidBlobs<P0>(&self, pcwszurl: P0, pcsidcount: *mut u32, ppsidblobs: *mut *mut super::Com::BLOB) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetImpersonationSidBlobs)(::windows::core::Vtable::as_raw(self), pcwszurl.into().abi(), pcsidcount, ppsidblobs).ok()
    }
}
impl ::core::cmp::PartialEq for IViewChapter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IViewChapter {}
impl ::core::fmt::Debug for IViewChapter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IViewChapter").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IViewFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IViewFilter {}
impl ::core::fmt::Debug for IViewFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IViewFilter").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IViewRowset {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IViewRowset {}
impl ::core::fmt::Debug for IViewRowset {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IViewRowset").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IViewSort {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IViewSort {}
impl ::core::fmt::Debug for IViewSort {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IViewSort").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWordBreaker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWordBreaker {}
impl ::core::fmt::Debug for IWordBreaker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWordBreaker").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWordFormSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWordFormSink {}
impl ::core::fmt::Debug for IWordFormSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWordFormSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWordSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWordSink {}
impl ::core::fmt::Debug for IWordSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWordSink").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::default::Default for KAGGETDIAG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for KAGREQDIAG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for KAGREQDIAG {
    fn eq(&self, other: &Self) -> bool {
        self.ulDiagFlags == other.ulDiagFlags && self.vt == other.vt && self.sDiagField == other.sDiagField
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for KAGREQDIAG {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for KAGREQDIAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("KAGREQDIAG").field("ulDiagFlags", &self.ulDiagFlags).field("vt", &self.vt).field("sDiagField", &self.sDiagField).finish()
    }
}
impl ::core::default::Default for KAGREQDIAGFLAGSENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KAGREQDIAGFLAGSENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KAGREQDIAGFLAGSENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for LOCKMODEENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for LOCKMODEENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LOCKMODEENUM").field(&self.0).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for MDAXISINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for MDAXISINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MSDSDBINITPROPENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MSDSDBINITPROPENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSDSDBINITPROPENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for MSDSSESSIONPROPENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MSDSSESSIONPROPENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MSDSSESSIONPROPENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for NAMED_ENTITY_CERTAINTY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NAMED_ENTITY_CERTAINTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NAMED_ENTITY_CERTAINTY").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for NATLANGUAGERESTRICTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for NODERESTRICTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::PartialEq for NODERESTRICTION {
    fn eq(&self, other: &Self) -> bool {
        self.cRes == other.cRes && self.paRes == other.paRes && self.reserved == other.reserved
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::Eq for NODERESTRICTION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::fmt::Debug for NODERESTRICTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NODERESTRICTION").field("cRes", &self.cRes).field("paRes", &self.paRes).field("reserved", &self.reserved).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for NOTRESTRICTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::PartialEq for NOTRESTRICTION {
    fn eq(&self, other: &Self) -> bool {
        self.pRes == other.pRes
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::Eq for NOTRESTRICTION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::fmt::Debug for NOTRESTRICTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NOTRESTRICTION").field("pRes", &self.pRes).finish()
    }
}
impl ::core::default::Default for ODBC_VS_ARGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for OLEDBSimpleProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OLEDBSimpleProvider {}
impl ::core::fmt::Debug for OLEDBSimpleProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OLEDBSimpleProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for OLEDBSimpleProviderListener {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for OLEDBSimpleProviderListener {}
impl ::core::fmt::Debug for OLEDBSimpleProviderListener {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OLEDBSimpleProviderListener").field(&self.0).finish()
    }
}
impl ::core::default::Default for OSPCOMP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OSPCOMP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OSPCOMP").field(&self.0).finish()
    }
}
impl ::core::default::Default for OSPFIND {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OSPFIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OSPFIND").field(&self.0).finish()
    }
}
impl ::core::default::Default for OSPFORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OSPFORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OSPFORMAT").field(&self.0).finish()
    }
}
impl ::core::default::Default for OSPRW {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OSPRW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OSPRW").field(&self.0).finish()
    }
}
impl ::core::default::Default for OSPXFER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OSPXFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OSPXFER").field(&self.0).finish()
    }
}
impl ::core::default::Default for PRIORITIZE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PRIORITIZE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PRIORITIZE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for PRIORITY_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PRIORITY_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PRIORITY_LEVEL").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for PROPERTYRESTRICTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PROXY_ACCESS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PROXY_ACCESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROXY_ACCESS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PROXY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PROXY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.pcwszUserAgent == other.pcwszUserAgent && self.paUseProxy == other.paUseProxy && self.fLocalBypass == other.fLocalBypass && self.dwPortNumber == other.dwPortNumber && self.pcwszProxyName == other.pcwszProxyName && self.pcwszBypassList == other.pcwszBypassList
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PROXY_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PROXY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROXY_INFO").field("dwSize", &self.dwSize).field("pcwszUserAgent", &self.pcwszUserAgent).field("paUseProxy", &self.paUseProxy).field("fLocalBypass", &self.fLocalBypass).field("dwPortNumber", &self.dwPortNumber).field("pcwszProxyName", &self.pcwszProxyName).field("pcwszBypassList", &self.pcwszBypassList).finish()
    }
}
impl ::core::default::Default for QUERY_PARSER_MANAGER_OPTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for QUERY_PARSER_MANAGER_OPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QUERY_PARSER_MANAGER_OPTION").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for RANGECATEGORIZE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::PartialEq for RANGECATEGORIZE {
    fn eq(&self, other: &Self) -> bool {
        self.cRange == other.cRange && self.aRangeBegin == other.aRangeBegin
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::Eq for RANGECATEGORIZE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::fmt::Debug for RANGECATEGORIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RANGECATEGORIZE").field("cRange", &self.cRange).field("aRangeBegin", &self.aRangeBegin).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for RESTRICTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl ::core::default::Default for RMTPACK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
impl ::core::default::Default for RMTPACK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for ROWSETEVENT_ITEMSTATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ROWSETEVENT_ITEMSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ROWSETEVENT_ITEMSTATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for ROWSETEVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ROWSETEVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ROWSETEVENT_TYPE").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for SEARCH_COLUMN_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SEARCH_INDEXING_PHASE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SEARCH_INDEXING_PHASE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SEARCH_INDEXING_PHASE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for SEARCH_ITEM_CHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for SEARCH_ITEM_CHANGE {
    fn eq(&self, other: &Self) -> bool {
        self.Change == other.Change && self.Priority == other.Priority && self.pUserData == other.pUserData && self.lpwszURL == other.lpwszURL && self.lpwszOldURL == other.lpwszOldURL
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for SEARCH_ITEM_CHANGE {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for SEARCH_ITEM_CHANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SEARCH_ITEM_CHANGE").field("Change", &self.Change).field("Priority", &self.Priority).field("pUserData", &self.pUserData).field("lpwszURL", &self.lpwszURL).field("lpwszOldURL", &self.lpwszOldURL).finish()
    }
}
impl ::core::default::Default for SEARCH_ITEM_INDEXING_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SEARCH_ITEM_INDEXING_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.dwDocID == other.dwDocID && self.hrIndexingStatus == other.hrIndexingStatus
    }
}
impl ::core::cmp::Eq for SEARCH_ITEM_INDEXING_STATUS {}
impl ::core::fmt::Debug for SEARCH_ITEM_INDEXING_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SEARCH_ITEM_INDEXING_STATUS").field("dwDocID", &self.dwDocID).field("hrIndexingStatus", &self.hrIndexingStatus).finish()
    }
}
impl ::core::default::Default for SEARCH_ITEM_PERSISTENT_CHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SEARCH_ITEM_PERSISTENT_CHANGE {
    fn eq(&self, other: &Self) -> bool {
        self.Change == other.Change && self.URL == other.URL && self.OldURL == other.OldURL && self.Priority == other.Priority
    }
}
impl ::core::cmp::Eq for SEARCH_ITEM_PERSISTENT_CHANGE {}
impl ::core::fmt::Debug for SEARCH_ITEM_PERSISTENT_CHANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SEARCH_ITEM_PERSISTENT_CHANGE").field("Change", &self.Change).field("URL", &self.URL).field("OldURL", &self.OldURL).field("Priority", &self.Priority).finish()
    }
}
impl ::core::default::Default for SEARCH_KIND_OF_CHANGE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SEARCH_KIND_OF_CHANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SEARCH_KIND_OF_CHANGE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SEARCH_NOTIFICATION_PRIORITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SEARCH_NOTIFICATION_PRIORITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SEARCH_NOTIFICATION_PRIORITY").field(&self.0).finish()
    }
}
impl ::core::default::Default for SEARCH_QUERY_SYNTAX {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SEARCH_QUERY_SYNTAX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SEARCH_QUERY_SYNTAX").field(&self.0).finish()
    }
}
impl ::core::default::Default for SEARCH_TERM_EXPANSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SEARCH_TERM_EXPANSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SEARCH_TERM_EXPANSION").field(&self.0).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Storage_IndexServer")]
impl ::core::default::Default for SEC_OBJECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Storage_IndexServer")]
impl ::core::default::Default for SEC_OBJECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Storage_IndexServer")]
impl ::core::default::Default for SEC_OBJECT_ELEMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Storage_IndexServer")]
impl ::core::default::Default for SEC_OBJECT_ELEMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for SORTKEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for SORTSET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::PartialEq for SORTSET {
    fn eq(&self, other: &Self) -> bool {
        self.cCol == other.cCol && self.aCol == other.aCol
    }
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::Eq for SORTSET {}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::fmt::Debug for SORTSET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SORTSET").field("cCol", &self.cCol).field("aCol", &self.aCol).finish()
    }
}
impl ::core::default::Default for SQLINTERVAL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SQLINTERVAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SQLINTERVAL").field(&self.0).finish()
    }
}
impl ::core::default::Default for SQLPERF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SQLPERF {
    fn eq(&self, other: &Self) -> bool {
        self.TimerResolution == other.TimerResolution
            && self.SQLidu == other.SQLidu
            && self.SQLiduRows == other.SQLiduRows
            && self.SQLSelects == other.SQLSelects
            && self.SQLSelectRows == other.SQLSelectRows
            && self.Transactions == other.Transactions
            && self.SQLPrepares == other.SQLPrepares
            && self.ExecDirects == other.ExecDirects
            && self.SQLExecutes == other.SQLExecutes
            && self.CursorOpens == other.CursorOpens
            && self.CursorSize == other.CursorSize
            && self.CursorUsed == other.CursorUsed
            && self.PercentCursorUsed == other.PercentCursorUsed
            && self.AvgFetchTime == other.AvgFetchTime
            && self.AvgCursorSize == other.AvgCursorSize
            && self.AvgCursorUsed == other.AvgCursorUsed
            && self.SQLFetchTime == other.SQLFetchTime
            && self.SQLFetchCount == other.SQLFetchCount
            && self.CurrentStmtCount == other.CurrentStmtCount
            && self.MaxOpenStmt == other.MaxOpenStmt
            && self.SumOpenStmt == other.SumOpenStmt
            && self.CurrentConnectionCount == other.CurrentConnectionCount
            && self.MaxConnectionsOpened == other.MaxConnectionsOpened
            && self.SumConnectionsOpened == other.SumConnectionsOpened
            && self.SumConnectiontime == other.SumConnectiontime
            && self.AvgTimeOpened == other.AvgTimeOpened
            && self.ServerRndTrips == other.ServerRndTrips
            && self.BuffersSent == other.BuffersSent
            && self.BuffersRec == other.BuffersRec
            && self.BytesSent == other.BytesSent
            && self.BytesRec == other.BytesRec
            && self.msExecutionTime == other.msExecutionTime
            && self.msNetWorkServerTime == other.msNetWorkServerTime
    }
}
impl ::core::cmp::Eq for SQLPERF {}
impl ::core::fmt::Debug for SQLPERF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SQLPERF")
            .field("TimerResolution", &self.TimerResolution)
            .field("SQLidu", &self.SQLidu)
            .field("SQLiduRows", &self.SQLiduRows)
            .field("SQLSelects", &self.SQLSelects)
            .field("SQLSelectRows", &self.SQLSelectRows)
            .field("Transactions", &self.Transactions)
            .field("SQLPrepares", &self.SQLPrepares)
            .field("ExecDirects", &self.ExecDirects)
            .field("SQLExecutes", &self.SQLExecutes)
            .field("CursorOpens", &self.CursorOpens)
            .field("CursorSize", &self.CursorSize)
            .field("CursorUsed", &self.CursorUsed)
            .field("PercentCursorUsed", &self.PercentCursorUsed)
            .field("AvgFetchTime", &self.AvgFetchTime)
            .field("AvgCursorSize", &self.AvgCursorSize)
            .field("AvgCursorUsed", &self.AvgCursorUsed)
            .field("SQLFetchTime", &self.SQLFetchTime)
            .field("SQLFetchCount", &self.SQLFetchCount)
            .field("CurrentStmtCount", &self.CurrentStmtCount)
            .field("MaxOpenStmt", &self.MaxOpenStmt)
            .field("SumOpenStmt", &self.SumOpenStmt)
            .field("CurrentConnectionCount", &self.CurrentConnectionCount)
            .field("MaxConnectionsOpened", &self.MaxConnectionsOpened)
            .field("SumConnectionsOpened", &self.SumConnectionsOpened)
            .field("SumConnectiontime", &self.SumConnectiontime)
            .field("AvgTimeOpened", &self.AvgTimeOpened)
            .field("ServerRndTrips", &self.ServerRndTrips)
            .field("BuffersSent", &self.BuffersSent)
            .field("BuffersRec", &self.BuffersRec)
            .field("BytesSent", &self.BytesSent)
            .field("BytesRec", &self.BytesRec)
            .field("msExecutionTime", &self.msExecutionTime)
            .field("msNetWorkServerTime", &self.msNetWorkServerTime)
            .finish()
    }
}
impl ::core::default::Default for SQLVARENUM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SQLVARENUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SQLVARENUM").field(&self.0).finish()
    }
}
impl ::core::default::Default for SQL_DAY_SECOND_STRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SQL_DAY_SECOND_STRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.day == other.day && self.hour == other.hour && self.minute == other.minute && self.second == other.second && self.fraction == other.fraction
    }
}
impl ::core::cmp::Eq for SQL_DAY_SECOND_STRUCT {}
impl ::core::fmt::Debug for SQL_DAY_SECOND_STRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SQL_DAY_SECOND_STRUCT").field("day", &self.day).field("hour", &self.hour).field("minute", &self.minute).field("second", &self.second).field("fraction", &self.fraction).finish()
    }
}
impl ::core::default::Default for SQL_INTERVAL_STRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SQL_NUMERIC_STRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SQL_NUMERIC_STRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.precision == other.precision && self.scale == other.scale && self.sign == other.sign && self.val == other.val
    }
}
impl ::core::cmp::Eq for SQL_NUMERIC_STRUCT {}
impl ::core::fmt::Debug for SQL_NUMERIC_STRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SQL_NUMERIC_STRUCT").field("precision", &self.precision).field("scale", &self.scale).field("sign", &self.sign).field("val", &self.val).finish()
    }
}
impl ::core::default::Default for SQL_YEAR_MONTH_STRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SQL_YEAR_MONTH_STRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.year == other.year && self.month == other.month
    }
}
impl ::core::cmp::Eq for SQL_YEAR_MONTH_STRUCT {}
impl ::core::fmt::Debug for SQL_YEAR_MONTH_STRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SQL_YEAR_MONTH_STRUCT").field("year", &self.year).field("month", &self.month).finish()
    }
}
impl ::core::default::Default for SSERRORINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SSERRORINFO {
    fn eq(&self, other: &Self) -> bool {
        self.pwszMessage == other.pwszMessage && self.pwszServer == other.pwszServer && self.pwszProcedure == other.pwszProcedure && self.lNative == other.lNative && self.bState == other.bState && self.bClass == other.bClass && self.wLineNumber == other.wLineNumber
    }
}
impl ::core::cmp::Eq for SSERRORINFO {}
impl ::core::fmt::Debug for SSERRORINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SSERRORINFO").field("pwszMessage", &self.pwszMessage).field("pwszServer", &self.pwszServer).field("pwszProcedure", &self.pwszProcedure).field("lNative", &self.lNative).field("bState", &self.bState).field("bClass", &self.bClass).field("wLineNumber", &self.wLineNumber).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for SSVARIANT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for STRUCTURED_QUERY_MULTIOPTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STRUCTURED_QUERY_MULTIOPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STRUCTURED_QUERY_MULTIOPTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for STRUCTURED_QUERY_PARSE_ERROR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STRUCTURED_QUERY_PARSE_ERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STRUCTURED_QUERY_PARSE_ERROR").field(&self.0).finish()
    }
}
impl ::core::default::Default for STRUCTURED_QUERY_RESOLVE_OPTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STRUCTURED_QUERY_RESOLVE_OPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STRUCTURED_QUERY_RESOLVE_OPTION").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for STRUCTURED_QUERY_RESOLVE_OPTION {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for STRUCTURED_QUERY_RESOLVE_OPTION {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for STRUCTURED_QUERY_RESOLVE_OPTION {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for STRUCTURED_QUERY_RESOLVE_OPTION {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for STRUCTURED_QUERY_RESOLVE_OPTION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for STRUCTURED_QUERY_SINGLE_OPTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STRUCTURED_QUERY_SINGLE_OPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STRUCTURED_QUERY_SINGLE_OPTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for STRUCTURED_QUERY_SYNTAX {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STRUCTURED_QUERY_SYNTAX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STRUCTURED_QUERY_SYNTAX").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SUBSCRIPTIONINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SUBSCRIPTIONINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.fUpdateFlags == other.fUpdateFlags
            && self.schedule == other.schedule
            && self.customGroupCookie == other.customGroupCookie
            && self.pTrigger == other.pTrigger
            && self.dwRecurseLevels == other.dwRecurseLevels
            && self.fWebcrawlerFlags == other.fWebcrawlerFlags
            && self.bMailNotification == other.bMailNotification
            && self.bGleam == other.bGleam
            && self.bChangesOnly == other.bChangesOnly
            && self.bNeedPassword == other.bNeedPassword
            && self.fChannelFlags == other.fChannelFlags
            && self.bstrUserName == other.bstrUserName
            && self.bstrPassword == other.bstrPassword
            && self.bstrFriendlyName == other.bstrFriendlyName
            && self.dwMaxSizeKB == other.dwMaxSizeKB
            && self.subType == other.subType
            && self.fTaskFlags == other.fTaskFlags
            && self.dwReserved == other.dwReserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SUBSCRIPTIONINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SUBSCRIPTIONINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SUBSCRIPTIONINFO")
            .field("cbSize", &self.cbSize)
            .field("fUpdateFlags", &self.fUpdateFlags)
            .field("schedule", &self.schedule)
            .field("customGroupCookie", &self.customGroupCookie)
            .field("pTrigger", &self.pTrigger)
            .field("dwRecurseLevels", &self.dwRecurseLevels)
            .field("fWebcrawlerFlags", &self.fWebcrawlerFlags)
            .field("bMailNotification", &self.bMailNotification)
            .field("bGleam", &self.bGleam)
            .field("bChangesOnly", &self.bChangesOnly)
            .field("bNeedPassword", &self.bNeedPassword)
            .field("fChannelFlags", &self.fChannelFlags)
            .field("bstrUserName", &self.bstrUserName)
            .field("bstrPassword", &self.bstrPassword)
            .field("bstrFriendlyName", &self.bstrFriendlyName)
            .field("dwMaxSizeKB", &self.dwMaxSizeKB)
            .field("subType", &self.subType)
            .field("fTaskFlags", &self.fTaskFlags)
            .field("dwReserved", &self.dwReserved)
            .finish()
    }
}
impl ::core::default::Default for SUBSCRIPTIONINFOFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SUBSCRIPTIONINFOFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SUBSCRIPTIONINFOFLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for SUBSCRIPTIONITEMINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SUBSCRIPTIONITEMINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwFlags == other.dwFlags && self.dwPriority == other.dwPriority && self.ScheduleGroup == other.ScheduleGroup && self.clsidAgent == other.clsidAgent
    }
}
impl ::core::cmp::Eq for SUBSCRIPTIONITEMINFO {}
impl ::core::fmt::Debug for SUBSCRIPTIONITEMINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SUBSCRIPTIONITEMINFO").field("cbSize", &self.cbSize).field("dwFlags", &self.dwFlags).field("dwPriority", &self.dwPriority).field("ScheduleGroup", &self.ScheduleGroup).field("clsidAgent", &self.clsidAgent).finish()
    }
}
impl ::core::default::Default for SUBSCRIPTIONSCHEDULE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SUBSCRIPTIONSCHEDULE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SUBSCRIPTIONSCHEDULE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SUBSCRIPTIONTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SUBSCRIPTIONTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SUBSCRIPTIONTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for TEXT_SOURCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for TIMEOUT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TIMEOUT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwConnectTimeout == other.dwConnectTimeout && self.dwDataTimeout == other.dwDataTimeout
    }
}
impl ::core::cmp::Eq for TIMEOUT_INFO {}
impl ::core::fmt::Debug for TIMEOUT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TIMEOUT_INFO").field("dwSize", &self.dwSize).field("dwConnectTimeout", &self.dwConnectTimeout).field("dwDataTimeout", &self.dwDataTimeout).finish()
    }
}
impl ::core::default::Default for TIMESTAMP_STRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TIMESTAMP_STRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.year == other.year && self.month == other.month && self.day == other.day && self.hour == other.hour && self.minute == other.minute && self.second == other.second && self.fraction == other.fraction
    }
}
impl ::core::cmp::Eq for TIMESTAMP_STRUCT {}
impl ::core::fmt::Debug for TIMESTAMP_STRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TIMESTAMP_STRUCT").field("year", &self.year).field("month", &self.month).field("day", &self.day).field("hour", &self.hour).field("minute", &self.minute).field("second", &self.second).field("fraction", &self.fraction).finish()
    }
}
impl ::core::default::Default for TIME_STRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TIME_STRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.hour == other.hour && self.minute == other.minute && self.second == other.second
    }
}
impl ::core::cmp::Eq for TIME_STRUCT {}
impl ::core::fmt::Debug for TIME_STRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TIME_STRUCT").field("hour", &self.hour).field("minute", &self.minute).field("second", &self.second).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::default::Default for VECTORRESTRICTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::PartialEq for VECTORRESTRICTION {
    fn eq(&self, other: &Self) -> bool {
        self.Node == other.Node && self.RankMethod == other.RankMethod
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::cmp::Eq for VECTORRESTRICTION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::core::fmt::Debug for VECTORRESTRICTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VECTORRESTRICTION").field("Node", &self.Node).field("RankMethod", &self.RankMethod).finish()
    }
}
impl ::core::default::Default for WEBCRAWL_RECURSEFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WEBCRAWL_RECURSEFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WEBCRAWL_RECURSEFLAGS").field(&self.0).finish()
    }
}

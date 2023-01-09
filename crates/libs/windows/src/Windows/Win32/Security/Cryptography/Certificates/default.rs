impl ::core::default::Default for ADDED_CERT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ADDED_CERT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ADDED_CERT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for AlgorithmFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AlgorithmFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AlgorithmFlags").field(&self.0).finish()
    }
}
impl ::core::default::Default for AlgorithmOperationFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AlgorithmOperationFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AlgorithmOperationFlags").field(&self.0).finish()
    }
}
impl ::core::default::Default for AlgorithmType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AlgorithmType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AlgorithmType").field(&self.0).finish()
    }
}
impl ::core::default::Default for AlternativeNameType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AlternativeNameType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AlternativeNameType").field(&self.0).finish()
    }
}
impl ::core::default::Default for CAINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CAINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.CAType == other.CAType && self.cCASignatureCerts == other.cCASignatureCerts && self.cCAExchangeCerts == other.cCAExchangeCerts && self.cExitModules == other.cExitModules && self.lPropIdMax == other.lPropIdMax && self.lRoleSeparationEnabled == other.lRoleSeparationEnabled && self.cKRACertUsedCount == other.cKRACertUsedCount && self.cKRACertCount == other.cKRACertCount && self.fAdvancedServer == other.fAdvancedServer
    }
}
impl ::core::cmp::Eq for CAINFO {}
impl ::core::fmt::Debug for CAINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CAINFO")
            .field("cbSize", &self.cbSize)
            .field("CAType", &self.CAType)
            .field("cCASignatureCerts", &self.cCASignatureCerts)
            .field("cCAExchangeCerts", &self.cCAExchangeCerts)
            .field("cExitModules", &self.cExitModules)
            .field("lPropIdMax", &self.lPropIdMax)
            .field("lRoleSeparationEnabled", &self.lRoleSeparationEnabled)
            .field("cKRACertUsedCount", &self.cKRACertUsedCount)
            .field("cKRACertCount", &self.cKRACertCount)
            .field("fAdvancedServer", &self.fAdvancedServer)
            .finish()
    }
}
impl ::core::default::Default for CERTADMIN_GET_ROLES_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CERTADMIN_GET_ROLES_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CERTADMIN_GET_ROLES_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CERTADMIN_GET_ROLES_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CERTADMIN_GET_ROLES_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CERTADMIN_GET_ROLES_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CERTADMIN_GET_ROLES_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CERTADMIN_GET_ROLES_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CERTENROLL_OBJECTID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CERTENROLL_OBJECTID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CERTENROLL_OBJECTID").field(&self.0).finish()
    }
}
impl ::core::default::Default for CERTENROLL_PROPERTYID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CERTENROLL_PROPERTYID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CERTENROLL_PROPERTYID").field(&self.0).finish()
    }
}
impl ::core::default::Default for CERTTRANSBLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERTTRANSBLOB {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.pb == other.pb
    }
}
impl ::core::cmp::Eq for CERTTRANSBLOB {}
impl ::core::fmt::Debug for CERTTRANSBLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERTTRANSBLOB").field("cb", &self.cb).field("pb", &self.pb).finish()
    }
}
impl ::core::default::Default for CERTVIEWRESTRICTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CERTVIEWRESTRICTION {
    fn eq(&self, other: &Self) -> bool {
        self.ColumnIndex == other.ColumnIndex && self.SeekOperator == other.SeekOperator && self.SortOrder == other.SortOrder && self.pbValue == other.pbValue && self.cbValue == other.cbValue
    }
}
impl ::core::cmp::Eq for CERTVIEWRESTRICTION {}
impl ::core::fmt::Debug for CERTVIEWRESTRICTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CERTVIEWRESTRICTION").field("ColumnIndex", &self.ColumnIndex).field("SeekOperator", &self.SeekOperator).field("SortOrder", &self.SortOrder).field("pbValue", &self.pbValue).field("cbValue", &self.cbValue).finish()
    }
}
impl ::core::default::Default for CERT_ALT_NAME {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CERT_ALT_NAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CERT_ALT_NAME").field(&self.0).finish()
    }
}
impl ::core::default::Default for CERT_CREATE_REQUEST_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CERT_CREATE_REQUEST_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CERT_CREATE_REQUEST_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for CERT_DELETE_ROW_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CERT_DELETE_ROW_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CERT_DELETE_ROW_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for CERT_EXIT_EVENT_MASK {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CERT_EXIT_EVENT_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CERT_EXIT_EVENT_MASK").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CERT_EXIT_EVENT_MASK {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CERT_EXIT_EVENT_MASK {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CERT_EXIT_EVENT_MASK {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CERT_EXIT_EVENT_MASK {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CERT_EXIT_EVENT_MASK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CERT_GET_CONFIG_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CERT_GET_CONFIG_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CERT_GET_CONFIG_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for CERT_IMPORT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CERT_IMPORT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CERT_IMPORT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for CERT_PROPERTY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CERT_PROPERTY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CERT_PROPERTY_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CERT_REQUEST_OUT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CERT_REQUEST_OUT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CERT_REQUEST_OUT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CERT_VIEW_COLUMN_INDEX {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CERT_VIEW_COLUMN_INDEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CERT_VIEW_COLUMN_INDEX").field(&self.0).finish()
    }
}
impl ::core::default::Default for CERT_VIEW_SEEK_OPERATOR_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CERT_VIEW_SEEK_OPERATOR_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CERT_VIEW_SEEK_OPERATOR_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for CRLRevocationReason {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CRLRevocationReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CRLRevocationReason").field(&self.0).finish()
    }
}
impl ::core::default::Default for CR_DISP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CR_DISP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CR_DISP").field(&self.0).finish()
    }
}
impl ::core::default::Default for CSBACKUP_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CSBACKUP_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CSBACKUP_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CSEDB_RSTMAPW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CSEDB_RSTMAPW {
    fn eq(&self, other: &Self) -> bool {
        self.pwszDatabaseName == other.pwszDatabaseName && self.pwszNewDatabaseName == other.pwszNewDatabaseName
    }
}
impl ::core::cmp::Eq for CSEDB_RSTMAPW {}
impl ::core::fmt::Debug for CSEDB_RSTMAPW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CSEDB_RSTMAPW").field("pwszDatabaseName", &self.pwszDatabaseName).field("pwszNewDatabaseName", &self.pwszNewDatabaseName).finish()
    }
}
impl ::core::default::Default for CVRC_COLUMN {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CVRC_COLUMN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CVRC_COLUMN").field(&self.0).finish()
    }
}
impl ::core::default::Default for CVRC_TABLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CVRC_TABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CVRC_TABLE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CommitTemplateFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CommitTemplateFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CommitTemplateFlags").field(&self.0).finish()
    }
}
impl ::core::default::Default for DelayRetryAction {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DelayRetryAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DelayRetryAction").field(&self.0).finish()
    }
}
impl ::core::default::Default for ENUM_CATYPES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ENUM_CATYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ENUM_CATYPES").field(&self.0).finish()
    }
}
impl ::core::default::Default for ENUM_CERT_COLUMN_VALUE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ENUM_CERT_COLUMN_VALUE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ENUM_CERT_COLUMN_VALUE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for EncodingType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EncodingType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EncodingType").field(&self.0).finish()
    }
}
impl ::core::default::Default for EnrollmentCAProperty {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EnrollmentCAProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EnrollmentCAProperty").field(&self.0).finish()
    }
}
impl ::core::default::Default for EnrollmentDisplayStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EnrollmentDisplayStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EnrollmentDisplayStatus").field(&self.0).finish()
    }
}
impl ::core::default::Default for EnrollmentEnrollStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EnrollmentEnrollStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EnrollmentEnrollStatus").field(&self.0).finish()
    }
}
impl ::core::default::Default for EnrollmentPolicyFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EnrollmentPolicyFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EnrollmentPolicyFlags").field(&self.0).finish()
    }
}
impl ::core::default::Default for EnrollmentPolicyServerPropertyFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EnrollmentPolicyServerPropertyFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EnrollmentPolicyServerPropertyFlags").field(&self.0).finish()
    }
}
impl ::core::default::Default for EnrollmentSelectionStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EnrollmentSelectionStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EnrollmentSelectionStatus").field(&self.0).finish()
    }
}
impl ::core::default::Default for EnrollmentTemplateProperty {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EnrollmentTemplateProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EnrollmentTemplateProperty").field(&self.0).finish()
    }
}
impl ::core::default::Default for FULL_RESPONSE_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FULL_RESPONSE_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FULL_RESPONSE_PROPERTY_ID").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAlternativeName {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAlternativeName {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAlternativeName {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAlternativeName").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IAlternativeNames {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IAlternativeNames {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IAlternativeNames {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAlternativeNames").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IBinaryConverter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IBinaryConverter {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IBinaryConverter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBinaryConverter").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IBinaryConverter2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IBinaryConverter2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IBinaryConverter2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBinaryConverter2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IBinaryConverter2 {
    pub unsafe fn StringToString(&self, strencodedin: &::windows::core::BSTR, encodingin: EncodingType, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.StringToString)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strencodedin), encodingin, encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn VariantByteArrayToString(&self, pvarbytearray: *const super::super::super::System::Com::VARIANT, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.VariantByteArrayToString)(::windows::core::Vtable::as_raw(self), pvarbytearray, encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn StringToVariantByteArray(&self, strencoded: &::windows::core::BSTR, encoding: EncodingType) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.StringToVariantByteArray)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strencoded), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICEnroll {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICEnroll {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICEnroll {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICEnroll").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICEnroll2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICEnroll2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICEnroll2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICEnroll2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ICEnroll2 {
    pub unsafe fn createFilePKCS10(&self, dnname: &::windows::core::BSTR, usage: &::windows::core::BSTR, wszpkcs10filename: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.createFilePKCS10)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(dnname), ::core::mem::transmute_copy(usage), ::core::mem::transmute_copy(wszpkcs10filename)).ok()
    }
    pub unsafe fn acceptFilePKCS7(&self, wszpkcs7filename: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.acceptFilePKCS7)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(wszpkcs7filename)).ok()
    }
    pub unsafe fn createPKCS10(&self, dnname: &::windows::core::BSTR, usage: &::windows::core::BSTR) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.createPKCS10)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(dnname), ::core::mem::transmute_copy(usage), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn acceptPKCS7(&self, pkcs7: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.acceptPKCS7)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(pkcs7)).ok()
    }
    pub unsafe fn getCertFromPKCS7(&self, wszpkcs7: &::windows::core::BSTR) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.getCertFromPKCS7)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(wszpkcs7), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn enumProviders(&self, dwindex: i32, dwflags: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.enumProviders)(::windows::core::Vtable::as_raw(self), dwindex, dwflags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn enumContainers(&self, dwindex: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.enumContainers)(::windows::core::Vtable::as_raw(self), dwindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn freeRequestInfo(&self, pkcs7orpkcs10: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.freeRequestInfo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(pkcs7orpkcs10)).ok()
    }
    pub unsafe fn MyStoreName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.MyStoreName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMyStoreName(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetMyStoreName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname)).ok()
    }
    pub unsafe fn MyStoreType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.MyStoreType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMyStoreType(&self, bstrtype: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetMyStoreType)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrtype)).ok()
    }
    pub unsafe fn MyStoreFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.MyStoreFlags)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMyStoreFlags(&self, dwflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetMyStoreFlags)(::windows::core::Vtable::as_raw(self), dwflags).ok()
    }
    pub unsafe fn CAStoreName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CAStoreName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetCAStoreName(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetCAStoreName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname)).ok()
    }
    pub unsafe fn CAStoreType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CAStoreType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetCAStoreType(&self, bstrtype: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetCAStoreType)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrtype)).ok()
    }
    pub unsafe fn CAStoreFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CAStoreFlags)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetCAStoreFlags(&self, dwflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetCAStoreFlags)(::windows::core::Vtable::as_raw(self), dwflags).ok()
    }
    pub unsafe fn RootStoreName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.RootStoreName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetRootStoreName(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetRootStoreName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname)).ok()
    }
    pub unsafe fn RootStoreType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.RootStoreType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetRootStoreType(&self, bstrtype: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetRootStoreType)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrtype)).ok()
    }
    pub unsafe fn RootStoreFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.RootStoreFlags)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetRootStoreFlags(&self, dwflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetRootStoreFlags)(::windows::core::Vtable::as_raw(self), dwflags).ok()
    }
    pub unsafe fn RequestStoreName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.RequestStoreName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetRequestStoreName(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetRequestStoreName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname)).ok()
    }
    pub unsafe fn RequestStoreType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.RequestStoreType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetRequestStoreType(&self, bstrtype: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetRequestStoreType)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrtype)).ok()
    }
    pub unsafe fn RequestStoreFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.RequestStoreFlags)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetRequestStoreFlags(&self, dwflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetRequestStoreFlags)(::windows::core::Vtable::as_raw(self), dwflags).ok()
    }
    pub unsafe fn ContainerName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ContainerName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetContainerName(&self, bstrcontainer: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetContainerName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrcontainer)).ok()
    }
    pub unsafe fn ProviderName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ProviderName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetProviderName(&self, bstrprovider: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetProviderName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrprovider)).ok()
    }
    pub unsafe fn ProviderType(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ProviderType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetProviderType(&self, dwtype: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetProviderType)(::windows::core::Vtable::as_raw(self), dwtype).ok()
    }
    pub unsafe fn KeySpec(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.KeySpec)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetKeySpec(&self, dw: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetKeySpec)(::windows::core::Vtable::as_raw(self), dw).ok()
    }
    pub unsafe fn ProviderFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ProviderFlags)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetProviderFlags(&self, dwflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetProviderFlags)(::windows::core::Vtable::as_raw(self), dwflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UseExistingKeySet(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.UseExistingKeySet)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUseExistingKeySet<P0>(&self, fuseexistingkeys: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetUseExistingKeySet)(::windows::core::Vtable::as_raw(self), fuseexistingkeys.into()).ok()
    }
    pub unsafe fn GenKeyFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GenKeyFlags)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetGenKeyFlags(&self, dwflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetGenKeyFlags)(::windows::core::Vtable::as_raw(self), dwflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteRequestCert(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DeleteRequestCert)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDeleteRequestCert<P0>(&self, fdelete: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetDeleteRequestCert)(::windows::core::Vtable::as_raw(self), fdelete.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteCertToCSP(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.WriteCertToCSP)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetWriteCertToCSP<P0>(&self, fbool: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetWriteCertToCSP)(::windows::core::Vtable::as_raw(self), fbool.into()).ok()
    }
    pub unsafe fn SPCFileName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SPCFileName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetSPCFileName(&self, bstr: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetSPCFileName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstr)).ok()
    }
    pub unsafe fn PVKFileName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.PVKFileName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPVKFileName(&self, bstr: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPVKFileName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstr)).ok()
    }
    pub unsafe fn HashAlgorithm(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.HashAlgorithm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetHashAlgorithm(&self, bstr: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetHashAlgorithm)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstr)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICEnroll3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICEnroll3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICEnroll3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICEnroll3").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ICEnroll3 {
    pub unsafe fn createFilePKCS10(&self, dnname: &::windows::core::BSTR, usage: &::windows::core::BSTR, wszpkcs10filename: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.createFilePKCS10)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(dnname), ::core::mem::transmute_copy(usage), ::core::mem::transmute_copy(wszpkcs10filename)).ok()
    }
    pub unsafe fn acceptFilePKCS7(&self, wszpkcs7filename: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.acceptFilePKCS7)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(wszpkcs7filename)).ok()
    }
    pub unsafe fn createPKCS10(&self, dnname: &::windows::core::BSTR, usage: &::windows::core::BSTR) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.createPKCS10)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(dnname), ::core::mem::transmute_copy(usage), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn acceptPKCS7(&self, pkcs7: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.acceptPKCS7)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(pkcs7)).ok()
    }
    pub unsafe fn getCertFromPKCS7(&self, wszpkcs7: &::windows::core::BSTR) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.getCertFromPKCS7)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(wszpkcs7), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn enumProviders(&self, dwindex: i32, dwflags: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.enumProviders)(::windows::core::Vtable::as_raw(self), dwindex, dwflags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn enumContainers(&self, dwindex: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.enumContainers)(::windows::core::Vtable::as_raw(self), dwindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn freeRequestInfo(&self, pkcs7orpkcs10: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.freeRequestInfo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(pkcs7orpkcs10)).ok()
    }
    pub unsafe fn MyStoreName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.MyStoreName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMyStoreName(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetMyStoreName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname)).ok()
    }
    pub unsafe fn MyStoreType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.MyStoreType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMyStoreType(&self, bstrtype: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetMyStoreType)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrtype)).ok()
    }
    pub unsafe fn MyStoreFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.MyStoreFlags)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMyStoreFlags(&self, dwflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetMyStoreFlags)(::windows::core::Vtable::as_raw(self), dwflags).ok()
    }
    pub unsafe fn CAStoreName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CAStoreName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetCAStoreName(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetCAStoreName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname)).ok()
    }
    pub unsafe fn CAStoreType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CAStoreType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetCAStoreType(&self, bstrtype: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetCAStoreType)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrtype)).ok()
    }
    pub unsafe fn CAStoreFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CAStoreFlags)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetCAStoreFlags(&self, dwflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetCAStoreFlags)(::windows::core::Vtable::as_raw(self), dwflags).ok()
    }
    pub unsafe fn RootStoreName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.RootStoreName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetRootStoreName(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetRootStoreName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname)).ok()
    }
    pub unsafe fn RootStoreType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.RootStoreType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetRootStoreType(&self, bstrtype: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetRootStoreType)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrtype)).ok()
    }
    pub unsafe fn RootStoreFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.RootStoreFlags)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetRootStoreFlags(&self, dwflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetRootStoreFlags)(::windows::core::Vtable::as_raw(self), dwflags).ok()
    }
    pub unsafe fn RequestStoreName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.RequestStoreName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetRequestStoreName(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetRequestStoreName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname)).ok()
    }
    pub unsafe fn RequestStoreType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.RequestStoreType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetRequestStoreType(&self, bstrtype: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetRequestStoreType)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrtype)).ok()
    }
    pub unsafe fn RequestStoreFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.RequestStoreFlags)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetRequestStoreFlags(&self, dwflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetRequestStoreFlags)(::windows::core::Vtable::as_raw(self), dwflags).ok()
    }
    pub unsafe fn ContainerName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.ContainerName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetContainerName(&self, bstrcontainer: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetContainerName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrcontainer)).ok()
    }
    pub unsafe fn ProviderName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.ProviderName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetProviderName(&self, bstrprovider: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetProviderName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrprovider)).ok()
    }
    pub unsafe fn ProviderType(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.ProviderType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetProviderType(&self, dwtype: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetProviderType)(::windows::core::Vtable::as_raw(self), dwtype).ok()
    }
    pub unsafe fn KeySpec(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.KeySpec)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetKeySpec(&self, dw: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetKeySpec)(::windows::core::Vtable::as_raw(self), dw).ok()
    }
    pub unsafe fn ProviderFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.ProviderFlags)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetProviderFlags(&self, dwflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetProviderFlags)(::windows::core::Vtable::as_raw(self), dwflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UseExistingKeySet(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.UseExistingKeySet)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUseExistingKeySet<P0>(&self, fuseexistingkeys: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetUseExistingKeySet)(::windows::core::Vtable::as_raw(self), fuseexistingkeys.into()).ok()
    }
    pub unsafe fn GenKeyFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GenKeyFlags)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetGenKeyFlags(&self, dwflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetGenKeyFlags)(::windows::core::Vtable::as_raw(self), dwflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteRequestCert(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.DeleteRequestCert)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDeleteRequestCert<P0>(&self, fdelete: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetDeleteRequestCert)(::windows::core::Vtable::as_raw(self), fdelete.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteCertToCSP(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.WriteCertToCSP)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetWriteCertToCSP<P0>(&self, fbool: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetWriteCertToCSP)(::windows::core::Vtable::as_raw(self), fbool.into()).ok()
    }
    pub unsafe fn SPCFileName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.SPCFileName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetSPCFileName(&self, bstr: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetSPCFileName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstr)).ok()
    }
    pub unsafe fn PVKFileName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.PVKFileName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPVKFileName(&self, bstr: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPVKFileName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstr)).ok()
    }
    pub unsafe fn HashAlgorithm(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.HashAlgorithm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetHashAlgorithm(&self, bstr: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetHashAlgorithm)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstr)).ok()
    }
    pub unsafe fn addCertTypeToRequest(&self, certtype: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.addCertTypeToRequest)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(certtype)).ok()
    }
    pub unsafe fn addNameValuePairToSignature(&self, name: &::windows::core::BSTR, value: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.addNameValuePairToSignature)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteCertToUserDS(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.WriteCertToUserDS)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetWriteCertToUserDS<P0>(&self, fbool: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetWriteCertToUserDS)(::windows::core::Vtable::as_raw(self), fbool.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnableT61DNEncoding(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnableT61DNEncoding)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableT61DNEncoding<P0>(&self, fbool: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetEnableT61DNEncoding)(::windows::core::Vtable::as_raw(self), fbool.into()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICEnroll4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICEnroll4 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICEnroll4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICEnroll4").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ICEnroll4 {
    pub unsafe fn createFilePKCS10(&self, dnname: &::windows::core::BSTR, usage: &::windows::core::BSTR, wszpkcs10filename: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.createFilePKCS10)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(dnname), ::core::mem::transmute_copy(usage), ::core::mem::transmute_copy(wszpkcs10filename)).ok()
    }
    pub unsafe fn acceptFilePKCS7(&self, wszpkcs7filename: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.acceptFilePKCS7)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(wszpkcs7filename)).ok()
    }
    pub unsafe fn createPKCS10(&self, dnname: &::windows::core::BSTR, usage: &::windows::core::BSTR) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.createPKCS10)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(dnname), ::core::mem::transmute_copy(usage), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn acceptPKCS7(&self, pkcs7: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.acceptPKCS7)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(pkcs7)).ok()
    }
    pub unsafe fn getCertFromPKCS7(&self, wszpkcs7: &::windows::core::BSTR) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.getCertFromPKCS7)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(wszpkcs7), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn enumProviders(&self, dwindex: i32, dwflags: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.enumProviders)(::windows::core::Vtable::as_raw(self), dwindex, dwflags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn enumContainers(&self, dwindex: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.enumContainers)(::windows::core::Vtable::as_raw(self), dwindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn freeRequestInfo(&self, pkcs7orpkcs10: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.freeRequestInfo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(pkcs7orpkcs10)).ok()
    }
    pub unsafe fn MyStoreName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.MyStoreName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMyStoreName(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetMyStoreName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname)).ok()
    }
    pub unsafe fn MyStoreType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.MyStoreType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMyStoreType(&self, bstrtype: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetMyStoreType)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrtype)).ok()
    }
    pub unsafe fn MyStoreFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.MyStoreFlags)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMyStoreFlags(&self, dwflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetMyStoreFlags)(::windows::core::Vtable::as_raw(self), dwflags).ok()
    }
    pub unsafe fn CAStoreName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CAStoreName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetCAStoreName(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetCAStoreName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname)).ok()
    }
    pub unsafe fn CAStoreType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CAStoreType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetCAStoreType(&self, bstrtype: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetCAStoreType)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrtype)).ok()
    }
    pub unsafe fn CAStoreFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CAStoreFlags)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetCAStoreFlags(&self, dwflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetCAStoreFlags)(::windows::core::Vtable::as_raw(self), dwflags).ok()
    }
    pub unsafe fn RootStoreName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.RootStoreName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetRootStoreName(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetRootStoreName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname)).ok()
    }
    pub unsafe fn RootStoreType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.RootStoreType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetRootStoreType(&self, bstrtype: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetRootStoreType)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrtype)).ok()
    }
    pub unsafe fn RootStoreFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.RootStoreFlags)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetRootStoreFlags(&self, dwflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetRootStoreFlags)(::windows::core::Vtable::as_raw(self), dwflags).ok()
    }
    pub unsafe fn RequestStoreName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.RequestStoreName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetRequestStoreName(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetRequestStoreName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrname)).ok()
    }
    pub unsafe fn RequestStoreType(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.RequestStoreType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetRequestStoreType(&self, bstrtype: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetRequestStoreType)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrtype)).ok()
    }
    pub unsafe fn RequestStoreFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.RequestStoreFlags)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetRequestStoreFlags(&self, dwflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetRequestStoreFlags)(::windows::core::Vtable::as_raw(self), dwflags).ok()
    }
    pub unsafe fn ContainerName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ContainerName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetContainerName(&self, bstrcontainer: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetContainerName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrcontainer)).ok()
    }
    pub unsafe fn ProviderName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ProviderName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetProviderName(&self, bstrprovider: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetProviderName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstrprovider)).ok()
    }
    pub unsafe fn ProviderType(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ProviderType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetProviderType(&self, dwtype: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetProviderType)(::windows::core::Vtable::as_raw(self), dwtype).ok()
    }
    pub unsafe fn KeySpec(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.KeySpec)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetKeySpec(&self, dw: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetKeySpec)(::windows::core::Vtable::as_raw(self), dw).ok()
    }
    pub unsafe fn ProviderFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ProviderFlags)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetProviderFlags(&self, dwflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetProviderFlags)(::windows::core::Vtable::as_raw(self), dwflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UseExistingKeySet(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.UseExistingKeySet)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUseExistingKeySet<P0>(&self, fuseexistingkeys: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetUseExistingKeySet)(::windows::core::Vtable::as_raw(self), fuseexistingkeys.into()).ok()
    }
    pub unsafe fn GenKeyFlags(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GenKeyFlags)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetGenKeyFlags(&self, dwflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetGenKeyFlags)(::windows::core::Vtable::as_raw(self), dwflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteRequestCert(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DeleteRequestCert)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDeleteRequestCert<P0>(&self, fdelete: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetDeleteRequestCert)(::windows::core::Vtable::as_raw(self), fdelete.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteCertToCSP(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.WriteCertToCSP)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetWriteCertToCSP<P0>(&self, fbool: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetWriteCertToCSP)(::windows::core::Vtable::as_raw(self), fbool.into()).ok()
    }
    pub unsafe fn SPCFileName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SPCFileName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetSPCFileName(&self, bstr: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetSPCFileName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstr)).ok()
    }
    pub unsafe fn PVKFileName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.PVKFileName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPVKFileName(&self, bstr: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPVKFileName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstr)).ok()
    }
    pub unsafe fn HashAlgorithm(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.HashAlgorithm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetHashAlgorithm(&self, bstr: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetHashAlgorithm)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(bstr)).ok()
    }
    pub unsafe fn addCertTypeToRequest(&self, certtype: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.addCertTypeToRequest)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(certtype)).ok()
    }
    pub unsafe fn addNameValuePairToSignature(&self, name: &::windows::core::BSTR, value: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.addNameValuePairToSignature)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteCertToUserDS(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.WriteCertToUserDS)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetWriteCertToUserDS<P0>(&self, fbool: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetWriteCertToUserDS)(::windows::core::Vtable::as_raw(self), fbool.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnableT61DNEncoding(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.EnableT61DNEncoding)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableT61DNEncoding<P0>(&self, fbool: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetEnableT61DNEncoding)(::windows::core::Vtable::as_raw(self), fbool.into()).ok()
    }
    pub unsafe fn InstallPKCS7(&self, pkcs7: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.InstallPKCS7)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(pkcs7)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetSupportedKeySpec(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSupportedKeySpec)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetKeyLen<P0, P1>(&self, fmin: P0, fexchange: P1) -> ::windows::core::Result<i32>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetKeyLen)(::windows::core::Vtable::as_raw(self), fmin.into(), fexchange.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumAlgs(&self, dwindex: i32, algclass: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumAlgs)(::windows::core::Vtable::as_raw(self), dwindex, algclass, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAlgName(&self, algid: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetAlgName)(::windows::core::Vtable::as_raw(self), algid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReuseHardwareKeyIfUnableToGenNew<P0>(&self, freusehardwarekeyifunabletogennew: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetReuseHardwareKeyIfUnableToGenNew)(::windows::core::Vtable::as_raw(self), freusehardwarekeyifunabletogennew.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReuseHardwareKeyIfUnableToGenNew(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ReuseHardwareKeyIfUnableToGenNew)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetHashAlgID(&self, hashalgid: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetHashAlgID)(::windows::core::Vtable::as_raw(self), hashalgid).ok()
    }
    pub unsafe fn HashAlgID(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.HashAlgID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLimitExchangeKeyToEncipherment<P0>(&self, flimitexchangekeytoencipherment: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetLimitExchangeKeyToEncipherment)(::windows::core::Vtable::as_raw(self), flimitexchangekeytoencipherment.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LimitExchangeKeyToEncipherment(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.LimitExchangeKeyToEncipherment)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableSMIMECapabilities<P0>(&self, fenablesmimecapabilities: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetEnableSMIMECapabilities)(::windows::core::Vtable::as_raw(self), fenablesmimecapabilities.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnableSMIMECapabilities(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnableSMIMECapabilities)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICertAdmin {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICertAdmin {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICertAdmin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertAdmin").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICertAdmin2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICertAdmin2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICertAdmin2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertAdmin2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ICertAdmin2 {
    pub unsafe fn IsValidCertificate(&self, strconfig: &::windows::core::BSTR, strserialnumber: &::windows::core::BSTR) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsValidCertificate)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strconfig), ::core::mem::transmute_copy(strserialnumber), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetRevocationReason(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetRevocationReason)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RevokeCertificate(&self, strconfig: &::windows::core::BSTR, strserialnumber: &::windows::core::BSTR, reason: i32, date: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.RevokeCertificate)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strconfig), ::core::mem::transmute_copy(strserialnumber), reason, date).ok()
    }
    pub unsafe fn SetRequestAttributes(&self, strconfig: &::windows::core::BSTR, requestid: i32, strattributes: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetRequestAttributes)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strconfig), requestid, ::core::mem::transmute_copy(strattributes)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetCertificateExtension(&self, strconfig: &::windows::core::BSTR, requestid: i32, strextensionname: &::windows::core::BSTR, r#type: CERT_PROPERTY_TYPE, flags: i32, pvarvalue: *const super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetCertificateExtension)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strconfig), requestid, ::core::mem::transmute_copy(strextensionname), r#type, flags, pvarvalue).ok()
    }
    pub unsafe fn DenyRequest(&self, strconfig: &::windows::core::BSTR, requestid: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DenyRequest)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strconfig), requestid).ok()
    }
    pub unsafe fn ResubmitRequest(&self, strconfig: &::windows::core::BSTR, requestid: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ResubmitRequest)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strconfig), requestid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn PublishCRL(&self, strconfig: &::windows::core::BSTR, date: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.PublishCRL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strconfig), date).ok()
    }
    pub unsafe fn GetCRL(&self, strconfig: &::windows::core::BSTR, flags: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCRL)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strconfig), flags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ImportCertificate(&self, strconfig: &::windows::core::BSTR, strcertificate: &::windows::core::BSTR, flags: CERT_IMPORT_FLAGS) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ImportCertificate)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strconfig), ::core::mem::transmute_copy(strcertificate), flags, result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICertConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICertConfig {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICertConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertConfig").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICertConfig2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICertConfig2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICertConfig2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertConfig2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ICertConfig2 {
    pub unsafe fn Reset(&self, index: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Reset)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Next(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Next)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetField(&self, strfieldname: &::windows::core::BSTR) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetField)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strfieldname), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetConfig(&self, flags: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetConfig)(::windows::core::Vtable::as_raw(self), flags, result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICertEncodeAltName {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICertEncodeAltName {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICertEncodeAltName {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertEncodeAltName").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICertEncodeAltName2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICertEncodeAltName2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICertEncodeAltName2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertEncodeAltName2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ICertEncodeAltName2 {
    pub unsafe fn Decode(&self, strbinary: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Decode)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strbinary)).ok()
    }
    pub unsafe fn GetNameCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetNameCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetNameChoice(&self, nameindex: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetNameChoice)(::windows::core::Vtable::as_raw(self), nameindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetName(&self, nameindex: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetName)(::windows::core::Vtable::as_raw(self), nameindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Reset(&self, namecount: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Reset)(::windows::core::Vtable::as_raw(self), namecount).ok()
    }
    pub unsafe fn SetNameEntry(&self, nameindex: i32, namechoice: CERT_ALT_NAME, strname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetNameEntry)(::windows::core::Vtable::as_raw(self), nameindex, namechoice, ::core::mem::transmute_copy(strname)).ok()
    }
    pub unsafe fn Encode(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Encode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICertEncodeBitString {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICertEncodeBitString {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICertEncodeBitString {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertEncodeBitString").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICertEncodeBitString2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICertEncodeBitString2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICertEncodeBitString2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertEncodeBitString2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ICertEncodeBitString2 {
    pub unsafe fn Decode(&self, strbinary: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Decode)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strbinary)).ok()
    }
    pub unsafe fn GetBitCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBitCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetBitString(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBitString)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Encode(&self, bitcount: i32, strbitstring: &::windows::core::BSTR) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Encode)(::windows::core::Vtable::as_raw(self), bitcount, ::core::mem::transmute_copy(strbitstring), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICertEncodeCRLDistInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICertEncodeCRLDistInfo {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICertEncodeCRLDistInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertEncodeCRLDistInfo").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICertEncodeCRLDistInfo2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICertEncodeCRLDistInfo2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICertEncodeCRLDistInfo2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertEncodeCRLDistInfo2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ICertEncodeCRLDistInfo2 {
    pub unsafe fn Decode(&self, strbinary: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Decode)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strbinary)).ok()
    }
    pub unsafe fn GetDistPointCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDistPointCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetNameCount(&self, distpointindex: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetNameCount)(::windows::core::Vtable::as_raw(self), distpointindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetNameChoice(&self, distpointindex: i32, nameindex: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetNameChoice)(::windows::core::Vtable::as_raw(self), distpointindex, nameindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetName(&self, distpointindex: i32, nameindex: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetName)(::windows::core::Vtable::as_raw(self), distpointindex, nameindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Reset(&self, distpointcount: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Reset)(::windows::core::Vtable::as_raw(self), distpointcount).ok()
    }
    pub unsafe fn SetNameCount(&self, distpointindex: i32, namecount: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetNameCount)(::windows::core::Vtable::as_raw(self), distpointindex, namecount).ok()
    }
    pub unsafe fn SetNameEntry(&self, distpointindex: i32, nameindex: i32, namechoice: CERT_ALT_NAME, strname: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetNameEntry)(::windows::core::Vtable::as_raw(self), distpointindex, nameindex, namechoice, ::core::mem::transmute_copy(strname)).ok()
    }
    pub unsafe fn Encode(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Encode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICertEncodeDateArray {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICertEncodeDateArray {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICertEncodeDateArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertEncodeDateArray").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICertEncodeDateArray2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICertEncodeDateArray2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICertEncodeDateArray2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertEncodeDateArray2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ICertEncodeDateArray2 {
    pub unsafe fn Decode(&self, strbinary: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Decode)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strbinary)).ok()
    }
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetValue(&self, index: i32) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetValue)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Reset(&self, count: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Reset)(::windows::core::Vtable::as_raw(self), count).ok()
    }
    pub unsafe fn SetValue(&self, index: i32, value: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetValue)(::windows::core::Vtable::as_raw(self), index, value).ok()
    }
    pub unsafe fn Encode(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Encode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICertEncodeLongArray {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICertEncodeLongArray {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICertEncodeLongArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertEncodeLongArray").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICertEncodeLongArray2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICertEncodeLongArray2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICertEncodeLongArray2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertEncodeLongArray2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ICertEncodeLongArray2 {
    pub unsafe fn Decode(&self, strbinary: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Decode)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strbinary)).ok()
    }
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetValue(&self, index: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetValue)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Reset(&self, count: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Reset)(::windows::core::Vtable::as_raw(self), count).ok()
    }
    pub unsafe fn SetValue(&self, index: i32, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetValue)(::windows::core::Vtable::as_raw(self), index, value).ok()
    }
    pub unsafe fn Encode(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Encode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICertEncodeStringArray {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICertEncodeStringArray {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICertEncodeStringArray {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertEncodeStringArray").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICertEncodeStringArray2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICertEncodeStringArray2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICertEncodeStringArray2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertEncodeStringArray2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ICertEncodeStringArray2 {
    pub unsafe fn Decode(&self, strbinary: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Decode)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strbinary)).ok()
    }
    pub unsafe fn GetStringType(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStringType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetValue(&self, index: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetValue)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Reset(&self, count: i32, stringtype: super::CERT_RDN_ATTR_VALUE_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Reset)(::windows::core::Vtable::as_raw(self), count, stringtype).ok()
    }
    pub unsafe fn SetValue(&self, index: i32, str: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetValue)(::windows::core::Vtable::as_raw(self), index, ::core::mem::transmute_copy(str)).ok()
    }
    pub unsafe fn Encode(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Encode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICertExit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICertExit {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICertExit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertExit").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICertExit2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICertExit2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICertExit2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertExit2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ICertExit2 {
    pub unsafe fn Initialize(&self, strconfig: &::windows::core::BSTR) -> ::windows::core::Result<CERT_EXIT_EVENT_MASK> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Initialize)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strconfig), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Notify(&self, exitevent: i32, context: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Notify)(::windows::core::Vtable::as_raw(self), exitevent, context).ok()
    }
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDescription)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICertGetConfig {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICertGetConfig {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICertGetConfig {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertGetConfig").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICertManageModule {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICertManageModule {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICertManageModule {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertManageModule").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICertPolicy {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICertPolicy {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICertPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertPolicy").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICertPolicy2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICertPolicy2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICertPolicy2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertPolicy2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ICertPolicy2 {
    pub unsafe fn Initialize(&self, strconfig: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Initialize)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strconfig)).ok()
    }
    pub unsafe fn VerifyRequest(&self, strconfig: &::windows::core::BSTR, context: i32, bnewrequest: i32, flags: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.VerifyRequest)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strconfig), context, bnewrequest, flags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDescription)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ShutDown(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ShutDown)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICertProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICertProperties {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICertProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertProperties").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICertProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICertProperty {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICertProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertProperty").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICertPropertyArchived {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICertPropertyArchived {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICertPropertyArchived {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertPropertyArchived").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ICertPropertyArchived {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InitializeFromCertificate<P0>(&self, machinecontext: P0, encoding: EncodingType, strcertificate: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.InitializeFromCertificate)(::windows::core::Vtable::as_raw(self), machinecontext.into(), encoding, ::core::mem::transmute_copy(strcertificate)).ok()
    }
    pub unsafe fn InitializeDecode(&self, encoding: EncodingType, strencodeddata: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.InitializeDecode)(::windows::core::Vtable::as_raw(self), encoding, ::core::mem::transmute_copy(strencodeddata)).ok()
    }
    pub unsafe fn PropertyId(&self) -> ::windows::core::Result<CERTENROLL_PROPERTYID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.PropertyId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPropertyId(&self, value: CERTENROLL_PROPERTYID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPropertyId)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn get_RawData(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_RawData)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoveFromCertificate<P0>(&self, machinecontext: P0, encoding: EncodingType, strcertificate: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.RemoveFromCertificate)(::windows::core::Vtable::as_raw(self), machinecontext.into(), encoding, ::core::mem::transmute_copy(strcertificate)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetValueOnCertificate<P0>(&self, machinecontext: P0, encoding: EncodingType, strcertificate: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetValueOnCertificate)(::windows::core::Vtable::as_raw(self), machinecontext.into(), encoding, ::core::mem::transmute_copy(strcertificate)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICertPropertyArchivedKeyHash {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICertPropertyArchivedKeyHash {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICertPropertyArchivedKeyHash {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertPropertyArchivedKeyHash").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ICertPropertyArchivedKeyHash {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InitializeFromCertificate<P0>(&self, machinecontext: P0, encoding: EncodingType, strcertificate: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.InitializeFromCertificate)(::windows::core::Vtable::as_raw(self), machinecontext.into(), encoding, ::core::mem::transmute_copy(strcertificate)).ok()
    }
    pub unsafe fn InitializeDecode(&self, encoding: EncodingType, strencodeddata: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.InitializeDecode)(::windows::core::Vtable::as_raw(self), encoding, ::core::mem::transmute_copy(strencodeddata)).ok()
    }
    pub unsafe fn PropertyId(&self) -> ::windows::core::Result<CERTENROLL_PROPERTYID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.PropertyId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPropertyId(&self, value: CERTENROLL_PROPERTYID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPropertyId)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn get_RawData(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_RawData)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoveFromCertificate<P0>(&self, machinecontext: P0, encoding: EncodingType, strcertificate: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.RemoveFromCertificate)(::windows::core::Vtable::as_raw(self), machinecontext.into(), encoding, ::core::mem::transmute_copy(strcertificate)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetValueOnCertificate<P0>(&self, machinecontext: P0, encoding: EncodingType, strcertificate: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetValueOnCertificate)(::windows::core::Vtable::as_raw(self), machinecontext.into(), encoding, ::core::mem::transmute_copy(strcertificate)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICertPropertyAutoEnroll {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICertPropertyAutoEnroll {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICertPropertyAutoEnroll {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertPropertyAutoEnroll").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ICertPropertyAutoEnroll {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InitializeFromCertificate<P0>(&self, machinecontext: P0, encoding: EncodingType, strcertificate: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.InitializeFromCertificate)(::windows::core::Vtable::as_raw(self), machinecontext.into(), encoding, ::core::mem::transmute_copy(strcertificate)).ok()
    }
    pub unsafe fn InitializeDecode(&self, encoding: EncodingType, strencodeddata: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.InitializeDecode)(::windows::core::Vtable::as_raw(self), encoding, ::core::mem::transmute_copy(strencodeddata)).ok()
    }
    pub unsafe fn PropertyId(&self) -> ::windows::core::Result<CERTENROLL_PROPERTYID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.PropertyId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPropertyId(&self, value: CERTENROLL_PROPERTYID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPropertyId)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn get_RawData(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_RawData)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoveFromCertificate<P0>(&self, machinecontext: P0, encoding: EncodingType, strcertificate: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.RemoveFromCertificate)(::windows::core::Vtable::as_raw(self), machinecontext.into(), encoding, ::core::mem::transmute_copy(strcertificate)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetValueOnCertificate<P0>(&self, machinecontext: P0, encoding: EncodingType, strcertificate: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetValueOnCertificate)(::windows::core::Vtable::as_raw(self), machinecontext.into(), encoding, ::core::mem::transmute_copy(strcertificate)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICertPropertyBackedUp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICertPropertyBackedUp {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICertPropertyBackedUp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertPropertyBackedUp").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ICertPropertyBackedUp {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InitializeFromCertificate<P0>(&self, machinecontext: P0, encoding: EncodingType, strcertificate: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.InitializeFromCertificate)(::windows::core::Vtable::as_raw(self), machinecontext.into(), encoding, ::core::mem::transmute_copy(strcertificate)).ok()
    }
    pub unsafe fn InitializeDecode(&self, encoding: EncodingType, strencodeddata: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.InitializeDecode)(::windows::core::Vtable::as_raw(self), encoding, ::core::mem::transmute_copy(strencodeddata)).ok()
    }
    pub unsafe fn PropertyId(&self) -> ::windows::core::Result<CERTENROLL_PROPERTYID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.PropertyId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPropertyId(&self, value: CERTENROLL_PROPERTYID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPropertyId)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn get_RawData(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_RawData)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoveFromCertificate<P0>(&self, machinecontext: P0, encoding: EncodingType, strcertificate: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.RemoveFromCertificate)(::windows::core::Vtable::as_raw(self), machinecontext.into(), encoding, ::core::mem::transmute_copy(strcertificate)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetValueOnCertificate<P0>(&self, machinecontext: P0, encoding: EncodingType, strcertificate: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetValueOnCertificate)(::windows::core::Vtable::as_raw(self), machinecontext.into(), encoding, ::core::mem::transmute_copy(strcertificate)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICertPropertyDescription {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICertPropertyDescription {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICertPropertyDescription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertPropertyDescription").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ICertPropertyDescription {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InitializeFromCertificate<P0>(&self, machinecontext: P0, encoding: EncodingType, strcertificate: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.InitializeFromCertificate)(::windows::core::Vtable::as_raw(self), machinecontext.into(), encoding, ::core::mem::transmute_copy(strcertificate)).ok()
    }
    pub unsafe fn InitializeDecode(&self, encoding: EncodingType, strencodeddata: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.InitializeDecode)(::windows::core::Vtable::as_raw(self), encoding, ::core::mem::transmute_copy(strencodeddata)).ok()
    }
    pub unsafe fn PropertyId(&self) -> ::windows::core::Result<CERTENROLL_PROPERTYID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.PropertyId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPropertyId(&self, value: CERTENROLL_PROPERTYID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPropertyId)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn get_RawData(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_RawData)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoveFromCertificate<P0>(&self, machinecontext: P0, encoding: EncodingType, strcertificate: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.RemoveFromCertificate)(::windows::core::Vtable::as_raw(self), machinecontext.into(), encoding, ::core::mem::transmute_copy(strcertificate)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetValueOnCertificate<P0>(&self, machinecontext: P0, encoding: EncodingType, strcertificate: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetValueOnCertificate)(::windows::core::Vtable::as_raw(self), machinecontext.into(), encoding, ::core::mem::transmute_copy(strcertificate)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICertPropertyEnrollment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICertPropertyEnrollment {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICertPropertyEnrollment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertPropertyEnrollment").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ICertPropertyEnrollment {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InitializeFromCertificate<P0>(&self, machinecontext: P0, encoding: EncodingType, strcertificate: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.InitializeFromCertificate)(::windows::core::Vtable::as_raw(self), machinecontext.into(), encoding, ::core::mem::transmute_copy(strcertificate)).ok()
    }
    pub unsafe fn InitializeDecode(&self, encoding: EncodingType, strencodeddata: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.InitializeDecode)(::windows::core::Vtable::as_raw(self), encoding, ::core::mem::transmute_copy(strencodeddata)).ok()
    }
    pub unsafe fn PropertyId(&self) -> ::windows::core::Result<CERTENROLL_PROPERTYID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.PropertyId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPropertyId(&self, value: CERTENROLL_PROPERTYID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPropertyId)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn get_RawData(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_RawData)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoveFromCertificate<P0>(&self, machinecontext: P0, encoding: EncodingType, strcertificate: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.RemoveFromCertificate)(::windows::core::Vtable::as_raw(self), machinecontext.into(), encoding, ::core::mem::transmute_copy(strcertificate)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetValueOnCertificate<P0>(&self, machinecontext: P0, encoding: EncodingType, strcertificate: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetValueOnCertificate)(::windows::core::Vtable::as_raw(self), machinecontext.into(), encoding, ::core::mem::transmute_copy(strcertificate)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICertPropertyEnrollmentPolicyServer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICertPropertyEnrollmentPolicyServer {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICertPropertyEnrollmentPolicyServer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertPropertyEnrollmentPolicyServer").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ICertPropertyEnrollmentPolicyServer {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InitializeFromCertificate<P0>(&self, machinecontext: P0, encoding: EncodingType, strcertificate: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.InitializeFromCertificate)(::windows::core::Vtable::as_raw(self), machinecontext.into(), encoding, ::core::mem::transmute_copy(strcertificate)).ok()
    }
    pub unsafe fn InitializeDecode(&self, encoding: EncodingType, strencodeddata: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.InitializeDecode)(::windows::core::Vtable::as_raw(self), encoding, ::core::mem::transmute_copy(strencodeddata)).ok()
    }
    pub unsafe fn PropertyId(&self) -> ::windows::core::Result<CERTENROLL_PROPERTYID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.PropertyId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPropertyId(&self, value: CERTENROLL_PROPERTYID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPropertyId)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn get_RawData(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_RawData)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoveFromCertificate<P0>(&self, machinecontext: P0, encoding: EncodingType, strcertificate: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.RemoveFromCertificate)(::windows::core::Vtable::as_raw(self), machinecontext.into(), encoding, ::core::mem::transmute_copy(strcertificate)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetValueOnCertificate<P0>(&self, machinecontext: P0, encoding: EncodingType, strcertificate: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetValueOnCertificate)(::windows::core::Vtable::as_raw(self), machinecontext.into(), encoding, ::core::mem::transmute_copy(strcertificate)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICertPropertyFriendlyName {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICertPropertyFriendlyName {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICertPropertyFriendlyName {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertPropertyFriendlyName").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ICertPropertyFriendlyName {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InitializeFromCertificate<P0>(&self, machinecontext: P0, encoding: EncodingType, strcertificate: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.InitializeFromCertificate)(::windows::core::Vtable::as_raw(self), machinecontext.into(), encoding, ::core::mem::transmute_copy(strcertificate)).ok()
    }
    pub unsafe fn InitializeDecode(&self, encoding: EncodingType, strencodeddata: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.InitializeDecode)(::windows::core::Vtable::as_raw(self), encoding, ::core::mem::transmute_copy(strencodeddata)).ok()
    }
    pub unsafe fn PropertyId(&self) -> ::windows::core::Result<CERTENROLL_PROPERTYID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.PropertyId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPropertyId(&self, value: CERTENROLL_PROPERTYID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPropertyId)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn get_RawData(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_RawData)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoveFromCertificate<P0>(&self, machinecontext: P0, encoding: EncodingType, strcertificate: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.RemoveFromCertificate)(::windows::core::Vtable::as_raw(self), machinecontext.into(), encoding, ::core::mem::transmute_copy(strcertificate)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetValueOnCertificate<P0>(&self, machinecontext: P0, encoding: EncodingType, strcertificate: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetValueOnCertificate)(::windows::core::Vtable::as_raw(self), machinecontext.into(), encoding, ::core::mem::transmute_copy(strcertificate)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICertPropertyKeyProvInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICertPropertyKeyProvInfo {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICertPropertyKeyProvInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertPropertyKeyProvInfo").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ICertPropertyKeyProvInfo {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InitializeFromCertificate<P0>(&self, machinecontext: P0, encoding: EncodingType, strcertificate: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.InitializeFromCertificate)(::windows::core::Vtable::as_raw(self), machinecontext.into(), encoding, ::core::mem::transmute_copy(strcertificate)).ok()
    }
    pub unsafe fn InitializeDecode(&self, encoding: EncodingType, strencodeddata: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.InitializeDecode)(::windows::core::Vtable::as_raw(self), encoding, ::core::mem::transmute_copy(strencodeddata)).ok()
    }
    pub unsafe fn PropertyId(&self) -> ::windows::core::Result<CERTENROLL_PROPERTYID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.PropertyId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPropertyId(&self, value: CERTENROLL_PROPERTYID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPropertyId)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn get_RawData(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_RawData)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoveFromCertificate<P0>(&self, machinecontext: P0, encoding: EncodingType, strcertificate: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.RemoveFromCertificate)(::windows::core::Vtable::as_raw(self), machinecontext.into(), encoding, ::core::mem::transmute_copy(strcertificate)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetValueOnCertificate<P0>(&self, machinecontext: P0, encoding: EncodingType, strcertificate: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetValueOnCertificate)(::windows::core::Vtable::as_raw(self), machinecontext.into(), encoding, ::core::mem::transmute_copy(strcertificate)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICertPropertyRenewal {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICertPropertyRenewal {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICertPropertyRenewal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertPropertyRenewal").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ICertPropertyRenewal {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InitializeFromCertificate<P0>(&self, machinecontext: P0, encoding: EncodingType, strcertificate: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.InitializeFromCertificate)(::windows::core::Vtable::as_raw(self), machinecontext.into(), encoding, ::core::mem::transmute_copy(strcertificate)).ok()
    }
    pub unsafe fn InitializeDecode(&self, encoding: EncodingType, strencodeddata: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.InitializeDecode)(::windows::core::Vtable::as_raw(self), encoding, ::core::mem::transmute_copy(strencodeddata)).ok()
    }
    pub unsafe fn PropertyId(&self) -> ::windows::core::Result<CERTENROLL_PROPERTYID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.PropertyId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPropertyId(&self, value: CERTENROLL_PROPERTYID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPropertyId)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn get_RawData(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_RawData)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoveFromCertificate<P0>(&self, machinecontext: P0, encoding: EncodingType, strcertificate: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.RemoveFromCertificate)(::windows::core::Vtable::as_raw(self), machinecontext.into(), encoding, ::core::mem::transmute_copy(strcertificate)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetValueOnCertificate<P0>(&self, machinecontext: P0, encoding: EncodingType, strcertificate: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetValueOnCertificate)(::windows::core::Vtable::as_raw(self), machinecontext.into(), encoding, ::core::mem::transmute_copy(strcertificate)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICertPropertyRequestOriginator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICertPropertyRequestOriginator {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICertPropertyRequestOriginator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertPropertyRequestOriginator").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ICertPropertyRequestOriginator {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InitializeFromCertificate<P0>(&self, machinecontext: P0, encoding: EncodingType, strcertificate: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.InitializeFromCertificate)(::windows::core::Vtable::as_raw(self), machinecontext.into(), encoding, ::core::mem::transmute_copy(strcertificate)).ok()
    }
    pub unsafe fn InitializeDecode(&self, encoding: EncodingType, strencodeddata: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.InitializeDecode)(::windows::core::Vtable::as_raw(self), encoding, ::core::mem::transmute_copy(strencodeddata)).ok()
    }
    pub unsafe fn PropertyId(&self) -> ::windows::core::Result<CERTENROLL_PROPERTYID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.PropertyId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPropertyId(&self, value: CERTENROLL_PROPERTYID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPropertyId)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn get_RawData(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_RawData)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoveFromCertificate<P0>(&self, machinecontext: P0, encoding: EncodingType, strcertificate: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.RemoveFromCertificate)(::windows::core::Vtable::as_raw(self), machinecontext.into(), encoding, ::core::mem::transmute_copy(strcertificate)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetValueOnCertificate<P0>(&self, machinecontext: P0, encoding: EncodingType, strcertificate: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetValueOnCertificate)(::windows::core::Vtable::as_raw(self), machinecontext.into(), encoding, ::core::mem::transmute_copy(strcertificate)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICertPropertySHA1Hash {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICertPropertySHA1Hash {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICertPropertySHA1Hash {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertPropertySHA1Hash").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ICertPropertySHA1Hash {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InitializeFromCertificate<P0>(&self, machinecontext: P0, encoding: EncodingType, strcertificate: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.InitializeFromCertificate)(::windows::core::Vtable::as_raw(self), machinecontext.into(), encoding, ::core::mem::transmute_copy(strcertificate)).ok()
    }
    pub unsafe fn InitializeDecode(&self, encoding: EncodingType, strencodeddata: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.InitializeDecode)(::windows::core::Vtable::as_raw(self), encoding, ::core::mem::transmute_copy(strencodeddata)).ok()
    }
    pub unsafe fn PropertyId(&self) -> ::windows::core::Result<CERTENROLL_PROPERTYID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.PropertyId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPropertyId(&self, value: CERTENROLL_PROPERTYID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPropertyId)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn get_RawData(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_RawData)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoveFromCertificate<P0>(&self, machinecontext: P0, encoding: EncodingType, strcertificate: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.RemoveFromCertificate)(::windows::core::Vtable::as_raw(self), machinecontext.into(), encoding, ::core::mem::transmute_copy(strcertificate)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetValueOnCertificate<P0>(&self, machinecontext: P0, encoding: EncodingType, strcertificate: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetValueOnCertificate)(::windows::core::Vtable::as_raw(self), machinecontext.into(), encoding, ::core::mem::transmute_copy(strcertificate)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICertRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICertRequest {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICertRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertRequest").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICertRequest2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICertRequest2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICertRequest2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertRequest2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ICertRequest2 {
    pub unsafe fn Submit(&self, flags: i32, strrequest: &::windows::core::BSTR, strattributes: &::windows::core::BSTR, strconfig: &::windows::core::BSTR) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Submit)(::windows::core::Vtable::as_raw(self), flags, ::core::mem::transmute_copy(strrequest), ::core::mem::transmute_copy(strattributes), ::core::mem::transmute_copy(strconfig), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RetrievePending(&self, requestid: i32, strconfig: &::windows::core::BSTR) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.RetrievePending)(::windows::core::Vtable::as_raw(self), requestid, ::core::mem::transmute_copy(strconfig), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetLastStatus(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetLastStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetRequestId(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetRequestId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDispositionMessage(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDispositionMessage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCACertificate(&self, fexchangecertificate: i32, strconfig: &::windows::core::BSTR, flags: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCACertificate)(::windows::core::Vtable::as_raw(self), fexchangecertificate, ::core::mem::transmute_copy(strconfig), flags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCertificate(&self, flags: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCertificate)(::windows::core::Vtable::as_raw(self), flags, result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICertRequest3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICertRequest3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICertRequest3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertRequest3").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ICertRequest3 {
    pub unsafe fn Submit(&self, flags: i32, strrequest: &::windows::core::BSTR, strattributes: &::windows::core::BSTR, strconfig: &::windows::core::BSTR) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Submit)(::windows::core::Vtable::as_raw(self), flags, ::core::mem::transmute_copy(strrequest), ::core::mem::transmute_copy(strattributes), ::core::mem::transmute_copy(strconfig), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RetrievePending(&self, requestid: i32, strconfig: &::windows::core::BSTR) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.RetrievePending)(::windows::core::Vtable::as_raw(self), requestid, ::core::mem::transmute_copy(strconfig), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetLastStatus(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetLastStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetRequestId(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetRequestId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDispositionMessage(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDispositionMessage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCACertificate(&self, fexchangecertificate: i32, strconfig: &::windows::core::BSTR, flags: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetCACertificate)(::windows::core::Vtable::as_raw(self), fexchangecertificate, ::core::mem::transmute_copy(strconfig), flags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCertificate(&self, flags: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetCertificate)(::windows::core::Vtable::as_raw(self), flags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetIssuedCertificate(&self, strconfig: &::windows::core::BSTR, requestid: i32, strserialnumber: &::windows::core::BSTR) -> ::windows::core::Result<CR_DISP> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetIssuedCertificate)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strconfig), requestid, ::core::mem::transmute_copy(strserialnumber), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetErrorMessageText(&self, hrmessage: i32, flags: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetErrorMessageText)(::windows::core::Vtable::as_raw(self), hrmessage, flags, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetCAProperty(&self, strconfig: &::windows::core::BSTR, propid: i32, propindex: i32, proptype: i32, flags: i32) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCAProperty)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strconfig), propid, propindex, proptype, flags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCAPropertyFlags(&self, strconfig: &::windows::core::BSTR, propid: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCAPropertyFlags)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strconfig), propid, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCAPropertyDisplayName(&self, strconfig: &::windows::core::BSTR, propid: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCAPropertyDisplayName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strconfig), propid, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetFullResponseProperty(&self, propid: FULL_RESPONSE_PROPERTY_ID, propindex: i32, proptype: CERT_PROPERTY_TYPE, flags: CERT_REQUEST_OUT_TYPE) -> ::windows::core::Result<super::super::super::System::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFullResponseProperty)(::windows::core::Vtable::as_raw(self), propid, propindex, proptype, flags, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ICertRequestD {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICertRequestD {}
impl ::core::fmt::Debug for ICertRequestD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertRequestD").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ICertRequestD2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICertRequestD2 {}
impl ::core::fmt::Debug for ICertRequestD2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertRequestD2").field(&self.0).finish()
    }
}
impl ICertRequestD2 {
    pub unsafe fn Request<P0, P1>(&self, dwflags: u32, pwszauthority: P0, pdwrequestid: *mut u32, pdwdisposition: *mut u32, pwszattributes: P1, pctbrequest: *const CERTTRANSBLOB, pctbcertchain: *mut CERTTRANSBLOB, pctbencodedcert: *mut CERTTRANSBLOB, pctbdispositionmessage: *mut CERTTRANSBLOB) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Request)(::windows::core::Vtable::as_raw(self), dwflags, pwszauthority.into().abi(), pdwrequestid, pdwdisposition, pwszattributes.into().abi(), pctbrequest, pctbcertchain, pctbencodedcert, pctbdispositionmessage).ok()
    }
    pub unsafe fn GetCACert<P0>(&self, fchain: u32, pwszauthority: P0) -> ::windows::core::Result<CERTTRANSBLOB>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCACert)(::windows::core::Vtable::as_raw(self), fchain, pwszauthority.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Ping<P0>(&self, pwszauthority: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Ping)(::windows::core::Vtable::as_raw(self), pwszauthority.into().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICertServerExit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICertServerExit {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICertServerExit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertServerExit").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICertServerPolicy {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICertServerPolicy {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICertServerPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertServerPolicy").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICertView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICertView {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICertView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertView").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICertView2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICertView2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICertView2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertView2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ICertView2 {
    pub unsafe fn OpenConnection(&self, strconfig: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OpenConnection)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strconfig)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumCertViewColumn(&self, fresultcolumn: CVRC_COLUMN) -> ::windows::core::Result<IEnumCERTVIEWCOLUMN> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumCertViewColumn)(::windows::core::Vtable::as_raw(self), fresultcolumn, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetColumnCount(&self, fresultcolumn: CVRC_COLUMN, pccolumn: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetColumnCount)(::windows::core::Vtable::as_raw(self), fresultcolumn, pccolumn).ok()
    }
    pub unsafe fn GetColumnIndex(&self, fresultcolumn: CVRC_COLUMN, strcolumnname: &::windows::core::BSTR, pcolumnindex: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetColumnIndex)(::windows::core::Vtable::as_raw(self), fresultcolumn, ::core::mem::transmute_copy(strcolumnname), pcolumnindex).ok()
    }
    pub unsafe fn SetResultColumnCount(&self, cresultcolumn: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetResultColumnCount)(::windows::core::Vtable::as_raw(self), cresultcolumn).ok()
    }
    pub unsafe fn SetResultColumn(&self, columnindex: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetResultColumn)(::windows::core::Vtable::as_raw(self), columnindex).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetRestriction(&self, columnindex: CERT_VIEW_COLUMN_INDEX, seekoperator: CERT_VIEW_SEEK_OPERATOR_FLAGS, sortorder: i32, pvarvalue: *const super::super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetRestriction)(::windows::core::Vtable::as_raw(self), columnindex, seekoperator, sortorder, pvarvalue).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OpenView(&self) -> ::windows::core::Result<IEnumCERTVIEWROW> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.OpenView)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICertificateAttestationChallenge {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICertificateAttestationChallenge {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICertificateAttestationChallenge {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertificateAttestationChallenge").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICertificateAttestationChallenge2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICertificateAttestationChallenge2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICertificateAttestationChallenge2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertificateAttestationChallenge2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ICertificateAttestationChallenge2 {
    pub unsafe fn Initialize(&self, encoding: EncodingType, strpendingfullcmcresponsewithchallenge: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Initialize)(::windows::core::Vtable::as_raw(self), encoding, ::core::mem::transmute_copy(strpendingfullcmcresponsewithchallenge)).ok()
    }
    pub unsafe fn DecryptChallenge(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DecryptChallenge)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RequestID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.RequestID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICertificatePolicies {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICertificatePolicies {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICertificatePolicies {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertificatePolicies").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICertificatePolicy {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICertificatePolicy {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICertificatePolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertificatePolicy").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICertificationAuthorities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICertificationAuthorities {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICertificationAuthorities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertificationAuthorities").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICertificationAuthority {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICertificationAuthority {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICertificationAuthority {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICertificationAuthority").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICryptAttribute {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICryptAttribute {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICryptAttribute {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICryptAttribute").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICryptAttributes {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICryptAttributes {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICryptAttributes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICryptAttributes").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICspAlgorithm {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICspAlgorithm {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICspAlgorithm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICspAlgorithm").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICspAlgorithms {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICspAlgorithms {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICspAlgorithms {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICspAlgorithms").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICspInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICspInformation {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICspInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICspInformation").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICspInformations {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICspInformations {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICspInformations {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICspInformations").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICspStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICspStatus {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICspStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICspStatus").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ICspStatuses {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ICspStatuses {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ICspStatuses {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICspStatuses").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnroll {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnroll {}
impl ::core::fmt::Debug for IEnroll {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnroll").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnroll2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnroll2 {}
impl ::core::fmt::Debug for IEnroll2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnroll2").field(&self.0).finish()
    }
}
impl IEnroll2 {
    pub unsafe fn createFilePKCS10WStr<P0, P1, P2>(&self, dnname: P0, usage: P1, wszpkcs10filename: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.createFilePKCS10WStr)(::windows::core::Vtable::as_raw(self), dnname.into().abi(), usage.into().abi(), wszpkcs10filename.into().abi()).ok()
    }
    pub unsafe fn acceptFilePKCS7WStr<P0>(&self, wszpkcs7filename: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.acceptFilePKCS7WStr)(::windows::core::Vtable::as_raw(self), wszpkcs7filename.into().abi()).ok()
    }
    pub unsafe fn createPKCS10WStr<P0, P1>(&self, dnname: P0, usage: P1, ppkcs10blob: *mut super::CRYPT_INTEGER_BLOB) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.createPKCS10WStr)(::windows::core::Vtable::as_raw(self), dnname.into().abi(), usage.into().abi(), ppkcs10blob).ok()
    }
    pub unsafe fn acceptPKCS7Blob(&self, pblobpkcs7: *mut super::CRYPT_INTEGER_BLOB) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.acceptPKCS7Blob)(::windows::core::Vtable::as_raw(self), pblobpkcs7).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn getCertContextFromPKCS7(&self, pblobpkcs7: *mut super::CRYPT_INTEGER_BLOB) -> *mut super::CERT_CONTEXT {
        (::windows::core::Vtable::vtable(self).base__.getCertContextFromPKCS7)(::windows::core::Vtable::as_raw(self), pblobpkcs7)
    }
    pub unsafe fn getMyStore(&self) -> super::HCERTSTORE {
        (::windows::core::Vtable::vtable(self).base__.getMyStore)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn getCAStore(&self) -> super::HCERTSTORE {
        (::windows::core::Vtable::vtable(self).base__.getCAStore)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn getROOTHStore(&self) -> super::HCERTSTORE {
        (::windows::core::Vtable::vtable(self).base__.getROOTHStore)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn enumProvidersWStr(&self, dwindex: i32, dwflags: i32, pbstrprovname: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.enumProvidersWStr)(::windows::core::Vtable::as_raw(self), dwindex, dwflags, pbstrprovname).ok()
    }
    pub unsafe fn enumContainersWStr(&self, dwindex: i32, pbstr: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.enumContainersWStr)(::windows::core::Vtable::as_raw(self), dwindex, pbstr).ok()
    }
    pub unsafe fn freeRequestInfoBlob(&self, pkcs7orpkcs10: super::CRYPT_INTEGER_BLOB) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.freeRequestInfoBlob)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pkcs7orpkcs10)).ok()
    }
    pub unsafe fn MyStoreNameWStr(&self, szwname: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.MyStoreNameWStr)(::windows::core::Vtable::as_raw(self), szwname).ok()
    }
    pub unsafe fn SetMyStoreNameWStr<P0>(&self, szwname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetMyStoreNameWStr)(::windows::core::Vtable::as_raw(self), szwname.into().abi()).ok()
    }
    pub unsafe fn MyStoreTypeWStr(&self, szwtype: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.MyStoreTypeWStr)(::windows::core::Vtable::as_raw(self), szwtype).ok()
    }
    pub unsafe fn SetMyStoreTypeWStr<P0>(&self, szwtype: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetMyStoreTypeWStr)(::windows::core::Vtable::as_raw(self), szwtype.into().abi()).ok()
    }
    pub unsafe fn MyStoreFlags(&self, pdwflags: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.MyStoreFlags)(::windows::core::Vtable::as_raw(self), pdwflags).ok()
    }
    pub unsafe fn SetMyStoreFlags(&self, dwflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetMyStoreFlags)(::windows::core::Vtable::as_raw(self), dwflags).ok()
    }
    pub unsafe fn CAStoreNameWStr(&self, szwname: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CAStoreNameWStr)(::windows::core::Vtable::as_raw(self), szwname).ok()
    }
    pub unsafe fn SetCAStoreNameWStr<P0>(&self, szwname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetCAStoreNameWStr)(::windows::core::Vtable::as_raw(self), szwname.into().abi()).ok()
    }
    pub unsafe fn CAStoreTypeWStr(&self, szwtype: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CAStoreTypeWStr)(::windows::core::Vtable::as_raw(self), szwtype).ok()
    }
    pub unsafe fn SetCAStoreTypeWStr<P0>(&self, szwtype: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetCAStoreTypeWStr)(::windows::core::Vtable::as_raw(self), szwtype.into().abi()).ok()
    }
    pub unsafe fn CAStoreFlags(&self, pdwflags: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CAStoreFlags)(::windows::core::Vtable::as_raw(self), pdwflags).ok()
    }
    pub unsafe fn SetCAStoreFlags(&self, dwflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetCAStoreFlags)(::windows::core::Vtable::as_raw(self), dwflags).ok()
    }
    pub unsafe fn RootStoreNameWStr(&self, szwname: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.RootStoreNameWStr)(::windows::core::Vtable::as_raw(self), szwname).ok()
    }
    pub unsafe fn SetRootStoreNameWStr<P0>(&self, szwname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetRootStoreNameWStr)(::windows::core::Vtable::as_raw(self), szwname.into().abi()).ok()
    }
    pub unsafe fn RootStoreTypeWStr(&self, szwtype: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.RootStoreTypeWStr)(::windows::core::Vtable::as_raw(self), szwtype).ok()
    }
    pub unsafe fn SetRootStoreTypeWStr<P0>(&self, szwtype: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetRootStoreTypeWStr)(::windows::core::Vtable::as_raw(self), szwtype.into().abi()).ok()
    }
    pub unsafe fn RootStoreFlags(&self, pdwflags: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.RootStoreFlags)(::windows::core::Vtable::as_raw(self), pdwflags).ok()
    }
    pub unsafe fn SetRootStoreFlags(&self, dwflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetRootStoreFlags)(::windows::core::Vtable::as_raw(self), dwflags).ok()
    }
    pub unsafe fn RequestStoreNameWStr(&self, szwname: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.RequestStoreNameWStr)(::windows::core::Vtable::as_raw(self), szwname).ok()
    }
    pub unsafe fn SetRequestStoreNameWStr<P0>(&self, szwname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetRequestStoreNameWStr)(::windows::core::Vtable::as_raw(self), szwname.into().abi()).ok()
    }
    pub unsafe fn RequestStoreTypeWStr(&self, szwtype: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.RequestStoreTypeWStr)(::windows::core::Vtable::as_raw(self), szwtype).ok()
    }
    pub unsafe fn SetRequestStoreTypeWStr<P0>(&self, szwtype: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetRequestStoreTypeWStr)(::windows::core::Vtable::as_raw(self), szwtype.into().abi()).ok()
    }
    pub unsafe fn RequestStoreFlags(&self, pdwflags: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.RequestStoreFlags)(::windows::core::Vtable::as_raw(self), pdwflags).ok()
    }
    pub unsafe fn SetRequestStoreFlags(&self, dwflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetRequestStoreFlags)(::windows::core::Vtable::as_raw(self), dwflags).ok()
    }
    pub unsafe fn ContainerNameWStr(&self, szwcontainer: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ContainerNameWStr)(::windows::core::Vtable::as_raw(self), szwcontainer).ok()
    }
    pub unsafe fn SetContainerNameWStr<P0>(&self, szwcontainer: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetContainerNameWStr)(::windows::core::Vtable::as_raw(self), szwcontainer.into().abi()).ok()
    }
    pub unsafe fn ProviderNameWStr(&self, szwprovider: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ProviderNameWStr)(::windows::core::Vtable::as_raw(self), szwprovider).ok()
    }
    pub unsafe fn SetProviderNameWStr<P0>(&self, szwprovider: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetProviderNameWStr)(::windows::core::Vtable::as_raw(self), szwprovider.into().abi()).ok()
    }
    pub unsafe fn ProviderType(&self, pdwtype: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ProviderType)(::windows::core::Vtable::as_raw(self), pdwtype).ok()
    }
    pub unsafe fn SetProviderType(&self, dwtype: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetProviderType)(::windows::core::Vtable::as_raw(self), dwtype).ok()
    }
    pub unsafe fn KeySpec(&self, pdw: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.KeySpec)(::windows::core::Vtable::as_raw(self), pdw).ok()
    }
    pub unsafe fn SetKeySpec(&self, dw: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetKeySpec)(::windows::core::Vtable::as_raw(self), dw).ok()
    }
    pub unsafe fn ProviderFlags(&self, pdwflags: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ProviderFlags)(::windows::core::Vtable::as_raw(self), pdwflags).ok()
    }
    pub unsafe fn SetProviderFlags(&self, dwflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetProviderFlags)(::windows::core::Vtable::as_raw(self), dwflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UseExistingKeySet(&self, fuseexistingkeys: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.UseExistingKeySet)(::windows::core::Vtable::as_raw(self), fuseexistingkeys).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUseExistingKeySet<P0>(&self, fuseexistingkeys: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetUseExistingKeySet)(::windows::core::Vtable::as_raw(self), fuseexistingkeys.into()).ok()
    }
    pub unsafe fn GenKeyFlags(&self, pdwflags: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GenKeyFlags)(::windows::core::Vtable::as_raw(self), pdwflags).ok()
    }
    pub unsafe fn SetGenKeyFlags(&self, dwflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetGenKeyFlags)(::windows::core::Vtable::as_raw(self), dwflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteRequestCert(&self, fdelete: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DeleteRequestCert)(::windows::core::Vtable::as_raw(self), fdelete).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDeleteRequestCert<P0>(&self, fdelete: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetDeleteRequestCert)(::windows::core::Vtable::as_raw(self), fdelete.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteCertToUserDS(&self, fbool: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.WriteCertToUserDS)(::windows::core::Vtable::as_raw(self), fbool).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetWriteCertToUserDS<P0>(&self, fbool: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetWriteCertToUserDS)(::windows::core::Vtable::as_raw(self), fbool.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnableT61DNEncoding(&self, fbool: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.EnableT61DNEncoding)(::windows::core::Vtable::as_raw(self), fbool).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableT61DNEncoding<P0>(&self, fbool: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetEnableT61DNEncoding)(::windows::core::Vtable::as_raw(self), fbool.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteCertToCSP(&self, fbool: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.WriteCertToCSP)(::windows::core::Vtable::as_raw(self), fbool).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetWriteCertToCSP<P0>(&self, fbool: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetWriteCertToCSP)(::windows::core::Vtable::as_raw(self), fbool.into()).ok()
    }
    pub unsafe fn SPCFileNameWStr(&self, szw: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SPCFileNameWStr)(::windows::core::Vtable::as_raw(self), szw).ok()
    }
    pub unsafe fn SetSPCFileNameWStr<P0>(&self, szw: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetSPCFileNameWStr)(::windows::core::Vtable::as_raw(self), szw.into().abi()).ok()
    }
    pub unsafe fn PVKFileNameWStr(&self, szw: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.PVKFileNameWStr)(::windows::core::Vtable::as_raw(self), szw).ok()
    }
    pub unsafe fn SetPVKFileNameWStr<P0>(&self, szw: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPVKFileNameWStr)(::windows::core::Vtable::as_raw(self), szw.into().abi()).ok()
    }
    pub unsafe fn HashAlgorithmWStr(&self, szw: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.HashAlgorithmWStr)(::windows::core::Vtable::as_raw(self), szw).ok()
    }
    pub unsafe fn SetHashAlgorithmWStr<P0>(&self, szw: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetHashAlgorithmWStr)(::windows::core::Vtable::as_raw(self), szw.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RenewalCertificate(&self, ppcertcontext: *mut *mut super::CERT_CONTEXT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.RenewalCertificate)(::windows::core::Vtable::as_raw(self), ppcertcontext).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRenewalCertificate(&self, pcertcontext: *const super::CERT_CONTEXT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetRenewalCertificate)(::windows::core::Vtable::as_raw(self), pcertcontext).ok()
    }
    pub unsafe fn AddCertTypeToRequestWStr<P0>(&self, szw: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AddCertTypeToRequestWStr)(::windows::core::Vtable::as_raw(self), szw.into().abi()).ok()
    }
    pub unsafe fn AddNameValuePairToSignatureWStr<P0, P1>(&self, name: P0, value: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AddNameValuePairToSignatureWStr)(::windows::core::Vtable::as_raw(self), name.into().abi(), value.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddExtensionsToRequest(&self, pcertextensions: *mut super::CERT_EXTENSIONS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.AddExtensionsToRequest)(::windows::core::Vtable::as_raw(self), pcertextensions).ok()
    }
    pub unsafe fn AddAuthenticatedAttributesToPKCS7Request(&self, pattributes: *mut super::CRYPT_ATTRIBUTES) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.AddAuthenticatedAttributesToPKCS7Request)(::windows::core::Vtable::as_raw(self), pattributes).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreatePKCS7RequestFromRequest(&self, prequest: *mut super::CRYPT_INTEGER_BLOB, psigningcertcontext: *const super::CERT_CONTEXT, ppkcs7blob: *mut super::CRYPT_INTEGER_BLOB) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreatePKCS7RequestFromRequest)(::windows::core::Vtable::as_raw(self), prequest, psigningcertcontext, ppkcs7blob).ok()
    }
}
impl ::core::cmp::PartialEq for IEnroll4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnroll4 {}
impl ::core::fmt::Debug for IEnroll4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnroll4").field(&self.0).finish()
    }
}
impl IEnroll4 {
    pub unsafe fn createFilePKCS10WStr<P0, P1, P2>(&self, dnname: P0, usage: P1, wszpkcs10filename: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.createFilePKCS10WStr)(::windows::core::Vtable::as_raw(self), dnname.into().abi(), usage.into().abi(), wszpkcs10filename.into().abi()).ok()
    }
    pub unsafe fn acceptFilePKCS7WStr<P0>(&self, wszpkcs7filename: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.acceptFilePKCS7WStr)(::windows::core::Vtable::as_raw(self), wszpkcs7filename.into().abi()).ok()
    }
    pub unsafe fn createPKCS10WStr<P0, P1>(&self, dnname: P0, usage: P1, ppkcs10blob: *mut super::CRYPT_INTEGER_BLOB) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.createPKCS10WStr)(::windows::core::Vtable::as_raw(self), dnname.into().abi(), usage.into().abi(), ppkcs10blob).ok()
    }
    pub unsafe fn acceptPKCS7Blob(&self, pblobpkcs7: *mut super::CRYPT_INTEGER_BLOB) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.acceptPKCS7Blob)(::windows::core::Vtable::as_raw(self), pblobpkcs7).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn getCertContextFromPKCS7(&self, pblobpkcs7: *mut super::CRYPT_INTEGER_BLOB) -> *mut super::CERT_CONTEXT {
        (::windows::core::Vtable::vtable(self).base__.base__.getCertContextFromPKCS7)(::windows::core::Vtable::as_raw(self), pblobpkcs7)
    }
    pub unsafe fn getMyStore(&self) -> super::HCERTSTORE {
        (::windows::core::Vtable::vtable(self).base__.base__.getMyStore)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn getCAStore(&self) -> super::HCERTSTORE {
        (::windows::core::Vtable::vtable(self).base__.base__.getCAStore)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn getROOTHStore(&self) -> super::HCERTSTORE {
        (::windows::core::Vtable::vtable(self).base__.base__.getROOTHStore)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn enumProvidersWStr(&self, dwindex: i32, dwflags: i32, pbstrprovname: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.enumProvidersWStr)(::windows::core::Vtable::as_raw(self), dwindex, dwflags, pbstrprovname).ok()
    }
    pub unsafe fn enumContainersWStr(&self, dwindex: i32, pbstr: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.enumContainersWStr)(::windows::core::Vtable::as_raw(self), dwindex, pbstr).ok()
    }
    pub unsafe fn freeRequestInfoBlob(&self, pkcs7orpkcs10: super::CRYPT_INTEGER_BLOB) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.freeRequestInfoBlob)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pkcs7orpkcs10)).ok()
    }
    pub unsafe fn MyStoreNameWStr(&self, szwname: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.MyStoreNameWStr)(::windows::core::Vtable::as_raw(self), szwname).ok()
    }
    pub unsafe fn SetMyStoreNameWStr<P0>(&self, szwname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetMyStoreNameWStr)(::windows::core::Vtable::as_raw(self), szwname.into().abi()).ok()
    }
    pub unsafe fn MyStoreTypeWStr(&self, szwtype: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.MyStoreTypeWStr)(::windows::core::Vtable::as_raw(self), szwtype).ok()
    }
    pub unsafe fn SetMyStoreTypeWStr<P0>(&self, szwtype: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetMyStoreTypeWStr)(::windows::core::Vtable::as_raw(self), szwtype.into().abi()).ok()
    }
    pub unsafe fn MyStoreFlags(&self, pdwflags: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.MyStoreFlags)(::windows::core::Vtable::as_raw(self), pdwflags).ok()
    }
    pub unsafe fn SetMyStoreFlags(&self, dwflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetMyStoreFlags)(::windows::core::Vtable::as_raw(self), dwflags).ok()
    }
    pub unsafe fn CAStoreNameWStr(&self, szwname: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CAStoreNameWStr)(::windows::core::Vtable::as_raw(self), szwname).ok()
    }
    pub unsafe fn SetCAStoreNameWStr<P0>(&self, szwname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetCAStoreNameWStr)(::windows::core::Vtable::as_raw(self), szwname.into().abi()).ok()
    }
    pub unsafe fn CAStoreTypeWStr(&self, szwtype: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CAStoreTypeWStr)(::windows::core::Vtable::as_raw(self), szwtype).ok()
    }
    pub unsafe fn SetCAStoreTypeWStr<P0>(&self, szwtype: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetCAStoreTypeWStr)(::windows::core::Vtable::as_raw(self), szwtype.into().abi()).ok()
    }
    pub unsafe fn CAStoreFlags(&self, pdwflags: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CAStoreFlags)(::windows::core::Vtable::as_raw(self), pdwflags).ok()
    }
    pub unsafe fn SetCAStoreFlags(&self, dwflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetCAStoreFlags)(::windows::core::Vtable::as_raw(self), dwflags).ok()
    }
    pub unsafe fn RootStoreNameWStr(&self, szwname: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.RootStoreNameWStr)(::windows::core::Vtable::as_raw(self), szwname).ok()
    }
    pub unsafe fn SetRootStoreNameWStr<P0>(&self, szwname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetRootStoreNameWStr)(::windows::core::Vtable::as_raw(self), szwname.into().abi()).ok()
    }
    pub unsafe fn RootStoreTypeWStr(&self, szwtype: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.RootStoreTypeWStr)(::windows::core::Vtable::as_raw(self), szwtype).ok()
    }
    pub unsafe fn SetRootStoreTypeWStr<P0>(&self, szwtype: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetRootStoreTypeWStr)(::windows::core::Vtable::as_raw(self), szwtype.into().abi()).ok()
    }
    pub unsafe fn RootStoreFlags(&self, pdwflags: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.RootStoreFlags)(::windows::core::Vtable::as_raw(self), pdwflags).ok()
    }
    pub unsafe fn SetRootStoreFlags(&self, dwflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetRootStoreFlags)(::windows::core::Vtable::as_raw(self), dwflags).ok()
    }
    pub unsafe fn RequestStoreNameWStr(&self, szwname: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.RequestStoreNameWStr)(::windows::core::Vtable::as_raw(self), szwname).ok()
    }
    pub unsafe fn SetRequestStoreNameWStr<P0>(&self, szwname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetRequestStoreNameWStr)(::windows::core::Vtable::as_raw(self), szwname.into().abi()).ok()
    }
    pub unsafe fn RequestStoreTypeWStr(&self, szwtype: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.RequestStoreTypeWStr)(::windows::core::Vtable::as_raw(self), szwtype).ok()
    }
    pub unsafe fn SetRequestStoreTypeWStr<P0>(&self, szwtype: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetRequestStoreTypeWStr)(::windows::core::Vtable::as_raw(self), szwtype.into().abi()).ok()
    }
    pub unsafe fn RequestStoreFlags(&self, pdwflags: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.RequestStoreFlags)(::windows::core::Vtable::as_raw(self), pdwflags).ok()
    }
    pub unsafe fn SetRequestStoreFlags(&self, dwflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetRequestStoreFlags)(::windows::core::Vtable::as_raw(self), dwflags).ok()
    }
    pub unsafe fn ContainerNameWStr(&self, szwcontainer: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.ContainerNameWStr)(::windows::core::Vtable::as_raw(self), szwcontainer).ok()
    }
    pub unsafe fn SetContainerNameWStr<P0>(&self, szwcontainer: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetContainerNameWStr)(::windows::core::Vtable::as_raw(self), szwcontainer.into().abi()).ok()
    }
    pub unsafe fn ProviderNameWStr(&self, szwprovider: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.ProviderNameWStr)(::windows::core::Vtable::as_raw(self), szwprovider).ok()
    }
    pub unsafe fn SetProviderNameWStr<P0>(&self, szwprovider: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetProviderNameWStr)(::windows::core::Vtable::as_raw(self), szwprovider.into().abi()).ok()
    }
    pub unsafe fn ProviderType(&self, pdwtype: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.ProviderType)(::windows::core::Vtable::as_raw(self), pdwtype).ok()
    }
    pub unsafe fn SetProviderType(&self, dwtype: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetProviderType)(::windows::core::Vtable::as_raw(self), dwtype).ok()
    }
    pub unsafe fn KeySpec(&self, pdw: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.KeySpec)(::windows::core::Vtable::as_raw(self), pdw).ok()
    }
    pub unsafe fn SetKeySpec(&self, dw: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetKeySpec)(::windows::core::Vtable::as_raw(self), dw).ok()
    }
    pub unsafe fn ProviderFlags(&self, pdwflags: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.ProviderFlags)(::windows::core::Vtable::as_raw(self), pdwflags).ok()
    }
    pub unsafe fn SetProviderFlags(&self, dwflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetProviderFlags)(::windows::core::Vtable::as_raw(self), dwflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UseExistingKeySet(&self, fuseexistingkeys: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.UseExistingKeySet)(::windows::core::Vtable::as_raw(self), fuseexistingkeys).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUseExistingKeySet<P0>(&self, fuseexistingkeys: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetUseExistingKeySet)(::windows::core::Vtable::as_raw(self), fuseexistingkeys.into()).ok()
    }
    pub unsafe fn GenKeyFlags(&self, pdwflags: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GenKeyFlags)(::windows::core::Vtable::as_raw(self), pdwflags).ok()
    }
    pub unsafe fn SetGenKeyFlags(&self, dwflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetGenKeyFlags)(::windows::core::Vtable::as_raw(self), dwflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeleteRequestCert(&self, fdelete: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.DeleteRequestCert)(::windows::core::Vtable::as_raw(self), fdelete).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDeleteRequestCert<P0>(&self, fdelete: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetDeleteRequestCert)(::windows::core::Vtable::as_raw(self), fdelete.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteCertToUserDS(&self, fbool: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.WriteCertToUserDS)(::windows::core::Vtable::as_raw(self), fbool).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetWriteCertToUserDS<P0>(&self, fbool: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetWriteCertToUserDS)(::windows::core::Vtable::as_raw(self), fbool.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnableT61DNEncoding(&self, fbool: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.EnableT61DNEncoding)(::windows::core::Vtable::as_raw(self), fbool).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableT61DNEncoding<P0>(&self, fbool: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetEnableT61DNEncoding)(::windows::core::Vtable::as_raw(self), fbool.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WriteCertToCSP(&self, fbool: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.WriteCertToCSP)(::windows::core::Vtable::as_raw(self), fbool).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetWriteCertToCSP<P0>(&self, fbool: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetWriteCertToCSP)(::windows::core::Vtable::as_raw(self), fbool.into()).ok()
    }
    pub unsafe fn SPCFileNameWStr(&self, szw: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SPCFileNameWStr)(::windows::core::Vtable::as_raw(self), szw).ok()
    }
    pub unsafe fn SetSPCFileNameWStr<P0>(&self, szw: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetSPCFileNameWStr)(::windows::core::Vtable::as_raw(self), szw.into().abi()).ok()
    }
    pub unsafe fn PVKFileNameWStr(&self, szw: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.PVKFileNameWStr)(::windows::core::Vtable::as_raw(self), szw).ok()
    }
    pub unsafe fn SetPVKFileNameWStr<P0>(&self, szw: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPVKFileNameWStr)(::windows::core::Vtable::as_raw(self), szw.into().abi()).ok()
    }
    pub unsafe fn HashAlgorithmWStr(&self, szw: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.HashAlgorithmWStr)(::windows::core::Vtable::as_raw(self), szw).ok()
    }
    pub unsafe fn SetHashAlgorithmWStr<P0>(&self, szw: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetHashAlgorithmWStr)(::windows::core::Vtable::as_raw(self), szw.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RenewalCertificate(&self, ppcertcontext: *mut *mut super::CERT_CONTEXT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.RenewalCertificate)(::windows::core::Vtable::as_raw(self), ppcertcontext).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRenewalCertificate(&self, pcertcontext: *const super::CERT_CONTEXT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetRenewalCertificate)(::windows::core::Vtable::as_raw(self), pcertcontext).ok()
    }
    pub unsafe fn AddCertTypeToRequestWStr<P0>(&self, szw: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.AddCertTypeToRequestWStr)(::windows::core::Vtable::as_raw(self), szw.into().abi()).ok()
    }
    pub unsafe fn AddNameValuePairToSignatureWStr<P0, P1>(&self, name: P0, value: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.AddNameValuePairToSignatureWStr)(::windows::core::Vtable::as_raw(self), name.into().abi(), value.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddExtensionsToRequest(&self, pcertextensions: *mut super::CERT_EXTENSIONS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.AddExtensionsToRequest)(::windows::core::Vtable::as_raw(self), pcertextensions).ok()
    }
    pub unsafe fn AddAuthenticatedAttributesToPKCS7Request(&self, pattributes: *mut super::CRYPT_ATTRIBUTES) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.AddAuthenticatedAttributesToPKCS7Request)(::windows::core::Vtable::as_raw(self), pattributes).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreatePKCS7RequestFromRequest(&self, prequest: *mut super::CRYPT_INTEGER_BLOB, psigningcertcontext: *const super::CERT_CONTEXT, ppkcs7blob: *mut super::CRYPT_INTEGER_BLOB) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CreatePKCS7RequestFromRequest)(::windows::core::Vtable::as_raw(self), prequest, psigningcertcontext, ppkcs7blob).ok()
    }
    pub unsafe fn InstallPKCS7Blob(&self, pblobpkcs7: *mut super::CRYPT_INTEGER_BLOB) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.InstallPKCS7Blob)(::windows::core::Vtable::as_raw(self), pblobpkcs7).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Reset)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetSupportedKeySpec(&self, pdwkeyspec: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetSupportedKeySpec)(::windows::core::Vtable::as_raw(self), pdwkeyspec).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetKeyLen<P0, P1>(&self, fmin: P0, fexchange: P1, pdwkeysize: *mut i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetKeyLen)(::windows::core::Vtable::as_raw(self), fmin.into(), fexchange.into(), pdwkeysize).ok()
    }
    pub unsafe fn EnumAlgs(&self, dwindex: i32, algclass: i32, pdwalgid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.EnumAlgs)(::windows::core::Vtable::as_raw(self), dwindex, algclass, pdwalgid).ok()
    }
    pub unsafe fn GetAlgNameWStr(&self, algid: i32, ppwsz: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetAlgNameWStr)(::windows::core::Vtable::as_raw(self), algid, ppwsz).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetReuseHardwareKeyIfUnableToGenNew<P0>(&self, freusehardwarekeyifunabletogennew: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetReuseHardwareKeyIfUnableToGenNew)(::windows::core::Vtable::as_raw(self), freusehardwarekeyifunabletogennew.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReuseHardwareKeyIfUnableToGenNew(&self, freusehardwarekeyifunabletogennew: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ReuseHardwareKeyIfUnableToGenNew)(::windows::core::Vtable::as_raw(self), freusehardwarekeyifunabletogennew).ok()
    }
    pub unsafe fn SetHashAlgID(&self, hashalgid: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetHashAlgID)(::windows::core::Vtable::as_raw(self), hashalgid).ok()
    }
    pub unsafe fn HashAlgID(&self, hashalgid: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.HashAlgID)(::windows::core::Vtable::as_raw(self), hashalgid).ok()
    }
    pub unsafe fn SetHStoreMy<P0>(&self, hstore: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::HCERTSTORE>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetHStoreMy)(::windows::core::Vtable::as_raw(self), hstore.into()).ok()
    }
    pub unsafe fn SetHStoreCA<P0>(&self, hstore: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::HCERTSTORE>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetHStoreCA)(::windows::core::Vtable::as_raw(self), hstore.into()).ok()
    }
    pub unsafe fn SetHStoreROOT<P0>(&self, hstore: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::HCERTSTORE>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetHStoreROOT)(::windows::core::Vtable::as_raw(self), hstore.into()).ok()
    }
    pub unsafe fn SetHStoreRequest<P0>(&self, hstore: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::HCERTSTORE>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetHStoreRequest)(::windows::core::Vtable::as_raw(self), hstore.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLimitExchangeKeyToEncipherment<P0>(&self, flimitexchangekeytoencipherment: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetLimitExchangeKeyToEncipherment)(::windows::core::Vtable::as_raw(self), flimitexchangekeytoencipherment.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LimitExchangeKeyToEncipherment(&self, flimitexchangekeytoencipherment: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.LimitExchangeKeyToEncipherment)(::windows::core::Vtable::as_raw(self), flimitexchangekeytoencipherment).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableSMIMECapabilities<P0>(&self, fenablesmimecapabilities: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetEnableSMIMECapabilities)(::windows::core::Vtable::as_raw(self), fenablesmimecapabilities.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnableSMIMECapabilities(&self, fenablesmimecapabilities: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.EnableSMIMECapabilities)(::windows::core::Vtable::as_raw(self), fenablesmimecapabilities).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IEnumCERTVIEWATTRIBUTE {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IEnumCERTVIEWATTRIBUTE {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IEnumCERTVIEWATTRIBUTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumCERTVIEWATTRIBUTE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IEnumCERTVIEWCOLUMN {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IEnumCERTVIEWCOLUMN {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IEnumCERTVIEWCOLUMN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumCERTVIEWCOLUMN").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IEnumCERTVIEWEXTENSION {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IEnumCERTVIEWEXTENSION {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IEnumCERTVIEWEXTENSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumCERTVIEWEXTENSION").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IEnumCERTVIEWROW {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IEnumCERTVIEWROW {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IEnumCERTVIEWROW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumCERTVIEWROW").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for INDESPolicy {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INDESPolicy {}
impl ::core::fmt::Debug for INDESPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INDESPolicy").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IOCSPAdmin {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IOCSPAdmin {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IOCSPAdmin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOCSPAdmin").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IOCSPCAConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IOCSPCAConfiguration {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IOCSPCAConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOCSPCAConfiguration").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IOCSPCAConfigurationCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IOCSPCAConfigurationCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IOCSPCAConfigurationCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOCSPCAConfigurationCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IOCSPProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IOCSPProperty {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IOCSPProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOCSPProperty").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IOCSPPropertyCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IOCSPPropertyCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IOCSPPropertyCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IOCSPPropertyCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IObjectId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IObjectId {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IObjectId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IObjectId").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IObjectIds {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IObjectIds {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IObjectIds {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IObjectIds").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPolicyQualifier {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPolicyQualifier {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPolicyQualifier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPolicyQualifier").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IPolicyQualifiers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IPolicyQualifiers {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IPolicyQualifiers {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPolicyQualifiers").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISignerCertificate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISignerCertificate {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISignerCertificate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISignerCertificate").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISignerCertificates {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISignerCertificates {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISignerCertificates {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISignerCertificates").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISmimeCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISmimeCapabilities {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISmimeCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISmimeCapabilities").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISmimeCapability {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISmimeCapability {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISmimeCapability {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISmimeCapability").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX500DistinguishedName {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX500DistinguishedName {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX500DistinguishedName {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX500DistinguishedName").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509Attribute {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509Attribute {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509Attribute {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509Attribute").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509AttributeArchiveKey {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509AttributeArchiveKey {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509AttributeArchiveKey {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509AttributeArchiveKey").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IX509AttributeArchiveKey {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize<P0>(&self, pobjectid: P0, encoding: EncodingType, strencodeddata: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IObjectId>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Initialize)(::windows::core::Vtable::as_raw(self), pobjectid.into().abi(), encoding, ::core::mem::transmute_copy(strencodeddata)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ObjectId(&self) -> ::windows::core::Result<IObjectId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ObjectId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn get_RawData(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_RawData)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509AttributeArchiveKeyHash {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509AttributeArchiveKeyHash {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509AttributeArchiveKeyHash {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509AttributeArchiveKeyHash").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IX509AttributeArchiveKeyHash {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize<P0>(&self, pobjectid: P0, encoding: EncodingType, strencodeddata: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IObjectId>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Initialize)(::windows::core::Vtable::as_raw(self), pobjectid.into().abi(), encoding, ::core::mem::transmute_copy(strencodeddata)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ObjectId(&self) -> ::windows::core::Result<IObjectId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ObjectId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn get_RawData(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_RawData)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509AttributeClientId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509AttributeClientId {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509AttributeClientId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509AttributeClientId").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IX509AttributeClientId {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize<P0>(&self, pobjectid: P0, encoding: EncodingType, strencodeddata: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IObjectId>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Initialize)(::windows::core::Vtable::as_raw(self), pobjectid.into().abi(), encoding, ::core::mem::transmute_copy(strencodeddata)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ObjectId(&self) -> ::windows::core::Result<IObjectId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ObjectId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn get_RawData(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_RawData)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509AttributeCspProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509AttributeCspProvider {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509AttributeCspProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509AttributeCspProvider").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IX509AttributeCspProvider {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize<P0>(&self, pobjectid: P0, encoding: EncodingType, strencodeddata: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IObjectId>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Initialize)(::windows::core::Vtable::as_raw(self), pobjectid.into().abi(), encoding, ::core::mem::transmute_copy(strencodeddata)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ObjectId(&self) -> ::windows::core::Result<IObjectId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ObjectId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn get_RawData(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_RawData)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509AttributeExtensions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509AttributeExtensions {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509AttributeExtensions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509AttributeExtensions").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IX509AttributeExtensions {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize<P0>(&self, pobjectid: P0, encoding: EncodingType, strencodeddata: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IObjectId>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Initialize)(::windows::core::Vtable::as_raw(self), pobjectid.into().abi(), encoding, ::core::mem::transmute_copy(strencodeddata)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ObjectId(&self) -> ::windows::core::Result<IObjectId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ObjectId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn get_RawData(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_RawData)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509AttributeOSVersion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509AttributeOSVersion {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509AttributeOSVersion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509AttributeOSVersion").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IX509AttributeOSVersion {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize<P0>(&self, pobjectid: P0, encoding: EncodingType, strencodeddata: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IObjectId>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Initialize)(::windows::core::Vtable::as_raw(self), pobjectid.into().abi(), encoding, ::core::mem::transmute_copy(strencodeddata)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ObjectId(&self) -> ::windows::core::Result<IObjectId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ObjectId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn get_RawData(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_RawData)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509AttributeRenewalCertificate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509AttributeRenewalCertificate {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509AttributeRenewalCertificate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509AttributeRenewalCertificate").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IX509AttributeRenewalCertificate {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize<P0>(&self, pobjectid: P0, encoding: EncodingType, strencodeddata: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IObjectId>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Initialize)(::windows::core::Vtable::as_raw(self), pobjectid.into().abi(), encoding, ::core::mem::transmute_copy(strencodeddata)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ObjectId(&self) -> ::windows::core::Result<IObjectId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ObjectId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn get_RawData(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_RawData)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509Attributes {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509Attributes {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509Attributes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509Attributes").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509CertificateRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509CertificateRequest {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509CertificateRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509CertificateRequest").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509CertificateRequestCertificate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509CertificateRequestCertificate {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509CertificateRequestCertificate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509CertificateRequestCertificate").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IX509CertificateRequestCertificate {
    pub unsafe fn Initialize(&self, context: X509CertificateEnrollmentContext) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Initialize)(::windows::core::Vtable::as_raw(self), context).ok()
    }
    pub unsafe fn Encode(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Encode)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ResetForEncode(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.ResetForEncode)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetInnerRequest(&self, level: InnerRequestLevel) -> ::windows::core::Result<IX509CertificateRequest> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetInnerRequest)(::windows::core::Vtable::as_raw(self), level, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Type(&self) -> ::windows::core::Result<X509RequestType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Type)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnrollmentContext(&self) -> ::windows::core::Result<X509CertificateEnrollmentContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.EnrollmentContext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Silent(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Silent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSilent<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetSilent)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
    pub unsafe fn ParentWindow(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.ParentWindow)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetParentWindow(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetParentWindow)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn UIContextMessage(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.UIContextMessage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetUIContextMessage(&self, value: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetUIContextMessage)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SuppressDefaults(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.SuppressDefaults)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSuppressDefaults<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetSuppressDefaults)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
    pub unsafe fn get_RenewalCertificate(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.get_RenewalCertificate)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn put_RenewalCertificate(&self, encoding: EncodingType, value: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.put_RenewalCertificate)(::windows::core::Vtable::as_raw(self), encoding, ::core::mem::transmute_copy(value)).ok()
    }
    pub unsafe fn ClientId(&self) -> ::windows::core::Result<RequestClientInfoClientId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.ClientId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetClientId(&self, value: RequestClientInfoClientId) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetClientId)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CspInformations(&self) -> ::windows::core::Result<ICspInformations> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CspInformations)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetCspInformations<P0>(&self, pvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ICspInformations>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetCspInformations)(::windows::core::Vtable::as_raw(self), pvalue.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn HashAlgorithm(&self) -> ::windows::core::Result<IObjectId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.HashAlgorithm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetHashAlgorithm<P0>(&self, pvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IObjectId>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetHashAlgorithm)(::windows::core::Vtable::as_raw(self), pvalue.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AlternateSignatureAlgorithm(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.AlternateSignatureAlgorithm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAlternateSignatureAlgorithm<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetAlternateSignatureAlgorithm)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
    pub unsafe fn get_RawData(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.get_RawData)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn InitializeFromTemplateName(&self, context: X509CertificateEnrollmentContext, strtemplatename: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.InitializeFromTemplateName)(::windows::core::Vtable::as_raw(self), context, ::core::mem::transmute_copy(strtemplatename)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeFromPrivateKey<P0>(&self, context: X509CertificateEnrollmentContext, pprivatekey: P0, strtemplatename: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IX509PrivateKey>>,
    {
        (::windows::core::Vtable::vtable(self).base__.InitializeFromPrivateKey)(::windows::core::Vtable::as_raw(self), context, pprivatekey.into().abi(), ::core::mem::transmute_copy(strtemplatename)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeFromPublicKey<P0>(&self, context: X509CertificateEnrollmentContext, ppublickey: P0, strtemplatename: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IX509PublicKey>>,
    {
        (::windows::core::Vtable::vtable(self).base__.InitializeFromPublicKey)(::windows::core::Vtable::as_raw(self), context, ppublickey.into().abi(), ::core::mem::transmute_copy(strtemplatename)).ok()
    }
    pub unsafe fn InitializeFromCertificate(&self, context: X509CertificateEnrollmentContext, strcertificate: &::windows::core::BSTR, encoding: EncodingType, inheritoptions: X509RequestInheritOptions) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.InitializeFromCertificate)(::windows::core::Vtable::as_raw(self), context, ::core::mem::transmute_copy(strcertificate), encoding, inheritoptions).ok()
    }
    pub unsafe fn InitializeDecode(&self, strencodeddata: &::windows::core::BSTR, encoding: EncodingType) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.InitializeDecode)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strencodeddata), encoding).ok()
    }
    pub unsafe fn CheckSignature(&self, allowedsignaturetypes: Pkcs10AllowedSignatureTypes) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CheckSignature)(::windows::core::Vtable::as_raw(self), allowedsignaturetypes).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSmartCard(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsSmartCard)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn TemplateObjectId(&self) -> ::windows::core::Result<IObjectId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.TemplateObjectId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PublicKey(&self) -> ::windows::core::Result<IX509PublicKey> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.PublicKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrivateKey(&self) -> ::windows::core::Result<IX509PrivateKey> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.PrivateKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn NullSigned(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.NullSigned)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReuseKey(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ReuseKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn get_OldCertificate(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_OldCertificate)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Subject(&self) -> ::windows::core::Result<IX500DistinguishedName> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Subject)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSubject<P0>(&self, pvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IX500DistinguishedName>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetSubject)(::windows::core::Vtable::as_raw(self), pvalue.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CspStatuses(&self) -> ::windows::core::Result<ICspStatuses> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CspStatuses)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SmimeCapabilities(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SmimeCapabilities)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSmimeCapabilities<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetSmimeCapabilities)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SignatureInformation(&self) -> ::windows::core::Result<IX509SignatureInformation> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SignatureInformation)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn KeyContainerNamePrefix(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.KeyContainerNamePrefix)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetKeyContainerNamePrefix(&self, value: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetKeyContainerNamePrefix)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CryptAttributes(&self) -> ::windows::core::Result<ICryptAttributes> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CryptAttributes)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn X509Extensions(&self) -> ::windows::core::Result<IX509Extensions> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.X509Extensions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CriticalExtensions(&self) -> ::windows::core::Result<IObjectIds> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CriticalExtensions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SuppressOids(&self) -> ::windows::core::Result<IObjectIds> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SuppressOids)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn get_RawDataToBeSigned(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_RawDataToBeSigned)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn get_Signature(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_Signature)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetCspStatuses(&self, keyspec: X509KeySpec) -> ::windows::core::Result<ICspStatuses> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCspStatuses)(::windows::core::Vtable::as_raw(self), keyspec, result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509CertificateRequestCertificate2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509CertificateRequestCertificate2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509CertificateRequestCertificate2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509CertificateRequestCertificate2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IX509CertificateRequestCertificate2 {
    pub unsafe fn Initialize(&self, context: X509CertificateEnrollmentContext) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Initialize)(::windows::core::Vtable::as_raw(self), context).ok()
    }
    pub unsafe fn Encode(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Encode)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ResetForEncode(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ResetForEncode)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetInnerRequest(&self, level: InnerRequestLevel) -> ::windows::core::Result<IX509CertificateRequest> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetInnerRequest)(::windows::core::Vtable::as_raw(self), level, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Type(&self) -> ::windows::core::Result<X509RequestType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Type)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnrollmentContext(&self) -> ::windows::core::Result<X509CertificateEnrollmentContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.EnrollmentContext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Silent(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Silent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSilent<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetSilent)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
    pub unsafe fn ParentWindow(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ParentWindow)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetParentWindow(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetParentWindow)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn UIContextMessage(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.UIContextMessage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetUIContextMessage(&self, value: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetUIContextMessage)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SuppressDefaults(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SuppressDefaults)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSuppressDefaults<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetSuppressDefaults)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
    pub unsafe fn get_RenewalCertificate(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.get_RenewalCertificate)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn put_RenewalCertificate(&self, encoding: EncodingType, value: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.put_RenewalCertificate)(::windows::core::Vtable::as_raw(self), encoding, ::core::mem::transmute_copy(value)).ok()
    }
    pub unsafe fn ClientId(&self) -> ::windows::core::Result<RequestClientInfoClientId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ClientId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetClientId(&self, value: RequestClientInfoClientId) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetClientId)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CspInformations(&self) -> ::windows::core::Result<ICspInformations> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CspInformations)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetCspInformations<P0>(&self, pvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ICspInformations>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetCspInformations)(::windows::core::Vtable::as_raw(self), pvalue.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn HashAlgorithm(&self) -> ::windows::core::Result<IObjectId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.HashAlgorithm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetHashAlgorithm<P0>(&self, pvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IObjectId>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetHashAlgorithm)(::windows::core::Vtable::as_raw(self), pvalue.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AlternateSignatureAlgorithm(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.AlternateSignatureAlgorithm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAlternateSignatureAlgorithm<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetAlternateSignatureAlgorithm)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
    pub unsafe fn get_RawData(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.get_RawData)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn InitializeFromTemplateName(&self, context: X509CertificateEnrollmentContext, strtemplatename: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.InitializeFromTemplateName)(::windows::core::Vtable::as_raw(self), context, ::core::mem::transmute_copy(strtemplatename)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeFromPrivateKey<P0>(&self, context: X509CertificateEnrollmentContext, pprivatekey: P0, strtemplatename: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IX509PrivateKey>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.InitializeFromPrivateKey)(::windows::core::Vtable::as_raw(self), context, pprivatekey.into().abi(), ::core::mem::transmute_copy(strtemplatename)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeFromPublicKey<P0>(&self, context: X509CertificateEnrollmentContext, ppublickey: P0, strtemplatename: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IX509PublicKey>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.InitializeFromPublicKey)(::windows::core::Vtable::as_raw(self), context, ppublickey.into().abi(), ::core::mem::transmute_copy(strtemplatename)).ok()
    }
    pub unsafe fn InitializeFromCertificate(&self, context: X509CertificateEnrollmentContext, strcertificate: &::windows::core::BSTR, encoding: EncodingType, inheritoptions: X509RequestInheritOptions) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.InitializeFromCertificate)(::windows::core::Vtable::as_raw(self), context, ::core::mem::transmute_copy(strcertificate), encoding, inheritoptions).ok()
    }
    pub unsafe fn InitializeDecode(&self, strencodeddata: &::windows::core::BSTR, encoding: EncodingType) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.InitializeDecode)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strencodeddata), encoding).ok()
    }
    pub unsafe fn CheckSignature(&self, allowedsignaturetypes: Pkcs10AllowedSignatureTypes) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CheckSignature)(::windows::core::Vtable::as_raw(self), allowedsignaturetypes).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSmartCard(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.IsSmartCard)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn TemplateObjectId(&self) -> ::windows::core::Result<IObjectId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.TemplateObjectId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PublicKey(&self) -> ::windows::core::Result<IX509PublicKey> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.PublicKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrivateKey(&self) -> ::windows::core::Result<IX509PrivateKey> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.PrivateKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn NullSigned(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.NullSigned)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReuseKey(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.ReuseKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn get_OldCertificate(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.get_OldCertificate)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Subject(&self) -> ::windows::core::Result<IX500DistinguishedName> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Subject)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSubject<P0>(&self, pvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IX500DistinguishedName>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetSubject)(::windows::core::Vtable::as_raw(self), pvalue.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CspStatuses(&self) -> ::windows::core::Result<ICspStatuses> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CspStatuses)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SmimeCapabilities(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.SmimeCapabilities)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSmimeCapabilities<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetSmimeCapabilities)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SignatureInformation(&self) -> ::windows::core::Result<IX509SignatureInformation> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.SignatureInformation)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn KeyContainerNamePrefix(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.KeyContainerNamePrefix)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetKeyContainerNamePrefix(&self, value: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetKeyContainerNamePrefix)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CryptAttributes(&self) -> ::windows::core::Result<ICryptAttributes> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CryptAttributes)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn X509Extensions(&self) -> ::windows::core::Result<IX509Extensions> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.X509Extensions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CriticalExtensions(&self) -> ::windows::core::Result<IObjectIds> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CriticalExtensions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SuppressOids(&self) -> ::windows::core::Result<IObjectIds> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.SuppressOids)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn get_RawDataToBeSigned(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.get_RawDataToBeSigned)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn get_Signature(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.get_Signature)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetCspStatuses(&self, keyspec: X509KeySpec) -> ::windows::core::Result<ICspStatuses> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetCspStatuses)(::windows::core::Vtable::as_raw(self), keyspec, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CheckPublicKeySignature<P0>(&self, ppublickey: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IX509PublicKey>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CheckPublicKeySignature)(::windows::core::Vtable::as_raw(self), ppublickey.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Issuer(&self) -> ::windows::core::Result<IX500DistinguishedName> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Issuer)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetIssuer<P0>(&self, pvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IX500DistinguishedName>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetIssuer)(::windows::core::Vtable::as_raw(self), pvalue.into().abi()).ok()
    }
    pub unsafe fn NotBefore(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.NotBefore)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetNotBefore(&self, value: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetNotBefore)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn NotAfter(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.NotAfter)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetNotAfter(&self, value: f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetNotAfter)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn get_SerialNumber(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_SerialNumber)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn put_SerialNumber(&self, encoding: EncodingType, value: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.put_SerialNumber)(::windows::core::Vtable::as_raw(self), encoding, ::core::mem::transmute_copy(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SignerCertificate(&self) -> ::windows::core::Result<ISignerCertificate> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SignerCertificate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSignerCertificate<P0>(&self, pvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ISignerCertificate>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetSignerCertificate)(::windows::core::Vtable::as_raw(self), pvalue.into().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509CertificateRequestCmc {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509CertificateRequestCmc {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509CertificateRequestCmc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509CertificateRequestCmc").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IX509CertificateRequestCmc {
    pub unsafe fn Initialize(&self, context: X509CertificateEnrollmentContext) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Initialize)(::windows::core::Vtable::as_raw(self), context).ok()
    }
    pub unsafe fn Encode(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Encode)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ResetForEncode(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.ResetForEncode)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetInnerRequest(&self, level: InnerRequestLevel) -> ::windows::core::Result<IX509CertificateRequest> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetInnerRequest)(::windows::core::Vtable::as_raw(self), level, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Type(&self) -> ::windows::core::Result<X509RequestType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Type)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnrollmentContext(&self) -> ::windows::core::Result<X509CertificateEnrollmentContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.EnrollmentContext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Silent(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Silent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSilent<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetSilent)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
    pub unsafe fn ParentWindow(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.ParentWindow)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetParentWindow(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetParentWindow)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn UIContextMessage(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.UIContextMessage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetUIContextMessage(&self, value: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetUIContextMessage)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SuppressDefaults(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.SuppressDefaults)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSuppressDefaults<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetSuppressDefaults)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
    pub unsafe fn get_RenewalCertificate(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.get_RenewalCertificate)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn put_RenewalCertificate(&self, encoding: EncodingType, value: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.put_RenewalCertificate)(::windows::core::Vtable::as_raw(self), encoding, ::core::mem::transmute_copy(value)).ok()
    }
    pub unsafe fn ClientId(&self) -> ::windows::core::Result<RequestClientInfoClientId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.ClientId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetClientId(&self, value: RequestClientInfoClientId) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetClientId)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CspInformations(&self) -> ::windows::core::Result<ICspInformations> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CspInformations)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetCspInformations<P0>(&self, pvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ICspInformations>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetCspInformations)(::windows::core::Vtable::as_raw(self), pvalue.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn HashAlgorithm(&self) -> ::windows::core::Result<IObjectId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.HashAlgorithm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetHashAlgorithm<P0>(&self, pvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IObjectId>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetHashAlgorithm)(::windows::core::Vtable::as_raw(self), pvalue.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AlternateSignatureAlgorithm(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.AlternateSignatureAlgorithm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAlternateSignatureAlgorithm<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetAlternateSignatureAlgorithm)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
    pub unsafe fn get_RawData(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.get_RawData)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn InitializeFromTemplateName(&self, context: X509CertificateEnrollmentContext, strtemplatename: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.InitializeFromTemplateName)(::windows::core::Vtable::as_raw(self), context, ::core::mem::transmute_copy(strtemplatename)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InitializeFromCertificate<P0>(&self, context: X509CertificateEnrollmentContext, renewalrequest: P0, strcertificate: &::windows::core::BSTR, encoding: EncodingType, inheritoptions: X509RequestInheritOptions) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.InitializeFromCertificate)(::windows::core::Vtable::as_raw(self), context, renewalrequest.into(), ::core::mem::transmute_copy(strcertificate), encoding, inheritoptions).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeFromInnerRequest<P0>(&self, pinnerrequest: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IX509CertificateRequest>>,
    {
        (::windows::core::Vtable::vtable(self).base__.InitializeFromInnerRequest)(::windows::core::Vtable::as_raw(self), pinnerrequest.into().abi()).ok()
    }
    pub unsafe fn InitializeDecode(&self, strencodeddata: &::windows::core::BSTR, encoding: EncodingType) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.InitializeDecode)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strencodeddata), encoding).ok()
    }
    pub unsafe fn RequesterName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.RequesterName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetRequesterName(&self, value: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetRequesterName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SignerCertificate(&self) -> ::windows::core::Result<ISignerCertificate> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SignerCertificate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSignerCertificate<P0>(&self, pvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ISignerCertificate>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetSignerCertificate)(::windows::core::Vtable::as_raw(self), pvalue.into().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509CertificateRequestCmc2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509CertificateRequestCmc2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509CertificateRequestCmc2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509CertificateRequestCmc2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IX509CertificateRequestCmc2 {
    pub unsafe fn Initialize(&self, context: X509CertificateEnrollmentContext) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Initialize)(::windows::core::Vtable::as_raw(self), context).ok()
    }
    pub unsafe fn Encode(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Encode)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ResetForEncode(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ResetForEncode)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetInnerRequest(&self, level: InnerRequestLevel) -> ::windows::core::Result<IX509CertificateRequest> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetInnerRequest)(::windows::core::Vtable::as_raw(self), level, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Type(&self) -> ::windows::core::Result<X509RequestType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Type)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnrollmentContext(&self) -> ::windows::core::Result<X509CertificateEnrollmentContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.EnrollmentContext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Silent(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Silent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSilent<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetSilent)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
    pub unsafe fn ParentWindow(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ParentWindow)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetParentWindow(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetParentWindow)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn UIContextMessage(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.UIContextMessage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetUIContextMessage(&self, value: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetUIContextMessage)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SuppressDefaults(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SuppressDefaults)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSuppressDefaults<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetSuppressDefaults)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
    pub unsafe fn get_RenewalCertificate(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.get_RenewalCertificate)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn put_RenewalCertificate(&self, encoding: EncodingType, value: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.put_RenewalCertificate)(::windows::core::Vtable::as_raw(self), encoding, ::core::mem::transmute_copy(value)).ok()
    }
    pub unsafe fn ClientId(&self) -> ::windows::core::Result<RequestClientInfoClientId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ClientId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetClientId(&self, value: RequestClientInfoClientId) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetClientId)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CspInformations(&self) -> ::windows::core::Result<ICspInformations> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CspInformations)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetCspInformations<P0>(&self, pvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ICspInformations>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetCspInformations)(::windows::core::Vtable::as_raw(self), pvalue.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn HashAlgorithm(&self) -> ::windows::core::Result<IObjectId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.HashAlgorithm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetHashAlgorithm<P0>(&self, pvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IObjectId>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetHashAlgorithm)(::windows::core::Vtable::as_raw(self), pvalue.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AlternateSignatureAlgorithm(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.AlternateSignatureAlgorithm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAlternateSignatureAlgorithm<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetAlternateSignatureAlgorithm)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
    pub unsafe fn get_RawData(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.get_RawData)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn InitializeFromTemplateName(&self, context: X509CertificateEnrollmentContext, strtemplatename: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.InitializeFromTemplateName)(::windows::core::Vtable::as_raw(self), context, ::core::mem::transmute_copy(strtemplatename)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InitializeFromCertificate<P0>(&self, context: X509CertificateEnrollmentContext, renewalrequest: P0, strcertificate: &::windows::core::BSTR, encoding: EncodingType, inheritoptions: X509RequestInheritOptions) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.InitializeFromCertificate)(::windows::core::Vtable::as_raw(self), context, renewalrequest.into(), ::core::mem::transmute_copy(strcertificate), encoding, inheritoptions).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeFromInnerRequest<P0>(&self, pinnerrequest: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IX509CertificateRequest>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.InitializeFromInnerRequest)(::windows::core::Vtable::as_raw(self), pinnerrequest.into().abi()).ok()
    }
    pub unsafe fn InitializeDecode(&self, strencodeddata: &::windows::core::BSTR, encoding: EncodingType) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.InitializeDecode)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strencodeddata), encoding).ok()
    }
    pub unsafe fn RequesterName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.RequesterName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetRequesterName(&self, value: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetRequesterName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SignerCertificate(&self) -> ::windows::core::Result<ISignerCertificate> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.SignerCertificate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSignerCertificate<P0>(&self, pvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ISignerCertificate>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetSignerCertificate)(::windows::core::Vtable::as_raw(self), pvalue.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeFromInnerRequestTemplateName<P0>(&self, pinnerrequest: P0, strtemplatename: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IX509CertificateRequest>>,
    {
        (::windows::core::Vtable::vtable(self).base__.InitializeFromInnerRequestTemplateName)(::windows::core::Vtable::as_raw(self), pinnerrequest.into().abi(), ::core::mem::transmute_copy(strtemplatename)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn TemplateObjectId(&self) -> ::windows::core::Result<IObjectId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.TemplateObjectId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn NullSigned(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.NullSigned)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CryptAttributes(&self) -> ::windows::core::Result<ICryptAttributes> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CryptAttributes)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NameValuePairs(&self) -> ::windows::core::Result<IX509NameValuePairs> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.NameValuePairs)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn X509Extensions(&self) -> ::windows::core::Result<IX509Extensions> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.X509Extensions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CriticalExtensions(&self) -> ::windows::core::Result<IObjectIds> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CriticalExtensions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SuppressOids(&self) -> ::windows::core::Result<IObjectIds> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SuppressOids)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn TransactionId(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.TransactionId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTransactionId(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetTransactionId)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn get_SenderNonce(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_SenderNonce)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn put_SenderNonce(&self, encoding: EncodingType, value: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.put_SenderNonce)(::windows::core::Vtable::as_raw(self), encoding, ::core::mem::transmute_copy(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SignatureInformation(&self) -> ::windows::core::Result<IX509SignatureInformation> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SignatureInformation)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ArchivePrivateKey(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ArchivePrivateKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetArchivePrivateKey<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetArchivePrivateKey)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
    pub unsafe fn get_KeyArchivalCertificate(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_KeyArchivalCertificate)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn put_KeyArchivalCertificate(&self, encoding: EncodingType, value: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.put_KeyArchivalCertificate)(::windows::core::Vtable::as_raw(self), encoding, ::core::mem::transmute_copy(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EncryptionAlgorithm(&self) -> ::windows::core::Result<IObjectId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EncryptionAlgorithm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetEncryptionAlgorithm<P0>(&self, pvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IObjectId>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetEncryptionAlgorithm)(::windows::core::Vtable::as_raw(self), pvalue.into().abi()).ok()
    }
    pub unsafe fn EncryptionStrength(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EncryptionStrength)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetEncryptionStrength(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetEncryptionStrength)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn get_EncryptedKeyHash(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_EncryptedKeyHash)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SignerCertificates(&self) -> ::windows::core::Result<ISignerCertificates> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SignerCertificates)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509CertificateRequestPkcs10 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509CertificateRequestPkcs10 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509CertificateRequestPkcs10 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509CertificateRequestPkcs10").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IX509CertificateRequestPkcs10 {
    pub unsafe fn Initialize(&self, context: X509CertificateEnrollmentContext) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Initialize)(::windows::core::Vtable::as_raw(self), context).ok()
    }
    pub unsafe fn Encode(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Encode)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ResetForEncode(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ResetForEncode)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetInnerRequest(&self, level: InnerRequestLevel) -> ::windows::core::Result<IX509CertificateRequest> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetInnerRequest)(::windows::core::Vtable::as_raw(self), level, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Type(&self) -> ::windows::core::Result<X509RequestType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Type)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnrollmentContext(&self) -> ::windows::core::Result<X509CertificateEnrollmentContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnrollmentContext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Silent(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Silent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSilent<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetSilent)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
    pub unsafe fn ParentWindow(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ParentWindow)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetParentWindow(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetParentWindow)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn UIContextMessage(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.UIContextMessage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetUIContextMessage(&self, value: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetUIContextMessage)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SuppressDefaults(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SuppressDefaults)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSuppressDefaults<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetSuppressDefaults)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
    pub unsafe fn get_RenewalCertificate(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_RenewalCertificate)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn put_RenewalCertificate(&self, encoding: EncodingType, value: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.put_RenewalCertificate)(::windows::core::Vtable::as_raw(self), encoding, ::core::mem::transmute_copy(value)).ok()
    }
    pub unsafe fn ClientId(&self) -> ::windows::core::Result<RequestClientInfoClientId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ClientId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetClientId(&self, value: RequestClientInfoClientId) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetClientId)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CspInformations(&self) -> ::windows::core::Result<ICspInformations> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CspInformations)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetCspInformations<P0>(&self, pvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ICspInformations>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetCspInformations)(::windows::core::Vtable::as_raw(self), pvalue.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn HashAlgorithm(&self) -> ::windows::core::Result<IObjectId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.HashAlgorithm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetHashAlgorithm<P0>(&self, pvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IObjectId>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetHashAlgorithm)(::windows::core::Vtable::as_raw(self), pvalue.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AlternateSignatureAlgorithm(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.AlternateSignatureAlgorithm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAlternateSignatureAlgorithm<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetAlternateSignatureAlgorithm)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
    pub unsafe fn get_RawData(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_RawData)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509CertificateRequestPkcs10V2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509CertificateRequestPkcs10V2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509CertificateRequestPkcs10V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509CertificateRequestPkcs10V2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IX509CertificateRequestPkcs10V2 {
    pub unsafe fn Initialize(&self, context: X509CertificateEnrollmentContext) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Initialize)(::windows::core::Vtable::as_raw(self), context).ok()
    }
    pub unsafe fn Encode(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Encode)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ResetForEncode(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.ResetForEncode)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetInnerRequest(&self, level: InnerRequestLevel) -> ::windows::core::Result<IX509CertificateRequest> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetInnerRequest)(::windows::core::Vtable::as_raw(self), level, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Type(&self) -> ::windows::core::Result<X509RequestType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Type)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnrollmentContext(&self) -> ::windows::core::Result<X509CertificateEnrollmentContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.EnrollmentContext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Silent(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Silent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSilent<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetSilent)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
    pub unsafe fn ParentWindow(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.ParentWindow)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetParentWindow(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetParentWindow)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn UIContextMessage(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.UIContextMessage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetUIContextMessage(&self, value: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetUIContextMessage)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SuppressDefaults(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.SuppressDefaults)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSuppressDefaults<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetSuppressDefaults)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
    pub unsafe fn get_RenewalCertificate(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.get_RenewalCertificate)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn put_RenewalCertificate(&self, encoding: EncodingType, value: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.put_RenewalCertificate)(::windows::core::Vtable::as_raw(self), encoding, ::core::mem::transmute_copy(value)).ok()
    }
    pub unsafe fn ClientId(&self) -> ::windows::core::Result<RequestClientInfoClientId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.ClientId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetClientId(&self, value: RequestClientInfoClientId) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetClientId)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CspInformations(&self) -> ::windows::core::Result<ICspInformations> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CspInformations)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetCspInformations<P0>(&self, pvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ICspInformations>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetCspInformations)(::windows::core::Vtable::as_raw(self), pvalue.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn HashAlgorithm(&self) -> ::windows::core::Result<IObjectId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.HashAlgorithm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetHashAlgorithm<P0>(&self, pvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IObjectId>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetHashAlgorithm)(::windows::core::Vtable::as_raw(self), pvalue.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AlternateSignatureAlgorithm(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.AlternateSignatureAlgorithm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAlternateSignatureAlgorithm<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetAlternateSignatureAlgorithm)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
    pub unsafe fn get_RawData(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.get_RawData)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn InitializeFromTemplateName(&self, context: X509CertificateEnrollmentContext, strtemplatename: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.InitializeFromTemplateName)(::windows::core::Vtable::as_raw(self), context, ::core::mem::transmute_copy(strtemplatename)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeFromPrivateKey<P0>(&self, context: X509CertificateEnrollmentContext, pprivatekey: P0, strtemplatename: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IX509PrivateKey>>,
    {
        (::windows::core::Vtable::vtable(self).base__.InitializeFromPrivateKey)(::windows::core::Vtable::as_raw(self), context, pprivatekey.into().abi(), ::core::mem::transmute_copy(strtemplatename)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeFromPublicKey<P0>(&self, context: X509CertificateEnrollmentContext, ppublickey: P0, strtemplatename: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IX509PublicKey>>,
    {
        (::windows::core::Vtable::vtable(self).base__.InitializeFromPublicKey)(::windows::core::Vtable::as_raw(self), context, ppublickey.into().abi(), ::core::mem::transmute_copy(strtemplatename)).ok()
    }
    pub unsafe fn InitializeFromCertificate(&self, context: X509CertificateEnrollmentContext, strcertificate: &::windows::core::BSTR, encoding: EncodingType, inheritoptions: X509RequestInheritOptions) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.InitializeFromCertificate)(::windows::core::Vtable::as_raw(self), context, ::core::mem::transmute_copy(strcertificate), encoding, inheritoptions).ok()
    }
    pub unsafe fn InitializeDecode(&self, strencodeddata: &::windows::core::BSTR, encoding: EncodingType) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.InitializeDecode)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strencodeddata), encoding).ok()
    }
    pub unsafe fn CheckSignature(&self, allowedsignaturetypes: Pkcs10AllowedSignatureTypes) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CheckSignature)(::windows::core::Vtable::as_raw(self), allowedsignaturetypes).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSmartCard(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsSmartCard)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn TemplateObjectId(&self) -> ::windows::core::Result<IObjectId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.TemplateObjectId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PublicKey(&self) -> ::windows::core::Result<IX509PublicKey> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.PublicKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrivateKey(&self) -> ::windows::core::Result<IX509PrivateKey> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.PrivateKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn NullSigned(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.NullSigned)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReuseKey(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ReuseKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn get_OldCertificate(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_OldCertificate)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Subject(&self) -> ::windows::core::Result<IX500DistinguishedName> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Subject)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSubject<P0>(&self, pvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IX500DistinguishedName>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetSubject)(::windows::core::Vtable::as_raw(self), pvalue.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CspStatuses(&self) -> ::windows::core::Result<ICspStatuses> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CspStatuses)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SmimeCapabilities(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SmimeCapabilities)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSmimeCapabilities<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetSmimeCapabilities)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SignatureInformation(&self) -> ::windows::core::Result<IX509SignatureInformation> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SignatureInformation)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn KeyContainerNamePrefix(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.KeyContainerNamePrefix)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetKeyContainerNamePrefix(&self, value: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetKeyContainerNamePrefix)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CryptAttributes(&self) -> ::windows::core::Result<ICryptAttributes> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CryptAttributes)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn X509Extensions(&self) -> ::windows::core::Result<IX509Extensions> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.X509Extensions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CriticalExtensions(&self) -> ::windows::core::Result<IObjectIds> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CriticalExtensions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SuppressOids(&self) -> ::windows::core::Result<IObjectIds> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SuppressOids)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn get_RawDataToBeSigned(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_RawDataToBeSigned)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn get_Signature(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_Signature)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetCspStatuses(&self, keyspec: X509KeySpec) -> ::windows::core::Result<ICspStatuses> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCspStatuses)(::windows::core::Vtable::as_raw(self), keyspec, result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509CertificateRequestPkcs10V3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509CertificateRequestPkcs10V3 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509CertificateRequestPkcs10V3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509CertificateRequestPkcs10V3").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IX509CertificateRequestPkcs10V3 {
    pub unsafe fn Initialize(&self, context: X509CertificateEnrollmentContext) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Initialize)(::windows::core::Vtable::as_raw(self), context).ok()
    }
    pub unsafe fn Encode(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Encode)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ResetForEncode(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ResetForEncode)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetInnerRequest(&self, level: InnerRequestLevel) -> ::windows::core::Result<IX509CertificateRequest> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetInnerRequest)(::windows::core::Vtable::as_raw(self), level, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Type(&self) -> ::windows::core::Result<X509RequestType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Type)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnrollmentContext(&self) -> ::windows::core::Result<X509CertificateEnrollmentContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.EnrollmentContext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Silent(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Silent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSilent<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetSilent)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
    pub unsafe fn ParentWindow(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ParentWindow)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetParentWindow(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetParentWindow)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn UIContextMessage(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.UIContextMessage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetUIContextMessage(&self, value: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetUIContextMessage)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SuppressDefaults(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SuppressDefaults)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSuppressDefaults<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetSuppressDefaults)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
    pub unsafe fn get_RenewalCertificate(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.get_RenewalCertificate)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn put_RenewalCertificate(&self, encoding: EncodingType, value: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.put_RenewalCertificate)(::windows::core::Vtable::as_raw(self), encoding, ::core::mem::transmute_copy(value)).ok()
    }
    pub unsafe fn ClientId(&self) -> ::windows::core::Result<RequestClientInfoClientId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ClientId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetClientId(&self, value: RequestClientInfoClientId) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetClientId)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CspInformations(&self) -> ::windows::core::Result<ICspInformations> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CspInformations)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetCspInformations<P0>(&self, pvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ICspInformations>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetCspInformations)(::windows::core::Vtable::as_raw(self), pvalue.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn HashAlgorithm(&self) -> ::windows::core::Result<IObjectId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.HashAlgorithm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetHashAlgorithm<P0>(&self, pvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IObjectId>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetHashAlgorithm)(::windows::core::Vtable::as_raw(self), pvalue.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AlternateSignatureAlgorithm(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.AlternateSignatureAlgorithm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAlternateSignatureAlgorithm<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetAlternateSignatureAlgorithm)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
    pub unsafe fn get_RawData(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.get_RawData)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn InitializeFromTemplateName(&self, context: X509CertificateEnrollmentContext, strtemplatename: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.InitializeFromTemplateName)(::windows::core::Vtable::as_raw(self), context, ::core::mem::transmute_copy(strtemplatename)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeFromPrivateKey<P0>(&self, context: X509CertificateEnrollmentContext, pprivatekey: P0, strtemplatename: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IX509PrivateKey>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.InitializeFromPrivateKey)(::windows::core::Vtable::as_raw(self), context, pprivatekey.into().abi(), ::core::mem::transmute_copy(strtemplatename)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeFromPublicKey<P0>(&self, context: X509CertificateEnrollmentContext, ppublickey: P0, strtemplatename: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IX509PublicKey>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.InitializeFromPublicKey)(::windows::core::Vtable::as_raw(self), context, ppublickey.into().abi(), ::core::mem::transmute_copy(strtemplatename)).ok()
    }
    pub unsafe fn InitializeFromCertificate(&self, context: X509CertificateEnrollmentContext, strcertificate: &::windows::core::BSTR, encoding: EncodingType, inheritoptions: X509RequestInheritOptions) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.InitializeFromCertificate)(::windows::core::Vtable::as_raw(self), context, ::core::mem::transmute_copy(strcertificate), encoding, inheritoptions).ok()
    }
    pub unsafe fn InitializeDecode(&self, strencodeddata: &::windows::core::BSTR, encoding: EncodingType) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.InitializeDecode)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strencodeddata), encoding).ok()
    }
    pub unsafe fn CheckSignature(&self, allowedsignaturetypes: Pkcs10AllowedSignatureTypes) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CheckSignature)(::windows::core::Vtable::as_raw(self), allowedsignaturetypes).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSmartCard(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.IsSmartCard)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn TemplateObjectId(&self) -> ::windows::core::Result<IObjectId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.TemplateObjectId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PublicKey(&self) -> ::windows::core::Result<IX509PublicKey> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.PublicKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrivateKey(&self) -> ::windows::core::Result<IX509PrivateKey> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.PrivateKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn NullSigned(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.NullSigned)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReuseKey(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.ReuseKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn get_OldCertificate(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.get_OldCertificate)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Subject(&self) -> ::windows::core::Result<IX500DistinguishedName> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Subject)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSubject<P0>(&self, pvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IX500DistinguishedName>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetSubject)(::windows::core::Vtable::as_raw(self), pvalue.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CspStatuses(&self) -> ::windows::core::Result<ICspStatuses> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CspStatuses)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SmimeCapabilities(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.SmimeCapabilities)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSmimeCapabilities<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetSmimeCapabilities)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SignatureInformation(&self) -> ::windows::core::Result<IX509SignatureInformation> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.SignatureInformation)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn KeyContainerNamePrefix(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.KeyContainerNamePrefix)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetKeyContainerNamePrefix(&self, value: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetKeyContainerNamePrefix)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CryptAttributes(&self) -> ::windows::core::Result<ICryptAttributes> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CryptAttributes)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn X509Extensions(&self) -> ::windows::core::Result<IX509Extensions> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.X509Extensions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CriticalExtensions(&self) -> ::windows::core::Result<IObjectIds> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CriticalExtensions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SuppressOids(&self) -> ::windows::core::Result<IObjectIds> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.SuppressOids)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn get_RawDataToBeSigned(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.get_RawDataToBeSigned)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn get_Signature(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.get_Signature)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetCspStatuses(&self, keyspec: X509KeySpec) -> ::windows::core::Result<ICspStatuses> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetCspStatuses)(::windows::core::Vtable::as_raw(self), keyspec, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeFromTemplate<P0, P1>(&self, context: X509CertificateEnrollmentContext, ppolicyserver: P0, ptemplate: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IX509EnrollmentPolicyServer>>,
        P1: ::std::convert::Into<::windows::core::InParam<IX509CertificateTemplate>>,
    {
        (::windows::core::Vtable::vtable(self).base__.InitializeFromTemplate)(::windows::core::Vtable::as_raw(self), context, ppolicyserver.into().abi(), ptemplate.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeFromPrivateKeyTemplate<P0, P1, P2>(&self, context: X509CertificateEnrollmentContext, pprivatekey: P0, ppolicyserver: P1, ptemplate: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IX509PrivateKey>>,
        P1: ::std::convert::Into<::windows::core::InParam<IX509EnrollmentPolicyServer>>,
        P2: ::std::convert::Into<::windows::core::InParam<IX509CertificateTemplate>>,
    {
        (::windows::core::Vtable::vtable(self).base__.InitializeFromPrivateKeyTemplate)(::windows::core::Vtable::as_raw(self), context, pprivatekey.into().abi(), ppolicyserver.into().abi(), ptemplate.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeFromPublicKeyTemplate<P0, P1, P2>(&self, context: X509CertificateEnrollmentContext, ppublickey: P0, ppolicyserver: P1, ptemplate: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IX509PublicKey>>,
        P1: ::std::convert::Into<::windows::core::InParam<IX509EnrollmentPolicyServer>>,
        P2: ::std::convert::Into<::windows::core::InParam<IX509CertificateTemplate>>,
    {
        (::windows::core::Vtable::vtable(self).base__.InitializeFromPublicKeyTemplate)(::windows::core::Vtable::as_raw(self), context, ppublickey.into().abi(), ppolicyserver.into().abi(), ptemplate.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PolicyServer(&self) -> ::windows::core::Result<IX509EnrollmentPolicyServer> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.PolicyServer)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Template(&self) -> ::windows::core::Result<IX509CertificateTemplate> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Template)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509CertificateRequestPkcs10V4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509CertificateRequestPkcs10V4 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509CertificateRequestPkcs10V4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509CertificateRequestPkcs10V4").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IX509CertificateRequestPkcs10V4 {
    pub unsafe fn Initialize(&self, context: X509CertificateEnrollmentContext) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.Initialize)(::windows::core::Vtable::as_raw(self), context).ok()
    }
    pub unsafe fn Encode(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.Encode)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ResetForEncode(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ResetForEncode)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetInnerRequest(&self, level: InnerRequestLevel) -> ::windows::core::Result<IX509CertificateRequest> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetInnerRequest)(::windows::core::Vtable::as_raw(self), level, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Type(&self) -> ::windows::core::Result<X509RequestType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.Type)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnrollmentContext(&self) -> ::windows::core::Result<X509CertificateEnrollmentContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.EnrollmentContext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Silent(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.Silent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSilent<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetSilent)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
    pub unsafe fn ParentWindow(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ParentWindow)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetParentWindow(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetParentWindow)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn UIContextMessage(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.UIContextMessage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetUIContextMessage(&self, value: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetUIContextMessage)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SuppressDefaults(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SuppressDefaults)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSuppressDefaults<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetSuppressDefaults)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
    pub unsafe fn get_RenewalCertificate(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.get_RenewalCertificate)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn put_RenewalCertificate(&self, encoding: EncodingType, value: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.put_RenewalCertificate)(::windows::core::Vtable::as_raw(self), encoding, ::core::mem::transmute_copy(value)).ok()
    }
    pub unsafe fn ClientId(&self) -> ::windows::core::Result<RequestClientInfoClientId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ClientId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetClientId(&self, value: RequestClientInfoClientId) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetClientId)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CspInformations(&self) -> ::windows::core::Result<ICspInformations> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CspInformations)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetCspInformations<P0>(&self, pvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ICspInformations>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetCspInformations)(::windows::core::Vtable::as_raw(self), pvalue.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn HashAlgorithm(&self) -> ::windows::core::Result<IObjectId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.HashAlgorithm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetHashAlgorithm<P0>(&self, pvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IObjectId>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetHashAlgorithm)(::windows::core::Vtable::as_raw(self), pvalue.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AlternateSignatureAlgorithm(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.AlternateSignatureAlgorithm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAlternateSignatureAlgorithm<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetAlternateSignatureAlgorithm)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
    pub unsafe fn get_RawData(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.get_RawData)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn InitializeFromTemplateName(&self, context: X509CertificateEnrollmentContext, strtemplatename: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.InitializeFromTemplateName)(::windows::core::Vtable::as_raw(self), context, ::core::mem::transmute_copy(strtemplatename)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeFromPrivateKey<P0>(&self, context: X509CertificateEnrollmentContext, pprivatekey: P0, strtemplatename: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IX509PrivateKey>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.InitializeFromPrivateKey)(::windows::core::Vtable::as_raw(self), context, pprivatekey.into().abi(), ::core::mem::transmute_copy(strtemplatename)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeFromPublicKey<P0>(&self, context: X509CertificateEnrollmentContext, ppublickey: P0, strtemplatename: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IX509PublicKey>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.InitializeFromPublicKey)(::windows::core::Vtable::as_raw(self), context, ppublickey.into().abi(), ::core::mem::transmute_copy(strtemplatename)).ok()
    }
    pub unsafe fn InitializeFromCertificate(&self, context: X509CertificateEnrollmentContext, strcertificate: &::windows::core::BSTR, encoding: EncodingType, inheritoptions: X509RequestInheritOptions) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.InitializeFromCertificate)(::windows::core::Vtable::as_raw(self), context, ::core::mem::transmute_copy(strcertificate), encoding, inheritoptions).ok()
    }
    pub unsafe fn InitializeDecode(&self, strencodeddata: &::windows::core::BSTR, encoding: EncodingType) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.InitializeDecode)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strencodeddata), encoding).ok()
    }
    pub unsafe fn CheckSignature(&self, allowedsignaturetypes: Pkcs10AllowedSignatureTypes) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CheckSignature)(::windows::core::Vtable::as_raw(self), allowedsignaturetypes).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSmartCard(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.IsSmartCard)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn TemplateObjectId(&self) -> ::windows::core::Result<IObjectId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.TemplateObjectId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PublicKey(&self) -> ::windows::core::Result<IX509PublicKey> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.PublicKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PrivateKey(&self) -> ::windows::core::Result<IX509PrivateKey> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.PrivateKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn NullSigned(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.NullSigned)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReuseKey(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ReuseKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn get_OldCertificate(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.get_OldCertificate)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Subject(&self) -> ::windows::core::Result<IX500DistinguishedName> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Subject)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSubject<P0>(&self, pvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IX500DistinguishedName>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetSubject)(::windows::core::Vtable::as_raw(self), pvalue.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CspStatuses(&self) -> ::windows::core::Result<ICspStatuses> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CspStatuses)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SmimeCapabilities(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SmimeCapabilities)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSmimeCapabilities<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetSmimeCapabilities)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SignatureInformation(&self) -> ::windows::core::Result<IX509SignatureInformation> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SignatureInformation)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn KeyContainerNamePrefix(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.KeyContainerNamePrefix)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetKeyContainerNamePrefix(&self, value: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetKeyContainerNamePrefix)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CryptAttributes(&self) -> ::windows::core::Result<ICryptAttributes> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CryptAttributes)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn X509Extensions(&self) -> ::windows::core::Result<IX509Extensions> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.X509Extensions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CriticalExtensions(&self) -> ::windows::core::Result<IObjectIds> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CriticalExtensions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SuppressOids(&self) -> ::windows::core::Result<IObjectIds> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SuppressOids)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn get_RawDataToBeSigned(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.get_RawDataToBeSigned)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn get_Signature(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.get_Signature)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetCspStatuses(&self, keyspec: X509KeySpec) -> ::windows::core::Result<ICspStatuses> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetCspStatuses)(::windows::core::Vtable::as_raw(self), keyspec, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeFromTemplate<P0, P1>(&self, context: X509CertificateEnrollmentContext, ppolicyserver: P0, ptemplate: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IX509EnrollmentPolicyServer>>,
        P1: ::std::convert::Into<::windows::core::InParam<IX509CertificateTemplate>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.InitializeFromTemplate)(::windows::core::Vtable::as_raw(self), context, ppolicyserver.into().abi(), ptemplate.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeFromPrivateKeyTemplate<P0, P1, P2>(&self, context: X509CertificateEnrollmentContext, pprivatekey: P0, ppolicyserver: P1, ptemplate: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IX509PrivateKey>>,
        P1: ::std::convert::Into<::windows::core::InParam<IX509EnrollmentPolicyServer>>,
        P2: ::std::convert::Into<::windows::core::InParam<IX509CertificateTemplate>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.InitializeFromPrivateKeyTemplate)(::windows::core::Vtable::as_raw(self), context, pprivatekey.into().abi(), ppolicyserver.into().abi(), ptemplate.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeFromPublicKeyTemplate<P0, P1, P2>(&self, context: X509CertificateEnrollmentContext, ppublickey: P0, ppolicyserver: P1, ptemplate: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IX509PublicKey>>,
        P1: ::std::convert::Into<::windows::core::InParam<IX509EnrollmentPolicyServer>>,
        P2: ::std::convert::Into<::windows::core::InParam<IX509CertificateTemplate>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.InitializeFromPublicKeyTemplate)(::windows::core::Vtable::as_raw(self), context, ppublickey.into().abi(), ppolicyserver.into().abi(), ptemplate.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PolicyServer(&self) -> ::windows::core::Result<IX509EnrollmentPolicyServer> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.PolicyServer)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Template(&self) -> ::windows::core::Result<IX509CertificateTemplate> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Template)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AttestPrivateKey(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.AttestPrivateKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAttestPrivateKey<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetAttestPrivateKey)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
    pub unsafe fn get_AttestationEncryptionCertificate(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_AttestationEncryptionCertificate)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn put_AttestationEncryptionCertificate(&self, encoding: EncodingType, value: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.put_AttestationEncryptionCertificate)(::windows::core::Vtable::as_raw(self), encoding, ::core::mem::transmute_copy(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EncryptionAlgorithm(&self) -> ::windows::core::Result<IObjectId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EncryptionAlgorithm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetEncryptionAlgorithm<P0>(&self, pvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IObjectId>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetEncryptionAlgorithm)(::windows::core::Vtable::as_raw(self), pvalue.into().abi()).ok()
    }
    pub unsafe fn EncryptionStrength(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EncryptionStrength)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetEncryptionStrength(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetEncryptionStrength)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn ChallengePassword(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ChallengePassword)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetChallengePassword(&self, value: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetChallengePassword)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NameValuePairs(&self) -> ::windows::core::Result<IX509NameValuePairs> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.NameValuePairs)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509CertificateRequestPkcs7 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509CertificateRequestPkcs7 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509CertificateRequestPkcs7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509CertificateRequestPkcs7").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IX509CertificateRequestPkcs7 {
    pub unsafe fn Initialize(&self, context: X509CertificateEnrollmentContext) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Initialize)(::windows::core::Vtable::as_raw(self), context).ok()
    }
    pub unsafe fn Encode(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Encode)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ResetForEncode(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ResetForEncode)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetInnerRequest(&self, level: InnerRequestLevel) -> ::windows::core::Result<IX509CertificateRequest> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetInnerRequest)(::windows::core::Vtable::as_raw(self), level, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Type(&self) -> ::windows::core::Result<X509RequestType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Type)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnrollmentContext(&self) -> ::windows::core::Result<X509CertificateEnrollmentContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnrollmentContext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Silent(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Silent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSilent<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetSilent)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
    pub unsafe fn ParentWindow(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ParentWindow)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetParentWindow(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetParentWindow)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn UIContextMessage(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.UIContextMessage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetUIContextMessage(&self, value: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetUIContextMessage)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SuppressDefaults(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SuppressDefaults)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSuppressDefaults<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetSuppressDefaults)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
    pub unsafe fn get_RenewalCertificate(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_RenewalCertificate)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn put_RenewalCertificate(&self, encoding: EncodingType, value: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.put_RenewalCertificate)(::windows::core::Vtable::as_raw(self), encoding, ::core::mem::transmute_copy(value)).ok()
    }
    pub unsafe fn ClientId(&self) -> ::windows::core::Result<RequestClientInfoClientId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ClientId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetClientId(&self, value: RequestClientInfoClientId) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetClientId)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CspInformations(&self) -> ::windows::core::Result<ICspInformations> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CspInformations)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetCspInformations<P0>(&self, pvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ICspInformations>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetCspInformations)(::windows::core::Vtable::as_raw(self), pvalue.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn HashAlgorithm(&self) -> ::windows::core::Result<IObjectId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.HashAlgorithm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetHashAlgorithm<P0>(&self, pvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IObjectId>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetHashAlgorithm)(::windows::core::Vtable::as_raw(self), pvalue.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AlternateSignatureAlgorithm(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.AlternateSignatureAlgorithm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAlternateSignatureAlgorithm<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetAlternateSignatureAlgorithm)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
    pub unsafe fn get_RawData(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_RawData)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509CertificateRequestPkcs7V2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509CertificateRequestPkcs7V2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509CertificateRequestPkcs7V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509CertificateRequestPkcs7V2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IX509CertificateRequestPkcs7V2 {
    pub unsafe fn Initialize(&self, context: X509CertificateEnrollmentContext) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Initialize)(::windows::core::Vtable::as_raw(self), context).ok()
    }
    pub unsafe fn Encode(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Encode)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn ResetForEncode(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.ResetForEncode)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetInnerRequest(&self, level: InnerRequestLevel) -> ::windows::core::Result<IX509CertificateRequest> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetInnerRequest)(::windows::core::Vtable::as_raw(self), level, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Type(&self) -> ::windows::core::Result<X509RequestType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Type)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnrollmentContext(&self) -> ::windows::core::Result<X509CertificateEnrollmentContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.EnrollmentContext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Silent(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.Silent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSilent<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetSilent)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
    pub unsafe fn ParentWindow(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.ParentWindow)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetParentWindow(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetParentWindow)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn UIContextMessage(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.UIContextMessage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetUIContextMessage(&self, value: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetUIContextMessage)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SuppressDefaults(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.SuppressDefaults)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSuppressDefaults<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetSuppressDefaults)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
    pub unsafe fn get_RenewalCertificate(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.get_RenewalCertificate)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn put_RenewalCertificate(&self, encoding: EncodingType, value: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.put_RenewalCertificate)(::windows::core::Vtable::as_raw(self), encoding, ::core::mem::transmute_copy(value)).ok()
    }
    pub unsafe fn ClientId(&self) -> ::windows::core::Result<RequestClientInfoClientId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.ClientId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetClientId(&self, value: RequestClientInfoClientId) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetClientId)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CspInformations(&self) -> ::windows::core::Result<ICspInformations> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CspInformations)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetCspInformations<P0>(&self, pvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ICspInformations>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetCspInformations)(::windows::core::Vtable::as_raw(self), pvalue.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn HashAlgorithm(&self) -> ::windows::core::Result<IObjectId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.HashAlgorithm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetHashAlgorithm<P0>(&self, pvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IObjectId>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetHashAlgorithm)(::windows::core::Vtable::as_raw(self), pvalue.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AlternateSignatureAlgorithm(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.AlternateSignatureAlgorithm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAlternateSignatureAlgorithm<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetAlternateSignatureAlgorithm)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
    pub unsafe fn get_RawData(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.get_RawData)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn InitializeFromTemplateName(&self, context: X509CertificateEnrollmentContext, strtemplatename: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.InitializeFromTemplateName)(::windows::core::Vtable::as_raw(self), context, ::core::mem::transmute_copy(strtemplatename)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InitializeFromCertificate<P0>(&self, context: X509CertificateEnrollmentContext, renewalrequest: P0, strcertificate: &::windows::core::BSTR, encoding: EncodingType, inheritoptions: X509RequestInheritOptions) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.InitializeFromCertificate)(::windows::core::Vtable::as_raw(self), context, renewalrequest.into(), ::core::mem::transmute_copy(strcertificate), encoding, inheritoptions).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeFromInnerRequest<P0>(&self, pinnerrequest: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IX509CertificateRequest>>,
    {
        (::windows::core::Vtable::vtable(self).base__.InitializeFromInnerRequest)(::windows::core::Vtable::as_raw(self), pinnerrequest.into().abi()).ok()
    }
    pub unsafe fn InitializeDecode(&self, strencodeddata: &::windows::core::BSTR, encoding: EncodingType) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.InitializeDecode)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strencodeddata), encoding).ok()
    }
    pub unsafe fn RequesterName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.RequesterName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetRequesterName(&self, value: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetRequesterName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SignerCertificate(&self) -> ::windows::core::Result<ISignerCertificate> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SignerCertificate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSignerCertificate<P0>(&self, pvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ISignerCertificate>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetSignerCertificate)(::windows::core::Vtable::as_raw(self), pvalue.into().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509CertificateRevocationList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509CertificateRevocationList {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509CertificateRevocationList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509CertificateRevocationList").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509CertificateRevocationListEntries {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509CertificateRevocationListEntries {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509CertificateRevocationListEntries {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509CertificateRevocationListEntries").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509CertificateRevocationListEntry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509CertificateRevocationListEntry {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509CertificateRevocationListEntry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509CertificateRevocationListEntry").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509CertificateTemplate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509CertificateTemplate {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509CertificateTemplate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509CertificateTemplate").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509CertificateTemplateWritable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509CertificateTemplateWritable {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509CertificateTemplateWritable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509CertificateTemplateWritable").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509CertificateTemplates {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509CertificateTemplates {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509CertificateTemplates {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509CertificateTemplates").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509EndorsementKey {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509EndorsementKey {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509EndorsementKey {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509EndorsementKey").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509Enrollment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509Enrollment {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509Enrollment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509Enrollment").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509Enrollment2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509Enrollment2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509Enrollment2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509Enrollment2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IX509Enrollment2 {
    pub unsafe fn Initialize(&self, context: X509CertificateEnrollmentContext) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Initialize)(::windows::core::Vtable::as_raw(self), context).ok()
    }
    pub unsafe fn InitializeFromTemplateName(&self, context: X509CertificateEnrollmentContext, strtemplatename: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.InitializeFromTemplateName)(::windows::core::Vtable::as_raw(self), context, ::core::mem::transmute_copy(strtemplatename)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeFromRequest<P0>(&self, prequest: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IX509CertificateRequest>>,
    {
        (::windows::core::Vtable::vtable(self).base__.InitializeFromRequest)(::windows::core::Vtable::as_raw(self), prequest.into().abi()).ok()
    }
    pub unsafe fn CreateRequest(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateRequest)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Enroll(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Enroll)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn InstallResponse(&self, restrictions: InstallResponseRestrictionFlags, strresponse: &::windows::core::BSTR, encoding: EncodingType, strpassword: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.InstallResponse)(::windows::core::Vtable::as_raw(self), restrictions, ::core::mem::transmute_copy(strresponse), encoding, ::core::mem::transmute_copy(strpassword)).ok()
    }
    pub unsafe fn CreatePFX(&self, strpassword: &::windows::core::BSTR, exportoptions: PFXExportOptions, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreatePFX)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strpassword), exportoptions, encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Request(&self) -> ::windows::core::Result<IX509CertificateRequest> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Request)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Silent(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Silent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSilent<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetSilent)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
    pub unsafe fn ParentWindow(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ParentWindow)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetParentWindow(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetParentWindow)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn NameValuePairs(&self) -> ::windows::core::Result<IX509NameValuePairs> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.NameValuePairs)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnrollmentContext(&self) -> ::windows::core::Result<X509CertificateEnrollmentContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnrollmentContext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Status(&self) -> ::windows::core::Result<IX509EnrollmentStatus> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Status)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn get_Certificate(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_Certificate)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn get_Response(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_Response)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CertificateFriendlyName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CertificateFriendlyName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetCertificateFriendlyName(&self, strvalue: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetCertificateFriendlyName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strvalue)).ok()
    }
    pub unsafe fn CertificateDescription(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CertificateDescription)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetCertificateDescription(&self, strvalue: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetCertificateDescription)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strvalue)).ok()
    }
    pub unsafe fn RequestId(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.RequestId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CAConfigString(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CAConfigString)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509EnrollmentHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509EnrollmentHelper {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509EnrollmentHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509EnrollmentHelper").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509EnrollmentPolicyServer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509EnrollmentPolicyServer {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509EnrollmentPolicyServer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509EnrollmentPolicyServer").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509EnrollmentStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509EnrollmentStatus {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509EnrollmentStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509EnrollmentStatus").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509EnrollmentWebClassFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509EnrollmentWebClassFactory {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509EnrollmentWebClassFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509EnrollmentWebClassFactory").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509Extension {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509Extension {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509Extension {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509Extension").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509ExtensionAlternativeNames {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509ExtensionAlternativeNames {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509ExtensionAlternativeNames {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509ExtensionAlternativeNames").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IX509ExtensionAlternativeNames {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize<P0>(&self, pobjectid: P0, encoding: EncodingType, strencodeddata: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IObjectId>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Initialize)(::windows::core::Vtable::as_raw(self), pobjectid.into().abi(), encoding, ::core::mem::transmute_copy(strencodeddata)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ObjectId(&self) -> ::windows::core::Result<IObjectId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ObjectId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn get_RawData(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_RawData)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Critical(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Critical)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCritical<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetCritical)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509ExtensionAuthorityKeyIdentifier {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509ExtensionAuthorityKeyIdentifier {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509ExtensionAuthorityKeyIdentifier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509ExtensionAuthorityKeyIdentifier").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IX509ExtensionAuthorityKeyIdentifier {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize<P0>(&self, pobjectid: P0, encoding: EncodingType, strencodeddata: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IObjectId>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Initialize)(::windows::core::Vtable::as_raw(self), pobjectid.into().abi(), encoding, ::core::mem::transmute_copy(strencodeddata)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ObjectId(&self) -> ::windows::core::Result<IObjectId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ObjectId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn get_RawData(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_RawData)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Critical(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Critical)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCritical<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetCritical)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509ExtensionBasicConstraints {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509ExtensionBasicConstraints {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509ExtensionBasicConstraints {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509ExtensionBasicConstraints").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IX509ExtensionBasicConstraints {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize<P0>(&self, pobjectid: P0, encoding: EncodingType, strencodeddata: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IObjectId>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Initialize)(::windows::core::Vtable::as_raw(self), pobjectid.into().abi(), encoding, ::core::mem::transmute_copy(strencodeddata)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ObjectId(&self) -> ::windows::core::Result<IObjectId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ObjectId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn get_RawData(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_RawData)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Critical(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Critical)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCritical<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetCritical)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509ExtensionCertificatePolicies {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509ExtensionCertificatePolicies {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509ExtensionCertificatePolicies {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509ExtensionCertificatePolicies").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IX509ExtensionCertificatePolicies {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize<P0>(&self, pobjectid: P0, encoding: EncodingType, strencodeddata: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IObjectId>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Initialize)(::windows::core::Vtable::as_raw(self), pobjectid.into().abi(), encoding, ::core::mem::transmute_copy(strencodeddata)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ObjectId(&self) -> ::windows::core::Result<IObjectId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ObjectId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn get_RawData(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_RawData)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Critical(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Critical)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCritical<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetCritical)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509ExtensionEnhancedKeyUsage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509ExtensionEnhancedKeyUsage {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509ExtensionEnhancedKeyUsage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509ExtensionEnhancedKeyUsage").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IX509ExtensionEnhancedKeyUsage {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize<P0>(&self, pobjectid: P0, encoding: EncodingType, strencodeddata: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IObjectId>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Initialize)(::windows::core::Vtable::as_raw(self), pobjectid.into().abi(), encoding, ::core::mem::transmute_copy(strencodeddata)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ObjectId(&self) -> ::windows::core::Result<IObjectId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ObjectId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn get_RawData(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_RawData)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Critical(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Critical)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCritical<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetCritical)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509ExtensionKeyUsage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509ExtensionKeyUsage {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509ExtensionKeyUsage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509ExtensionKeyUsage").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IX509ExtensionKeyUsage {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize<P0>(&self, pobjectid: P0, encoding: EncodingType, strencodeddata: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IObjectId>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Initialize)(::windows::core::Vtable::as_raw(self), pobjectid.into().abi(), encoding, ::core::mem::transmute_copy(strencodeddata)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ObjectId(&self) -> ::windows::core::Result<IObjectId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ObjectId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn get_RawData(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_RawData)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Critical(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Critical)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCritical<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetCritical)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509ExtensionMSApplicationPolicies {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509ExtensionMSApplicationPolicies {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509ExtensionMSApplicationPolicies {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509ExtensionMSApplicationPolicies").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IX509ExtensionMSApplicationPolicies {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize<P0>(&self, pobjectid: P0, encoding: EncodingType, strencodeddata: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IObjectId>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Initialize)(::windows::core::Vtable::as_raw(self), pobjectid.into().abi(), encoding, ::core::mem::transmute_copy(strencodeddata)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ObjectId(&self) -> ::windows::core::Result<IObjectId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ObjectId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn get_RawData(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_RawData)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Critical(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Critical)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCritical<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetCritical)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509ExtensionSmimeCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509ExtensionSmimeCapabilities {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509ExtensionSmimeCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509ExtensionSmimeCapabilities").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IX509ExtensionSmimeCapabilities {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize<P0>(&self, pobjectid: P0, encoding: EncodingType, strencodeddata: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IObjectId>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Initialize)(::windows::core::Vtable::as_raw(self), pobjectid.into().abi(), encoding, ::core::mem::transmute_copy(strencodeddata)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ObjectId(&self) -> ::windows::core::Result<IObjectId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ObjectId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn get_RawData(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_RawData)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Critical(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Critical)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCritical<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetCritical)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509ExtensionSubjectKeyIdentifier {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509ExtensionSubjectKeyIdentifier {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509ExtensionSubjectKeyIdentifier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509ExtensionSubjectKeyIdentifier").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IX509ExtensionSubjectKeyIdentifier {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize<P0>(&self, pobjectid: P0, encoding: EncodingType, strencodeddata: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IObjectId>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Initialize)(::windows::core::Vtable::as_raw(self), pobjectid.into().abi(), encoding, ::core::mem::transmute_copy(strencodeddata)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ObjectId(&self) -> ::windows::core::Result<IObjectId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ObjectId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn get_RawData(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_RawData)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Critical(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Critical)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCritical<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetCritical)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509ExtensionTemplate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509ExtensionTemplate {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509ExtensionTemplate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509ExtensionTemplate").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IX509ExtensionTemplate {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize<P0>(&self, pobjectid: P0, encoding: EncodingType, strencodeddata: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IObjectId>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Initialize)(::windows::core::Vtable::as_raw(self), pobjectid.into().abi(), encoding, ::core::mem::transmute_copy(strencodeddata)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ObjectId(&self) -> ::windows::core::Result<IObjectId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ObjectId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn get_RawData(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_RawData)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Critical(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Critical)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCritical<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetCritical)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509ExtensionTemplateName {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509ExtensionTemplateName {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509ExtensionTemplateName {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509ExtensionTemplateName").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IX509ExtensionTemplateName {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize<P0>(&self, pobjectid: P0, encoding: EncodingType, strencodeddata: &::windows::core::BSTR) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IObjectId>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Initialize)(::windows::core::Vtable::as_raw(self), pobjectid.into().abi(), encoding, ::core::mem::transmute_copy(strencodeddata)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ObjectId(&self) -> ::windows::core::Result<IObjectId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ObjectId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn get_RawData(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_RawData)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Critical(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Critical)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCritical<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetCritical)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509Extensions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509Extensions {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509Extensions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509Extensions").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509MachineEnrollmentFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509MachineEnrollmentFactory {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509MachineEnrollmentFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509MachineEnrollmentFactory").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509NameValuePair {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509NameValuePair {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509NameValuePair {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509NameValuePair").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509NameValuePairs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509NameValuePairs {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509NameValuePairs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509NameValuePairs").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509PolicyServerListManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509PolicyServerListManager {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509PolicyServerListManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509PolicyServerListManager").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509PolicyServerUrl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509PolicyServerUrl {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509PolicyServerUrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509PolicyServerUrl").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509PrivateKey {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509PrivateKey {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509PrivateKey {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509PrivateKey").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509PrivateKey2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509PrivateKey2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509PrivateKey2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509PrivateKey2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IX509PrivateKey2 {
    pub unsafe fn Open(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Open)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Create(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Create)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Close)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Delete)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Verify(&self, verifytype: X509PrivateKeyVerify) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Verify)(::windows::core::Vtable::as_raw(self), verifytype).ok()
    }
    pub unsafe fn Import(&self, strexporttype: &::windows::core::BSTR, strencodedkey: &::windows::core::BSTR, encoding: EncodingType) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Import)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strexporttype), ::core::mem::transmute_copy(strencodedkey), encoding).ok()
    }
    pub unsafe fn Export(&self, strexporttype: &::windows::core::BSTR, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Export)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strexporttype), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExportPublicKey(&self) -> ::windows::core::Result<IX509PublicKey> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ExportPublicKey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ContainerName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ContainerName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetContainerName(&self, value: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetContainerName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(value)).ok()
    }
    pub unsafe fn ContainerNamePrefix(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ContainerNamePrefix)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetContainerNamePrefix(&self, value: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetContainerNamePrefix)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(value)).ok()
    }
    pub unsafe fn ReaderName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ReaderName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetReaderName(&self, value: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetReaderName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CspInformations(&self) -> ::windows::core::Result<ICspInformations> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CspInformations)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetCspInformations<P0>(&self, pvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ICspInformations>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetCspInformations)(::windows::core::Vtable::as_raw(self), pvalue.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CspStatus(&self) -> ::windows::core::Result<ICspStatus> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CspStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetCspStatus<P0>(&self, pvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ICspStatus>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetCspStatus)(::windows::core::Vtable::as_raw(self), pvalue.into().abi()).ok()
    }
    pub unsafe fn ProviderName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ProviderName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetProviderName(&self, value: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetProviderName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(value)).ok()
    }
    pub unsafe fn ProviderType(&self) -> ::windows::core::Result<X509ProviderType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ProviderType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetProviderType(&self, value: X509ProviderType) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetProviderType)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn LegacyCsp(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.LegacyCsp)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLegacyCsp<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetLegacyCsp)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Algorithm(&self) -> ::windows::core::Result<IObjectId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Algorithm)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetAlgorithm<P0>(&self, pvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IObjectId>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetAlgorithm)(::windows::core::Vtable::as_raw(self), pvalue.into().abi()).ok()
    }
    pub unsafe fn KeySpec(&self) -> ::windows::core::Result<X509KeySpec> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.KeySpec)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetKeySpec(&self, value: X509KeySpec) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetKeySpec)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn Length(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Length)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetLength(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetLength)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn ExportPolicy(&self) -> ::windows::core::Result<X509PrivateKeyExportFlags> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ExportPolicy)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetExportPolicy(&self, value: X509PrivateKeyExportFlags) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetExportPolicy)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn KeyUsage(&self) -> ::windows::core::Result<X509PrivateKeyUsageFlags> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.KeyUsage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetKeyUsage(&self, value: X509PrivateKeyUsageFlags) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetKeyUsage)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn KeyProtection(&self) -> ::windows::core::Result<X509PrivateKeyProtection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.KeyProtection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetKeyProtection(&self, value: X509PrivateKeyProtection) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetKeyProtection)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MachineContext(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.MachineContext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMachineContext<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetMachineContext)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
    pub unsafe fn SecurityDescriptor(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SecurityDescriptor)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetSecurityDescriptor(&self, value: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetSecurityDescriptor)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(value)).ok()
    }
    pub unsafe fn get_Certificate(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_Certificate)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn put_Certificate(&self, encoding: EncodingType, value: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.put_Certificate)(::windows::core::Vtable::as_raw(self), encoding, ::core::mem::transmute_copy(value)).ok()
    }
    pub unsafe fn UniqueContainerName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.UniqueContainerName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Opened(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Opened)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DefaultContainer(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DefaultContainer)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Existing(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Existing)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetExisting<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetExisting)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Silent(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Silent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSilent<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetSilent)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
    pub unsafe fn ParentWindow(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ParentWindow)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetParentWindow(&self, value: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetParentWindow)(::windows::core::Vtable::as_raw(self), value).ok()
    }
    pub unsafe fn UIContextMessage(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.UIContextMessage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetUIContextMessage(&self, value: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetUIContextMessage)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(value)).ok()
    }
    pub unsafe fn SetPin(&self, value: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPin)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(value)).ok()
    }
    pub unsafe fn FriendlyName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.FriendlyName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetFriendlyName(&self, value: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetFriendlyName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(value)).ok()
    }
    pub unsafe fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Description)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDescription(&self, value: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetDescription)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(value)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509PublicKey {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509PublicKey {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509PublicKey {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509PublicKey").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509SCEPEnrollment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509SCEPEnrollment {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509SCEPEnrollment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509SCEPEnrollment").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509SCEPEnrollment2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509SCEPEnrollment2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509SCEPEnrollment2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509SCEPEnrollment2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IX509SCEPEnrollment2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize<P0>(&self, prequest: P0, strthumbprint: &::windows::core::BSTR, thumprintencoding: EncodingType, strservercertificates: &::windows::core::BSTR, encoding: EncodingType) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IX509CertificateRequestPkcs10>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Initialize)(::windows::core::Vtable::as_raw(self), prequest.into().abi(), ::core::mem::transmute_copy(strthumbprint), thumprintencoding, ::core::mem::transmute_copy(strservercertificates), encoding).ok()
    }
    pub unsafe fn InitializeForPending(&self, context: X509CertificateEnrollmentContext) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.InitializeForPending)(::windows::core::Vtable::as_raw(self), context).ok()
    }
    pub unsafe fn CreateRequestMessage(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateRequestMessage)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateRetrievePendingMessage(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateRetrievePendingMessage)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateRetrieveCertificateMessage(&self, context: X509CertificateEnrollmentContext, strissuer: &::windows::core::BSTR, issuerencoding: EncodingType, strserialnumber: &::windows::core::BSTR, serialnumberencoding: EncodingType, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateRetrieveCertificateMessage)(::windows::core::Vtable::as_raw(self), context, ::core::mem::transmute_copy(strissuer), issuerencoding, ::core::mem::transmute_copy(strserialnumber), serialnumberencoding, encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ProcessResponseMessage(&self, strresponse: &::windows::core::BSTR, encoding: EncodingType) -> ::windows::core::Result<X509SCEPDisposition> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ProcessResponseMessage)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strresponse), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetServerCapabilities(&self, value: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetServerCapabilities)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(value)).ok()
    }
    pub unsafe fn FailInfo(&self) -> ::windows::core::Result<X509SCEPFailInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.FailInfo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SignerCertificate(&self) -> ::windows::core::Result<ISignerCertificate> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SignerCertificate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSignerCertificate<P0>(&self, pvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ISignerCertificate>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetSignerCertificate)(::windows::core::Vtable::as_raw(self), pvalue.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OldCertificate(&self) -> ::windows::core::Result<ISignerCertificate> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.OldCertificate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetOldCertificate<P0>(&self, pvalue: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ISignerCertificate>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetOldCertificate)(::windows::core::Vtable::as_raw(self), pvalue.into().abi()).ok()
    }
    pub unsafe fn get_TransactionId(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_TransactionId)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn put_TransactionId(&self, encoding: EncodingType, value: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.put_TransactionId)(::windows::core::Vtable::as_raw(self), encoding, ::core::mem::transmute_copy(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Request(&self) -> ::windows::core::Result<IX509CertificateRequestPkcs10> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Request)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CertificateFriendlyName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CertificateFriendlyName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetCertificateFriendlyName(&self, value: &::windows::core::BSTR) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetCertificateFriendlyName)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Status(&self) -> ::windows::core::Result<IX509EnrollmentStatus> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Status)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn get_Certificate(&self, encoding: EncodingType) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.get_Certificate)(::windows::core::Vtable::as_raw(self), encoding, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Silent(&self) -> ::windows::core::Result<super::super::super::Foundation::VARIANT_BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Silent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSilent<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::VARIANT_BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetSilent)(::windows::core::Vtable::as_raw(self), value.into()).ok()
    }
    pub unsafe fn DeleteRequest(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DeleteRequest)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509SCEPEnrollmentHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509SCEPEnrollmentHelper {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509SCEPEnrollmentHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509SCEPEnrollmentHelper").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IX509SignatureInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IX509SignatureInformation {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IX509SignatureInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IX509SignatureInformation").field(&self.0).finish()
    }
}
impl ::core::default::Default for ImportPFXFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ImportPFXFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ImportPFXFlags").field(&self.0).finish()
    }
}
impl ::core::default::Default for InnerRequestLevel {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for InnerRequestLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InnerRequestLevel").field(&self.0).finish()
    }
}
impl ::core::default::Default for InstallResponseRestrictionFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for InstallResponseRestrictionFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InstallResponseRestrictionFlags").field(&self.0).finish()
    }
}
impl ::core::default::Default for KeyAttestationClaimType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KeyAttestationClaimType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyAttestationClaimType").field(&self.0).finish()
    }
}
impl ::core::default::Default for KeyIdentifierHashAlgorithm {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for KeyIdentifierHashAlgorithm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KeyIdentifierHashAlgorithm").field(&self.0).finish()
    }
}
impl ::core::default::Default for OCSPRequestFlag {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OCSPRequestFlag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OCSPRequestFlag").field(&self.0).finish()
    }
}
impl ::core::default::Default for OCSPSigningFlag {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for OCSPSigningFlag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OCSPSigningFlag").field(&self.0).finish()
    }
}
impl ::core::default::Default for ObjectIdGroupId {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ObjectIdGroupId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ObjectIdGroupId").field(&self.0).finish()
    }
}
impl ::core::default::Default for ObjectIdPublicKeyFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ObjectIdPublicKeyFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ObjectIdPublicKeyFlags").field(&self.0).finish()
    }
}
impl ::core::default::Default for PENDING_REQUEST_DESIRED_PROPERTY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PENDING_REQUEST_DESIRED_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PENDING_REQUEST_DESIRED_PROPERTY").field(&self.0).finish()
    }
}
impl ::core::default::Default for PFXExportOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PFXExportOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PFXExportOptions").field(&self.0).finish()
    }
}
impl ::core::default::Default for Pkcs10AllowedSignatureTypes {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for Pkcs10AllowedSignatureTypes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Pkcs10AllowedSignatureTypes").field(&self.0).finish()
    }
}
impl ::core::default::Default for PolicyQualifierType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PolicyQualifierType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PolicyQualifierType").field(&self.0).finish()
    }
}
impl ::core::default::Default for PolicyServerUrlFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PolicyServerUrlFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PolicyServerUrlFlags").field(&self.0).finish()
    }
}
impl ::core::default::Default for PolicyServerUrlPropertyID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PolicyServerUrlPropertyID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PolicyServerUrlPropertyID").field(&self.0).finish()
    }
}
impl ::core::default::Default for RequestClientInfoClientId {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RequestClientInfoClientId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RequestClientInfoClientId").field(&self.0).finish()
    }
}
impl ::core::default::Default for WebEnrollmentFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WebEnrollmentFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebEnrollmentFlags").field(&self.0).finish()
    }
}
impl ::core::default::Default for WebSecurityLevel {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WebSecurityLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebSecurityLevel").field(&self.0).finish()
    }
}
impl ::core::default::Default for X500NameFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for X500NameFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("X500NameFlags").field(&self.0).finish()
    }
}
impl ::core::default::Default for X509CertificateEnrollmentContext {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for X509CertificateEnrollmentContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("X509CertificateEnrollmentContext").field(&self.0).finish()
    }
}
impl ::core::default::Default for X509CertificateTemplateEnrollmentFlag {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for X509CertificateTemplateEnrollmentFlag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("X509CertificateTemplateEnrollmentFlag").field(&self.0).finish()
    }
}
impl ::core::default::Default for X509CertificateTemplateGeneralFlag {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for X509CertificateTemplateGeneralFlag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("X509CertificateTemplateGeneralFlag").field(&self.0).finish()
    }
}
impl ::core::default::Default for X509CertificateTemplatePrivateKeyFlag {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for X509CertificateTemplatePrivateKeyFlag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("X509CertificateTemplatePrivateKeyFlag").field(&self.0).finish()
    }
}
impl ::core::default::Default for X509CertificateTemplateSubjectNameFlag {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for X509CertificateTemplateSubjectNameFlag {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("X509CertificateTemplateSubjectNameFlag").field(&self.0).finish()
    }
}
impl ::core::default::Default for X509EnrollmentAuthFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for X509EnrollmentAuthFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("X509EnrollmentAuthFlags").field(&self.0).finish()
    }
}
impl ::core::default::Default for X509EnrollmentPolicyExportFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for X509EnrollmentPolicyExportFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("X509EnrollmentPolicyExportFlags").field(&self.0).finish()
    }
}
impl ::core::default::Default for X509EnrollmentPolicyLoadOption {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for X509EnrollmentPolicyLoadOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("X509EnrollmentPolicyLoadOption").field(&self.0).finish()
    }
}
impl ::core::default::Default for X509HardwareKeyUsageFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for X509HardwareKeyUsageFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("X509HardwareKeyUsageFlags").field(&self.0).finish()
    }
}
impl ::core::default::Default for X509KeyParametersExportType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for X509KeyParametersExportType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("X509KeyParametersExportType").field(&self.0).finish()
    }
}
impl ::core::default::Default for X509KeySpec {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for X509KeySpec {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("X509KeySpec").field(&self.0).finish()
    }
}
impl ::core::default::Default for X509KeyUsageFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for X509KeyUsageFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("X509KeyUsageFlags").field(&self.0).finish()
    }
}
impl ::core::default::Default for X509PrivateKeyExportFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for X509PrivateKeyExportFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("X509PrivateKeyExportFlags").field(&self.0).finish()
    }
}
impl ::core::default::Default for X509PrivateKeyProtection {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for X509PrivateKeyProtection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("X509PrivateKeyProtection").field(&self.0).finish()
    }
}
impl ::core::default::Default for X509PrivateKeyUsageFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for X509PrivateKeyUsageFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("X509PrivateKeyUsageFlags").field(&self.0).finish()
    }
}
impl ::core::default::Default for X509PrivateKeyVerify {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for X509PrivateKeyVerify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("X509PrivateKeyVerify").field(&self.0).finish()
    }
}
impl ::core::default::Default for X509ProviderType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for X509ProviderType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("X509ProviderType").field(&self.0).finish()
    }
}
impl ::core::default::Default for X509RequestInheritOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for X509RequestInheritOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("X509RequestInheritOptions").field(&self.0).finish()
    }
}
impl ::core::default::Default for X509RequestType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for X509RequestType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("X509RequestType").field(&self.0).finish()
    }
}
impl ::core::default::Default for X509SCEPDisposition {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for X509SCEPDisposition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("X509SCEPDisposition").field(&self.0).finish()
    }
}
impl ::core::default::Default for X509SCEPFailInfo {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for X509SCEPFailInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("X509SCEPFailInfo").field(&self.0).finish()
    }
}
impl ::core::default::Default for X509SCEPMessageType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for X509SCEPMessageType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("X509SCEPMessageType").field(&self.0).finish()
    }
}
impl ::core::default::Default for X509SCEPProcessMessageFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for X509SCEPProcessMessageFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("X509SCEPProcessMessageFlags").field(&self.0).finish()
    }
}
impl ::core::default::Default for XEKL_KEYSIZE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for XEKL_KEYSIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XEKL_KEYSIZE").field(&self.0).finish()
    }
}
impl ::core::default::Default for XEKL_KEYSPEC {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for XEKL_KEYSPEC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XEKL_KEYSPEC").field(&self.0).finish()
    }
}

impl ::core::default::Default for CF_CALLBACK_CANCEL_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CF_CALLBACK_CANCEL_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_CALLBACK_CANCEL_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_CALLBACK_CANCEL_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_CALLBACK_CANCEL_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_CALLBACK_CANCEL_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_CALLBACK_CANCEL_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_CALLBACK_CANCEL_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CF_CALLBACK_CLOSE_COMPLETION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CF_CALLBACK_CLOSE_COMPLETION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_CALLBACK_CLOSE_COMPLETION_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_CALLBACK_CLOSE_COMPLETION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_CALLBACK_CLOSE_COMPLETION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_CALLBACK_CLOSE_COMPLETION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_CALLBACK_CLOSE_COMPLETION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_CALLBACK_CLOSE_COMPLETION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CF_CALLBACK_DEHYDRATE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CF_CALLBACK_DEHYDRATE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_CALLBACK_DEHYDRATE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_CALLBACK_DEHYDRATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_CALLBACK_DEHYDRATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_CALLBACK_DEHYDRATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_CALLBACK_DEHYDRATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_CALLBACK_DEHYDRATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CF_CALLBACK_DEHYDRATION_REASON {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CF_CALLBACK_DEHYDRATION_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_CALLBACK_DEHYDRATION_REASON").field(&self.0).finish()
    }
}
impl ::core::default::Default for CF_CALLBACK_DELETE_COMPLETION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CF_CALLBACK_DELETE_COMPLETION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_CALLBACK_DELETE_COMPLETION_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_CALLBACK_DELETE_COMPLETION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_CALLBACK_DELETE_COMPLETION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_CALLBACK_DELETE_COMPLETION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_CALLBACK_DELETE_COMPLETION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_CALLBACK_DELETE_COMPLETION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CF_CALLBACK_DELETE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CF_CALLBACK_DELETE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_CALLBACK_DELETE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_CALLBACK_DELETE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_CALLBACK_DELETE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_CALLBACK_DELETE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_CALLBACK_DELETE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_CALLBACK_DELETE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CF_CALLBACK_FETCH_DATA_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CF_CALLBACK_FETCH_DATA_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_CALLBACK_FETCH_DATA_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_CALLBACK_FETCH_DATA_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_CALLBACK_FETCH_DATA_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_CALLBACK_FETCH_DATA_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_CALLBACK_FETCH_DATA_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_CALLBACK_FETCH_DATA_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CF_CALLBACK_FETCH_PLACEHOLDERS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CF_CALLBACK_FETCH_PLACEHOLDERS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_CALLBACK_FETCH_PLACEHOLDERS_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_CALLBACK_FETCH_PLACEHOLDERS_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_CALLBACK_FETCH_PLACEHOLDERS_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_CALLBACK_FETCH_PLACEHOLDERS_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_CALLBACK_FETCH_PLACEHOLDERS_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_CALLBACK_FETCH_PLACEHOLDERS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_CorrelationVector"))]
impl ::core::default::Default for CF_CALLBACK_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_CorrelationVector"))]
impl ::core::cmp::PartialEq for CF_CALLBACK_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.StructSize == other.StructSize
            && self.ConnectionKey == other.ConnectionKey
            && self.CallbackContext == other.CallbackContext
            && self.VolumeGuidName == other.VolumeGuidName
            && self.VolumeDosName == other.VolumeDosName
            && self.VolumeSerialNumber == other.VolumeSerialNumber
            && self.SyncRootFileId == other.SyncRootFileId
            && self.SyncRootIdentity == other.SyncRootIdentity
            && self.SyncRootIdentityLength == other.SyncRootIdentityLength
            && self.FileId == other.FileId
            && self.FileSize == other.FileSize
            && self.FileIdentity == other.FileIdentity
            && self.FileIdentityLength == other.FileIdentityLength
            && self.NormalizedPath == other.NormalizedPath
            && self.TransferKey == other.TransferKey
            && self.PriorityHint == other.PriorityHint
            && self.CorrelationVector == other.CorrelationVector
            && self.ProcessInfo == other.ProcessInfo
            && self.RequestKey == other.RequestKey
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_CorrelationVector"))]
impl ::core::cmp::Eq for CF_CALLBACK_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_CorrelationVector"))]
impl ::core::fmt::Debug for CF_CALLBACK_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_CALLBACK_INFO")
            .field("StructSize", &self.StructSize)
            .field("ConnectionKey", &self.ConnectionKey)
            .field("CallbackContext", &self.CallbackContext)
            .field("VolumeGuidName", &self.VolumeGuidName)
            .field("VolumeDosName", &self.VolumeDosName)
            .field("VolumeSerialNumber", &self.VolumeSerialNumber)
            .field("SyncRootFileId", &self.SyncRootFileId)
            .field("SyncRootIdentity", &self.SyncRootIdentity)
            .field("SyncRootIdentityLength", &self.SyncRootIdentityLength)
            .field("FileId", &self.FileId)
            .field("FileSize", &self.FileSize)
            .field("FileIdentity", &self.FileIdentity)
            .field("FileIdentityLength", &self.FileIdentityLength)
            .field("NormalizedPath", &self.NormalizedPath)
            .field("TransferKey", &self.TransferKey)
            .field("PriorityHint", &self.PriorityHint)
            .field("CorrelationVector", &self.CorrelationVector)
            .field("ProcessInfo", &self.ProcessInfo)
            .field("RequestKey", &self.RequestKey)
            .finish()
    }
}
impl ::core::default::Default for CF_CALLBACK_OPEN_COMPLETION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CF_CALLBACK_OPEN_COMPLETION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_CALLBACK_OPEN_COMPLETION_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_CALLBACK_OPEN_COMPLETION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_CALLBACK_OPEN_COMPLETION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_CALLBACK_OPEN_COMPLETION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_CALLBACK_OPEN_COMPLETION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_CALLBACK_OPEN_COMPLETION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CF_CALLBACK_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_CorrelationVector"))]
impl ::core::default::Default for CF_CALLBACK_REGISTRATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CF_CALLBACK_RENAME_COMPLETION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CF_CALLBACK_RENAME_COMPLETION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_CALLBACK_RENAME_COMPLETION_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_CALLBACK_RENAME_COMPLETION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_CALLBACK_RENAME_COMPLETION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_CALLBACK_RENAME_COMPLETION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_CALLBACK_RENAME_COMPLETION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_CALLBACK_RENAME_COMPLETION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CF_CALLBACK_RENAME_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CF_CALLBACK_RENAME_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_CALLBACK_RENAME_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_CALLBACK_RENAME_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_CALLBACK_RENAME_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_CALLBACK_RENAME_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_CALLBACK_RENAME_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_CALLBACK_RENAME_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CF_CALLBACK_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CF_CALLBACK_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_CALLBACK_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CF_CALLBACK_VALIDATE_DATA_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CF_CALLBACK_VALIDATE_DATA_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_CALLBACK_VALIDATE_DATA_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_CALLBACK_VALIDATE_DATA_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_CALLBACK_VALIDATE_DATA_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_CALLBACK_VALIDATE_DATA_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_CALLBACK_VALIDATE_DATA_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_CALLBACK_VALIDATE_DATA_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CF_CONNECT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CF_CONNECT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_CONNECT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_CONNECT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_CONNECT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_CONNECT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_CONNECT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_CONNECT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CF_CONVERT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CF_CONVERT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_CONVERT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_CONVERT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_CONVERT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_CONVERT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_CONVERT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_CONVERT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CF_CREATE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CF_CREATE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_CREATE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_CREATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_CREATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_CREATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_CREATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_CREATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CF_DEHYDRATE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CF_DEHYDRATE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_DEHYDRATE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_DEHYDRATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_DEHYDRATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_DEHYDRATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_DEHYDRATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_DEHYDRATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CF_FILE_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CF_FILE_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.StartingOffset == other.StartingOffset && self.Length == other.Length
    }
}
impl ::core::cmp::Eq for CF_FILE_RANGE {}
impl ::core::fmt::Debug for CF_FILE_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_FILE_RANGE").field("StartingOffset", &self.StartingOffset).field("Length", &self.Length).finish()
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::default::Default for CF_FS_METADATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::cmp::PartialEq for CF_FS_METADATA {
    fn eq(&self, other: &Self) -> bool {
        self.BasicInfo == other.BasicInfo && self.FileSize == other.FileSize
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::cmp::Eq for CF_FS_METADATA {}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::fmt::Debug for CF_FS_METADATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_FS_METADATA").field("BasicInfo", &self.BasicInfo).field("FileSize", &self.FileSize).finish()
    }
}
impl ::core::default::Default for CF_HARDLINK_POLICY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CF_HARDLINK_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_HARDLINK_POLICY").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_HARDLINK_POLICY {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_HARDLINK_POLICY {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_HARDLINK_POLICY {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_HARDLINK_POLICY {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_HARDLINK_POLICY {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CF_HYDRATE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CF_HYDRATE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_HYDRATE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_HYDRATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_HYDRATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_HYDRATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_HYDRATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_HYDRATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CF_HYDRATION_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CF_HYDRATION_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.Primary == other.Primary && self.Modifier == other.Modifier
    }
}
impl ::core::cmp::Eq for CF_HYDRATION_POLICY {}
impl ::core::fmt::Debug for CF_HYDRATION_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_HYDRATION_POLICY").field("Primary", &self.Primary).field("Modifier", &self.Modifier).finish()
    }
}
impl ::core::default::Default for CF_HYDRATION_POLICY_MODIFIER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CF_HYDRATION_POLICY_MODIFIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_HYDRATION_POLICY_MODIFIER").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_HYDRATION_POLICY_MODIFIER {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_HYDRATION_POLICY_MODIFIER {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_HYDRATION_POLICY_MODIFIER {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_HYDRATION_POLICY_MODIFIER {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_HYDRATION_POLICY_MODIFIER {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CF_HYDRATION_POLICY_MODIFIER_USHORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CF_HYDRATION_POLICY_MODIFIER_USHORT {
    fn eq(&self, other: &Self) -> bool {
        self.us == other.us
    }
}
impl ::core::cmp::Eq for CF_HYDRATION_POLICY_MODIFIER_USHORT {}
impl ::core::fmt::Debug for CF_HYDRATION_POLICY_MODIFIER_USHORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_HYDRATION_POLICY_MODIFIER_USHORT").field("us", &self.us).finish()
    }
}
impl ::core::default::Default for CF_HYDRATION_POLICY_PRIMARY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CF_HYDRATION_POLICY_PRIMARY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_HYDRATION_POLICY_PRIMARY").field(&self.0).finish()
    }
}
impl ::core::default::Default for CF_HYDRATION_POLICY_PRIMARY_USHORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CF_HYDRATION_POLICY_PRIMARY_USHORT {
    fn eq(&self, other: &Self) -> bool {
        self.us == other.us
    }
}
impl ::core::cmp::Eq for CF_HYDRATION_POLICY_PRIMARY_USHORT {}
impl ::core::fmt::Debug for CF_HYDRATION_POLICY_PRIMARY_USHORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_HYDRATION_POLICY_PRIMARY_USHORT").field("us", &self.us).finish()
    }
}
impl ::core::default::Default for CF_INSYNC_POLICY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CF_INSYNC_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_INSYNC_POLICY").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_INSYNC_POLICY {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_INSYNC_POLICY {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_INSYNC_POLICY {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_INSYNC_POLICY {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_INSYNC_POLICY {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CF_IN_SYNC_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CF_IN_SYNC_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_IN_SYNC_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CF_OPEN_FILE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CF_OPEN_FILE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_OPEN_FILE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_OPEN_FILE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_OPEN_FILE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_OPEN_FILE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_OPEN_FILE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_OPEN_FILE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CF_OPERATION_ACK_DATA_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CF_OPERATION_ACK_DATA_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_OPERATION_ACK_DATA_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_OPERATION_ACK_DATA_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_OPERATION_ACK_DATA_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_OPERATION_ACK_DATA_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_OPERATION_ACK_DATA_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_OPERATION_ACK_DATA_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CF_OPERATION_ACK_DEHYDRATE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CF_OPERATION_ACK_DEHYDRATE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_OPERATION_ACK_DEHYDRATE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_OPERATION_ACK_DEHYDRATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_OPERATION_ACK_DEHYDRATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_OPERATION_ACK_DEHYDRATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_OPERATION_ACK_DEHYDRATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_OPERATION_ACK_DEHYDRATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CF_OPERATION_ACK_DELETE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CF_OPERATION_ACK_DELETE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_OPERATION_ACK_DELETE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_OPERATION_ACK_DELETE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_OPERATION_ACK_DELETE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_OPERATION_ACK_DELETE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_OPERATION_ACK_DELETE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_OPERATION_ACK_DELETE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CF_OPERATION_ACK_RENAME_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CF_OPERATION_ACK_RENAME_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_OPERATION_ACK_RENAME_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_OPERATION_ACK_RENAME_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_OPERATION_ACK_RENAME_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_OPERATION_ACK_RENAME_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_OPERATION_ACK_RENAME_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_OPERATION_ACK_RENAME_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_CorrelationVector"))]
impl ::core::default::Default for CF_OPERATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_CorrelationVector"))]
impl ::core::cmp::PartialEq for CF_OPERATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.StructSize == other.StructSize && self.Type == other.Type && self.ConnectionKey == other.ConnectionKey && self.TransferKey == other.TransferKey && self.CorrelationVector == other.CorrelationVector && self.SyncStatus == other.SyncStatus && self.RequestKey == other.RequestKey
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_CorrelationVector"))]
impl ::core::cmp::Eq for CF_OPERATION_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_CorrelationVector"))]
impl ::core::fmt::Debug for CF_OPERATION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_OPERATION_INFO").field("StructSize", &self.StructSize).field("Type", &self.Type).field("ConnectionKey", &self.ConnectionKey).field("TransferKey", &self.TransferKey).field("CorrelationVector", &self.CorrelationVector).field("SyncStatus", &self.SyncStatus).field("RequestKey", &self.RequestKey).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::default::Default for CF_OPERATION_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CF_OPERATION_RESTART_HYDRATION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CF_OPERATION_RESTART_HYDRATION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_OPERATION_RESTART_HYDRATION_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_OPERATION_RESTART_HYDRATION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_OPERATION_RESTART_HYDRATION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_OPERATION_RESTART_HYDRATION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_OPERATION_RESTART_HYDRATION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_OPERATION_RESTART_HYDRATION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CF_OPERATION_RETRIEVE_DATA_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CF_OPERATION_RETRIEVE_DATA_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_OPERATION_RETRIEVE_DATA_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_OPERATION_RETRIEVE_DATA_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_OPERATION_RETRIEVE_DATA_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_OPERATION_RETRIEVE_DATA_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_OPERATION_RETRIEVE_DATA_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_OPERATION_RETRIEVE_DATA_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CF_OPERATION_TRANSFER_DATA_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CF_OPERATION_TRANSFER_DATA_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_OPERATION_TRANSFER_DATA_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_OPERATION_TRANSFER_DATA_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_OPERATION_TRANSFER_DATA_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_OPERATION_TRANSFER_DATA_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_OPERATION_TRANSFER_DATA_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_OPERATION_TRANSFER_DATA_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CF_OPERATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CF_OPERATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_OPERATION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CF_PIN_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CF_PIN_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_PIN_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CF_PLACEHOLDER_BASIC_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CF_PLACEHOLDER_BASIC_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.PinState == other.PinState && self.InSyncState == other.InSyncState && self.FileId == other.FileId && self.SyncRootFileId == other.SyncRootFileId && self.FileIdentityLength == other.FileIdentityLength && self.FileIdentity == other.FileIdentity
    }
}
impl ::core::cmp::Eq for CF_PLACEHOLDER_BASIC_INFO {}
impl ::core::fmt::Debug for CF_PLACEHOLDER_BASIC_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_PLACEHOLDER_BASIC_INFO").field("PinState", &self.PinState).field("InSyncState", &self.InSyncState).field("FileId", &self.FileId).field("SyncRootFileId", &self.SyncRootFileId).field("FileIdentityLength", &self.FileIdentityLength).field("FileIdentity", &self.FileIdentity).finish()
    }
}
impl ::core::default::Default for CF_PLACEHOLDER_CREATE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CF_PLACEHOLDER_CREATE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_PLACEHOLDER_CREATE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_PLACEHOLDER_CREATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_PLACEHOLDER_CREATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_PLACEHOLDER_CREATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_PLACEHOLDER_CREATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_PLACEHOLDER_CREATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::default::Default for CF_PLACEHOLDER_CREATE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::cmp::PartialEq for CF_PLACEHOLDER_CREATE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.RelativeFileName == other.RelativeFileName && self.FsMetadata == other.FsMetadata && self.FileIdentity == other.FileIdentity && self.FileIdentityLength == other.FileIdentityLength && self.Flags == other.Flags && self.Result == other.Result && self.CreateUsn == other.CreateUsn
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::cmp::Eq for CF_PLACEHOLDER_CREATE_INFO {}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::fmt::Debug for CF_PLACEHOLDER_CREATE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_PLACEHOLDER_CREATE_INFO").field("RelativeFileName", &self.RelativeFileName).field("FsMetadata", &self.FsMetadata).field("FileIdentity", &self.FileIdentity).field("FileIdentityLength", &self.FileIdentityLength).field("Flags", &self.Flags).field("Result", &self.Result).field("CreateUsn", &self.CreateUsn).finish()
    }
}
impl ::core::default::Default for CF_PLACEHOLDER_INFO_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CF_PLACEHOLDER_INFO_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_PLACEHOLDER_INFO_CLASS").field(&self.0).finish()
    }
}
impl ::core::default::Default for CF_PLACEHOLDER_MANAGEMENT_POLICY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CF_PLACEHOLDER_MANAGEMENT_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_PLACEHOLDER_MANAGEMENT_POLICY").field(&self.0).finish()
    }
}
impl ::core::default::Default for CF_PLACEHOLDER_RANGE_INFO_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CF_PLACEHOLDER_RANGE_INFO_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_PLACEHOLDER_RANGE_INFO_CLASS").field(&self.0).finish()
    }
}
impl ::core::default::Default for CF_PLACEHOLDER_STANDARD_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CF_PLACEHOLDER_STANDARD_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.OnDiskDataSize == other.OnDiskDataSize && self.ValidatedDataSize == other.ValidatedDataSize && self.ModifiedDataSize == other.ModifiedDataSize && self.PropertiesSize == other.PropertiesSize && self.PinState == other.PinState && self.InSyncState == other.InSyncState && self.FileId == other.FileId && self.SyncRootFileId == other.SyncRootFileId && self.FileIdentityLength == other.FileIdentityLength && self.FileIdentity == other.FileIdentity
    }
}
impl ::core::cmp::Eq for CF_PLACEHOLDER_STANDARD_INFO {}
impl ::core::fmt::Debug for CF_PLACEHOLDER_STANDARD_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_PLACEHOLDER_STANDARD_INFO")
            .field("OnDiskDataSize", &self.OnDiskDataSize)
            .field("ValidatedDataSize", &self.ValidatedDataSize)
            .field("ModifiedDataSize", &self.ModifiedDataSize)
            .field("PropertiesSize", &self.PropertiesSize)
            .field("PinState", &self.PinState)
            .field("InSyncState", &self.InSyncState)
            .field("FileId", &self.FileId)
            .field("SyncRootFileId", &self.SyncRootFileId)
            .field("FileIdentityLength", &self.FileIdentityLength)
            .field("FileIdentity", &self.FileIdentity)
            .finish()
    }
}
impl ::core::default::Default for CF_PLACEHOLDER_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CF_PLACEHOLDER_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_PLACEHOLDER_STATE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_PLACEHOLDER_STATE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_PLACEHOLDER_STATE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_PLACEHOLDER_STATE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_PLACEHOLDER_STATE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_PLACEHOLDER_STATE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CF_PLATFORM_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CF_PLATFORM_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.BuildNumber == other.BuildNumber && self.RevisionNumber == other.RevisionNumber && self.IntegrationNumber == other.IntegrationNumber
    }
}
impl ::core::cmp::Eq for CF_PLATFORM_INFO {}
impl ::core::fmt::Debug for CF_PLATFORM_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_PLATFORM_INFO").field("BuildNumber", &self.BuildNumber).field("RevisionNumber", &self.RevisionNumber).field("IntegrationNumber", &self.IntegrationNumber).finish()
    }
}
impl ::core::default::Default for CF_POPULATION_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CF_POPULATION_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.Primary == other.Primary && self.Modifier == other.Modifier
    }
}
impl ::core::cmp::Eq for CF_POPULATION_POLICY {}
impl ::core::fmt::Debug for CF_POPULATION_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_POPULATION_POLICY").field("Primary", &self.Primary).field("Modifier", &self.Modifier).finish()
    }
}
impl ::core::default::Default for CF_POPULATION_POLICY_MODIFIER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CF_POPULATION_POLICY_MODIFIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_POPULATION_POLICY_MODIFIER").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_POPULATION_POLICY_MODIFIER {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_POPULATION_POLICY_MODIFIER {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_POPULATION_POLICY_MODIFIER {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_POPULATION_POLICY_MODIFIER {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_POPULATION_POLICY_MODIFIER {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CF_POPULATION_POLICY_MODIFIER_USHORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CF_POPULATION_POLICY_MODIFIER_USHORT {
    fn eq(&self, other: &Self) -> bool {
        self.us == other.us
    }
}
impl ::core::cmp::Eq for CF_POPULATION_POLICY_MODIFIER_USHORT {}
impl ::core::fmt::Debug for CF_POPULATION_POLICY_MODIFIER_USHORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_POPULATION_POLICY_MODIFIER_USHORT").field("us", &self.us).finish()
    }
}
impl ::core::default::Default for CF_POPULATION_POLICY_PRIMARY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CF_POPULATION_POLICY_PRIMARY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_POPULATION_POLICY_PRIMARY").field(&self.0).finish()
    }
}
impl ::core::default::Default for CF_POPULATION_POLICY_PRIMARY_USHORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CF_POPULATION_POLICY_PRIMARY_USHORT {
    fn eq(&self, other: &Self) -> bool {
        self.us == other.us
    }
}
impl ::core::cmp::Eq for CF_POPULATION_POLICY_PRIMARY_USHORT {}
impl ::core::fmt::Debug for CF_POPULATION_POLICY_PRIMARY_USHORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_POPULATION_POLICY_PRIMARY_USHORT").field("us", &self.us).finish()
    }
}
impl ::core::default::Default for CF_PROCESS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CF_PROCESS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.StructSize == other.StructSize && self.ProcessId == other.ProcessId && self.ImagePath == other.ImagePath && self.PackageName == other.PackageName && self.ApplicationId == other.ApplicationId && self.CommandLine == other.CommandLine && self.SessionId == other.SessionId
    }
}
impl ::core::cmp::Eq for CF_PROCESS_INFO {}
impl ::core::fmt::Debug for CF_PROCESS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_PROCESS_INFO").field("StructSize", &self.StructSize).field("ProcessId", &self.ProcessId).field("ImagePath", &self.ImagePath).field("PackageName", &self.PackageName).field("ApplicationId", &self.ApplicationId).field("CommandLine", &self.CommandLine).field("SessionId", &self.SessionId).finish()
    }
}
impl ::core::default::Default for CF_REGISTER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CF_REGISTER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_REGISTER_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_REGISTER_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_REGISTER_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_REGISTER_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_REGISTER_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_REGISTER_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CF_REVERT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CF_REVERT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_REVERT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_REVERT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_REVERT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_REVERT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_REVERT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_REVERT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CF_SET_IN_SYNC_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CF_SET_IN_SYNC_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_SET_IN_SYNC_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_SET_IN_SYNC_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_SET_IN_SYNC_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_SET_IN_SYNC_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_SET_IN_SYNC_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_SET_IN_SYNC_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CF_SET_PIN_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CF_SET_PIN_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_SET_PIN_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_SET_PIN_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_SET_PIN_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_SET_PIN_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_SET_PIN_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_SET_PIN_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CF_SYNC_POLICIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CF_SYNC_POLICIES {
    fn eq(&self, other: &Self) -> bool {
        self.StructSize == other.StructSize && self.Hydration == other.Hydration && self.Population == other.Population && self.InSync == other.InSync && self.HardLink == other.HardLink && self.PlaceholderManagement == other.PlaceholderManagement
    }
}
impl ::core::cmp::Eq for CF_SYNC_POLICIES {}
impl ::core::fmt::Debug for CF_SYNC_POLICIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_SYNC_POLICIES").field("StructSize", &self.StructSize).field("Hydration", &self.Hydration).field("Population", &self.Population).field("InSync", &self.InSync).field("HardLink", &self.HardLink).field("PlaceholderManagement", &self.PlaceholderManagement).finish()
    }
}
impl ::core::default::Default for CF_SYNC_PROVIDER_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CF_SYNC_PROVIDER_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_SYNC_PROVIDER_STATUS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_SYNC_PROVIDER_STATUS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_SYNC_PROVIDER_STATUS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_SYNC_PROVIDER_STATUS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_SYNC_PROVIDER_STATUS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_SYNC_PROVIDER_STATUS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for CF_SYNC_REGISTRATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CF_SYNC_REGISTRATION {
    fn eq(&self, other: &Self) -> bool {
        self.StructSize == other.StructSize && self.ProviderName == other.ProviderName && self.ProviderVersion == other.ProviderVersion && self.SyncRootIdentity == other.SyncRootIdentity && self.SyncRootIdentityLength == other.SyncRootIdentityLength && self.FileIdentity == other.FileIdentity && self.FileIdentityLength == other.FileIdentityLength && self.ProviderId == other.ProviderId
    }
}
impl ::core::cmp::Eq for CF_SYNC_REGISTRATION {}
impl ::core::fmt::Debug for CF_SYNC_REGISTRATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_SYNC_REGISTRATION").field("StructSize", &self.StructSize).field("ProviderName", &self.ProviderName).field("ProviderVersion", &self.ProviderVersion).field("SyncRootIdentity", &self.SyncRootIdentity).field("SyncRootIdentityLength", &self.SyncRootIdentityLength).field("FileIdentity", &self.FileIdentity).field("FileIdentityLength", &self.FileIdentityLength).field("ProviderId", &self.ProviderId).finish()
    }
}
impl ::core::default::Default for CF_SYNC_ROOT_BASIC_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CF_SYNC_ROOT_BASIC_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.SyncRootFileId == other.SyncRootFileId
    }
}
impl ::core::cmp::Eq for CF_SYNC_ROOT_BASIC_INFO {}
impl ::core::fmt::Debug for CF_SYNC_ROOT_BASIC_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_SYNC_ROOT_BASIC_INFO").field("SyncRootFileId", &self.SyncRootFileId).finish()
    }
}
impl ::core::default::Default for CF_SYNC_ROOT_INFO_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CF_SYNC_ROOT_INFO_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_SYNC_ROOT_INFO_CLASS").field(&self.0).finish()
    }
}
impl ::core::default::Default for CF_SYNC_ROOT_PROVIDER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CF_SYNC_ROOT_PROVIDER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ProviderStatus == other.ProviderStatus && self.ProviderName == other.ProviderName && self.ProviderVersion == other.ProviderVersion
    }
}
impl ::core::cmp::Eq for CF_SYNC_ROOT_PROVIDER_INFO {}
impl ::core::fmt::Debug for CF_SYNC_ROOT_PROVIDER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_SYNC_ROOT_PROVIDER_INFO").field("ProviderStatus", &self.ProviderStatus).field("ProviderName", &self.ProviderName).field("ProviderVersion", &self.ProviderVersion).finish()
    }
}
impl ::core::default::Default for CF_SYNC_ROOT_STANDARD_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CF_SYNC_ROOT_STANDARD_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.SyncRootFileId == other.SyncRootFileId && self.HydrationPolicy == other.HydrationPolicy && self.PopulationPolicy == other.PopulationPolicy && self.InSyncPolicy == other.InSyncPolicy && self.HardLinkPolicy == other.HardLinkPolicy && self.ProviderStatus == other.ProviderStatus && self.ProviderName == other.ProviderName && self.ProviderVersion == other.ProviderVersion && self.SyncRootIdentityLength == other.SyncRootIdentityLength && self.SyncRootIdentity == other.SyncRootIdentity
    }
}
impl ::core::cmp::Eq for CF_SYNC_ROOT_STANDARD_INFO {}
impl ::core::fmt::Debug for CF_SYNC_ROOT_STANDARD_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_SYNC_ROOT_STANDARD_INFO")
            .field("SyncRootFileId", &self.SyncRootFileId)
            .field("HydrationPolicy", &self.HydrationPolicy)
            .field("PopulationPolicy", &self.PopulationPolicy)
            .field("InSyncPolicy", &self.InSyncPolicy)
            .field("HardLinkPolicy", &self.HardLinkPolicy)
            .field("ProviderStatus", &self.ProviderStatus)
            .field("ProviderName", &self.ProviderName)
            .field("ProviderVersion", &self.ProviderVersion)
            .field("SyncRootIdentityLength", &self.SyncRootIdentityLength)
            .field("SyncRootIdentity", &self.SyncRootIdentity)
            .finish()
    }
}
impl ::core::default::Default for CF_SYNC_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CF_SYNC_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.StructSize == other.StructSize && self.Code == other.Code && self.DescriptionOffset == other.DescriptionOffset && self.DescriptionLength == other.DescriptionLength && self.DeviceIdOffset == other.DeviceIdOffset && self.DeviceIdLength == other.DeviceIdLength
    }
}
impl ::core::cmp::Eq for CF_SYNC_STATUS {}
impl ::core::fmt::Debug for CF_SYNC_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CF_SYNC_STATUS").field("StructSize", &self.StructSize).field("Code", &self.Code).field("DescriptionOffset", &self.DescriptionOffset).field("DescriptionLength", &self.DescriptionLength).field("DeviceIdOffset", &self.DeviceIdOffset).field("DeviceIdLength", &self.DeviceIdLength).finish()
    }
}
impl ::core::default::Default for CF_UPDATE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CF_UPDATE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CF_UPDATE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CF_UPDATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CF_UPDATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CF_UPDATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CF_UPDATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CF_UPDATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}

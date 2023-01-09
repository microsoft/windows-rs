impl ::core::cmp::PartialEq for PreviewBuildsManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PreviewBuildsManager {}
impl ::core::fmt::Debug for PreviewBuildsManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PreviewBuildsManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for PreviewBuildsState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PreviewBuildsState {}
impl ::core::fmt::Debug for PreviewBuildsState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PreviewBuildsState").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for WindowsUpdate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WindowsUpdate {}
impl ::core::fmt::Debug for WindowsUpdate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowsUpdate").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for WindowsUpdateActionCompletedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WindowsUpdateActionCompletedEventArgs {}
impl ::core::fmt::Debug for WindowsUpdateActionCompletedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowsUpdateActionCompletedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for WindowsUpdateActionProgress {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WindowsUpdateActionProgress {}
impl ::core::fmt::Debug for WindowsUpdateActionProgress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowsUpdateActionProgress").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for WindowsUpdateActionResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WindowsUpdateActionResult {}
impl ::core::fmt::Debug for WindowsUpdateActionResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowsUpdateActionResult").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for WindowsUpdateAdministrator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WindowsUpdateAdministrator {}
impl ::core::fmt::Debug for WindowsUpdateAdministrator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowsUpdateAdministrator").field(&self.0).finish()
    }
}
impl ::core::default::Default for WindowsUpdateAdministratorOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WindowsUpdateAdministratorOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowsUpdateAdministratorOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for WindowsUpdateAdministratorOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for WindowsUpdateAdministratorOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for WindowsUpdateAdministratorOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for WindowsUpdateAdministratorOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for WindowsUpdateAdministratorOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for WindowsUpdateAdministratorStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WindowsUpdateAdministratorStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowsUpdateAdministratorStatus").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for WindowsUpdateApprovalData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WindowsUpdateApprovalData {}
impl ::core::fmt::Debug for WindowsUpdateApprovalData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowsUpdateApprovalData").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for WindowsUpdateAttentionRequiredInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WindowsUpdateAttentionRequiredInfo {}
impl ::core::fmt::Debug for WindowsUpdateAttentionRequiredInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowsUpdateAttentionRequiredInfo").field(&self.0).finish()
    }
}
impl ::core::default::Default for WindowsUpdateAttentionRequiredReason {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WindowsUpdateAttentionRequiredReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowsUpdateAttentionRequiredReason").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for WindowsUpdateAttentionRequiredReasonChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WindowsUpdateAttentionRequiredReasonChangedEventArgs {}
impl ::core::fmt::Debug for WindowsUpdateAttentionRequiredReasonChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowsUpdateAttentionRequiredReasonChangedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for WindowsUpdateGetAdministratorResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WindowsUpdateGetAdministratorResult {}
impl ::core::fmt::Debug for WindowsUpdateGetAdministratorResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowsUpdateGetAdministratorResult").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for WindowsUpdateItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WindowsUpdateItem {}
impl ::core::fmt::Debug for WindowsUpdateItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowsUpdateItem").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for WindowsUpdateManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WindowsUpdateManager {}
impl ::core::fmt::Debug for WindowsUpdateManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowsUpdateManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for WindowsUpdateProgressChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WindowsUpdateProgressChangedEventArgs {}
impl ::core::fmt::Debug for WindowsUpdateProgressChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowsUpdateProgressChangedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for WindowsUpdateRestartRequestOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WindowsUpdateRestartRequestOptions {}
impl ::core::fmt::Debug for WindowsUpdateRestartRequestOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowsUpdateRestartRequestOptions").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for WindowsUpdateScanCompletedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WindowsUpdateScanCompletedEventArgs {}
impl ::core::fmt::Debug for WindowsUpdateScanCompletedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowsUpdateScanCompletedEventArgs").field(&self.0).finish()
    }
}

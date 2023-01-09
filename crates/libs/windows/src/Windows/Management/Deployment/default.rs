impl ::core::default::Default for AddPackageByAppInstallerOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for AddPackageByAppInstallerOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AddPackageByAppInstallerOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for AddPackageByAppInstallerOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for AddPackageByAppInstallerOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for AddPackageByAppInstallerOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for AddPackageByAppInstallerOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for AddPackageByAppInstallerOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::cmp::PartialEq for AddPackageOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AddPackageOptions {}
impl ::core::fmt::Debug for AddPackageOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AddPackageOptions").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for AppInstallerManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AppInstallerManager {}
impl ::core::fmt::Debug for AppInstallerManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppInstallerManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for AutoUpdateSettingsOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AutoUpdateSettingsOptions {}
impl ::core::fmt::Debug for AutoUpdateSettingsOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutoUpdateSettingsOptions").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for CreateSharedPackageContainerOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CreateSharedPackageContainerOptions {}
impl ::core::fmt::Debug for CreateSharedPackageContainerOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CreateSharedPackageContainerOptions").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for CreateSharedPackageContainerResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CreateSharedPackageContainerResult {}
impl ::core::fmt::Debug for CreateSharedPackageContainerResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CreateSharedPackageContainerResult").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for DeleteSharedPackageContainerOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeleteSharedPackageContainerOptions {}
impl ::core::fmt::Debug for DeleteSharedPackageContainerOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeleteSharedPackageContainerOptions").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for DeleteSharedPackageContainerResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeleteSharedPackageContainerResult {}
impl ::core::fmt::Debug for DeleteSharedPackageContainerResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeleteSharedPackageContainerResult").field(&self.0).finish()
    }
}
impl ::core::default::Default for DeploymentOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DeploymentOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeploymentOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DeploymentOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DeploymentOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DeploymentOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DeploymentOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DeploymentOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for DeploymentProgress {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DeploymentProgress {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state && self.percentage == other.percentage
    }
}
impl ::core::cmp::Eq for DeploymentProgress {}
impl ::core::fmt::Debug for DeploymentProgress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DeploymentProgress").field("state", &self.state).field("percentage", &self.percentage).finish()
    }
}
impl ::core::default::Default for DeploymentProgressState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DeploymentProgressState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeploymentProgressState").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for DeploymentResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeploymentResult {}
impl ::core::fmt::Debug for DeploymentResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeploymentResult").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for FindSharedPackageContainerOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FindSharedPackageContainerOptions {}
impl ::core::fmt::Debug for FindSharedPackageContainerOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FindSharedPackageContainerOptions").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for PackageAllUserProvisioningOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PackageAllUserProvisioningOptions {}
impl ::core::fmt::Debug for PackageAllUserProvisioningOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageAllUserProvisioningOptions").field(&self.0).finish()
    }
}
impl ::core::default::Default for PackageInstallState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PackageInstallState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageInstallState").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for PackageManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PackageManager {}
impl ::core::fmt::Debug for PackageManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for PackageManagerDebugSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PackageManagerDebugSettings {}
impl ::core::fmt::Debug for PackageManagerDebugSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageManagerDebugSettings").field(&self.0).finish()
    }
}
impl ::core::default::Default for PackageState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PackageState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageState").field(&self.0).finish()
    }
}
impl ::core::default::Default for PackageStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PackageStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageStatus").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PackageStatus {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PackageStatus {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PackageStatus {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PackageStatus {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PackageStatus {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for PackageStubPreference {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PackageStubPreference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageStubPreference").field(&self.0).finish()
    }
}
impl ::core::default::Default for PackageTypes {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PackageTypes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageTypes").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PackageTypes {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PackageTypes {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PackageTypes {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PackageTypes {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PackageTypes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::cmp::PartialEq for PackageUserInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PackageUserInformation {}
impl ::core::fmt::Debug for PackageUserInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageUserInformation").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for PackageVolume {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PackageVolume {}
impl ::core::fmt::Debug for PackageVolume {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageVolume").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for RegisterPackageOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RegisterPackageOptions {}
impl ::core::fmt::Debug for RegisterPackageOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RegisterPackageOptions").field(&self.0).finish()
    }
}
impl ::core::default::Default for RemovalOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RemovalOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemovalOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for RemovalOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for RemovalOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for RemovalOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for RemovalOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for RemovalOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::cmp::PartialEq for SharedPackageContainer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SharedPackageContainer {}
impl ::core::fmt::Debug for SharedPackageContainer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SharedPackageContainer").field(&self.0).finish()
    }
}
impl ::core::default::Default for SharedPackageContainerCreationCollisionOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SharedPackageContainerCreationCollisionOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SharedPackageContainerCreationCollisionOptions").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for SharedPackageContainerManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SharedPackageContainerManager {}
impl ::core::fmt::Debug for SharedPackageContainerManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SharedPackageContainerManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for SharedPackageContainerMember {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SharedPackageContainerMember {}
impl ::core::fmt::Debug for SharedPackageContainerMember {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SharedPackageContainerMember").field(&self.0).finish()
    }
}
impl ::core::default::Default for SharedPackageContainerOperationStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SharedPackageContainerOperationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SharedPackageContainerOperationStatus").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for StagePackageOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StagePackageOptions {}
impl ::core::fmt::Debug for StagePackageOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StagePackageOptions").field(&self.0).finish()
    }
}
impl ::core::default::Default for StubPackageOption {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for StubPackageOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StubPackageOption").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for UpdateSharedPackageContainerOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UpdateSharedPackageContainerOptions {}
impl ::core::fmt::Debug for UpdateSharedPackageContainerOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UpdateSharedPackageContainerOptions").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for UpdateSharedPackageContainerResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UpdateSharedPackageContainerResult {}
impl ::core::fmt::Debug for UpdateSharedPackageContainerResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UpdateSharedPackageContainerResult").field(&self.0).finish()
    }
}

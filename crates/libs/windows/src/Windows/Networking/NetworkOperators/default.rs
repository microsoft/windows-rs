impl ::core::default::Default for DataClasses {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DataClasses {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DataClasses").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DataClasses {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DataClasses {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DataClasses {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DataClasses {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DataClasses {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::cmp::PartialEq for ESim {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ESim {}
impl ::core::fmt::Debug for ESim {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESim").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ESimAddedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ESimAddedEventArgs {}
impl ::core::fmt::Debug for ESimAddedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESimAddedEventArgs").field(&self.0).finish()
    }
}
impl ::core::default::Default for ESimAuthenticationPreference {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ESimAuthenticationPreference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESimAuthenticationPreference").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ESimDiscoverEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ESimDiscoverEvent {}
impl ::core::fmt::Debug for ESimDiscoverEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESimDiscoverEvent").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ESimDiscoverResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ESimDiscoverResult {}
impl ::core::fmt::Debug for ESimDiscoverResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESimDiscoverResult").field(&self.0).finish()
    }
}
impl ::core::default::Default for ESimDiscoverResultKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ESimDiscoverResultKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESimDiscoverResultKind").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ESimDownloadProfileMetadataResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ESimDownloadProfileMetadataResult {}
impl ::core::fmt::Debug for ESimDownloadProfileMetadataResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESimDownloadProfileMetadataResult").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ESimOperationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ESimOperationResult {}
impl ::core::fmt::Debug for ESimOperationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESimOperationResult").field(&self.0).finish()
    }
}
impl ::core::default::Default for ESimOperationStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ESimOperationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESimOperationStatus").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ESimPolicy {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ESimPolicy {}
impl ::core::fmt::Debug for ESimPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESimPolicy").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ESimProfile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ESimProfile {}
impl ::core::fmt::Debug for ESimProfile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESimProfile").field(&self.0).finish()
    }
}
impl ::core::default::Default for ESimProfileClass {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ESimProfileClass {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESimProfileClass").field(&self.0).finish()
    }
}
impl ::core::default::Default for ESimProfileInstallProgress {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ESimProfileInstallProgress {
    fn eq(&self, other: &Self) -> bool {
        self.TotalSizeInBytes == other.TotalSizeInBytes && self.InstalledSizeInBytes == other.InstalledSizeInBytes
    }
}
impl ::core::cmp::Eq for ESimProfileInstallProgress {}
impl ::core::fmt::Debug for ESimProfileInstallProgress {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ESimProfileInstallProgress").field("TotalSizeInBytes", &self.TotalSizeInBytes).field("InstalledSizeInBytes", &self.InstalledSizeInBytes).finish()
    }
}
impl ::core::cmp::PartialEq for ESimProfileMetadata {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ESimProfileMetadata {}
impl ::core::fmt::Debug for ESimProfileMetadata {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESimProfileMetadata").field(&self.0).finish()
    }
}
impl ::core::default::Default for ESimProfileMetadataState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ESimProfileMetadataState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESimProfileMetadataState").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ESimProfilePolicy {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ESimProfilePolicy {}
impl ::core::fmt::Debug for ESimProfilePolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESimProfilePolicy").field(&self.0).finish()
    }
}
impl ::core::default::Default for ESimProfileState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ESimProfileState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESimProfileState").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ESimRemovedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ESimRemovedEventArgs {}
impl ::core::fmt::Debug for ESimRemovedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESimRemovedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ESimServiceInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ESimServiceInfo {}
impl ::core::fmt::Debug for ESimServiceInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESimServiceInfo").field(&self.0).finish()
    }
}
impl ::core::default::Default for ESimState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ESimState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESimState").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ESimUpdatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ESimUpdatedEventArgs {}
impl ::core::fmt::Debug for ESimUpdatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESimUpdatedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ESimWatcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ESimWatcher {}
impl ::core::fmt::Debug for ESimWatcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESimWatcher").field(&self.0).finish()
    }
}
impl ::core::default::Default for ESimWatcherStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ESimWatcherStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ESimWatcherStatus").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for HotspotAuthenticationContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HotspotAuthenticationContext {}
impl ::core::fmt::Debug for HotspotAuthenticationContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HotspotAuthenticationContext").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for HotspotAuthenticationEventDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HotspotAuthenticationEventDetails {}
impl ::core::fmt::Debug for HotspotAuthenticationEventDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HotspotAuthenticationEventDetails").field(&self.0).finish()
    }
}
impl ::core::default::Default for HotspotAuthenticationResponseCode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HotspotAuthenticationResponseCode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HotspotAuthenticationResponseCode").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for HotspotCredentialsAuthenticationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HotspotCredentialsAuthenticationResult {}
impl ::core::fmt::Debug for HotspotCredentialsAuthenticationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HotspotCredentialsAuthenticationResult").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandAccount {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandAccount {}
impl ::core::fmt::Debug for MobileBroadbandAccount {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandAccount").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandAccountEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandAccountEventArgs {}
impl ::core::fmt::Debug for MobileBroadbandAccountEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandAccountEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandAccountUpdatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandAccountUpdatedEventArgs {}
impl ::core::fmt::Debug for MobileBroadbandAccountUpdatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandAccountUpdatedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandAccountWatcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandAccountWatcher {}
impl ::core::fmt::Debug for MobileBroadbandAccountWatcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandAccountWatcher").field(&self.0).finish()
    }
}
impl ::core::default::Default for MobileBroadbandAccountWatcherStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MobileBroadbandAccountWatcherStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandAccountWatcherStatus").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandAntennaSar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandAntennaSar {}
impl ::core::fmt::Debug for MobileBroadbandAntennaSar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandAntennaSar").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandCellCdma {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandCellCdma {}
impl ::core::fmt::Debug for MobileBroadbandCellCdma {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandCellCdma").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandCellGsm {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandCellGsm {}
impl ::core::fmt::Debug for MobileBroadbandCellGsm {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandCellGsm").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandCellLte {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandCellLte {}
impl ::core::fmt::Debug for MobileBroadbandCellLte {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandCellLte").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandCellNR {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandCellNR {}
impl ::core::fmt::Debug for MobileBroadbandCellNR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandCellNR").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandCellTdscdma {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandCellTdscdma {}
impl ::core::fmt::Debug for MobileBroadbandCellTdscdma {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandCellTdscdma").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandCellUmts {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandCellUmts {}
impl ::core::fmt::Debug for MobileBroadbandCellUmts {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandCellUmts").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandCellsInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandCellsInfo {}
impl ::core::fmt::Debug for MobileBroadbandCellsInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandCellsInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandCurrentSlotIndexChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandCurrentSlotIndexChangedEventArgs {}
impl ::core::fmt::Debug for MobileBroadbandCurrentSlotIndexChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandCurrentSlotIndexChangedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandDeviceInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandDeviceInformation {}
impl ::core::fmt::Debug for MobileBroadbandDeviceInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandDeviceInformation").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandDeviceService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandDeviceService {}
impl ::core::fmt::Debug for MobileBroadbandDeviceService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandDeviceService").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandDeviceServiceCommandResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandDeviceServiceCommandResult {}
impl ::core::fmt::Debug for MobileBroadbandDeviceServiceCommandResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandDeviceServiceCommandResult").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandDeviceServiceCommandSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandDeviceServiceCommandSession {}
impl ::core::fmt::Debug for MobileBroadbandDeviceServiceCommandSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandDeviceServiceCommandSession").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandDeviceServiceDataReceivedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandDeviceServiceDataReceivedEventArgs {}
impl ::core::fmt::Debug for MobileBroadbandDeviceServiceDataReceivedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandDeviceServiceDataReceivedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandDeviceServiceDataSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandDeviceServiceDataSession {}
impl ::core::fmt::Debug for MobileBroadbandDeviceServiceDataSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandDeviceServiceDataSession").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandDeviceServiceInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandDeviceServiceInformation {}
impl ::core::fmt::Debug for MobileBroadbandDeviceServiceInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandDeviceServiceInformation").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandDeviceServiceTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandDeviceServiceTriggerDetails {}
impl ::core::fmt::Debug for MobileBroadbandDeviceServiceTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandDeviceServiceTriggerDetails").field(&self.0).finish()
    }
}
impl ::core::default::Default for MobileBroadbandDeviceType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MobileBroadbandDeviceType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandDeviceType").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandModem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandModem {}
impl ::core::fmt::Debug for MobileBroadbandModem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandModem").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandModemConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandModemConfiguration {}
impl ::core::fmt::Debug for MobileBroadbandModemConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandModemConfiguration").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandModemIsolation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandModemIsolation {}
impl ::core::fmt::Debug for MobileBroadbandModemIsolation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandModemIsolation").field(&self.0).finish()
    }
}
impl ::core::default::Default for MobileBroadbandModemStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MobileBroadbandModemStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandModemStatus").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandNetwork {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandNetwork {}
impl ::core::fmt::Debug for MobileBroadbandNetwork {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandNetwork").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandNetworkRegistrationStateChange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandNetworkRegistrationStateChange {}
impl ::core::fmt::Debug for MobileBroadbandNetworkRegistrationStateChange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandNetworkRegistrationStateChange").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandNetworkRegistrationStateChangeTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandNetworkRegistrationStateChangeTriggerDetails {}
impl ::core::fmt::Debug for MobileBroadbandNetworkRegistrationStateChangeTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandNetworkRegistrationStateChangeTriggerDetails").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandPco {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandPco {}
impl ::core::fmt::Debug for MobileBroadbandPco {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandPco").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandPcoDataChangeTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandPcoDataChangeTriggerDetails {}
impl ::core::fmt::Debug for MobileBroadbandPcoDataChangeTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandPcoDataChangeTriggerDetails").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandPin {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandPin {}
impl ::core::fmt::Debug for MobileBroadbandPin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandPin").field(&self.0).finish()
    }
}
impl ::core::default::Default for MobileBroadbandPinFormat {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MobileBroadbandPinFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandPinFormat").field(&self.0).finish()
    }
}
impl ::core::default::Default for MobileBroadbandPinLockState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MobileBroadbandPinLockState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandPinLockState").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandPinLockStateChange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandPinLockStateChange {}
impl ::core::fmt::Debug for MobileBroadbandPinLockStateChange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandPinLockStateChange").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandPinLockStateChangeTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandPinLockStateChangeTriggerDetails {}
impl ::core::fmt::Debug for MobileBroadbandPinLockStateChangeTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandPinLockStateChangeTriggerDetails").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandPinManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandPinManager {}
impl ::core::fmt::Debug for MobileBroadbandPinManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandPinManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandPinOperationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandPinOperationResult {}
impl ::core::fmt::Debug for MobileBroadbandPinOperationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandPinOperationResult").field(&self.0).finish()
    }
}
impl ::core::default::Default for MobileBroadbandPinType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MobileBroadbandPinType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandPinType").field(&self.0).finish()
    }
}
impl ::core::default::Default for MobileBroadbandRadioState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MobileBroadbandRadioState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandRadioState").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandRadioStateChange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandRadioStateChange {}
impl ::core::fmt::Debug for MobileBroadbandRadioStateChange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandRadioStateChange").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandRadioStateChangeTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandRadioStateChangeTriggerDetails {}
impl ::core::fmt::Debug for MobileBroadbandRadioStateChangeTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandRadioStateChangeTriggerDetails").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandSarManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandSarManager {}
impl ::core::fmt::Debug for MobileBroadbandSarManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandSarManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandSlotInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandSlotInfo {}
impl ::core::fmt::Debug for MobileBroadbandSlotInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandSlotInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandSlotInfoChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandSlotInfoChangedEventArgs {}
impl ::core::fmt::Debug for MobileBroadbandSlotInfoChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandSlotInfoChangedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandSlotManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandSlotManager {}
impl ::core::fmt::Debug for MobileBroadbandSlotManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandSlotManager").field(&self.0).finish()
    }
}
impl ::core::default::Default for MobileBroadbandSlotState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MobileBroadbandSlotState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandSlotState").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandTransmissionStateChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandTransmissionStateChangedEventArgs {}
impl ::core::fmt::Debug for MobileBroadbandTransmissionStateChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandTransmissionStateChangedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandUicc {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandUicc {}
impl ::core::fmt::Debug for MobileBroadbandUicc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandUicc").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandUiccApp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandUiccApp {}
impl ::core::fmt::Debug for MobileBroadbandUiccApp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandUiccApp").field(&self.0).finish()
    }
}
impl ::core::default::Default for MobileBroadbandUiccAppOperationStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MobileBroadbandUiccAppOperationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandUiccAppOperationStatus").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandUiccAppReadRecordResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandUiccAppReadRecordResult {}
impl ::core::fmt::Debug for MobileBroadbandUiccAppReadRecordResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandUiccAppReadRecordResult").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandUiccAppRecordDetailsResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandUiccAppRecordDetailsResult {}
impl ::core::fmt::Debug for MobileBroadbandUiccAppRecordDetailsResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandUiccAppRecordDetailsResult").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for MobileBroadbandUiccAppsResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MobileBroadbandUiccAppsResult {}
impl ::core::fmt::Debug for MobileBroadbandUiccAppsResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MobileBroadbandUiccAppsResult").field(&self.0).finish()
    }
}
impl ::core::default::Default for NetworkDeviceStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NetworkDeviceStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NetworkDeviceStatus").field(&self.0).finish()
    }
}
impl ::core::default::Default for NetworkOperatorDataUsageNotificationKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NetworkOperatorDataUsageNotificationKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NetworkOperatorDataUsageNotificationKind").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for NetworkOperatorDataUsageTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NetworkOperatorDataUsageTriggerDetails {}
impl ::core::fmt::Debug for NetworkOperatorDataUsageTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NetworkOperatorDataUsageTriggerDetails").field(&self.0).finish()
    }
}
impl ::core::default::Default for NetworkOperatorEventMessageType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NetworkOperatorEventMessageType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NetworkOperatorEventMessageType").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for NetworkOperatorNotificationEventDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NetworkOperatorNotificationEventDetails {}
impl ::core::fmt::Debug for NetworkOperatorNotificationEventDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NetworkOperatorNotificationEventDetails").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for NetworkOperatorTetheringAccessPointConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NetworkOperatorTetheringAccessPointConfiguration {}
impl ::core::fmt::Debug for NetworkOperatorTetheringAccessPointConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NetworkOperatorTetheringAccessPointConfiguration").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for NetworkOperatorTetheringClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NetworkOperatorTetheringClient {}
impl ::core::fmt::Debug for NetworkOperatorTetheringClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NetworkOperatorTetheringClient").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for NetworkOperatorTetheringManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NetworkOperatorTetheringManager {}
impl ::core::fmt::Debug for NetworkOperatorTetheringManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NetworkOperatorTetheringManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for NetworkOperatorTetheringOperationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NetworkOperatorTetheringOperationResult {}
impl ::core::fmt::Debug for NetworkOperatorTetheringOperationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NetworkOperatorTetheringOperationResult").field(&self.0).finish()
    }
}
impl ::core::default::Default for NetworkRegistrationState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for NetworkRegistrationState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NetworkRegistrationState").field(&self.0).finish()
    }
}
impl ::core::default::Default for ProfileMediaType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ProfileMediaType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProfileMediaType").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation")]
impl ::core::default::Default for ProfileUsage {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::PartialEq for ProfileUsage {
    fn eq(&self, other: &Self) -> bool {
        self.UsageInMegabytes == other.UsageInMegabytes && self.LastSyncTime == other.LastSyncTime
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::Eq for ProfileUsage {}
#[cfg(feature = "Foundation")]
impl ::core::fmt::Debug for ProfileUsage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ProfileUsage").field("UsageInMegabytes", &self.UsageInMegabytes).field("LastSyncTime", &self.LastSyncTime).finish()
    }
}
impl ::core::cmp::PartialEq for ProvisionFromXmlDocumentResults {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProvisionFromXmlDocumentResults {}
impl ::core::fmt::Debug for ProvisionFromXmlDocumentResults {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProvisionFromXmlDocumentResults").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ProvisionedProfile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProvisionedProfile {}
impl ::core::fmt::Debug for ProvisionedProfile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProvisionedProfile").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ProvisioningAgent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ProvisioningAgent {}
impl ::core::fmt::Debug for ProvisioningAgent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ProvisioningAgent").field(&self.0).finish()
    }
}
impl ::core::default::Default for TetheringCapability {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TetheringCapability {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TetheringCapability").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for TetheringEntitlementCheckTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TetheringEntitlementCheckTriggerDetails {}
impl ::core::fmt::Debug for TetheringEntitlementCheckTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TetheringEntitlementCheckTriggerDetails").field(&self.0).finish()
    }
}
impl ::core::default::Default for TetheringOperationStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TetheringOperationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TetheringOperationStatus").field(&self.0).finish()
    }
}
impl ::core::default::Default for TetheringOperationalState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TetheringOperationalState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TetheringOperationalState").field(&self.0).finish()
    }
}
impl ::core::default::Default for TetheringWiFiBand {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TetheringWiFiBand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TetheringWiFiBand").field(&self.0).finish()
    }
}
impl ::core::default::Default for UiccAccessCondition {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UiccAccessCondition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UiccAccessCondition").field(&self.0).finish()
    }
}
impl ::core::default::Default for UiccAppKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UiccAppKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UiccAppKind").field(&self.0).finish()
    }
}
impl ::core::default::Default for UiccAppRecordKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UiccAppRecordKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UiccAppRecordKind").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for UssdMessage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UssdMessage {}
impl ::core::fmt::Debug for UssdMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UssdMessage").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for UssdReply {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UssdReply {}
impl ::core::fmt::Debug for UssdReply {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UssdReply").field(&self.0).finish()
    }
}
impl ::core::default::Default for UssdResultCode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for UssdResultCode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UssdResultCode").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for UssdSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UssdSession {}
impl ::core::fmt::Debug for UssdSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UssdSession").field(&self.0).finish()
    }
}

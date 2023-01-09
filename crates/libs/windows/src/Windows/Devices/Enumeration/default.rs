impl ::core::cmp::PartialEq for DeviceAccessChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceAccessChangedEventArgs {}
impl ::core::fmt::Debug for DeviceAccessChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceAccessChangedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for DeviceAccessInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceAccessInformation {}
impl ::core::fmt::Debug for DeviceAccessInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceAccessInformation").field(&self.0).finish()
    }
}
impl ::core::default::Default for DeviceAccessStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DeviceAccessStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceAccessStatus").field(&self.0).finish()
    }
}
impl ::core::default::Default for DeviceClass {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DeviceClass {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceClass").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for DeviceConnectionChangeTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceConnectionChangeTriggerDetails {}
impl ::core::fmt::Debug for DeviceConnectionChangeTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceConnectionChangeTriggerDetails").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for DeviceDisconnectButtonClickedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceDisconnectButtonClickedEventArgs {}
impl ::core::fmt::Debug for DeviceDisconnectButtonClickedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceDisconnectButtonClickedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for DeviceInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceInformation {}
impl ::core::fmt::Debug for DeviceInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceInformation").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for DeviceInformationCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for DeviceInformationCollection {}
#[cfg(feature = "Foundation_Collections")]
impl ::core::fmt::Debug for DeviceInformationCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceInformationCollection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for DeviceInformationCustomPairing {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceInformationCustomPairing {}
impl ::core::fmt::Debug for DeviceInformationCustomPairing {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceInformationCustomPairing").field(&self.0).finish()
    }
}
impl ::core::default::Default for DeviceInformationKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DeviceInformationKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceInformationKind").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for DeviceInformationPairing {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceInformationPairing {}
impl ::core::fmt::Debug for DeviceInformationPairing {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceInformationPairing").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for DeviceInformationUpdate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceInformationUpdate {}
impl ::core::fmt::Debug for DeviceInformationUpdate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceInformationUpdate").field(&self.0).finish()
    }
}
impl ::core::default::Default for DevicePairingKinds {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DevicePairingKinds {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DevicePairingKinds").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DevicePairingKinds {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DevicePairingKinds {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DevicePairingKinds {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DevicePairingKinds {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DevicePairingKinds {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for DevicePairingProtectionLevel {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DevicePairingProtectionLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DevicePairingProtectionLevel").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for DevicePairingRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DevicePairingRequestedEventArgs {}
impl ::core::fmt::Debug for DevicePairingRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DevicePairingRequestedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for DevicePairingResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DevicePairingResult {}
impl ::core::fmt::Debug for DevicePairingResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DevicePairingResult").field(&self.0).finish()
    }
}
impl ::core::default::Default for DevicePairingResultStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DevicePairingResultStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DevicePairingResultStatus").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for DevicePicker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DevicePicker {}
impl ::core::fmt::Debug for DevicePicker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DevicePicker").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for DevicePickerAppearance {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DevicePickerAppearance {}
impl ::core::fmt::Debug for DevicePickerAppearance {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DevicePickerAppearance").field(&self.0).finish()
    }
}
impl ::core::default::Default for DevicePickerDisplayStatusOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DevicePickerDisplayStatusOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DevicePickerDisplayStatusOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DevicePickerDisplayStatusOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DevicePickerDisplayStatusOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DevicePickerDisplayStatusOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DevicePickerDisplayStatusOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DevicePickerDisplayStatusOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::cmp::PartialEq for DevicePickerFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DevicePickerFilter {}
impl ::core::fmt::Debug for DevicePickerFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DevicePickerFilter").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for DeviceSelectedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceSelectedEventArgs {}
impl ::core::fmt::Debug for DeviceSelectedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceSelectedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::cmp::PartialEq for DeviceThumbnail {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::cmp::Eq for DeviceThumbnail {}
#[cfg(feature = "Storage_Streams")]
impl ::core::fmt::Debug for DeviceThumbnail {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceThumbnail").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for DeviceUnpairingResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceUnpairingResult {}
impl ::core::fmt::Debug for DeviceUnpairingResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceUnpairingResult").field(&self.0).finish()
    }
}
impl ::core::default::Default for DeviceUnpairingResultStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DeviceUnpairingResultStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceUnpairingResultStatus").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for DeviceWatcher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceWatcher {}
impl ::core::fmt::Debug for DeviceWatcher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceWatcher").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for DeviceWatcherEvent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceWatcherEvent {}
impl ::core::fmt::Debug for DeviceWatcherEvent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceWatcherEvent").field(&self.0).finish()
    }
}
impl ::core::default::Default for DeviceWatcherEventKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DeviceWatcherEventKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceWatcherEventKind").field(&self.0).finish()
    }
}
impl ::core::default::Default for DeviceWatcherStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DeviceWatcherStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceWatcherStatus").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for DeviceWatcherTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for DeviceWatcherTriggerDetails {}
impl ::core::fmt::Debug for DeviceWatcherTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DeviceWatcherTriggerDetails").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for EnclosureLocation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for EnclosureLocation {}
impl ::core::fmt::Debug for EnclosureLocation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EnclosureLocation").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDevicePairingSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDevicePairingSettings {}
impl ::core::fmt::Debug for IDevicePairingSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDevicePairingSettings").field(&self.0).finish()
    }
}
impl ::core::default::Default for Panel {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for Panel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Panel").field(&self.0).finish()
    }
}

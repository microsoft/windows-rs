impl ::core::cmp::PartialEq for GattCharacteristic {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattCharacteristic {}
impl ::core::fmt::Debug for GattCharacteristic {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattCharacteristic").field(&self.0).finish()
    }
}
impl ::core::default::Default for GattCharacteristicProperties {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GattCharacteristicProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattCharacteristicProperties").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for GattCharacteristicProperties {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for GattCharacteristicProperties {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for GattCharacteristicProperties {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for GattCharacteristicProperties {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for GattCharacteristicProperties {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::cmp::PartialEq for GattCharacteristicsResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattCharacteristicsResult {}
impl ::core::fmt::Debug for GattCharacteristicsResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattCharacteristicsResult").field(&self.0).finish()
    }
}
impl ::core::default::Default for GattClientCharacteristicConfigurationDescriptorValue {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GattClientCharacteristicConfigurationDescriptorValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattClientCharacteristicConfigurationDescriptorValue").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for GattClientNotificationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattClientNotificationResult {}
impl ::core::fmt::Debug for GattClientNotificationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattClientNotificationResult").field(&self.0).finish()
    }
}
impl ::core::default::Default for GattCommunicationStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GattCommunicationStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattCommunicationStatus").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for GattDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattDescriptor {}
impl ::core::fmt::Debug for GattDescriptor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattDescriptor").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for GattDescriptorsResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattDescriptorsResult {}
impl ::core::fmt::Debug for GattDescriptorsResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattDescriptorsResult").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for GattDeviceService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattDeviceService {}
impl ::core::fmt::Debug for GattDeviceService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattDeviceService").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for GattDeviceServicesResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattDeviceServicesResult {}
impl ::core::fmt::Debug for GattDeviceServicesResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattDeviceServicesResult").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for GattLocalCharacteristic {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattLocalCharacteristic {}
impl ::core::fmt::Debug for GattLocalCharacteristic {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattLocalCharacteristic").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for GattLocalCharacteristicParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattLocalCharacteristicParameters {}
impl ::core::fmt::Debug for GattLocalCharacteristicParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattLocalCharacteristicParameters").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for GattLocalCharacteristicResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattLocalCharacteristicResult {}
impl ::core::fmt::Debug for GattLocalCharacteristicResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattLocalCharacteristicResult").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for GattLocalDescriptor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattLocalDescriptor {}
impl ::core::fmt::Debug for GattLocalDescriptor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattLocalDescriptor").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for GattLocalDescriptorParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattLocalDescriptorParameters {}
impl ::core::fmt::Debug for GattLocalDescriptorParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattLocalDescriptorParameters").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for GattLocalDescriptorResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattLocalDescriptorResult {}
impl ::core::fmt::Debug for GattLocalDescriptorResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattLocalDescriptorResult").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for GattLocalService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattLocalService {}
impl ::core::fmt::Debug for GattLocalService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattLocalService").field(&self.0).finish()
    }
}
impl ::core::default::Default for GattOpenStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GattOpenStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattOpenStatus").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for GattPresentationFormat {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattPresentationFormat {}
impl ::core::fmt::Debug for GattPresentationFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattPresentationFormat").field(&self.0).finish()
    }
}
impl ::core::default::Default for GattProtectionLevel {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GattProtectionLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattProtectionLevel").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for GattReadClientCharacteristicConfigurationDescriptorResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattReadClientCharacteristicConfigurationDescriptorResult {}
impl ::core::fmt::Debug for GattReadClientCharacteristicConfigurationDescriptorResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattReadClientCharacteristicConfigurationDescriptorResult").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for GattReadRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattReadRequest {}
impl ::core::fmt::Debug for GattReadRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattReadRequest").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for GattReadRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattReadRequestedEventArgs {}
impl ::core::fmt::Debug for GattReadRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattReadRequestedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for GattReadResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattReadResult {}
impl ::core::fmt::Debug for GattReadResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattReadResult").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for GattReliableWriteTransaction {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattReliableWriteTransaction {}
impl ::core::fmt::Debug for GattReliableWriteTransaction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattReliableWriteTransaction").field(&self.0).finish()
    }
}
impl ::core::default::Default for GattRequestState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GattRequestState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattRequestState").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for GattRequestStateChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattRequestStateChangedEventArgs {}
impl ::core::fmt::Debug for GattRequestStateChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattRequestStateChangedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for GattServiceProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattServiceProvider {}
impl ::core::fmt::Debug for GattServiceProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattServiceProvider").field(&self.0).finish()
    }
}
impl ::core::default::Default for GattServiceProviderAdvertisementStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GattServiceProviderAdvertisementStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattServiceProviderAdvertisementStatus").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for GattServiceProviderAdvertisementStatusChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattServiceProviderAdvertisementStatusChangedEventArgs {}
impl ::core::fmt::Debug for GattServiceProviderAdvertisementStatusChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattServiceProviderAdvertisementStatusChangedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for GattServiceProviderAdvertisingParameters {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattServiceProviderAdvertisingParameters {}
impl ::core::fmt::Debug for GattServiceProviderAdvertisingParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattServiceProviderAdvertisingParameters").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for GattServiceProviderResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattServiceProviderResult {}
impl ::core::fmt::Debug for GattServiceProviderResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattServiceProviderResult").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for GattSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattSession {}
impl ::core::fmt::Debug for GattSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattSession").field(&self.0).finish()
    }
}
impl ::core::default::Default for GattSessionStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GattSessionStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattSessionStatus").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for GattSessionStatusChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattSessionStatusChangedEventArgs {}
impl ::core::fmt::Debug for GattSessionStatusChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattSessionStatusChangedEventArgs").field(&self.0).finish()
    }
}
impl ::core::default::Default for GattSharingMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GattSharingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattSharingMode").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for GattSubscribedClient {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattSubscribedClient {}
impl ::core::fmt::Debug for GattSubscribedClient {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattSubscribedClient").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for GattValueChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattValueChangedEventArgs {}
impl ::core::fmt::Debug for GattValueChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattValueChangedEventArgs").field(&self.0).finish()
    }
}
impl ::core::default::Default for GattWriteOption {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GattWriteOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattWriteOption").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for GattWriteRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattWriteRequest {}
impl ::core::fmt::Debug for GattWriteRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattWriteRequest").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for GattWriteRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattWriteRequestedEventArgs {}
impl ::core::fmt::Debug for GattWriteRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattWriteRequestedEventArgs").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for GattWriteResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GattWriteResult {}
impl ::core::fmt::Debug for GattWriteResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GattWriteResult").field(&self.0).finish()
    }
}

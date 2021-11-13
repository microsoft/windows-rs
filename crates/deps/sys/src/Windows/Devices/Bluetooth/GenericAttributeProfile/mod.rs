#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct GattCharacteristic(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GattCharacteristic {}
impl ::core::clone::Clone for GattCharacteristic {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GattCharacteristicProperties(pub u32);
impl GattCharacteristicProperties {
    pub const None: Self = Self(0u32);
    pub const Broadcast: Self = Self(1u32);
    pub const Read: Self = Self(2u32);
    pub const WriteWithoutResponse: Self = Self(4u32);
    pub const Write: Self = Self(8u32);
    pub const Notify: Self = Self(16u32);
    pub const Indicate: Self = Self(32u32);
    pub const AuthenticatedSignedWrites: Self = Self(64u32);
    pub const ExtendedProperties: Self = Self(128u32);
    pub const ReliableWrites: Self = Self(256u32);
    pub const WritableAuxiliaries: Self = Self(512u32);
}
impl ::core::marker::Copy for GattCharacteristicProperties {}
impl ::core::clone::Clone for GattCharacteristicProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GattCharacteristicsResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GattCharacteristicsResult {}
impl ::core::clone::Clone for GattCharacteristicsResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GattClientCharacteristicConfigurationDescriptorValue(pub i32);
impl GattClientCharacteristicConfigurationDescriptorValue {
    pub const None: Self = Self(0i32);
    pub const Notify: Self = Self(1i32);
    pub const Indicate: Self = Self(2i32);
}
impl ::core::marker::Copy for GattClientCharacteristicConfigurationDescriptorValue {}
impl ::core::clone::Clone for GattClientCharacteristicConfigurationDescriptorValue {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GattClientNotificationResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GattClientNotificationResult {}
impl ::core::clone::Clone for GattClientNotificationResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GattCommunicationStatus(pub i32);
impl GattCommunicationStatus {
    pub const Success: Self = Self(0i32);
    pub const Unreachable: Self = Self(1i32);
    pub const ProtocolError: Self = Self(2i32);
    pub const AccessDenied: Self = Self(3i32);
}
impl ::core::marker::Copy for GattCommunicationStatus {}
impl ::core::clone::Clone for GattCommunicationStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GattDescriptor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GattDescriptor {}
impl ::core::clone::Clone for GattDescriptor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GattDescriptorsResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GattDescriptorsResult {}
impl ::core::clone::Clone for GattDescriptorsResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GattDeviceService(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GattDeviceService {}
impl ::core::clone::Clone for GattDeviceService {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GattDeviceServicesResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GattDeviceServicesResult {}
impl ::core::clone::Clone for GattDeviceServicesResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GattLocalCharacteristic(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GattLocalCharacteristic {}
impl ::core::clone::Clone for GattLocalCharacteristic {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GattLocalCharacteristicParameters(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GattLocalCharacteristicParameters {}
impl ::core::clone::Clone for GattLocalCharacteristicParameters {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GattLocalCharacteristicResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GattLocalCharacteristicResult {}
impl ::core::clone::Clone for GattLocalCharacteristicResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GattLocalDescriptor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GattLocalDescriptor {}
impl ::core::clone::Clone for GattLocalDescriptor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GattLocalDescriptorParameters(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GattLocalDescriptorParameters {}
impl ::core::clone::Clone for GattLocalDescriptorParameters {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GattLocalDescriptorResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GattLocalDescriptorResult {}
impl ::core::clone::Clone for GattLocalDescriptorResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GattLocalService(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GattLocalService {}
impl ::core::clone::Clone for GattLocalService {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GattOpenStatus(pub i32);
impl GattOpenStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Success: Self = Self(1i32);
    pub const AlreadyOpened: Self = Self(2i32);
    pub const NotFound: Self = Self(3i32);
    pub const SharingViolation: Self = Self(4i32);
    pub const AccessDenied: Self = Self(5i32);
}
impl ::core::marker::Copy for GattOpenStatus {}
impl ::core::clone::Clone for GattOpenStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GattPresentationFormat(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GattPresentationFormat {}
impl ::core::clone::Clone for GattPresentationFormat {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GattProtectionLevel(pub i32);
impl GattProtectionLevel {
    pub const Plain: Self = Self(0i32);
    pub const AuthenticationRequired: Self = Self(1i32);
    pub const EncryptionRequired: Self = Self(2i32);
    pub const EncryptionAndAuthenticationRequired: Self = Self(3i32);
}
impl ::core::marker::Copy for GattProtectionLevel {}
impl ::core::clone::Clone for GattProtectionLevel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GattReadClientCharacteristicConfigurationDescriptorResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GattReadClientCharacteristicConfigurationDescriptorResult {}
impl ::core::clone::Clone for GattReadClientCharacteristicConfigurationDescriptorResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GattReadRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GattReadRequest {}
impl ::core::clone::Clone for GattReadRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GattReadRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GattReadRequestedEventArgs {}
impl ::core::clone::Clone for GattReadRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GattReadResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GattReadResult {}
impl ::core::clone::Clone for GattReadResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GattReliableWriteTransaction(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GattReliableWriteTransaction {}
impl ::core::clone::Clone for GattReliableWriteTransaction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GattRequestState(pub i32);
impl GattRequestState {
    pub const Pending: Self = Self(0i32);
    pub const Completed: Self = Self(1i32);
    pub const Canceled: Self = Self(2i32);
}
impl ::core::marker::Copy for GattRequestState {}
impl ::core::clone::Clone for GattRequestState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GattRequestStateChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GattRequestStateChangedEventArgs {}
impl ::core::clone::Clone for GattRequestStateChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GattServiceProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GattServiceProvider {}
impl ::core::clone::Clone for GattServiceProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GattServiceProviderAdvertisementStatus(pub i32);
impl GattServiceProviderAdvertisementStatus {
    pub const Created: Self = Self(0i32);
    pub const Stopped: Self = Self(1i32);
    pub const Started: Self = Self(2i32);
    pub const Aborted: Self = Self(3i32);
    pub const StartedWithoutAllAdvertisementData: Self = Self(4i32);
}
impl ::core::marker::Copy for GattServiceProviderAdvertisementStatus {}
impl ::core::clone::Clone for GattServiceProviderAdvertisementStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GattServiceProviderAdvertisementStatusChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GattServiceProviderAdvertisementStatusChangedEventArgs {}
impl ::core::clone::Clone for GattServiceProviderAdvertisementStatusChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GattServiceProviderAdvertisingParameters(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GattServiceProviderAdvertisingParameters {}
impl ::core::clone::Clone for GattServiceProviderAdvertisingParameters {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GattServiceProviderResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GattServiceProviderResult {}
impl ::core::clone::Clone for GattServiceProviderResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GattSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GattSession {}
impl ::core::clone::Clone for GattSession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GattSessionStatus(pub i32);
impl GattSessionStatus {
    pub const Closed: Self = Self(0i32);
    pub const Active: Self = Self(1i32);
}
impl ::core::marker::Copy for GattSessionStatus {}
impl ::core::clone::Clone for GattSessionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GattSessionStatusChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GattSessionStatusChangedEventArgs {}
impl ::core::clone::Clone for GattSessionStatusChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GattSharingMode(pub i32);
impl GattSharingMode {
    pub const Unspecified: Self = Self(0i32);
    pub const Exclusive: Self = Self(1i32);
    pub const SharedReadOnly: Self = Self(2i32);
    pub const SharedReadAndWrite: Self = Self(3i32);
}
impl ::core::marker::Copy for GattSharingMode {}
impl ::core::clone::Clone for GattSharingMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GattSubscribedClient(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GattSubscribedClient {}
impl ::core::clone::Clone for GattSubscribedClient {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GattValueChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GattValueChangedEventArgs {}
impl ::core::clone::Clone for GattValueChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GattWriteOption(pub i32);
impl GattWriteOption {
    pub const WriteWithResponse: Self = Self(0i32);
    pub const WriteWithoutResponse: Self = Self(1i32);
}
impl ::core::marker::Copy for GattWriteOption {}
impl ::core::clone::Clone for GattWriteOption {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GattWriteRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GattWriteRequest {}
impl ::core::clone::Clone for GattWriteRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GattWriteRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GattWriteRequestedEventArgs {}
impl ::core::clone::Clone for GattWriteRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GattWriteResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GattWriteResult {}
impl ::core::clone::Clone for GattWriteResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattCharacteristic(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattCharacteristic {}
impl ::core::clone::Clone for IGattCharacteristic {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattCharacteristic2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattCharacteristic2 {}
impl ::core::clone::Clone for IGattCharacteristic2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattCharacteristic3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattCharacteristic3 {}
impl ::core::clone::Clone for IGattCharacteristic3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattCharacteristicStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattCharacteristicStatics {}
impl ::core::clone::Clone for IGattCharacteristicStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattCharacteristicUuidsStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattCharacteristicUuidsStatics {}
impl ::core::clone::Clone for IGattCharacteristicUuidsStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattCharacteristicUuidsStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattCharacteristicUuidsStatics2 {}
impl ::core::clone::Clone for IGattCharacteristicUuidsStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattCharacteristicsResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattCharacteristicsResult {}
impl ::core::clone::Clone for IGattCharacteristicsResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattClientNotificationResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattClientNotificationResult {}
impl ::core::clone::Clone for IGattClientNotificationResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattClientNotificationResult2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattClientNotificationResult2 {}
impl ::core::clone::Clone for IGattClientNotificationResult2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattDescriptor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattDescriptor {}
impl ::core::clone::Clone for IGattDescriptor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattDescriptor2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattDescriptor2 {}
impl ::core::clone::Clone for IGattDescriptor2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattDescriptorStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattDescriptorStatics {}
impl ::core::clone::Clone for IGattDescriptorStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattDescriptorUuidsStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattDescriptorUuidsStatics {}
impl ::core::clone::Clone for IGattDescriptorUuidsStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattDescriptorsResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattDescriptorsResult {}
impl ::core::clone::Clone for IGattDescriptorsResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattDeviceService(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattDeviceService {}
impl ::core::clone::Clone for IGattDeviceService {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattDeviceService2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattDeviceService2 {}
impl ::core::clone::Clone for IGattDeviceService2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattDeviceService3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattDeviceService3 {}
impl ::core::clone::Clone for IGattDeviceService3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattDeviceServiceStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattDeviceServiceStatics {}
impl ::core::clone::Clone for IGattDeviceServiceStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattDeviceServiceStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattDeviceServiceStatics2 {}
impl ::core::clone::Clone for IGattDeviceServiceStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattDeviceServicesResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattDeviceServicesResult {}
impl ::core::clone::Clone for IGattDeviceServicesResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattLocalCharacteristic(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattLocalCharacteristic {}
impl ::core::clone::Clone for IGattLocalCharacteristic {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattLocalCharacteristicParameters(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattLocalCharacteristicParameters {}
impl ::core::clone::Clone for IGattLocalCharacteristicParameters {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattLocalCharacteristicResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattLocalCharacteristicResult {}
impl ::core::clone::Clone for IGattLocalCharacteristicResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattLocalDescriptor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattLocalDescriptor {}
impl ::core::clone::Clone for IGattLocalDescriptor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattLocalDescriptorParameters(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattLocalDescriptorParameters {}
impl ::core::clone::Clone for IGattLocalDescriptorParameters {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattLocalDescriptorResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattLocalDescriptorResult {}
impl ::core::clone::Clone for IGattLocalDescriptorResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattLocalService(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattLocalService {}
impl ::core::clone::Clone for IGattLocalService {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattPresentationFormat(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattPresentationFormat {}
impl ::core::clone::Clone for IGattPresentationFormat {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattPresentationFormatStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattPresentationFormatStatics {}
impl ::core::clone::Clone for IGattPresentationFormatStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattPresentationFormatStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattPresentationFormatStatics2 {}
impl ::core::clone::Clone for IGattPresentationFormatStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattPresentationFormatTypesStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattPresentationFormatTypesStatics {}
impl ::core::clone::Clone for IGattPresentationFormatTypesStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattProtocolErrorStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattProtocolErrorStatics {}
impl ::core::clone::Clone for IGattProtocolErrorStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattReadClientCharacteristicConfigurationDescriptorResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattReadClientCharacteristicConfigurationDescriptorResult {}
impl ::core::clone::Clone for IGattReadClientCharacteristicConfigurationDescriptorResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattReadClientCharacteristicConfigurationDescriptorResult2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattReadClientCharacteristicConfigurationDescriptorResult2 {}
impl ::core::clone::Clone for IGattReadClientCharacteristicConfigurationDescriptorResult2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattReadRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattReadRequest {}
impl ::core::clone::Clone for IGattReadRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattReadRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattReadRequestedEventArgs {}
impl ::core::clone::Clone for IGattReadRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattReadResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattReadResult {}
impl ::core::clone::Clone for IGattReadResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattReadResult2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattReadResult2 {}
impl ::core::clone::Clone for IGattReadResult2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattReliableWriteTransaction(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattReliableWriteTransaction {}
impl ::core::clone::Clone for IGattReliableWriteTransaction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattReliableWriteTransaction2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattReliableWriteTransaction2 {}
impl ::core::clone::Clone for IGattReliableWriteTransaction2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattRequestStateChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattRequestStateChangedEventArgs {}
impl ::core::clone::Clone for IGattRequestStateChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattServiceProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattServiceProvider {}
impl ::core::clone::Clone for IGattServiceProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattServiceProviderAdvertisementStatusChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattServiceProviderAdvertisementStatusChangedEventArgs {}
impl ::core::clone::Clone for IGattServiceProviderAdvertisementStatusChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattServiceProviderAdvertisingParameters(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattServiceProviderAdvertisingParameters {}
impl ::core::clone::Clone for IGattServiceProviderAdvertisingParameters {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattServiceProviderAdvertisingParameters2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattServiceProviderAdvertisingParameters2 {}
impl ::core::clone::Clone for IGattServiceProviderAdvertisingParameters2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattServiceProviderResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattServiceProviderResult {}
impl ::core::clone::Clone for IGattServiceProviderResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattServiceProviderStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattServiceProviderStatics {}
impl ::core::clone::Clone for IGattServiceProviderStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattServiceUuidsStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattServiceUuidsStatics {}
impl ::core::clone::Clone for IGattServiceUuidsStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattServiceUuidsStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattServiceUuidsStatics2 {}
impl ::core::clone::Clone for IGattServiceUuidsStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattSession {}
impl ::core::clone::Clone for IGattSession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattSessionStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattSessionStatics {}
impl ::core::clone::Clone for IGattSessionStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattSessionStatusChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattSessionStatusChangedEventArgs {}
impl ::core::clone::Clone for IGattSessionStatusChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattSubscribedClient(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattSubscribedClient {}
impl ::core::clone::Clone for IGattSubscribedClient {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattValueChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattValueChangedEventArgs {}
impl ::core::clone::Clone for IGattValueChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattWriteRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattWriteRequest {}
impl ::core::clone::Clone for IGattWriteRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattWriteRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattWriteRequestedEventArgs {}
impl ::core::clone::Clone for IGattWriteRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattWriteResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattWriteResult {}
impl ::core::clone::Clone for IGattWriteResult {
    fn clone(&self) -> Self {
        *self
    }
}

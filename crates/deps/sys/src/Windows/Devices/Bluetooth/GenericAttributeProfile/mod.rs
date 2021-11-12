#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct GattCharacteristic(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GattCharacteristicProperties(pub u32);
impl GattCharacteristicProperties {
    pub const None: GattCharacteristicProperties = GattCharacteristicProperties(0u32);
    pub const Broadcast: GattCharacteristicProperties = GattCharacteristicProperties(1u32);
    pub const Read: GattCharacteristicProperties = GattCharacteristicProperties(2u32);
    pub const WriteWithoutResponse: GattCharacteristicProperties = GattCharacteristicProperties(4u32);
    pub const Write: GattCharacteristicProperties = GattCharacteristicProperties(8u32);
    pub const Notify: GattCharacteristicProperties = GattCharacteristicProperties(16u32);
    pub const Indicate: GattCharacteristicProperties = GattCharacteristicProperties(32u32);
    pub const AuthenticatedSignedWrites: GattCharacteristicProperties = GattCharacteristicProperties(64u32);
    pub const ExtendedProperties: GattCharacteristicProperties = GattCharacteristicProperties(128u32);
    pub const ReliableWrites: GattCharacteristicProperties = GattCharacteristicProperties(256u32);
    pub const WritableAuxiliaries: GattCharacteristicProperties = GattCharacteristicProperties(512u32);
}
#[repr(transparent)]
pub struct GattCharacteristicsResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GattClientCharacteristicConfigurationDescriptorValue(pub i32);
impl GattClientCharacteristicConfigurationDescriptorValue {
    pub const None: GattClientCharacteristicConfigurationDescriptorValue = GattClientCharacteristicConfigurationDescriptorValue(0i32);
    pub const Notify: GattClientCharacteristicConfigurationDescriptorValue = GattClientCharacteristicConfigurationDescriptorValue(1i32);
    pub const Indicate: GattClientCharacteristicConfigurationDescriptorValue = GattClientCharacteristicConfigurationDescriptorValue(2i32);
}
#[repr(transparent)]
pub struct GattClientNotificationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GattCommunicationStatus(pub i32);
impl GattCommunicationStatus {
    pub const Success: GattCommunicationStatus = GattCommunicationStatus(0i32);
    pub const Unreachable: GattCommunicationStatus = GattCommunicationStatus(1i32);
    pub const ProtocolError: GattCommunicationStatus = GattCommunicationStatus(2i32);
    pub const AccessDenied: GattCommunicationStatus = GattCommunicationStatus(3i32);
}
#[repr(transparent)]
pub struct GattDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GattDescriptorsResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GattDeviceService(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GattDeviceServicesResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GattLocalCharacteristic(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GattLocalCharacteristicParameters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GattLocalCharacteristicResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GattLocalDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GattLocalDescriptorParameters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GattLocalDescriptorResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GattLocalService(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GattOpenStatus(pub i32);
impl GattOpenStatus {
    pub const Unspecified: GattOpenStatus = GattOpenStatus(0i32);
    pub const Success: GattOpenStatus = GattOpenStatus(1i32);
    pub const AlreadyOpened: GattOpenStatus = GattOpenStatus(2i32);
    pub const NotFound: GattOpenStatus = GattOpenStatus(3i32);
    pub const SharingViolation: GattOpenStatus = GattOpenStatus(4i32);
    pub const AccessDenied: GattOpenStatus = GattOpenStatus(5i32);
}
#[repr(transparent)]
pub struct GattPresentationFormat(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GattProtectionLevel(pub i32);
impl GattProtectionLevel {
    pub const Plain: GattProtectionLevel = GattProtectionLevel(0i32);
    pub const AuthenticationRequired: GattProtectionLevel = GattProtectionLevel(1i32);
    pub const EncryptionRequired: GattProtectionLevel = GattProtectionLevel(2i32);
    pub const EncryptionAndAuthenticationRequired: GattProtectionLevel = GattProtectionLevel(3i32);
}
#[repr(transparent)]
pub struct GattReadClientCharacteristicConfigurationDescriptorResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GattReadRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GattReadRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GattReadResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GattReliableWriteTransaction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GattRequestState(pub i32);
impl GattRequestState {
    pub const Pending: GattRequestState = GattRequestState(0i32);
    pub const Completed: GattRequestState = GattRequestState(1i32);
    pub const Canceled: GattRequestState = GattRequestState(2i32);
}
#[repr(transparent)]
pub struct GattRequestStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GattServiceProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GattServiceProviderAdvertisementStatus(pub i32);
impl GattServiceProviderAdvertisementStatus {
    pub const Created: GattServiceProviderAdvertisementStatus = GattServiceProviderAdvertisementStatus(0i32);
    pub const Stopped: GattServiceProviderAdvertisementStatus = GattServiceProviderAdvertisementStatus(1i32);
    pub const Started: GattServiceProviderAdvertisementStatus = GattServiceProviderAdvertisementStatus(2i32);
    pub const Aborted: GattServiceProviderAdvertisementStatus = GattServiceProviderAdvertisementStatus(3i32);
    pub const StartedWithoutAllAdvertisementData: GattServiceProviderAdvertisementStatus = GattServiceProviderAdvertisementStatus(4i32);
}
#[repr(transparent)]
pub struct GattServiceProviderAdvertisementStatusChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GattServiceProviderAdvertisingParameters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GattServiceProviderResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GattSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GattSessionStatus(pub i32);
impl GattSessionStatus {
    pub const Closed: GattSessionStatus = GattSessionStatus(0i32);
    pub const Active: GattSessionStatus = GattSessionStatus(1i32);
}
#[repr(transparent)]
pub struct GattSessionStatusChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GattSharingMode(pub i32);
impl GattSharingMode {
    pub const Unspecified: GattSharingMode = GattSharingMode(0i32);
    pub const Exclusive: GattSharingMode = GattSharingMode(1i32);
    pub const SharedReadOnly: GattSharingMode = GattSharingMode(2i32);
    pub const SharedReadAndWrite: GattSharingMode = GattSharingMode(3i32);
}
#[repr(transparent)]
pub struct GattSubscribedClient(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GattValueChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GattWriteOption(pub i32);
impl GattWriteOption {
    pub const WriteWithResponse: GattWriteOption = GattWriteOption(0i32);
    pub const WriteWithoutResponse: GattWriteOption = GattWriteOption(1i32);
}
#[repr(transparent)]
pub struct GattWriteRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GattWriteRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GattWriteResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattCharacteristic(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattCharacteristic2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattCharacteristic3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattCharacteristicStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattCharacteristicUuidsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattCharacteristicUuidsStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattCharacteristicsResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattClientNotificationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattClientNotificationResult2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattDescriptor2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattDescriptorStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattDescriptorUuidsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattDescriptorsResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattDeviceService(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattDeviceService2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattDeviceService3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattDeviceServiceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattDeviceServiceStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattDeviceServicesResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattLocalCharacteristic(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattLocalCharacteristicParameters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattLocalCharacteristicResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattLocalDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattLocalDescriptorParameters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattLocalDescriptorResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattLocalService(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattPresentationFormat(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattPresentationFormatStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattPresentationFormatStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattPresentationFormatTypesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattProtocolErrorStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattReadClientCharacteristicConfigurationDescriptorResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattReadClientCharacteristicConfigurationDescriptorResult2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattReadRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattReadRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattReadResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattReadResult2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattReliableWriteTransaction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattReliableWriteTransaction2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattRequestStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattServiceProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattServiceProviderAdvertisementStatusChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattServiceProviderAdvertisingParameters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattServiceProviderAdvertisingParameters2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattServiceProviderResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattServiceProviderStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattServiceUuidsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattServiceUuidsStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattSessionStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattSessionStatusChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattSubscribedClient(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattValueChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattWriteRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattWriteRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattWriteResult(pub *mut ::core::ffi::c_void);

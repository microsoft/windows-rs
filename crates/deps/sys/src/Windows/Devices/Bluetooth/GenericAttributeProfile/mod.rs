#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct GattCharacteristic(pub *mut ::core::ffi::c_void);
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
#[repr(transparent)]
pub struct GattCharacteristicsResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GattClientCharacteristicConfigurationDescriptorValue(pub i32);
impl GattClientCharacteristicConfigurationDescriptorValue {
    pub const None: Self = Self(0i32);
    pub const Notify: Self = Self(1i32);
    pub const Indicate: Self = Self(2i32);
}
#[repr(transparent)]
pub struct GattClientNotificationResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GattCommunicationStatus(pub i32);
impl GattCommunicationStatus {
    pub const Success: Self = Self(0i32);
    pub const Unreachable: Self = Self(1i32);
    pub const ProtocolError: Self = Self(2i32);
    pub const AccessDenied: Self = Self(3i32);
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
    pub const Unspecified: Self = Self(0i32);
    pub const Success: Self = Self(1i32);
    pub const AlreadyOpened: Self = Self(2i32);
    pub const NotFound: Self = Self(3i32);
    pub const SharingViolation: Self = Self(4i32);
    pub const AccessDenied: Self = Self(5i32);
}
#[repr(transparent)]
pub struct GattPresentationFormat(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GattProtectionLevel(pub i32);
impl GattProtectionLevel {
    pub const Plain: Self = Self(0i32);
    pub const AuthenticationRequired: Self = Self(1i32);
    pub const EncryptionRequired: Self = Self(2i32);
    pub const EncryptionAndAuthenticationRequired: Self = Self(3i32);
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
    pub const Pending: Self = Self(0i32);
    pub const Completed: Self = Self(1i32);
    pub const Canceled: Self = Self(2i32);
}
#[repr(transparent)]
pub struct GattRequestStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GattServiceProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GattServiceProviderAdvertisementStatus(pub i32);
impl GattServiceProviderAdvertisementStatus {
    pub const Created: Self = Self(0i32);
    pub const Stopped: Self = Self(1i32);
    pub const Started: Self = Self(2i32);
    pub const Aborted: Self = Self(3i32);
    pub const StartedWithoutAllAdvertisementData: Self = Self(4i32);
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
    pub const Closed: Self = Self(0i32);
    pub const Active: Self = Self(1i32);
}
#[repr(transparent)]
pub struct GattSessionStatusChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GattSharingMode(pub i32);
impl GattSharingMode {
    pub const Unspecified: Self = Self(0i32);
    pub const Exclusive: Self = Self(1i32);
    pub const SharedReadOnly: Self = Self(2i32);
    pub const SharedReadAndWrite: Self = Self(3i32);
}
#[repr(transparent)]
pub struct GattSubscribedClient(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GattValueChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GattWriteOption(pub i32);
impl GattWriteOption {
    pub const WriteWithResponse: Self = Self(0i32);
    pub const WriteWithoutResponse: Self = Self(1i32);
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

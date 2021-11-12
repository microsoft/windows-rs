#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct GattCharacteristic(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct GattCharacteristicProperties(i32);
#[repr(transparent)]
pub struct GattCharacteristicUuids(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GattCharacteristicsResult(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct GattClientCharacteristicConfigurationDescriptorValue(i32);
#[repr(transparent)]
pub struct GattClientNotificationResult(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct GattCommunicationStatus(i32);
#[repr(transparent)]
pub struct GattDescriptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GattDescriptorUuids(pub *mut ::core::ffi::c_void);
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
#[repr(C)]
pub struct GattOpenStatus(i32);
#[repr(transparent)]
pub struct GattPresentationFormat(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GattPresentationFormatTypes(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct GattProtectionLevel(i32);
#[repr(transparent)]
pub struct GattProtocolError(pub *mut ::core::ffi::c_void);
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
#[repr(C)]
pub struct GattRequestState(i32);
#[repr(transparent)]
pub struct GattRequestStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GattServiceProvider(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct GattServiceProviderAdvertisementStatus(i32);
#[repr(transparent)]
pub struct GattServiceProviderAdvertisementStatusChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GattServiceProviderAdvertisingParameters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GattServiceProviderResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GattServiceUuids(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GattSession(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct GattSessionStatus(i32);
#[repr(transparent)]
pub struct GattSessionStatusChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct GattSharingMode(i32);
#[repr(transparent)]
pub struct GattSubscribedClient(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GattValueChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct GattWriteOption(i32);
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

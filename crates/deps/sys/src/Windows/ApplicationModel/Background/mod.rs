#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ActivitySensorTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ActivitySensorTrigger {}
impl ::core::clone::Clone for ActivitySensorTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AlarmAccessStatus(pub i32);
impl AlarmAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const AllowedWithWakeupCapability: Self = Self(1i32);
    pub const AllowedWithoutWakeupCapability: Self = Self(2i32);
    pub const Denied: Self = Self(3i32);
}
impl ::core::marker::Copy for AlarmAccessStatus {}
impl ::core::clone::Clone for AlarmAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBroadcastTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppBroadcastTrigger {}
impl ::core::clone::Clone for AppBroadcastTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppBroadcastTriggerProviderInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppBroadcastTriggerProviderInfo {}
impl ::core::clone::Clone for AppBroadcastTriggerProviderInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ApplicationTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ApplicationTrigger {}
impl ::core::clone::Clone for ApplicationTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ApplicationTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ApplicationTriggerDetails {}
impl ::core::clone::Clone for ApplicationTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ApplicationTriggerResult(pub i32);
impl ApplicationTriggerResult {
    pub const Allowed: Self = Self(0i32);
    pub const CurrentlyRunning: Self = Self(1i32);
    pub const DisabledByPolicy: Self = Self(2i32);
    pub const UnknownError: Self = Self(3i32);
}
impl ::core::marker::Copy for ApplicationTriggerResult {}
impl ::core::clone::Clone for ApplicationTriggerResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppointmentStoreNotificationTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppointmentStoreNotificationTrigger {}
impl ::core::clone::Clone for AppointmentStoreNotificationTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BackgroundAccessRequestKind(pub i32);
impl BackgroundAccessRequestKind {
    pub const AlwaysAllowed: Self = Self(0i32);
    pub const AllowedSubjectToSystemPolicy: Self = Self(1i32);
}
impl ::core::marker::Copy for BackgroundAccessRequestKind {}
impl ::core::clone::Clone for BackgroundAccessRequestKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BackgroundAccessStatus(pub i32);
impl BackgroundAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const AllowedWithAlwaysOnRealTimeConnectivity: Self = Self(1i32);
    pub const AllowedMayUseActiveRealTimeConnectivity: Self = Self(2i32);
    pub const Denied: Self = Self(3i32);
    pub const AlwaysAllowed: Self = Self(4i32);
    pub const AllowedSubjectToSystemPolicy: Self = Self(5i32);
    pub const DeniedBySystemPolicy: Self = Self(6i32);
    pub const DeniedByUser: Self = Self(7i32);
}
impl ::core::marker::Copy for BackgroundAccessStatus {}
impl ::core::clone::Clone for BackgroundAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BackgroundTaskBuilder(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BackgroundTaskBuilder {}
impl ::core::clone::Clone for BackgroundTaskBuilder {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BackgroundTaskCanceledEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BackgroundTaskCanceledEventHandler {}
impl ::core::clone::Clone for BackgroundTaskCanceledEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BackgroundTaskCancellationReason(pub i32);
impl BackgroundTaskCancellationReason {
    pub const Abort: Self = Self(0i32);
    pub const Terminating: Self = Self(1i32);
    pub const LoggingOff: Self = Self(2i32);
    pub const ServicingUpdate: Self = Self(3i32);
    pub const IdleTask: Self = Self(4i32);
    pub const Uninstall: Self = Self(5i32);
    pub const ConditionLoss: Self = Self(6i32);
    pub const SystemPolicy: Self = Self(7i32);
    pub const QuietHoursEntered: Self = Self(8i32);
    pub const ExecutionTimeExceeded: Self = Self(9i32);
    pub const ResourceRevocation: Self = Self(10i32);
    pub const EnergySaver: Self = Self(11i32);
}
impl ::core::marker::Copy for BackgroundTaskCancellationReason {}
impl ::core::clone::Clone for BackgroundTaskCancellationReason {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BackgroundTaskCompletedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BackgroundTaskCompletedEventArgs {}
impl ::core::clone::Clone for BackgroundTaskCompletedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BackgroundTaskCompletedEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BackgroundTaskCompletedEventHandler {}
impl ::core::clone::Clone for BackgroundTaskCompletedEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BackgroundTaskDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BackgroundTaskDeferral {}
impl ::core::clone::Clone for BackgroundTaskDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BackgroundTaskProgressEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BackgroundTaskProgressEventArgs {}
impl ::core::clone::Clone for BackgroundTaskProgressEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BackgroundTaskProgressEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BackgroundTaskProgressEventHandler {}
impl ::core::clone::Clone for BackgroundTaskProgressEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BackgroundTaskRegistration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BackgroundTaskRegistration {}
impl ::core::clone::Clone for BackgroundTaskRegistration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BackgroundTaskRegistrationGroup(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BackgroundTaskRegistrationGroup {}
impl ::core::clone::Clone for BackgroundTaskRegistrationGroup {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BackgroundTaskThrottleCounter(pub i32);
impl BackgroundTaskThrottleCounter {
    pub const All: Self = Self(0i32);
    pub const Cpu: Self = Self(1i32);
    pub const Network: Self = Self(2i32);
}
impl ::core::marker::Copy for BackgroundTaskThrottleCounter {}
impl ::core::clone::Clone for BackgroundTaskThrottleCounter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BackgroundWorkCostValue(pub i32);
impl BackgroundWorkCostValue {
    pub const Low: Self = Self(0i32);
    pub const Medium: Self = Self(1i32);
    pub const High: Self = Self(2i32);
}
impl ::core::marker::Copy for BackgroundWorkCostValue {}
impl ::core::clone::Clone for BackgroundWorkCostValue {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BluetoothLEAdvertisementPublisherTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BluetoothLEAdvertisementPublisherTrigger {}
impl ::core::clone::Clone for BluetoothLEAdvertisementPublisherTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BluetoothLEAdvertisementWatcherTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for BluetoothLEAdvertisementWatcherTrigger {}
impl ::core::clone::Clone for BluetoothLEAdvertisementWatcherTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CachedFileUpdaterTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CachedFileUpdaterTrigger {}
impl ::core::clone::Clone for CachedFileUpdaterTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CachedFileUpdaterTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CachedFileUpdaterTriggerDetails {}
impl ::core::clone::Clone for CachedFileUpdaterTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ChatMessageNotificationTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ChatMessageNotificationTrigger {}
impl ::core::clone::Clone for ChatMessageNotificationTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ChatMessageReceivedNotificationTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ChatMessageReceivedNotificationTrigger {}
impl ::core::clone::Clone for ChatMessageReceivedNotificationTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CommunicationBlockingAppSetAsActiveTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CommunicationBlockingAppSetAsActiveTrigger {}
impl ::core::clone::Clone for CommunicationBlockingAppSetAsActiveTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContactStoreNotificationTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContactStoreNotificationTrigger {}
impl ::core::clone::Clone for ContactStoreNotificationTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ContentPrefetchTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ContentPrefetchTrigger {}
impl ::core::clone::Clone for ContentPrefetchTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ConversationalAgentTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ConversationalAgentTrigger {}
impl ::core::clone::Clone for ConversationalAgentTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CustomSystemEventTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CustomSystemEventTrigger {}
impl ::core::clone::Clone for CustomSystemEventTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CustomSystemEventTriggerRecurrence(pub i32);
impl CustomSystemEventTriggerRecurrence {
    pub const Once: Self = Self(0i32);
    pub const Always: Self = Self(1i32);
}
impl ::core::marker::Copy for CustomSystemEventTriggerRecurrence {}
impl ::core::clone::Clone for CustomSystemEventTriggerRecurrence {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DeviceConnectionChangeTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DeviceConnectionChangeTrigger {}
impl ::core::clone::Clone for DeviceConnectionChangeTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DeviceManufacturerNotificationTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DeviceManufacturerNotificationTrigger {}
impl ::core::clone::Clone for DeviceManufacturerNotificationTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DeviceServicingTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DeviceServicingTrigger {}
impl ::core::clone::Clone for DeviceServicingTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DeviceTriggerResult(pub i32);
impl DeviceTriggerResult {
    pub const Allowed: Self = Self(0i32);
    pub const DeniedByUser: Self = Self(1i32);
    pub const DeniedBySystem: Self = Self(2i32);
    pub const LowBattery: Self = Self(3i32);
}
impl ::core::marker::Copy for DeviceTriggerResult {}
impl ::core::clone::Clone for DeviceTriggerResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DeviceUseTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DeviceUseTrigger {}
impl ::core::clone::Clone for DeviceUseTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DeviceWatcherTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DeviceWatcherTrigger {}
impl ::core::clone::Clone for DeviceWatcherTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct EmailStoreNotificationTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for EmailStoreNotificationTrigger {}
impl ::core::clone::Clone for EmailStoreNotificationTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GattCharacteristicNotificationTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GattCharacteristicNotificationTrigger {}
impl ::core::clone::Clone for GattCharacteristicNotificationTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GattServiceProviderTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GattServiceProviderTrigger {}
impl ::core::clone::Clone for GattServiceProviderTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GattServiceProviderTriggerResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GattServiceProviderTriggerResult {}
impl ::core::clone::Clone for GattServiceProviderTriggerResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GeovisitTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GeovisitTrigger {}
impl ::core::clone::Clone for GeovisitTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IActivitySensorTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IActivitySensorTrigger {}
impl ::core::clone::Clone for IActivitySensorTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IActivitySensorTriggerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IActivitySensorTriggerFactory {}
impl ::core::clone::Clone for IActivitySensorTriggerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAlarmApplicationManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAlarmApplicationManagerStatics {}
impl ::core::clone::Clone for IAlarmApplicationManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBroadcastTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBroadcastTrigger {}
impl ::core::clone::Clone for IAppBroadcastTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBroadcastTriggerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBroadcastTriggerFactory {}
impl ::core::clone::Clone for IAppBroadcastTriggerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppBroadcastTriggerProviderInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppBroadcastTriggerProviderInfo {}
impl ::core::clone::Clone for IAppBroadcastTriggerProviderInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IApplicationTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IApplicationTrigger {}
impl ::core::clone::Clone for IApplicationTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IApplicationTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IApplicationTriggerDetails {}
impl ::core::clone::Clone for IApplicationTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppointmentStoreNotificationTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppointmentStoreNotificationTrigger {}
impl ::core::clone::Clone for IAppointmentStoreNotificationTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBackgroundCondition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBackgroundCondition {}
impl ::core::clone::Clone for IBackgroundCondition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBackgroundExecutionManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBackgroundExecutionManagerStatics {}
impl ::core::clone::Clone for IBackgroundExecutionManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBackgroundExecutionManagerStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBackgroundExecutionManagerStatics2 {}
impl ::core::clone::Clone for IBackgroundExecutionManagerStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBackgroundExecutionManagerStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBackgroundExecutionManagerStatics3 {}
impl ::core::clone::Clone for IBackgroundExecutionManagerStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBackgroundTask(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBackgroundTask {}
impl ::core::clone::Clone for IBackgroundTask {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBackgroundTaskBuilder(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBackgroundTaskBuilder {}
impl ::core::clone::Clone for IBackgroundTaskBuilder {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBackgroundTaskBuilder2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBackgroundTaskBuilder2 {}
impl ::core::clone::Clone for IBackgroundTaskBuilder2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBackgroundTaskBuilder3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBackgroundTaskBuilder3 {}
impl ::core::clone::Clone for IBackgroundTaskBuilder3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBackgroundTaskBuilder4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBackgroundTaskBuilder4 {}
impl ::core::clone::Clone for IBackgroundTaskBuilder4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBackgroundTaskBuilder5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBackgroundTaskBuilder5 {}
impl ::core::clone::Clone for IBackgroundTaskBuilder5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBackgroundTaskCompletedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBackgroundTaskCompletedEventArgs {}
impl ::core::clone::Clone for IBackgroundTaskCompletedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBackgroundTaskDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBackgroundTaskDeferral {}
impl ::core::clone::Clone for IBackgroundTaskDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBackgroundTaskInstance(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBackgroundTaskInstance {}
impl ::core::clone::Clone for IBackgroundTaskInstance {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBackgroundTaskInstance2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBackgroundTaskInstance2 {}
impl ::core::clone::Clone for IBackgroundTaskInstance2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBackgroundTaskInstance4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBackgroundTaskInstance4 {}
impl ::core::clone::Clone for IBackgroundTaskInstance4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBackgroundTaskProgressEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBackgroundTaskProgressEventArgs {}
impl ::core::clone::Clone for IBackgroundTaskProgressEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBackgroundTaskRegistration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBackgroundTaskRegistration {}
impl ::core::clone::Clone for IBackgroundTaskRegistration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBackgroundTaskRegistration2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBackgroundTaskRegistration2 {}
impl ::core::clone::Clone for IBackgroundTaskRegistration2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBackgroundTaskRegistration3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBackgroundTaskRegistration3 {}
impl ::core::clone::Clone for IBackgroundTaskRegistration3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBackgroundTaskRegistrationGroup(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBackgroundTaskRegistrationGroup {}
impl ::core::clone::Clone for IBackgroundTaskRegistrationGroup {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBackgroundTaskRegistrationGroupFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBackgroundTaskRegistrationGroupFactory {}
impl ::core::clone::Clone for IBackgroundTaskRegistrationGroupFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBackgroundTaskRegistrationStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBackgroundTaskRegistrationStatics {}
impl ::core::clone::Clone for IBackgroundTaskRegistrationStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBackgroundTaskRegistrationStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBackgroundTaskRegistrationStatics2 {}
impl ::core::clone::Clone for IBackgroundTaskRegistrationStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBackgroundTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBackgroundTrigger {}
impl ::core::clone::Clone for IBackgroundTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBackgroundWorkCostStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBackgroundWorkCostStatics {}
impl ::core::clone::Clone for IBackgroundWorkCostStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementPublisherTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothLEAdvertisementPublisherTrigger {}
impl ::core::clone::Clone for IBluetoothLEAdvertisementPublisherTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementPublisherTrigger2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothLEAdvertisementPublisherTrigger2 {}
impl ::core::clone::Clone for IBluetoothLEAdvertisementPublisherTrigger2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementWatcherTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothLEAdvertisementWatcherTrigger {}
impl ::core::clone::Clone for IBluetoothLEAdvertisementWatcherTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementWatcherTrigger2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBluetoothLEAdvertisementWatcherTrigger2 {}
impl ::core::clone::Clone for IBluetoothLEAdvertisementWatcherTrigger2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICachedFileUpdaterTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICachedFileUpdaterTrigger {}
impl ::core::clone::Clone for ICachedFileUpdaterTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICachedFileUpdaterTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICachedFileUpdaterTriggerDetails {}
impl ::core::clone::Clone for ICachedFileUpdaterTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IChatMessageNotificationTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IChatMessageNotificationTrigger {}
impl ::core::clone::Clone for IChatMessageNotificationTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IChatMessageReceivedNotificationTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IChatMessageReceivedNotificationTrigger {}
impl ::core::clone::Clone for IChatMessageReceivedNotificationTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICommunicationBlockingAppSetAsActiveTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICommunicationBlockingAppSetAsActiveTrigger {}
impl ::core::clone::Clone for ICommunicationBlockingAppSetAsActiveTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContactStoreNotificationTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContactStoreNotificationTrigger {}
impl ::core::clone::Clone for IContactStoreNotificationTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContentPrefetchTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContentPrefetchTrigger {}
impl ::core::clone::Clone for IContentPrefetchTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IContentPrefetchTriggerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IContentPrefetchTriggerFactory {}
impl ::core::clone::Clone for IContentPrefetchTriggerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICustomSystemEventTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICustomSystemEventTrigger {}
impl ::core::clone::Clone for ICustomSystemEventTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICustomSystemEventTriggerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICustomSystemEventTriggerFactory {}
impl ::core::clone::Clone for ICustomSystemEventTriggerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDeviceConnectionChangeTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDeviceConnectionChangeTrigger {}
impl ::core::clone::Clone for IDeviceConnectionChangeTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDeviceConnectionChangeTriggerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDeviceConnectionChangeTriggerStatics {}
impl ::core::clone::Clone for IDeviceConnectionChangeTriggerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDeviceManufacturerNotificationTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDeviceManufacturerNotificationTrigger {}
impl ::core::clone::Clone for IDeviceManufacturerNotificationTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDeviceManufacturerNotificationTriggerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDeviceManufacturerNotificationTriggerFactory {}
impl ::core::clone::Clone for IDeviceManufacturerNotificationTriggerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDeviceServicingTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDeviceServicingTrigger {}
impl ::core::clone::Clone for IDeviceServicingTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDeviceUseTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDeviceUseTrigger {}
impl ::core::clone::Clone for IDeviceUseTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDeviceWatcherTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDeviceWatcherTrigger {}
impl ::core::clone::Clone for IDeviceWatcherTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEmailStoreNotificationTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEmailStoreNotificationTrigger {}
impl ::core::clone::Clone for IEmailStoreNotificationTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattCharacteristicNotificationTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattCharacteristicNotificationTrigger {}
impl ::core::clone::Clone for IGattCharacteristicNotificationTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattCharacteristicNotificationTrigger2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattCharacteristicNotificationTrigger2 {}
impl ::core::clone::Clone for IGattCharacteristicNotificationTrigger2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattCharacteristicNotificationTriggerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattCharacteristicNotificationTriggerFactory {}
impl ::core::clone::Clone for IGattCharacteristicNotificationTriggerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattCharacteristicNotificationTriggerFactory2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattCharacteristicNotificationTriggerFactory2 {}
impl ::core::clone::Clone for IGattCharacteristicNotificationTriggerFactory2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattServiceProviderTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattServiceProviderTrigger {}
impl ::core::clone::Clone for IGattServiceProviderTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattServiceProviderTriggerResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattServiceProviderTriggerResult {}
impl ::core::clone::Clone for IGattServiceProviderTriggerResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGattServiceProviderTriggerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGattServiceProviderTriggerStatics {}
impl ::core::clone::Clone for IGattServiceProviderTriggerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGeovisitTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGeovisitTrigger {}
impl ::core::clone::Clone for IGeovisitTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILocationTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILocationTrigger {}
impl ::core::clone::Clone for ILocationTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILocationTriggerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILocationTriggerFactory {}
impl ::core::clone::Clone for ILocationTriggerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMaintenanceTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMaintenanceTrigger {}
impl ::core::clone::Clone for IMaintenanceTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMaintenanceTriggerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMaintenanceTriggerFactory {}
impl ::core::clone::Clone for IMaintenanceTriggerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaProcessingTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaProcessingTrigger {}
impl ::core::clone::Clone for IMediaProcessingTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetworkOperatorHotspotAuthenticationTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetworkOperatorHotspotAuthenticationTrigger {}
impl ::core::clone::Clone for INetworkOperatorHotspotAuthenticationTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetworkOperatorNotificationTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetworkOperatorNotificationTrigger {}
impl ::core::clone::Clone for INetworkOperatorNotificationTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetworkOperatorNotificationTriggerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetworkOperatorNotificationTriggerFactory {}
impl ::core::clone::Clone for INetworkOperatorNotificationTriggerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhoneTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhoneTrigger {}
impl ::core::clone::Clone for IPhoneTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPhoneTriggerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPhoneTriggerFactory {}
impl ::core::clone::Clone for IPhoneTriggerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPushNotificationTriggerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPushNotificationTriggerFactory {}
impl ::core::clone::Clone for IPushNotificationTriggerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRcsEndUserMessageAvailableTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRcsEndUserMessageAvailableTrigger {}
impl ::core::clone::Clone for IRcsEndUserMessageAvailableTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRfcommConnectionTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRfcommConnectionTrigger {}
impl ::core::clone::Clone for IRfcommConnectionTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISecondaryAuthenticationFactorAuthenticationTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISecondaryAuthenticationFactorAuthenticationTrigger {}
impl ::core::clone::Clone for ISecondaryAuthenticationFactorAuthenticationTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISensorDataThresholdTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISensorDataThresholdTrigger {}
impl ::core::clone::Clone for ISensorDataThresholdTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISensorDataThresholdTriggerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISensorDataThresholdTriggerFactory {}
impl ::core::clone::Clone for ISensorDataThresholdTriggerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmartCardTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmartCardTrigger {}
impl ::core::clone::Clone for ISmartCardTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmartCardTriggerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmartCardTriggerFactory {}
impl ::core::clone::Clone for ISmartCardTriggerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISmsMessageReceivedTriggerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISmsMessageReceivedTriggerFactory {}
impl ::core::clone::Clone for ISmsMessageReceivedTriggerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISocketActivityTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISocketActivityTrigger {}
impl ::core::clone::Clone for ISocketActivityTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageLibraryChangeTrackerTriggerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageLibraryChangeTrackerTriggerFactory {}
impl ::core::clone::Clone for IStorageLibraryChangeTrackerTriggerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageLibraryContentChangedTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageLibraryContentChangedTrigger {}
impl ::core::clone::Clone for IStorageLibraryContentChangedTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStorageLibraryContentChangedTriggerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStorageLibraryContentChangedTriggerStatics {}
impl ::core::clone::Clone for IStorageLibraryContentChangedTriggerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISystemCondition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISystemCondition {}
impl ::core::clone::Clone for ISystemCondition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISystemConditionFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISystemConditionFactory {}
impl ::core::clone::Clone for ISystemConditionFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISystemTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISystemTrigger {}
impl ::core::clone::Clone for ISystemTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISystemTriggerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISystemTriggerFactory {}
impl ::core::clone::Clone for ISystemTriggerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimeTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimeTrigger {}
impl ::core::clone::Clone for ITimeTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimeTriggerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimeTriggerFactory {}
impl ::core::clone::Clone for ITimeTriggerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToastNotificationActionTriggerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToastNotificationActionTriggerFactory {}
impl ::core::clone::Clone for IToastNotificationActionTriggerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IToastNotificationHistoryChangedTriggerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IToastNotificationHistoryChangedTriggerFactory {}
impl ::core::clone::Clone for IToastNotificationHistoryChangedTriggerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUserNotificationChangedTriggerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUserNotificationChangedTriggerFactory {}
impl ::core::clone::Clone for IUserNotificationChangedTriggerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LocationTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for LocationTrigger {}
impl ::core::clone::Clone for LocationTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LocationTriggerType(pub i32);
impl LocationTriggerType {
    pub const Geofence: Self = Self(0i32);
}
impl ::core::marker::Copy for LocationTriggerType {}
impl ::core::clone::Clone for LocationTriggerType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MaintenanceTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MaintenanceTrigger {}
impl ::core::clone::Clone for MaintenanceTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaProcessingTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaProcessingTrigger {}
impl ::core::clone::Clone for MediaProcessingTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaProcessingTriggerResult(pub i32);
impl MediaProcessingTriggerResult {
    pub const Allowed: Self = Self(0i32);
    pub const CurrentlyRunning: Self = Self(1i32);
    pub const DisabledByPolicy: Self = Self(2i32);
    pub const UnknownError: Self = Self(3i32);
}
impl ::core::marker::Copy for MediaProcessingTriggerResult {}
impl ::core::clone::Clone for MediaProcessingTriggerResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandDeviceServiceNotificationTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MobileBroadbandDeviceServiceNotificationTrigger {}
impl ::core::clone::Clone for MobileBroadbandDeviceServiceNotificationTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandPcoDataChangeTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MobileBroadbandPcoDataChangeTrigger {}
impl ::core::clone::Clone for MobileBroadbandPcoDataChangeTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandPinLockStateChangeTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MobileBroadbandPinLockStateChangeTrigger {}
impl ::core::clone::Clone for MobileBroadbandPinLockStateChangeTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandRadioStateChangeTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MobileBroadbandRadioStateChangeTrigger {}
impl ::core::clone::Clone for MobileBroadbandRadioStateChangeTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MobileBroadbandRegistrationStateChangeTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MobileBroadbandRegistrationStateChangeTrigger {}
impl ::core::clone::Clone for MobileBroadbandRegistrationStateChangeTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NetworkOperatorDataUsageTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for NetworkOperatorDataUsageTrigger {}
impl ::core::clone::Clone for NetworkOperatorDataUsageTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NetworkOperatorHotspotAuthenticationTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for NetworkOperatorHotspotAuthenticationTrigger {}
impl ::core::clone::Clone for NetworkOperatorHotspotAuthenticationTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct NetworkOperatorNotificationTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for NetworkOperatorNotificationTrigger {}
impl ::core::clone::Clone for NetworkOperatorNotificationTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PaymentAppCanMakePaymentTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PaymentAppCanMakePaymentTrigger {}
impl ::core::clone::Clone for PaymentAppCanMakePaymentTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PhoneTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PhoneTrigger {}
impl ::core::clone::Clone for PhoneTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PushNotificationTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PushNotificationTrigger {}
impl ::core::clone::Clone for PushNotificationTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RcsEndUserMessageAvailableTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RcsEndUserMessageAvailableTrigger {}
impl ::core::clone::Clone for RcsEndUserMessageAvailableTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RfcommConnectionTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RfcommConnectionTrigger {}
impl ::core::clone::Clone for RfcommConnectionTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorAuthenticationTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SecondaryAuthenticationFactorAuthenticationTrigger {}
impl ::core::clone::Clone for SecondaryAuthenticationFactorAuthenticationTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SensorDataThresholdTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SensorDataThresholdTrigger {}
impl ::core::clone::Clone for SensorDataThresholdTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmartCardTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SmartCardTrigger {}
impl ::core::clone::Clone for SmartCardTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SmsMessageReceivedTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SmsMessageReceivedTrigger {}
impl ::core::clone::Clone for SmsMessageReceivedTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SocketActivityTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SocketActivityTrigger {}
impl ::core::clone::Clone for SocketActivityTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StorageLibraryChangeTrackerTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for StorageLibraryChangeTrackerTrigger {}
impl ::core::clone::Clone for StorageLibraryChangeTrackerTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StorageLibraryContentChangedTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for StorageLibraryContentChangedTrigger {}
impl ::core::clone::Clone for StorageLibraryContentChangedTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SystemCondition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SystemCondition {}
impl ::core::clone::Clone for SystemCondition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SystemConditionType(pub i32);
impl SystemConditionType {
    pub const Invalid: Self = Self(0i32);
    pub const UserPresent: Self = Self(1i32);
    pub const UserNotPresent: Self = Self(2i32);
    pub const InternetAvailable: Self = Self(3i32);
    pub const InternetNotAvailable: Self = Self(4i32);
    pub const SessionConnected: Self = Self(5i32);
    pub const SessionDisconnected: Self = Self(6i32);
    pub const FreeNetworkAvailable: Self = Self(7i32);
    pub const BackgroundWorkCostNotHigh: Self = Self(8i32);
}
impl ::core::marker::Copy for SystemConditionType {}
impl ::core::clone::Clone for SystemConditionType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SystemTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SystemTrigger {}
impl ::core::clone::Clone for SystemTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SystemTriggerType(pub i32);
impl SystemTriggerType {
    pub const Invalid: Self = Self(0i32);
    pub const SmsReceived: Self = Self(1i32);
    pub const UserPresent: Self = Self(2i32);
    pub const UserAway: Self = Self(3i32);
    pub const NetworkStateChange: Self = Self(4i32);
    pub const ControlChannelReset: Self = Self(5i32);
    pub const InternetAvailable: Self = Self(6i32);
    pub const SessionConnected: Self = Self(7i32);
    pub const ServicingComplete: Self = Self(8i32);
    pub const LockScreenApplicationAdded: Self = Self(9i32);
    pub const LockScreenApplicationRemoved: Self = Self(10i32);
    pub const TimeZoneChange: Self = Self(11i32);
    pub const OnlineIdConnectedStateChange: Self = Self(12i32);
    pub const BackgroundWorkCostChange: Self = Self(13i32);
    pub const PowerStateChange: Self = Self(14i32);
    pub const DefaultSignInAccountChange: Self = Self(15i32);
}
impl ::core::marker::Copy for SystemTriggerType {}
impl ::core::clone::Clone for SystemTriggerType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TetheringEntitlementCheckTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TetheringEntitlementCheckTrigger {}
impl ::core::clone::Clone for TetheringEntitlementCheckTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TimeTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TimeTrigger {}
impl ::core::clone::Clone for TimeTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ToastNotificationActionTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ToastNotificationActionTrigger {}
impl ::core::clone::Clone for ToastNotificationActionTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ToastNotificationHistoryChangedTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ToastNotificationHistoryChangedTrigger {}
impl ::core::clone::Clone for ToastNotificationHistoryChangedTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UserNotificationChangedTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UserNotificationChangedTrigger {}
impl ::core::clone::Clone for UserNotificationChangedTrigger {
    fn clone(&self) -> Self {
        *self
    }
}

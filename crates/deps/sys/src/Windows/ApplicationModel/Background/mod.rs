#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ActivitySensorTrigger(pub *mut ::core::ffi::c_void);
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
#[repr(transparent)]
pub struct AppBroadcastTriggerProviderInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ApplicationTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ApplicationTriggerDetails(pub *mut ::core::ffi::c_void);
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
#[repr(transparent)]
pub struct BackgroundTaskCanceledEventHandler(pub *mut ::core::ffi::c_void);
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
#[repr(transparent)]
pub struct BackgroundTaskCompletedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BackgroundTaskDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BackgroundTaskProgressEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BackgroundTaskProgressEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BackgroundTaskRegistration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BackgroundTaskRegistrationGroup(pub *mut ::core::ffi::c_void);
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
#[repr(transparent)]
pub struct BluetoothLEAdvertisementWatcherTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CachedFileUpdaterTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CachedFileUpdaterTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChatMessageNotificationTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ChatMessageReceivedNotificationTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CommunicationBlockingAppSetAsActiveTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContactStoreNotificationTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ContentPrefetchTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ConversationalAgentTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CustomSystemEventTrigger(pub *mut ::core::ffi::c_void);
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
#[repr(transparent)]
pub struct DeviceManufacturerNotificationTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DeviceServicingTrigger(pub *mut ::core::ffi::c_void);
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
#[repr(transparent)]
pub struct DeviceWatcherTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EmailStoreNotificationTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GattCharacteristicNotificationTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GattServiceProviderTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GattServiceProviderTriggerResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GeovisitTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActivitySensorTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActivitySensorTriggerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAlarmApplicationManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBroadcastTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBroadcastTriggerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppBroadcastTriggerProviderInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IApplicationTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IApplicationTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppointmentStoreNotificationTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackgroundCondition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackgroundExecutionManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackgroundExecutionManagerStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackgroundExecutionManagerStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackgroundTask(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackgroundTaskBuilder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackgroundTaskBuilder2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackgroundTaskBuilder3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackgroundTaskBuilder4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackgroundTaskBuilder5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackgroundTaskCompletedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackgroundTaskDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackgroundTaskInstance(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackgroundTaskInstance2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackgroundTaskInstance4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackgroundTaskProgressEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackgroundTaskRegistration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackgroundTaskRegistration2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackgroundTaskRegistration3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackgroundTaskRegistrationGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackgroundTaskRegistrationGroupFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackgroundTaskRegistrationStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackgroundTaskRegistrationStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackgroundTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackgroundWorkCostStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementPublisherTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementPublisherTrigger2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementWatcherTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBluetoothLEAdvertisementWatcherTrigger2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICachedFileUpdaterTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICachedFileUpdaterTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IChatMessageNotificationTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IChatMessageReceivedNotificationTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICommunicationBlockingAppSetAsActiveTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactStoreNotificationTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContentPrefetchTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContentPrefetchTriggerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICustomSystemEventTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICustomSystemEventTriggerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeviceConnectionChangeTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeviceConnectionChangeTriggerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeviceManufacturerNotificationTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeviceManufacturerNotificationTriggerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeviceServicingTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeviceUseTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeviceWatcherTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailStoreNotificationTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattCharacteristicNotificationTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattCharacteristicNotificationTrigger2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattCharacteristicNotificationTriggerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattCharacteristicNotificationTriggerFactory2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattServiceProviderTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattServiceProviderTriggerResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGattServiceProviderTriggerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGeovisitTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILocationTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILocationTriggerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMaintenanceTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMaintenanceTriggerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaProcessingTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetworkOperatorHotspotAuthenticationTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetworkOperatorNotificationTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetworkOperatorNotificationTriggerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhoneTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPhoneTriggerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPushNotificationTriggerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRcsEndUserMessageAvailableTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRfcommConnectionTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISecondaryAuthenticationFactorAuthenticationTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISensorDataThresholdTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISensorDataThresholdTriggerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmartCardTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmartCardTriggerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISmsMessageReceivedTriggerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISocketActivityTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageLibraryChangeTrackerTriggerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageLibraryContentChangedTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageLibraryContentChangedTriggerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemCondition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemConditionFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemTriggerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimeTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimeTriggerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastNotificationActionTriggerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IToastNotificationHistoryChangedTriggerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserNotificationChangedTriggerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LocationTrigger(pub *mut ::core::ffi::c_void);
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
#[repr(transparent)]
pub struct MediaProcessingTrigger(pub *mut ::core::ffi::c_void);
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
#[repr(transparent)]
pub struct MobileBroadbandPcoDataChangeTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MobileBroadbandPinLockStateChangeTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MobileBroadbandRadioStateChangeTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MobileBroadbandRegistrationStateChangeTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NetworkOperatorDataUsageTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NetworkOperatorHotspotAuthenticationTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct NetworkOperatorNotificationTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PaymentAppCanMakePaymentTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PhoneTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PushNotificationTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RcsEndUserMessageAvailableTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RfcommConnectionTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SecondaryAuthenticationFactorAuthenticationTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SensorDataThresholdTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmartCardTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SmsMessageReceivedTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SocketActivityTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorageLibraryChangeTrackerTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StorageLibraryContentChangedTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SystemCondition(pub *mut ::core::ffi::c_void);
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
#[repr(transparent)]
pub struct TimeTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ToastNotificationActionTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ToastNotificationHistoryChangedTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UserNotificationChangedTrigger(pub *mut ::core::ffi::c_void);

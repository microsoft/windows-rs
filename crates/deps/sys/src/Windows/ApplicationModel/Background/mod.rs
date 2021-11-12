#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ActivitySensorTrigger(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct AlarmAccessStatus(i32);
#[repr(transparent)]
pub struct AlarmApplicationManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppBroadcastTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppBroadcastTriggerProviderInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ApplicationTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ApplicationTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct ApplicationTriggerResult(i32);
#[repr(transparent)]
pub struct AppointmentStoreNotificationTrigger(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct BackgroundAccessRequestKind(i32);
#[repr(C)]
pub struct BackgroundAccessStatus(i32);
#[repr(C)]
pub struct BackgroundAlarmApplicationContract(i32);
#[repr(transparent)]
pub struct BackgroundExecutionManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BackgroundTaskBuilder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BackgroundTaskCanceledEventHandler(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct BackgroundTaskCancellationReason(i32);
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
#[repr(C)]
pub struct BackgroundTaskThrottleCounter(i32);
#[repr(transparent)]
pub struct BackgroundWorkCost(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct BackgroundWorkCostValue(i32);
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
#[repr(C)]
pub struct CustomSystemEventTriggerRecurrence(i32);
#[repr(transparent)]
pub struct DeviceConnectionChangeTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DeviceManufacturerNotificationTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DeviceServicingTrigger(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct DeviceTriggerResult(i32);
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
#[repr(C)]
pub struct LocationTriggerType(i32);
#[repr(transparent)]
pub struct MaintenanceTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaProcessingTrigger(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct MediaProcessingTriggerResult(i32);
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
#[repr(C)]
pub struct SystemConditionType(i32);
#[repr(transparent)]
pub struct SystemTrigger(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct SystemTriggerType(i32);
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

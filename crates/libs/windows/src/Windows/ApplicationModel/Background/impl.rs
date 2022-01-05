#[cfg(feature = "implement_exclusive")]
pub trait IActivitySensorTriggerImpl: Sized + IBackgroundTriggerImpl {
    fn SubscribedActivities(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Devices::Sensors::ActivityType>>;
    fn ReportInterval(&self) -> ::windows::core::Result<u32>;
    fn SupportedActivities(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Devices::Sensors::ActivityType>>;
    fn MinimumReportInterval(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IActivitySensorTriggerFactoryImpl: Sized {
    fn Create(&self, reportintervalinmilliseconds: u32) -> ::windows::core::Result<ActivitySensorTrigger>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAlarmApplicationManagerStaticsImpl: Sized {
    fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AlarmAccessStatus>>;
    fn GetAccessStatus(&self) -> ::windows::core::Result<AlarmAccessStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastTriggerImpl: Sized + IBackgroundTriggerImpl {
    fn SetProviderInfo(&self, value: &::core::option::Option<AppBroadcastTriggerProviderInfo>) -> ::windows::core::Result<()>;
    fn ProviderInfo(&self) -> ::windows::core::Result<AppBroadcastTriggerProviderInfo>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastTriggerFactoryImpl: Sized {
    fn CreateAppBroadcastTrigger(&self, providerkey: &::windows::core::HSTRING) -> ::windows::core::Result<AppBroadcastTrigger>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppBroadcastTriggerProviderInfoImpl: Sized {
    fn SetDisplayNameResource(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DisplayNameResource(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLogoResource(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn LogoResource(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetVideoKeyFrameInterval(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn VideoKeyFrameInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetMaxVideoBitrate(&self, value: u32) -> ::windows::core::Result<()>;
    fn MaxVideoBitrate(&self) -> ::windows::core::Result<u32>;
    fn SetMaxVideoWidth(&self, value: u32) -> ::windows::core::Result<()>;
    fn MaxVideoWidth(&self) -> ::windows::core::Result<u32>;
    fn SetMaxVideoHeight(&self, value: u32) -> ::windows::core::Result<()>;
    fn MaxVideoHeight(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationTriggerImpl: Sized + IBackgroundTriggerImpl {
    fn RequestAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ApplicationTriggerResult>>;
    fn RequestAsyncWithArguments(&self, arguments: &::core::option::Option<super::super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ApplicationTriggerResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IApplicationTriggerDetailsImpl: Sized {
    fn Arguments(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppointmentStoreNotificationTriggerImpl: Sized + IBackgroundTriggerImpl {}
pub trait IBackgroundConditionImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundExecutionManagerStaticsImpl: Sized {
    fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BackgroundAccessStatus>>;
    fn RequestAccessForApplicationAsync(&self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BackgroundAccessStatus>>;
    fn RemoveAccess(&self) -> ::windows::core::Result<()>;
    fn RemoveAccessForApplication(&self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GetAccessStatus(&self) -> ::windows::core::Result<BackgroundAccessStatus>;
    fn GetAccessStatusForApplication(&self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<BackgroundAccessStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundExecutionManagerStatics2Impl: Sized {
    fn RequestAccessKindAsync(&self, requestedaccess: BackgroundAccessRequestKind, reason: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundExecutionManagerStatics3Impl: Sized {
    fn RequestAccessKindForModernStandbyAsync(&self, requestedaccess: BackgroundAccessRequestKind, reason: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn GetAccessStatusForModernStandby(&self) -> ::windows::core::Result<BackgroundAccessStatus>;
    fn GetAccessStatusForModernStandbyForApplication(&self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<BackgroundAccessStatus>;
}
pub trait IBackgroundTaskImpl: Sized {
    fn Run(&self, taskinstance: &::core::option::Option<IBackgroundTaskInstance>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundTaskBuilderImpl: Sized {
    fn SetTaskEntryPoint(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TaskEntryPoint(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTrigger(&self, trigger: &::core::option::Option<IBackgroundTrigger>) -> ::windows::core::Result<()>;
    fn AddCondition(&self, condition: &::core::option::Option<IBackgroundCondition>) -> ::windows::core::Result<()>;
    fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Register(&self) -> ::windows::core::Result<BackgroundTaskRegistration>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundTaskBuilder2Impl: Sized + IBackgroundTaskBuilderImpl {
    fn SetCancelOnConditionLoss(&self, value: bool) -> ::windows::core::Result<()>;
    fn CancelOnConditionLoss(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundTaskBuilder3Impl: Sized + IBackgroundTaskBuilderImpl {
    fn SetIsNetworkRequested(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsNetworkRequested(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundTaskBuilder4Impl: Sized + IBackgroundTaskBuilderImpl {
    fn TaskGroup(&self) -> ::windows::core::Result<BackgroundTaskRegistrationGroup>;
    fn SetTaskGroup(&self, value: &::core::option::Option<BackgroundTaskRegistrationGroup>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundTaskBuilder5Impl: Sized {
    fn SetTaskEntryPointClsid(&self, taskentrypoint: &::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundTaskCompletedEventArgsImpl: Sized {
    fn InstanceId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn CheckResult(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundTaskDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
pub trait IBackgroundTaskInstanceImpl: Sized {
    fn InstanceId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Task(&self) -> ::windows::core::Result<BackgroundTaskRegistration>;
    fn Progress(&self) -> ::windows::core::Result<u32>;
    fn SetProgress(&self, value: u32) -> ::windows::core::Result<()>;
    fn TriggerDetails(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn Canceled(&self, cancelhandler: &::core::option::Option<BackgroundTaskCanceledEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCanceled(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SuspendedCount(&self) -> ::windows::core::Result<u32>;
    fn GetDeferral(&self) -> ::windows::core::Result<BackgroundTaskDeferral>;
}
pub trait IBackgroundTaskInstance2Impl: Sized + IBackgroundTaskInstanceImpl {
    fn GetThrottleCount(&self, counter: BackgroundTaskThrottleCounter) -> ::windows::core::Result<u32>;
}
pub trait IBackgroundTaskInstance4Impl: Sized + IBackgroundTaskInstanceImpl {
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundTaskProgressEventArgsImpl: Sized {
    fn InstanceId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Progress(&self) -> ::windows::core::Result<u32>;
}
pub trait IBackgroundTaskRegistrationImpl: Sized {
    fn TaskId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Progress(&self, handler: &::core::option::Option<BackgroundTaskProgressEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveProgress(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Completed(&self, handler: &::core::option::Option<BackgroundTaskCompletedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCompleted(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Unregister(&self, canceltask: bool) -> ::windows::core::Result<()>;
}
pub trait IBackgroundTaskRegistration2Impl: Sized + IBackgroundTaskRegistrationImpl {
    fn Trigger(&self) -> ::windows::core::Result<IBackgroundTrigger>;
}
pub trait IBackgroundTaskRegistration3Impl: Sized + IBackgroundTaskRegistrationImpl {
    fn TaskGroup(&self) -> ::windows::core::Result<BackgroundTaskRegistrationGroup>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundTaskRegistrationGroupImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BackgroundActivated(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<BackgroundTaskRegistrationGroup, super::Activation::BackgroundActivatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveBackgroundActivated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AllTasks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::GUID, BackgroundTaskRegistration>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundTaskRegistrationGroupFactoryImpl: Sized {
    fn Create(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<BackgroundTaskRegistrationGroup>;
    fn CreateWithName(&self, id: &::windows::core::HSTRING, name: &::windows::core::HSTRING) -> ::windows::core::Result<BackgroundTaskRegistrationGroup>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundTaskRegistrationStaticsImpl: Sized {
    fn AllTasks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::GUID, IBackgroundTaskRegistration>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundTaskRegistrationStatics2Impl: Sized {
    fn AllTaskGroups(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, BackgroundTaskRegistrationGroup>>;
    fn GetTaskGroup(&self, groupid: &::windows::core::HSTRING) -> ::windows::core::Result<BackgroundTaskRegistrationGroup>;
}
pub trait IBackgroundTriggerImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundWorkCostStaticsImpl: Sized {
    fn CurrentBackgroundWorkCost(&self) -> ::windows::core::Result<BackgroundWorkCostValue>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEAdvertisementPublisherTriggerImpl: Sized + IBackgroundTriggerImpl {
    fn Advertisement(&self) -> ::windows::core::Result<super::super::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisement>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEAdvertisementPublisherTrigger2Impl: Sized {
    fn PreferredTransmitPowerLevelInDBm(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i16>>;
    fn SetPreferredTransmitPowerLevelInDBm(&self, value: &::core::option::Option<super::super::Foundation::IReference<i16>>) -> ::windows::core::Result<()>;
    fn UseExtendedFormat(&self) -> ::windows::core::Result<bool>;
    fn SetUseExtendedFormat(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsAnonymous(&self) -> ::windows::core::Result<bool>;
    fn SetIsAnonymous(&self, value: bool) -> ::windows::core::Result<()>;
    fn IncludeTransmitPowerLevel(&self) -> ::windows::core::Result<bool>;
    fn SetIncludeTransmitPowerLevel(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEAdvertisementWatcherTriggerImpl: Sized + IBackgroundTriggerImpl {
    fn MinSamplingInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn MaxSamplingInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn MinOutOfRangeTimeout(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn MaxOutOfRangeTimeout(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SignalStrengthFilter(&self) -> ::windows::core::Result<super::super::Devices::Bluetooth::BluetoothSignalStrengthFilter>;
    fn SetSignalStrengthFilter(&self, value: &::core::option::Option<super::super::Devices::Bluetooth::BluetoothSignalStrengthFilter>) -> ::windows::core::Result<()>;
    fn AdvertisementFilter(&self) -> ::windows::core::Result<super::super::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementFilter>;
    fn SetAdvertisementFilter(&self, value: &::core::option::Option<super::super::Devices::Bluetooth::Advertisement::BluetoothLEAdvertisementFilter>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBluetoothLEAdvertisementWatcherTrigger2Impl: Sized {
    fn AllowExtendedAdvertisements(&self) -> ::windows::core::Result<bool>;
    fn SetAllowExtendedAdvertisements(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICachedFileUpdaterTriggerImpl: Sized + IBackgroundTriggerImpl {}
#[cfg(feature = "implement_exclusive")]
pub trait ICachedFileUpdaterTriggerDetailsImpl: Sized {
    fn UpdateTarget(&self) -> ::windows::core::Result<super::super::Storage::Provider::CachedFileTarget>;
    fn UpdateRequest(&self) -> ::windows::core::Result<super::super::Storage::Provider::FileUpdateRequest>;
    fn CanRequestUserInput(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IChatMessageNotificationTriggerImpl: Sized + IBackgroundTriggerImpl {}
#[cfg(feature = "implement_exclusive")]
pub trait IChatMessageReceivedNotificationTriggerImpl: Sized + IBackgroundTriggerImpl {}
#[cfg(feature = "implement_exclusive")]
pub trait ICommunicationBlockingAppSetAsActiveTriggerImpl: Sized + IBackgroundTriggerImpl {}
#[cfg(feature = "implement_exclusive")]
pub trait IContactStoreNotificationTriggerImpl: Sized + IBackgroundTriggerImpl {}
#[cfg(feature = "implement_exclusive")]
pub trait IContentPrefetchTriggerImpl: Sized + IBackgroundTriggerImpl {
    fn WaitInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentPrefetchTriggerFactoryImpl: Sized {
    fn Create(&self, waitinterval: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<ContentPrefetchTrigger>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICustomSystemEventTriggerImpl: Sized {
    fn TriggerId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Recurrence(&self) -> ::windows::core::Result<CustomSystemEventTriggerRecurrence>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICustomSystemEventTriggerFactoryImpl: Sized {
    fn Create(&self, triggerid: &::windows::core::HSTRING, recurrence: CustomSystemEventTriggerRecurrence) -> ::windows::core::Result<CustomSystemEventTrigger>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceConnectionChangeTriggerImpl: Sized + IBackgroundTriggerImpl {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CanMaintainConnection(&self) -> ::windows::core::Result<bool>;
    fn MaintainConnection(&self) -> ::windows::core::Result<bool>;
    fn SetMaintainConnection(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceConnectionChangeTriggerStaticsImpl: Sized {
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceConnectionChangeTrigger>>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IDeviceManufacturerNotificationTriggerImpl: Sized + IBackgroundTriggerImpl {
    fn TriggerQualifier(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn OneShot(&self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IDeviceManufacturerNotificationTriggerFactoryImpl: Sized {
    fn Create(&self, triggerqualifier: &::windows::core::HSTRING, oneshot: bool) -> ::windows::core::Result<DeviceManufacturerNotificationTrigger>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceServicingTriggerImpl: Sized + IBackgroundTriggerImpl {
    fn RequestAsyncSimple(&self, deviceid: &::windows::core::HSTRING, expectedduration: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceTriggerResult>>;
    fn RequestAsyncWithArguments(&self, deviceid: &::windows::core::HSTRING, expectedduration: &super::super::Foundation::TimeSpan, arguments: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceTriggerResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceUseTriggerImpl: Sized + IBackgroundTriggerImpl {
    fn RequestAsyncSimple(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceTriggerResult>>;
    fn RequestAsyncWithArguments(&self, deviceid: &::windows::core::HSTRING, arguments: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DeviceTriggerResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeviceWatcherTriggerImpl: Sized + IBackgroundTriggerImpl {}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailStoreNotificationTriggerImpl: Sized + IBackgroundTriggerImpl {}
#[cfg(feature = "implement_exclusive")]
pub trait IGattCharacteristicNotificationTriggerImpl: Sized + IBackgroundTriggerImpl {
    fn Characteristic(&self) -> ::windows::core::Result<super::super::Devices::Bluetooth::GenericAttributeProfile::GattCharacteristic>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattCharacteristicNotificationTrigger2Impl: Sized {
    fn EventTriggeringMode(&self) -> ::windows::core::Result<super::super::Devices::Bluetooth::Background::BluetoothEventTriggeringMode>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattCharacteristicNotificationTriggerFactoryImpl: Sized {
    fn Create(&self, characteristic: &::core::option::Option<super::super::Devices::Bluetooth::GenericAttributeProfile::GattCharacteristic>) -> ::windows::core::Result<GattCharacteristicNotificationTrigger>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattCharacteristicNotificationTriggerFactory2Impl: Sized {
    fn CreateWithEventTriggeringMode(&self, characteristic: &::core::option::Option<super::super::Devices::Bluetooth::GenericAttributeProfile::GattCharacteristic>, eventtriggeringmode: super::super::Devices::Bluetooth::Background::BluetoothEventTriggeringMode) -> ::windows::core::Result<GattCharacteristicNotificationTrigger>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattServiceProviderTriggerImpl: Sized {
    fn TriggerId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Service(&self) -> ::windows::core::Result<super::super::Devices::Bluetooth::GenericAttributeProfile::GattLocalService>;
    fn SetAdvertisingParameters(&self, value: &::core::option::Option<super::super::Devices::Bluetooth::GenericAttributeProfile::GattServiceProviderAdvertisingParameters>) -> ::windows::core::Result<()>;
    fn AdvertisingParameters(&self) -> ::windows::core::Result<super::super::Devices::Bluetooth::GenericAttributeProfile::GattServiceProviderAdvertisingParameters>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattServiceProviderTriggerResultImpl: Sized {
    fn Trigger(&self) -> ::windows::core::Result<GattServiceProviderTrigger>;
    fn Error(&self) -> ::windows::core::Result<super::super::Devices::Bluetooth::BluetoothError>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGattServiceProviderTriggerStaticsImpl: Sized {
    fn CreateAsync(&self, triggerid: &::windows::core::HSTRING, serviceuuid: &::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<GattServiceProviderTriggerResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeovisitTriggerImpl: Sized + IBackgroundTriggerImpl {
    fn MonitoringScope(&self) -> ::windows::core::Result<super::super::Devices::Geolocation::VisitMonitoringScope>;
    fn SetMonitoringScope(&self, value: super::super::Devices::Geolocation::VisitMonitoringScope) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILocationTriggerImpl: Sized + IBackgroundTriggerImpl {
    fn TriggerType(&self) -> ::windows::core::Result<LocationTriggerType>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILocationTriggerFactoryImpl: Sized {
    fn Create(&self, triggertype: LocationTriggerType) -> ::windows::core::Result<LocationTrigger>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMaintenanceTriggerImpl: Sized + IBackgroundTriggerImpl {
    fn FreshnessTime(&self) -> ::windows::core::Result<u32>;
    fn OneShot(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMaintenanceTriggerFactoryImpl: Sized {
    fn Create(&self, freshnesstime: u32, oneshot: bool) -> ::windows::core::Result<MaintenanceTrigger>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaProcessingTriggerImpl: Sized + IBackgroundTriggerImpl {
    fn RequestAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaProcessingTriggerResult>>;
    fn RequestAsyncWithArguments(&self, arguments: &::core::option::Option<super::super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MediaProcessingTriggerResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INetworkOperatorHotspotAuthenticationTriggerImpl: Sized + IBackgroundTriggerImpl {}
#[cfg(feature = "implement_exclusive")]
pub trait INetworkOperatorNotificationTriggerImpl: Sized + IBackgroundTriggerImpl {
    fn NetworkAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait INetworkOperatorNotificationTriggerFactoryImpl: Sized {
    fn Create(&self, networkaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<NetworkOperatorNotificationTrigger>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneTriggerImpl: Sized + IBackgroundTriggerImpl {
    fn OneShot(&self) -> ::windows::core::Result<bool>;
    fn TriggerType(&self) -> ::windows::core::Result<super::Calls::Background::PhoneTriggerType>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneTriggerFactoryImpl: Sized {
    fn Create(&self, r#type: super::Calls::Background::PhoneTriggerType, oneshot: bool) -> ::windows::core::Result<PhoneTrigger>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPushNotificationTriggerFactoryImpl: Sized {
    fn Create(&self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<PushNotificationTrigger>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRcsEndUserMessageAvailableTriggerImpl: Sized + IBackgroundTriggerImpl {}
#[cfg(feature = "implement_exclusive")]
pub trait IRfcommConnectionTriggerImpl: Sized + IBackgroundTriggerImpl {
    fn InboundConnection(&self) -> ::windows::core::Result<super::super::Devices::Bluetooth::Background::RfcommInboundConnectionInformation>;
    fn OutboundConnection(&self) -> ::windows::core::Result<super::super::Devices::Bluetooth::Background::RfcommOutboundConnectionInformation>;
    fn AllowMultipleConnections(&self) -> ::windows::core::Result<bool>;
    fn SetAllowMultipleConnections(&self, value: bool) -> ::windows::core::Result<()>;
    fn ProtectionLevel(&self) -> ::windows::core::Result<super::super::Networking::Sockets::SocketProtectionLevel>;
    fn SetProtectionLevel(&self, value: super::super::Networking::Sockets::SocketProtectionLevel) -> ::windows::core::Result<()>;
    fn RemoteHostName(&self) -> ::windows::core::Result<super::super::Networking::HostName>;
    fn SetRemoteHostName(&self, value: &::core::option::Option<super::super::Networking::HostName>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait ISecondaryAuthenticationFactorAuthenticationTriggerImpl: Sized + IBackgroundTriggerImpl {}
#[cfg(feature = "implement_exclusive")]
pub trait ISensorDataThresholdTriggerImpl: Sized + IBackgroundTriggerImpl {}
#[cfg(feature = "implement_exclusive")]
pub trait ISensorDataThresholdTriggerFactoryImpl: Sized {
    fn Create(&self, threshold: &::core::option::Option<super::super::Devices::Sensors::ISensorDataThreshold>) -> ::windows::core::Result<SensorDataThresholdTrigger>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardTriggerImpl: Sized + IBackgroundTriggerImpl {
    fn TriggerType(&self) -> ::windows::core::Result<super::super::Devices::SmartCards::SmartCardTriggerType>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmartCardTriggerFactoryImpl: Sized {
    fn Create(&self, triggertype: super::super::Devices::SmartCards::SmartCardTriggerType) -> ::windows::core::Result<SmartCardTrigger>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISmsMessageReceivedTriggerFactoryImpl: Sized {
    fn Create(&self, filterrules: &::core::option::Option<super::super::Devices::Sms::SmsFilterRules>) -> ::windows::core::Result<SmsMessageReceivedTrigger>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISocketActivityTriggerImpl: Sized {
    fn IsWakeFromLowPowerSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageLibraryChangeTrackerTriggerFactoryImpl: Sized {
    fn Create(&self, tracker: &::core::option::Option<super::super::Storage::StorageLibraryChangeTracker>) -> ::windows::core::Result<StorageLibraryChangeTrackerTrigger>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageLibraryContentChangedTriggerImpl: Sized + IBackgroundTriggerImpl {}
#[cfg(feature = "implement_exclusive")]
pub trait IStorageLibraryContentChangedTriggerStaticsImpl: Sized {
    fn Create(&self, storagelibrary: &::core::option::Option<super::super::Storage::StorageLibrary>) -> ::windows::core::Result<StorageLibraryContentChangedTrigger>;
    fn CreateFromLibraries(&self, storagelibraries: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Storage::StorageLibrary>>) -> ::windows::core::Result<StorageLibraryContentChangedTrigger>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemConditionImpl: Sized + IBackgroundConditionImpl {
    fn ConditionType(&self) -> ::windows::core::Result<SystemConditionType>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemConditionFactoryImpl: Sized {
    fn Create(&self, conditiontype: SystemConditionType) -> ::windows::core::Result<SystemCondition>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemTriggerImpl: Sized + IBackgroundTriggerImpl {
    fn OneShot(&self) -> ::windows::core::Result<bool>;
    fn TriggerType(&self) -> ::windows::core::Result<SystemTriggerType>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISystemTriggerFactoryImpl: Sized {
    fn Create(&self, triggertype: SystemTriggerType, oneshot: bool) -> ::windows::core::Result<SystemTrigger>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimeTriggerImpl: Sized + IBackgroundTriggerImpl {
    fn FreshnessTime(&self) -> ::windows::core::Result<u32>;
    fn OneShot(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITimeTriggerFactoryImpl: Sized {
    fn Create(&self, freshnesstime: u32, oneshot: bool) -> ::windows::core::Result<TimeTrigger>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastNotificationActionTriggerFactoryImpl: Sized {
    fn Create(&self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<ToastNotificationActionTrigger>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IToastNotificationHistoryChangedTriggerFactoryImpl: Sized {
    fn Create(&self, applicationid: &::windows::core::HSTRING) -> ::windows::core::Result<ToastNotificationHistoryChangedTrigger>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserNotificationChangedTriggerFactoryImpl: Sized {
    fn Create(&self, notificationkinds: super::super::UI::Notifications::NotificationKinds) -> ::windows::core::Result<UserNotificationChangedTrigger>;
}

#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IPushNotificationChannel();
    fn IPushNotificationChannelManagerForUser();
    fn IPushNotificationChannelManagerForUser2();
    fn IPushNotificationChannelManagerStatics();
    fn IPushNotificationChannelManagerStatics2();
    fn IPushNotificationChannelManagerStatics3();
    fn IPushNotificationChannelManagerStatics4();
    fn IPushNotificationChannelsRevokedEventArgs();
    fn IPushNotificationReceivedEventArgs();
    fn IRawNotification();
    fn IRawNotification2();
    fn IRawNotification3();
    fn PushNotificationChannel();
    fn PushNotificationChannelManager();
    fn PushNotificationChannelManagerForUser();
    fn PushNotificationChannelsRevokedEventArgs();
    fn PushNotificationReceivedEventArgs();
    fn PushNotificationType();
    fn RawNotification();
}

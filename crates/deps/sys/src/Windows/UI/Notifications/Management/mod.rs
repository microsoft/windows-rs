#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IUserNotificationListener();
    fn IUserNotificationListenerStatics();
    fn UserNotificationListener();
    fn UserNotificationListenerAccessStatus();
}

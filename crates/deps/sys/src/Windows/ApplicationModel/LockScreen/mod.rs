#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn ILockApplicationHost();
    fn ILockApplicationHostStatics();
    fn ILockScreenBadge();
    fn ILockScreenInfo();
    fn ILockScreenUnlockingDeferral();
    fn ILockScreenUnlockingEventArgs();
    fn LockApplicationHost();
    fn LockScreenBadge();
    fn LockScreenInfo();
    fn LockScreenUnlockingDeferral();
    fn LockScreenUnlockingEventArgs();
}

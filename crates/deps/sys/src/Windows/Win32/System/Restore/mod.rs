#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn ACCESSIBILITY_SETTING();
    fn APPLICATION_RUN();
    fn BACKUP();
    fn BACKUP_RECOVERY();
    fn BEGIN_NESTED_SYSTEM_CHANGE_NORP();
    fn CHECKPOINT();
    fn CRITICAL_UPDATE();
    fn DESKTOP_SETTING();
    fn FIRSTRUN();
    fn MANUAL_CHECKPOINT();
    fn MAX_DESC();
    fn MAX_DESC_W();
    fn MAX_EVENT();
    fn MAX_RPT();
    fn MIN_EVENT();
    fn MIN_RPT();
    fn OE_SETTING();
    fn RESTORE();
    fn RESTOREPOINTINFOA();
    fn RESTOREPOINTINFOW();
    fn RESTOREPOINTINFO_EVENT_TYPE();
    fn RESTOREPOINTINFO_TYPE();
    fn SRSetRestorePointA();
    fn SRSetRestorePointW();
    fn STATEMGRSTATUS();
    fn WINDOWS_BOOT();
    fn WINDOWS_SHUTDOWN();
    fn WINDOWS_UPDATE();
    fn _RESTOREPTINFOEX();
}

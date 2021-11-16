#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
pub const CLSID_CTask: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 344708384, data2: 41643, data3: 4558, data4: [177, 31, 0, 170, 0, 83, 5, 3] };
pub const CLSID_CTaskScheduler: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 344708394, data2: 41643, data3: 4558, data4: [177, 31, 0, 170, 0, 83, 5, 3] };
#[repr(C)]
pub struct DAILY {
    pub DaysInterval: u16,
}
impl ::core::marker::Copy for DAILY {}
impl ::core::clone::Clone for DAILY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAction(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAction {}
impl ::core::clone::Clone for IAction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IActionCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IActionCollection {}
impl ::core::clone::Clone for IActionCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IBootTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IBootTrigger {}
impl ::core::clone::Clone for IBootTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IComHandlerAction(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IComHandlerAction {}
impl ::core::clone::Clone for IComHandlerAction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDailyTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDailyTrigger {}
impl ::core::clone::Clone for IDailyTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEmailAction(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEmailAction {}
impl ::core::clone::Clone for IEmailAction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEnumWorkItems(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEnumWorkItems {}
impl ::core::clone::Clone for IEnumWorkItems {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEventTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEventTrigger {}
impl ::core::clone::Clone for IEventTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IExecAction(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IExecAction {}
impl ::core::clone::Clone for IExecAction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IExecAction2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IExecAction2 {}
impl ::core::clone::Clone for IExecAction2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IIdleSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IIdleSettings {}
impl ::core::clone::Clone for IIdleSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IIdleTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IIdleTrigger {}
impl ::core::clone::Clone for IIdleTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILogonTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILogonTrigger {}
impl ::core::clone::Clone for ILogonTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMaintenanceSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMaintenanceSettings {}
impl ::core::clone::Clone for IMaintenanceSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMonthlyDOWTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMonthlyDOWTrigger {}
impl ::core::clone::Clone for IMonthlyDOWTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMonthlyTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMonthlyTrigger {}
impl ::core::clone::Clone for IMonthlyTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetworkSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetworkSettings {}
impl ::core::clone::Clone for INetworkSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrincipal(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrincipal {}
impl ::core::clone::Clone for IPrincipal {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrincipal2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrincipal2 {}
impl ::core::clone::Clone for IPrincipal2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IProvideTaskPage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IProvideTaskPage {}
impl ::core::clone::Clone for IProvideTaskPage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRegisteredTask(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRegisteredTask {}
impl ::core::clone::Clone for IRegisteredTask {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRegisteredTaskCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRegisteredTaskCollection {}
impl ::core::clone::Clone for IRegisteredTaskCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRegistrationInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRegistrationInfo {}
impl ::core::clone::Clone for IRegistrationInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRegistrationTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRegistrationTrigger {}
impl ::core::clone::Clone for IRegistrationTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRepetitionPattern(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRepetitionPattern {}
impl ::core::clone::Clone for IRepetitionPattern {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRunningTask(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRunningTask {}
impl ::core::clone::Clone for IRunningTask {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRunningTaskCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRunningTaskCollection {}
impl ::core::clone::Clone for IRunningTaskCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IScheduledWorkItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IScheduledWorkItem {}
impl ::core::clone::Clone for IScheduledWorkItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISessionStateChangeTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISessionStateChangeTrigger {}
impl ::core::clone::Clone for ISessionStateChangeTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IShowMessageAction(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IShowMessageAction {}
impl ::core::clone::Clone for IShowMessageAction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITask(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITask {}
impl ::core::clone::Clone for ITask {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITaskDefinition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITaskDefinition {}
impl ::core::clone::Clone for ITaskDefinition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITaskFolder(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITaskFolder {}
impl ::core::clone::Clone for ITaskFolder {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITaskFolderCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITaskFolderCollection {}
impl ::core::clone::Clone for ITaskFolderCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITaskHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITaskHandler {}
impl ::core::clone::Clone for ITaskHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITaskHandlerStatus(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITaskHandlerStatus {}
impl ::core::clone::Clone for ITaskHandlerStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITaskNamedValueCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITaskNamedValueCollection {}
impl ::core::clone::Clone for ITaskNamedValueCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITaskNamedValuePair(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITaskNamedValuePair {}
impl ::core::clone::Clone for ITaskNamedValuePair {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITaskScheduler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITaskScheduler {}
impl ::core::clone::Clone for ITaskScheduler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITaskService(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITaskService {}
impl ::core::clone::Clone for ITaskService {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITaskSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITaskSettings {}
impl ::core::clone::Clone for ITaskSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITaskSettings2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITaskSettings2 {}
impl ::core::clone::Clone for ITaskSettings2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITaskSettings3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITaskSettings3 {}
impl ::core::clone::Clone for ITaskSettings3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITaskTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITaskTrigger {}
impl ::core::clone::Clone for ITaskTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITaskVariables(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITaskVariables {}
impl ::core::clone::Clone for ITaskVariables {
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
pub struct ITrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITrigger {}
impl ::core::clone::Clone for ITrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITriggerCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITriggerCollection {}
impl ::core::clone::Clone for ITriggerCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWeeklyTrigger(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWeeklyTrigger {}
impl ::core::clone::Clone for IWeeklyTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MONTHLYDATE {
    pub rgfDays: u32,
    pub rgfMonths: u16,
}
impl ::core::marker::Copy for MONTHLYDATE {}
impl ::core::clone::Clone for MONTHLYDATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct MONTHLYDOW {
    pub wWhichWeek: u16,
    pub rgfDaysOfTheWeek: u16,
    pub rgfMonths: u16,
}
impl ::core::marker::Copy for MONTHLYDOW {}
impl ::core::clone::Clone for MONTHLYDOW {
    fn clone(&self) -> Self {
        *self
    }
}
pub const TASKPAGE_TASK: i32 = 0i32;
pub const TASKPAGE_SCHEDULE: i32 = 1i32;
pub const TASKPAGE_SETTINGS: i32 = 2i32;
pub const TASK_ACTION_EXEC: i32 = 0i32;
pub const TASK_ACTION_COM_HANDLER: i32 = 5i32;
pub const TASK_ACTION_SEND_EMAIL: i32 = 6i32;
pub const TASK_ACTION_SHOW_MESSAGE: i32 = 7i32;
pub const TASK_APRIL: u32 = 8u32;
pub const TASK_AUGUST: u32 = 128u32;
pub const TASK_COMPATIBILITY_AT: i32 = 0i32;
pub const TASK_COMPATIBILITY_V1: i32 = 1i32;
pub const TASK_COMPATIBILITY_V2: i32 = 2i32;
pub const TASK_COMPATIBILITY_V2_1: i32 = 3i32;
pub const TASK_COMPATIBILITY_V2_2: i32 = 4i32;
pub const TASK_COMPATIBILITY_V2_3: i32 = 5i32;
pub const TASK_COMPATIBILITY_V2_4: i32 = 6i32;
pub const TASK_VALIDATE_ONLY: i32 = 1i32;
pub const TASK_CREATE: i32 = 2i32;
pub const TASK_UPDATE: i32 = 4i32;
pub const TASK_CREATE_OR_UPDATE: i32 = 6i32;
pub const TASK_DISABLE: i32 = 8i32;
pub const TASK_DONT_ADD_PRINCIPAL_ACE: i32 = 16i32;
pub const TASK_IGNORE_REGISTRATION_TRIGGERS: i32 = 32i32;
pub const TASK_DECEMBER: u32 = 2048u32;
pub const TASK_ENUM_HIDDEN: i32 = 1i32;
pub const TASK_FEBRUARY: u32 = 2u32;
pub const TASK_FIRST_WEEK: u32 = 1u32;
pub const TASK_FLAG_DELETE_WHEN_DONE: u32 = 2u32;
pub const TASK_FLAG_DISABLED: u32 = 4u32;
pub const TASK_FLAG_DONT_START_IF_ON_BATTERIES: u32 = 64u32;
pub const TASK_FLAG_HIDDEN: u32 = 512u32;
pub const TASK_FLAG_INTERACTIVE: u32 = 1u32;
pub const TASK_FLAG_KILL_IF_GOING_ON_BATTERIES: u32 = 128u32;
pub const TASK_FLAG_KILL_ON_IDLE_END: u32 = 32u32;
pub const TASK_FLAG_RESTART_ON_IDLE_RESUME: u32 = 2048u32;
pub const TASK_FLAG_RUN_IF_CONNECTED_TO_INTERNET: u32 = 1024u32;
pub const TASK_FLAG_RUN_ONLY_IF_DOCKED: u32 = 256u32;
pub const TASK_FLAG_RUN_ONLY_IF_LOGGED_ON: u32 = 8192u32;
pub const TASK_FLAG_START_ONLY_IF_IDLE: u32 = 16u32;
pub const TASK_FLAG_SYSTEM_REQUIRED: u32 = 4096u32;
pub const TASK_FOURTH_WEEK: u32 = 4u32;
pub const TASK_FRIDAY: u32 = 32u32;
pub const TASK_INSTANCES_PARALLEL: i32 = 0i32;
pub const TASK_INSTANCES_QUEUE: i32 = 1i32;
pub const TASK_INSTANCES_IGNORE_NEW: i32 = 2i32;
pub const TASK_INSTANCES_STOP_EXISTING: i32 = 3i32;
pub const TASK_JANUARY: u32 = 1u32;
pub const TASK_JULY: u32 = 64u32;
pub const TASK_JUNE: u32 = 32u32;
pub const TASK_LAST_WEEK: u32 = 5u32;
pub const TASK_LOGON_NONE: i32 = 0i32;
pub const TASK_LOGON_PASSWORD: i32 = 1i32;
pub const TASK_LOGON_S4U: i32 = 2i32;
pub const TASK_LOGON_INTERACTIVE_TOKEN: i32 = 3i32;
pub const TASK_LOGON_GROUP: i32 = 4i32;
pub const TASK_LOGON_SERVICE_ACCOUNT: i32 = 5i32;
pub const TASK_LOGON_INTERACTIVE_TOKEN_OR_PASSWORD: i32 = 6i32;
pub const TASK_MARCH: u32 = 4u32;
pub const TASK_MAX_RUN_TIMES: u32 = 1440u32;
pub const TASK_MAY: u32 = 16u32;
pub const TASK_MONDAY: u32 = 2u32;
pub const TASK_NOVEMBER: u32 = 1024u32;
pub const TASK_OCTOBER: u32 = 512u32;
pub const TASK_PROCESSTOKENSID_NONE: i32 = 0i32;
pub const TASK_PROCESSTOKENSID_UNRESTRICTED: i32 = 1i32;
pub const TASK_PROCESSTOKENSID_DEFAULT: i32 = 2i32;
pub const TASK_RUNLEVEL_LUA: i32 = 0i32;
pub const TASK_RUNLEVEL_HIGHEST: i32 = 1i32;
pub const TASK_RUN_NO_FLAGS: i32 = 0i32;
pub const TASK_RUN_AS_SELF: i32 = 1i32;
pub const TASK_RUN_IGNORE_CONSTRAINTS: i32 = 2i32;
pub const TASK_RUN_USE_SESSION_ID: i32 = 4i32;
pub const TASK_RUN_USER_SID: i32 = 8i32;
pub const TASK_SATURDAY: u32 = 64u32;
pub const TASK_SECOND_WEEK: u32 = 2u32;
pub const TASK_SEPTEMBER: u32 = 256u32;
pub const TASK_CONSOLE_CONNECT: i32 = 1i32;
pub const TASK_CONSOLE_DISCONNECT: i32 = 2i32;
pub const TASK_REMOTE_CONNECT: i32 = 3i32;
pub const TASK_REMOTE_DISCONNECT: i32 = 4i32;
pub const TASK_SESSION_LOCK: i32 = 7i32;
pub const TASK_SESSION_UNLOCK: i32 = 8i32;
pub const TASK_STATE_UNKNOWN: i32 = 0i32;
pub const TASK_STATE_DISABLED: i32 = 1i32;
pub const TASK_STATE_QUEUED: i32 = 2i32;
pub const TASK_STATE_READY: i32 = 3i32;
pub const TASK_STATE_RUNNING: i32 = 4i32;
pub const TASK_SUNDAY: u32 = 1u32;
pub const TASK_THIRD_WEEK: u32 = 3u32;
pub const TASK_THURSDAY: u32 = 16u32;
#[repr(C)]
pub struct TASK_TRIGGER {
    pub cbTriggerSize: u16,
    pub Reserved1: u16,
    pub wBeginYear: u16,
    pub wBeginMonth: u16,
    pub wBeginDay: u16,
    pub wEndYear: u16,
    pub wEndMonth: u16,
    pub wEndDay: u16,
    pub wStartHour: u16,
    pub wStartMinute: u16,
    pub MinutesDuration: u32,
    pub MinutesInterval: u32,
    pub rgFlags: u32,
    pub TriggerType: TASK_TRIGGER_TYPE,
    pub Type: TRIGGER_TYPE_UNION,
    pub Reserved2: u16,
    pub wRandomMinutesInterval: u16,
}
impl ::core::marker::Copy for TASK_TRIGGER {}
impl ::core::clone::Clone for TASK_TRIGGER {
    fn clone(&self) -> Self {
        *self
    }
}
pub const TASK_TRIGGER_FLAG_DISABLED: u32 = 4u32;
pub const TASK_TRIGGER_FLAG_HAS_END_DATE: u32 = 1u32;
pub const TASK_TRIGGER_FLAG_KILL_AT_DURATION_END: u32 = 2u32;
pub const TASK_TIME_TRIGGER_ONCE: i32 = 0i32;
pub const TASK_TIME_TRIGGER_DAILY: i32 = 1i32;
pub const TASK_TIME_TRIGGER_WEEKLY: i32 = 2i32;
pub const TASK_TIME_TRIGGER_MONTHLYDATE: i32 = 3i32;
pub const TASK_TIME_TRIGGER_MONTHLYDOW: i32 = 4i32;
pub const TASK_EVENT_TRIGGER_ON_IDLE: i32 = 5i32;
pub const TASK_EVENT_TRIGGER_AT_SYSTEMSTART: i32 = 6i32;
pub const TASK_EVENT_TRIGGER_AT_LOGON: i32 = 7i32;
pub const TASK_TRIGGER_EVENT: i32 = 0i32;
pub const TASK_TRIGGER_TIME: i32 = 1i32;
pub const TASK_TRIGGER_DAILY: i32 = 2i32;
pub const TASK_TRIGGER_WEEKLY: i32 = 3i32;
pub const TASK_TRIGGER_MONTHLY: i32 = 4i32;
pub const TASK_TRIGGER_MONTHLYDOW: i32 = 5i32;
pub const TASK_TRIGGER_IDLE: i32 = 6i32;
pub const TASK_TRIGGER_REGISTRATION: i32 = 7i32;
pub const TASK_TRIGGER_BOOT: i32 = 8i32;
pub const TASK_TRIGGER_LOGON: i32 = 9i32;
pub const TASK_TRIGGER_SESSION_STATE_CHANGE: i32 = 11i32;
pub const TASK_TRIGGER_CUSTOM_TRIGGER_01: i32 = 12i32;
pub const TASK_TUESDAY: u32 = 4u32;
pub const TASK_WEDNESDAY: u32 = 8u32;
#[repr(C)]
pub union TRIGGER_TYPE_UNION {
    pub Daily: DAILY,
    pub Weekly: WEEKLY,
    pub MonthlyDate: MONTHLYDATE,
    pub MonthlyDOW: MONTHLYDOW,
}
impl ::core::marker::Copy for TRIGGER_TYPE_UNION {}
impl ::core::clone::Clone for TRIGGER_TYPE_UNION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const TaskHandlerPS: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 4071005623,
    data2: 55852,
    data3: 17234,
    data4: [144, 102, 134, 254, 230, 218, 202, 201],
};
pub const TaskHandlerStatusPS: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2668963437,
    data2: 55226,
    data3: 18672,
    data4: [147, 193, 230, 137, 95, 111, 229, 172],
};
pub const TaskScheduler: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 260519583, data2: 42213, data3: 19708, data4: [189, 62, 115, 230, 21, 69, 114, 221] };
#[repr(C)]
pub struct WEEKLY {
    pub WeeksInterval: u16,
    pub rgfDaysOfTheWeek: u16,
}
impl ::core::marker::Copy for WEEKLY {}
impl ::core::clone::Clone for WEEKLY {
    fn clone(&self) -> Self {
        *self
    }
}

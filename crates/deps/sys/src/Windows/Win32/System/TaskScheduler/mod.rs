#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
pub const CLSID_CTask: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 344708384, data2: 41643, data3: 4558, data4: [177, 31, 0, 170, 0, 83, 5, 3] };
pub const CLSID_CTaskScheduler: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 344708394, data2: 41643, data3: 4558, data4: [177, 31, 0, 170, 0, 83, 5, 3] };
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
#[repr(transparent)]
pub struct IActionCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBootTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComHandlerAction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDailyTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEmailAction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumWorkItems(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEventTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IExecAction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IExecAction2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIdleSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIdleTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILogonTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMaintenanceSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMonthlyDOWTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMonthlyTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetworkSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrincipal(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrincipal2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProvideTaskPage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRegisteredTask(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRegisteredTaskCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRegistrationInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRegistrationTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRepetitionPattern(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRunningTask(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRunningTaskCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IScheduledWorkItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISessionStateChangeTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShowMessageAction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITask(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITaskDefinition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITaskFolder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITaskFolderCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITaskHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITaskHandlerStatus(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITaskNamedValueCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITaskNamedValuePair(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITaskScheduler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITaskService(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITaskSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITaskSettings2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITaskSettings3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITaskTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITaskVariables(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimeTrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITrigger(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITriggerCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWeeklyTrigger(pub *mut ::core::ffi::c_void);
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
#[repr(transparent)]
pub struct TASKPAGE(pub i32);
pub const TASKPAGE_TASK: TASKPAGE = TASKPAGE(0i32);
pub const TASKPAGE_SCHEDULE: TASKPAGE = TASKPAGE(1i32);
pub const TASKPAGE_SETTINGS: TASKPAGE = TASKPAGE(2i32);
#[repr(transparent)]
pub struct TASK_ACTION_TYPE(pub i32);
pub const TASK_ACTION_EXEC: TASK_ACTION_TYPE = TASK_ACTION_TYPE(0i32);
pub const TASK_ACTION_COM_HANDLER: TASK_ACTION_TYPE = TASK_ACTION_TYPE(5i32);
pub const TASK_ACTION_SEND_EMAIL: TASK_ACTION_TYPE = TASK_ACTION_TYPE(6i32);
pub const TASK_ACTION_SHOW_MESSAGE: TASK_ACTION_TYPE = TASK_ACTION_TYPE(7i32);
pub const TASK_APRIL: u32 = 8u32;
pub const TASK_AUGUST: u32 = 128u32;
#[repr(transparent)]
pub struct TASK_COMPATIBILITY(pub i32);
pub const TASK_COMPATIBILITY_AT: TASK_COMPATIBILITY = TASK_COMPATIBILITY(0i32);
pub const TASK_COMPATIBILITY_V1: TASK_COMPATIBILITY = TASK_COMPATIBILITY(1i32);
pub const TASK_COMPATIBILITY_V2: TASK_COMPATIBILITY = TASK_COMPATIBILITY(2i32);
pub const TASK_COMPATIBILITY_V2_1: TASK_COMPATIBILITY = TASK_COMPATIBILITY(3i32);
pub const TASK_COMPATIBILITY_V2_2: TASK_COMPATIBILITY = TASK_COMPATIBILITY(4i32);
pub const TASK_COMPATIBILITY_V2_3: TASK_COMPATIBILITY = TASK_COMPATIBILITY(5i32);
pub const TASK_COMPATIBILITY_V2_4: TASK_COMPATIBILITY = TASK_COMPATIBILITY(6i32);
#[repr(transparent)]
pub struct TASK_CREATION(pub i32);
pub const TASK_VALIDATE_ONLY: TASK_CREATION = TASK_CREATION(1i32);
pub const TASK_CREATE: TASK_CREATION = TASK_CREATION(2i32);
pub const TASK_UPDATE: TASK_CREATION = TASK_CREATION(4i32);
pub const TASK_CREATE_OR_UPDATE: TASK_CREATION = TASK_CREATION(6i32);
pub const TASK_DISABLE: TASK_CREATION = TASK_CREATION(8i32);
pub const TASK_DONT_ADD_PRINCIPAL_ACE: TASK_CREATION = TASK_CREATION(16i32);
pub const TASK_IGNORE_REGISTRATION_TRIGGERS: TASK_CREATION = TASK_CREATION(32i32);
pub const TASK_DECEMBER: u32 = 2048u32;
#[repr(transparent)]
pub struct TASK_ENUM_FLAGS(pub i32);
pub const TASK_ENUM_HIDDEN: TASK_ENUM_FLAGS = TASK_ENUM_FLAGS(1i32);
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
#[repr(transparent)]
pub struct TASK_INSTANCES_POLICY(pub i32);
pub const TASK_INSTANCES_PARALLEL: TASK_INSTANCES_POLICY = TASK_INSTANCES_POLICY(0i32);
pub const TASK_INSTANCES_QUEUE: TASK_INSTANCES_POLICY = TASK_INSTANCES_POLICY(1i32);
pub const TASK_INSTANCES_IGNORE_NEW: TASK_INSTANCES_POLICY = TASK_INSTANCES_POLICY(2i32);
pub const TASK_INSTANCES_STOP_EXISTING: TASK_INSTANCES_POLICY = TASK_INSTANCES_POLICY(3i32);
pub const TASK_JANUARY: u32 = 1u32;
pub const TASK_JULY: u32 = 64u32;
pub const TASK_JUNE: u32 = 32u32;
pub const TASK_LAST_WEEK: u32 = 5u32;
#[repr(transparent)]
pub struct TASK_LOGON_TYPE(pub i32);
pub const TASK_LOGON_NONE: TASK_LOGON_TYPE = TASK_LOGON_TYPE(0i32);
pub const TASK_LOGON_PASSWORD: TASK_LOGON_TYPE = TASK_LOGON_TYPE(1i32);
pub const TASK_LOGON_S4U: TASK_LOGON_TYPE = TASK_LOGON_TYPE(2i32);
pub const TASK_LOGON_INTERACTIVE_TOKEN: TASK_LOGON_TYPE = TASK_LOGON_TYPE(3i32);
pub const TASK_LOGON_GROUP: TASK_LOGON_TYPE = TASK_LOGON_TYPE(4i32);
pub const TASK_LOGON_SERVICE_ACCOUNT: TASK_LOGON_TYPE = TASK_LOGON_TYPE(5i32);
pub const TASK_LOGON_INTERACTIVE_TOKEN_OR_PASSWORD: TASK_LOGON_TYPE = TASK_LOGON_TYPE(6i32);
pub const TASK_MARCH: u32 = 4u32;
pub const TASK_MAX_RUN_TIMES: u32 = 1440u32;
pub const TASK_MAY: u32 = 16u32;
pub const TASK_MONDAY: u32 = 2u32;
pub const TASK_NOVEMBER: u32 = 1024u32;
pub const TASK_OCTOBER: u32 = 512u32;
#[repr(transparent)]
pub struct TASK_PROCESSTOKENSID_TYPE(pub i32);
pub const TASK_PROCESSTOKENSID_NONE: TASK_PROCESSTOKENSID_TYPE = TASK_PROCESSTOKENSID_TYPE(0i32);
pub const TASK_PROCESSTOKENSID_UNRESTRICTED: TASK_PROCESSTOKENSID_TYPE = TASK_PROCESSTOKENSID_TYPE(1i32);
pub const TASK_PROCESSTOKENSID_DEFAULT: TASK_PROCESSTOKENSID_TYPE = TASK_PROCESSTOKENSID_TYPE(2i32);
#[repr(transparent)]
pub struct TASK_RUNLEVEL_TYPE(pub i32);
pub const TASK_RUNLEVEL_LUA: TASK_RUNLEVEL_TYPE = TASK_RUNLEVEL_TYPE(0i32);
pub const TASK_RUNLEVEL_HIGHEST: TASK_RUNLEVEL_TYPE = TASK_RUNLEVEL_TYPE(1i32);
#[repr(transparent)]
pub struct TASK_RUN_FLAGS(pub i32);
pub const TASK_RUN_NO_FLAGS: TASK_RUN_FLAGS = TASK_RUN_FLAGS(0i32);
pub const TASK_RUN_AS_SELF: TASK_RUN_FLAGS = TASK_RUN_FLAGS(1i32);
pub const TASK_RUN_IGNORE_CONSTRAINTS: TASK_RUN_FLAGS = TASK_RUN_FLAGS(2i32);
pub const TASK_RUN_USE_SESSION_ID: TASK_RUN_FLAGS = TASK_RUN_FLAGS(4i32);
pub const TASK_RUN_USER_SID: TASK_RUN_FLAGS = TASK_RUN_FLAGS(8i32);
pub const TASK_SATURDAY: u32 = 64u32;
pub const TASK_SECOND_WEEK: u32 = 2u32;
pub const TASK_SEPTEMBER: u32 = 256u32;
#[repr(transparent)]
pub struct TASK_SESSION_STATE_CHANGE_TYPE(pub i32);
pub const TASK_CONSOLE_CONNECT: TASK_SESSION_STATE_CHANGE_TYPE = TASK_SESSION_STATE_CHANGE_TYPE(1i32);
pub const TASK_CONSOLE_DISCONNECT: TASK_SESSION_STATE_CHANGE_TYPE = TASK_SESSION_STATE_CHANGE_TYPE(2i32);
pub const TASK_REMOTE_CONNECT: TASK_SESSION_STATE_CHANGE_TYPE = TASK_SESSION_STATE_CHANGE_TYPE(3i32);
pub const TASK_REMOTE_DISCONNECT: TASK_SESSION_STATE_CHANGE_TYPE = TASK_SESSION_STATE_CHANGE_TYPE(4i32);
pub const TASK_SESSION_LOCK: TASK_SESSION_STATE_CHANGE_TYPE = TASK_SESSION_STATE_CHANGE_TYPE(7i32);
pub const TASK_SESSION_UNLOCK: TASK_SESSION_STATE_CHANGE_TYPE = TASK_SESSION_STATE_CHANGE_TYPE(8i32);
#[repr(transparent)]
pub struct TASK_STATE(pub i32);
pub const TASK_STATE_UNKNOWN: TASK_STATE = TASK_STATE(0i32);
pub const TASK_STATE_DISABLED: TASK_STATE = TASK_STATE(1i32);
pub const TASK_STATE_QUEUED: TASK_STATE = TASK_STATE(2i32);
pub const TASK_STATE_READY: TASK_STATE = TASK_STATE(3i32);
pub const TASK_STATE_RUNNING: TASK_STATE = TASK_STATE(4i32);
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
#[repr(transparent)]
pub struct TASK_TRIGGER_TYPE(pub i32);
pub const TASK_TIME_TRIGGER_ONCE: TASK_TRIGGER_TYPE = TASK_TRIGGER_TYPE(0i32);
pub const TASK_TIME_TRIGGER_DAILY: TASK_TRIGGER_TYPE = TASK_TRIGGER_TYPE(1i32);
pub const TASK_TIME_TRIGGER_WEEKLY: TASK_TRIGGER_TYPE = TASK_TRIGGER_TYPE(2i32);
pub const TASK_TIME_TRIGGER_MONTHLYDATE: TASK_TRIGGER_TYPE = TASK_TRIGGER_TYPE(3i32);
pub const TASK_TIME_TRIGGER_MONTHLYDOW: TASK_TRIGGER_TYPE = TASK_TRIGGER_TYPE(4i32);
pub const TASK_EVENT_TRIGGER_ON_IDLE: TASK_TRIGGER_TYPE = TASK_TRIGGER_TYPE(5i32);
pub const TASK_EVENT_TRIGGER_AT_SYSTEMSTART: TASK_TRIGGER_TYPE = TASK_TRIGGER_TYPE(6i32);
pub const TASK_EVENT_TRIGGER_AT_LOGON: TASK_TRIGGER_TYPE = TASK_TRIGGER_TYPE(7i32);
#[repr(transparent)]
pub struct TASK_TRIGGER_TYPE2(pub i32);
pub const TASK_TRIGGER_EVENT: TASK_TRIGGER_TYPE2 = TASK_TRIGGER_TYPE2(0i32);
pub const TASK_TRIGGER_TIME: TASK_TRIGGER_TYPE2 = TASK_TRIGGER_TYPE2(1i32);
pub const TASK_TRIGGER_DAILY: TASK_TRIGGER_TYPE2 = TASK_TRIGGER_TYPE2(2i32);
pub const TASK_TRIGGER_WEEKLY: TASK_TRIGGER_TYPE2 = TASK_TRIGGER_TYPE2(3i32);
pub const TASK_TRIGGER_MONTHLY: TASK_TRIGGER_TYPE2 = TASK_TRIGGER_TYPE2(4i32);
pub const TASK_TRIGGER_MONTHLYDOW: TASK_TRIGGER_TYPE2 = TASK_TRIGGER_TYPE2(5i32);
pub const TASK_TRIGGER_IDLE: TASK_TRIGGER_TYPE2 = TASK_TRIGGER_TYPE2(6i32);
pub const TASK_TRIGGER_REGISTRATION: TASK_TRIGGER_TYPE2 = TASK_TRIGGER_TYPE2(7i32);
pub const TASK_TRIGGER_BOOT: TASK_TRIGGER_TYPE2 = TASK_TRIGGER_TYPE2(8i32);
pub const TASK_TRIGGER_LOGON: TASK_TRIGGER_TYPE2 = TASK_TRIGGER_TYPE2(9i32);
pub const TASK_TRIGGER_SESSION_STATE_CHANGE: TASK_TRIGGER_TYPE2 = TASK_TRIGGER_TYPE2(11i32);
pub const TASK_TRIGGER_CUSTOM_TRIGGER_01: TASK_TRIGGER_TYPE2 = TASK_TRIGGER_TYPE2(12i32);
pub const TASK_TUESDAY: u32 = 4u32;
pub const TASK_WEDNESDAY: u32 = 8u32;
#[repr(C)]
pub union TRIGGER_TYPE_UNION {
    pub Daily: DAILY,
    pub Weekly: WEEKLY,
    pub MonthlyDate: MONTHLYDATE,
    pub MonthlyDOW: MONTHLYDOW,
}
impl ::core::clone::Clone for TRIGGER_TYPE_UNION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const TaskHandlerPS: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 4071005623,
    data2: 55852,
    data3: 17234,
    data4: [144, 102, 134, 254, 230, 218, 202, 201],
};
pub const TaskHandlerStatusPS: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2668963437,
    data2: 55226,
    data3: 18672,
    data4: [147, 193, 230, 137, 95, 111, 229, 172],
};
pub const TaskScheduler: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 260519583, data2: 42213, data3: 19708, data4: [189, 62, 115, 230, 21, 69, 114, 221] };
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

#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub const CLSID_CTask: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 344708384, data2: 41643, data3: 4558, data4: [177, 31, 0, 170, 0, 83, 5, 3] };
pub const CLSID_CTaskScheduler: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 344708394, data2: 41643, data3: 4558, data4: [177, 31, 0, 170, 0, 83, 5, 3] };
pub struct DAILY(i32);
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
pub struct MONTHLYDATE(i32);
pub struct MONTHLYDOW(i32);
pub struct TASKPAGE(i32);
pub struct TASK_ACTION_TYPE(i32);
pub const TASK_APRIL: u32 = 8u32;
pub const TASK_AUGUST: u32 = 128u32;
pub struct TASK_COMPATIBILITY(i32);
pub struct TASK_CREATION(i32);
pub const TASK_DECEMBER: u32 = 2048u32;
pub struct TASK_ENUM_FLAGS(i32);
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
pub struct TASK_INSTANCES_POLICY(i32);
pub const TASK_JANUARY: u32 = 1u32;
pub const TASK_JULY: u32 = 64u32;
pub const TASK_JUNE: u32 = 32u32;
pub const TASK_LAST_WEEK: u32 = 5u32;
pub struct TASK_LOGON_TYPE(i32);
pub const TASK_MARCH: u32 = 4u32;
pub const TASK_MAX_RUN_TIMES: u32 = 1440u32;
pub const TASK_MAY: u32 = 16u32;
pub const TASK_MONDAY: u32 = 2u32;
pub const TASK_NOVEMBER: u32 = 1024u32;
pub const TASK_OCTOBER: u32 = 512u32;
pub struct TASK_PROCESSTOKENSID_TYPE(i32);
pub struct TASK_RUNLEVEL_TYPE(i32);
pub struct TASK_RUN_FLAGS(i32);
pub const TASK_SATURDAY: u32 = 64u32;
pub const TASK_SECOND_WEEK: u32 = 2u32;
pub const TASK_SEPTEMBER: u32 = 256u32;
pub struct TASK_SESSION_STATE_CHANGE_TYPE(i32);
pub struct TASK_STATE(i32);
pub const TASK_SUNDAY: u32 = 1u32;
pub const TASK_THIRD_WEEK: u32 = 3u32;
pub const TASK_THURSDAY: u32 = 16u32;
pub struct TASK_TRIGGER(i32);
pub const TASK_TRIGGER_FLAG_DISABLED: u32 = 4u32;
pub const TASK_TRIGGER_FLAG_HAS_END_DATE: u32 = 1u32;
pub const TASK_TRIGGER_FLAG_KILL_AT_DURATION_END: u32 = 2u32;
pub struct TASK_TRIGGER_TYPE(i32);
pub struct TASK_TRIGGER_TYPE2(i32);
pub const TASK_TUESDAY: u32 = 4u32;
pub const TASK_WEDNESDAY: u32 = 8u32;
pub struct TRIGGER_TYPE_UNION(i32);
pub struct TaskHandlerPS(i32);
pub struct TaskHandlerStatusPS(i32);
pub struct TaskScheduler(i32);
pub struct WEEKLY(i32);

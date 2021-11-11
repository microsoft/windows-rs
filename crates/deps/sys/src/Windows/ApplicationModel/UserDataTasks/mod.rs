#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "ApplicationModel_UserDataTasks_DataProvider")]
pub mod DataProvider;
#[link(name = "windows")]
extern "system" {
    fn IUserDataTask();
    fn IUserDataTaskBatch();
    fn IUserDataTaskList();
    fn IUserDataTaskListLimitedWriteOperations();
    fn IUserDataTaskListSyncManager();
    fn IUserDataTaskManager();
    fn IUserDataTaskManagerStatics();
    fn IUserDataTaskQueryOptions();
    fn IUserDataTaskReader();
    fn IUserDataTaskRecurrenceProperties();
    fn IUserDataTaskRegenerationProperties();
    fn IUserDataTaskStore();
    fn UserDataTask();
    fn UserDataTaskBatch();
    fn UserDataTaskDaysOfWeek();
    fn UserDataTaskDetailsKind();
    fn UserDataTaskKind();
    fn UserDataTaskList();
    fn UserDataTaskListLimitedWriteOperations();
    fn UserDataTaskListOtherAppReadAccess();
    fn UserDataTaskListOtherAppWriteAccess();
    fn UserDataTaskListSyncManager();
    fn UserDataTaskListSyncStatus();
    fn UserDataTaskManager();
    fn UserDataTaskPriority();
    fn UserDataTaskQueryKind();
    fn UserDataTaskQueryOptions();
    fn UserDataTaskQuerySortProperty();
    fn UserDataTaskReader();
    fn UserDataTaskRecurrenceProperties();
    fn UserDataTaskRecurrenceUnit();
    fn UserDataTaskRegenerationProperties();
    fn UserDataTaskRegenerationUnit();
    fn UserDataTaskSensitivity();
    fn UserDataTaskStore();
    fn UserDataTaskStoreAccessType();
    fn UserDataTaskWeekOfMonth();
}

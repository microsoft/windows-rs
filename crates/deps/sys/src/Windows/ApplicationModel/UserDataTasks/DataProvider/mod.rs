#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IUserDataTaskDataProviderConnection();
    fn IUserDataTaskDataProviderTriggerDetails();
    fn IUserDataTaskListCompleteTaskRequest();
    fn IUserDataTaskListCompleteTaskRequestEventArgs();
    fn IUserDataTaskListCreateOrUpdateTaskRequest();
    fn IUserDataTaskListCreateOrUpdateTaskRequestEventArgs();
    fn IUserDataTaskListDeleteTaskRequest();
    fn IUserDataTaskListDeleteTaskRequestEventArgs();
    fn IUserDataTaskListSkipOccurrenceRequest();
    fn IUserDataTaskListSkipOccurrenceRequestEventArgs();
    fn IUserDataTaskListSyncManagerSyncRequest();
    fn IUserDataTaskListSyncManagerSyncRequestEventArgs();
    fn UserDataTaskDataProviderConnection();
    fn UserDataTaskDataProviderTriggerDetails();
    fn UserDataTaskListCompleteTaskRequest();
    fn UserDataTaskListCompleteTaskRequestEventArgs();
    fn UserDataTaskListCreateOrUpdateTaskRequest();
    fn UserDataTaskListCreateOrUpdateTaskRequestEventArgs();
    fn UserDataTaskListDeleteTaskRequest();
    fn UserDataTaskListDeleteTaskRequestEventArgs();
    fn UserDataTaskListSkipOccurrenceRequest();
    fn UserDataTaskListSkipOccurrenceRequestEventArgs();
    fn UserDataTaskListSyncManagerSyncRequest();
    fn UserDataTaskListSyncManagerSyncRequestEventArgs();
}

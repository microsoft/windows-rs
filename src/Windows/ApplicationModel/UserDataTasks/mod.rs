#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "ApplicationModel_UserDataTasks_DataProvider")]
pub mod DataProvider;
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserDataTask(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserDataTask {
    type Vtable = IUserDataTask_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2087028177, 57556, 20377, [174, 226, 188, 45, 93, 218, 223, 76]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTask_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut UserDataTaskDetailsKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: UserDataTaskDetailsKind) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut UserDataTaskKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut UserDataTaskPriority) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: UserDataTaskPriority) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut UserDataTaskSensitivity) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: UserDataTaskSensitivity) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserDataTaskBatch(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserDataTaskBatch {
    type Vtable = IUserDataTaskBatch_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(942515710, 8373, 17180, [143, 66, 165, 210, 146, 236, 147, 12]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskBatch_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserDataTaskList(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserDataTaskList {
    type Vtable = IUserDataTaskList_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1229008441, 31773, 19953, [190, 211, 49, 75, 124, 191, 94, 78]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskList_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut UserDataTaskListOtherAppReadAccess) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: UserDataTaskListOtherAppReadAccess) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut UserDataTaskListOtherAppWriteAccess) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: UserDataTaskListOtherAppWriteAccess) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, options: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, userdatatask: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, userdatatask: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, userdatataskid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserDataTaskListLimitedWriteOperations(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserDataTaskListLimitedWriteOperations {
    type Vtable = IUserDataTaskListLimitedWriteOperations_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2057463794, 24696, 16771, [145, 158, 79, 41, 241, 156, 250, 233]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskListLimitedWriteOperations_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, userdatataskid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, userdatatask: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, userdatataskid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, userdatataskid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserDataTaskListSyncManager(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserDataTaskListSyncManager {
    type Vtable = IUserDataTaskListSyncManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2388204181, 7631, 18079, [147, 236, 186, 72, 187, 85, 60, 107]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskListSyncManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut UserDataTaskListSyncStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: UserDataTaskListSyncStatus) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserDataTaskManager(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserDataTaskManager {
    type Vtable = IUserDataTaskManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2219952404, 58891, 18601, [146, 17, 127, 184, 165, 108, 184, 76]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, accesstype: UserDataTaskStoreAccessType, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserDataTaskManagerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserDataTaskManagerStatics {
    type Vtable = IUserDataTaskManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3008707064, 50434, 18428, [168, 30, 16, 8, 131, 113, 157, 85]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, user: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserDataTaskQueryOptions(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserDataTaskQueryOptions {
    type Vtable = IUserDataTaskQueryOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2510235629, 37018, 19760, [140, 27, 51, 29, 143, 230, 103, 226]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskQueryOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut UserDataTaskQuerySortProperty) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: UserDataTaskQuerySortProperty) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut UserDataTaskQueryKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: UserDataTaskQueryKind) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserDataTaskReader(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserDataTaskReader {
    type Vtable = IUserDataTaskReader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(65439921, 19663, 17664, [136, 59, 231, 98, 144, 207, 237, 99]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskReader_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserDataTaskRecurrenceProperties(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserDataTaskRecurrenceProperties {
    type Vtable = IUserDataTaskRecurrenceProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1944027312, 10182, 16590, [177, 73, 156, 212, 20, 133, 166, 158]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskRecurrenceProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut UserDataTaskRecurrenceUnit) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: UserDataTaskRecurrenceUnit) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserDataTaskRegenerationProperties(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserDataTaskRegenerationProperties {
    type Vtable = IUserDataTaskRegenerationProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2460680199, 2318, 18180, [187, 92, 132, 252, 11, 13, 156, 49]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskRegenerationProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut UserDataTaskRegenerationUnit) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: UserDataTaskRegenerationUnit) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserDataTaskStore(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUserDataTaskStore {
    type Vtable = IUserDataTaskStore_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4033518768, 61915, 17850, [138, 98, 8, 96, 4, 192, 33, 61]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskStore_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, userdataaccountid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tasklistid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct UserDataTask(::windows::runtime::IInspectable);
impl UserDataTask {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<UserDataTask, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
    pub fn ListId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
    pub fn RemoteId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
    pub fn SetRemoteId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`, `Foundation`*"]
    pub fn CompletedDate(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`, `Foundation`*"]
    pub fn SetCompletedDate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
    pub fn Details(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
    pub fn SetDetails<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
    pub fn DetailsKind(&self) -> ::windows::runtime::Result<UserDataTaskDetailsKind> {
        let this = self;
        unsafe {
            let mut result__: UserDataTaskDetailsKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UserDataTaskDetailsKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
    pub fn SetDetailsKind(&self, value: UserDataTaskDetailsKind) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`, `Foundation`*"]
    pub fn DueDate(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`, `Foundation`*"]
    pub fn SetDueDate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<UserDataTaskKind> {
        let this = self;
        unsafe {
            let mut result__: UserDataTaskKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UserDataTaskKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
    pub fn Priority(&self) -> ::windows::runtime::Result<UserDataTaskPriority> {
        let this = self;
        unsafe {
            let mut result__: UserDataTaskPriority = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UserDataTaskPriority>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
    pub fn SetPriority(&self, value: UserDataTaskPriority) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
    pub fn RecurrenceProperties(&self) -> ::windows::runtime::Result<UserDataTaskRecurrenceProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UserDataTaskRecurrenceProperties>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
    pub fn SetRecurrenceProperties<'a, Param0: ::windows::runtime::IntoParam<'a, UserDataTaskRecurrenceProperties>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
    pub fn RegenerationProperties(&self) -> ::windows::runtime::Result<UserDataTaskRegenerationProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UserDataTaskRegenerationProperties>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
    pub fn SetRegenerationProperties<'a, Param0: ::windows::runtime::IntoParam<'a, UserDataTaskRegenerationProperties>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`, `Foundation`*"]
    pub fn Reminder(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`, `Foundation`*"]
    pub fn SetReminder<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
    pub fn Sensitivity(&self) -> ::windows::runtime::Result<UserDataTaskSensitivity> {
        let this = self;
        unsafe {
            let mut result__: UserDataTaskSensitivity = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UserDataTaskSensitivity>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
    pub fn SetSensitivity(&self, value: UserDataTaskSensitivity) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
    pub fn Subject(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
    pub fn SetSubject<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`, `Foundation`*"]
    pub fn StartDate(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`, `Foundation`*"]
    pub fn SetStartDate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).32)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for UserDataTask {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.UserDataTask;{7c6585d1-e0d4-4f99-aee2-bc2d5ddadf4c})");
}
unsafe impl ::windows::runtime::Interface for UserDataTask {
    type Vtable = IUserDataTask_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2087028177, 57556, 20377, [174, 226, 188, 45, 93, 218, 223, 76]);
}
impl ::windows::runtime::RuntimeName for UserDataTask {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTask";
}
unsafe impl ::std::marker::Send for UserDataTask {}
unsafe impl ::std::marker::Sync for UserDataTask {}
#[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct UserDataTaskBatch(::windows::runtime::IInspectable);
impl UserDataTaskBatch {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`, `Foundation_Collections`*"]
    pub fn Tasks(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVectorView<UserDataTask>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<UserDataTask>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for UserDataTaskBatch {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.UserDataTaskBatch;{382da5fe-20b5-431c-8f42-a5d292ec930c})");
}
unsafe impl ::windows::runtime::Interface for UserDataTaskBatch {
    type Vtable = IUserDataTaskBatch_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(942515710, 8373, 17180, [143, 66, 165, 210, 146, 236, 147, 12]);
}
impl ::windows::runtime::RuntimeName for UserDataTaskBatch {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTaskBatch";
}
unsafe impl ::std::marker::Send for UserDataTaskBatch {}
unsafe impl ::std::marker::Sync for UserDataTaskBatch {}
#[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct UserDataTaskDaysOfWeek(pub u32);
impl UserDataTaskDaysOfWeek {
    pub const None: UserDataTaskDaysOfWeek = UserDataTaskDaysOfWeek(0u32);
    pub const Sunday: UserDataTaskDaysOfWeek = UserDataTaskDaysOfWeek(1u32);
    pub const Monday: UserDataTaskDaysOfWeek = UserDataTaskDaysOfWeek(2u32);
    pub const Tuesday: UserDataTaskDaysOfWeek = UserDataTaskDaysOfWeek(4u32);
    pub const Wednesday: UserDataTaskDaysOfWeek = UserDataTaskDaysOfWeek(8u32);
    pub const Thursday: UserDataTaskDaysOfWeek = UserDataTaskDaysOfWeek(16u32);
    pub const Friday: UserDataTaskDaysOfWeek = UserDataTaskDaysOfWeek(32u32);
    pub const Saturday: UserDataTaskDaysOfWeek = UserDataTaskDaysOfWeek(64u32);
}
impl ::std::convert::From<u32> for UserDataTaskDaysOfWeek {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UserDataTaskDaysOfWeek {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for UserDataTaskDaysOfWeek {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskDaysOfWeek;u4)");
}
impl ::windows::runtime::DefaultType for UserDataTaskDaysOfWeek {
    type DefaultType = Self;
}
impl ::std::ops::BitOr for UserDataTaskDaysOfWeek {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for UserDataTaskDaysOfWeek {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for UserDataTaskDaysOfWeek {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for UserDataTaskDaysOfWeek {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for UserDataTaskDaysOfWeek {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct UserDataTaskDetailsKind(pub i32);
impl UserDataTaskDetailsKind {
    pub const PlainText: UserDataTaskDetailsKind = UserDataTaskDetailsKind(0i32);
    pub const Html: UserDataTaskDetailsKind = UserDataTaskDetailsKind(1i32);
}
impl ::std::convert::From<i32> for UserDataTaskDetailsKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UserDataTaskDetailsKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for UserDataTaskDetailsKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskDetailsKind;i4)");
}
impl ::windows::runtime::DefaultType for UserDataTaskDetailsKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct UserDataTaskKind(pub i32);
impl UserDataTaskKind {
    pub const Single: UserDataTaskKind = UserDataTaskKind(0i32);
    pub const Recurring: UserDataTaskKind = UserDataTaskKind(1i32);
    pub const Regenerating: UserDataTaskKind = UserDataTaskKind(2i32);
}
impl ::std::convert::From<i32> for UserDataTaskKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UserDataTaskKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for UserDataTaskKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskKind;i4)");
}
impl ::windows::runtime::DefaultType for UserDataTaskKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct UserDataTaskList(::windows::runtime::IInspectable);
impl UserDataTaskList {
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
    pub fn UserDataAccountId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
    pub fn DisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
    pub fn SetDisplayName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
    pub fn SourceDisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
    pub fn OtherAppReadAccess(&self) -> ::windows::runtime::Result<UserDataTaskListOtherAppReadAccess> {
        let this = self;
        unsafe {
            let mut result__: UserDataTaskListOtherAppReadAccess = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UserDataTaskListOtherAppReadAccess>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
    pub fn SetOtherAppReadAccess(&self, value: UserDataTaskListOtherAppReadAccess) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
    pub fn OtherAppWriteAccess(&self) -> ::windows::runtime::Result<UserDataTaskListOtherAppWriteAccess> {
        let this = self;
        unsafe {
            let mut result__: UserDataTaskListOtherAppWriteAccess = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UserDataTaskListOtherAppWriteAccess>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
    pub fn SetOtherAppWriteAccess(&self, value: UserDataTaskListOtherAppWriteAccess) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
    pub fn LimitedWriteOperations(&self) -> ::windows::runtime::Result<UserDataTaskListLimitedWriteOperations> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UserDataTaskListLimitedWriteOperations>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
    pub fn SyncManager(&self) -> ::windows::runtime::Result<UserDataTaskListSyncManager> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UserDataTaskListSyncManager>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`, `Foundation`*"]
    pub fn RegisterSyncManagerAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
    pub fn GetTaskReader(&self) -> ::windows::runtime::Result<UserDataTaskReader> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UserDataTaskReader>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
    pub fn GetTaskReaderWithOptions<'a, Param0: ::windows::runtime::IntoParam<'a, UserDataTaskQueryOptions>>(&self, options: Param0) -> ::windows::runtime::Result<UserDataTaskReader> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), options.into_param().abi(), &mut result__).from_abi::<UserDataTaskReader>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`, `Foundation`*"]
    pub fn GetTaskAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, userdatatask: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<UserDataTask>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), userdatatask.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<UserDataTask>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`, `Foundation`*"]
    pub fn SaveTaskAsync<'a, Param0: ::windows::runtime::IntoParam<'a, UserDataTask>>(&self, userdatatask: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), userdatatask.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`, `Foundation`*"]
    pub fn DeleteTaskAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, userdatataskid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), userdatataskid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`, `Foundation`*"]
    pub fn DeleteAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`, `Foundation`*"]
    pub fn SaveAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for UserDataTaskList {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.UserDataTaskList;{49412e39-7c1d-4df1-bed3-314b7cbf5e4e})");
}
unsafe impl ::windows::runtime::Interface for UserDataTaskList {
    type Vtable = IUserDataTaskList_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1229008441, 31773, 19953, [190, 211, 49, 75, 124, 191, 94, 78]);
}
impl ::windows::runtime::RuntimeName for UserDataTaskList {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTaskList";
}
unsafe impl ::std::marker::Send for UserDataTaskList {}
unsafe impl ::std::marker::Sync for UserDataTaskList {}
#[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct UserDataTaskListLimitedWriteOperations(::windows::runtime::IInspectable);
impl UserDataTaskListLimitedWriteOperations {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`, `Foundation`*"]
    pub fn TryCompleteTaskAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, userdatataskid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), userdatataskid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`, `Foundation`*"]
    pub fn TryCreateOrUpdateTaskAsync<'a, Param0: ::windows::runtime::IntoParam<'a, UserDataTask>>(&self, userdatatask: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), userdatatask.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`, `Foundation`*"]
    pub fn TryDeleteTaskAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, userdatataskid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), userdatataskid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`, `Foundation`*"]
    pub fn TrySkipOccurrenceAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, userdatataskid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), userdatataskid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for UserDataTaskListLimitedWriteOperations {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.UserDataTaskListLimitedWriteOperations;{7aa267f2-6078-4183-919e-4f29f19cfae9})");
}
unsafe impl ::windows::runtime::Interface for UserDataTaskListLimitedWriteOperations {
    type Vtable = IUserDataTaskListLimitedWriteOperations_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2057463794, 24696, 16771, [145, 158, 79, 41, 241, 156, 250, 233]);
}
impl ::windows::runtime::RuntimeName for UserDataTaskListLimitedWriteOperations {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTaskListLimitedWriteOperations";
}
unsafe impl ::std::marker::Send for UserDataTaskListLimitedWriteOperations {}
unsafe impl ::std::marker::Sync for UserDataTaskListLimitedWriteOperations {}
#[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct UserDataTaskListOtherAppReadAccess(pub i32);
impl UserDataTaskListOtherAppReadAccess {
    pub const Full: UserDataTaskListOtherAppReadAccess = UserDataTaskListOtherAppReadAccess(0i32);
    pub const SystemOnly: UserDataTaskListOtherAppReadAccess = UserDataTaskListOtherAppReadAccess(1i32);
    pub const None: UserDataTaskListOtherAppReadAccess = UserDataTaskListOtherAppReadAccess(2i32);
}
impl ::std::convert::From<i32> for UserDataTaskListOtherAppReadAccess {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UserDataTaskListOtherAppReadAccess {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for UserDataTaskListOtherAppReadAccess {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskListOtherAppReadAccess;i4)");
}
impl ::windows::runtime::DefaultType for UserDataTaskListOtherAppReadAccess {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct UserDataTaskListOtherAppWriteAccess(pub i32);
impl UserDataTaskListOtherAppWriteAccess {
    pub const Limited: UserDataTaskListOtherAppWriteAccess = UserDataTaskListOtherAppWriteAccess(0i32);
    pub const None: UserDataTaskListOtherAppWriteAccess = UserDataTaskListOtherAppWriteAccess(1i32);
}
impl ::std::convert::From<i32> for UserDataTaskListOtherAppWriteAccess {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UserDataTaskListOtherAppWriteAccess {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for UserDataTaskListOtherAppWriteAccess {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskListOtherAppWriteAccess;i4)");
}
impl ::windows::runtime::DefaultType for UserDataTaskListOtherAppWriteAccess {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct UserDataTaskListSyncManager(::windows::runtime::IInspectable);
impl UserDataTaskListSyncManager {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`, `Foundation`*"]
    pub fn LastAttemptedSyncTime(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`, `Foundation`*"]
    pub fn SetLastAttemptedSyncTime<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::DateTime>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`, `Foundation`*"]
    pub fn LastSuccessfulSyncTime(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`, `Foundation`*"]
    pub fn SetLastSuccessfulSyncTime<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::DateTime>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<UserDataTaskListSyncStatus> {
        let this = self;
        unsafe {
            let mut result__: UserDataTaskListSyncStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UserDataTaskListSyncStatus>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
    pub fn SetStatus(&self, value: UserDataTaskListSyncStatus) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`, `Foundation`*"]
    pub fn SyncAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`, `Foundation`*"]
    pub fn SyncStatusChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::TypedEventHandler<UserDataTaskListSyncManager, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`, `Foundation`*"]
    pub fn RemoveSyncStatusChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for UserDataTaskListSyncManager {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.UserDataTaskListSyncManager;{8e591a95-1dcf-469f-93ec-ba48bb553c6b})");
}
unsafe impl ::windows::runtime::Interface for UserDataTaskListSyncManager {
    type Vtable = IUserDataTaskListSyncManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2388204181, 7631, 18079, [147, 236, 186, 72, 187, 85, 60, 107]);
}
impl ::windows::runtime::RuntimeName for UserDataTaskListSyncManager {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTaskListSyncManager";
}
unsafe impl ::std::marker::Send for UserDataTaskListSyncManager {}
unsafe impl ::std::marker::Sync for UserDataTaskListSyncManager {}
#[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct UserDataTaskListSyncStatus(pub i32);
impl UserDataTaskListSyncStatus {
    pub const Idle: UserDataTaskListSyncStatus = UserDataTaskListSyncStatus(0i32);
    pub const Syncing: UserDataTaskListSyncStatus = UserDataTaskListSyncStatus(1i32);
    pub const UpToDate: UserDataTaskListSyncStatus = UserDataTaskListSyncStatus(2i32);
    pub const AuthenticationError: UserDataTaskListSyncStatus = UserDataTaskListSyncStatus(3i32);
    pub const PolicyError: UserDataTaskListSyncStatus = UserDataTaskListSyncStatus(4i32);
    pub const UnknownError: UserDataTaskListSyncStatus = UserDataTaskListSyncStatus(5i32);
}
impl ::std::convert::From<i32> for UserDataTaskListSyncStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UserDataTaskListSyncStatus {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for UserDataTaskListSyncStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskListSyncStatus;i4)");
}
impl ::windows::runtime::DefaultType for UserDataTaskListSyncStatus {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct UserDataTaskManager(::windows::runtime::IInspectable);
impl UserDataTaskManager {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`, `Foundation`*"]
    pub fn RequestStoreAsync(&self, accesstype: UserDataTaskStoreAccessType) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<UserDataTaskStore>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), accesstype, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<UserDataTaskStore>>(result__)
        }
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`, `System`*"]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
    pub fn GetDefault() -> ::windows::runtime::Result<UserDataTaskManager> {
        Self::IUserDataTaskManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UserDataTaskManager>(result__)
        })
    }
    #[cfg(feature = "System")]
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`, `System`*"]
    pub fn GetForUser<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::User>>(user: Param0) -> ::windows::runtime::Result<UserDataTaskManager> {
        Self::IUserDataTaskManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<UserDataTaskManager>(result__)
        })
    }
    pub fn IUserDataTaskManagerStatics<R, F: FnOnce(&IUserDataTaskManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<UserDataTaskManager, IUserDataTaskManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for UserDataTaskManager {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.UserDataTaskManager;{8451c914-e60b-48a9-9211-7fb8a56cb84c})");
}
unsafe impl ::windows::runtime::Interface for UserDataTaskManager {
    type Vtable = IUserDataTaskManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2219952404, 58891, 18601, [146, 17, 127, 184, 165, 108, 184, 76]);
}
impl ::windows::runtime::RuntimeName for UserDataTaskManager {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTaskManager";
}
unsafe impl ::std::marker::Send for UserDataTaskManager {}
unsafe impl ::std::marker::Sync for UserDataTaskManager {}
#[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct UserDataTaskPriority(pub i32);
impl UserDataTaskPriority {
    pub const Normal: UserDataTaskPriority = UserDataTaskPriority(0i32);
    pub const Low: UserDataTaskPriority = UserDataTaskPriority(-1i32);
    pub const High: UserDataTaskPriority = UserDataTaskPriority(1i32);
}
impl ::std::convert::From<i32> for UserDataTaskPriority {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UserDataTaskPriority {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for UserDataTaskPriority {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskPriority;i4)");
}
impl ::windows::runtime::DefaultType for UserDataTaskPriority {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct UserDataTaskQueryKind(pub i32);
impl UserDataTaskQueryKind {
    pub const All: UserDataTaskQueryKind = UserDataTaskQueryKind(0i32);
    pub const Incomplete: UserDataTaskQueryKind = UserDataTaskQueryKind(1i32);
    pub const Complete: UserDataTaskQueryKind = UserDataTaskQueryKind(2i32);
}
impl ::std::convert::From<i32> for UserDataTaskQueryKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UserDataTaskQueryKind {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for UserDataTaskQueryKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskQueryKind;i4)");
}
impl ::windows::runtime::DefaultType for UserDataTaskQueryKind {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct UserDataTaskQueryOptions(::windows::runtime::IInspectable);
impl UserDataTaskQueryOptions {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<UserDataTaskQueryOptions, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
    pub fn SortProperty(&self) -> ::windows::runtime::Result<UserDataTaskQuerySortProperty> {
        let this = self;
        unsafe {
            let mut result__: UserDataTaskQuerySortProperty = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UserDataTaskQuerySortProperty>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
    pub fn SetSortProperty(&self, value: UserDataTaskQuerySortProperty) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<UserDataTaskQueryKind> {
        let this = self;
        unsafe {
            let mut result__: UserDataTaskQueryKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UserDataTaskQueryKind>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
    pub fn SetKind(&self, value: UserDataTaskQueryKind) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for UserDataTaskQueryOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.UserDataTaskQueryOptions;{959f27ed-909a-4d30-8c1b-331d8fe667e2})");
}
unsafe impl ::windows::runtime::Interface for UserDataTaskQueryOptions {
    type Vtable = IUserDataTaskQueryOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2510235629, 37018, 19760, [140, 27, 51, 29, 143, 230, 103, 226]);
}
impl ::windows::runtime::RuntimeName for UserDataTaskQueryOptions {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTaskQueryOptions";
}
unsafe impl ::std::marker::Send for UserDataTaskQueryOptions {}
unsafe impl ::std::marker::Sync for UserDataTaskQueryOptions {}
#[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct UserDataTaskQuerySortProperty(pub i32);
impl UserDataTaskQuerySortProperty {
    pub const DueDate: UserDataTaskQuerySortProperty = UserDataTaskQuerySortProperty(0i32);
}
impl ::std::convert::From<i32> for UserDataTaskQuerySortProperty {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UserDataTaskQuerySortProperty {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for UserDataTaskQuerySortProperty {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskQuerySortProperty;i4)");
}
impl ::windows::runtime::DefaultType for UserDataTaskQuerySortProperty {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct UserDataTaskReader(::windows::runtime::IInspectable);
impl UserDataTaskReader {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`, `Foundation`*"]
    pub fn ReadBatchAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<UserDataTaskBatch>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<UserDataTaskBatch>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for UserDataTaskReader {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.UserDataTaskReader;{03e688b1-4ccf-4500-883b-e76290cfed63})");
}
unsafe impl ::windows::runtime::Interface for UserDataTaskReader {
    type Vtable = IUserDataTaskReader_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(65439921, 19663, 17664, [136, 59, 231, 98, 144, 207, 237, 99]);
}
impl ::windows::runtime::RuntimeName for UserDataTaskReader {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTaskReader";
}
unsafe impl ::std::marker::Send for UserDataTaskReader {}
unsafe impl ::std::marker::Sync for UserDataTaskReader {}
#[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct UserDataTaskRecurrenceProperties(::windows::runtime::IInspectable);
impl UserDataTaskRecurrenceProperties {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<UserDataTaskRecurrenceProperties, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
    pub fn Unit(&self) -> ::windows::runtime::Result<UserDataTaskRecurrenceUnit> {
        let this = self;
        unsafe {
            let mut result__: UserDataTaskRecurrenceUnit = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UserDataTaskRecurrenceUnit>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
    pub fn SetUnit(&self, value: UserDataTaskRecurrenceUnit) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`, `Foundation`*"]
    pub fn Occurrences(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`, `Foundation`*"]
    pub fn SetOccurrences<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<i32>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`, `Foundation`*"]
    pub fn Until(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`, `Foundation`*"]
    pub fn SetUntil<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
    pub fn Interval(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
    pub fn SetInterval(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`, `Foundation`*"]
    pub fn DaysOfWeek(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<UserDataTaskDaysOfWeek>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<UserDataTaskDaysOfWeek>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`, `Foundation`*"]
    pub fn SetDaysOfWeek<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<UserDataTaskDaysOfWeek>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`, `Foundation`*"]
    pub fn WeekOfMonth(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<UserDataTaskWeekOfMonth>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<UserDataTaskWeekOfMonth>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`, `Foundation`*"]
    pub fn SetWeekOfMonth<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<UserDataTaskWeekOfMonth>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`, `Foundation`*"]
    pub fn Month(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`, `Foundation`*"]
    pub fn SetMonth<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<i32>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`, `Foundation`*"]
    pub fn Day(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`, `Foundation`*"]
    pub fn SetDay<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<i32>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for UserDataTaskRecurrenceProperties {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.UserDataTaskRecurrenceProperties;{73df80b0-27c6-40ce-b149-9cd41485a69e})");
}
unsafe impl ::windows::runtime::Interface for UserDataTaskRecurrenceProperties {
    type Vtable = IUserDataTaskRecurrenceProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1944027312, 10182, 16590, [177, 73, 156, 212, 20, 133, 166, 158]);
}
impl ::windows::runtime::RuntimeName for UserDataTaskRecurrenceProperties {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTaskRecurrenceProperties";
}
unsafe impl ::std::marker::Send for UserDataTaskRecurrenceProperties {}
unsafe impl ::std::marker::Sync for UserDataTaskRecurrenceProperties {}
#[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct UserDataTaskRecurrenceUnit(pub i32);
impl UserDataTaskRecurrenceUnit {
    pub const Daily: UserDataTaskRecurrenceUnit = UserDataTaskRecurrenceUnit(0i32);
    pub const Weekly: UserDataTaskRecurrenceUnit = UserDataTaskRecurrenceUnit(1i32);
    pub const Monthly: UserDataTaskRecurrenceUnit = UserDataTaskRecurrenceUnit(2i32);
    pub const MonthlyOnDay: UserDataTaskRecurrenceUnit = UserDataTaskRecurrenceUnit(3i32);
    pub const Yearly: UserDataTaskRecurrenceUnit = UserDataTaskRecurrenceUnit(4i32);
    pub const YearlyOnDay: UserDataTaskRecurrenceUnit = UserDataTaskRecurrenceUnit(5i32);
}
impl ::std::convert::From<i32> for UserDataTaskRecurrenceUnit {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UserDataTaskRecurrenceUnit {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for UserDataTaskRecurrenceUnit {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskRecurrenceUnit;i4)");
}
impl ::windows::runtime::DefaultType for UserDataTaskRecurrenceUnit {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct UserDataTaskRegenerationProperties(::windows::runtime::IInspectable);
impl UserDataTaskRegenerationProperties {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<UserDataTaskRegenerationProperties, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
    pub fn Unit(&self) -> ::windows::runtime::Result<UserDataTaskRegenerationUnit> {
        let this = self;
        unsafe {
            let mut result__: UserDataTaskRegenerationUnit = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UserDataTaskRegenerationUnit>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
    pub fn SetUnit(&self, value: UserDataTaskRegenerationUnit) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`, `Foundation`*"]
    pub fn Occurrences(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`, `Foundation`*"]
    pub fn SetOccurrences<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<i32>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`, `Foundation`*"]
    pub fn Until(&self) -> ::windows::runtime::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`, `Foundation`*"]
    pub fn SetUntil<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
    pub fn Interval(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
    pub fn SetInterval(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for UserDataTaskRegenerationProperties {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.UserDataTaskRegenerationProperties;{92ab0007-090e-4704-bb5c-84fc0b0d9c31})");
}
unsafe impl ::windows::runtime::Interface for UserDataTaskRegenerationProperties {
    type Vtable = IUserDataTaskRegenerationProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2460680199, 2318, 18180, [187, 92, 132, 252, 11, 13, 156, 49]);
}
impl ::windows::runtime::RuntimeName for UserDataTaskRegenerationProperties {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTaskRegenerationProperties";
}
unsafe impl ::std::marker::Send for UserDataTaskRegenerationProperties {}
unsafe impl ::std::marker::Sync for UserDataTaskRegenerationProperties {}
#[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct UserDataTaskRegenerationUnit(pub i32);
impl UserDataTaskRegenerationUnit {
    pub const Daily: UserDataTaskRegenerationUnit = UserDataTaskRegenerationUnit(0i32);
    pub const Weekly: UserDataTaskRegenerationUnit = UserDataTaskRegenerationUnit(1i32);
    pub const Monthly: UserDataTaskRegenerationUnit = UserDataTaskRegenerationUnit(2i32);
    pub const Yearly: UserDataTaskRegenerationUnit = UserDataTaskRegenerationUnit(4i32);
}
impl ::std::convert::From<i32> for UserDataTaskRegenerationUnit {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UserDataTaskRegenerationUnit {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for UserDataTaskRegenerationUnit {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskRegenerationUnit;i4)");
}
impl ::windows::runtime::DefaultType for UserDataTaskRegenerationUnit {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct UserDataTaskSensitivity(pub i32);
impl UserDataTaskSensitivity {
    pub const Public: UserDataTaskSensitivity = UserDataTaskSensitivity(0i32);
    pub const Private: UserDataTaskSensitivity = UserDataTaskSensitivity(1i32);
}
impl ::std::convert::From<i32> for UserDataTaskSensitivity {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UserDataTaskSensitivity {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for UserDataTaskSensitivity {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskSensitivity;i4)");
}
impl ::windows::runtime::DefaultType for UserDataTaskSensitivity {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct UserDataTaskStore(::windows::runtime::IInspectable);
impl UserDataTaskStore {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`, `Foundation`*"]
    pub fn CreateListAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<UserDataTaskList>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<UserDataTaskList>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`, `Foundation`*"]
    pub fn CreateListInAccountAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, userdataaccountid: Param1) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<UserDataTaskList>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), name.into_param().abi(), userdataaccountid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<UserDataTaskList>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`, `Foundation`, `Foundation_Collections`*"]
    pub fn FindListsAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<UserDataTaskList>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<UserDataTaskList>>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `ApplicationModel_UserDataTasks`, `Foundation`*"]
    pub fn GetListAsync<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, tasklistid: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<UserDataTaskList>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), tasklistid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<UserDataTaskList>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for UserDataTaskStore {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.UserDataTaskStore;{f06a9cb0-f1db-45ba-8a62-086004c0213d})");
}
unsafe impl ::windows::runtime::Interface for UserDataTaskStore {
    type Vtable = IUserDataTaskStore_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4033518768, 61915, 17850, [138, 98, 8, 96, 4, 192, 33, 61]);
}
impl ::windows::runtime::RuntimeName for UserDataTaskStore {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTaskStore";
}
unsafe impl ::std::marker::Send for UserDataTaskStore {}
unsafe impl ::std::marker::Sync for UserDataTaskStore {}
#[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct UserDataTaskStoreAccessType(pub i32);
impl UserDataTaskStoreAccessType {
    pub const AppTasksReadWrite: UserDataTaskStoreAccessType = UserDataTaskStoreAccessType(0i32);
    pub const AllTasksLimitedReadWrite: UserDataTaskStoreAccessType = UserDataTaskStoreAccessType(1i32);
}
impl ::std::convert::From<i32> for UserDataTaskStoreAccessType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UserDataTaskStoreAccessType {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for UserDataTaskStoreAccessType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskStoreAccessType;i4)");
}
impl ::windows::runtime::DefaultType for UserDataTaskStoreAccessType {
    type DefaultType = Self;
}
#[doc = "*Required features: `ApplicationModel_UserDataTasks`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct UserDataTaskWeekOfMonth(pub i32);
impl UserDataTaskWeekOfMonth {
    pub const First: UserDataTaskWeekOfMonth = UserDataTaskWeekOfMonth(0i32);
    pub const Second: UserDataTaskWeekOfMonth = UserDataTaskWeekOfMonth(1i32);
    pub const Third: UserDataTaskWeekOfMonth = UserDataTaskWeekOfMonth(2i32);
    pub const Fourth: UserDataTaskWeekOfMonth = UserDataTaskWeekOfMonth(3i32);
    pub const Last: UserDataTaskWeekOfMonth = UserDataTaskWeekOfMonth(4i32);
}
impl ::std::convert::From<i32> for UserDataTaskWeekOfMonth {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for UserDataTaskWeekOfMonth {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for UserDataTaskWeekOfMonth {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskWeekOfMonth;i4)");
}
impl ::windows::runtime::DefaultType for UserDataTaskWeekOfMonth {
    type DefaultType = Self;
}

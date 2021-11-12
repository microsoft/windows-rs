#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "ApplicationModel_UserDataTasks_DataProvider")]
pub mod DataProvider;
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserDataTask(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUserDataTask {
    type Vtable = IUserDataTask_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7c6585d1_e0d4_4f99_aee2_bc2d5ddadf4c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTask_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut UserDataTaskDetailsKind) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: UserDataTaskDetailsKind) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut UserDataTaskKind) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut UserDataTaskPriority) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: UserDataTaskPriority) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut UserDataTaskSensitivity) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: UserDataTaskSensitivity) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserDataTaskBatch(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUserDataTaskBatch {
    type Vtable = IUserDataTaskBatch_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x382da5fe_20b5_431c_8f42_a5d292ec930c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskBatch_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserDataTaskList(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUserDataTaskList {
    type Vtable = IUserDataTaskList_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49412e39_7c1d_4df1_bed3_314b7cbf5e4e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskList_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut UserDataTaskListOtherAppReadAccess) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: UserDataTaskListOtherAppReadAccess) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut UserDataTaskListOtherAppWriteAccess) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: UserDataTaskListOtherAppWriteAccess) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, userdatatask: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, userdatatask: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, userdatataskid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserDataTaskListLimitedWriteOperations(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUserDataTaskListLimitedWriteOperations {
    type Vtable = IUserDataTaskListLimitedWriteOperations_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7aa267f2_6078_4183_919e_4f29f19cfae9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskListLimitedWriteOperations_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, userdatataskid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, userdatatask: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, userdatataskid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, userdatataskid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserDataTaskListSyncManager(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUserDataTaskListSyncManager {
    type Vtable = IUserDataTaskListSyncManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8e591a95_1dcf_469f_93ec_ba48bb553c6b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskListSyncManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut UserDataTaskListSyncStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: UserDataTaskListSyncStatus) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserDataTaskManager(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUserDataTaskManager {
    type Vtable = IUserDataTaskManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8451c914_e60b_48a9_9211_7fb8a56cb84c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, accesstype: UserDataTaskStoreAccessType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserDataTaskManagerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUserDataTaskManagerStatics {
    type Vtable = IUserDataTaskManagerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb35539f8_c502_47fc_a81e_100883719d55);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "System")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserDataTaskQueryOptions(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUserDataTaskQueryOptions {
    type Vtable = IUserDataTaskQueryOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x959f27ed_909a_4d30_8c1b_331d8fe667e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskQueryOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut UserDataTaskQuerySortProperty) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: UserDataTaskQuerySortProperty) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut UserDataTaskQueryKind) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: UserDataTaskQueryKind) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserDataTaskReader(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUserDataTaskReader {
    type Vtable = IUserDataTaskReader_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03e688b1_4ccf_4500_883b_e76290cfed63);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskReader_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserDataTaskRecurrenceProperties(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUserDataTaskRecurrenceProperties {
    type Vtable = IUserDataTaskRecurrenceProperties_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73df80b0_27c6_40ce_b149_9cd41485a69e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskRecurrenceProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut UserDataTaskRecurrenceUnit) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: UserDataTaskRecurrenceUnit) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserDataTaskRegenerationProperties(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUserDataTaskRegenerationProperties {
    type Vtable = IUserDataTaskRegenerationProperties_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92ab0007_090e_4704_bb5c_84fc0b0d9c31);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskRegenerationProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut UserDataTaskRegenerationUnit) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: UserDataTaskRegenerationUnit) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUserDataTaskStore(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUserDataTaskStore {
    type Vtable = IUserDataTaskStore_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf06a9cb0_f1db_45ba_8a62_086004c0213d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskStore_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, userdataaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, tasklistid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct UserDataTask(pub ::windows::core::IInspectable);
impl UserDataTask {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UserDataTask, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ListId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn RemoteId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetRemoteId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn CompletedDate(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetCompletedDate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Details(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetDetails<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn DetailsKind(&self) -> ::windows::core::Result<UserDataTaskDetailsKind> {
        let this = self;
        unsafe {
            let mut result__: UserDataTaskDetailsKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataTaskDetailsKind>(result__)
        }
    }
    pub fn SetDetailsKind(&self, value: UserDataTaskDetailsKind) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn DueDate(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetDueDate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Kind(&self) -> ::windows::core::Result<UserDataTaskKind> {
        let this = self;
        unsafe {
            let mut result__: UserDataTaskKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataTaskKind>(result__)
        }
    }
    pub fn Priority(&self) -> ::windows::core::Result<UserDataTaskPriority> {
        let this = self;
        unsafe {
            let mut result__: UserDataTaskPriority = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataTaskPriority>(result__)
        }
    }
    pub fn SetPriority(&self, value: UserDataTaskPriority) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn RecurrenceProperties(&self) -> ::windows::core::Result<UserDataTaskRecurrenceProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataTaskRecurrenceProperties>(result__)
        }
    }
    pub fn SetRecurrenceProperties<'a, Param0: ::windows::core::IntoParam<'a, UserDataTaskRecurrenceProperties>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn RegenerationProperties(&self) -> ::windows::core::Result<UserDataTaskRegenerationProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataTaskRegenerationProperties>(result__)
        }
    }
    pub fn SetRegenerationProperties<'a, Param0: ::windows::core::IntoParam<'a, UserDataTaskRegenerationProperties>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Reminder(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetReminder<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Sensitivity(&self) -> ::windows::core::Result<UserDataTaskSensitivity> {
        let this = self;
        unsafe {
            let mut result__: UserDataTaskSensitivity = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataTaskSensitivity>(result__)
        }
    }
    pub fn SetSensitivity(&self, value: UserDataTaskSensitivity) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetSubject<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn StartDate(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetStartDate<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataTask {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.UserDataTask;{7c6585d1-e0d4-4f99-aee2-bc2d5ddadf4c})");
}
unsafe impl ::windows::core::Interface for UserDataTask {
    type Vtable = IUserDataTask_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7c6585d1_e0d4_4f99_aee2_bc2d5ddadf4c);
}
impl ::windows::core::RuntimeName for UserDataTask {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTask";
}
impl ::core::convert::From<UserDataTask> for ::windows::core::IUnknown {
    fn from(value: UserDataTask) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&UserDataTask> for ::windows::core::IUnknown {
    fn from(value: &UserDataTask) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserDataTask {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserDataTask {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<UserDataTask> for ::windows::core::IInspectable {
    fn from(value: UserDataTask) -> Self {
        value.0
    }
}
impl ::core::convert::From<&UserDataTask> for ::windows::core::IInspectable {
    fn from(value: &UserDataTask) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserDataTask {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserDataTask {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for UserDataTask {}
unsafe impl ::core::marker::Sync for UserDataTask {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct UserDataTaskBatch(pub ::windows::core::IInspectable);
impl UserDataTaskBatch {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Tasks(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UserDataTask>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<UserDataTask>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskBatch {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.UserDataTaskBatch;{382da5fe-20b5-431c-8f42-a5d292ec930c})");
}
unsafe impl ::windows::core::Interface for UserDataTaskBatch {
    type Vtable = IUserDataTaskBatch_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x382da5fe_20b5_431c_8f42_a5d292ec930c);
}
impl ::windows::core::RuntimeName for UserDataTaskBatch {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTaskBatch";
}
impl ::core::convert::From<UserDataTaskBatch> for ::windows::core::IUnknown {
    fn from(value: UserDataTaskBatch) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&UserDataTaskBatch> for ::windows::core::IUnknown {
    fn from(value: &UserDataTaskBatch) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserDataTaskBatch {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserDataTaskBatch {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<UserDataTaskBatch> for ::windows::core::IInspectable {
    fn from(value: UserDataTaskBatch) -> Self {
        value.0
    }
}
impl ::core::convert::From<&UserDataTaskBatch> for ::windows::core::IInspectable {
    fn from(value: &UserDataTaskBatch) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserDataTaskBatch {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserDataTaskBatch {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for UserDataTaskBatch {}
unsafe impl ::core::marker::Sync for UserDataTaskBatch {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
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
impl ::core::convert::From<u32> for UserDataTaskDaysOfWeek {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for UserDataTaskDaysOfWeek {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskDaysOfWeek {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskDaysOfWeek;u4)");
}
impl ::windows::core::DefaultType for UserDataTaskDaysOfWeek {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for UserDataTaskDaysOfWeek {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for UserDataTaskDaysOfWeek {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for UserDataTaskDaysOfWeek {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for UserDataTaskDaysOfWeek {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for UserDataTaskDaysOfWeek {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct UserDataTaskDetailsKind(pub i32);
impl UserDataTaskDetailsKind {
    pub const PlainText: UserDataTaskDetailsKind = UserDataTaskDetailsKind(0i32);
    pub const Html: UserDataTaskDetailsKind = UserDataTaskDetailsKind(1i32);
}
impl ::core::convert::From<i32> for UserDataTaskDetailsKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for UserDataTaskDetailsKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskDetailsKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskDetailsKind;i4)");
}
impl ::windows::core::DefaultType for UserDataTaskDetailsKind {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct UserDataTaskKind(pub i32);
impl UserDataTaskKind {
    pub const Single: UserDataTaskKind = UserDataTaskKind(0i32);
    pub const Recurring: UserDataTaskKind = UserDataTaskKind(1i32);
    pub const Regenerating: UserDataTaskKind = UserDataTaskKind(2i32);
}
impl ::core::convert::From<i32> for UserDataTaskKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for UserDataTaskKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskKind;i4)");
}
impl ::windows::core::DefaultType for UserDataTaskKind {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct UserDataTaskList(pub ::windows::core::IInspectable);
impl UserDataTaskList {
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn UserDataAccountId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn SetDisplayName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn SourceDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn OtherAppReadAccess(&self) -> ::windows::core::Result<UserDataTaskListOtherAppReadAccess> {
        let this = self;
        unsafe {
            let mut result__: UserDataTaskListOtherAppReadAccess = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataTaskListOtherAppReadAccess>(result__)
        }
    }
    pub fn SetOtherAppReadAccess(&self, value: UserDataTaskListOtherAppReadAccess) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn OtherAppWriteAccess(&self) -> ::windows::core::Result<UserDataTaskListOtherAppWriteAccess> {
        let this = self;
        unsafe {
            let mut result__: UserDataTaskListOtherAppWriteAccess = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataTaskListOtherAppWriteAccess>(result__)
        }
    }
    pub fn SetOtherAppWriteAccess(&self, value: UserDataTaskListOtherAppWriteAccess) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn LimitedWriteOperations(&self) -> ::windows::core::Result<UserDataTaskListLimitedWriteOperations> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataTaskListLimitedWriteOperations>(result__)
        }
    }
    pub fn SyncManager(&self) -> ::windows::core::Result<UserDataTaskListSyncManager> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataTaskListSyncManager>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RegisterSyncManagerAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn GetTaskReader(&self) -> ::windows::core::Result<UserDataTaskReader> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataTaskReader>(result__)
        }
    }
    pub fn GetTaskReaderWithOptions<'a, Param0: ::windows::core::IntoParam<'a, UserDataTaskQueryOptions>>(&self, options: Param0) -> ::windows::core::Result<UserDataTaskReader> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), options.into_param().abi(), &mut result__).from_abi::<UserDataTaskReader>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetTaskAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, userdatatask: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UserDataTask>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), userdatatask.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<UserDataTask>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SaveTaskAsync<'a, Param0: ::windows::core::IntoParam<'a, UserDataTask>>(&self, userdatatask: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), userdatatask.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn DeleteTaskAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, userdatataskid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), userdatataskid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SaveAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskList {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.UserDataTaskList;{49412e39-7c1d-4df1-bed3-314b7cbf5e4e})");
}
unsafe impl ::windows::core::Interface for UserDataTaskList {
    type Vtable = IUserDataTaskList_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49412e39_7c1d_4df1_bed3_314b7cbf5e4e);
}
impl ::windows::core::RuntimeName for UserDataTaskList {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTaskList";
}
impl ::core::convert::From<UserDataTaskList> for ::windows::core::IUnknown {
    fn from(value: UserDataTaskList) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&UserDataTaskList> for ::windows::core::IUnknown {
    fn from(value: &UserDataTaskList) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserDataTaskList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserDataTaskList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<UserDataTaskList> for ::windows::core::IInspectable {
    fn from(value: UserDataTaskList) -> Self {
        value.0
    }
}
impl ::core::convert::From<&UserDataTaskList> for ::windows::core::IInspectable {
    fn from(value: &UserDataTaskList) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserDataTaskList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserDataTaskList {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for UserDataTaskList {}
unsafe impl ::core::marker::Sync for UserDataTaskList {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct UserDataTaskListLimitedWriteOperations(pub ::windows::core::IInspectable);
impl UserDataTaskListLimitedWriteOperations {
    #[cfg(feature = "Foundation")]
    pub fn TryCompleteTaskAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, userdatataskid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), userdatataskid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn TryCreateOrUpdateTaskAsync<'a, Param0: ::windows::core::IntoParam<'a, UserDataTask>>(&self, userdatatask: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), userdatatask.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn TryDeleteTaskAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, userdatataskid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), userdatataskid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn TrySkipOccurrenceAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, userdatataskid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), userdatataskid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskListLimitedWriteOperations {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.UserDataTaskListLimitedWriteOperations;{7aa267f2-6078-4183-919e-4f29f19cfae9})");
}
unsafe impl ::windows::core::Interface for UserDataTaskListLimitedWriteOperations {
    type Vtable = IUserDataTaskListLimitedWriteOperations_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7aa267f2_6078_4183_919e_4f29f19cfae9);
}
impl ::windows::core::RuntimeName for UserDataTaskListLimitedWriteOperations {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTaskListLimitedWriteOperations";
}
impl ::core::convert::From<UserDataTaskListLimitedWriteOperations> for ::windows::core::IUnknown {
    fn from(value: UserDataTaskListLimitedWriteOperations) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&UserDataTaskListLimitedWriteOperations> for ::windows::core::IUnknown {
    fn from(value: &UserDataTaskListLimitedWriteOperations) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserDataTaskListLimitedWriteOperations {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserDataTaskListLimitedWriteOperations {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<UserDataTaskListLimitedWriteOperations> for ::windows::core::IInspectable {
    fn from(value: UserDataTaskListLimitedWriteOperations) -> Self {
        value.0
    }
}
impl ::core::convert::From<&UserDataTaskListLimitedWriteOperations> for ::windows::core::IInspectable {
    fn from(value: &UserDataTaskListLimitedWriteOperations) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserDataTaskListLimitedWriteOperations {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserDataTaskListLimitedWriteOperations {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for UserDataTaskListLimitedWriteOperations {}
unsafe impl ::core::marker::Sync for UserDataTaskListLimitedWriteOperations {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct UserDataTaskListOtherAppReadAccess(pub i32);
impl UserDataTaskListOtherAppReadAccess {
    pub const Full: UserDataTaskListOtherAppReadAccess = UserDataTaskListOtherAppReadAccess(0i32);
    pub const SystemOnly: UserDataTaskListOtherAppReadAccess = UserDataTaskListOtherAppReadAccess(1i32);
    pub const None: UserDataTaskListOtherAppReadAccess = UserDataTaskListOtherAppReadAccess(2i32);
}
impl ::core::convert::From<i32> for UserDataTaskListOtherAppReadAccess {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for UserDataTaskListOtherAppReadAccess {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskListOtherAppReadAccess {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskListOtherAppReadAccess;i4)");
}
impl ::windows::core::DefaultType for UserDataTaskListOtherAppReadAccess {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct UserDataTaskListOtherAppWriteAccess(pub i32);
impl UserDataTaskListOtherAppWriteAccess {
    pub const Limited: UserDataTaskListOtherAppWriteAccess = UserDataTaskListOtherAppWriteAccess(0i32);
    pub const None: UserDataTaskListOtherAppWriteAccess = UserDataTaskListOtherAppWriteAccess(1i32);
}
impl ::core::convert::From<i32> for UserDataTaskListOtherAppWriteAccess {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for UserDataTaskListOtherAppWriteAccess {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskListOtherAppWriteAccess {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskListOtherAppWriteAccess;i4)");
}
impl ::windows::core::DefaultType for UserDataTaskListOtherAppWriteAccess {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct UserDataTaskListSyncManager(pub ::windows::core::IInspectable);
impl UserDataTaskListSyncManager {
    #[cfg(feature = "Foundation")]
    pub fn LastAttemptedSyncTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetLastAttemptedSyncTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn LastSuccessfulSyncTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetLastSuccessfulSyncTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Status(&self) -> ::windows::core::Result<UserDataTaskListSyncStatus> {
        let this = self;
        unsafe {
            let mut result__: UserDataTaskListSyncStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataTaskListSyncStatus>(result__)
        }
    }
    pub fn SetStatus(&self, value: UserDataTaskListSyncStatus) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn SyncAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SyncStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<UserDataTaskListSyncManager, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveSyncStatusChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskListSyncManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.UserDataTaskListSyncManager;{8e591a95-1dcf-469f-93ec-ba48bb553c6b})");
}
unsafe impl ::windows::core::Interface for UserDataTaskListSyncManager {
    type Vtable = IUserDataTaskListSyncManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8e591a95_1dcf_469f_93ec_ba48bb553c6b);
}
impl ::windows::core::RuntimeName for UserDataTaskListSyncManager {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTaskListSyncManager";
}
impl ::core::convert::From<UserDataTaskListSyncManager> for ::windows::core::IUnknown {
    fn from(value: UserDataTaskListSyncManager) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&UserDataTaskListSyncManager> for ::windows::core::IUnknown {
    fn from(value: &UserDataTaskListSyncManager) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserDataTaskListSyncManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserDataTaskListSyncManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<UserDataTaskListSyncManager> for ::windows::core::IInspectable {
    fn from(value: UserDataTaskListSyncManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&UserDataTaskListSyncManager> for ::windows::core::IInspectable {
    fn from(value: &UserDataTaskListSyncManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserDataTaskListSyncManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserDataTaskListSyncManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for UserDataTaskListSyncManager {}
unsafe impl ::core::marker::Sync for UserDataTaskListSyncManager {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
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
impl ::core::convert::From<i32> for UserDataTaskListSyncStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for UserDataTaskListSyncStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskListSyncStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskListSyncStatus;i4)");
}
impl ::windows::core::DefaultType for UserDataTaskListSyncStatus {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct UserDataTaskManager(pub ::windows::core::IInspectable);
impl UserDataTaskManager {
    #[cfg(feature = "Foundation")]
    pub fn RequestStoreAsync(&self, accesstype: UserDataTaskStoreAccessType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UserDataTaskStore>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), accesstype, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<UserDataTaskStore>>(result__)
        }
    }
    #[cfg(feature = "System")]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::System::User>(result__)
        }
    }
    pub fn GetDefault() -> ::windows::core::Result<UserDataTaskManager> {
        Self::IUserDataTaskManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataTaskManager>(result__)
        })
    }
    #[cfg(feature = "System")]
    pub fn GetForUser<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::User>>(user: Param0) -> ::windows::core::Result<UserDataTaskManager> {
        Self::IUserDataTaskManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), user.into_param().abi(), &mut result__).from_abi::<UserDataTaskManager>(result__)
        })
    }
    pub fn IUserDataTaskManagerStatics<R, F: FnOnce(&IUserDataTaskManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UserDataTaskManager, IUserDataTaskManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.UserDataTaskManager;{8451c914-e60b-48a9-9211-7fb8a56cb84c})");
}
unsafe impl ::windows::core::Interface for UserDataTaskManager {
    type Vtable = IUserDataTaskManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8451c914_e60b_48a9_9211_7fb8a56cb84c);
}
impl ::windows::core::RuntimeName for UserDataTaskManager {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTaskManager";
}
impl ::core::convert::From<UserDataTaskManager> for ::windows::core::IUnknown {
    fn from(value: UserDataTaskManager) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&UserDataTaskManager> for ::windows::core::IUnknown {
    fn from(value: &UserDataTaskManager) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserDataTaskManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserDataTaskManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<UserDataTaskManager> for ::windows::core::IInspectable {
    fn from(value: UserDataTaskManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&UserDataTaskManager> for ::windows::core::IInspectable {
    fn from(value: &UserDataTaskManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserDataTaskManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserDataTaskManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for UserDataTaskManager {}
unsafe impl ::core::marker::Sync for UserDataTaskManager {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct UserDataTaskPriority(pub i32);
impl UserDataTaskPriority {
    pub const Normal: UserDataTaskPriority = UserDataTaskPriority(0i32);
    pub const Low: UserDataTaskPriority = UserDataTaskPriority(-1i32);
    pub const High: UserDataTaskPriority = UserDataTaskPriority(1i32);
}
impl ::core::convert::From<i32> for UserDataTaskPriority {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for UserDataTaskPriority {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskPriority {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskPriority;i4)");
}
impl ::windows::core::DefaultType for UserDataTaskPriority {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct UserDataTaskQueryKind(pub i32);
impl UserDataTaskQueryKind {
    pub const All: UserDataTaskQueryKind = UserDataTaskQueryKind(0i32);
    pub const Incomplete: UserDataTaskQueryKind = UserDataTaskQueryKind(1i32);
    pub const Complete: UserDataTaskQueryKind = UserDataTaskQueryKind(2i32);
}
impl ::core::convert::From<i32> for UserDataTaskQueryKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for UserDataTaskQueryKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskQueryKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskQueryKind;i4)");
}
impl ::windows::core::DefaultType for UserDataTaskQueryKind {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct UserDataTaskQueryOptions(pub ::windows::core::IInspectable);
impl UserDataTaskQueryOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UserDataTaskQueryOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn SortProperty(&self) -> ::windows::core::Result<UserDataTaskQuerySortProperty> {
        let this = self;
        unsafe {
            let mut result__: UserDataTaskQuerySortProperty = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataTaskQuerySortProperty>(result__)
        }
    }
    pub fn SetSortProperty(&self, value: UserDataTaskQuerySortProperty) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Kind(&self) -> ::windows::core::Result<UserDataTaskQueryKind> {
        let this = self;
        unsafe {
            let mut result__: UserDataTaskQueryKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataTaskQueryKind>(result__)
        }
    }
    pub fn SetKind(&self, value: UserDataTaskQueryKind) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskQueryOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.UserDataTaskQueryOptions;{959f27ed-909a-4d30-8c1b-331d8fe667e2})");
}
unsafe impl ::windows::core::Interface for UserDataTaskQueryOptions {
    type Vtable = IUserDataTaskQueryOptions_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x959f27ed_909a_4d30_8c1b_331d8fe667e2);
}
impl ::windows::core::RuntimeName for UserDataTaskQueryOptions {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTaskQueryOptions";
}
impl ::core::convert::From<UserDataTaskQueryOptions> for ::windows::core::IUnknown {
    fn from(value: UserDataTaskQueryOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&UserDataTaskQueryOptions> for ::windows::core::IUnknown {
    fn from(value: &UserDataTaskQueryOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserDataTaskQueryOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserDataTaskQueryOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<UserDataTaskQueryOptions> for ::windows::core::IInspectable {
    fn from(value: UserDataTaskQueryOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&UserDataTaskQueryOptions> for ::windows::core::IInspectable {
    fn from(value: &UserDataTaskQueryOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserDataTaskQueryOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserDataTaskQueryOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for UserDataTaskQueryOptions {}
unsafe impl ::core::marker::Sync for UserDataTaskQueryOptions {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct UserDataTaskQuerySortProperty(pub i32);
impl UserDataTaskQuerySortProperty {
    pub const DueDate: UserDataTaskQuerySortProperty = UserDataTaskQuerySortProperty(0i32);
}
impl ::core::convert::From<i32> for UserDataTaskQuerySortProperty {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for UserDataTaskQuerySortProperty {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskQuerySortProperty {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskQuerySortProperty;i4)");
}
impl ::windows::core::DefaultType for UserDataTaskQuerySortProperty {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct UserDataTaskReader(pub ::windows::core::IInspectable);
impl UserDataTaskReader {
    #[cfg(feature = "Foundation")]
    pub fn ReadBatchAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UserDataTaskBatch>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<UserDataTaskBatch>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskReader {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.UserDataTaskReader;{03e688b1-4ccf-4500-883b-e76290cfed63})");
}
unsafe impl ::windows::core::Interface for UserDataTaskReader {
    type Vtable = IUserDataTaskReader_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03e688b1_4ccf_4500_883b_e76290cfed63);
}
impl ::windows::core::RuntimeName for UserDataTaskReader {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTaskReader";
}
impl ::core::convert::From<UserDataTaskReader> for ::windows::core::IUnknown {
    fn from(value: UserDataTaskReader) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&UserDataTaskReader> for ::windows::core::IUnknown {
    fn from(value: &UserDataTaskReader) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserDataTaskReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserDataTaskReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<UserDataTaskReader> for ::windows::core::IInspectable {
    fn from(value: UserDataTaskReader) -> Self {
        value.0
    }
}
impl ::core::convert::From<&UserDataTaskReader> for ::windows::core::IInspectable {
    fn from(value: &UserDataTaskReader) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserDataTaskReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserDataTaskReader {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for UserDataTaskReader {}
unsafe impl ::core::marker::Sync for UserDataTaskReader {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct UserDataTaskRecurrenceProperties(pub ::windows::core::IInspectable);
impl UserDataTaskRecurrenceProperties {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UserDataTaskRecurrenceProperties, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Unit(&self) -> ::windows::core::Result<UserDataTaskRecurrenceUnit> {
        let this = self;
        unsafe {
            let mut result__: UserDataTaskRecurrenceUnit = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataTaskRecurrenceUnit>(result__)
        }
    }
    pub fn SetUnit(&self, value: UserDataTaskRecurrenceUnit) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Occurrences(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetOccurrences<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<i32>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Until(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetUntil<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Interval(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn SetInterval(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn DaysOfWeek(&self) -> ::windows::core::Result<super::super::Foundation::IReference<UserDataTaskDaysOfWeek>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<UserDataTaskDaysOfWeek>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetDaysOfWeek<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<UserDataTaskDaysOfWeek>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn WeekOfMonth(&self) -> ::windows::core::Result<super::super::Foundation::IReference<UserDataTaskWeekOfMonth>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<UserDataTaskWeekOfMonth>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetWeekOfMonth<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<UserDataTaskWeekOfMonth>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Month(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetMonth<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<i32>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Day(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetDay<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<i32>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskRecurrenceProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.UserDataTaskRecurrenceProperties;{73df80b0-27c6-40ce-b149-9cd41485a69e})");
}
unsafe impl ::windows::core::Interface for UserDataTaskRecurrenceProperties {
    type Vtable = IUserDataTaskRecurrenceProperties_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73df80b0_27c6_40ce_b149_9cd41485a69e);
}
impl ::windows::core::RuntimeName for UserDataTaskRecurrenceProperties {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTaskRecurrenceProperties";
}
impl ::core::convert::From<UserDataTaskRecurrenceProperties> for ::windows::core::IUnknown {
    fn from(value: UserDataTaskRecurrenceProperties) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&UserDataTaskRecurrenceProperties> for ::windows::core::IUnknown {
    fn from(value: &UserDataTaskRecurrenceProperties) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserDataTaskRecurrenceProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserDataTaskRecurrenceProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<UserDataTaskRecurrenceProperties> for ::windows::core::IInspectable {
    fn from(value: UserDataTaskRecurrenceProperties) -> Self {
        value.0
    }
}
impl ::core::convert::From<&UserDataTaskRecurrenceProperties> for ::windows::core::IInspectable {
    fn from(value: &UserDataTaskRecurrenceProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserDataTaskRecurrenceProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserDataTaskRecurrenceProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for UserDataTaskRecurrenceProperties {}
unsafe impl ::core::marker::Sync for UserDataTaskRecurrenceProperties {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
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
impl ::core::convert::From<i32> for UserDataTaskRecurrenceUnit {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for UserDataTaskRecurrenceUnit {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskRecurrenceUnit {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskRecurrenceUnit;i4)");
}
impl ::windows::core::DefaultType for UserDataTaskRecurrenceUnit {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct UserDataTaskRegenerationProperties(pub ::windows::core::IInspectable);
impl UserDataTaskRegenerationProperties {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UserDataTaskRegenerationProperties, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Unit(&self) -> ::windows::core::Result<UserDataTaskRegenerationUnit> {
        let this = self;
        unsafe {
            let mut result__: UserDataTaskRegenerationUnit = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UserDataTaskRegenerationUnit>(result__)
        }
    }
    pub fn SetUnit(&self, value: UserDataTaskRegenerationUnit) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Occurrences(&self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<i32>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetOccurrences<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<i32>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Until(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetUntil<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IReference<super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Interval(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn SetInterval(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskRegenerationProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.UserDataTaskRegenerationProperties;{92ab0007-090e-4704-bb5c-84fc0b0d9c31})");
}
unsafe impl ::windows::core::Interface for UserDataTaskRegenerationProperties {
    type Vtable = IUserDataTaskRegenerationProperties_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92ab0007_090e_4704_bb5c_84fc0b0d9c31);
}
impl ::windows::core::RuntimeName for UserDataTaskRegenerationProperties {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTaskRegenerationProperties";
}
impl ::core::convert::From<UserDataTaskRegenerationProperties> for ::windows::core::IUnknown {
    fn from(value: UserDataTaskRegenerationProperties) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&UserDataTaskRegenerationProperties> for ::windows::core::IUnknown {
    fn from(value: &UserDataTaskRegenerationProperties) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserDataTaskRegenerationProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserDataTaskRegenerationProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<UserDataTaskRegenerationProperties> for ::windows::core::IInspectable {
    fn from(value: UserDataTaskRegenerationProperties) -> Self {
        value.0
    }
}
impl ::core::convert::From<&UserDataTaskRegenerationProperties> for ::windows::core::IInspectable {
    fn from(value: &UserDataTaskRegenerationProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserDataTaskRegenerationProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserDataTaskRegenerationProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for UserDataTaskRegenerationProperties {}
unsafe impl ::core::marker::Sync for UserDataTaskRegenerationProperties {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct UserDataTaskRegenerationUnit(pub i32);
impl UserDataTaskRegenerationUnit {
    pub const Daily: UserDataTaskRegenerationUnit = UserDataTaskRegenerationUnit(0i32);
    pub const Weekly: UserDataTaskRegenerationUnit = UserDataTaskRegenerationUnit(1i32);
    pub const Monthly: UserDataTaskRegenerationUnit = UserDataTaskRegenerationUnit(2i32);
    pub const Yearly: UserDataTaskRegenerationUnit = UserDataTaskRegenerationUnit(4i32);
}
impl ::core::convert::From<i32> for UserDataTaskRegenerationUnit {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for UserDataTaskRegenerationUnit {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskRegenerationUnit {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskRegenerationUnit;i4)");
}
impl ::windows::core::DefaultType for UserDataTaskRegenerationUnit {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct UserDataTaskSensitivity(pub i32);
impl UserDataTaskSensitivity {
    pub const Public: UserDataTaskSensitivity = UserDataTaskSensitivity(0i32);
    pub const Private: UserDataTaskSensitivity = UserDataTaskSensitivity(1i32);
}
impl ::core::convert::From<i32> for UserDataTaskSensitivity {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for UserDataTaskSensitivity {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskSensitivity {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskSensitivity;i4)");
}
impl ::windows::core::DefaultType for UserDataTaskSensitivity {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct UserDataTaskStore(pub ::windows::core::IInspectable);
impl UserDataTaskStore {
    #[cfg(feature = "Foundation")]
    pub fn CreateListAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UserDataTaskList>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<UserDataTaskList>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn CreateListInAccountAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0, userdataaccountid: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UserDataTaskList>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), name.into_param().abi(), userdataaccountid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<UserDataTaskList>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn FindListsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<UserDataTaskList>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<UserDataTaskList>>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetListAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, tasklistid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UserDataTaskList>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), tasklistid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<UserDataTaskList>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskStore {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.UserDataTaskStore;{f06a9cb0-f1db-45ba-8a62-086004c0213d})");
}
unsafe impl ::windows::core::Interface for UserDataTaskStore {
    type Vtable = IUserDataTaskStore_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf06a9cb0_f1db_45ba_8a62_086004c0213d);
}
impl ::windows::core::RuntimeName for UserDataTaskStore {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.UserDataTaskStore";
}
impl ::core::convert::From<UserDataTaskStore> for ::windows::core::IUnknown {
    fn from(value: UserDataTaskStore) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&UserDataTaskStore> for ::windows::core::IUnknown {
    fn from(value: &UserDataTaskStore) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UserDataTaskStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UserDataTaskStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<UserDataTaskStore> for ::windows::core::IInspectable {
    fn from(value: UserDataTaskStore) -> Self {
        value.0
    }
}
impl ::core::convert::From<&UserDataTaskStore> for ::windows::core::IInspectable {
    fn from(value: &UserDataTaskStore) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UserDataTaskStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UserDataTaskStore {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for UserDataTaskStore {}
unsafe impl ::core::marker::Sync for UserDataTaskStore {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct UserDataTaskStoreAccessType(pub i32);
impl UserDataTaskStoreAccessType {
    pub const AppTasksReadWrite: UserDataTaskStoreAccessType = UserDataTaskStoreAccessType(0i32);
    pub const AllTasksLimitedReadWrite: UserDataTaskStoreAccessType = UserDataTaskStoreAccessType(1i32);
}
impl ::core::convert::From<i32> for UserDataTaskStoreAccessType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for UserDataTaskStoreAccessType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskStoreAccessType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskStoreAccessType;i4)");
}
impl ::windows::core::DefaultType for UserDataTaskStoreAccessType {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct UserDataTaskWeekOfMonth(pub i32);
impl UserDataTaskWeekOfMonth {
    pub const First: UserDataTaskWeekOfMonth = UserDataTaskWeekOfMonth(0i32);
    pub const Second: UserDataTaskWeekOfMonth = UserDataTaskWeekOfMonth(1i32);
    pub const Third: UserDataTaskWeekOfMonth = UserDataTaskWeekOfMonth(2i32);
    pub const Fourth: UserDataTaskWeekOfMonth = UserDataTaskWeekOfMonth(3i32);
    pub const Last: UserDataTaskWeekOfMonth = UserDataTaskWeekOfMonth(4i32);
}
impl ::core::convert::From<i32> for UserDataTaskWeekOfMonth {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for UserDataTaskWeekOfMonth {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskWeekOfMonth {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.UserDataTasks.UserDataTaskWeekOfMonth;i4)");
}
impl ::windows::core::DefaultType for UserDataTaskWeekOfMonth {
    type DefaultType = Self;
}

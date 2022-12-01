#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataTaskDataProviderConnection(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUserDataTaskDataProviderConnection {
    type Vtable = IUserDataTaskDataProviderConnection_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserDataTaskDataProviderConnection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9ff39d1d_a447_428b_afe9_e5402bdeb041);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskDataProviderConnection_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CreateOrUpdateTaskRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateOrUpdateTaskRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCreateOrUpdateTaskRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCreateOrUpdateTaskRequested: usize,
    #[cfg(feature = "Foundation")]
    pub SyncRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SyncRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSyncRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSyncRequested: usize,
    #[cfg(feature = "Foundation")]
    pub SkipOccurrenceRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SkipOccurrenceRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSkipOccurrenceRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSkipOccurrenceRequested: usize,
    #[cfg(feature = "Foundation")]
    pub CompleteTaskRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CompleteTaskRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCompleteTaskRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCompleteTaskRequested: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteTaskRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteTaskRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveDeleteTaskRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveDeleteTaskRequested: usize,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataTaskDataProviderTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUserDataTaskDataProviderTriggerDetails {
    type Vtable = IUserDataTaskDataProviderTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserDataTaskDataProviderTriggerDetails {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae273202_b1c9_453e_afc5_b30af3bd217d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskDataProviderTriggerDetails_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Connection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataTaskListCompleteTaskRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUserDataTaskListCompleteTaskRequest {
    type Vtable = IUserDataTaskListCompleteTaskRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserDataTaskListCompleteTaskRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf65e14a3_1a42_49da_8552_2873e52c55eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskListCompleteTaskRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TaskListId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TaskId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, completedtaskid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataTaskListCompleteTaskRequestEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUserDataTaskListCompleteTaskRequestEventArgs {
    type Vtable = IUserDataTaskListCompleteTaskRequestEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserDataTaskListCompleteTaskRequestEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd77c393d_4cf2_48ad_87fd_963f0eaa7a95);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskListCompleteTaskRequestEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataTaskListCreateOrUpdateTaskRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUserDataTaskListCreateOrUpdateTaskRequest {
    type Vtable = IUserDataTaskListCreateOrUpdateTaskRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserDataTaskListCreateOrUpdateTaskRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2133772c_55c2_4300_8279_04326e07cce4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskListCreateOrUpdateTaskRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TaskListId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Task: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, createdorupdateduserdatatask: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataTaskListCreateOrUpdateTaskRequestEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUserDataTaskListCreateOrUpdateTaskRequestEventArgs {
    type Vtable = IUserDataTaskListCreateOrUpdateTaskRequestEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserDataTaskListCreateOrUpdateTaskRequestEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x12c55a52_e378_419b_ae38_a5e9e604476e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskListCreateOrUpdateTaskRequestEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataTaskListDeleteTaskRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUserDataTaskListDeleteTaskRequest {
    type Vtable = IUserDataTaskListDeleteTaskRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserDataTaskListDeleteTaskRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b863c68_7657_4f3d_b074_d47ec8df07e7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskListDeleteTaskRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TaskListId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TaskId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataTaskListDeleteTaskRequestEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUserDataTaskListDeleteTaskRequestEventArgs {
    type Vtable = IUserDataTaskListDeleteTaskRequestEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserDataTaskListDeleteTaskRequestEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6063dad9_f562_4145_8efe_d50078c92b7f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskListDeleteTaskRequestEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataTaskListSkipOccurrenceRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUserDataTaskListSkipOccurrenceRequest {
    type Vtable = IUserDataTaskListSkipOccurrenceRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserDataTaskListSkipOccurrenceRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab87e34d_1cd3_431c_9f58_089aa4338d85);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskListSkipOccurrenceRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TaskListId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TaskId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataTaskListSkipOccurrenceRequestEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUserDataTaskListSkipOccurrenceRequestEventArgs {
    type Vtable = IUserDataTaskListSkipOccurrenceRequestEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserDataTaskListSkipOccurrenceRequestEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a3b924a_cc2f_4e7b_aacd_a5b9d29cfa4e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskListSkipOccurrenceRequestEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataTaskListSyncManagerSyncRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUserDataTaskListSyncManagerSyncRequest {
    type Vtable = IUserDataTaskListSyncManagerSyncRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserDataTaskListSyncManagerSyncRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x40a73807_7590_4149_ae19_b211431a9f48);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskListSyncManagerSyncRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TaskListId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ReportCompletedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportCompletedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ReportFailedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ReportFailedAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUserDataTaskListSyncManagerSyncRequestEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IUserDataTaskListSyncManagerSyncRequestEventArgs {
    type Vtable = IUserDataTaskListSyncManagerSyncRequestEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IUserDataTaskListSyncManagerSyncRequestEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8ead1c12_768e_43bd_8385_5cdc351ffdea);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserDataTaskListSyncManagerSyncRequestEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc = "*Required features: `\"ApplicationModel_UserDataTasks_DataProvider\"`*"]
#[repr(transparent)]
pub struct UserDataTaskDataProviderConnection(::windows::core::IUnknown);
impl UserDataTaskDataProviderConnection {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateOrUpdateTaskRequested(&self, handler: &super::super::super::Foundation::TypedEventHandler<UserDataTaskDataProviderConnection, UserDataTaskListCreateOrUpdateTaskRequestEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateOrUpdateTaskRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCreateOrUpdateTaskRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveCreateOrUpdateTaskRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SyncRequested(&self, handler: &super::super::super::Foundation::TypedEventHandler<UserDataTaskDataProviderConnection, UserDataTaskListSyncManagerSyncRequestEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SyncRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSyncRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveSyncRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SkipOccurrenceRequested(&self, handler: &super::super::super::Foundation::TypedEventHandler<UserDataTaskDataProviderConnection, UserDataTaskListSkipOccurrenceRequestEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SkipOccurrenceRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSkipOccurrenceRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveSkipOccurrenceRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CompleteTaskRequested(&self, handler: &super::super::super::Foundation::TypedEventHandler<UserDataTaskDataProviderConnection, UserDataTaskListCompleteTaskRequestEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CompleteTaskRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCompleteTaskRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveCompleteTaskRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteTaskRequested(&self, handler: &super::super::super::Foundation::TypedEventHandler<UserDataTaskDataProviderConnection, UserDataTaskListDeleteTaskRequestEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DeleteTaskRequested)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveDeleteTaskRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveDeleteTaskRequested)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    pub fn Start(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Start)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for UserDataTaskDataProviderConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataTaskDataProviderConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataTaskDataProviderConnection {}
impl ::core::fmt::Debug for UserDataTaskDataProviderConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskDataProviderConnection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskDataProviderConnection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskDataProviderConnection;{9ff39d1d-a447-428b-afe9-e5402bdeb041})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for UserDataTaskDataProviderConnection {
    type Vtable = IUserDataTaskDataProviderConnection_Vtbl;
}
unsafe impl ::windows::core::Interface for UserDataTaskDataProviderConnection {
    const IID: ::windows::core::GUID = <IUserDataTaskDataProviderConnection as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserDataTaskDataProviderConnection {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskDataProviderConnection";
}
::windows::core::interface_hierarchy!(UserDataTaskDataProviderConnection, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for UserDataTaskDataProviderConnection {}
unsafe impl ::core::marker::Sync for UserDataTaskDataProviderConnection {}
#[doc = "*Required features: `\"ApplicationModel_UserDataTasks_DataProvider\"`*"]
#[repr(transparent)]
pub struct UserDataTaskDataProviderTriggerDetails(::windows::core::IUnknown);
impl UserDataTaskDataProviderTriggerDetails {
    pub fn Connection(&self) -> ::windows::core::Result<UserDataTaskDataProviderConnection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Connection)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for UserDataTaskDataProviderTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataTaskDataProviderTriggerDetails {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataTaskDataProviderTriggerDetails {}
impl ::core::fmt::Debug for UserDataTaskDataProviderTriggerDetails {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskDataProviderTriggerDetails").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskDataProviderTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskDataProviderTriggerDetails;{ae273202-b1c9-453e-afc5-b30af3bd217d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for UserDataTaskDataProviderTriggerDetails {
    type Vtable = IUserDataTaskDataProviderTriggerDetails_Vtbl;
}
unsafe impl ::windows::core::Interface for UserDataTaskDataProviderTriggerDetails {
    const IID: ::windows::core::GUID = <IUserDataTaskDataProviderTriggerDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserDataTaskDataProviderTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskDataProviderTriggerDetails";
}
::windows::core::interface_hierarchy!(UserDataTaskDataProviderTriggerDetails, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for UserDataTaskDataProviderTriggerDetails {}
unsafe impl ::core::marker::Sync for UserDataTaskDataProviderTriggerDetails {}
#[doc = "*Required features: `\"ApplicationModel_UserDataTasks_DataProvider\"`*"]
#[repr(transparent)]
pub struct UserDataTaskListCompleteTaskRequest(::windows::core::IUnknown);
impl UserDataTaskListCompleteTaskRequest {
    pub fn TaskListId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TaskListId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn TaskId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TaskId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportCompletedAsync(&self, completedtaskid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportCompletedAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(completedtaskid), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportFailedAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for UserDataTaskListCompleteTaskRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataTaskListCompleteTaskRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataTaskListCompleteTaskRequest {}
impl ::core::fmt::Debug for UserDataTaskListCompleteTaskRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskListCompleteTaskRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskListCompleteTaskRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListCompleteTaskRequest;{f65e14a3-1a42-49da-8552-2873e52c55eb})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for UserDataTaskListCompleteTaskRequest {
    type Vtable = IUserDataTaskListCompleteTaskRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for UserDataTaskListCompleteTaskRequest {
    const IID: ::windows::core::GUID = <IUserDataTaskListCompleteTaskRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserDataTaskListCompleteTaskRequest {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListCompleteTaskRequest";
}
::windows::core::interface_hierarchy!(UserDataTaskListCompleteTaskRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for UserDataTaskListCompleteTaskRequest {}
unsafe impl ::core::marker::Sync for UserDataTaskListCompleteTaskRequest {}
#[doc = "*Required features: `\"ApplicationModel_UserDataTasks_DataProvider\"`*"]
#[repr(transparent)]
pub struct UserDataTaskListCompleteTaskRequestEventArgs(::windows::core::IUnknown);
impl UserDataTaskListCompleteTaskRequestEventArgs {
    pub fn Request(&self) -> ::windows::core::Result<UserDataTaskListCompleteTaskRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Request)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for UserDataTaskListCompleteTaskRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataTaskListCompleteTaskRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataTaskListCompleteTaskRequestEventArgs {}
impl ::core::fmt::Debug for UserDataTaskListCompleteTaskRequestEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskListCompleteTaskRequestEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskListCompleteTaskRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListCompleteTaskRequestEventArgs;{d77c393d-4cf2-48ad-87fd-963f0eaa7a95})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for UserDataTaskListCompleteTaskRequestEventArgs {
    type Vtable = IUserDataTaskListCompleteTaskRequestEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for UserDataTaskListCompleteTaskRequestEventArgs {
    const IID: ::windows::core::GUID = <IUserDataTaskListCompleteTaskRequestEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserDataTaskListCompleteTaskRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListCompleteTaskRequestEventArgs";
}
::windows::core::interface_hierarchy!(UserDataTaskListCompleteTaskRequestEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for UserDataTaskListCompleteTaskRequestEventArgs {}
unsafe impl ::core::marker::Sync for UserDataTaskListCompleteTaskRequestEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_UserDataTasks_DataProvider\"`*"]
#[repr(transparent)]
pub struct UserDataTaskListCreateOrUpdateTaskRequest(::windows::core::IUnknown);
impl UserDataTaskListCreateOrUpdateTaskRequest {
    pub fn TaskListId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TaskListId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Task(&self) -> ::windows::core::Result<super::UserDataTask> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Task)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportCompletedAsync(&self, createdorupdateduserdatatask: &super::UserDataTask) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportCompletedAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(createdorupdateduserdatatask), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportFailedAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for UserDataTaskListCreateOrUpdateTaskRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataTaskListCreateOrUpdateTaskRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataTaskListCreateOrUpdateTaskRequest {}
impl ::core::fmt::Debug for UserDataTaskListCreateOrUpdateTaskRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskListCreateOrUpdateTaskRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskListCreateOrUpdateTaskRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListCreateOrUpdateTaskRequest;{2133772c-55c2-4300-8279-04326e07cce4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for UserDataTaskListCreateOrUpdateTaskRequest {
    type Vtable = IUserDataTaskListCreateOrUpdateTaskRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for UserDataTaskListCreateOrUpdateTaskRequest {
    const IID: ::windows::core::GUID = <IUserDataTaskListCreateOrUpdateTaskRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserDataTaskListCreateOrUpdateTaskRequest {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListCreateOrUpdateTaskRequest";
}
::windows::core::interface_hierarchy!(UserDataTaskListCreateOrUpdateTaskRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for UserDataTaskListCreateOrUpdateTaskRequest {}
unsafe impl ::core::marker::Sync for UserDataTaskListCreateOrUpdateTaskRequest {}
#[doc = "*Required features: `\"ApplicationModel_UserDataTasks_DataProvider\"`*"]
#[repr(transparent)]
pub struct UserDataTaskListCreateOrUpdateTaskRequestEventArgs(::windows::core::IUnknown);
impl UserDataTaskListCreateOrUpdateTaskRequestEventArgs {
    pub fn Request(&self) -> ::windows::core::Result<UserDataTaskListCreateOrUpdateTaskRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Request)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for UserDataTaskListCreateOrUpdateTaskRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataTaskListCreateOrUpdateTaskRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataTaskListCreateOrUpdateTaskRequestEventArgs {}
impl ::core::fmt::Debug for UserDataTaskListCreateOrUpdateTaskRequestEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskListCreateOrUpdateTaskRequestEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskListCreateOrUpdateTaskRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListCreateOrUpdateTaskRequestEventArgs;{12c55a52-e378-419b-ae38-a5e9e604476e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for UserDataTaskListCreateOrUpdateTaskRequestEventArgs {
    type Vtable = IUserDataTaskListCreateOrUpdateTaskRequestEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for UserDataTaskListCreateOrUpdateTaskRequestEventArgs {
    const IID: ::windows::core::GUID = <IUserDataTaskListCreateOrUpdateTaskRequestEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserDataTaskListCreateOrUpdateTaskRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListCreateOrUpdateTaskRequestEventArgs";
}
::windows::core::interface_hierarchy!(UserDataTaskListCreateOrUpdateTaskRequestEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for UserDataTaskListCreateOrUpdateTaskRequestEventArgs {}
unsafe impl ::core::marker::Sync for UserDataTaskListCreateOrUpdateTaskRequestEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_UserDataTasks_DataProvider\"`*"]
#[repr(transparent)]
pub struct UserDataTaskListDeleteTaskRequest(::windows::core::IUnknown);
impl UserDataTaskListDeleteTaskRequest {
    pub fn TaskListId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TaskListId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn TaskId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TaskId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportCompletedAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportFailedAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for UserDataTaskListDeleteTaskRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataTaskListDeleteTaskRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataTaskListDeleteTaskRequest {}
impl ::core::fmt::Debug for UserDataTaskListDeleteTaskRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskListDeleteTaskRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskListDeleteTaskRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListDeleteTaskRequest;{4b863c68-7657-4f3d-b074-d47ec8df07e7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for UserDataTaskListDeleteTaskRequest {
    type Vtable = IUserDataTaskListDeleteTaskRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for UserDataTaskListDeleteTaskRequest {
    const IID: ::windows::core::GUID = <IUserDataTaskListDeleteTaskRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserDataTaskListDeleteTaskRequest {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListDeleteTaskRequest";
}
::windows::core::interface_hierarchy!(UserDataTaskListDeleteTaskRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for UserDataTaskListDeleteTaskRequest {}
unsafe impl ::core::marker::Sync for UserDataTaskListDeleteTaskRequest {}
#[doc = "*Required features: `\"ApplicationModel_UserDataTasks_DataProvider\"`*"]
#[repr(transparent)]
pub struct UserDataTaskListDeleteTaskRequestEventArgs(::windows::core::IUnknown);
impl UserDataTaskListDeleteTaskRequestEventArgs {
    pub fn Request(&self) -> ::windows::core::Result<UserDataTaskListDeleteTaskRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Request)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for UserDataTaskListDeleteTaskRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataTaskListDeleteTaskRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataTaskListDeleteTaskRequestEventArgs {}
impl ::core::fmt::Debug for UserDataTaskListDeleteTaskRequestEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskListDeleteTaskRequestEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskListDeleteTaskRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListDeleteTaskRequestEventArgs;{6063dad9-f562-4145-8efe-d50078c92b7f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for UserDataTaskListDeleteTaskRequestEventArgs {
    type Vtable = IUserDataTaskListDeleteTaskRequestEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for UserDataTaskListDeleteTaskRequestEventArgs {
    const IID: ::windows::core::GUID = <IUserDataTaskListDeleteTaskRequestEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserDataTaskListDeleteTaskRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListDeleteTaskRequestEventArgs";
}
::windows::core::interface_hierarchy!(UserDataTaskListDeleteTaskRequestEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for UserDataTaskListDeleteTaskRequestEventArgs {}
unsafe impl ::core::marker::Sync for UserDataTaskListDeleteTaskRequestEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_UserDataTasks_DataProvider\"`*"]
#[repr(transparent)]
pub struct UserDataTaskListSkipOccurrenceRequest(::windows::core::IUnknown);
impl UserDataTaskListSkipOccurrenceRequest {
    pub fn TaskListId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TaskListId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn TaskId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TaskId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportCompletedAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportFailedAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for UserDataTaskListSkipOccurrenceRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataTaskListSkipOccurrenceRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataTaskListSkipOccurrenceRequest {}
impl ::core::fmt::Debug for UserDataTaskListSkipOccurrenceRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskListSkipOccurrenceRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskListSkipOccurrenceRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListSkipOccurrenceRequest;{ab87e34d-1cd3-431c-9f58-089aa4338d85})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for UserDataTaskListSkipOccurrenceRequest {
    type Vtable = IUserDataTaskListSkipOccurrenceRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for UserDataTaskListSkipOccurrenceRequest {
    const IID: ::windows::core::GUID = <IUserDataTaskListSkipOccurrenceRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserDataTaskListSkipOccurrenceRequest {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListSkipOccurrenceRequest";
}
::windows::core::interface_hierarchy!(UserDataTaskListSkipOccurrenceRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for UserDataTaskListSkipOccurrenceRequest {}
unsafe impl ::core::marker::Sync for UserDataTaskListSkipOccurrenceRequest {}
#[doc = "*Required features: `\"ApplicationModel_UserDataTasks_DataProvider\"`*"]
#[repr(transparent)]
pub struct UserDataTaskListSkipOccurrenceRequestEventArgs(::windows::core::IUnknown);
impl UserDataTaskListSkipOccurrenceRequestEventArgs {
    pub fn Request(&self) -> ::windows::core::Result<UserDataTaskListSkipOccurrenceRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Request)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for UserDataTaskListSkipOccurrenceRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataTaskListSkipOccurrenceRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataTaskListSkipOccurrenceRequestEventArgs {}
impl ::core::fmt::Debug for UserDataTaskListSkipOccurrenceRequestEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskListSkipOccurrenceRequestEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskListSkipOccurrenceRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListSkipOccurrenceRequestEventArgs;{7a3b924a-cc2f-4e7b-aacd-a5b9d29cfa4e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for UserDataTaskListSkipOccurrenceRequestEventArgs {
    type Vtable = IUserDataTaskListSkipOccurrenceRequestEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for UserDataTaskListSkipOccurrenceRequestEventArgs {
    const IID: ::windows::core::GUID = <IUserDataTaskListSkipOccurrenceRequestEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserDataTaskListSkipOccurrenceRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListSkipOccurrenceRequestEventArgs";
}
::windows::core::interface_hierarchy!(UserDataTaskListSkipOccurrenceRequestEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for UserDataTaskListSkipOccurrenceRequestEventArgs {}
unsafe impl ::core::marker::Sync for UserDataTaskListSkipOccurrenceRequestEventArgs {}
#[doc = "*Required features: `\"ApplicationModel_UserDataTasks_DataProvider\"`*"]
#[repr(transparent)]
pub struct UserDataTaskListSyncManagerSyncRequest(::windows::core::IUnknown);
impl UserDataTaskListSyncManagerSyncRequest {
    pub fn TaskListId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TaskListId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportCompletedAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReportFailedAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for UserDataTaskListSyncManagerSyncRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataTaskListSyncManagerSyncRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataTaskListSyncManagerSyncRequest {}
impl ::core::fmt::Debug for UserDataTaskListSyncManagerSyncRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskListSyncManagerSyncRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskListSyncManagerSyncRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListSyncManagerSyncRequest;{40a73807-7590-4149-ae19-b211431a9f48})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for UserDataTaskListSyncManagerSyncRequest {
    type Vtable = IUserDataTaskListSyncManagerSyncRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for UserDataTaskListSyncManagerSyncRequest {
    const IID: ::windows::core::GUID = <IUserDataTaskListSyncManagerSyncRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserDataTaskListSyncManagerSyncRequest {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListSyncManagerSyncRequest";
}
::windows::core::interface_hierarchy!(UserDataTaskListSyncManagerSyncRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for UserDataTaskListSyncManagerSyncRequest {}
unsafe impl ::core::marker::Sync for UserDataTaskListSyncManagerSyncRequest {}
#[doc = "*Required features: `\"ApplicationModel_UserDataTasks_DataProvider\"`*"]
#[repr(transparent)]
pub struct UserDataTaskListSyncManagerSyncRequestEventArgs(::windows::core::IUnknown);
impl UserDataTaskListSyncManagerSyncRequestEventArgs {
    pub fn Request(&self) -> ::windows::core::Result<UserDataTaskListSyncManagerSyncRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Request)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDeferral)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for UserDataTaskListSyncManagerSyncRequestEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UserDataTaskListSyncManagerSyncRequestEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UserDataTaskListSyncManagerSyncRequestEventArgs {}
impl ::core::fmt::Debug for UserDataTaskListSyncManagerSyncRequestEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UserDataTaskListSyncManagerSyncRequestEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UserDataTaskListSyncManagerSyncRequestEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListSyncManagerSyncRequestEventArgs;{8ead1c12-768e-43bd-8385-5cdc351ffdea})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for UserDataTaskListSyncManagerSyncRequestEventArgs {
    type Vtable = IUserDataTaskListSyncManagerSyncRequestEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for UserDataTaskListSyncManagerSyncRequestEventArgs {
    const IID: ::windows::core::GUID = <IUserDataTaskListSyncManagerSyncRequestEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UserDataTaskListSyncManagerSyncRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListSyncManagerSyncRequestEventArgs";
}
::windows::core::interface_hierarchy!(UserDataTaskListSyncManagerSyncRequestEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for UserDataTaskListSyncManagerSyncRequestEventArgs {}
unsafe impl ::core::marker::Sync for UserDataTaskListSyncManagerSyncRequestEventArgs {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

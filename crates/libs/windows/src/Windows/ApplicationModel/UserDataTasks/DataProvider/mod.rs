::windows_core::imp::com_interface!(IUserDataTaskDataProviderConnection, IUserDataTaskDataProviderConnection_Vtbl, 0x9ff39d1d_a447_428b_afe9_e5402bdeb041);
#[repr(C)]
pub struct IUserDataTaskDataProviderConnection_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateOrUpdateTaskRequested: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveCreateOrUpdateTaskRequested: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub SyncRequested: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveSyncRequested: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub SkipOccurrenceRequested: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveSkipOccurrenceRequested: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub CompleteTaskRequested: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveCompleteTaskRequested: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub DeleteTaskRequested: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub RemoveDeleteTaskRequested: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IUserDataTaskDataProviderTriggerDetails, IUserDataTaskDataProviderTriggerDetails_Vtbl, 0xae273202_b1c9_453e_afc5_b30af3bd217d);
#[repr(C)]
pub struct IUserDataTaskDataProviderTriggerDetails_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Connection: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IUserDataTaskListCompleteTaskRequest, IUserDataTaskListCompleteTaskRequest_Vtbl, 0xf65e14a3_1a42_49da_8552_2873e52c55eb);
#[repr(C)]
pub struct IUserDataTaskListCompleteTaskRequest_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TaskListId: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TaskId: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ReportCompletedAsync: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::std::mem::MaybeUninit<::windows_core::HSTRING>, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReportFailedAsync: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IUserDataTaskListCompleteTaskRequestEventArgs, IUserDataTaskListCompleteTaskRequestEventArgs_Vtbl, 0xd77c393d_4cf2_48ad_87fd_963f0eaa7a95);
#[repr(C)]
pub struct IUserDataTaskListCompleteTaskRequestEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IUserDataTaskListCreateOrUpdateTaskRequest, IUserDataTaskListCreateOrUpdateTaskRequest_Vtbl, 0x2133772c_55c2_4300_8279_04326e07cce4);
#[repr(C)]
pub struct IUserDataTaskListCreateOrUpdateTaskRequest_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TaskListId: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Task: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReportCompletedAsync: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReportFailedAsync: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IUserDataTaskListCreateOrUpdateTaskRequestEventArgs, IUserDataTaskListCreateOrUpdateTaskRequestEventArgs_Vtbl, 0x12c55a52_e378_419b_ae38_a5e9e604476e);
#[repr(C)]
pub struct IUserDataTaskListCreateOrUpdateTaskRequestEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IUserDataTaskListDeleteTaskRequest, IUserDataTaskListDeleteTaskRequest_Vtbl, 0x4b863c68_7657_4f3d_b074_d47ec8df07e7);
#[repr(C)]
pub struct IUserDataTaskListDeleteTaskRequest_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TaskListId: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TaskId: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ReportCompletedAsync: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReportFailedAsync: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IUserDataTaskListDeleteTaskRequestEventArgs, IUserDataTaskListDeleteTaskRequestEventArgs_Vtbl, 0x6063dad9_f562_4145_8efe_d50078c92b7f);
#[repr(C)]
pub struct IUserDataTaskListDeleteTaskRequestEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IUserDataTaskListSkipOccurrenceRequest, IUserDataTaskListSkipOccurrenceRequest_Vtbl, 0xab87e34d_1cd3_431c_9f58_089aa4338d85);
#[repr(C)]
pub struct IUserDataTaskListSkipOccurrenceRequest_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TaskListId: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub TaskId: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ReportCompletedAsync: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReportFailedAsync: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IUserDataTaskListSkipOccurrenceRequestEventArgs, IUserDataTaskListSkipOccurrenceRequestEventArgs_Vtbl, 0x7a3b924a_cc2f_4e7b_aacd_a5b9d29cfa4e);
#[repr(C)]
pub struct IUserDataTaskListSkipOccurrenceRequestEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IUserDataTaskListSyncManagerSyncRequest, IUserDataTaskListSyncManagerSyncRequest_Vtbl, 0x40a73807_7590_4149_ae19_b211431a9f48);
#[repr(C)]
pub struct IUserDataTaskListSyncManagerSyncRequest_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub TaskListId: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ReportCompletedAsync: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ReportFailedAsync: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IUserDataTaskListSyncManagerSyncRequestEventArgs, IUserDataTaskListSyncManagerSyncRequestEventArgs_Vtbl, 0x8ead1c12_768e_43bd_8385_5cdc351ffdea);
#[repr(C)]
pub struct IUserDataTaskListSyncManagerSyncRequestEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct UserDataTaskDataProviderConnection(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(UserDataTaskDataProviderConnection, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl UserDataTaskDataProviderConnection {
    pub fn CreateOrUpdateTaskRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<UserDataTaskDataProviderConnection, UserDataTaskListCreateOrUpdateTaskRequestEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateOrUpdateTaskRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCreateOrUpdateTaskRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCreateOrUpdateTaskRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SyncRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<UserDataTaskDataProviderConnection, UserDataTaskListSyncManagerSyncRequestEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SyncRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSyncRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSyncRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn SkipOccurrenceRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<UserDataTaskDataProviderConnection, UserDataTaskListSkipOccurrenceRequestEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SkipOccurrenceRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSkipOccurrenceRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveSkipOccurrenceRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn CompleteTaskRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<UserDataTaskDataProviderConnection, UserDataTaskListCompleteTaskRequestEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CompleteTaskRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveCompleteTaskRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveCompleteTaskRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn DeleteTaskRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<UserDataTaskDataProviderConnection, UserDataTaskListDeleteTaskRequestEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DeleteTaskRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveDeleteTaskRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveDeleteTaskRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
impl ::windows_core::RuntimeType for UserDataTaskDataProviderConnection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for UserDataTaskDataProviderConnection {
    type Vtable = IUserDataTaskDataProviderConnection_Vtbl;
    const IID: ::windows_core::GUID = <IUserDataTaskDataProviderConnection as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserDataTaskDataProviderConnection {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskDataProviderConnection";
}
unsafe impl ::core::marker::Send for UserDataTaskDataProviderConnection {}
unsafe impl ::core::marker::Sync for UserDataTaskDataProviderConnection {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct UserDataTaskDataProviderTriggerDetails(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(UserDataTaskDataProviderTriggerDetails, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl UserDataTaskDataProviderTriggerDetails {
    pub fn Connection(&self) -> ::windows_core::Result<UserDataTaskDataProviderConnection> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Connection)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
}
impl ::windows_core::RuntimeType for UserDataTaskDataProviderTriggerDetails {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for UserDataTaskDataProviderTriggerDetails {
    type Vtable = IUserDataTaskDataProviderTriggerDetails_Vtbl;
    const IID: ::windows_core::GUID = <IUserDataTaskDataProviderTriggerDetails as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserDataTaskDataProviderTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskDataProviderTriggerDetails";
}
unsafe impl ::core::marker::Send for UserDataTaskDataProviderTriggerDetails {}
unsafe impl ::core::marker::Sync for UserDataTaskDataProviderTriggerDetails {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct UserDataTaskListCompleteTaskRequest(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(UserDataTaskListCompleteTaskRequest, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl UserDataTaskListCompleteTaskRequest {
    pub fn TaskListId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TaskListId)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn TaskId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TaskId)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportCompletedAsync(&self, completedtaskid: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportCompletedAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(completedtaskid), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportFailedAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedAsync)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
}
impl ::windows_core::RuntimeType for UserDataTaskListCompleteTaskRequest {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for UserDataTaskListCompleteTaskRequest {
    type Vtable = IUserDataTaskListCompleteTaskRequest_Vtbl;
    const IID: ::windows_core::GUID = <IUserDataTaskListCompleteTaskRequest as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserDataTaskListCompleteTaskRequest {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListCompleteTaskRequest";
}
unsafe impl ::core::marker::Send for UserDataTaskListCompleteTaskRequest {}
unsafe impl ::core::marker::Sync for UserDataTaskListCompleteTaskRequest {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct UserDataTaskListCompleteTaskRequestEventArgs(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(UserDataTaskListCompleteTaskRequestEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl UserDataTaskListCompleteTaskRequestEventArgs {
    pub fn Request(&self) -> ::windows_core::Result<UserDataTaskListCompleteTaskRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Request)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
}
impl ::windows_core::RuntimeType for UserDataTaskListCompleteTaskRequestEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for UserDataTaskListCompleteTaskRequestEventArgs {
    type Vtable = IUserDataTaskListCompleteTaskRequestEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IUserDataTaskListCompleteTaskRequestEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserDataTaskListCompleteTaskRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListCompleteTaskRequestEventArgs";
}
unsafe impl ::core::marker::Send for UserDataTaskListCompleteTaskRequestEventArgs {}
unsafe impl ::core::marker::Sync for UserDataTaskListCompleteTaskRequestEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct UserDataTaskListCreateOrUpdateTaskRequest(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(UserDataTaskListCreateOrUpdateTaskRequest, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl UserDataTaskListCreateOrUpdateTaskRequest {
    pub fn TaskListId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TaskListId)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn Task(&self) -> ::windows_core::Result<super::UserDataTask> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Task)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportCompletedAsync<P0>(&self, createdorupdateduserdatatask: P0) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::IntoParam<super::UserDataTask>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportCompletedAsync)(::windows_core::Interface::as_raw(this), createdorupdateduserdatatask.into_param().abi(), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportFailedAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedAsync)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
}
impl ::windows_core::RuntimeType for UserDataTaskListCreateOrUpdateTaskRequest {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for UserDataTaskListCreateOrUpdateTaskRequest {
    type Vtable = IUserDataTaskListCreateOrUpdateTaskRequest_Vtbl;
    const IID: ::windows_core::GUID = <IUserDataTaskListCreateOrUpdateTaskRequest as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserDataTaskListCreateOrUpdateTaskRequest {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListCreateOrUpdateTaskRequest";
}
unsafe impl ::core::marker::Send for UserDataTaskListCreateOrUpdateTaskRequest {}
unsafe impl ::core::marker::Sync for UserDataTaskListCreateOrUpdateTaskRequest {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct UserDataTaskListCreateOrUpdateTaskRequestEventArgs(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(UserDataTaskListCreateOrUpdateTaskRequestEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl UserDataTaskListCreateOrUpdateTaskRequestEventArgs {
    pub fn Request(&self) -> ::windows_core::Result<UserDataTaskListCreateOrUpdateTaskRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Request)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
}
impl ::windows_core::RuntimeType for UserDataTaskListCreateOrUpdateTaskRequestEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for UserDataTaskListCreateOrUpdateTaskRequestEventArgs {
    type Vtable = IUserDataTaskListCreateOrUpdateTaskRequestEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IUserDataTaskListCreateOrUpdateTaskRequestEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserDataTaskListCreateOrUpdateTaskRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListCreateOrUpdateTaskRequestEventArgs";
}
unsafe impl ::core::marker::Send for UserDataTaskListCreateOrUpdateTaskRequestEventArgs {}
unsafe impl ::core::marker::Sync for UserDataTaskListCreateOrUpdateTaskRequestEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct UserDataTaskListDeleteTaskRequest(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(UserDataTaskListDeleteTaskRequest, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl UserDataTaskListDeleteTaskRequest {
    pub fn TaskListId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TaskListId)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn TaskId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TaskId)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportCompletedAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportCompletedAsync)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportFailedAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedAsync)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
}
impl ::windows_core::RuntimeType for UserDataTaskListDeleteTaskRequest {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for UserDataTaskListDeleteTaskRequest {
    type Vtable = IUserDataTaskListDeleteTaskRequest_Vtbl;
    const IID: ::windows_core::GUID = <IUserDataTaskListDeleteTaskRequest as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserDataTaskListDeleteTaskRequest {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListDeleteTaskRequest";
}
unsafe impl ::core::marker::Send for UserDataTaskListDeleteTaskRequest {}
unsafe impl ::core::marker::Sync for UserDataTaskListDeleteTaskRequest {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct UserDataTaskListDeleteTaskRequestEventArgs(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(UserDataTaskListDeleteTaskRequestEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl UserDataTaskListDeleteTaskRequestEventArgs {
    pub fn Request(&self) -> ::windows_core::Result<UserDataTaskListDeleteTaskRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Request)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
}
impl ::windows_core::RuntimeType for UserDataTaskListDeleteTaskRequestEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for UserDataTaskListDeleteTaskRequestEventArgs {
    type Vtable = IUserDataTaskListDeleteTaskRequestEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IUserDataTaskListDeleteTaskRequestEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserDataTaskListDeleteTaskRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListDeleteTaskRequestEventArgs";
}
unsafe impl ::core::marker::Send for UserDataTaskListDeleteTaskRequestEventArgs {}
unsafe impl ::core::marker::Sync for UserDataTaskListDeleteTaskRequestEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct UserDataTaskListSkipOccurrenceRequest(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(UserDataTaskListSkipOccurrenceRequest, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl UserDataTaskListSkipOccurrenceRequest {
    pub fn TaskListId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TaskListId)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn TaskId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TaskId)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportCompletedAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportCompletedAsync)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportFailedAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedAsync)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
}
impl ::windows_core::RuntimeType for UserDataTaskListSkipOccurrenceRequest {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for UserDataTaskListSkipOccurrenceRequest {
    type Vtable = IUserDataTaskListSkipOccurrenceRequest_Vtbl;
    const IID: ::windows_core::GUID = <IUserDataTaskListSkipOccurrenceRequest as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserDataTaskListSkipOccurrenceRequest {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListSkipOccurrenceRequest";
}
unsafe impl ::core::marker::Send for UserDataTaskListSkipOccurrenceRequest {}
unsafe impl ::core::marker::Sync for UserDataTaskListSkipOccurrenceRequest {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct UserDataTaskListSkipOccurrenceRequestEventArgs(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(UserDataTaskListSkipOccurrenceRequestEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl UserDataTaskListSkipOccurrenceRequestEventArgs {
    pub fn Request(&self) -> ::windows_core::Result<UserDataTaskListSkipOccurrenceRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Request)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
}
impl ::windows_core::RuntimeType for UserDataTaskListSkipOccurrenceRequestEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for UserDataTaskListSkipOccurrenceRequestEventArgs {
    type Vtable = IUserDataTaskListSkipOccurrenceRequestEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IUserDataTaskListSkipOccurrenceRequestEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserDataTaskListSkipOccurrenceRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListSkipOccurrenceRequestEventArgs";
}
unsafe impl ::core::marker::Send for UserDataTaskListSkipOccurrenceRequestEventArgs {}
unsafe impl ::core::marker::Sync for UserDataTaskListSkipOccurrenceRequestEventArgs {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct UserDataTaskListSyncManagerSyncRequest(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(UserDataTaskListSyncManagerSyncRequest, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl UserDataTaskListSyncManagerSyncRequest {
    pub fn TaskListId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TaskListId)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportCompletedAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportCompletedAsync)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReportFailedAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReportFailedAsync)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
}
impl ::windows_core::RuntimeType for UserDataTaskListSyncManagerSyncRequest {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for UserDataTaskListSyncManagerSyncRequest {
    type Vtable = IUserDataTaskListSyncManagerSyncRequest_Vtbl;
    const IID: ::windows_core::GUID = <IUserDataTaskListSyncManagerSyncRequest as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserDataTaskListSyncManagerSyncRequest {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListSyncManagerSyncRequest";
}
unsafe impl ::core::marker::Send for UserDataTaskListSyncManagerSyncRequest {}
unsafe impl ::core::marker::Sync for UserDataTaskListSyncManagerSyncRequest {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct UserDataTaskListSyncManagerSyncRequestEventArgs(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(UserDataTaskListSyncManagerSyncRequestEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl UserDataTaskListSyncManagerSyncRequestEventArgs {
    pub fn Request(&self) -> ::windows_core::Result<UserDataTaskListSyncManagerSyncRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Request)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetDeferral(&self) -> ::windows_core::Result<super::super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDeferral)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
}
impl ::windows_core::RuntimeType for UserDataTaskListSyncManagerSyncRequestEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for UserDataTaskListSyncManagerSyncRequestEventArgs {
    type Vtable = IUserDataTaskListSyncManagerSyncRequestEventArgs_Vtbl;
    const IID: ::windows_core::GUID = <IUserDataTaskListSyncManagerSyncRequestEventArgs as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for UserDataTaskListSyncManagerSyncRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.DataProvider.UserDataTaskListSyncManagerSyncRequestEventArgs";
}
unsafe impl ::core::marker::Send for UserDataTaskListSyncManagerSyncRequestEventArgs {}
unsafe impl ::core::marker::Sync for UserDataTaskListSyncManagerSyncRequestEventArgs {}

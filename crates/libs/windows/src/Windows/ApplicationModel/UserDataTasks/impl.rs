#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IUserDataTaskImpl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ListId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RemoteId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetRemoteId(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn CompletedDate(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetCompletedDate(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn Details(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDetails(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DetailsKind(&mut self) -> ::windows::core::Result<UserDataTaskDetailsKind>;
    fn SetDetailsKind(&mut self, value: UserDataTaskDetailsKind) -> ::windows::core::Result<()>;
    fn DueDate(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetDueDate(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn Kind(&mut self) -> ::windows::core::Result<UserDataTaskKind>;
    fn Priority(&mut self) -> ::windows::core::Result<UserDataTaskPriority>;
    fn SetPriority(&mut self, value: UserDataTaskPriority) -> ::windows::core::Result<()>;
    fn RecurrenceProperties(&mut self) -> ::windows::core::Result<UserDataTaskRecurrenceProperties>;
    fn SetRecurrenceProperties(&mut self, value: &::core::option::Option<UserDataTaskRecurrenceProperties>) -> ::windows::core::Result<()>;
    fn RegenerationProperties(&mut self) -> ::windows::core::Result<UserDataTaskRegenerationProperties>;
    fn SetRegenerationProperties(&mut self, value: &::core::option::Option<UserDataTaskRegenerationProperties>) -> ::windows::core::Result<()>;
    fn Reminder(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetReminder(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn Sensitivity(&mut self) -> ::windows::core::Result<UserDataTaskSensitivity>;
    fn SetSensitivity(&mut self, value: UserDataTaskSensitivity) -> ::windows::core::Result<()>;
    fn Subject(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSubject(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn StartDate(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetStartDate(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserDataTask {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.IUserDataTask";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUserDataTaskVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserDataTaskImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserDataTaskVtbl {
        unsafe extern "system" fn Id<Impl: IUserDataTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ListId<Impl: IUserDataTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ListId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoteId<Impl: IUserDataTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteId<Impl: IUserDataTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRemoteId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CompletedDate<Impl: IUserDataTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompletedDate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompletedDate<Impl: IUserDataTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCompletedDate(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Details<Impl: IUserDataTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Details() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDetails<Impl: IUserDataTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDetails(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DetailsKind<Impl: IUserDataTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UserDataTaskDetailsKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DetailsKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDetailsKind<Impl: IUserDataTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: UserDataTaskDetailsKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDetailsKind(value).into()
        }
        unsafe extern "system" fn DueDate<Impl: IUserDataTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DueDate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDueDate<Impl: IUserDataTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDueDate(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Kind<Impl: IUserDataTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UserDataTaskKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Kind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Priority<Impl: IUserDataTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UserDataTaskPriority) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Priority() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Impl: IUserDataTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: UserDataTaskPriority) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPriority(value).into()
        }
        unsafe extern "system" fn RecurrenceProperties<Impl: IUserDataTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecurrenceProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRecurrenceProperties<Impl: IUserDataTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRecurrenceProperties(&*(&value as *const <UserDataTaskRecurrenceProperties as ::windows::core::Abi>::Abi as *const <UserDataTaskRecurrenceProperties as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RegenerationProperties<Impl: IUserDataTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegenerationProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRegenerationProperties<Impl: IUserDataTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRegenerationProperties(&*(&value as *const <UserDataTaskRegenerationProperties as ::windows::core::Abi>::Abi as *const <UserDataTaskRegenerationProperties as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Reminder<Impl: IUserDataTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reminder() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReminder<Impl: IUserDataTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReminder(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Sensitivity<Impl: IUserDataTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UserDataTaskSensitivity) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Sensitivity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSensitivity<Impl: IUserDataTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: UserDataTaskSensitivity) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSensitivity(value).into()
        }
        unsafe extern "system" fn Subject<Impl: IUserDataTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Subject() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubject<Impl: IUserDataTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSubject(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StartDate<Impl: IUserDataTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartDate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartDate<Impl: IUserDataTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStartDate(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserDataTask, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            ListId: ListId::<Impl, IMPL_OFFSET>,
            RemoteId: RemoteId::<Impl, IMPL_OFFSET>,
            SetRemoteId: SetRemoteId::<Impl, IMPL_OFFSET>,
            CompletedDate: CompletedDate::<Impl, IMPL_OFFSET>,
            SetCompletedDate: SetCompletedDate::<Impl, IMPL_OFFSET>,
            Details: Details::<Impl, IMPL_OFFSET>,
            SetDetails: SetDetails::<Impl, IMPL_OFFSET>,
            DetailsKind: DetailsKind::<Impl, IMPL_OFFSET>,
            SetDetailsKind: SetDetailsKind::<Impl, IMPL_OFFSET>,
            DueDate: DueDate::<Impl, IMPL_OFFSET>,
            SetDueDate: SetDueDate::<Impl, IMPL_OFFSET>,
            Kind: Kind::<Impl, IMPL_OFFSET>,
            Priority: Priority::<Impl, IMPL_OFFSET>,
            SetPriority: SetPriority::<Impl, IMPL_OFFSET>,
            RecurrenceProperties: RecurrenceProperties::<Impl, IMPL_OFFSET>,
            SetRecurrenceProperties: SetRecurrenceProperties::<Impl, IMPL_OFFSET>,
            RegenerationProperties: RegenerationProperties::<Impl, IMPL_OFFSET>,
            SetRegenerationProperties: SetRegenerationProperties::<Impl, IMPL_OFFSET>,
            Reminder: Reminder::<Impl, IMPL_OFFSET>,
            SetReminder: SetReminder::<Impl, IMPL_OFFSET>,
            Sensitivity: Sensitivity::<Impl, IMPL_OFFSET>,
            SetSensitivity: SetSensitivity::<Impl, IMPL_OFFSET>,
            Subject: Subject::<Impl, IMPL_OFFSET>,
            SetSubject: SetSubject::<Impl, IMPL_OFFSET>,
            StartDate: StartDate::<Impl, IMPL_OFFSET>,
            SetStartDate: SetStartDate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserDataTask as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IUserDataTaskBatchImpl: Sized {
    fn Tasks(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UserDataTask>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserDataTaskBatch {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.IUserDataTaskBatch";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IUserDataTaskBatchVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserDataTaskBatchImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserDataTaskBatchVtbl {
        unsafe extern "system" fn Tasks<Impl: IUserDataTaskBatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Tasks() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IUserDataTaskBatch, BASE_OFFSET>(), Tasks: Tasks::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserDataTaskBatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IUserDataTaskListImpl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn UserDataAccountId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SourceDisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn OtherAppReadAccess(&mut self) -> ::windows::core::Result<UserDataTaskListOtherAppReadAccess>;
    fn SetOtherAppReadAccess(&mut self, value: UserDataTaskListOtherAppReadAccess) -> ::windows::core::Result<()>;
    fn OtherAppWriteAccess(&mut self) -> ::windows::core::Result<UserDataTaskListOtherAppWriteAccess>;
    fn SetOtherAppWriteAccess(&mut self, value: UserDataTaskListOtherAppWriteAccess) -> ::windows::core::Result<()>;
    fn LimitedWriteOperations(&mut self) -> ::windows::core::Result<UserDataTaskListLimitedWriteOperations>;
    fn SyncManager(&mut self) -> ::windows::core::Result<UserDataTaskListSyncManager>;
    fn RegisterSyncManagerAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn GetTaskReader(&mut self) -> ::windows::core::Result<UserDataTaskReader>;
    fn GetTaskReaderWithOptions(&mut self, options: &::core::option::Option<UserDataTaskQueryOptions>) -> ::windows::core::Result<UserDataTaskReader>;
    fn GetTaskAsync(&mut self, userdatatask: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UserDataTask>>;
    fn SaveTaskAsync(&mut self, userdatatask: &::core::option::Option<UserDataTask>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DeleteTaskAsync(&mut self, userdatataskid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn DeleteAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SaveAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserDataTaskList {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.IUserDataTaskList";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUserDataTaskListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserDataTaskListImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserDataTaskListVtbl {
        unsafe extern "system" fn Id<Impl: IUserDataTaskListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserDataAccountId<Impl: IUserDataTaskListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserDataAccountId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Impl: IUserDataTaskListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Impl: IUserDataTaskListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SourceDisplayName<Impl: IUserDataTaskListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourceDisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OtherAppReadAccess<Impl: IUserDataTaskListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UserDataTaskListOtherAppReadAccess) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OtherAppReadAccess() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOtherAppReadAccess<Impl: IUserDataTaskListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: UserDataTaskListOtherAppReadAccess) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOtherAppReadAccess(value).into()
        }
        unsafe extern "system" fn OtherAppWriteAccess<Impl: IUserDataTaskListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UserDataTaskListOtherAppWriteAccess) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OtherAppWriteAccess() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOtherAppWriteAccess<Impl: IUserDataTaskListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: UserDataTaskListOtherAppWriteAccess) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOtherAppWriteAccess(value).into()
        }
        unsafe extern "system" fn LimitedWriteOperations<Impl: IUserDataTaskListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LimitedWriteOperations() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SyncManager<Impl: IUserDataTaskListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SyncManager() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterSyncManagerAsync<Impl: IUserDataTaskListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterSyncManagerAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTaskReader<Impl: IUserDataTaskListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTaskReader() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTaskReaderWithOptions<Impl: IUserDataTaskListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTaskReaderWithOptions(&*(&options as *const <UserDataTaskQueryOptions as ::windows::core::Abi>::Abi as *const <UserDataTaskQueryOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTaskAsync<Impl: IUserDataTaskListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, userdatatask: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTaskAsync(&*(&userdatatask as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveTaskAsync<Impl: IUserDataTaskListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, userdatatask: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveTaskAsync(&*(&userdatatask as *const <UserDataTask as ::windows::core::Abi>::Abi as *const <UserDataTask as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteTaskAsync<Impl: IUserDataTaskListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, userdatataskid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteTaskAsync(&*(&userdatataskid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteAsync<Impl: IUserDataTaskListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveAsync<Impl: IUserDataTaskListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserDataTaskList, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            UserDataAccountId: UserDataAccountId::<Impl, IMPL_OFFSET>,
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            SetDisplayName: SetDisplayName::<Impl, IMPL_OFFSET>,
            SourceDisplayName: SourceDisplayName::<Impl, IMPL_OFFSET>,
            OtherAppReadAccess: OtherAppReadAccess::<Impl, IMPL_OFFSET>,
            SetOtherAppReadAccess: SetOtherAppReadAccess::<Impl, IMPL_OFFSET>,
            OtherAppWriteAccess: OtherAppWriteAccess::<Impl, IMPL_OFFSET>,
            SetOtherAppWriteAccess: SetOtherAppWriteAccess::<Impl, IMPL_OFFSET>,
            LimitedWriteOperations: LimitedWriteOperations::<Impl, IMPL_OFFSET>,
            SyncManager: SyncManager::<Impl, IMPL_OFFSET>,
            RegisterSyncManagerAsync: RegisterSyncManagerAsync::<Impl, IMPL_OFFSET>,
            GetTaskReader: GetTaskReader::<Impl, IMPL_OFFSET>,
            GetTaskReaderWithOptions: GetTaskReaderWithOptions::<Impl, IMPL_OFFSET>,
            GetTaskAsync: GetTaskAsync::<Impl, IMPL_OFFSET>,
            SaveTaskAsync: SaveTaskAsync::<Impl, IMPL_OFFSET>,
            DeleteTaskAsync: DeleteTaskAsync::<Impl, IMPL_OFFSET>,
            DeleteAsync: DeleteAsync::<Impl, IMPL_OFFSET>,
            SaveAsync: SaveAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserDataTaskList as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IUserDataTaskListLimitedWriteOperationsImpl: Sized {
    fn TryCompleteTaskAsync(&mut self, userdatataskid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn TryCreateOrUpdateTaskAsync(&mut self, userdatatask: &::core::option::Option<UserDataTask>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TryDeleteTaskAsync(&mut self, userdatataskid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn TrySkipOccurrenceAsync(&mut self, userdatataskid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserDataTaskListLimitedWriteOperations {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.IUserDataTaskListLimitedWriteOperations";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUserDataTaskListLimitedWriteOperationsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserDataTaskListLimitedWriteOperationsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserDataTaskListLimitedWriteOperationsVtbl {
        unsafe extern "system" fn TryCompleteTaskAsync<Impl: IUserDataTaskListLimitedWriteOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, userdatataskid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryCompleteTaskAsync(&*(&userdatataskid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryCreateOrUpdateTaskAsync<Impl: IUserDataTaskListLimitedWriteOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, userdatatask: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryCreateOrUpdateTaskAsync(&*(&userdatatask as *const <UserDataTask as ::windows::core::Abi>::Abi as *const <UserDataTask as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryDeleteTaskAsync<Impl: IUserDataTaskListLimitedWriteOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, userdatataskid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryDeleteTaskAsync(&*(&userdatataskid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrySkipOccurrenceAsync<Impl: IUserDataTaskListLimitedWriteOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, userdatataskid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrySkipOccurrenceAsync(&*(&userdatataskid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserDataTaskListLimitedWriteOperations, BASE_OFFSET>(),
            TryCompleteTaskAsync: TryCompleteTaskAsync::<Impl, IMPL_OFFSET>,
            TryCreateOrUpdateTaskAsync: TryCreateOrUpdateTaskAsync::<Impl, IMPL_OFFSET>,
            TryDeleteTaskAsync: TryDeleteTaskAsync::<Impl, IMPL_OFFSET>,
            TrySkipOccurrenceAsync: TrySkipOccurrenceAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserDataTaskListLimitedWriteOperations as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IUserDataTaskListSyncManagerImpl: Sized {
    fn LastAttemptedSyncTime(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn SetLastAttemptedSyncTime(&mut self, value: &super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn LastSuccessfulSyncTime(&mut self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn SetLastSuccessfulSyncTime(&mut self, value: &super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn Status(&mut self) -> ::windows::core::Result<UserDataTaskListSyncStatus>;
    fn SetStatus(&mut self, value: UserDataTaskListSyncStatus) -> ::windows::core::Result<()>;
    fn SyncAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn SyncStatusChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<UserDataTaskListSyncManager, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSyncStatusChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserDataTaskListSyncManager {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.IUserDataTaskListSyncManager";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUserDataTaskListSyncManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserDataTaskListSyncManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserDataTaskListSyncManagerVtbl {
        unsafe extern "system" fn LastAttemptedSyncTime<Impl: IUserDataTaskListSyncManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastAttemptedSyncTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLastAttemptedSyncTime<Impl: IUserDataTaskListSyncManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLastAttemptedSyncTime(&*(&value as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LastSuccessfulSyncTime<Impl: IUserDataTaskListSyncManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastSuccessfulSyncTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLastSuccessfulSyncTime<Impl: IUserDataTaskListSyncManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLastSuccessfulSyncTime(&*(&value as *const <super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Status<Impl: IUserDataTaskListSyncManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UserDataTaskListSyncStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStatus<Impl: IUserDataTaskListSyncManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: UserDataTaskListSyncStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStatus(value).into()
        }
        unsafe extern "system" fn SyncAsync<Impl: IUserDataTaskListSyncManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SyncAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SyncStatusChanged<Impl: IUserDataTaskListSyncManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SyncStatusChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<UserDataTaskListSyncManager, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<UserDataTaskListSyncManager, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSyncStatusChanged<Impl: IUserDataTaskListSyncManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSyncStatusChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserDataTaskListSyncManager, BASE_OFFSET>(),
            LastAttemptedSyncTime: LastAttemptedSyncTime::<Impl, IMPL_OFFSET>,
            SetLastAttemptedSyncTime: SetLastAttemptedSyncTime::<Impl, IMPL_OFFSET>,
            LastSuccessfulSyncTime: LastSuccessfulSyncTime::<Impl, IMPL_OFFSET>,
            SetLastSuccessfulSyncTime: SetLastSuccessfulSyncTime::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
            SetStatus: SetStatus::<Impl, IMPL_OFFSET>,
            SyncAsync: SyncAsync::<Impl, IMPL_OFFSET>,
            SyncStatusChanged: SyncStatusChanged::<Impl, IMPL_OFFSET>,
            RemoveSyncStatusChanged: RemoveSyncStatusChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserDataTaskListSyncManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
pub trait IUserDataTaskManagerImpl: Sized {
    fn RequestStoreAsync(&mut self, accesstype: UserDataTaskStoreAccessType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UserDataTaskStore>>;
    fn User(&mut self) -> ::windows::core::Result<super::super::System::User>;
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserDataTaskManager {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.IUserDataTaskManager";
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl IUserDataTaskManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserDataTaskManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserDataTaskManagerVtbl {
        unsafe extern "system" fn RequestStoreAsync<Impl: IUserDataTaskManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accesstype: UserDataTaskStoreAccessType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestStoreAsync(accesstype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn User<Impl: IUserDataTaskManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).User() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserDataTaskManager, BASE_OFFSET>(),
            RequestStoreAsync: RequestStoreAsync::<Impl, IMPL_OFFSET>,
            User: User::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserDataTaskManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait IUserDataTaskManagerStaticsImpl: Sized {
    fn GetDefault(&mut self) -> ::windows::core::Result<UserDataTaskManager>;
    fn GetForUser(&mut self, user: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<UserDataTaskManager>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserDataTaskManagerStatics {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.IUserDataTaskManagerStatics";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl IUserDataTaskManagerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserDataTaskManagerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserDataTaskManagerStaticsVtbl {
        unsafe extern "system" fn GetDefault<Impl: IUserDataTaskManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefault() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetForUser<Impl: IUserDataTaskManagerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForUser(&*(&user as *const <super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::System::User as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserDataTaskManagerStatics, BASE_OFFSET>(),
            GetDefault: GetDefault::<Impl, IMPL_OFFSET>,
            GetForUser: GetForUser::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserDataTaskManagerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserDataTaskQueryOptionsImpl: Sized {
    fn SortProperty(&mut self) -> ::windows::core::Result<UserDataTaskQuerySortProperty>;
    fn SetSortProperty(&mut self, value: UserDataTaskQuerySortProperty) -> ::windows::core::Result<()>;
    fn Kind(&mut self) -> ::windows::core::Result<UserDataTaskQueryKind>;
    fn SetKind(&mut self, value: UserDataTaskQueryKind) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUserDataTaskQueryOptions {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.IUserDataTaskQueryOptions";
}
#[cfg(feature = "implement_exclusive")]
impl IUserDataTaskQueryOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserDataTaskQueryOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserDataTaskQueryOptionsVtbl {
        unsafe extern "system" fn SortProperty<Impl: IUserDataTaskQueryOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UserDataTaskQuerySortProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SortProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSortProperty<Impl: IUserDataTaskQueryOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: UserDataTaskQuerySortProperty) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSortProperty(value).into()
        }
        unsafe extern "system" fn Kind<Impl: IUserDataTaskQueryOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UserDataTaskQueryKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Kind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKind<Impl: IUserDataTaskQueryOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: UserDataTaskQueryKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKind(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserDataTaskQueryOptions, BASE_OFFSET>(),
            SortProperty: SortProperty::<Impl, IMPL_OFFSET>,
            SetSortProperty: SetSortProperty::<Impl, IMPL_OFFSET>,
            Kind: Kind::<Impl, IMPL_OFFSET>,
            SetKind: SetKind::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserDataTaskQueryOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IUserDataTaskReaderImpl: Sized {
    fn ReadBatchAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UserDataTaskBatch>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserDataTaskReader {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.IUserDataTaskReader";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUserDataTaskReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserDataTaskReaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserDataTaskReaderVtbl {
        unsafe extern "system" fn ReadBatchAsync<Impl: IUserDataTaskReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadBatchAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserDataTaskReader, BASE_OFFSET>(),
            ReadBatchAsync: ReadBatchAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserDataTaskReader as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IUserDataTaskRecurrencePropertiesImpl: Sized {
    fn Unit(&mut self) -> ::windows::core::Result<UserDataTaskRecurrenceUnit>;
    fn SetUnit(&mut self, value: UserDataTaskRecurrenceUnit) -> ::windows::core::Result<()>;
    fn Occurrences(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetOccurrences(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn Until(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetUntil(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn Interval(&mut self) -> ::windows::core::Result<i32>;
    fn SetInterval(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn DaysOfWeek(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<UserDataTaskDaysOfWeek>>;
    fn SetDaysOfWeek(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<UserDataTaskDaysOfWeek>>) -> ::windows::core::Result<()>;
    fn WeekOfMonth(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<UserDataTaskWeekOfMonth>>;
    fn SetWeekOfMonth(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<UserDataTaskWeekOfMonth>>) -> ::windows::core::Result<()>;
    fn Month(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetMonth(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn Day(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetDay(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserDataTaskRecurrenceProperties {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.IUserDataTaskRecurrenceProperties";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUserDataTaskRecurrencePropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserDataTaskRecurrencePropertiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserDataTaskRecurrencePropertiesVtbl {
        unsafe extern "system" fn Unit<Impl: IUserDataTaskRecurrencePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UserDataTaskRecurrenceUnit) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnit<Impl: IUserDataTaskRecurrencePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: UserDataTaskRecurrenceUnit) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUnit(value).into()
        }
        unsafe extern "system" fn Occurrences<Impl: IUserDataTaskRecurrencePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Occurrences() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOccurrences<Impl: IUserDataTaskRecurrencePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOccurrences(&*(&value as *const <super::super::Foundation::IReference<i32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<i32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Until<Impl: IUserDataTaskRecurrencePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Until() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUntil<Impl: IUserDataTaskRecurrencePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUntil(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Interval<Impl: IUserDataTaskRecurrencePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Interval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInterval<Impl: IUserDataTaskRecurrencePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInterval(value).into()
        }
        unsafe extern "system" fn DaysOfWeek<Impl: IUserDataTaskRecurrencePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DaysOfWeek() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDaysOfWeek<Impl: IUserDataTaskRecurrencePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDaysOfWeek(&*(&value as *const <super::super::Foundation::IReference<UserDataTaskDaysOfWeek> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<UserDataTaskDaysOfWeek> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn WeekOfMonth<Impl: IUserDataTaskRecurrencePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WeekOfMonth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWeekOfMonth<Impl: IUserDataTaskRecurrencePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWeekOfMonth(&*(&value as *const <super::super::Foundation::IReference<UserDataTaskWeekOfMonth> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<UserDataTaskWeekOfMonth> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Month<Impl: IUserDataTaskRecurrencePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Month() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMonth<Impl: IUserDataTaskRecurrencePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMonth(&*(&value as *const <super::super::Foundation::IReference<i32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<i32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Day<Impl: IUserDataTaskRecurrencePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Day() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDay<Impl: IUserDataTaskRecurrencePropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDay(&*(&value as *const <super::super::Foundation::IReference<i32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<i32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserDataTaskRecurrenceProperties, BASE_OFFSET>(),
            Unit: Unit::<Impl, IMPL_OFFSET>,
            SetUnit: SetUnit::<Impl, IMPL_OFFSET>,
            Occurrences: Occurrences::<Impl, IMPL_OFFSET>,
            SetOccurrences: SetOccurrences::<Impl, IMPL_OFFSET>,
            Until: Until::<Impl, IMPL_OFFSET>,
            SetUntil: SetUntil::<Impl, IMPL_OFFSET>,
            Interval: Interval::<Impl, IMPL_OFFSET>,
            SetInterval: SetInterval::<Impl, IMPL_OFFSET>,
            DaysOfWeek: DaysOfWeek::<Impl, IMPL_OFFSET>,
            SetDaysOfWeek: SetDaysOfWeek::<Impl, IMPL_OFFSET>,
            WeekOfMonth: WeekOfMonth::<Impl, IMPL_OFFSET>,
            SetWeekOfMonth: SetWeekOfMonth::<Impl, IMPL_OFFSET>,
            Month: Month::<Impl, IMPL_OFFSET>,
            SetMonth: SetMonth::<Impl, IMPL_OFFSET>,
            Day: Day::<Impl, IMPL_OFFSET>,
            SetDay: SetDay::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserDataTaskRecurrenceProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IUserDataTaskRegenerationPropertiesImpl: Sized {
    fn Unit(&mut self) -> ::windows::core::Result<UserDataTaskRegenerationUnit>;
    fn SetUnit(&mut self, value: UserDataTaskRegenerationUnit) -> ::windows::core::Result<()>;
    fn Occurrences(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<i32>>;
    fn SetOccurrences(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<i32>>) -> ::windows::core::Result<()>;
    fn Until(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetUntil(&mut self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn Interval(&mut self) -> ::windows::core::Result<i32>;
    fn SetInterval(&mut self, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserDataTaskRegenerationProperties {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.IUserDataTaskRegenerationProperties";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUserDataTaskRegenerationPropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserDataTaskRegenerationPropertiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserDataTaskRegenerationPropertiesVtbl {
        unsafe extern "system" fn Unit<Impl: IUserDataTaskRegenerationPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut UserDataTaskRegenerationUnit) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnit<Impl: IUserDataTaskRegenerationPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: UserDataTaskRegenerationUnit) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUnit(value).into()
        }
        unsafe extern "system" fn Occurrences<Impl: IUserDataTaskRegenerationPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Occurrences() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOccurrences<Impl: IUserDataTaskRegenerationPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOccurrences(&*(&value as *const <super::super::Foundation::IReference<i32> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<i32> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Until<Impl: IUserDataTaskRegenerationPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Until() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUntil<Impl: IUserDataTaskRegenerationPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUntil(&*(&value as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IReference<super::super::Foundation::DateTime> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Interval<Impl: IUserDataTaskRegenerationPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Interval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInterval<Impl: IUserDataTaskRegenerationPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInterval(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserDataTaskRegenerationProperties, BASE_OFFSET>(),
            Unit: Unit::<Impl, IMPL_OFFSET>,
            SetUnit: SetUnit::<Impl, IMPL_OFFSET>,
            Occurrences: Occurrences::<Impl, IMPL_OFFSET>,
            SetOccurrences: SetOccurrences::<Impl, IMPL_OFFSET>,
            Until: Until::<Impl, IMPL_OFFSET>,
            SetUntil: SetUntil::<Impl, IMPL_OFFSET>,
            Interval: Interval::<Impl, IMPL_OFFSET>,
            SetInterval: SetInterval::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserDataTaskRegenerationProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IUserDataTaskStoreImpl: Sized {
    fn CreateListAsync(&mut self, name: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UserDataTaskList>>;
    fn CreateListInAccountAsync(&mut self, name: &::windows::core::HSTRING, userdataaccountid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UserDataTaskList>>;
    fn FindListsAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<UserDataTaskList>>>;
    fn GetListAsync(&mut self, tasklistid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UserDataTaskList>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IUserDataTaskStore {
    const NAME: &'static str = "Windows.ApplicationModel.UserDataTasks.IUserDataTaskStore";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IUserDataTaskStoreVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserDataTaskStoreImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserDataTaskStoreVtbl {
        unsafe extern "system" fn CreateListAsync<Impl: IUserDataTaskStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateListAsync(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateListInAccountAsync<Impl: IUserDataTaskStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, userdataaccountid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateListInAccountAsync(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&userdataaccountid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindListsAsync<Impl: IUserDataTaskStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindListsAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetListAsync<Impl: IUserDataTaskStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tasklistid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetListAsync(&*(&tasklistid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserDataTaskStore, BASE_OFFSET>(),
            CreateListAsync: CreateListAsync::<Impl, IMPL_OFFSET>,
            CreateListInAccountAsync: CreateListInAccountAsync::<Impl, IMPL_OFFSET>,
            FindListsAsync: FindListsAsync::<Impl, IMPL_OFFSET>,
            GetListAsync: GetListAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserDataTaskStore as ::windows::core::Interface>::IID
    }
}

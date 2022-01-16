pub trait IBackgroundCondition_Impl: Sized {}
impl ::windows::core::RuntimeName for IBackgroundCondition {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundCondition";
}
impl IBackgroundCondition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCondition_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundCondition_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundCondition, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCondition as ::windows::core::Interface>::IID
    }
}
pub trait IBackgroundTask_Impl: Sized {
    fn Run(&mut self, taskinstance: &::core::option::Option<IBackgroundTaskInstance>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IBackgroundTask {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTask";
}
impl IBackgroundTask_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTask_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundTask_Vtbl {
        unsafe extern "system" fn Run<Impl: IBackgroundTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskinstance: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Run(&*(&taskinstance as *const <IBackgroundTaskInstance as ::windows::core::Abi>::Abi as *const <IBackgroundTaskInstance as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundTask, BASE_OFFSET>(), Run: Run::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTask as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IBackgroundTaskInstance_Impl: Sized {
    fn InstanceId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Task(&mut self) -> ::windows::core::Result<BackgroundTaskRegistration>;
    fn Progress(&mut self) -> ::windows::core::Result<u32>;
    fn SetProgress(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn TriggerDetails(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn Canceled(&mut self, cancelhandler: &::core::option::Option<BackgroundTaskCanceledEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCanceled(&mut self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SuspendedCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<BackgroundTaskDeferral>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IBackgroundTaskInstance {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskInstance";
}
#[cfg(feature = "Foundation")]
impl IBackgroundTaskInstance_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTaskInstance_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundTaskInstance_Vtbl {
        unsafe extern "system" fn InstanceId<Impl: IBackgroundTaskInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstanceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Task<Impl: IBackgroundTaskInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Task() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Progress<Impl: IBackgroundTaskInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Progress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProgress<Impl: IBackgroundTaskInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProgress(value).into()
        }
        unsafe extern "system" fn TriggerDetails<Impl: IBackgroundTaskInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TriggerDetails() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Canceled<Impl: IBackgroundTaskInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cancelhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Canceled(&*(&cancelhandler as *const <BackgroundTaskCanceledEventHandler as ::windows::core::Abi>::Abi as *const <BackgroundTaskCanceledEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCanceled<Impl: IBackgroundTaskInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCanceled(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SuspendedCount<Impl: IBackgroundTaskInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SuspendedCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IBackgroundTaskInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundTaskInstance, BASE_OFFSET>(),
            InstanceId: InstanceId::<Impl, IMPL_OFFSET>,
            Task: Task::<Impl, IMPL_OFFSET>,
            Progress: Progress::<Impl, IMPL_OFFSET>,
            SetProgress: SetProgress::<Impl, IMPL_OFFSET>,
            TriggerDetails: TriggerDetails::<Impl, IMPL_OFFSET>,
            Canceled: Canceled::<Impl, IMPL_OFFSET>,
            RemoveCanceled: RemoveCanceled::<Impl, IMPL_OFFSET>,
            SuspendedCount: SuspendedCount::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTaskInstance as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IBackgroundTaskInstance2_Impl: Sized + IBackgroundTaskInstance_Impl {
    fn GetThrottleCount(&mut self, counter: BackgroundTaskThrottleCounter) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IBackgroundTaskInstance2 {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskInstance2";
}
#[cfg(feature = "Foundation")]
impl IBackgroundTaskInstance2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTaskInstance2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundTaskInstance2_Vtbl {
        unsafe extern "system" fn GetThrottleCount<Impl: IBackgroundTaskInstance2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, counter: BackgroundTaskThrottleCounter, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetThrottleCount(counter) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundTaskInstance2, BASE_OFFSET>(),
            GetThrottleCount: GetThrottleCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTaskInstance2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "System"))]
pub trait IBackgroundTaskInstance4_Impl: Sized + IBackgroundTaskInstance_Impl {
    fn User(&mut self) -> ::windows::core::Result<super::super::System::User>;
}
#[cfg(all(feature = "Foundation", feature = "System"))]
impl ::windows::core::RuntimeName for IBackgroundTaskInstance4 {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskInstance4";
}
#[cfg(all(feature = "Foundation", feature = "System"))]
impl IBackgroundTaskInstance4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTaskInstance4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundTaskInstance4_Vtbl {
        unsafe extern "system" fn User<Impl: IBackgroundTaskInstance4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundTaskInstance4, BASE_OFFSET>(), User: User::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTaskInstance4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IBackgroundTaskRegistration_Impl: Sized {
    fn TaskId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Progress(&mut self, handler: &::core::option::Option<BackgroundTaskProgressEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveProgress(&mut self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Completed(&mut self, handler: &::core::option::Option<BackgroundTaskCompletedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCompleted(&mut self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Unregister(&mut self, canceltask: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IBackgroundTaskRegistration {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskRegistration";
}
#[cfg(feature = "Foundation")]
impl IBackgroundTaskRegistration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTaskRegistration_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundTaskRegistration_Vtbl {
        unsafe extern "system" fn TaskId<Impl: IBackgroundTaskRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TaskId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IBackgroundTaskRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Progress<Impl: IBackgroundTaskRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Progress(&*(&handler as *const <BackgroundTaskProgressEventHandler as ::windows::core::Abi>::Abi as *const <BackgroundTaskProgressEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveProgress<Impl: IBackgroundTaskRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveProgress(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Completed<Impl: IBackgroundTaskRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Completed(&*(&handler as *const <BackgroundTaskCompletedEventHandler as ::windows::core::Abi>::Abi as *const <BackgroundTaskCompletedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCompleted<Impl: IBackgroundTaskRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCompleted(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Unregister<Impl: IBackgroundTaskRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, canceltask: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unregister(canceltask).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundTaskRegistration, BASE_OFFSET>(),
            TaskId: TaskId::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            Progress: Progress::<Impl, IMPL_OFFSET>,
            RemoveProgress: RemoveProgress::<Impl, IMPL_OFFSET>,
            Completed: Completed::<Impl, IMPL_OFFSET>,
            RemoveCompleted: RemoveCompleted::<Impl, IMPL_OFFSET>,
            Unregister: Unregister::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTaskRegistration as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IBackgroundTaskRegistration2_Impl: Sized + IBackgroundTaskRegistration_Impl {
    fn Trigger(&mut self) -> ::windows::core::Result<IBackgroundTrigger>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IBackgroundTaskRegistration2 {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskRegistration2";
}
#[cfg(feature = "Foundation")]
impl IBackgroundTaskRegistration2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTaskRegistration2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundTaskRegistration2_Vtbl {
        unsafe extern "system" fn Trigger<Impl: IBackgroundTaskRegistration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Trigger() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundTaskRegistration2, BASE_OFFSET>(), Trigger: Trigger::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTaskRegistration2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IBackgroundTaskRegistration3_Impl: Sized + IBackgroundTaskRegistration_Impl {
    fn TaskGroup(&mut self) -> ::windows::core::Result<BackgroundTaskRegistrationGroup>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IBackgroundTaskRegistration3 {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskRegistration3";
}
#[cfg(feature = "Foundation")]
impl IBackgroundTaskRegistration3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTaskRegistration3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundTaskRegistration3_Vtbl {
        unsafe extern "system" fn TaskGroup<Impl: IBackgroundTaskRegistration3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TaskGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundTaskRegistration3, BASE_OFFSET>(), TaskGroup: TaskGroup::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTaskRegistration3 as ::windows::core::Interface>::IID
    }
}
pub trait IBackgroundTrigger_Impl: Sized {}
impl ::windows::core::RuntimeName for IBackgroundTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTrigger";
}
impl IBackgroundTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBackgroundTrigger_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundTrigger, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTrigger as ::windows::core::Interface>::IID
    }
}

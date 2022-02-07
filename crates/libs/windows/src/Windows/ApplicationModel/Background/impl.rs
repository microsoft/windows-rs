pub trait IBackgroundCondition_Impl: Sized {}
impl ::windows::core::RuntimeName for IBackgroundCondition {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundCondition";
}
impl IBackgroundCondition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundCondition_Impl, const OFFSET: isize>() -> IBackgroundCondition_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundCondition, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundCondition as ::windows::core::Interface>::IID
    }
}
pub trait IBackgroundTask_Impl: Sized {
    fn Run(&self, taskinstance: &::core::option::Option<IBackgroundTaskInstance>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IBackgroundTask {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTask";
}
impl IBackgroundTask_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTask_Impl, const OFFSET: isize>() -> IBackgroundTask_Vtbl {
        unsafe extern "system" fn Run<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskinstance: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Run(::core::mem::transmute(&taskinstance)).into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundTask, OFFSET>(), Run: Run::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTask as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IBackgroundTaskInstance_Impl: Sized {
    fn InstanceId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Task(&self) -> ::windows::core::Result<BackgroundTaskRegistration>;
    fn Progress(&self) -> ::windows::core::Result<u32>;
    fn SetProgress(&self, value: u32) -> ::windows::core::Result<()>;
    fn TriggerDetails(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn Canceled(&self, cancelhandler: &::core::option::Option<BackgroundTaskCanceledEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCanceled(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SuspendedCount(&self) -> ::windows::core::Result<u32>;
    fn GetDeferral(&self) -> ::windows::core::Result<BackgroundTaskDeferral>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IBackgroundTaskInstance {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskInstance";
}
#[cfg(feature = "Foundation")]
impl IBackgroundTaskInstance_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTaskInstance_Impl, const OFFSET: isize>() -> IBackgroundTaskInstance_Vtbl {
        unsafe extern "system" fn InstanceId<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTaskInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InstanceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Task<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTaskInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Task() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Progress<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTaskInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Progress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProgress<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTaskInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProgress(value).into()
        }
        unsafe extern "system" fn TriggerDetails<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTaskInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TriggerDetails() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Canceled<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTaskInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cancelhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Canceled(::core::mem::transmute(&cancelhandler)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCanceled<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTaskInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveCanceled(::core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn SuspendedCount<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTaskInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SuspendedCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTaskInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundTaskInstance, OFFSET>(),
            InstanceId: InstanceId::<Identity, Impl, OFFSET>,
            Task: Task::<Identity, Impl, OFFSET>,
            Progress: Progress::<Identity, Impl, OFFSET>,
            SetProgress: SetProgress::<Identity, Impl, OFFSET>,
            TriggerDetails: TriggerDetails::<Identity, Impl, OFFSET>,
            Canceled: Canceled::<Identity, Impl, OFFSET>,
            RemoveCanceled: RemoveCanceled::<Identity, Impl, OFFSET>,
            SuspendedCount: SuspendedCount::<Identity, Impl, OFFSET>,
            GetDeferral: GetDeferral::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTaskInstance as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IBackgroundTaskInstance2_Impl: Sized + IBackgroundTaskInstance_Impl {
    fn GetThrottleCount(&self, counter: BackgroundTaskThrottleCounter) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IBackgroundTaskInstance2 {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskInstance2";
}
#[cfg(feature = "Foundation")]
impl IBackgroundTaskInstance2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTaskInstance2_Impl, const OFFSET: isize>() -> IBackgroundTaskInstance2_Vtbl {
        unsafe extern "system" fn GetThrottleCount<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTaskInstance2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, counter: BackgroundTaskThrottleCounter, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundTaskInstance2, OFFSET>(),
            GetThrottleCount: GetThrottleCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTaskInstance2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "System"))]
pub trait IBackgroundTaskInstance4_Impl: Sized + IBackgroundTaskInstance_Impl {
    fn User(&self) -> ::windows::core::Result<super::super::System::User>;
}
#[cfg(all(feature = "Foundation", feature = "System"))]
impl ::windows::core::RuntimeName for IBackgroundTaskInstance4 {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskInstance4";
}
#[cfg(all(feature = "Foundation", feature = "System"))]
impl IBackgroundTaskInstance4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTaskInstance4_Impl, const OFFSET: isize>() -> IBackgroundTaskInstance4_Vtbl {
        unsafe extern "system" fn User<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTaskInstance4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).User() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundTaskInstance4, OFFSET>(), User: User::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTaskInstance4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IBackgroundTaskRegistration_Impl: Sized {
    fn TaskId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Progress(&self, handler: &::core::option::Option<BackgroundTaskProgressEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveProgress(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Completed(&self, handler: &::core::option::Option<BackgroundTaskCompletedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCompleted(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Unregister(&self, canceltask: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IBackgroundTaskRegistration {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskRegistration";
}
#[cfg(feature = "Foundation")]
impl IBackgroundTaskRegistration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTaskRegistration_Impl, const OFFSET: isize>() -> IBackgroundTaskRegistration_Vtbl {
        unsafe extern "system" fn TaskId<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTaskRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TaskId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTaskRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Progress<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTaskRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Progress(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveProgress<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTaskRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveProgress(::core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn Completed<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTaskRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Completed(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCompleted<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTaskRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveCompleted(::core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn Unregister<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTaskRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, canceltask: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Unregister(canceltask).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundTaskRegistration, OFFSET>(),
            TaskId: TaskId::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            Progress: Progress::<Identity, Impl, OFFSET>,
            RemoveProgress: RemoveProgress::<Identity, Impl, OFFSET>,
            Completed: Completed::<Identity, Impl, OFFSET>,
            RemoveCompleted: RemoveCompleted::<Identity, Impl, OFFSET>,
            Unregister: Unregister::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTaskRegistration as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IBackgroundTaskRegistration2_Impl: Sized + IBackgroundTaskRegistration_Impl {
    fn Trigger(&self) -> ::windows::core::Result<IBackgroundTrigger>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IBackgroundTaskRegistration2 {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskRegistration2";
}
#[cfg(feature = "Foundation")]
impl IBackgroundTaskRegistration2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTaskRegistration2_Impl, const OFFSET: isize>() -> IBackgroundTaskRegistration2_Vtbl {
        unsafe extern "system" fn Trigger<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTaskRegistration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Trigger() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundTaskRegistration2, OFFSET>(), Trigger: Trigger::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTaskRegistration2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IBackgroundTaskRegistration3_Impl: Sized + IBackgroundTaskRegistration_Impl {
    fn TaskGroup(&self) -> ::windows::core::Result<BackgroundTaskRegistrationGroup>;
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IBackgroundTaskRegistration3 {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskRegistration3";
}
#[cfg(feature = "Foundation")]
impl IBackgroundTaskRegistration3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTaskRegistration3_Impl, const OFFSET: isize>() -> IBackgroundTaskRegistration3_Vtbl {
        unsafe extern "system" fn TaskGroup<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTaskRegistration3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TaskGroup() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundTaskRegistration3, OFFSET>(), TaskGroup: TaskGroup::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTrigger_Impl, const OFFSET: isize>() -> IBackgroundTrigger_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundTrigger, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTrigger as ::windows::core::Interface>::IID
    }
}

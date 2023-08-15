#[doc = "*Required features: `\"ApplicationModel_Background\"`, `\"implement\"`*"]
pub trait IBackgroundCondition_Impl: Sized {}
impl ::windows_core::RuntimeName for IBackgroundCondition {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundCondition";
}
impl IBackgroundCondition_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundCondition_Impl, const OFFSET: isize>() -> IBackgroundCondition_Vtbl {
        Self { base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IBackgroundCondition, OFFSET>() }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IBackgroundCondition as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Background\"`, `\"implement\"`*"]
pub trait IBackgroundTask_Impl: Sized {
    fn Run(&self, taskinstance: ::core::option::Option<&IBackgroundTaskInstance>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IBackgroundTask {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTask";
}
impl IBackgroundTask_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTask_Impl, const OFFSET: isize>() -> IBackgroundTask_Vtbl {
        unsafe extern "system" fn Run<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskinstance: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Run(::windows_core::from_raw_borrowed(&taskinstance)).into()
        }
        Self { base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IBackgroundTask, OFFSET>(), Run: Run::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IBackgroundTask as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Background\"`, `\"Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Foundation")]
pub trait IBackgroundTaskInstance_Impl: Sized {
    fn InstanceId(&self) -> ::windows_core::Result<::windows_core::GUID>;
    fn Task(&self) -> ::windows_core::Result<BackgroundTaskRegistration>;
    fn Progress(&self) -> ::windows_core::Result<u32>;
    fn SetProgress(&self, value: u32) -> ::windows_core::Result<()>;
    fn TriggerDetails(&self) -> ::windows_core::Result<::windows_core::IInspectable>;
    fn Canceled(&self, cancelhandler: ::core::option::Option<&BackgroundTaskCanceledEventHandler>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCanceled(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn SuspendedCount(&self) -> ::windows_core::Result<u32>;
    fn GetDeferral(&self) -> ::windows_core::Result<BackgroundTaskDeferral>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::RuntimeName for IBackgroundTaskInstance {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskInstance";
}
#[cfg(feature = "Foundation")]
impl IBackgroundTaskInstance_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTaskInstance_Impl, const OFFSET: isize>() -> IBackgroundTaskInstance_Vtbl {
        unsafe extern "system" fn InstanceId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTaskInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InstanceId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Task<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTaskInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Task() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Progress<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTaskInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Progress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProgress<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTaskInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProgress(value).into()
        }
        unsafe extern "system" fn TriggerDetails<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTaskInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TriggerDetails() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Canceled<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTaskInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cancelhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Canceled(::windows_core::from_raw_borrowed(&cancelhandler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCanceled<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTaskInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveCanceled(::core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn SuspendedCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTaskInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SuspendedCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTaskInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IBackgroundTaskInstance, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IBackgroundTaskInstance as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Background\"`, `\"Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Foundation")]
pub trait IBackgroundTaskInstance2_Impl: Sized + IBackgroundTaskInstance_Impl {
    fn GetThrottleCount(&self, counter: BackgroundTaskThrottleCounter) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::RuntimeName for IBackgroundTaskInstance2 {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskInstance2";
}
#[cfg(feature = "Foundation")]
impl IBackgroundTaskInstance2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTaskInstance2_Impl, const OFFSET: isize>() -> IBackgroundTaskInstance2_Vtbl {
        unsafe extern "system" fn GetThrottleCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTaskInstance2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, counter: BackgroundTaskThrottleCounter, result__: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetThrottleCount(counter) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IBackgroundTaskInstance2, OFFSET>(),
            GetThrottleCount: GetThrottleCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IBackgroundTaskInstance2 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Background\"`, `\"Foundation\"`, `\"System\"`, `\"implement\"`*"]
#[cfg(all(feature = "Foundation", feature = "System"))]
pub trait IBackgroundTaskInstance4_Impl: Sized + IBackgroundTaskInstance_Impl {
    fn User(&self) -> ::windows_core::Result<super::super::System::User>;
}
#[cfg(all(feature = "Foundation", feature = "System"))]
impl ::windows_core::RuntimeName for IBackgroundTaskInstance4 {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskInstance4";
}
#[cfg(all(feature = "Foundation", feature = "System"))]
impl IBackgroundTaskInstance4_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTaskInstance4_Impl, const OFFSET: isize>() -> IBackgroundTaskInstance4_Vtbl {
        unsafe extern "system" fn User<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTaskInstance4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.User() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IBackgroundTaskInstance4, OFFSET>(), User: User::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IBackgroundTaskInstance4 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Background\"`, `\"Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Foundation")]
pub trait IBackgroundTaskRegistration_Impl: Sized {
    fn TaskId(&self) -> ::windows_core::Result<::windows_core::GUID>;
    fn Name(&self) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn Progress(&self, handler: ::core::option::Option<&BackgroundTaskProgressEventHandler>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveProgress(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn Completed(&self, handler: ::core::option::Option<&BackgroundTaskCompletedEventHandler>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCompleted(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn Unregister(&self, canceltask: bool) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::RuntimeName for IBackgroundTaskRegistration {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskRegistration";
}
#[cfg(feature = "Foundation")]
impl IBackgroundTaskRegistration_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTaskRegistration_Impl, const OFFSET: isize>() -> IBackgroundTaskRegistration_Vtbl {
        unsafe extern "system" fn TaskId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTaskRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TaskId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTaskRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Progress<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTaskRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Progress(::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveProgress<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTaskRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveProgress(::core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn Completed<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTaskRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Completed(::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCompleted<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTaskRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveCompleted(::core::mem::transmute(&cookie)).into()
        }
        unsafe extern "system" fn Unregister<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTaskRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, canceltask: bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unregister(canceltask).into()
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IBackgroundTaskRegistration, OFFSET>(),
            TaskId: TaskId::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            Progress: Progress::<Identity, Impl, OFFSET>,
            RemoveProgress: RemoveProgress::<Identity, Impl, OFFSET>,
            Completed: Completed::<Identity, Impl, OFFSET>,
            RemoveCompleted: RemoveCompleted::<Identity, Impl, OFFSET>,
            Unregister: Unregister::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IBackgroundTaskRegistration as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Background\"`, `\"Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Foundation")]
pub trait IBackgroundTaskRegistration2_Impl: Sized + IBackgroundTaskRegistration_Impl {
    fn Trigger(&self) -> ::windows_core::Result<IBackgroundTrigger>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::RuntimeName for IBackgroundTaskRegistration2 {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskRegistration2";
}
#[cfg(feature = "Foundation")]
impl IBackgroundTaskRegistration2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTaskRegistration2_Impl, const OFFSET: isize>() -> IBackgroundTaskRegistration2_Vtbl {
        unsafe extern "system" fn Trigger<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTaskRegistration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Trigger() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IBackgroundTaskRegistration2, OFFSET>(), Trigger: Trigger::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IBackgroundTaskRegistration2 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Background\"`, `\"Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Foundation")]
pub trait IBackgroundTaskRegistration3_Impl: Sized + IBackgroundTaskRegistration_Impl {
    fn TaskGroup(&self) -> ::windows_core::Result<BackgroundTaskRegistrationGroup>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::RuntimeName for IBackgroundTaskRegistration3 {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTaskRegistration3";
}
#[cfg(feature = "Foundation")]
impl IBackgroundTaskRegistration3_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTaskRegistration3_Impl, const OFFSET: isize>() -> IBackgroundTaskRegistration3_Vtbl {
        unsafe extern "system" fn TaskGroup<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTaskRegistration3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TaskGroup() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IBackgroundTaskRegistration3, OFFSET>(),
            TaskGroup: TaskGroup::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IBackgroundTaskRegistration3 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"ApplicationModel_Background\"`, `\"implement\"`*"]
pub trait IBackgroundTrigger_Impl: Sized {}
impl ::windows_core::RuntimeName for IBackgroundTrigger {
    const NAME: &'static str = "Windows.ApplicationModel.Background.IBackgroundTrigger";
}
impl IBackgroundTrigger_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBackgroundTrigger_Impl, const OFFSET: isize>() -> IBackgroundTrigger_Vtbl {
        Self { base__: ::windows_core::IInspectable_Vtbl::new::<Identity, IBackgroundTrigger, OFFSET>() }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IBackgroundTrigger as ::windows_core::ComInterface>::IID
    }
}

#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppInstallItem_Impl: Sized {
    fn ProductId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PackageFamilyName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn InstallType(&mut self) -> ::windows::core::Result<AppInstallType>;
    fn IsUserInitiated(&mut self) -> ::windows::core::Result<bool>;
    fn GetCurrentStatus(&mut self) -> ::windows::core::Result<AppInstallStatus>;
    fn Cancel(&mut self) -> ::windows::core::Result<()>;
    fn Pause(&mut self) -> ::windows::core::Result<()>;
    fn Restart(&mut self) -> ::windows::core::Result<()>;
    fn Completed(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<AppInstallItem, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCompleted(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StatusChanged(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<AppInstallItem, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveStatusChanged(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppInstallItem {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallItem";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppInstallItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppInstallItem_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppInstallItem_Vtbl {
        unsafe extern "system" fn ProductId<Impl: IAppInstallItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProductId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PackageFamilyName<Impl: IAppInstallItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PackageFamilyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallType<Impl: IAppInstallItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppInstallType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstallType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUserInitiated<Impl: IAppInstallItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsUserInitiated() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentStatus<Impl: IAppInstallItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Impl: IAppInstallItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cancel().into()
        }
        unsafe extern "system" fn Pause<Impl: IAppInstallItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn Restart<Impl: IAppInstallItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Restart().into()
        }
        unsafe extern "system" fn Completed<Impl: IAppInstallItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Completed(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<AppInstallItem, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<AppInstallItem, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCompleted<Impl: IAppInstallItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCompleted(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StatusChanged<Impl: IAppInstallItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatusChanged(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<AppInstallItem, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<AppInstallItem, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStatusChanged<Impl: IAppInstallItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStatusChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppInstallItem, BASE_OFFSET>(),
            ProductId: ProductId::<Impl, IMPL_OFFSET>,
            PackageFamilyName: PackageFamilyName::<Impl, IMPL_OFFSET>,
            InstallType: InstallType::<Impl, IMPL_OFFSET>,
            IsUserInitiated: IsUserInitiated::<Impl, IMPL_OFFSET>,
            GetCurrentStatus: GetCurrentStatus::<Impl, IMPL_OFFSET>,
            Cancel: Cancel::<Impl, IMPL_OFFSET>,
            Pause: Pause::<Impl, IMPL_OFFSET>,
            Restart: Restart::<Impl, IMPL_OFFSET>,
            Completed: Completed::<Impl, IMPL_OFFSET>,
            RemoveCompleted: RemoveCompleted::<Impl, IMPL_OFFSET>,
            StatusChanged: StatusChanged::<Impl, IMPL_OFFSET>,
            RemoveStatusChanged: RemoveStatusChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInstallItem as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInstallItem2_Impl: Sized {
    fn CancelWithTelemetry(&mut self, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PauseWithTelemetry(&mut self, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn RestartWithTelemetry(&mut self, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppInstallItem2 {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallItem2";
}
#[cfg(feature = "implement_exclusive")]
impl IAppInstallItem2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppInstallItem2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppInstallItem2_Vtbl {
        unsafe extern "system" fn CancelWithTelemetry<Impl: IAppInstallItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CancelWithTelemetry(&*(&correlationvector as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PauseWithTelemetry<Impl: IAppInstallItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PauseWithTelemetry(&*(&correlationvector as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RestartWithTelemetry<Impl: IAppInstallItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RestartWithTelemetry(&*(&correlationvector as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppInstallItem2, BASE_OFFSET>(),
            CancelWithTelemetry: CancelWithTelemetry::<Impl, IMPL_OFFSET>,
            PauseWithTelemetry: PauseWithTelemetry::<Impl, IMPL_OFFSET>,
            RestartWithTelemetry: RestartWithTelemetry::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInstallItem2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAppInstallItem3_Impl: Sized {
    fn Children(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>;
    fn ItemOperationsMightAffectOtherItems(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppInstallItem3 {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallItem3";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAppInstallItem3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppInstallItem3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppInstallItem3_Vtbl {
        unsafe extern "system" fn Children<Impl: IAppInstallItem3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Children() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemOperationsMightAffectOtherItems<Impl: IAppInstallItem3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemOperationsMightAffectOtherItems() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppInstallItem3, BASE_OFFSET>(),
            Children: Children::<Impl, IMPL_OFFSET>,
            ItemOperationsMightAffectOtherItems: ItemOperationsMightAffectOtherItems::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInstallItem3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInstallItem4_Impl: Sized {
    fn LaunchAfterInstall(&mut self) -> ::windows::core::Result<bool>;
    fn SetLaunchAfterInstall(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppInstallItem4 {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallItem4";
}
#[cfg(feature = "implement_exclusive")]
impl IAppInstallItem4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppInstallItem4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppInstallItem4_Vtbl {
        unsafe extern "system" fn LaunchAfterInstall<Impl: IAppInstallItem4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LaunchAfterInstall() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLaunchAfterInstall<Impl: IAppInstallItem4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLaunchAfterInstall(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppInstallItem4, BASE_OFFSET>(),
            LaunchAfterInstall: LaunchAfterInstall::<Impl, IMPL_OFFSET>,
            SetLaunchAfterInstall: SetLaunchAfterInstall::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInstallItem4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInstallItem5_Impl: Sized {
    fn PinToDesktopAfterInstall(&mut self) -> ::windows::core::Result<bool>;
    fn SetPinToDesktopAfterInstall(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn PinToStartAfterInstall(&mut self) -> ::windows::core::Result<bool>;
    fn SetPinToStartAfterInstall(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn PinToTaskbarAfterInstall(&mut self) -> ::windows::core::Result<bool>;
    fn SetPinToTaskbarAfterInstall(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn CompletedInstallToastNotificationMode(&mut self) -> ::windows::core::Result<AppInstallationToastNotificationMode>;
    fn SetCompletedInstallToastNotificationMode(&mut self, value: AppInstallationToastNotificationMode) -> ::windows::core::Result<()>;
    fn InstallInProgressToastNotificationMode(&mut self) -> ::windows::core::Result<AppInstallationToastNotificationMode>;
    fn SetInstallInProgressToastNotificationMode(&mut self, value: AppInstallationToastNotificationMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppInstallItem5 {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallItem5";
}
#[cfg(feature = "implement_exclusive")]
impl IAppInstallItem5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppInstallItem5_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppInstallItem5_Vtbl {
        unsafe extern "system" fn PinToDesktopAfterInstall<Impl: IAppInstallItem5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PinToDesktopAfterInstall() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPinToDesktopAfterInstall<Impl: IAppInstallItem5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPinToDesktopAfterInstall(value).into()
        }
        unsafe extern "system" fn PinToStartAfterInstall<Impl: IAppInstallItem5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PinToStartAfterInstall() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPinToStartAfterInstall<Impl: IAppInstallItem5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPinToStartAfterInstall(value).into()
        }
        unsafe extern "system" fn PinToTaskbarAfterInstall<Impl: IAppInstallItem5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PinToTaskbarAfterInstall() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPinToTaskbarAfterInstall<Impl: IAppInstallItem5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPinToTaskbarAfterInstall(value).into()
        }
        unsafe extern "system" fn CompletedInstallToastNotificationMode<Impl: IAppInstallItem5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppInstallationToastNotificationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompletedInstallToastNotificationMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompletedInstallToastNotificationMode<Impl: IAppInstallItem5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AppInstallationToastNotificationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCompletedInstallToastNotificationMode(value).into()
        }
        unsafe extern "system" fn InstallInProgressToastNotificationMode<Impl: IAppInstallItem5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppInstallationToastNotificationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstallInProgressToastNotificationMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInstallInProgressToastNotificationMode<Impl: IAppInstallItem5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AppInstallationToastNotificationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInstallInProgressToastNotificationMode(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppInstallItem5, BASE_OFFSET>(),
            PinToDesktopAfterInstall: PinToDesktopAfterInstall::<Impl, IMPL_OFFSET>,
            SetPinToDesktopAfterInstall: SetPinToDesktopAfterInstall::<Impl, IMPL_OFFSET>,
            PinToStartAfterInstall: PinToStartAfterInstall::<Impl, IMPL_OFFSET>,
            SetPinToStartAfterInstall: SetPinToStartAfterInstall::<Impl, IMPL_OFFSET>,
            PinToTaskbarAfterInstall: PinToTaskbarAfterInstall::<Impl, IMPL_OFFSET>,
            SetPinToTaskbarAfterInstall: SetPinToTaskbarAfterInstall::<Impl, IMPL_OFFSET>,
            CompletedInstallToastNotificationMode: CompletedInstallToastNotificationMode::<Impl, IMPL_OFFSET>,
            SetCompletedInstallToastNotificationMode: SetCompletedInstallToastNotificationMode::<Impl, IMPL_OFFSET>,
            InstallInProgressToastNotificationMode: InstallInProgressToastNotificationMode::<Impl, IMPL_OFFSET>,
            SetInstallInProgressToastNotificationMode: SetInstallInProgressToastNotificationMode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInstallItem5 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAppInstallManager_Impl: Sized {
    fn AppInstallItems(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>;
    fn Cancel(&mut self, productid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Pause(&mut self, productid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Restart(&mut self, productid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ItemCompleted(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<AppInstallManager, AppInstallManagerItemEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveItemCompleted(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ItemStatusChanged(&mut self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<AppInstallManager, AppInstallManagerItemEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveItemStatusChanged(&mut self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AutoUpdateSetting(&mut self) -> ::windows::core::Result<AutoUpdateSetting>;
    fn SetAutoUpdateSetting(&mut self, value: AutoUpdateSetting) -> ::windows::core::Result<()>;
    fn AcquisitionIdentity(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAcquisitionIdentity(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GetIsApplicableAsync(&mut self, productid: &::windows::core::HSTRING, skuid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn StartAppInstallAsync(&mut self, productid: &::windows::core::HSTRING, skuid: &::windows::core::HSTRING, repair: bool, forceuseofnonremovablestorage: bool) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>;
    fn UpdateAppByPackageFamilyNameAsync(&mut self, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>;
    fn SearchForUpdatesAsync(&mut self, productid: &::windows::core::HSTRING, skuid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>;
    fn SearchForAllUpdatesAsync(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>;
    fn IsStoreBlockedByPolicyAsync(&mut self, storeclientname: &::windows::core::HSTRING, storeclientpublisher: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn GetIsAppAllowedToInstallAsync(&mut self, productid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppInstallManager {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAppInstallManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppInstallManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppInstallManager_Vtbl {
        unsafe extern "system" fn AppInstallItems<Impl: IAppInstallManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppInstallItems() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Impl: IAppInstallManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cancel(&*(&productid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Pause<Impl: IAppInstallManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Pause(&*(&productid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Restart<Impl: IAppInstallManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Restart(&*(&productid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ItemCompleted<Impl: IAppInstallManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemCompleted(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<AppInstallManager, AppInstallManagerItemEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<AppInstallManager, AppInstallManagerItemEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveItemCompleted<Impl: IAppInstallManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveItemCompleted(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ItemStatusChanged<Impl: IAppInstallManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemStatusChanged(&*(&handler as *const <super::super::super::super::Foundation::TypedEventHandler<AppInstallManager, AppInstallManagerItemEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::TypedEventHandler<AppInstallManager, AppInstallManagerItemEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveItemStatusChanged<Impl: IAppInstallManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveItemStatusChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AutoUpdateSetting<Impl: IAppInstallManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AutoUpdateSetting) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoUpdateSetting() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoUpdateSetting<Impl: IAppInstallManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AutoUpdateSetting) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoUpdateSetting(value).into()
        }
        unsafe extern "system" fn AcquisitionIdentity<Impl: IAppInstallManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AcquisitionIdentity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAcquisitionIdentity<Impl: IAppInstallManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAcquisitionIdentity(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetIsApplicableAsync<Impl: IAppInstallManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsApplicableAsync(&*(&productid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&skuid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartAppInstallAsync<Impl: IAppInstallManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, repair: bool, forceuseofnonremovablestorage: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartAppInstallAsync(&*(&productid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&skuid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), repair, forceuseofnonremovablestorage) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateAppByPackageFamilyNameAsync<Impl: IAppInstallManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateAppByPackageFamilyNameAsync(&*(&packagefamilyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchForUpdatesAsync<Impl: IAppInstallManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchForUpdatesAsync(&*(&productid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&skuid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchForAllUpdatesAsync<Impl: IAppInstallManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchForAllUpdatesAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStoreBlockedByPolicyAsync<Impl: IAppInstallManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storeclientname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, storeclientpublisher: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsStoreBlockedByPolicyAsync(&*(&storeclientname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&storeclientpublisher as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsAppAllowedToInstallAsync<Impl: IAppInstallManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsAppAllowedToInstallAsync(&*(&productid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppInstallManager, BASE_OFFSET>(),
            AppInstallItems: AppInstallItems::<Impl, IMPL_OFFSET>,
            Cancel: Cancel::<Impl, IMPL_OFFSET>,
            Pause: Pause::<Impl, IMPL_OFFSET>,
            Restart: Restart::<Impl, IMPL_OFFSET>,
            ItemCompleted: ItemCompleted::<Impl, IMPL_OFFSET>,
            RemoveItemCompleted: RemoveItemCompleted::<Impl, IMPL_OFFSET>,
            ItemStatusChanged: ItemStatusChanged::<Impl, IMPL_OFFSET>,
            RemoveItemStatusChanged: RemoveItemStatusChanged::<Impl, IMPL_OFFSET>,
            AutoUpdateSetting: AutoUpdateSetting::<Impl, IMPL_OFFSET>,
            SetAutoUpdateSetting: SetAutoUpdateSetting::<Impl, IMPL_OFFSET>,
            AcquisitionIdentity: AcquisitionIdentity::<Impl, IMPL_OFFSET>,
            SetAcquisitionIdentity: SetAcquisitionIdentity::<Impl, IMPL_OFFSET>,
            GetIsApplicableAsync: GetIsApplicableAsync::<Impl, IMPL_OFFSET>,
            StartAppInstallAsync: StartAppInstallAsync::<Impl, IMPL_OFFSET>,
            UpdateAppByPackageFamilyNameAsync: UpdateAppByPackageFamilyNameAsync::<Impl, IMPL_OFFSET>,
            SearchForUpdatesAsync: SearchForUpdatesAsync::<Impl, IMPL_OFFSET>,
            SearchForAllUpdatesAsync: SearchForAllUpdatesAsync::<Impl, IMPL_OFFSET>,
            IsStoreBlockedByPolicyAsync: IsStoreBlockedByPolicyAsync::<Impl, IMPL_OFFSET>,
            GetIsAppAllowedToInstallAsync: GetIsAppAllowedToInstallAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInstallManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppInstallManager2_Impl: Sized {
    fn StartAppInstallWithTelemetryAsync(&mut self, productid: &::windows::core::HSTRING, skuid: &::windows::core::HSTRING, repair: bool, forceuseofnonremovablestorage: bool, catalogid: &::windows::core::HSTRING, bundleid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>;
    fn UpdateAppByPackageFamilyNameWithTelemetryAsync(&mut self, packagefamilyname: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>;
    fn SearchForUpdatesWithTelemetryAsync(&mut self, productid: &::windows::core::HSTRING, skuid: &::windows::core::HSTRING, catalogid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>;
    fn SearchForAllUpdatesWithTelemetryAsync(&mut self, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>;
    fn GetIsAppAllowedToInstallWithTelemetryAsync(&mut self, productid: &::windows::core::HSTRING, skuid: &::windows::core::HSTRING, catalogid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn CancelWithTelemetry(&mut self, productid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PauseWithTelemetry(&mut self, productid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn RestartWithTelemetry(&mut self, productid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppInstallManager2 {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppInstallManager2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppInstallManager2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppInstallManager2_Vtbl {
        unsafe extern "system" fn StartAppInstallWithTelemetryAsync<Impl: IAppInstallManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, repair: bool, forceuseofnonremovablestorage: bool, catalogid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, bundleid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartAppInstallWithTelemetryAsync(
                &*(&productid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&skuid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                repair,
                forceuseofnonremovablestorage,
                &*(&catalogid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&bundleid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&correlationvector as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateAppByPackageFamilyNameWithTelemetryAsync<Impl: IAppInstallManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateAppByPackageFamilyNameWithTelemetryAsync(&*(&packagefamilyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&correlationvector as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchForUpdatesWithTelemetryAsync<Impl: IAppInstallManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, catalogid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchForUpdatesWithTelemetryAsync(
                &*(&productid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&skuid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&catalogid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&correlationvector as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchForAllUpdatesWithTelemetryAsync<Impl: IAppInstallManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchForAllUpdatesWithTelemetryAsync(&*(&correlationvector as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsAppAllowedToInstallWithTelemetryAsync<Impl: IAppInstallManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, catalogid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsAppAllowedToInstallWithTelemetryAsync(
                &*(&productid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&skuid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&catalogid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&correlationvector as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelWithTelemetry<Impl: IAppInstallManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CancelWithTelemetry(&*(&productid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&correlationvector as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PauseWithTelemetry<Impl: IAppInstallManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PauseWithTelemetry(&*(&productid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&correlationvector as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RestartWithTelemetry<Impl: IAppInstallManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RestartWithTelemetry(&*(&productid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&correlationvector as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppInstallManager2, BASE_OFFSET>(),
            StartAppInstallWithTelemetryAsync: StartAppInstallWithTelemetryAsync::<Impl, IMPL_OFFSET>,
            UpdateAppByPackageFamilyNameWithTelemetryAsync: UpdateAppByPackageFamilyNameWithTelemetryAsync::<Impl, IMPL_OFFSET>,
            SearchForUpdatesWithTelemetryAsync: SearchForUpdatesWithTelemetryAsync::<Impl, IMPL_OFFSET>,
            SearchForAllUpdatesWithTelemetryAsync: SearchForAllUpdatesWithTelemetryAsync::<Impl, IMPL_OFFSET>,
            GetIsAppAllowedToInstallWithTelemetryAsync: GetIsAppAllowedToInstallWithTelemetryAsync::<Impl, IMPL_OFFSET>,
            CancelWithTelemetry: CancelWithTelemetry::<Impl, IMPL_OFFSET>,
            PauseWithTelemetry: PauseWithTelemetry::<Impl, IMPL_OFFSET>,
            RestartWithTelemetry: RestartWithTelemetry::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInstallManager2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Management_Deployment", feature = "System", feature = "implement_exclusive"))]
pub trait IAppInstallManager3_Impl: Sized {
    fn StartProductInstallAsync(&mut self, productid: &::windows::core::HSTRING, catalogid: &::windows::core::HSTRING, flightid: &::windows::core::HSTRING, clientid: &::windows::core::HSTRING, repair: bool, forceuseofnonremovablestorage: bool, correlationvector: &::windows::core::HSTRING, targetvolume: &::core::option::Option<super::super::super::super::Management::Deployment::PackageVolume>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>;
    fn StartProductInstallForUserAsync(&mut self, user: &::core::option::Option<super::super::super::super::System::User>, productid: &::windows::core::HSTRING, catalogid: &::windows::core::HSTRING, flightid: &::windows::core::HSTRING, clientid: &::windows::core::HSTRING, repair: bool, forceuseofnonremovablestorage: bool, correlationvector: &::windows::core::HSTRING, targetvolume: &::core::option::Option<super::super::super::super::Management::Deployment::PackageVolume>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>;
    fn UpdateAppByPackageFamilyNameForUserAsync(&mut self, user: &::core::option::Option<super::super::super::super::System::User>, packagefamilyname: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>;
    fn SearchForUpdatesForUserAsync(&mut self, user: &::core::option::Option<super::super::super::super::System::User>, productid: &::windows::core::HSTRING, skuid: &::windows::core::HSTRING, catalogid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>;
    fn SearchForAllUpdatesForUserAsync(&mut self, user: &::core::option::Option<super::super::super::super::System::User>, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>;
    fn GetIsAppAllowedToInstallForUserAsync(&mut self, user: &::core::option::Option<super::super::super::super::System::User>, productid: &::windows::core::HSTRING, skuid: &::windows::core::HSTRING, catalogid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn GetIsApplicableForUserAsync(&mut self, user: &::core::option::Option<super::super::super::super::System::User>, productid: &::windows::core::HSTRING, skuid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn MoveToFrontOfDownloadQueue(&mut self, productid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Management_Deployment", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppInstallManager3 {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager3";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Management_Deployment", feature = "System", feature = "implement_exclusive"))]
impl IAppInstallManager3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppInstallManager3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppInstallManager3_Vtbl {
        unsafe extern "system" fn StartProductInstallAsync<Impl: IAppInstallManager3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, catalogid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, flightid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, clientid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, repair: bool, forceuseofnonremovablestorage: bool, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, targetvolume: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartProductInstallAsync(
                &*(&productid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&catalogid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&flightid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&clientid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                repair,
                forceuseofnonremovablestorage,
                &*(&correlationvector as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&targetvolume as *const <super::super::super::super::Management::Deployment::PackageVolume as ::windows::core::Abi>::Abi as *const <super::super::super::super::Management::Deployment::PackageVolume as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartProductInstallForUserAsync<Impl: IAppInstallManager3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, catalogid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, flightid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, clientid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, repair: bool, forceuseofnonremovablestorage: bool, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, targetvolume: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartProductInstallForUserAsync(
                &*(&user as *const <super::super::super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::super::super::System::User as ::windows::core::DefaultType>::DefaultType),
                &*(&productid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&catalogid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&flightid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&clientid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                repair,
                forceuseofnonremovablestorage,
                &*(&correlationvector as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&targetvolume as *const <super::super::super::super::Management::Deployment::PackageVolume as ::windows::core::Abi>::Abi as *const <super::super::super::super::Management::Deployment::PackageVolume as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateAppByPackageFamilyNameForUserAsync<Impl: IAppInstallManager3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateAppByPackageFamilyNameForUserAsync(
                &*(&user as *const <super::super::super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::super::super::System::User as ::windows::core::DefaultType>::DefaultType),
                &*(&packagefamilyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&correlationvector as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchForUpdatesForUserAsync<Impl: IAppInstallManager3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, catalogid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchForUpdatesForUserAsync(
                &*(&user as *const <super::super::super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::super::super::System::User as ::windows::core::DefaultType>::DefaultType),
                &*(&productid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&skuid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&catalogid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&correlationvector as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchForAllUpdatesForUserAsync<Impl: IAppInstallManager3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchForAllUpdatesForUserAsync(&*(&user as *const <super::super::super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::super::super::System::User as ::windows::core::DefaultType>::DefaultType), &*(&correlationvector as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsAppAllowedToInstallForUserAsync<Impl: IAppInstallManager3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, catalogid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsAppAllowedToInstallForUserAsync(
                &*(&user as *const <super::super::super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::super::super::System::User as ::windows::core::DefaultType>::DefaultType),
                &*(&productid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&skuid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&catalogid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&correlationvector as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsApplicableForUserAsync<Impl: IAppInstallManager3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsApplicableForUserAsync(
                &*(&user as *const <super::super::super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::super::super::System::User as ::windows::core::DefaultType>::DefaultType),
                &*(&productid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&skuid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveToFrontOfDownloadQueue<Impl: IAppInstallManager3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MoveToFrontOfDownloadQueue(&*(&productid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&correlationvector as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppInstallManager3, BASE_OFFSET>(),
            StartProductInstallAsync: StartProductInstallAsync::<Impl, IMPL_OFFSET>,
            StartProductInstallForUserAsync: StartProductInstallForUserAsync::<Impl, IMPL_OFFSET>,
            UpdateAppByPackageFamilyNameForUserAsync: UpdateAppByPackageFamilyNameForUserAsync::<Impl, IMPL_OFFSET>,
            SearchForUpdatesForUserAsync: SearchForUpdatesForUserAsync::<Impl, IMPL_OFFSET>,
            SearchForAllUpdatesForUserAsync: SearchForAllUpdatesForUserAsync::<Impl, IMPL_OFFSET>,
            GetIsAppAllowedToInstallForUserAsync: GetIsAppAllowedToInstallForUserAsync::<Impl, IMPL_OFFSET>,
            GetIsApplicableForUserAsync: GetIsApplicableForUserAsync::<Impl, IMPL_OFFSET>,
            MoveToFrontOfDownloadQueue: MoveToFrontOfDownloadQueue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInstallManager3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
pub trait IAppInstallManager4_Impl: Sized {
    fn GetFreeUserEntitlementAsync(&mut self, storeid: &::windows::core::HSTRING, campaignid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<GetEntitlementResult>>;
    fn GetFreeUserEntitlementForUserAsync(&mut self, user: &::core::option::Option<super::super::super::super::System::User>, storeid: &::windows::core::HSTRING, campaignid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<GetEntitlementResult>>;
    fn GetFreeDeviceEntitlementAsync(&mut self, storeid: &::windows::core::HSTRING, campaignid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<GetEntitlementResult>>;
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppInstallManager4 {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager4";
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl IAppInstallManager4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppInstallManager4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppInstallManager4_Vtbl {
        unsafe extern "system" fn GetFreeUserEntitlementAsync<Impl: IAppInstallManager4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storeid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, campaignid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFreeUserEntitlementAsync(
                &*(&storeid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&campaignid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&correlationvector as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFreeUserEntitlementForUserAsync<Impl: IAppInstallManager4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, storeid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, campaignid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFreeUserEntitlementForUserAsync(
                &*(&user as *const <super::super::super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::super::super::System::User as ::windows::core::DefaultType>::DefaultType),
                &*(&storeid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&campaignid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&correlationvector as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFreeDeviceEntitlementAsync<Impl: IAppInstallManager4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storeid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, campaignid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFreeDeviceEntitlementAsync(
                &*(&storeid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&campaignid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&correlationvector as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppInstallManager4, BASE_OFFSET>(),
            GetFreeUserEntitlementAsync: GetFreeUserEntitlementAsync::<Impl, IMPL_OFFSET>,
            GetFreeUserEntitlementForUserAsync: GetFreeUserEntitlementForUserAsync::<Impl, IMPL_OFFSET>,
            GetFreeDeviceEntitlementAsync: GetFreeDeviceEntitlementAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInstallManager4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAppInstallManager5_Impl: Sized {
    fn AppInstallItemsWithGroupSupport(&mut self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppInstallManager5 {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager5";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAppInstallManager5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppInstallManager5_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppInstallManager5_Vtbl {
        unsafe extern "system" fn AppInstallItemsWithGroupSupport<Impl: IAppInstallManager5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppInstallItemsWithGroupSupport() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppInstallManager5, BASE_OFFSET>(),
            AppInstallItemsWithGroupSupport: AppInstallItemsWithGroupSupport::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInstallManager5 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System", feature = "implement_exclusive"))]
pub trait IAppInstallManager6_Impl: Sized {
    fn SearchForAllUpdatesWithUpdateOptionsAsync(&mut self, correlationvector: &::windows::core::HSTRING, clientid: &::windows::core::HSTRING, updateoptions: &::core::option::Option<AppUpdateOptions>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>;
    fn SearchForAllUpdatesWithUpdateOptionsForUserAsync(&mut self, user: &::core::option::Option<super::super::super::super::System::User>, correlationvector: &::windows::core::HSTRING, clientid: &::windows::core::HSTRING, updateoptions: &::core::option::Option<AppUpdateOptions>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>;
    fn SearchForUpdatesWithUpdateOptionsAsync(&mut self, productid: &::windows::core::HSTRING, skuid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING, clientid: &::windows::core::HSTRING, updateoptions: &::core::option::Option<AppUpdateOptions>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>;
    fn SearchForUpdatesWithUpdateOptionsForUserAsync(&mut self, user: &::core::option::Option<super::super::super::super::System::User>, productid: &::windows::core::HSTRING, skuid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING, clientid: &::windows::core::HSTRING, updateoptions: &::core::option::Option<AppUpdateOptions>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>;
    fn StartProductInstallWithOptionsAsync(&mut self, productid: &::windows::core::HSTRING, flightid: &::windows::core::HSTRING, clientid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING, installoptions: &::core::option::Option<AppInstallOptions>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>;
    fn StartProductInstallWithOptionsForUserAsync(&mut self, user: &::core::option::Option<super::super::super::super::System::User>, productid: &::windows::core::HSTRING, flightid: &::windows::core::HSTRING, clientid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING, installoptions: &::core::option::Option<AppInstallOptions>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>;
    fn GetIsPackageIdentityAllowedToInstallAsync(&mut self, correlationvector: &::windows::core::HSTRING, packageidentityname: &::windows::core::HSTRING, publishercertificatename: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn GetIsPackageIdentityAllowedToInstallForUserAsync(&mut self, user: &::core::option::Option<super::super::super::super::System::User>, correlationvector: &::windows::core::HSTRING, packageidentityname: &::windows::core::HSTRING, publishercertificatename: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppInstallManager6 {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager6";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System", feature = "implement_exclusive"))]
impl IAppInstallManager6_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppInstallManager6_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppInstallManager6_Vtbl {
        unsafe extern "system" fn SearchForAllUpdatesWithUpdateOptionsAsync<Impl: IAppInstallManager6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, clientid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, updateoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchForAllUpdatesWithUpdateOptionsAsync(
                &*(&correlationvector as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&clientid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&updateoptions as *const <AppUpdateOptions as ::windows::core::Abi>::Abi as *const <AppUpdateOptions as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchForAllUpdatesWithUpdateOptionsForUserAsync<Impl: IAppInstallManager6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, clientid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, updateoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchForAllUpdatesWithUpdateOptionsForUserAsync(
                &*(&user as *const <super::super::super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::super::super::System::User as ::windows::core::DefaultType>::DefaultType),
                &*(&correlationvector as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&clientid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&updateoptions as *const <AppUpdateOptions as ::windows::core::Abi>::Abi as *const <AppUpdateOptions as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchForUpdatesWithUpdateOptionsAsync<Impl: IAppInstallManager6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, clientid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, updateoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchForUpdatesWithUpdateOptionsAsync(
                &*(&productid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&skuid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&correlationvector as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&clientid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&updateoptions as *const <AppUpdateOptions as ::windows::core::Abi>::Abi as *const <AppUpdateOptions as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchForUpdatesWithUpdateOptionsForUserAsync<Impl: IAppInstallManager6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, clientid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, updateoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchForUpdatesWithUpdateOptionsForUserAsync(
                &*(&user as *const <super::super::super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::super::super::System::User as ::windows::core::DefaultType>::DefaultType),
                &*(&productid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&skuid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&correlationvector as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&clientid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&updateoptions as *const <AppUpdateOptions as ::windows::core::Abi>::Abi as *const <AppUpdateOptions as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartProductInstallWithOptionsAsync<Impl: IAppInstallManager6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, flightid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, clientid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, installoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartProductInstallWithOptionsAsync(
                &*(&productid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&flightid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&clientid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&correlationvector as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&installoptions as *const <AppInstallOptions as ::windows::core::Abi>::Abi as *const <AppInstallOptions as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartProductInstallWithOptionsForUserAsync<Impl: IAppInstallManager6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, flightid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, clientid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, installoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartProductInstallWithOptionsForUserAsync(
                &*(&user as *const <super::super::super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::super::super::System::User as ::windows::core::DefaultType>::DefaultType),
                &*(&productid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&flightid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&clientid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&correlationvector as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&installoptions as *const <AppInstallOptions as ::windows::core::Abi>::Abi as *const <AppInstallOptions as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsPackageIdentityAllowedToInstallAsync<Impl: IAppInstallManager6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packageidentityname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, publishercertificatename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsPackageIdentityAllowedToInstallAsync(
                &*(&correlationvector as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&packageidentityname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&publishercertificatename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsPackageIdentityAllowedToInstallForUserAsync<Impl: IAppInstallManager6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packageidentityname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, publishercertificatename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsPackageIdentityAllowedToInstallForUserAsync(
                &*(&user as *const <super::super::super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::super::super::System::User as ::windows::core::DefaultType>::DefaultType),
                &*(&correlationvector as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&packageidentityname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&publishercertificatename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppInstallManager6, BASE_OFFSET>(),
            SearchForAllUpdatesWithUpdateOptionsAsync: SearchForAllUpdatesWithUpdateOptionsAsync::<Impl, IMPL_OFFSET>,
            SearchForAllUpdatesWithUpdateOptionsForUserAsync: SearchForAllUpdatesWithUpdateOptionsForUserAsync::<Impl, IMPL_OFFSET>,
            SearchForUpdatesWithUpdateOptionsAsync: SearchForUpdatesWithUpdateOptionsAsync::<Impl, IMPL_OFFSET>,
            SearchForUpdatesWithUpdateOptionsForUserAsync: SearchForUpdatesWithUpdateOptionsForUserAsync::<Impl, IMPL_OFFSET>,
            StartProductInstallWithOptionsAsync: StartProductInstallWithOptionsAsync::<Impl, IMPL_OFFSET>,
            StartProductInstallWithOptionsForUserAsync: StartProductInstallWithOptionsForUserAsync::<Impl, IMPL_OFFSET>,
            GetIsPackageIdentityAllowedToInstallAsync: GetIsPackageIdentityAllowedToInstallAsync::<Impl, IMPL_OFFSET>,
            GetIsPackageIdentityAllowedToInstallForUserAsync: GetIsPackageIdentityAllowedToInstallForUserAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInstallManager6 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInstallManager7_Impl: Sized {
    fn CanInstallForAllUsers(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppInstallManager7 {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager7";
}
#[cfg(feature = "implement_exclusive")]
impl IAppInstallManager7_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppInstallManager7_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppInstallManager7_Vtbl {
        unsafe extern "system" fn CanInstallForAllUsers<Impl: IAppInstallManager7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanInstallForAllUsers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppInstallManager7, BASE_OFFSET>(),
            CanInstallForAllUsers: CanInstallForAllUsers::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInstallManager7 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInstallManagerItemEventArgs_Impl: Sized {
    fn Item(&mut self) -> ::windows::core::Result<AppInstallItem>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppInstallManagerItemEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManagerItemEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAppInstallManagerItemEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppInstallManagerItemEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppInstallManagerItemEventArgs_Vtbl {
        unsafe extern "system" fn Item<Impl: IAppInstallManagerItemEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAppInstallManagerItemEventArgs, BASE_OFFSET>(), Item: Item::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInstallManagerItemEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Management_Deployment", feature = "implement_exclusive"))]
pub trait IAppInstallOptions_Impl: Sized {
    fn CatalogId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCatalogId(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ForceUseOfNonRemovableStorage(&mut self) -> ::windows::core::Result<bool>;
    fn SetForceUseOfNonRemovableStorage(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn AllowForcedAppRestart(&mut self) -> ::windows::core::Result<bool>;
    fn SetAllowForcedAppRestart(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Repair(&mut self) -> ::windows::core::Result<bool>;
    fn SetRepair(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn TargetVolume(&mut self) -> ::windows::core::Result<super::super::super::super::Management::Deployment::PackageVolume>;
    fn SetTargetVolume(&mut self, value: &::core::option::Option<super::super::super::super::Management::Deployment::PackageVolume>) -> ::windows::core::Result<()>;
    fn LaunchAfterInstall(&mut self) -> ::windows::core::Result<bool>;
    fn SetLaunchAfterInstall(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Management_Deployment", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppInstallOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallOptions";
}
#[cfg(all(feature = "Management_Deployment", feature = "implement_exclusive"))]
impl IAppInstallOptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppInstallOptions_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppInstallOptions_Vtbl {
        unsafe extern "system" fn CatalogId<Impl: IAppInstallOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CatalogId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCatalogId<Impl: IAppInstallOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCatalogId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ForceUseOfNonRemovableStorage<Impl: IAppInstallOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ForceUseOfNonRemovableStorage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForceUseOfNonRemovableStorage<Impl: IAppInstallOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetForceUseOfNonRemovableStorage(value).into()
        }
        unsafe extern "system" fn AllowForcedAppRestart<Impl: IAppInstallOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowForcedAppRestart() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowForcedAppRestart<Impl: IAppInstallOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowForcedAppRestart(value).into()
        }
        unsafe extern "system" fn Repair<Impl: IAppInstallOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Repair() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRepair<Impl: IAppInstallOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRepair(value).into()
        }
        unsafe extern "system" fn TargetVolume<Impl: IAppInstallOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetVolume() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetVolume<Impl: IAppInstallOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetVolume(&*(&value as *const <super::super::super::super::Management::Deployment::PackageVolume as ::windows::core::Abi>::Abi as *const <super::super::super::super::Management::Deployment::PackageVolume as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LaunchAfterInstall<Impl: IAppInstallOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LaunchAfterInstall() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLaunchAfterInstall<Impl: IAppInstallOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLaunchAfterInstall(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppInstallOptions, BASE_OFFSET>(),
            CatalogId: CatalogId::<Impl, IMPL_OFFSET>,
            SetCatalogId: SetCatalogId::<Impl, IMPL_OFFSET>,
            ForceUseOfNonRemovableStorage: ForceUseOfNonRemovableStorage::<Impl, IMPL_OFFSET>,
            SetForceUseOfNonRemovableStorage: SetForceUseOfNonRemovableStorage::<Impl, IMPL_OFFSET>,
            AllowForcedAppRestart: AllowForcedAppRestart::<Impl, IMPL_OFFSET>,
            SetAllowForcedAppRestart: SetAllowForcedAppRestart::<Impl, IMPL_OFFSET>,
            Repair: Repair::<Impl, IMPL_OFFSET>,
            SetRepair: SetRepair::<Impl, IMPL_OFFSET>,
            TargetVolume: TargetVolume::<Impl, IMPL_OFFSET>,
            SetTargetVolume: SetTargetVolume::<Impl, IMPL_OFFSET>,
            LaunchAfterInstall: LaunchAfterInstall::<Impl, IMPL_OFFSET>,
            SetLaunchAfterInstall: SetLaunchAfterInstall::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInstallOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInstallOptions2_Impl: Sized {
    fn PinToDesktopAfterInstall(&mut self) -> ::windows::core::Result<bool>;
    fn SetPinToDesktopAfterInstall(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn PinToStartAfterInstall(&mut self) -> ::windows::core::Result<bool>;
    fn SetPinToStartAfterInstall(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn PinToTaskbarAfterInstall(&mut self) -> ::windows::core::Result<bool>;
    fn SetPinToTaskbarAfterInstall(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn CompletedInstallToastNotificationMode(&mut self) -> ::windows::core::Result<AppInstallationToastNotificationMode>;
    fn SetCompletedInstallToastNotificationMode(&mut self, value: AppInstallationToastNotificationMode) -> ::windows::core::Result<()>;
    fn InstallInProgressToastNotificationMode(&mut self) -> ::windows::core::Result<AppInstallationToastNotificationMode>;
    fn SetInstallInProgressToastNotificationMode(&mut self, value: AppInstallationToastNotificationMode) -> ::windows::core::Result<()>;
    fn InstallForAllUsers(&mut self) -> ::windows::core::Result<bool>;
    fn SetInstallForAllUsers(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn StageButDoNotInstall(&mut self) -> ::windows::core::Result<bool>;
    fn SetStageButDoNotInstall(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn CampaignId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCampaignId(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ExtendedCampaignId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetExtendedCampaignId(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppInstallOptions2 {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallOptions2";
}
#[cfg(feature = "implement_exclusive")]
impl IAppInstallOptions2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppInstallOptions2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppInstallOptions2_Vtbl {
        unsafe extern "system" fn PinToDesktopAfterInstall<Impl: IAppInstallOptions2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PinToDesktopAfterInstall() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPinToDesktopAfterInstall<Impl: IAppInstallOptions2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPinToDesktopAfterInstall(value).into()
        }
        unsafe extern "system" fn PinToStartAfterInstall<Impl: IAppInstallOptions2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PinToStartAfterInstall() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPinToStartAfterInstall<Impl: IAppInstallOptions2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPinToStartAfterInstall(value).into()
        }
        unsafe extern "system" fn PinToTaskbarAfterInstall<Impl: IAppInstallOptions2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PinToTaskbarAfterInstall() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPinToTaskbarAfterInstall<Impl: IAppInstallOptions2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPinToTaskbarAfterInstall(value).into()
        }
        unsafe extern "system" fn CompletedInstallToastNotificationMode<Impl: IAppInstallOptions2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppInstallationToastNotificationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CompletedInstallToastNotificationMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompletedInstallToastNotificationMode<Impl: IAppInstallOptions2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AppInstallationToastNotificationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCompletedInstallToastNotificationMode(value).into()
        }
        unsafe extern "system" fn InstallInProgressToastNotificationMode<Impl: IAppInstallOptions2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppInstallationToastNotificationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstallInProgressToastNotificationMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInstallInProgressToastNotificationMode<Impl: IAppInstallOptions2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AppInstallationToastNotificationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInstallInProgressToastNotificationMode(value).into()
        }
        unsafe extern "system" fn InstallForAllUsers<Impl: IAppInstallOptions2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstallForAllUsers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInstallForAllUsers<Impl: IAppInstallOptions2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInstallForAllUsers(value).into()
        }
        unsafe extern "system" fn StageButDoNotInstall<Impl: IAppInstallOptions2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StageButDoNotInstall() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStageButDoNotInstall<Impl: IAppInstallOptions2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStageButDoNotInstall(value).into()
        }
        unsafe extern "system" fn CampaignId<Impl: IAppInstallOptions2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CampaignId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCampaignId<Impl: IAppInstallOptions2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCampaignId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ExtendedCampaignId<Impl: IAppInstallOptions2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedCampaignId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtendedCampaignId<Impl: IAppInstallOptions2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExtendedCampaignId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppInstallOptions2, BASE_OFFSET>(),
            PinToDesktopAfterInstall: PinToDesktopAfterInstall::<Impl, IMPL_OFFSET>,
            SetPinToDesktopAfterInstall: SetPinToDesktopAfterInstall::<Impl, IMPL_OFFSET>,
            PinToStartAfterInstall: PinToStartAfterInstall::<Impl, IMPL_OFFSET>,
            SetPinToStartAfterInstall: SetPinToStartAfterInstall::<Impl, IMPL_OFFSET>,
            PinToTaskbarAfterInstall: PinToTaskbarAfterInstall::<Impl, IMPL_OFFSET>,
            SetPinToTaskbarAfterInstall: SetPinToTaskbarAfterInstall::<Impl, IMPL_OFFSET>,
            CompletedInstallToastNotificationMode: CompletedInstallToastNotificationMode::<Impl, IMPL_OFFSET>,
            SetCompletedInstallToastNotificationMode: SetCompletedInstallToastNotificationMode::<Impl, IMPL_OFFSET>,
            InstallInProgressToastNotificationMode: InstallInProgressToastNotificationMode::<Impl, IMPL_OFFSET>,
            SetInstallInProgressToastNotificationMode: SetInstallInProgressToastNotificationMode::<Impl, IMPL_OFFSET>,
            InstallForAllUsers: InstallForAllUsers::<Impl, IMPL_OFFSET>,
            SetInstallForAllUsers: SetInstallForAllUsers::<Impl, IMPL_OFFSET>,
            StageButDoNotInstall: StageButDoNotInstall::<Impl, IMPL_OFFSET>,
            SetStageButDoNotInstall: SetStageButDoNotInstall::<Impl, IMPL_OFFSET>,
            CampaignId: CampaignId::<Impl, IMPL_OFFSET>,
            SetCampaignId: SetCampaignId::<Impl, IMPL_OFFSET>,
            ExtendedCampaignId: ExtendedCampaignId::<Impl, IMPL_OFFSET>,
            SetExtendedCampaignId: SetExtendedCampaignId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInstallOptions2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInstallStatus_Impl: Sized {
    fn InstallState(&mut self) -> ::windows::core::Result<AppInstallState>;
    fn DownloadSizeInBytes(&mut self) -> ::windows::core::Result<u64>;
    fn BytesDownloaded(&mut self) -> ::windows::core::Result<u64>;
    fn PercentComplete(&mut self) -> ::windows::core::Result<f64>;
    fn ErrorCode(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppInstallStatus {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallStatus";
}
#[cfg(feature = "implement_exclusive")]
impl IAppInstallStatus_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppInstallStatus_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppInstallStatus_Vtbl {
        unsafe extern "system" fn InstallState<Impl: IAppInstallStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppInstallState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstallState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DownloadSizeInBytes<Impl: IAppInstallStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadSizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BytesDownloaded<Impl: IAppInstallStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BytesDownloaded() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PercentComplete<Impl: IAppInstallStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PercentComplete() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ErrorCode<Impl: IAppInstallStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ErrorCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppInstallStatus, BASE_OFFSET>(),
            InstallState: InstallState::<Impl, IMPL_OFFSET>,
            DownloadSizeInBytes: DownloadSizeInBytes::<Impl, IMPL_OFFSET>,
            BytesDownloaded: BytesDownloaded::<Impl, IMPL_OFFSET>,
            PercentComplete: PercentComplete::<Impl, IMPL_OFFSET>,
            ErrorCode: ErrorCode::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInstallStatus as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait IAppInstallStatus2_Impl: Sized {
    fn User(&mut self) -> ::windows::core::Result<super::super::super::super::System::User>;
    fn ReadyForLaunch(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppInstallStatus2 {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallStatus2";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl IAppInstallStatus2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppInstallStatus2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppInstallStatus2_Vtbl {
        unsafe extern "system" fn User<Impl: IAppInstallStatus2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReadyForLaunch<Impl: IAppInstallStatus2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadyForLaunch() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppInstallStatus2, BASE_OFFSET>(),
            User: User::<Impl, IMPL_OFFSET>,
            ReadyForLaunch: ReadyForLaunch::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInstallStatus2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInstallStatus3_Impl: Sized {
    fn IsStaged(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppInstallStatus3 {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallStatus3";
}
#[cfg(feature = "implement_exclusive")]
impl IAppInstallStatus3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppInstallStatus3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppInstallStatus3_Vtbl {
        unsafe extern "system" fn IsStaged<Impl: IAppInstallStatus3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsStaged() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAppInstallStatus3, BASE_OFFSET>(), IsStaged: IsStaged::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInstallStatus3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppUpdateOptions_Impl: Sized {
    fn CatalogId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCatalogId(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AllowForcedAppRestart(&mut self) -> ::windows::core::Result<bool>;
    fn SetAllowForcedAppRestart(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppUpdateOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.IAppUpdateOptions";
}
#[cfg(feature = "implement_exclusive")]
impl IAppUpdateOptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppUpdateOptions_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppUpdateOptions_Vtbl {
        unsafe extern "system" fn CatalogId<Impl: IAppUpdateOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CatalogId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCatalogId<Impl: IAppUpdateOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCatalogId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AllowForcedAppRestart<Impl: IAppUpdateOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowForcedAppRestart() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowForcedAppRestart<Impl: IAppUpdateOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowForcedAppRestart(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppUpdateOptions, BASE_OFFSET>(),
            CatalogId: CatalogId::<Impl, IMPL_OFFSET>,
            SetCatalogId: SetCatalogId::<Impl, IMPL_OFFSET>,
            AllowForcedAppRestart: AllowForcedAppRestart::<Impl, IMPL_OFFSET>,
            SetAllowForcedAppRestart: SetAllowForcedAppRestart::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppUpdateOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppUpdateOptions2_Impl: Sized {
    fn AutomaticallyDownloadAndInstallUpdateIfFound(&mut self) -> ::windows::core::Result<bool>;
    fn SetAutomaticallyDownloadAndInstallUpdateIfFound(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppUpdateOptions2 {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.IAppUpdateOptions2";
}
#[cfg(feature = "implement_exclusive")]
impl IAppUpdateOptions2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppUpdateOptions2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppUpdateOptions2_Vtbl {
        unsafe extern "system" fn AutomaticallyDownloadAndInstallUpdateIfFound<Impl: IAppUpdateOptions2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutomaticallyDownloadAndInstallUpdateIfFound() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutomaticallyDownloadAndInstallUpdateIfFound<Impl: IAppUpdateOptions2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutomaticallyDownloadAndInstallUpdateIfFound(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppUpdateOptions2, BASE_OFFSET>(),
            AutomaticallyDownloadAndInstallUpdateIfFound: AutomaticallyDownloadAndInstallUpdateIfFound::<Impl, IMPL_OFFSET>,
            SetAutomaticallyDownloadAndInstallUpdateIfFound: SetAutomaticallyDownloadAndInstallUpdateIfFound::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppUpdateOptions2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGetEntitlementResult_Impl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<GetEntitlementStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGetEntitlementResult {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.IGetEntitlementResult";
}
#[cfg(feature = "implement_exclusive")]
impl IGetEntitlementResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGetEntitlementResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGetEntitlementResult_Vtbl {
        unsafe extern "system" fn Status<Impl: IGetEntitlementResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GetEntitlementStatus) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IGetEntitlementResult, BASE_OFFSET>(), Status: Status::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGetEntitlementResult as ::windows::core::Interface>::IID
    }
}

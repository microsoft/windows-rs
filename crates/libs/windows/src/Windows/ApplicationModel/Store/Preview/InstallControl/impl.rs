#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppInstallItemImpl: Sized {
    fn ProductId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn InstallType(&self) -> ::windows::core::Result<AppInstallType>;
    fn IsUserInitiated(&self) -> ::windows::core::Result<bool>;
    fn GetCurrentStatus(&self) -> ::windows::core::Result<AppInstallStatus>;
    fn Cancel(&self) -> ::windows::core::Result<()>;
    fn Pause(&self) -> ::windows::core::Result<()>;
    fn Restart(&self) -> ::windows::core::Result<()>;
    fn Completed(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<AppInstallItem, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCompleted(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StatusChanged(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<AppInstallItem, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveStatusChanged(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppInstallItem {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallItem";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppInstallItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppInstallItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppInstallItemVtbl {
        unsafe extern "system" fn ProductId<Impl: IAppInstallItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PackageFamilyName<Impl: IAppInstallItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn InstallType<Impl: IAppInstallItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppInstallType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsUserInitiated<Impl: IAppInstallItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetCurrentStatus<Impl: IAppInstallItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Cancel<Impl: IAppInstallItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cancel().into()
        }
        unsafe extern "system" fn Pause<Impl: IAppInstallItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn Restart<Impl: IAppInstallItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Restart().into()
        }
        unsafe extern "system" fn Completed<Impl: IAppInstallItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveCompleted<Impl: IAppInstallItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCompleted(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StatusChanged<Impl: IAppInstallItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveStatusChanged<Impl: IAppInstallItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStatusChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAppInstallItem>,
            ::windows::core::GetTrustLevel,
            ProductId::<Impl, IMPL_OFFSET>,
            PackageFamilyName::<Impl, IMPL_OFFSET>,
            InstallType::<Impl, IMPL_OFFSET>,
            IsUserInitiated::<Impl, IMPL_OFFSET>,
            GetCurrentStatus::<Impl, IMPL_OFFSET>,
            Cancel::<Impl, IMPL_OFFSET>,
            Pause::<Impl, IMPL_OFFSET>,
            Restart::<Impl, IMPL_OFFSET>,
            Completed::<Impl, IMPL_OFFSET>,
            RemoveCompleted::<Impl, IMPL_OFFSET>,
            StatusChanged::<Impl, IMPL_OFFSET>,
            RemoveStatusChanged::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInstallItem as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInstallItem2Impl: Sized {
    fn CancelWithTelemetry(&self, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PauseWithTelemetry(&self, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn RestartWithTelemetry(&self, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppInstallItem2 {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallItem2";
}
#[cfg(feature = "implement_exclusive")]
impl IAppInstallItem2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppInstallItem2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppInstallItem2Vtbl {
        unsafe extern "system" fn CancelWithTelemetry<Impl: IAppInstallItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CancelWithTelemetry(&*(&correlationvector as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PauseWithTelemetry<Impl: IAppInstallItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PauseWithTelemetry(&*(&correlationvector as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RestartWithTelemetry<Impl: IAppInstallItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RestartWithTelemetry(&*(&correlationvector as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppInstallItem2>, ::windows::core::GetTrustLevel, CancelWithTelemetry::<Impl, IMPL_OFFSET>, PauseWithTelemetry::<Impl, IMPL_OFFSET>, RestartWithTelemetry::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInstallItem2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAppInstallItem3Impl: Sized {
    fn Children(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>;
    fn ItemOperationsMightAffectOtherItems(&self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppInstallItem3 {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallItem3";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAppInstallItem3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppInstallItem3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppInstallItem3Vtbl {
        unsafe extern "system" fn Children<Impl: IAppInstallItem3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ItemOperationsMightAffectOtherItems<Impl: IAppInstallItem3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppInstallItem3>, ::windows::core::GetTrustLevel, Children::<Impl, IMPL_OFFSET>, ItemOperationsMightAffectOtherItems::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInstallItem3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInstallItem4Impl: Sized {
    fn LaunchAfterInstall(&self) -> ::windows::core::Result<bool>;
    fn SetLaunchAfterInstall(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppInstallItem4 {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallItem4";
}
#[cfg(feature = "implement_exclusive")]
impl IAppInstallItem4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppInstallItem4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppInstallItem4Vtbl {
        unsafe extern "system" fn LaunchAfterInstall<Impl: IAppInstallItem4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetLaunchAfterInstall<Impl: IAppInstallItem4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLaunchAfterInstall(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppInstallItem4>, ::windows::core::GetTrustLevel, LaunchAfterInstall::<Impl, IMPL_OFFSET>, SetLaunchAfterInstall::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInstallItem4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInstallItem5Impl: Sized {
    fn PinToDesktopAfterInstall(&self) -> ::windows::core::Result<bool>;
    fn SetPinToDesktopAfterInstall(&self, value: bool) -> ::windows::core::Result<()>;
    fn PinToStartAfterInstall(&self) -> ::windows::core::Result<bool>;
    fn SetPinToStartAfterInstall(&self, value: bool) -> ::windows::core::Result<()>;
    fn PinToTaskbarAfterInstall(&self) -> ::windows::core::Result<bool>;
    fn SetPinToTaskbarAfterInstall(&self, value: bool) -> ::windows::core::Result<()>;
    fn CompletedInstallToastNotificationMode(&self) -> ::windows::core::Result<AppInstallationToastNotificationMode>;
    fn SetCompletedInstallToastNotificationMode(&self, value: AppInstallationToastNotificationMode) -> ::windows::core::Result<()>;
    fn InstallInProgressToastNotificationMode(&self) -> ::windows::core::Result<AppInstallationToastNotificationMode>;
    fn SetInstallInProgressToastNotificationMode(&self, value: AppInstallationToastNotificationMode) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppInstallItem5 {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallItem5";
}
#[cfg(feature = "implement_exclusive")]
impl IAppInstallItem5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppInstallItem5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppInstallItem5Vtbl {
        unsafe extern "system" fn PinToDesktopAfterInstall<Impl: IAppInstallItem5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPinToDesktopAfterInstall<Impl: IAppInstallItem5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPinToDesktopAfterInstall(value).into()
        }
        unsafe extern "system" fn PinToStartAfterInstall<Impl: IAppInstallItem5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPinToStartAfterInstall<Impl: IAppInstallItem5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPinToStartAfterInstall(value).into()
        }
        unsafe extern "system" fn PinToTaskbarAfterInstall<Impl: IAppInstallItem5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPinToTaskbarAfterInstall<Impl: IAppInstallItem5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPinToTaskbarAfterInstall(value).into()
        }
        unsafe extern "system" fn CompletedInstallToastNotificationMode<Impl: IAppInstallItem5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppInstallationToastNotificationMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCompletedInstallToastNotificationMode<Impl: IAppInstallItem5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AppInstallationToastNotificationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCompletedInstallToastNotificationMode(value).into()
        }
        unsafe extern "system" fn InstallInProgressToastNotificationMode<Impl: IAppInstallItem5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppInstallationToastNotificationMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetInstallInProgressToastNotificationMode<Impl: IAppInstallItem5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AppInstallationToastNotificationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInstallInProgressToastNotificationMode(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAppInstallItem5>,
            ::windows::core::GetTrustLevel,
            PinToDesktopAfterInstall::<Impl, IMPL_OFFSET>,
            SetPinToDesktopAfterInstall::<Impl, IMPL_OFFSET>,
            PinToStartAfterInstall::<Impl, IMPL_OFFSET>,
            SetPinToStartAfterInstall::<Impl, IMPL_OFFSET>,
            PinToTaskbarAfterInstall::<Impl, IMPL_OFFSET>,
            SetPinToTaskbarAfterInstall::<Impl, IMPL_OFFSET>,
            CompletedInstallToastNotificationMode::<Impl, IMPL_OFFSET>,
            SetCompletedInstallToastNotificationMode::<Impl, IMPL_OFFSET>,
            InstallInProgressToastNotificationMode::<Impl, IMPL_OFFSET>,
            SetInstallInProgressToastNotificationMode::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInstallItem5 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAppInstallManagerImpl: Sized {
    fn AppInstallItems(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>;
    fn Cancel(&self, productid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Pause(&self, productid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Restart(&self, productid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ItemCompleted(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<AppInstallManager, AppInstallManagerItemEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveItemCompleted(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ItemStatusChanged(&self, handler: &::core::option::Option<super::super::super::super::Foundation::TypedEventHandler<AppInstallManager, AppInstallManagerItemEventArgs>>) -> ::windows::core::Result<super::super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveItemStatusChanged(&self, token: &super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AutoUpdateSetting(&self) -> ::windows::core::Result<AutoUpdateSetting>;
    fn SetAutoUpdateSetting(&self, value: AutoUpdateSetting) -> ::windows::core::Result<()>;
    fn AcquisitionIdentity(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAcquisitionIdentity(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn GetIsApplicableAsync(&self, productid: &::windows::core::HSTRING, skuid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn StartAppInstallAsync(&self, productid: &::windows::core::HSTRING, skuid: &::windows::core::HSTRING, repair: bool, forceuseofnonremovablestorage: bool) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>;
    fn UpdateAppByPackageFamilyNameAsync(&self, packagefamilyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>;
    fn SearchForUpdatesAsync(&self, productid: &::windows::core::HSTRING, skuid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>;
    fn SearchForAllUpdatesAsync(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>;
    fn IsStoreBlockedByPolicyAsync(&self, storeclientname: &::windows::core::HSTRING, storeclientpublisher: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn GetIsAppAllowedToInstallAsync(&self, productid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppInstallManager {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAppInstallManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppInstallManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppInstallManagerVtbl {
        unsafe extern "system" fn AppInstallItems<Impl: IAppInstallManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Cancel<Impl: IAppInstallManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cancel(&*(&productid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Pause<Impl: IAppInstallManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Pause(&*(&productid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Restart<Impl: IAppInstallManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Restart(&*(&productid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ItemCompleted<Impl: IAppInstallManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveItemCompleted<Impl: IAppInstallManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveItemCompleted(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ItemStatusChanged<Impl: IAppInstallManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveItemStatusChanged<Impl: IAppInstallManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveItemStatusChanged(&*(&token as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AutoUpdateSetting<Impl: IAppInstallManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AutoUpdateSetting) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAutoUpdateSetting<Impl: IAppInstallManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AutoUpdateSetting) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoUpdateSetting(value).into()
        }
        unsafe extern "system" fn AcquisitionIdentity<Impl: IAppInstallManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAcquisitionIdentity<Impl: IAppInstallManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAcquisitionIdentity(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetIsApplicableAsync<Impl: IAppInstallManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StartAppInstallAsync<Impl: IAppInstallManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, repair: bool, forceuseofnonremovablestorage: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UpdateAppByPackageFamilyNameAsync<Impl: IAppInstallManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SearchForUpdatesAsync<Impl: IAppInstallManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SearchForAllUpdatesAsync<Impl: IAppInstallManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsStoreBlockedByPolicyAsync<Impl: IAppInstallManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storeclientname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, storeclientpublisher: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetIsAppAllowedToInstallAsync<Impl: IAppInstallManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAppInstallManager>,
            ::windows::core::GetTrustLevel,
            AppInstallItems::<Impl, IMPL_OFFSET>,
            Cancel::<Impl, IMPL_OFFSET>,
            Pause::<Impl, IMPL_OFFSET>,
            Restart::<Impl, IMPL_OFFSET>,
            ItemCompleted::<Impl, IMPL_OFFSET>,
            RemoveItemCompleted::<Impl, IMPL_OFFSET>,
            ItemStatusChanged::<Impl, IMPL_OFFSET>,
            RemoveItemStatusChanged::<Impl, IMPL_OFFSET>,
            AutoUpdateSetting::<Impl, IMPL_OFFSET>,
            SetAutoUpdateSetting::<Impl, IMPL_OFFSET>,
            AcquisitionIdentity::<Impl, IMPL_OFFSET>,
            SetAcquisitionIdentity::<Impl, IMPL_OFFSET>,
            GetIsApplicableAsync::<Impl, IMPL_OFFSET>,
            StartAppInstallAsync::<Impl, IMPL_OFFSET>,
            UpdateAppByPackageFamilyNameAsync::<Impl, IMPL_OFFSET>,
            SearchForUpdatesAsync::<Impl, IMPL_OFFSET>,
            SearchForAllUpdatesAsync::<Impl, IMPL_OFFSET>,
            IsStoreBlockedByPolicyAsync::<Impl, IMPL_OFFSET>,
            GetIsAppAllowedToInstallAsync::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInstallManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppInstallManager2Impl: Sized {
    fn StartAppInstallWithTelemetryAsync(&self, productid: &::windows::core::HSTRING, skuid: &::windows::core::HSTRING, repair: bool, forceuseofnonremovablestorage: bool, catalogid: &::windows::core::HSTRING, bundleid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>;
    fn UpdateAppByPackageFamilyNameWithTelemetryAsync(&self, packagefamilyname: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>;
    fn SearchForUpdatesWithTelemetryAsync(&self, productid: &::windows::core::HSTRING, skuid: &::windows::core::HSTRING, catalogid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>;
    fn SearchForAllUpdatesWithTelemetryAsync(&self, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>;
    fn GetIsAppAllowedToInstallWithTelemetryAsync(&self, productid: &::windows::core::HSTRING, skuid: &::windows::core::HSTRING, catalogid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn CancelWithTelemetry(&self, productid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PauseWithTelemetry(&self, productid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn RestartWithTelemetry(&self, productid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppInstallManager2 {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager2";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppInstallManager2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppInstallManager2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppInstallManager2Vtbl {
        unsafe extern "system" fn StartAppInstallWithTelemetryAsync<Impl: IAppInstallManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, repair: bool, forceuseofnonremovablestorage: bool, catalogid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, bundleid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UpdateAppByPackageFamilyNameWithTelemetryAsync<Impl: IAppInstallManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SearchForUpdatesWithTelemetryAsync<Impl: IAppInstallManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, catalogid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SearchForAllUpdatesWithTelemetryAsync<Impl: IAppInstallManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetIsAppAllowedToInstallWithTelemetryAsync<Impl: IAppInstallManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, catalogid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CancelWithTelemetry<Impl: IAppInstallManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CancelWithTelemetry(&*(&productid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&correlationvector as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PauseWithTelemetry<Impl: IAppInstallManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PauseWithTelemetry(&*(&productid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&correlationvector as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RestartWithTelemetry<Impl: IAppInstallManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RestartWithTelemetry(&*(&productid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&correlationvector as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAppInstallManager2>,
            ::windows::core::GetTrustLevel,
            StartAppInstallWithTelemetryAsync::<Impl, IMPL_OFFSET>,
            UpdateAppByPackageFamilyNameWithTelemetryAsync::<Impl, IMPL_OFFSET>,
            SearchForUpdatesWithTelemetryAsync::<Impl, IMPL_OFFSET>,
            SearchForAllUpdatesWithTelemetryAsync::<Impl, IMPL_OFFSET>,
            GetIsAppAllowedToInstallWithTelemetryAsync::<Impl, IMPL_OFFSET>,
            CancelWithTelemetry::<Impl, IMPL_OFFSET>,
            PauseWithTelemetry::<Impl, IMPL_OFFSET>,
            RestartWithTelemetry::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInstallManager2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Management_Deployment", feature = "System", feature = "implement_exclusive"))]
pub trait IAppInstallManager3Impl: Sized {
    fn StartProductInstallAsync(&self, productid: &::windows::core::HSTRING, catalogid: &::windows::core::HSTRING, flightid: &::windows::core::HSTRING, clientid: &::windows::core::HSTRING, repair: bool, forceuseofnonremovablestorage: bool, correlationvector: &::windows::core::HSTRING, targetvolume: &::core::option::Option<super::super::super::super::Management::Deployment::PackageVolume>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>;
    fn StartProductInstallForUserAsync(&self, user: &::core::option::Option<super::super::super::super::System::User>, productid: &::windows::core::HSTRING, catalogid: &::windows::core::HSTRING, flightid: &::windows::core::HSTRING, clientid: &::windows::core::HSTRING, repair: bool, forceuseofnonremovablestorage: bool, correlationvector: &::windows::core::HSTRING, targetvolume: &::core::option::Option<super::super::super::super::Management::Deployment::PackageVolume>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>;
    fn UpdateAppByPackageFamilyNameForUserAsync(&self, user: &::core::option::Option<super::super::super::super::System::User>, packagefamilyname: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>;
    fn SearchForUpdatesForUserAsync(&self, user: &::core::option::Option<super::super::super::super::System::User>, productid: &::windows::core::HSTRING, skuid: &::windows::core::HSTRING, catalogid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>;
    fn SearchForAllUpdatesForUserAsync(&self, user: &::core::option::Option<super::super::super::super::System::User>, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>;
    fn GetIsAppAllowedToInstallForUserAsync(&self, user: &::core::option::Option<super::super::super::super::System::User>, productid: &::windows::core::HSTRING, skuid: &::windows::core::HSTRING, catalogid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn GetIsApplicableForUserAsync(&self, user: &::core::option::Option<super::super::super::super::System::User>, productid: &::windows::core::HSTRING, skuid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn MoveToFrontOfDownloadQueue(&self, productid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Management_Deployment", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppInstallManager3 {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager3";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Management_Deployment", feature = "System", feature = "implement_exclusive"))]
impl IAppInstallManager3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppInstallManager3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppInstallManager3Vtbl {
        unsafe extern "system" fn StartProductInstallAsync<Impl: IAppInstallManager3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, catalogid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, flightid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, clientid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, repair: bool, forceuseofnonremovablestorage: bool, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, targetvolume: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StartProductInstallForUserAsync<Impl: IAppInstallManager3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, catalogid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, flightid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, clientid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, repair: bool, forceuseofnonremovablestorage: bool, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, targetvolume: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UpdateAppByPackageFamilyNameForUserAsync<Impl: IAppInstallManager3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, packagefamilyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SearchForUpdatesForUserAsync<Impl: IAppInstallManager3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, catalogid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SearchForAllUpdatesForUserAsync<Impl: IAppInstallManager3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetIsAppAllowedToInstallForUserAsync<Impl: IAppInstallManager3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, catalogid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetIsApplicableForUserAsync<Impl: IAppInstallManager3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MoveToFrontOfDownloadQueue<Impl: IAppInstallManager3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MoveToFrontOfDownloadQueue(&*(&productid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&correlationvector as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAppInstallManager3>,
            ::windows::core::GetTrustLevel,
            StartProductInstallAsync::<Impl, IMPL_OFFSET>,
            StartProductInstallForUserAsync::<Impl, IMPL_OFFSET>,
            UpdateAppByPackageFamilyNameForUserAsync::<Impl, IMPL_OFFSET>,
            SearchForUpdatesForUserAsync::<Impl, IMPL_OFFSET>,
            SearchForAllUpdatesForUserAsync::<Impl, IMPL_OFFSET>,
            GetIsAppAllowedToInstallForUserAsync::<Impl, IMPL_OFFSET>,
            GetIsApplicableForUserAsync::<Impl, IMPL_OFFSET>,
            MoveToFrontOfDownloadQueue::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInstallManager3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
pub trait IAppInstallManager4Impl: Sized {
    fn GetFreeUserEntitlementAsync(&self, storeid: &::windows::core::HSTRING, campaignid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<GetEntitlementResult>>;
    fn GetFreeUserEntitlementForUserAsync(&self, user: &::core::option::Option<super::super::super::super::System::User>, storeid: &::windows::core::HSTRING, campaignid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<GetEntitlementResult>>;
    fn GetFreeDeviceEntitlementAsync(&self, storeid: &::windows::core::HSTRING, campaignid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<GetEntitlementResult>>;
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppInstallManager4 {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager4";
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "implement_exclusive"))]
impl IAppInstallManager4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppInstallManager4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppInstallManager4Vtbl {
        unsafe extern "system" fn GetFreeUserEntitlementAsync<Impl: IAppInstallManager4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storeid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, campaignid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetFreeUserEntitlementForUserAsync<Impl: IAppInstallManager4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, storeid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, campaignid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetFreeDeviceEntitlementAsync<Impl: IAppInstallManager4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, storeid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, campaignid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppInstallManager4>, ::windows::core::GetTrustLevel, GetFreeUserEntitlementAsync::<Impl, IMPL_OFFSET>, GetFreeUserEntitlementForUserAsync::<Impl, IMPL_OFFSET>, GetFreeDeviceEntitlementAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInstallManager4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAppInstallManager5Impl: Sized {
    fn AppInstallItemsWithGroupSupport(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppInstallManager5 {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager5";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAppInstallManager5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppInstallManager5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppInstallManager5Vtbl {
        unsafe extern "system" fn AppInstallItemsWithGroupSupport<Impl: IAppInstallManager5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppInstallManager5>, ::windows::core::GetTrustLevel, AppInstallItemsWithGroupSupport::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInstallManager5 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System", feature = "implement_exclusive"))]
pub trait IAppInstallManager6Impl: Sized {
    fn SearchForAllUpdatesWithUpdateOptionsAsync(&self, correlationvector: &::windows::core::HSTRING, clientid: &::windows::core::HSTRING, updateoptions: &::core::option::Option<AppUpdateOptions>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>;
    fn SearchForAllUpdatesWithUpdateOptionsForUserAsync(&self, user: &::core::option::Option<super::super::super::super::System::User>, correlationvector: &::windows::core::HSTRING, clientid: &::windows::core::HSTRING, updateoptions: &::core::option::Option<AppUpdateOptions>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>;
    fn SearchForUpdatesWithUpdateOptionsAsync(&self, productid: &::windows::core::HSTRING, skuid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING, clientid: &::windows::core::HSTRING, updateoptions: &::core::option::Option<AppUpdateOptions>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>;
    fn SearchForUpdatesWithUpdateOptionsForUserAsync(&self, user: &::core::option::Option<super::super::super::super::System::User>, productid: &::windows::core::HSTRING, skuid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING, clientid: &::windows::core::HSTRING, updateoptions: &::core::option::Option<AppUpdateOptions>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<AppInstallItem>>;
    fn StartProductInstallWithOptionsAsync(&self, productid: &::windows::core::HSTRING, flightid: &::windows::core::HSTRING, clientid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING, installoptions: &::core::option::Option<AppInstallOptions>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>;
    fn StartProductInstallWithOptionsForUserAsync(&self, user: &::core::option::Option<super::super::super::super::System::User>, productid: &::windows::core::HSTRING, flightid: &::windows::core::HSTRING, clientid: &::windows::core::HSTRING, correlationvector: &::windows::core::HSTRING, installoptions: &::core::option::Option<AppInstallOptions>) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::super::super::Foundation::Collections::IVectorView<AppInstallItem>>>;
    fn GetIsPackageIdentityAllowedToInstallAsync(&self, correlationvector: &::windows::core::HSTRING, packageidentityname: &::windows::core::HSTRING, publishercertificatename: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
    fn GetIsPackageIdentityAllowedToInstallForUserAsync(&self, user: &::core::option::Option<super::super::super::super::System::User>, correlationvector: &::windows::core::HSTRING, packageidentityname: &::windows::core::HSTRING, publishercertificatename: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppInstallManager6 {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager6";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System", feature = "implement_exclusive"))]
impl IAppInstallManager6Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppInstallManager6Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppInstallManager6Vtbl {
        unsafe extern "system" fn SearchForAllUpdatesWithUpdateOptionsAsync<Impl: IAppInstallManager6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, clientid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, updateoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SearchForAllUpdatesWithUpdateOptionsForUserAsync<Impl: IAppInstallManager6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, clientid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, updateoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SearchForUpdatesWithUpdateOptionsAsync<Impl: IAppInstallManager6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, clientid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, updateoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SearchForUpdatesWithUpdateOptionsForUserAsync<Impl: IAppInstallManager6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, skuid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, clientid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, updateoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StartProductInstallWithOptionsAsync<Impl: IAppInstallManager6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, flightid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, clientid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, installoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StartProductInstallWithOptionsForUserAsync<Impl: IAppInstallManager6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, productid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, flightid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, clientid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, installoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetIsPackageIdentityAllowedToInstallAsync<Impl: IAppInstallManager6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packageidentityname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, publishercertificatename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetIsPackageIdentityAllowedToInstallForUserAsync<Impl: IAppInstallManager6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, user: ::windows::core::RawPtr, correlationvector: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, packageidentityname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, publishercertificatename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAppInstallManager6>,
            ::windows::core::GetTrustLevel,
            SearchForAllUpdatesWithUpdateOptionsAsync::<Impl, IMPL_OFFSET>,
            SearchForAllUpdatesWithUpdateOptionsForUserAsync::<Impl, IMPL_OFFSET>,
            SearchForUpdatesWithUpdateOptionsAsync::<Impl, IMPL_OFFSET>,
            SearchForUpdatesWithUpdateOptionsForUserAsync::<Impl, IMPL_OFFSET>,
            StartProductInstallWithOptionsAsync::<Impl, IMPL_OFFSET>,
            StartProductInstallWithOptionsForUserAsync::<Impl, IMPL_OFFSET>,
            GetIsPackageIdentityAllowedToInstallAsync::<Impl, IMPL_OFFSET>,
            GetIsPackageIdentityAllowedToInstallForUserAsync::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInstallManager6 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInstallManager7Impl: Sized {
    fn CanInstallForAllUsers(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppInstallManager7 {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManager7";
}
#[cfg(feature = "implement_exclusive")]
impl IAppInstallManager7Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppInstallManager7Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppInstallManager7Vtbl {
        unsafe extern "system" fn CanInstallForAllUsers<Impl: IAppInstallManager7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppInstallManager7>, ::windows::core::GetTrustLevel, CanInstallForAllUsers::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInstallManager7 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInstallManagerItemEventArgsImpl: Sized {
    fn Item(&self) -> ::windows::core::Result<AppInstallItem>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppInstallManagerItemEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallManagerItemEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAppInstallManagerItemEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppInstallManagerItemEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppInstallManagerItemEventArgsVtbl {
        unsafe extern "system" fn Item<Impl: IAppInstallManagerItemEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppInstallManagerItemEventArgs>, ::windows::core::GetTrustLevel, Item::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInstallManagerItemEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Management_Deployment", feature = "implement_exclusive"))]
pub trait IAppInstallOptionsImpl: Sized {
    fn CatalogId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCatalogId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ForceUseOfNonRemovableStorage(&self) -> ::windows::core::Result<bool>;
    fn SetForceUseOfNonRemovableStorage(&self, value: bool) -> ::windows::core::Result<()>;
    fn AllowForcedAppRestart(&self) -> ::windows::core::Result<bool>;
    fn SetAllowForcedAppRestart(&self, value: bool) -> ::windows::core::Result<()>;
    fn Repair(&self) -> ::windows::core::Result<bool>;
    fn SetRepair(&self, value: bool) -> ::windows::core::Result<()>;
    fn TargetVolume(&self) -> ::windows::core::Result<super::super::super::super::Management::Deployment::PackageVolume>;
    fn SetTargetVolume(&self, value: &::core::option::Option<super::super::super::super::Management::Deployment::PackageVolume>) -> ::windows::core::Result<()>;
    fn LaunchAfterInstall(&self) -> ::windows::core::Result<bool>;
    fn SetLaunchAfterInstall(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Management_Deployment", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppInstallOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallOptions";
}
#[cfg(all(feature = "Management_Deployment", feature = "implement_exclusive"))]
impl IAppInstallOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppInstallOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppInstallOptionsVtbl {
        unsafe extern "system" fn CatalogId<Impl: IAppInstallOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCatalogId<Impl: IAppInstallOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCatalogId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ForceUseOfNonRemovableStorage<Impl: IAppInstallOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetForceUseOfNonRemovableStorage<Impl: IAppInstallOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetForceUseOfNonRemovableStorage(value).into()
        }
        unsafe extern "system" fn AllowForcedAppRestart<Impl: IAppInstallOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAllowForcedAppRestart<Impl: IAppInstallOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowForcedAppRestart(value).into()
        }
        unsafe extern "system" fn Repair<Impl: IAppInstallOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRepair<Impl: IAppInstallOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRepair(value).into()
        }
        unsafe extern "system" fn TargetVolume<Impl: IAppInstallOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTargetVolume<Impl: IAppInstallOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetVolume(&*(&value as *const <super::super::super::super::Management::Deployment::PackageVolume as ::windows::core::Abi>::Abi as *const <super::super::super::super::Management::Deployment::PackageVolume as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn LaunchAfterInstall<Impl: IAppInstallOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetLaunchAfterInstall<Impl: IAppInstallOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLaunchAfterInstall(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAppInstallOptions>,
            ::windows::core::GetTrustLevel,
            CatalogId::<Impl, IMPL_OFFSET>,
            SetCatalogId::<Impl, IMPL_OFFSET>,
            ForceUseOfNonRemovableStorage::<Impl, IMPL_OFFSET>,
            SetForceUseOfNonRemovableStorage::<Impl, IMPL_OFFSET>,
            AllowForcedAppRestart::<Impl, IMPL_OFFSET>,
            SetAllowForcedAppRestart::<Impl, IMPL_OFFSET>,
            Repair::<Impl, IMPL_OFFSET>,
            SetRepair::<Impl, IMPL_OFFSET>,
            TargetVolume::<Impl, IMPL_OFFSET>,
            SetTargetVolume::<Impl, IMPL_OFFSET>,
            LaunchAfterInstall::<Impl, IMPL_OFFSET>,
            SetLaunchAfterInstall::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInstallOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInstallOptions2Impl: Sized {
    fn PinToDesktopAfterInstall(&self) -> ::windows::core::Result<bool>;
    fn SetPinToDesktopAfterInstall(&self, value: bool) -> ::windows::core::Result<()>;
    fn PinToStartAfterInstall(&self) -> ::windows::core::Result<bool>;
    fn SetPinToStartAfterInstall(&self, value: bool) -> ::windows::core::Result<()>;
    fn PinToTaskbarAfterInstall(&self) -> ::windows::core::Result<bool>;
    fn SetPinToTaskbarAfterInstall(&self, value: bool) -> ::windows::core::Result<()>;
    fn CompletedInstallToastNotificationMode(&self) -> ::windows::core::Result<AppInstallationToastNotificationMode>;
    fn SetCompletedInstallToastNotificationMode(&self, value: AppInstallationToastNotificationMode) -> ::windows::core::Result<()>;
    fn InstallInProgressToastNotificationMode(&self) -> ::windows::core::Result<AppInstallationToastNotificationMode>;
    fn SetInstallInProgressToastNotificationMode(&self, value: AppInstallationToastNotificationMode) -> ::windows::core::Result<()>;
    fn InstallForAllUsers(&self) -> ::windows::core::Result<bool>;
    fn SetInstallForAllUsers(&self, value: bool) -> ::windows::core::Result<()>;
    fn StageButDoNotInstall(&self) -> ::windows::core::Result<bool>;
    fn SetStageButDoNotInstall(&self, value: bool) -> ::windows::core::Result<()>;
    fn CampaignId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCampaignId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ExtendedCampaignId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetExtendedCampaignId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppInstallOptions2 {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallOptions2";
}
#[cfg(feature = "implement_exclusive")]
impl IAppInstallOptions2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppInstallOptions2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppInstallOptions2Vtbl {
        unsafe extern "system" fn PinToDesktopAfterInstall<Impl: IAppInstallOptions2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPinToDesktopAfterInstall<Impl: IAppInstallOptions2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPinToDesktopAfterInstall(value).into()
        }
        unsafe extern "system" fn PinToStartAfterInstall<Impl: IAppInstallOptions2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPinToStartAfterInstall<Impl: IAppInstallOptions2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPinToStartAfterInstall(value).into()
        }
        unsafe extern "system" fn PinToTaskbarAfterInstall<Impl: IAppInstallOptions2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPinToTaskbarAfterInstall<Impl: IAppInstallOptions2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPinToTaskbarAfterInstall(value).into()
        }
        unsafe extern "system" fn CompletedInstallToastNotificationMode<Impl: IAppInstallOptions2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppInstallationToastNotificationMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCompletedInstallToastNotificationMode<Impl: IAppInstallOptions2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AppInstallationToastNotificationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCompletedInstallToastNotificationMode(value).into()
        }
        unsafe extern "system" fn InstallInProgressToastNotificationMode<Impl: IAppInstallOptions2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppInstallationToastNotificationMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetInstallInProgressToastNotificationMode<Impl: IAppInstallOptions2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: AppInstallationToastNotificationMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInstallInProgressToastNotificationMode(value).into()
        }
        unsafe extern "system" fn InstallForAllUsers<Impl: IAppInstallOptions2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetInstallForAllUsers<Impl: IAppInstallOptions2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInstallForAllUsers(value).into()
        }
        unsafe extern "system" fn StageButDoNotInstall<Impl: IAppInstallOptions2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetStageButDoNotInstall<Impl: IAppInstallOptions2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStageButDoNotInstall(value).into()
        }
        unsafe extern "system" fn CampaignId<Impl: IAppInstallOptions2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCampaignId<Impl: IAppInstallOptions2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCampaignId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ExtendedCampaignId<Impl: IAppInstallOptions2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetExtendedCampaignId<Impl: IAppInstallOptions2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExtendedCampaignId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAppInstallOptions2>,
            ::windows::core::GetTrustLevel,
            PinToDesktopAfterInstall::<Impl, IMPL_OFFSET>,
            SetPinToDesktopAfterInstall::<Impl, IMPL_OFFSET>,
            PinToStartAfterInstall::<Impl, IMPL_OFFSET>,
            SetPinToStartAfterInstall::<Impl, IMPL_OFFSET>,
            PinToTaskbarAfterInstall::<Impl, IMPL_OFFSET>,
            SetPinToTaskbarAfterInstall::<Impl, IMPL_OFFSET>,
            CompletedInstallToastNotificationMode::<Impl, IMPL_OFFSET>,
            SetCompletedInstallToastNotificationMode::<Impl, IMPL_OFFSET>,
            InstallInProgressToastNotificationMode::<Impl, IMPL_OFFSET>,
            SetInstallInProgressToastNotificationMode::<Impl, IMPL_OFFSET>,
            InstallForAllUsers::<Impl, IMPL_OFFSET>,
            SetInstallForAllUsers::<Impl, IMPL_OFFSET>,
            StageButDoNotInstall::<Impl, IMPL_OFFSET>,
            SetStageButDoNotInstall::<Impl, IMPL_OFFSET>,
            CampaignId::<Impl, IMPL_OFFSET>,
            SetCampaignId::<Impl, IMPL_OFFSET>,
            ExtendedCampaignId::<Impl, IMPL_OFFSET>,
            SetExtendedCampaignId::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInstallOptions2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInstallStatusImpl: Sized {
    fn InstallState(&self) -> ::windows::core::Result<AppInstallState>;
    fn DownloadSizeInBytes(&self) -> ::windows::core::Result<u64>;
    fn BytesDownloaded(&self) -> ::windows::core::Result<u64>;
    fn PercentComplete(&self) -> ::windows::core::Result<f64>;
    fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppInstallStatus {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallStatus";
}
#[cfg(feature = "implement_exclusive")]
impl IAppInstallStatusVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppInstallStatusImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppInstallStatusVtbl {
        unsafe extern "system" fn InstallState<Impl: IAppInstallStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppInstallState) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DownloadSizeInBytes<Impl: IAppInstallStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn BytesDownloaded<Impl: IAppInstallStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PercentComplete<Impl: IAppInstallStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ErrorCode<Impl: IAppInstallStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppInstallStatus>, ::windows::core::GetTrustLevel, InstallState::<Impl, IMPL_OFFSET>, DownloadSizeInBytes::<Impl, IMPL_OFFSET>, BytesDownloaded::<Impl, IMPL_OFFSET>, PercentComplete::<Impl, IMPL_OFFSET>, ErrorCode::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInstallStatus as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
pub trait IAppInstallStatus2Impl: Sized {
    fn User(&self) -> ::windows::core::Result<super::super::super::super::System::User>;
    fn ReadyForLaunch(&self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppInstallStatus2 {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallStatus2";
}
#[cfg(all(feature = "System", feature = "implement_exclusive"))]
impl IAppInstallStatus2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppInstallStatus2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppInstallStatus2Vtbl {
        unsafe extern "system" fn User<Impl: IAppInstallStatus2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReadyForLaunch<Impl: IAppInstallStatus2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppInstallStatus2>, ::windows::core::GetTrustLevel, User::<Impl, IMPL_OFFSET>, ReadyForLaunch::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInstallStatus2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppInstallStatus3Impl: Sized {
    fn IsStaged(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppInstallStatus3 {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.IAppInstallStatus3";
}
#[cfg(feature = "implement_exclusive")]
impl IAppInstallStatus3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppInstallStatus3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppInstallStatus3Vtbl {
        unsafe extern "system" fn IsStaged<Impl: IAppInstallStatus3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppInstallStatus3>, ::windows::core::GetTrustLevel, IsStaged::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppInstallStatus3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppUpdateOptionsImpl: Sized {
    fn CatalogId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCatalogId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AllowForcedAppRestart(&self) -> ::windows::core::Result<bool>;
    fn SetAllowForcedAppRestart(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppUpdateOptions {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.IAppUpdateOptions";
}
#[cfg(feature = "implement_exclusive")]
impl IAppUpdateOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppUpdateOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppUpdateOptionsVtbl {
        unsafe extern "system" fn CatalogId<Impl: IAppUpdateOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCatalogId<Impl: IAppUpdateOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCatalogId(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AllowForcedAppRestart<Impl: IAppUpdateOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAllowForcedAppRestart<Impl: IAppUpdateOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowForcedAppRestart(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppUpdateOptions>, ::windows::core::GetTrustLevel, CatalogId::<Impl, IMPL_OFFSET>, SetCatalogId::<Impl, IMPL_OFFSET>, AllowForcedAppRestart::<Impl, IMPL_OFFSET>, SetAllowForcedAppRestart::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppUpdateOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppUpdateOptions2Impl: Sized {
    fn AutomaticallyDownloadAndInstallUpdateIfFound(&self) -> ::windows::core::Result<bool>;
    fn SetAutomaticallyDownloadAndInstallUpdateIfFound(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppUpdateOptions2 {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.IAppUpdateOptions2";
}
#[cfg(feature = "implement_exclusive")]
impl IAppUpdateOptions2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppUpdateOptions2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppUpdateOptions2Vtbl {
        unsafe extern "system" fn AutomaticallyDownloadAndInstallUpdateIfFound<Impl: IAppUpdateOptions2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAutomaticallyDownloadAndInstallUpdateIfFound<Impl: IAppUpdateOptions2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutomaticallyDownloadAndInstallUpdateIfFound(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppUpdateOptions2>, ::windows::core::GetTrustLevel, AutomaticallyDownloadAndInstallUpdateIfFound::<Impl, IMPL_OFFSET>, SetAutomaticallyDownloadAndInstallUpdateIfFound::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppUpdateOptions2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IGetEntitlementResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<GetEntitlementStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGetEntitlementResult {
    const NAME: &'static str = "Windows.ApplicationModel.Store.Preview.InstallControl.IGetEntitlementResult";
}
#[cfg(feature = "implement_exclusive")]
impl IGetEntitlementResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGetEntitlementResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGetEntitlementResultVtbl {
        unsafe extern "system" fn Status<Impl: IGetEntitlementResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut GetEntitlementStatus) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGetEntitlementResult>, ::windows::core::GetTrustLevel, Status::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGetEntitlementResult as ::windows::core::Interface>::IID
    }
}

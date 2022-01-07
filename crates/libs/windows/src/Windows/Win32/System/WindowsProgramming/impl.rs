pub trait ICameraUIControlImpl: Sized {
    fn Show();
    fn Close();
    fn Suspend();
    fn Resume();
    fn GetCurrentViewType();
    fn GetActiveItem();
    fn GetSelectedItems();
    fn RemoveCapturedItem();
}
impl ::windows::core::RuntimeName for ICameraUIControl {
    const NAME: &'static str = "Windows.Win32.System.WindowsProgramming.ICameraUIControl";
}
impl ICameraUIControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICameraUIControlImpl, const OFFSET: isize>() -> ICameraUIControlVtbl {
        unsafe extern "system" fn Show<Impl: ICameraUIControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwindow: *mut ::core::ffi::c_void, mode: CameraUIControlMode, selectionmode: CameraUIControlLinearSelectionMode, capturemode: CameraUIControlCaptureMode, photoformat: CameraUIControlPhotoFormat, videoformat: CameraUIControlVideoFormat, bhasclosebutton: super::super::Foundation::BOOL, peventcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Show(
                &*(&pwindow as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                mode,
                selectionmode,
                capturemode,
                photoformat,
                videoformat,
                &*(&bhasclosebutton as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&peventcallback as *const <ICameraUIControlEventCallback as ::windows::core::Abi>::Abi as *const <ICameraUIControlEventCallback as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: ICameraUIControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Close() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Suspend<Impl: ICameraUIControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdeferralrequired: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Suspend(::core::mem::transmute_copy(&pbdeferralrequired)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resume<Impl: ICameraUIControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Resume() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentViewType<Impl: ICameraUIControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pviewtype: *mut CameraUIControlViewType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentViewType(::core::mem::transmute_copy(&pviewtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActiveItem<Impl: ICameraUIControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstractiveitempath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActiveItem(::core::mem::transmute_copy(&pbstractiveitempath)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSelectedItems<Impl: ICameraUIControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppselecteditempaths: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSelectedItems(::core::mem::transmute_copy(&ppselecteditempaths)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCapturedItem<Impl: ICameraUIControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveCapturedItem(&*(&pszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ICameraUIControl>,
            ::windows::core::GetTrustLevel,
            Show::<Impl, OFFSET>,
            Close::<Impl, OFFSET>,
            Suspend::<Impl, OFFSET>,
            Resume::<Impl, OFFSET>,
            GetCurrentViewType::<Impl, OFFSET>,
            GetActiveItem::<Impl, OFFSET>,
            GetSelectedItems::<Impl, OFFSET>,
            RemoveCapturedItem::<Impl, OFFSET>,
        )
    }
}
pub trait ICameraUIControlEventCallbackImpl: Sized {
    fn OnStartupComplete();
    fn OnSuspendComplete();
    fn OnItemCaptured();
    fn OnItemDeleted();
    fn OnClosed();
}
impl ::windows::core::RuntimeName for ICameraUIControlEventCallback {
    const NAME: &'static str = "Windows.Win32.System.WindowsProgramming.ICameraUIControlEventCallback";
}
impl ICameraUIControlEventCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICameraUIControlEventCallbackImpl, const OFFSET: isize>() -> ICameraUIControlEventCallbackVtbl {
        unsafe extern "system" fn OnStartupComplete<Impl: ICameraUIControlEventCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnStartupComplete().into()
        }
        unsafe extern "system" fn OnSuspendComplete<Impl: ICameraUIControlEventCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnSuspendComplete().into()
        }
        unsafe extern "system" fn OnItemCaptured<Impl: ICameraUIControlEventCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnItemCaptured(&*(&pszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnItemDeleted<Impl: ICameraUIControlEventCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnItemDeleted(&*(&pszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnClosed<Impl: ICameraUIControlEventCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnClosed().into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICameraUIControlEventCallback>, ::windows::core::GetTrustLevel, OnStartupComplete::<Impl, OFFSET>, OnSuspendComplete::<Impl, OFFSET>, OnItemCaptured::<Impl, OFFSET>, OnItemDeleted::<Impl, OFFSET>, OnClosed::<Impl, OFFSET>)
    }
}
pub trait IClipServiceNotificationHelperImpl: Sized {
    fn ShowToast();
}
impl ::windows::core::RuntimeName for IClipServiceNotificationHelper {
    const NAME: &'static str = "Windows.Win32.System.WindowsProgramming.IClipServiceNotificationHelper";
}
impl IClipServiceNotificationHelperVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClipServiceNotificationHelperImpl, const OFFSET: isize>() -> IClipServiceNotificationHelperVtbl {
        unsafe extern "system" fn ShowToast<Impl: IClipServiceNotificationHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, titletext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bodytext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, packagename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, appid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, launchcommand: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowToast(
                &*(&titletext as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bodytext as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&packagename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&appid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&launchcommand as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IClipServiceNotificationHelper>, ::windows::core::GetTrustLevel, ShowToast::<Impl, OFFSET>)
    }
}
pub trait IContainerActivationHelperImpl: Sized {
    fn CanActivateClientVM();
}
impl ::windows::core::RuntimeName for IContainerActivationHelper {
    const NAME: &'static str = "Windows.Win32.System.WindowsProgramming.IContainerActivationHelper";
}
impl IContainerActivationHelperVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContainerActivationHelperImpl, const OFFSET: isize>() -> IContainerActivationHelperVtbl {
        unsafe extern "system" fn CanActivateClientVM<Impl: IContainerActivationHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isallowed: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanActivateClientVM(::core::mem::transmute_copy(&isallowed)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContainerActivationHelper>, ::windows::core::GetTrustLevel, CanActivateClientVM::<Impl, OFFSET>)
    }
}
pub trait IDefaultBrowserSyncSettingsImpl: Sized {
    fn IsEnabled();
}
impl ::windows::core::RuntimeName for IDefaultBrowserSyncSettings {
    const NAME: &'static str = "Windows.Win32.System.WindowsProgramming.IDefaultBrowserSyncSettings";
}
impl IDefaultBrowserSyncSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDefaultBrowserSyncSettingsImpl, const OFFSET: isize>() -> IDefaultBrowserSyncSettingsVtbl {
        unsafe extern "system" fn IsEnabled<Impl: IDefaultBrowserSyncSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDefaultBrowserSyncSettings>, ::windows::core::GetTrustLevel, IsEnabled::<Impl, OFFSET>)
    }
}
pub trait IDeleteBrowsingHistoryImpl: Sized {
    fn DeleteBrowsingHistory();
}
impl ::windows::core::RuntimeName for IDeleteBrowsingHistory {
    const NAME: &'static str = "Windows.Win32.System.WindowsProgramming.IDeleteBrowsingHistory";
}
impl IDeleteBrowsingHistoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeleteBrowsingHistoryImpl, const OFFSET: isize>() -> IDeleteBrowsingHistoryVtbl {
        unsafe extern "system" fn DeleteBrowsingHistory<Impl: IDeleteBrowsingHistoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteBrowsingHistory(dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDeleteBrowsingHistory>, ::windows::core::GetTrustLevel, DeleteBrowsingHistory::<Impl, OFFSET>)
    }
}
pub trait IEditionUpgradeBrokerImpl: Sized {
    fn InitializeParentWindow();
    fn UpdateOperatingSystem();
    fn ShowProductKeyUI();
    fn CanUpgrade();
}
impl ::windows::core::RuntimeName for IEditionUpgradeBroker {
    const NAME: &'static str = "Windows.Win32.System.WindowsProgramming.IEditionUpgradeBroker";
}
impl IEditionUpgradeBrokerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEditionUpgradeBrokerImpl, const OFFSET: isize>() -> IEditionUpgradeBrokerVtbl {
        unsafe extern "system" fn InitializeParentWindow<Impl: IEditionUpgradeBrokerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parenthandle: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitializeParentWindow(parenthandle) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateOperatingSystem<Impl: IEditionUpgradeBrokerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameter: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateOperatingSystem(&*(&parameter as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowProductKeyUI<Impl: IEditionUpgradeBrokerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowProductKeyUI() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanUpgrade<Impl: IEditionUpgradeBrokerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanUpgrade() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEditionUpgradeBroker>, ::windows::core::GetTrustLevel, InitializeParentWindow::<Impl, OFFSET>, UpdateOperatingSystem::<Impl, OFFSET>, ShowProductKeyUI::<Impl, OFFSET>, CanUpgrade::<Impl, OFFSET>)
    }
}
pub trait IEditionUpgradeHelperImpl: Sized {
    fn CanUpgrade();
    fn UpdateOperatingSystem();
    fn ShowProductKeyUI();
    fn GetOsProductContentId();
    fn GetGenuineLocalStatus();
}
impl ::windows::core::RuntimeName for IEditionUpgradeHelper {
    const NAME: &'static str = "Windows.Win32.System.WindowsProgramming.IEditionUpgradeHelper";
}
impl IEditionUpgradeHelperVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEditionUpgradeHelperImpl, const OFFSET: isize>() -> IEditionUpgradeHelperVtbl {
        unsafe extern "system" fn CanUpgrade<Impl: IEditionUpgradeHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isallowed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanUpgrade(::core::mem::transmute_copy(&isallowed)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateOperatingSystem<Impl: IEditionUpgradeHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateOperatingSystem(&*(&contentid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowProductKeyUI<Impl: IEditionUpgradeHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShowProductKeyUI() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOsProductContentId<Impl: IEditionUpgradeHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOsProductContentId(::core::mem::transmute_copy(&contentid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGenuineLocalStatus<Impl: IEditionUpgradeHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isgenuine: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGenuineLocalStatus(::core::mem::transmute_copy(&isgenuine)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEditionUpgradeHelper>, ::windows::core::GetTrustLevel, CanUpgrade::<Impl, OFFSET>, UpdateOperatingSystem::<Impl, OFFSET>, ShowProductKeyUI::<Impl, OFFSET>, GetOsProductContentId::<Impl, OFFSET>, GetGenuineLocalStatus::<Impl, OFFSET>)
    }
}
pub trait IWindowsLockModeHelperImpl: Sized {
    fn GetSMode();
}
impl ::windows::core::RuntimeName for IWindowsLockModeHelper {
    const NAME: &'static str = "Windows.Win32.System.WindowsProgramming.IWindowsLockModeHelper";
}
impl IWindowsLockModeHelperVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsLockModeHelperImpl, const OFFSET: isize>() -> IWindowsLockModeHelperVtbl {
        unsafe extern "system" fn GetSMode<Impl: IWindowsLockModeHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, issmode: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSMode(::core::mem::transmute_copy(&issmode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWindowsLockModeHelper>, ::windows::core::GetTrustLevel, GetSMode::<Impl, OFFSET>)
    }
}

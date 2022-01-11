#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ICameraUIControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICameraUIControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICameraUIControlVtbl {
        unsafe extern "system" fn Show<Impl: ICameraUIControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwindow: *mut ::core::ffi::c_void, mode: CameraUIControlMode, selectionmode: CameraUIControlLinearSelectionMode, capturemode: CameraUIControlCaptureMode, photoformat: CameraUIControlPhotoFormat, videoformat: CameraUIControlVideoFormat, bhasclosebutton: super::super::Foundation::BOOL, peventcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: ICameraUIControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Suspend<Impl: ICameraUIControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdeferralrequired: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Resume<Impl: ICameraUIControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrentViewType<Impl: ICameraUIControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pviewtype: *mut CameraUIControlViewType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetActiveItem<Impl: ICameraUIControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstractiveitempath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSelectedItems<Impl: ICameraUIControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppselecteditempaths: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveCapturedItem<Impl: ICameraUIControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Show::<Impl, IMPL_OFFSET>, Close::<Impl, IMPL_OFFSET>, Suspend::<Impl, IMPL_OFFSET>, Resume::<Impl, IMPL_OFFSET>, GetCurrentViewType::<Impl, IMPL_OFFSET>, GetActiveItem::<Impl, IMPL_OFFSET>, GetSelectedItems::<Impl, IMPL_OFFSET>, RemoveCapturedItem::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICameraUIControl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ICameraUIControlEventCallbackImpl: Sized {
    fn OnStartupComplete();
    fn OnSuspendComplete();
    fn OnItemCaptured();
    fn OnItemDeleted();
    fn OnClosed();
}
#[cfg(feature = "Win32_Foundation")]
impl ICameraUIControlEventCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICameraUIControlEventCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICameraUIControlEventCallbackVtbl {
        unsafe extern "system" fn OnStartupComplete<Impl: ICameraUIControlEventCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnSuspendComplete<Impl: ICameraUIControlEventCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnItemCaptured<Impl: ICameraUIControlEventCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnItemDeleted<Impl: ICameraUIControlEventCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnClosed<Impl: ICameraUIControlEventCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnStartupComplete::<Impl, IMPL_OFFSET>, OnSuspendComplete::<Impl, IMPL_OFFSET>, OnItemCaptured::<Impl, IMPL_OFFSET>, OnItemDeleted::<Impl, IMPL_OFFSET>, OnClosed::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICameraUIControlEventCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IClipServiceNotificationHelperImpl: Sized {
    fn ShowToast();
}
#[cfg(feature = "Win32_Foundation")]
impl IClipServiceNotificationHelperVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClipServiceNotificationHelperImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IClipServiceNotificationHelperVtbl {
        unsafe extern "system" fn ShowToast<Impl: IClipServiceNotificationHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, titletext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bodytext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, packagename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, appid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, launchcommand: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ShowToast::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClipServiceNotificationHelper as ::windows::core::Interface>::IID
    }
}
pub trait IContainerActivationHelperImpl: Sized {
    fn CanActivateClientVM();
}
impl IContainerActivationHelperVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContainerActivationHelperImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContainerActivationHelperVtbl {
        unsafe extern "system" fn CanActivateClientVM<Impl: IContainerActivationHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isallowed: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CanActivateClientVM::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContainerActivationHelper as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDefaultBrowserSyncSettingsImpl: Sized {
    fn IsEnabled();
}
#[cfg(feature = "Win32_Foundation")]
impl IDefaultBrowserSyncSettingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDefaultBrowserSyncSettingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDefaultBrowserSyncSettingsVtbl {
        unsafe extern "system" fn IsEnabled<Impl: IDefaultBrowserSyncSettingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, IsEnabled::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDefaultBrowserSyncSettings as ::windows::core::Interface>::IID
    }
}
pub trait IDeleteBrowsingHistoryImpl: Sized {
    fn DeleteBrowsingHistory();
}
impl IDeleteBrowsingHistoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeleteBrowsingHistoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDeleteBrowsingHistoryVtbl {
        unsafe extern "system" fn DeleteBrowsingHistory<Impl: IDeleteBrowsingHistoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, DeleteBrowsingHistory::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDeleteBrowsingHistory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEditionUpgradeBrokerImpl: Sized {
    fn InitializeParentWindow();
    fn UpdateOperatingSystem();
    fn ShowProductKeyUI();
    fn CanUpgrade();
}
#[cfg(feature = "Win32_Foundation")]
impl IEditionUpgradeBrokerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEditionUpgradeBrokerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEditionUpgradeBrokerVtbl {
        unsafe extern "system" fn InitializeParentWindow<Impl: IEditionUpgradeBrokerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parenthandle: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateOperatingSystem<Impl: IEditionUpgradeBrokerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameter: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ShowProductKeyUI<Impl: IEditionUpgradeBrokerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CanUpgrade<Impl: IEditionUpgradeBrokerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, InitializeParentWindow::<Impl, IMPL_OFFSET>, UpdateOperatingSystem::<Impl, IMPL_OFFSET>, ShowProductKeyUI::<Impl, IMPL_OFFSET>, CanUpgrade::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEditionUpgradeBroker as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEditionUpgradeHelperImpl: Sized {
    fn CanUpgrade();
    fn UpdateOperatingSystem();
    fn ShowProductKeyUI();
    fn GetOsProductContentId();
    fn GetGenuineLocalStatus();
}
#[cfg(feature = "Win32_Foundation")]
impl IEditionUpgradeHelperVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEditionUpgradeHelperImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEditionUpgradeHelperVtbl {
        unsafe extern "system" fn CanUpgrade<Impl: IEditionUpgradeHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isallowed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateOperatingSystem<Impl: IEditionUpgradeHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ShowProductKeyUI<Impl: IEditionUpgradeHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOsProductContentId<Impl: IEditionUpgradeHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGenuineLocalStatus<Impl: IEditionUpgradeHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isgenuine: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CanUpgrade::<Impl, IMPL_OFFSET>, UpdateOperatingSystem::<Impl, IMPL_OFFSET>, ShowProductKeyUI::<Impl, IMPL_OFFSET>, GetOsProductContentId::<Impl, IMPL_OFFSET>, GetGenuineLocalStatus::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEditionUpgradeHelper as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWindowsLockModeHelperImpl: Sized {
    fn GetSMode();
}
#[cfg(feature = "Win32_Foundation")]
impl IWindowsLockModeHelperVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsLockModeHelperImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWindowsLockModeHelperVtbl {
        unsafe extern "system" fn GetSMode<Impl: IWindowsLockModeHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, issmode: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetSMode::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowsLockModeHelper as ::windows::core::Interface>::IID
    }
}

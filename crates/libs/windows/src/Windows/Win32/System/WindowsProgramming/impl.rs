#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ICameraUIControl_Impl: Sized {
    fn Show(&mut self, pwindow: &::core::option::Option<::windows::core::IUnknown>, mode: CameraUIControlMode, selectionmode: CameraUIControlLinearSelectionMode, capturemode: CameraUIControlCaptureMode, photoformat: CameraUIControlPhotoFormat, videoformat: CameraUIControlVideoFormat, bhasclosebutton: super::super::Foundation::BOOL, peventcallback: &::core::option::Option<ICameraUIControlEventCallback>) -> ::windows::core::Result<()>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
    fn Suspend(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn Resume(&mut self) -> ::windows::core::Result<()>;
    fn GetCurrentViewType(&mut self) -> ::windows::core::Result<CameraUIControlViewType>;
    fn GetActiveItem(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetSelectedItems(&mut self) -> ::windows::core::Result<*mut super::Com::SAFEARRAY>;
    fn RemoveCapturedItem(&mut self, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ICameraUIControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICameraUIControl_Impl, const OFFSET: isize>() -> ICameraUIControl_Vtbl {
        unsafe extern "system" fn Show<Identity: ::windows::core::IUnknownImpl, Impl: ICameraUIControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwindow: *mut ::core::ffi::c_void, mode: CameraUIControlMode, selectionmode: CameraUIControlLinearSelectionMode, capturemode: CameraUIControlCaptureMode, photoformat: CameraUIControlPhotoFormat, videoformat: CameraUIControlVideoFormat, bhasclosebutton: super::super::Foundation::BOOL, peventcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Show(::core::mem::transmute(&pwindow), ::core::mem::transmute_copy(&mode), ::core::mem::transmute_copy(&selectionmode), ::core::mem::transmute_copy(&capturemode), ::core::mem::transmute_copy(&photoformat), ::core::mem::transmute_copy(&videoformat), ::core::mem::transmute_copy(&bhasclosebutton), ::core::mem::transmute(&peventcallback)).into()
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl, Impl: ICameraUIControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn Suspend<Identity: ::windows::core::IUnknownImpl, Impl: ICameraUIControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdeferralrequired: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Suspend() {
                ::core::result::Result::Ok(ok__) => {
                    *pbdeferralrequired = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resume<Identity: ::windows::core::IUnknownImpl, Impl: ICameraUIControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Resume().into()
        }
        unsafe extern "system" fn GetCurrentViewType<Identity: ::windows::core::IUnknownImpl, Impl: ICameraUIControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pviewtype: *mut CameraUIControlViewType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCurrentViewType() {
                ::core::result::Result::Ok(ok__) => {
                    *pviewtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActiveItem<Identity: ::windows::core::IUnknownImpl, Impl: ICameraUIControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstractiveitempath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetActiveItem() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstractiveitempath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSelectedItems<Identity: ::windows::core::IUnknownImpl, Impl: ICameraUIControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppselecteditempaths: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSelectedItems() {
                ::core::result::Result::Ok(ok__) => {
                    *ppselecteditempaths = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCapturedItem<Identity: ::windows::core::IUnknownImpl, Impl: ICameraUIControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveCapturedItem(::core::mem::transmute_copy(&pszpath)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Show: Show::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            Suspend: Suspend::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
            GetCurrentViewType: GetCurrentViewType::<Identity, Impl, OFFSET>,
            GetActiveItem: GetActiveItem::<Identity, Impl, OFFSET>,
            GetSelectedItems: GetSelectedItems::<Identity, Impl, OFFSET>,
            RemoveCapturedItem: RemoveCapturedItem::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICameraUIControl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ICameraUIControlEventCallback_Impl: Sized {
    fn OnStartupComplete(&mut self);
    fn OnSuspendComplete(&mut self);
    fn OnItemCaptured(&mut self, pszpath: super::super::Foundation::PWSTR);
    fn OnItemDeleted(&mut self, pszpath: super::super::Foundation::PWSTR);
    fn OnClosed(&mut self);
}
#[cfg(feature = "Win32_Foundation")]
impl ICameraUIControlEventCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICameraUIControlEventCallback_Impl, const OFFSET: isize>() -> ICameraUIControlEventCallback_Vtbl {
        unsafe extern "system" fn OnStartupComplete<Identity: ::windows::core::IUnknownImpl, Impl: ICameraUIControlEventCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnStartupComplete()
        }
        unsafe extern "system" fn OnSuspendComplete<Identity: ::windows::core::IUnknownImpl, Impl: ICameraUIControlEventCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnSuspendComplete()
        }
        unsafe extern "system" fn OnItemCaptured<Identity: ::windows::core::IUnknownImpl, Impl: ICameraUIControlEventCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnItemCaptured(::core::mem::transmute_copy(&pszpath))
        }
        unsafe extern "system" fn OnItemDeleted<Identity: ::windows::core::IUnknownImpl, Impl: ICameraUIControlEventCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnItemDeleted(::core::mem::transmute_copy(&pszpath))
        }
        unsafe extern "system" fn OnClosed<Identity: ::windows::core::IUnknownImpl, Impl: ICameraUIControlEventCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnClosed()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnStartupComplete: OnStartupComplete::<Identity, Impl, OFFSET>,
            OnSuspendComplete: OnSuspendComplete::<Identity, Impl, OFFSET>,
            OnItemCaptured: OnItemCaptured::<Identity, Impl, OFFSET>,
            OnItemDeleted: OnItemDeleted::<Identity, Impl, OFFSET>,
            OnClosed: OnClosed::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICameraUIControlEventCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IClipServiceNotificationHelper_Impl: Sized {
    fn ShowToast(&mut self, titletext: &super::super::Foundation::BSTR, bodytext: &super::super::Foundation::BSTR, packagename: &super::super::Foundation::BSTR, appid: &super::super::Foundation::BSTR, launchcommand: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IClipServiceNotificationHelper_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClipServiceNotificationHelper_Impl, const OFFSET: isize>() -> IClipServiceNotificationHelper_Vtbl {
        unsafe extern "system" fn ShowToast<Identity: ::windows::core::IUnknownImpl, Impl: IClipServiceNotificationHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, titletext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bodytext: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, packagename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, appid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, launchcommand: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ShowToast(::core::mem::transmute_copy(&titletext), ::core::mem::transmute_copy(&bodytext), ::core::mem::transmute_copy(&packagename), ::core::mem::transmute_copy(&appid), ::core::mem::transmute_copy(&launchcommand)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), ShowToast: ShowToast::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClipServiceNotificationHelper as ::windows::core::Interface>::IID
    }
}
pub trait IContainerActivationHelper_Impl: Sized {
    fn CanActivateClientVM(&mut self) -> ::windows::core::Result<i16>;
}
impl IContainerActivationHelper_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContainerActivationHelper_Impl, const OFFSET: isize>() -> IContainerActivationHelper_Vtbl {
        unsafe extern "system" fn CanActivateClientVM<Identity: ::windows::core::IUnknownImpl, Impl: IContainerActivationHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isallowed: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CanActivateClientVM() {
                ::core::result::Result::Ok(ok__) => {
                    *isallowed = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), CanActivateClientVM: CanActivateClientVM::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContainerActivationHelper as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDefaultBrowserSyncSettings_Impl: Sized {
    fn IsEnabled(&mut self) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl IDefaultBrowserSyncSettings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDefaultBrowserSyncSettings_Impl, const OFFSET: isize>() -> IDefaultBrowserSyncSettings_Vtbl {
        unsafe extern "system" fn IsEnabled<Identity: ::windows::core::IUnknownImpl, Impl: IDefaultBrowserSyncSettings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsEnabled()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), IsEnabled: IsEnabled::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDefaultBrowserSyncSettings as ::windows::core::Interface>::IID
    }
}
pub trait IDeleteBrowsingHistory_Impl: Sized {
    fn DeleteBrowsingHistory(&mut self, dwflags: u32) -> ::windows::core::Result<()>;
}
impl IDeleteBrowsingHistory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeleteBrowsingHistory_Impl, const OFFSET: isize>() -> IDeleteBrowsingHistory_Vtbl {
        unsafe extern "system" fn DeleteBrowsingHistory<Identity: ::windows::core::IUnknownImpl, Impl: IDeleteBrowsingHistory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteBrowsingHistory(::core::mem::transmute_copy(&dwflags)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), DeleteBrowsingHistory: DeleteBrowsingHistory::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDeleteBrowsingHistory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEditionUpgradeBroker_Impl: Sized {
    fn InitializeParentWindow(&mut self, parenthandle: u32) -> ::windows::core::Result<()>;
    fn UpdateOperatingSystem(&mut self, parameter: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ShowProductKeyUI(&mut self) -> ::windows::core::Result<()>;
    fn CanUpgrade(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IEditionUpgradeBroker_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEditionUpgradeBroker_Impl, const OFFSET: isize>() -> IEditionUpgradeBroker_Vtbl {
        unsafe extern "system" fn InitializeParentWindow<Identity: ::windows::core::IUnknownImpl, Impl: IEditionUpgradeBroker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parenthandle: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InitializeParentWindow(::core::mem::transmute_copy(&parenthandle)).into()
        }
        unsafe extern "system" fn UpdateOperatingSystem<Identity: ::windows::core::IUnknownImpl, Impl: IEditionUpgradeBroker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parameter: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UpdateOperatingSystem(::core::mem::transmute_copy(&parameter)).into()
        }
        unsafe extern "system" fn ShowProductKeyUI<Identity: ::windows::core::IUnknownImpl, Impl: IEditionUpgradeBroker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ShowProductKeyUI().into()
        }
        unsafe extern "system" fn CanUpgrade<Identity: ::windows::core::IUnknownImpl, Impl: IEditionUpgradeBroker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CanUpgrade().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            InitializeParentWindow: InitializeParentWindow::<Identity, Impl, OFFSET>,
            UpdateOperatingSystem: UpdateOperatingSystem::<Identity, Impl, OFFSET>,
            ShowProductKeyUI: ShowProductKeyUI::<Identity, Impl, OFFSET>,
            CanUpgrade: CanUpgrade::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEditionUpgradeBroker as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEditionUpgradeHelper_Impl: Sized {
    fn CanUpgrade(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn UpdateOperatingSystem(&mut self, contentid: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn ShowProductKeyUI(&mut self) -> ::windows::core::Result<()>;
    fn GetOsProductContentId(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetGenuineLocalStatus(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IEditionUpgradeHelper_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEditionUpgradeHelper_Impl, const OFFSET: isize>() -> IEditionUpgradeHelper_Vtbl {
        unsafe extern "system" fn CanUpgrade<Identity: ::windows::core::IUnknownImpl, Impl: IEditionUpgradeHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isallowed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CanUpgrade() {
                ::core::result::Result::Ok(ok__) => {
                    *isallowed = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateOperatingSystem<Identity: ::windows::core::IUnknownImpl, Impl: IEditionUpgradeHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UpdateOperatingSystem(::core::mem::transmute_copy(&contentid)).into()
        }
        unsafe extern "system" fn ShowProductKeyUI<Identity: ::windows::core::IUnknownImpl, Impl: IEditionUpgradeHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ShowProductKeyUI().into()
        }
        unsafe extern "system" fn GetOsProductContentId<Identity: ::windows::core::IUnknownImpl, Impl: IEditionUpgradeHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetOsProductContentId() {
                ::core::result::Result::Ok(ok__) => {
                    *contentid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGenuineLocalStatus<Identity: ::windows::core::IUnknownImpl, Impl: IEditionUpgradeHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isgenuine: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetGenuineLocalStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *isgenuine = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CanUpgrade: CanUpgrade::<Identity, Impl, OFFSET>,
            UpdateOperatingSystem: UpdateOperatingSystem::<Identity, Impl, OFFSET>,
            ShowProductKeyUI: ShowProductKeyUI::<Identity, Impl, OFFSET>,
            GetOsProductContentId: GetOsProductContentId::<Identity, Impl, OFFSET>,
            GetGenuineLocalStatus: GetGenuineLocalStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEditionUpgradeHelper as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWindowsLockModeHelper_Impl: Sized {
    fn GetSMode(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWindowsLockModeHelper_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsLockModeHelper_Impl, const OFFSET: isize>() -> IWindowsLockModeHelper_Vtbl {
        unsafe extern "system" fn GetSMode<Identity: ::windows::core::IUnknownImpl, Impl: IWindowsLockModeHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, issmode: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSMode() {
                ::core::result::Result::Ok(ok__) => {
                    *issmode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetSMode: GetSMode::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWindowsLockModeHelper as ::windows::core::Interface>::IID
    }
}
